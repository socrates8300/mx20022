//! Emit `Validatable` and `IsoMessage` trait implementations for generated types.
//!
//! Each type in the IR gets an `impl Validatable` that checks XSD constraints
//! and recurses into nested fields. Document-level types additionally get
//! `impl IsoMessage` with message type and root path metadata.

use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::types::{
    Cardinality, Constraint, EnumDef, FieldDef, NewtypeDef, OpaqueDef, RustType, StructDef,
    TypeDef, TypeGraph, TypeRef, ValueWithAttrDef,
};

use super::util::make_ident;

/// Emit `impl Validatable` blocks for all types and `impl IsoMessage` for
/// document types in the given [`TypeGraph`].
///
/// `choice_types` is the set of type names that are xs:choice enums, needed
/// to know when struct fields use `ChoiceWrapper<T>`.
pub fn emit_validatable_impls(graph: &TypeGraph, choice_types: &HashSet<String>) -> TokenStream {
    let mut tokens = TokenStream::new();

    for type_def in graph.types.values() {
        let impl_tokens = match type_def {
            TypeDef::Struct(d) => emit_struct_validatable(d, choice_types),
            TypeDef::Enum(d) => emit_enum_validatable(d),
            TypeDef::Newtype(d) => emit_newtype_validatable(d),
            TypeDef::CodeEnum(d) => emit_noop_validatable(&d.name),
            TypeDef::ValueWithAttr(d) => emit_value_with_attr_validatable(d, choice_types),
            TypeDef::Opaque(d) => emit_opaque_validatable(d),
        };
        tokens.extend(impl_tokens);
    }

    // Generate IsoMessage impls for document-level types.
    for root in &graph.root_elements {
        tokens.extend(emit_iso_message(
            &root.type_name,
            &root.xml_name,
            &graph.namespace,
        ));
    }

    tokens
}

// ── Struct ────────────────────────────────────────────────────────────────────

fn emit_struct_validatable(def: &StructDef, choice_types: &HashSet<String>) -> TokenStream {
    let name = make_ident(&def.name);

    let field_validations: TokenStream = def
        .fields
        .iter()
        .map(|f| emit_field_validation(f, choice_types))
        .collect();

    quote! {
        impl crate::common::validate::Validatable for #name {
            fn validate_constraints(
                &self,
                path: &str,
                violations: &mut Vec<crate::common::validate::ConstraintViolation>,
            ) {
                #field_validations
            }
        }
    }
}

fn emit_field_validation(field: &FieldDef, choice_types: &HashSet<String>) -> TokenStream {
    let rust_name = make_ident(&field.rust_name);
    let xml_name = &field.xml_name;
    let child_path = format!("{{path}}/{xml_name}");

    // For choice types wrapped in ChoiceWrapper, we access .inner
    let is_choice = match &field.type_ref {
        TypeRef::Named(n) => choice_types.contains(n),
        TypeRef::Builtin(_) => false,
    };

    match &field.cardinality {
        Cardinality::Required => {
            let call = if is_choice {
                quote! { self.#rust_name.inner.validate_constraints("", violations); }
            } else {
                quote! { self.#rust_name.validate_constraints("", violations); }
            };
            quote! {
                {
                    let snap = violations.len();
                    #call
                    if violations.len() > snap {
                        let pfx = format!(#child_path);
                        for v in &mut violations[snap..] {
                            v.path.insert_str(0, &pfx);
                        }
                    }
                }
            }
        }
        Cardinality::Optional => {
            let call = if is_choice {
                quote! { wrapper.inner.validate_constraints("", violations); }
            } else {
                quote! { val.validate_constraints("", violations); }
            };
            let binding = if is_choice {
                quote! { if let Some(ref wrapper) = self.#rust_name }
            } else {
                quote! { if let Some(ref val) = self.#rust_name }
            };
            quote! {
                #binding {
                    let snap = violations.len();
                    #call
                    if violations.len() > snap {
                        let pfx = format!(#child_path);
                        for v in &mut violations[snap..] {
                            v.path.insert_str(0, &pfx);
                        }
                    }
                }
            }
        }
        Cardinality::Vec | Cardinality::BoundedVec(_) => {
            let indexed_path = format!("{{path}}/{xml_name}[{{idx}}]");
            let call = if is_choice {
                quote! { elem.inner.validate_constraints("", violations); }
            } else {
                quote! { elem.validate_constraints("", violations); }
            };
            quote! {
                for (idx, elem) in self.#rust_name.iter().enumerate() {
                    let snap = violations.len();
                    #call
                    if violations.len() > snap {
                        let pfx = format!(#indexed_path);
                        for v in &mut violations[snap..] {
                            v.path.insert_str(0, &pfx);
                        }
                    }
                }
            }
        }
    }
}

// ── Enum (choice types) ──────────────────────────────────────────────────────

fn emit_enum_validatable(def: &EnumDef) -> TokenStream {
    let name = make_ident(&def.name);

    let match_arms: TokenStream = def
        .variants
        .iter()
        .map(|v| {
            let variant_name = make_ident(&v.rust_name);
            let xml_name = &v.xml_name;
            let child_path = format!("{{path}}/{xml_name}");
            quote! {
                Self::#variant_name(inner) => {
                    let snap = violations.len();
                    inner.validate_constraints("", violations);
                    if violations.len() > snap {
                        let pfx = format!(#child_path);
                        for v in &mut violations[snap..] {
                            v.path.insert_str(0, &pfx);
                        }
                    }
                }
            }
        })
        .collect();

    quote! {
        impl crate::common::validate::Validatable for #name {
            fn validate_constraints(
                &self,
                path: &str,
                violations: &mut Vec<crate::common::validate::ConstraintViolation>,
            ) {
                match self {
                    #match_arms
                }
            }
        }
    }
}

// ── Newtype ──────────────────────────────────────────────────────────────────

fn emit_newtype_validatable(def: &NewtypeDef) -> TokenStream {
    let name = make_ident(&def.name);

    let ascii_only = def
        .constraints
        .iter()
        .any(|c| matches!(c, Constraint::Pattern(p) if is_ascii_only_pattern(p)));

    // Count length constraints to decide whether to hoist a shared `let len`.
    let length_constraint_count = def
        .constraints
        .iter()
        .filter(|c| matches!(c, Constraint::MinLength(_) | Constraint::MaxLength(_)))
        .count();
    let hoist_len = length_constraint_count >= 2;

    // Only generate checks for constraints we can evaluate without regex.
    let constraint_checks: TokenStream = def
        .constraints
        .iter()
        .filter_map(|c| emit_constraint_check(c, def.inner, ascii_only, hoist_len))
        .collect();

    // If there are no constraint checks, emit a no-op body.
    if constraint_checks.is_empty() {
        return emit_noop_validatable(&def.name);
    }

    // For bool newtypes, there are no string constraints.
    if def.inner == RustType::Bool {
        return emit_noop_validatable(&def.name);
    }

    // When multiple length constraints exist, hoist a shared length binding.
    let hoisted_len = if hoist_len {
        if ascii_only {
            quote! { let len = self.0.len(); }
        } else {
            quote! { let len = self.0.chars().count(); }
        }
    } else {
        TokenStream::new()
    };

    quote! {
        impl crate::common::validate::Validatable for #name {
            #[allow(clippy::unreadable_literal)]
            fn validate_constraints(
                &self,
                path: &str,
                violations: &mut Vec<crate::common::validate::ConstraintViolation>,
            ) {
                #hoisted_len
                #constraint_checks
            }
        }
    }
}

/// Parts of a constraint check that can be reused in both `Validatable` impls
/// and `TryFrom`/`new()` constructors.
pub(crate) struct ConstraintCheckParts {
    /// Optional setup code (e.g. `let len = value.len();`).
    pub preamble: TokenStream,
    /// Boolean expression: `true` = violation. Operates on `value: &str`.
    pub condition: TokenStream,
    /// Format-string expression for the error message.
    pub message: TokenStream,
    /// Token path to the `ConstraintKind` variant.
    pub kind: TokenStream,
}

/// Extract a reusable constraint check expression from a constraint.
///
/// The `condition` field is a boolean expression over `value: &str` that
/// evaluates to `true` when the constraint is violated.
///
/// Returns `None` for constraints that cannot be checked (e.g. `MinInclusive`
/// on non-decimal types).
pub(crate) fn emit_constraint_expr(
    constraint: &Constraint,
    inner: RustType,
    ascii_only: bool,
    hoist_len: bool,
) -> Option<ConstraintCheckParts> {
    match constraint {
        Constraint::MinLength(n) => {
            #[allow(clippy::cast_possible_truncation)]
            let n_lit = *n as usize;
            let msg = format!("value is shorter than minimum length {n}");
            // When hoisted, the caller emits `let len = ...` once; preamble is empty.
            let preamble = if hoist_len {
                TokenStream::new()
            } else if ascii_only {
                quote! { let len = value.len(); }
            } else {
                quote! { let len = value.chars().count(); }
            };
            Some(ConstraintCheckParts {
                preamble,
                condition: quote! { len < #n_lit },
                message: quote! { format!("{} (got {})", #msg, len) },
                kind: quote! { crate::common::validate::ConstraintKind::MinLength },
            })
        }
        Constraint::MaxLength(n) => {
            #[allow(clippy::cast_possible_truncation)]
            let n_lit = *n as usize;
            let msg = format!("value exceeds maximum length {n}");
            let preamble = if hoist_len {
                TokenStream::new()
            } else if ascii_only {
                quote! { let len = value.len(); }
            } else {
                quote! { let len = value.chars().count(); }
            };
            Some(ConstraintCheckParts {
                preamble,
                condition: quote! { len > #n_lit },
                message: quote! { format!("{} (got {})", #msg, len) },
                kind: quote! { crate::common::validate::ConstraintKind::MaxLength },
            })
        }
        Constraint::TotalDigits(n) => {
            if !matches!(inner, RustType::Decimal) {
                return None;
            }
            let n_lit = *n as usize;
            let msg = format!("value exceeds maximum total digits {n}");
            Some(ConstraintCheckParts {
                preamble: quote! {
                    let digit_count = value.chars()
                        .filter(char::is_ascii_digit)
                        .count();
                },
                condition: quote! { digit_count > #n_lit },
                message: quote! { format!("{} (got {})", #msg, digit_count) },
                kind: quote! { crate::common::validate::ConstraintKind::TotalDigits },
            })
        }
        Constraint::FractionDigits(n) => {
            if !matches!(inner, RustType::Decimal) {
                return None;
            }
            let n_lit = *n as usize;
            let msg = format!("value exceeds maximum fraction digits {n}");
            Some(ConstraintCheckParts {
                preamble: quote! {
                    let frac_count = value.find('.')
                        .map_or(0, |dot| {
                            value[dot + 1..].chars()
                                .filter(char::is_ascii_digit)
                                .count()
                        });
                },
                condition: quote! { frac_count > #n_lit },
                message: quote! { format!("{} (got {})", #msg, frac_count) },
                kind: quote! { crate::common::validate::ConstraintKind::FractionDigits },
            })
        }
        Constraint::Pattern(pat) => {
            let condition = super::pattern_codegen::emit_pattern_check(pat)?;
            let msg = format!("value does not match pattern {pat}");
            Some(ConstraintCheckParts {
                preamble: TokenStream::new(),
                condition,
                message: quote! { #msg.to_string() },
                kind: quote! { crate::common::validate::ConstraintKind::Pattern },
            })
        }
        // MinInclusive/MaxInclusive require numeric parsing — deferred.
        Constraint::MinInclusive(_) | Constraint::MaxInclusive(_) => None,
    }
}

/// Returns `true` if the XSD pattern provably matches only ASCII characters.
///
/// Conservative: returns `false` for any pattern containing constructs that
/// *could* match non-ASCII (negated classes, `.`, `\w`, `\s`, `\p`, etc.).
pub(crate) fn is_ascii_only_pattern(pattern: &str) -> bool {
    let bytes = pattern.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        let b = bytes[i];
        // Non-ASCII byte → not ASCII-only.
        if b > 0x7E {
            return false;
        }
        if b == b'\\' {
            if i + 1 >= len {
                return false;
            }
            let next = bytes[i + 1];
            // Only allow literal escapes of ASCII punctuation and \\. All
            // shorthand classes (\w, \W, \s, \S, \d, \D, \p, \P) are
            // conservatively rejected — in XSD \d is [0-9] (ASCII), but we
            // keep it simple and safe.
            match next {
                b'p' | b'P' | b'w' | b'W' | b's' | b'S' | b'd' | b'D' => return false,
                _ if next > 0x7E => return false,
                _ => {
                    i += 2;
                    continue;
                }
            }
        }
        // `.` matches any character including non-ASCII.
        if b == b'.' {
            return false;
        }
        // Character class: check ranges stay within ASCII.
        if b == b'[' {
            i += 1;
            // Negated character class `[^...]` can match non-ASCII.
            if i < len && bytes[i] == b'^' {
                return false;
            }
            while i < len && bytes[i] != b']' {
                if bytes[i] > 0x7E {
                    return false;
                }
                // Backslash inside character class — reject shorthand classes.
                if bytes[i] == b'\\' {
                    if i + 1 >= len {
                        return false;
                    }
                    let next = bytes[i + 1];
                    match next {
                        b'p' | b'P' | b'w' | b'W' | b's' | b'S' | b'd' | b'D' => {
                            return false;
                        }
                        _ if next > 0x7E => return false,
                        _ => {
                            i += 2;
                            continue;
                        }
                    }
                }
                // Check for range like A-Z.
                if i + 2 < len && bytes[i + 1] == b'-' && bytes[i + 2] != b']' {
                    let start = bytes[i];
                    let end = bytes[i + 2];
                    if start > 0x7E || end > 0x7E {
                        return false;
                    }
                    i += 3;
                } else {
                    i += 1;
                }
            }
            // Skip closing bracket.
            if i < len {
                i += 1;
            }
            continue;
        }
        i += 1;
    }
    true
}

fn emit_constraint_check(
    constraint: &Constraint,
    inner: RustType,
    ascii_only: bool,
    hoist_len: bool,
) -> Option<TokenStream> {
    let parts = emit_constraint_expr(constraint, inner, ascii_only, hoist_len)?;
    let preamble = parts.preamble;
    let condition = parts.condition;
    let message = parts.message;
    let kind = parts.kind;

    // When length is hoisted, length constraints don't need `value` binding.
    let needs_value = !hoist_len
        || !matches!(
            constraint,
            Constraint::MinLength(_) | Constraint::MaxLength(_)
        );
    let value_binding = if needs_value {
        quote! { let value: &str = &self.0; }
    } else {
        TokenStream::new()
    };

    Some(quote! {
        {
            #value_binding
            #preamble
            let violated = #condition;
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: #message,
                    kind: #kind,
                });
            }
        }
    })
}

// ── ValueWithAttr ────────────────────────────────────────────────────────────

fn emit_value_with_attr_validatable(
    def: &ValueWithAttrDef,
    choice_types: &HashSet<String>,
) -> TokenStream {
    let name = make_ident(&def.name);

    // Validate the value field — passes path directly (no child segment).
    let value_validation = match &def.value_type {
        TypeRef::Named(_) => {
            quote! {
                self.value.validate_constraints(path, violations);
            }
        }
        TypeRef::Builtin(_) => TokenStream::new(), // String, bool, etc. — no-op
    };

    // Validate attribute fields.
    let attr_validations: TokenStream = def
        .attributes
        .iter()
        .map(|attr| {
            let rust_name = make_ident(&attr.rust_name);
            let xml_name = &attr.xml_name;
            let attr_path = format!("{{path}}/@{xml_name}");

            let is_named = matches!(&attr.type_ref, TypeRef::Named(n) if !choice_types.contains(n));
            let is_choice = matches!(&attr.type_ref, TypeRef::Named(n) if choice_types.contains(n));

            if attr.required {
                if is_named {
                    quote! {
                        {
                            let snap = violations.len();
                            self.#rust_name.validate_constraints("", violations);
                            if violations.len() > snap {
                                let pfx = format!(#attr_path);
                                for v in &mut violations[snap..] {
                                    v.path.insert_str(0, &pfx);
                                }
                            }
                        }
                    }
                } else if is_choice {
                    quote! {
                        {
                            let snap = violations.len();
                            self.#rust_name.inner.validate_constraints("", violations);
                            if violations.len() > snap {
                                let pfx = format!(#attr_path);
                                for v in &mut violations[snap..] {
                                    v.path.insert_str(0, &pfx);
                                }
                            }
                        }
                    }
                } else {
                    TokenStream::new()
                }
            } else if is_named {
                quote! {
                    if let Some(ref val) = self.#rust_name {
                        let snap = violations.len();
                        val.validate_constraints("", violations);
                        if violations.len() > snap {
                            let pfx = format!(#attr_path);
                            for v in &mut violations[snap..] {
                                v.path.insert_str(0, &pfx);
                            }
                        }
                    }
                }
            } else {
                TokenStream::new()
            }
        })
        .collect();

    quote! {
        impl crate::common::validate::Validatable for #name {
            fn validate_constraints(
                &self,
                path: &str,
                violations: &mut Vec<crate::common::validate::ConstraintViolation>,
            ) {
                #value_validation
                #attr_validations
            }
        }
    }
}

// ── Opaque ───────────────────────────────────────────────────────────────────

fn emit_opaque_validatable(def: &OpaqueDef) -> TokenStream {
    emit_noop_validatable(&def.name)
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Emit a no-op `impl Validatable` for types with no constraints to check.
fn emit_noop_validatable(type_name: &str) -> TokenStream {
    let name = make_ident(type_name);
    quote! {
        impl crate::common::validate::Validatable for #name {
            fn validate_constraints(
                &self,
                _path: &str,
                _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
            ) {}
        }
    }
}

/// Extract an ISO 20022 message type from a namespace URI.
///
/// For example, `"urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13"` → `"pacs.008.001.13"`.
fn extract_message_type(namespace: &str) -> &str {
    namespace.rsplit(':').next().unwrap_or(namespace)
}

/// Emit `impl IsoMessage` for a document-level type.
fn emit_iso_message(type_name: &str, root_xml_name: &str, namespace: &str) -> TokenStream {
    let name = make_ident(type_name);
    let msg_type = extract_message_type(namespace);
    let root_path = format!("/{root_xml_name}");

    quote! {
        impl crate::common::validate::IsoMessage for #name {
            fn message_type(&self) -> &'static str {
                #msg_type
            }
            fn root_path(&self) -> &'static str {
                #root_path
            }
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{
        AttrDef, Constraint, EnumDef, FieldDef, NewtypeDef, OpaqueDef, RootElement, StructDef,
        TypeGraph, ValueWithAttrDef, VariantDef,
    };

    fn make_graph(types: Vec<(String, TypeDef)>, root_elements: Vec<RootElement>) -> TypeGraph {
        TypeGraph {
            namespace: "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13".to_owned(),
            root_elements,
            types: types.into_iter().collect(),
        }
    }

    fn parse_and_check(tokens: &TokenStream) {
        let src = tokens.to_string();
        syn::parse_file(&src).unwrap_or_else(|e| {
            panic!("generated code is not valid Rust:\n{src}\nerror: {e}");
        });
    }

    #[test]
    fn extract_message_type_from_namespace() {
        assert_eq!(
            extract_message_type("urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13"),
            "pacs.008.001.13"
        );
        assert_eq!(
            extract_message_type("urn:iso:std:iso:20022:tech:xsd:head.001.001.04"),
            "head.001.001.04"
        );
    }

    #[test]
    fn newtype_with_maxlength_emits_check() {
        let def = NewtypeDef {
            name: "Max35Text".to_owned(),
            inner: RustType::String,
            constraints: vec![Constraint::MaxLength(35)],
        };
        let ts = emit_newtype_validatable(&def);
        let src = ts.to_string();
        assert!(src.contains("chars"), "should count chars: {src}");
        assert!(
            src.contains("MaxLength"),
            "should reference MaxLength: {src}"
        );
        parse_and_check(&ts);
    }

    #[test]
    fn newtype_with_minlength_emits_check() {
        let def = NewtypeDef {
            name: "Min1Text".to_owned(),
            inner: RustType::String,
            constraints: vec![Constraint::MinLength(1)],
        };
        let ts = emit_newtype_validatable(&def);
        let src = ts.to_string();
        assert!(
            src.contains("MinLength"),
            "should reference MinLength: {src}"
        );
        parse_and_check(&ts);
    }

    #[test]
    fn newtype_with_total_digits_emits_check() {
        let def = NewtypeDef {
            name: "Amount".to_owned(),
            inner: RustType::Decimal,
            constraints: vec![Constraint::TotalDigits(18)],
        };
        let ts = emit_newtype_validatable(&def);
        let src = ts.to_string();
        assert!(src.contains("is_ascii_digit"), "should count digits: {src}");
        assert!(
            src.contains("TotalDigits"),
            "should reference TotalDigits: {src}"
        );
        parse_and_check(&ts);
    }

    #[test]
    fn newtype_with_fraction_digits_emits_check() {
        let def = NewtypeDef {
            name: "Amount".to_owned(),
            inner: RustType::Decimal,
            constraints: vec![Constraint::FractionDigits(5)],
        };
        let ts = emit_newtype_validatable(&def);
        let src = ts.to_string();
        assert!(
            src.contains("FractionDigits"),
            "should reference FractionDigits: {src}"
        );
        parse_and_check(&ts);
    }

    #[test]
    fn newtype_pattern_emits_real_check() {
        let def = NewtypeDef {
            name: "ActiveCurrencyCode".to_owned(),
            inner: RustType::String,
            constraints: vec![Constraint::Pattern("[A-Z]{3,3}".to_owned())],
        };
        let ts = emit_newtype_validatable(&def);
        let src = ts.to_string();
        // Pattern should now emit a real constraint check (not a no-op).
        assert!(
            src.contains("Pattern"),
            "should reference ConstraintKind::Pattern: {src}"
        );
        // path is still used (in newtype validatable the path param is used for violation push).
        parse_and_check(&ts);
    }

    #[test]
    fn struct_emits_recursive_validation() {
        let def = StructDef {
            name: "GroupHeader".to_owned(),
            fields: vec![
                FieldDef {
                    xml_name: "MsgId".to_owned(),
                    rust_name: "msg_id".to_owned(),
                    type_ref: TypeRef::Named("Max35Text".to_owned()),
                    cardinality: Cardinality::Required,
                },
                FieldDef {
                    xml_name: "PmtTpInf".to_owned(),
                    rust_name: "pmt_tp_inf".to_owned(),
                    type_ref: TypeRef::Named("SomeType".to_owned()),
                    cardinality: Cardinality::Optional,
                },
                FieldDef {
                    xml_name: "Items".to_owned(),
                    rust_name: "items".to_owned(),
                    type_ref: TypeRef::Named("ItemType".to_owned()),
                    cardinality: Cardinality::Vec,
                },
            ],
        };
        let ts = emit_struct_validatable(&def, &HashSet::new());
        let src = ts.to_string();
        // Required field: deferred path call.
        assert!(
            src.contains("self . msg_id . validate_constraints"),
            "src = {src}"
        );
        // Snapshot+patch pattern present.
        assert!(
            src.contains("insert_str"),
            "should use deferred paths: {src}"
        );
        // Optional field: if let Some.
        assert!(src.contains("if let Some"), "src = {src}");
        // Vec field: enumerate loop with index.
        assert!(src.contains("enumerate"), "src = {src}");
        parse_and_check(&ts);
    }

    #[test]
    fn enum_emits_match_arms() {
        let def = EnumDef {
            name: "MyChoice".to_owned(),
            variants: vec![
                VariantDef {
                    xml_name: "TypeA".to_owned(),
                    rust_name: "TypeA".to_owned(),
                    type_ref: TypeRef::Named("A".to_owned()),
                },
                VariantDef {
                    xml_name: "TypeB".to_owned(),
                    rust_name: "TypeB".to_owned(),
                    type_ref: TypeRef::Named("B".to_owned()),
                },
            ],
        };
        let ts = emit_enum_validatable(&def);
        let src = ts.to_string();
        assert!(src.contains("Self :: TypeA"), "src = {src}");
        assert!(src.contains("Self :: TypeB"), "src = {src}");
        assert!(src.contains("match self"), "src = {src}");
        parse_and_check(&ts);
    }

    #[test]
    fn code_enum_emits_noop() {
        let ts = emit_noop_validatable("ChargeBearerType1Code");
        let src = ts.to_string();
        assert!(src.contains("_path"), "should have unused path: {src}");
        parse_and_check(&ts);
    }

    #[test]
    fn opaque_emits_noop() {
        let def = OpaqueDef {
            name: "SignatureEnvelope".to_owned(),
            namespace: Some("##other".to_owned()),
        };
        let ts = emit_opaque_validatable(&def);
        let src = ts.to_string();
        assert!(src.contains("_path"), "src = {src}");
        parse_and_check(&ts);
    }

    #[test]
    fn value_with_attr_emits_validation() {
        let def = ValueWithAttrDef {
            name: "ActiveCurrencyAndAmount".to_owned(),
            value_type: TypeRef::Named("AmountType".to_owned()),
            attributes: vec![AttrDef {
                xml_name: "Ccy".to_owned(),
                rust_name: "ccy".to_owned(),
                type_ref: TypeRef::Named("CurrencyCode".to_owned()),
                required: true,
            }],
        };
        let ts = emit_value_with_attr_validatable(&def, &HashSet::new());
        let src = ts.to_string();
        assert!(
            src.contains("self . value . validate_constraints"),
            "src = {src}"
        );
        assert!(
            src.contains("self . ccy . validate_constraints"),
            "src = {src}"
        );
        assert!(src.contains("@Ccy"), "should have attr path: {src}");
        assert!(
            src.contains("insert_str"),
            "should use deferred attr paths: {src}"
        );
        parse_and_check(&ts);
    }

    #[test]
    fn iso_message_impl() {
        let ts = emit_iso_message(
            "Document",
            "Document",
            "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13",
        );
        let src = ts.to_string();
        assert!(src.contains("pacs.008.001.13"), "src = {src}");
        assert!(src.contains("/Document"), "src = {src}");
        parse_and_check(&ts);
    }

    #[test]
    fn full_emit_with_validatable() {
        let graph = make_graph(
            vec![
                (
                    "Max35Text".to_owned(),
                    TypeDef::Newtype(NewtypeDef {
                        name: "Max35Text".to_owned(),
                        inner: RustType::String,
                        constraints: vec![Constraint::MinLength(1), Constraint::MaxLength(35)],
                    }),
                ),
                (
                    "Document".to_owned(),
                    TypeDef::Struct(StructDef {
                        name: "Document".to_owned(),
                        fields: vec![FieldDef {
                            xml_name: "MsgId".to_owned(),
                            rust_name: "msg_id".to_owned(),
                            type_ref: TypeRef::Named("Max35Text".to_owned()),
                            cardinality: Cardinality::Required,
                        }],
                    }),
                ),
            ],
            vec![RootElement {
                xml_name: "Document".to_owned(),
                type_name: "Document".to_owned(),
            }],
        );

        let choice_types = HashSet::new();
        let ts = emit_validatable_impls(&graph, &choice_types);
        let src = ts.to_string();

        // Should have Validatable impl for Max35Text.
        assert!(
            src.contains("impl crate :: common :: validate :: Validatable for Max35Text"),
            "src = {src}"
        );
        // Should have Validatable impl for Document.
        assert!(
            src.contains("impl crate :: common :: validate :: Validatable for Document"),
            "src = {src}"
        );
        // Should have IsoMessage impl for Document.
        assert!(
            src.contains("impl crate :: common :: validate :: IsoMessage for Document"),
            "src = {src}"
        );
        parse_and_check(&ts);
    }

    #[test]
    fn ascii_only_pattern_detection() {
        // ASCII-only patterns.
        assert!(is_ascii_only_pattern("[A-Z]{3,3}"));
        assert!(is_ascii_only_pattern("[A-Za-z0-9]+"));
        assert!(is_ascii_only_pattern("[0-9]{4}-[0-9]{2}-[0-9]{2}"));
        assert!(is_ascii_only_pattern(
            "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}"
        ));
        assert!(is_ascii_only_pattern("")); // empty pattern

        // Non-ASCII patterns.
        assert!(!is_ascii_only_pattern(".")); // dot matches anything
        assert!(!is_ascii_only_pattern("\\w+")); // \w shorthand
        assert!(!is_ascii_only_pattern("\\s+")); // \s shorthand
        assert!(!is_ascii_only_pattern("\\d+")); // \d shorthand (conservative)
        assert!(!is_ascii_only_pattern("\\p{L}")); // Unicode property

        // Negated character class matches non-ASCII.
        assert!(!is_ascii_only_pattern("[^A-Z]"));
        assert!(!is_ascii_only_pattern("[^0-9]+"));

        // Shorthand class inside character class.
        assert!(!is_ascii_only_pattern("[\\w]+"));
        assert!(!is_ascii_only_pattern("[A-Z\\s]+"));
    }

    #[test]
    fn ascii_type_useslen() {
        let def = NewtypeDef {
            name: "CurrencyCode".to_owned(),
            inner: RustType::String,
            constraints: vec![
                Constraint::Pattern("[A-Z]{3,3}".to_owned()),
                Constraint::MinLength(3),
                Constraint::MaxLength(3),
            ],
        };
        let ts = emit_newtype_validatable(&def);
        let src = ts.to_string();
        assert!(
            src.contains(". len ()"),
            "ASCII type should use .len(): {src}"
        );
        assert!(
            !src.contains("chars"),
            "ASCII type should NOT use .chars().count(): {src}"
        );
        parse_and_check(&ts);
    }

    #[test]
    fn non_ascii_type_uses_chars_count() {
        let def = NewtypeDef {
            name: "FreeText".to_owned(),
            inner: RustType::String,
            constraints: vec![Constraint::MinLength(1), Constraint::MaxLength(100)],
        };
        let ts = emit_newtype_validatable(&def);
        let src = ts.to_string();
        assert!(
            src.contains("chars"),
            "non-ASCII type should use .chars().count(): {src}"
        );
        parse_and_check(&ts);
    }
}
