//! Emit [`NewtypeDef`] as a Rust newtype struct.

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::types::{Constraint, NewtypeDef, RustType};

use super::util::{make_ident, standard_derives};

/// Emit a single [`NewtypeDef`] as a `pub struct` newtype.
///
/// The generated struct is `#[serde(transparent)]` so it serializes as its
/// inner value. Constraints from XSD facets are preserved as doc comments.
///
/// If the newtype has constraints and is not `bool`-inner, also emits:
/// - `TryFrom<String>` — validates constraints, returns `ConstraintError` on failure
/// - `new(impl Into<String>)` — convenience constructor delegating to `TryFrom`
/// - `From<T> for String` — unwrap the inner value
pub fn emit_newtype(def: &NewtypeDef) -> TokenStream {
    let derives = standard_derives();
    let name = make_ident(&def.name);
    let inner_type = rust_type_tokens(def.inner);

    // Emit constraint doc comments if any.
    let constraint_docs: TokenStream = def
        .constraints
        .iter()
        .map(|c| {
            let doc = constraint_doc(c);
            quote! { #[doc = #doc] }
        })
        .collect();

    let mut tokens = quote! {
        #constraint_docs
        #derives
        #[serde(transparent)]
        pub struct #name(pub #inner_type);
    };

    // Emit TryFrom + new + From for constrained, non-bool newtypes.
    if !def.constraints.is_empty() && def.inner != RustType::Bool {
        tokens.extend(emit_try_from(def));
    }

    tokens
}

/// Emit `TryFrom<String>`, `new()`, and `From<T> for String` for a constrained newtype.
fn emit_try_from(def: &NewtypeDef) -> TokenStream {
    let name = make_ident(&def.name);

    // Collect constraint checks. Each produces a guard block.
    let guard_blocks: Vec<TokenStream> = def
        .constraints
        .iter()
        .filter_map(|c| {
            let parts = super::validate::emit_constraint_expr(c, def.inner)?;
            let condition = parts.condition;
            let message = parts.message;
            let kind = parts.kind;
            Some(quote! {
                {
                    let violated = #condition;
                    if violated {
                        return Err(crate::common::validate::ConstraintError {
                            kind: #kind,
                            message: #message,
                        });
                    }
                }
            })
        })
        .collect();

    // If no constraints produced checks (e.g. all MinInclusive on String),
    // skip emitting TryFrom entirely.
    if guard_blocks.is_empty() {
        return TokenStream::new();
    }

    quote! {
        impl TryFrom<String> for #name {
            type Error = crate::common::validate::ConstraintError;

            #[allow(clippy::unreadable_literal)]
            fn try_from(value: String) -> Result<Self, Self::Error> {
                {
                    let value: &str = &value;
                    #(#guard_blocks)*
                }
                Ok(Self(value))
            }
        }

        impl #name {
            /// Construct a validated instance, checking all XSD constraints.
            #[allow(clippy::unreadable_literal)]
            pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
                Self::try_from(value.into())
            }
        }

        impl From<#name> for String {
            fn from(v: #name) -> Self {
                v.0
            }
        }
    }
}

fn rust_type_tokens(rt: RustType) -> TokenStream {
    match rt {
        RustType::Bool => quote! { bool },
        RustType::String | RustType::Decimal | RustType::Date | RustType::DateTime => {
            quote! { String }
        }
    }
}

fn constraint_doc(c: &Constraint) -> String {
    match c {
        Constraint::MinLength(n) => format!(" Minimum length: {n}"),
        Constraint::MaxLength(n) => format!(" Maximum length: {n}"),
        Constraint::Pattern(p) => format!(" Pattern: `{p}`"),
        Constraint::MinInclusive(v) => format!(" Minimum value (inclusive): {v}"),
        Constraint::MaxInclusive(v) => format!(" Maximum value (inclusive): {v}"),
        Constraint::TotalDigits(n) => format!(" Total digits: {n}"),
        Constraint::FractionDigits(n) => format!(" Fraction digits: {n}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{Constraint, NewtypeDef, RustType};

    #[test]
    fn newtype_string() {
        let def = NewtypeDef {
            name: "Max35Text".to_owned(),
            inner: RustType::String,
            constraints: vec![],
        };
        let ts = emit_newtype(&def);
        let src = ts.to_string();
        assert!(src.contains("pub struct Max35Text"), "src = {src}");
        assert!(src.contains("(pub String)"), "src = {src}");
        assert!(src.contains("transparent"), "src = {src}");
    }

    #[test]
    fn newtype_bool() {
        let def = NewtypeDef {
            name: "TrueFalseIndicator".to_owned(),
            inner: RustType::Bool,
            constraints: vec![],
        };
        let ts = emit_newtype(&def);
        let src = ts.to_string();
        assert!(src.contains("(pub bool)"), "src = {src}");
    }

    #[test]
    fn newtype_decimal_maps_to_string() {
        let def = NewtypeDef {
            name: "ImpliedCurrencyAndAmount".to_owned(),
            inner: RustType::Decimal,
            constraints: vec![],
        };
        let ts = emit_newtype(&def);
        let src = ts.to_string();
        assert!(src.contains("(pub String)"), "src = {src}");
    }

    #[test]
    fn newtype_with_constraints_emits_docs() {
        let def = NewtypeDef {
            name: "Max35Text".to_owned(),
            inner: RustType::String,
            constraints: vec![Constraint::MinLength(1), Constraint::MaxLength(35)],
        };
        let ts = emit_newtype(&def);
        let src = ts.to_string();
        assert!(src.contains("Minimum length"), "src = {src}");
        assert!(src.contains("Maximum length"), "src = {src}");
    }

    #[test]
    fn newtype_with_pattern_emits_doc() {
        let def = NewtypeDef {
            name: "BICFIDec2014Identifier".to_owned(),
            inner: RustType::String,
            constraints: vec![Constraint::Pattern(
                "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}".to_owned(),
            )],
        };
        let ts = emit_newtype(&def);
        let src = ts.to_string();
        assert!(src.contains("Pattern"), "src = {src}");
    }

    #[test]
    fn newtype_is_valid_rust() {
        let def = NewtypeDef {
            name: "Max35Text".to_owned(),
            inner: RustType::String,
            constraints: vec![Constraint::MinLength(1), Constraint::MaxLength(35)],
        };
        let ts = emit_newtype(&def);
        let src = ts.to_string();
        syn::parse_file(&src).expect("must be parseable Rust");
    }

    #[test]
    fn constrained_newtype_emits_try_from_new_from() {
        let def = NewtypeDef {
            name: "CountryCode".to_owned(),
            inner: RustType::String,
            constraints: vec![Constraint::Pattern("[A-Z]{2,2}".to_owned())],
        };
        let ts = emit_newtype(&def);
        let src = ts.to_string();
        assert!(
            src.contains("TryFrom < String >"),
            "should emit TryFrom: {src}"
        );
        assert!(src.contains("fn new"), "should emit new(): {src}");
        assert!(
            src.contains("impl From < CountryCode > for String"),
            "should emit From<T> for String: {src}"
        );
        syn::parse_file(&src).expect("must be parseable Rust");
    }

    #[test]
    fn unconstrained_newtype_no_try_from() {
        let def = NewtypeDef {
            name: "FreeText".to_owned(),
            inner: RustType::String,
            constraints: vec![],
        };
        let ts = emit_newtype(&def);
        let src = ts.to_string();
        assert!(
            !src.contains("TryFrom"),
            "unconstrained should NOT emit TryFrom: {src}"
        );
    }

    #[test]
    fn bool_newtype_no_try_from() {
        let def = NewtypeDef {
            name: "TrueFalseIndicator".to_owned(),
            inner: RustType::Bool,
            constraints: vec![Constraint::MinLength(1)], // nonsensical but tests the guard
        };
        let ts = emit_newtype(&def);
        let src = ts.to_string();
        assert!(
            !src.contains("TryFrom"),
            "bool inner should NOT emit TryFrom: {src}"
        );
    }

    #[test]
    fn pattern_only_newtype_emits_try_from() {
        let def = NewtypeDef {
            name: "ActiveCurrencyCode".to_owned(),
            inner: RustType::String,
            constraints: vec![Constraint::Pattern("[A-Z]{3,3}".to_owned())],
        };
        let ts = emit_newtype(&def);
        let src = ts.to_string();
        assert!(
            src.contains("TryFrom"),
            "pattern-only should emit TryFrom: {src}"
        );
        assert!(
            src.contains("ConstraintError"),
            "should reference ConstraintError: {src}"
        );
        syn::parse_file(&src).expect("must be parseable Rust");
    }

    #[test]
    fn constrained_newtype_with_length_is_valid_rust() {
        let def = NewtypeDef {
            name: "Max35Text".to_owned(),
            inner: RustType::String,
            constraints: vec![Constraint::MinLength(1), Constraint::MaxLength(35)],
        };
        let ts = emit_newtype(&def);
        let src = ts.to_string();
        assert!(src.contains("TryFrom"), "should emit TryFrom: {src}");
        syn::parse_file(&src).expect("must be parseable Rust");
    }

    #[test]
    fn decimal_with_digits_emits_try_from() {
        let def = NewtypeDef {
            name: "Amount".to_owned(),
            inner: RustType::Decimal,
            constraints: vec![Constraint::TotalDigits(18), Constraint::FractionDigits(5)],
        };
        let ts = emit_newtype(&def);
        let src = ts.to_string();
        assert!(src.contains("TryFrom"), "should emit TryFrom: {src}");
        syn::parse_file(&src).expect("must be parseable Rust");
    }
}
