//! Shared utilities for the emit submodules.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::ir::types::{Cardinality, RustType, TypeRef};

/// Rust keywords that require `r#` prefixing when used as identifiers.
///
/// This list covers all Rust keywords as of edition 2021.
const RUST_KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in",
    "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "try", "type",
    "typeof", "union", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// Create a `proc_macro2::Ident` from a string, using `r#` raw identifier
/// syntax if the name is a Rust keyword.
///
/// # Panics
///
/// Panics if `name` is empty or otherwise not a valid Rust identifier even
/// with raw-identifier escaping.
pub fn make_ident(name: &str) -> Ident {
    if RUST_KEYWORDS.contains(&name) {
        // raw_ident requires the name without the "r#" prefix.
        Ident::new_raw(name, Span::call_site())
    } else {
        Ident::new(name, Span::call_site())
    }
}

/// Resolve a [`TypeRef`] to its Rust type token stream.
///
/// - `TypeRef::Named(n)` → the identifier `n`
/// - `TypeRef::Builtin(RustType::String)` → `String`
/// - `TypeRef::Builtin(RustType::Bool)` → `bool`
/// - `TypeRef::Builtin(RustType::Decimal | Date | DateTime)` → `String`
pub fn type_ref_tokens(type_ref: &TypeRef) -> TokenStream {
    match type_ref {
        TypeRef::Named(name) => {
            let ident = make_ident(name);
            quote! { #ident }
        }
        TypeRef::Builtin(RustType::Bool) => quote! { bool },
        TypeRef::Builtin(
            RustType::String | RustType::Decimal | RustType::Date | RustType::DateTime,
        ) => {
            quote! { String }
        }
    }
}

/// Wrap a base type token stream with the appropriate cardinality container.
///
/// - `Required` → `T`
/// - `Optional` → `Option<T>`
/// - `Vec` → `Vec<T>`
/// - `BoundedVec(_)` → `Vec<T>`  (upper bound is emitted as a doc comment
///   by the caller)
pub fn apply_cardinality(base: TokenStream, cardinality: &Cardinality) -> TokenStream {
    match cardinality {
        Cardinality::Required => base,
        Cardinality::Optional => quote! { Option<#base> },
        Cardinality::Vec | Cardinality::BoundedVec(_) => quote! { Vec<#base> },
    }
}

/// Standard derive macro list used on all generated types.
pub fn standard_derives() -> TokenStream {
    quote! {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_ident_normal() {
        let id = make_ident("my_field");
        assert_eq!(id.to_string(), "my_field");
    }

    #[test]
    fn make_ident_keyword_type() {
        let id = make_ident("type");
        // Raw idents display as "r#type" in to_string().
        assert_eq!(id.to_string(), "r#type");
    }

    #[test]
    fn make_ident_keyword_ref() {
        let id = make_ident("ref");
        assert_eq!(id.to_string(), "r#ref");
    }

    #[test]
    fn type_ref_named() {
        let ts = type_ref_tokens(&TypeRef::Named("Max35Text".to_owned()));
        assert_eq!(ts.to_string(), "Max35Text");
    }

    #[test]
    fn type_ref_builtin_string() {
        let ts = type_ref_tokens(&TypeRef::Builtin(RustType::String));
        assert_eq!(ts.to_string(), "String");
    }

    #[test]
    fn type_ref_builtin_bool() {
        let ts = type_ref_tokens(&TypeRef::Builtin(RustType::Bool));
        assert_eq!(ts.to_string(), "bool");
    }

    #[test]
    fn type_ref_builtin_decimal_is_string() {
        let ts = type_ref_tokens(&TypeRef::Builtin(RustType::Decimal));
        assert_eq!(ts.to_string(), "String");
    }

    #[test]
    fn apply_cardinality_required() {
        let base = quote! { String };
        let ts = apply_cardinality(base, &Cardinality::Required);
        assert_eq!(ts.to_string(), "String");
    }

    #[test]
    fn apply_cardinality_optional() {
        let base = quote! { String };
        let ts = apply_cardinality(base, &Cardinality::Optional);
        assert_eq!(ts.to_string(), "Option < String >");
    }

    #[test]
    fn apply_cardinality_vec() {
        let base = quote! { String };
        let ts = apply_cardinality(base, &Cardinality::Vec);
        assert_eq!(ts.to_string(), "Vec < String >");
    }

    #[test]
    fn apply_cardinality_bounded_vec() {
        let base = quote! { String };
        let ts = apply_cardinality(base, &Cardinality::BoundedVec(10));
        assert_eq!(ts.to_string(), "Vec < String >");
    }
}
