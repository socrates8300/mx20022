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
            if is_choice {
                quote! {
                    self.#rust_name.inner.validate_constraints(
                        &format!(#child_path), violations,
                    );
                }
            } else {
                quote! {
                    self.#rust_name.validate_constraints(
                        &format!(#child_path), violations,
                    );
                }
            }
        }
        Cardinality::Optional => {
            if is_choice {
                quote! {
                    if let Some(ref wrapper) = self.#rust_name {
                        wrapper.inner.validate_constraints(
                            &format!(#child_path), violations,
                        );
                    }
                }
            } else {
                quote! {
                    if let Some(ref val) = self.#rust_name {
                        val.validate_constraints(
                            &format!(#child_path), violations,
                        );
                    }
                }
            }
        }
        Cardinality::Vec | Cardinality::BoundedVec(_) => {
            let indexed_path = format!("{{path}}/{xml_name}[{{i}}]");
            if is_choice {
                quote! {
                    for (i, item) in self.#rust_name.iter().enumerate() {
                        item.inner.validate_constraints(
                            &format!(#indexed_path), violations,
                        );
                    }
                }
            } else {
                quote! {
                    for (i, item) in self.#rust_name.iter().enumerate() {
                        item.validate_constraints(
                            &format!(#indexed_path), violations,
                        );
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
                    inner.validate_constraints(&format!(#child_path), violations);
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

    // Only generate checks for constraints we can evaluate without regex.
    let constraint_checks: TokenStream = def
        .constraints
        .iter()
        .filter_map(|c| emit_constraint_check(c, def.inner))
        .collect();

    // If there are no constraint checks, emit a no-op body.
    if constraint_checks.is_empty() {
        return emit_noop_validatable(&def.name);
    }

    // For bool newtypes, there are no string constraints.
    if def.inner == RustType::Bool {
        return emit_noop_validatable(&def.name);
    }

    quote! {
        impl crate::common::validate::Validatable for #name {
            #[allow(clippy::unreadable_literal)]
            fn validate_constraints(
                &self,
                path: &str,
                violations: &mut Vec<crate::common::validate::ConstraintViolation>,
            ) {
                #constraint_checks
            }
        }
    }
}

fn emit_constraint_check(constraint: &Constraint, inner: RustType) -> Option<TokenStream> {
    // Skip constraints that need regex or numeric parsing for now.
    match constraint {
        Constraint::MinLength(n) => {
            // XSD constraint values are always small; truncation is not a concern.
            #[allow(clippy::cast_possible_truncation)]
            let n_lit = *n as usize;
            let msg = format!("value is shorter than minimum length {n}");
            Some(quote! {
                {
                    let len = self.0.chars().count();
                    if len < #n_lit {
                        violations.push(crate::common::validate::ConstraintViolation {
                            path: path.to_string(),
                            message: format!(
                                "{} (got {})", #msg, len
                            ),
                            kind: crate::common::validate::ConstraintKind::MinLength,
                        });
                    }
                }
            })
        }
        Constraint::MaxLength(n) => {
            #[allow(clippy::cast_possible_truncation)]
            let n_lit = *n as usize;
            let msg = format!("value exceeds maximum length {n}");
            Some(quote! {
                {
                    let len = self.0.chars().count();
                    if len > #n_lit {
                        violations.push(crate::common::validate::ConstraintViolation {
                            path: path.to_string(),
                            message: format!(
                                "{} (got {})", #msg, len
                            ),
                            kind: crate::common::validate::ConstraintKind::MaxLength,
                        });
                    }
                }
            })
        }
        Constraint::TotalDigits(n) => {
            if !matches!(inner, RustType::Decimal) {
                return None;
            }
            let n_lit = *n as usize;
            let msg = format!("value exceeds maximum total digits {n}");
            Some(quote! {
                {
                    let digit_count = self.0.chars()
                        .filter(char::is_ascii_digit)
                        .count();
                    if digit_count > #n_lit {
                        violations.push(crate::common::validate::ConstraintViolation {
                            path: path.to_string(),
                            message: format!(
                                "{} (got {})", #msg, digit_count
                            ),
                            kind: crate::common::validate::ConstraintKind::TotalDigits,
                        });
                    }
                }
            })
        }
        Constraint::FractionDigits(n) => {
            if !matches!(inner, RustType::Decimal) {
                return None;
            }
            let n_lit = *n as usize;
            let msg = format!("value exceeds maximum fraction digits {n}");
            Some(quote! {
                {
                    let frac_count = self.0.find('.')
                        .map_or(0, |dot| {
                            self.0[dot + 1..].chars()
                                .filter(char::is_ascii_digit)
                                .count()
                        });
                    if frac_count > #n_lit {
                        violations.push(crate::common::validate::ConstraintViolation {
                            path: path.to_string(),
                            message: format!(
                                "{} (got {})", #msg, frac_count
                            ),
                            kind: crate::common::validate::ConstraintKind::FractionDigits,
                        });
                    }
                }
            })
        }
        // Pattern and MinInclusive/MaxInclusive require regex or numeric
        // parsing — deferred to the RuleRegistry in the validate crate.
        Constraint::Pattern(_) | Constraint::MinInclusive(_) | Constraint::MaxInclusive(_) => None,
    }
}

// ── ValueWithAttr ────────────────────────────────────────────────────────────

fn emit_value_with_attr_validatable(
    def: &ValueWithAttrDef,
    choice_types: &HashSet<String>,
) -> TokenStream {
    let name = make_ident(&def.name);

    // Validate the value field.
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
                        self.#rust_name.validate_constraints(
                            &format!(#attr_path), violations,
                        );
                    }
                } else if is_choice {
                    quote! {
                        self.#rust_name.inner.validate_constraints(
                            &format!(#attr_path), violations,
                        );
                    }
                } else {
                    TokenStream::new()
                }
            } else if is_named {
                quote! {
                    if let Some(ref val) = self.#rust_name {
                        val.validate_constraints(
                            &format!(#attr_path), violations,
                        );
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
        AttrDef, CodeEnumDef, CodeValue, Constraint, EnumDef, FieldDef, NewtypeDef, OpaqueDef,
        RootElement, StructDef, TypeGraph, ValueWithAttrDef, VariantDef,
    };
    use indexmap::IndexMap;

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
    fn newtype_with_max_length_emits_check() {
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
    fn newtype_with_min_length_emits_check() {
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
    fn newtype_pattern_only_emits_noop() {
        let def = NewtypeDef {
            name: "ActiveCurrencyCode".to_owned(),
            inner: RustType::String,
            constraints: vec![Constraint::Pattern("[A-Z]{3,3}".to_owned())],
        };
        let ts = emit_newtype_validatable(&def);
        let src = ts.to_string();
        // Pattern-only should be a no-op (no constraint checks).
        assert!(
            src.contains("_path"),
            "should have unused path param: {src}"
        );
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
        // Required field: direct call.
        assert!(
            src.contains("self . msg_id . validate_constraints"),
            "src = {src}"
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
}
