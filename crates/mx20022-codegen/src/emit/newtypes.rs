//! Emit [`NewtypeDef`] as a Rust newtype struct.

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::types::{Constraint, NewtypeDef, RustType};

use super::util::{make_ident, standard_derives};

/// Emit a single [`NewtypeDef`] as a `pub struct` newtype.
///
/// The generated struct is `#[serde(transparent)]` so it serializes as its
/// inner value. Constraints from XSD facets are preserved as doc comments.
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

    quote! {
        #constraint_docs
        #derives
        #[serde(transparent)]
        pub struct #name(pub #inner_type);
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
}
