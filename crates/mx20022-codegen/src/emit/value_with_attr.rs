//! Emit [`ValueWithAttrDef`] as a Rust struct with a `$value` field.
//!
//! quick-xml uses `"$value"` as a magic rename for the text content of an
//! element, and `"@AttrName"` for attribute fields.

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::types::ValueWithAttrDef;

use super::util::{make_ident, standard_derives, type_ref_tokens};

/// Emit a single [`ValueWithAttrDef`] as a `pub struct`.
///
/// The struct has:
/// - A `value` field with `#[serde(rename = "$value")]` carrying the text
///   content of the XML element.
/// - One field per XML attribute with `#[serde(rename = "@AttrName")]`.
///   Optional attributes (`required = false`) also get
///   `#[serde(skip_serializing_if = "Option::is_none")]`.
pub fn emit_value_with_attr(def: &ValueWithAttrDef) -> TokenStream {
    let derives = standard_derives();
    let name = make_ident(&def.name);

    // The text-content field.
    let value_type = type_ref_tokens(&def.value_type);
    let value_field = quote! {
        #[serde(rename = "$value")]
        pub value: #value_type,
    };

    // Attribute fields.
    let attr_fields: TokenStream = def
        .attributes
        .iter()
        .map(|attr| {
            let serde_name = format!("@{}", attr.xml_name);
            let rust_name = make_ident(&attr.rust_name);
            let attr_type = type_ref_tokens(&attr.type_ref);

            if attr.required {
                quote! {
                    #[serde(rename = #serde_name)]
                    pub #rust_name: #attr_type,
                }
            } else {
                quote! {
                    #[serde(rename = #serde_name)]
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub #rust_name: Option<#attr_type>,
                }
            }
        })
        .collect();

    quote! {
        #derives
        pub struct #name {
            #value_field
            #attr_fields
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{AttrDef, RustType, TypeRef, ValueWithAttrDef};

    fn attr(xml_name: &str, rust_name: &str, type_ref: TypeRef, required: bool) -> AttrDef {
        AttrDef {
            xml_name: xml_name.to_owned(),
            rust_name: rust_name.to_owned(),
            type_ref,
            required,
        }
    }

    #[test]
    fn value_with_required_attr() {
        let def = ValueWithAttrDef {
            name: "ActiveCurrencyAndAmount".to_owned(),
            value_type: TypeRef::Builtin(RustType::Decimal),
            attributes: vec![attr(
                "Ccy",
                "ccy",
                TypeRef::Named("ActiveCurrencyCode".to_owned()),
                true,
            )],
        };
        let ts = emit_value_with_attr(&def);
        let src = ts.to_string();
        assert!(
            src.contains("pub struct ActiveCurrencyAndAmount"),
            "src = {src}"
        );
        assert!(src.contains("\"$value\""), "src = {src}");
        assert!(src.contains("\"@Ccy\""), "src = {src}");
        assert!(src.contains("pub ccy : ActiveCurrencyCode"), "src = {src}");
        // Required attr should NOT be wrapped in Option.
        assert!(
            !src.contains("Option < ActiveCurrencyCode >"),
            "src = {src}"
        );
    }

    #[test]
    fn value_with_optional_attr() {
        let def = ValueWithAttrDef {
            name: "AmountWithOptCcy".to_owned(),
            value_type: TypeRef::Builtin(RustType::Decimal),
            attributes: vec![attr(
                "Ccy",
                "ccy",
                TypeRef::Named("CurrencyCode".to_owned()),
                false,
            )],
        };
        let ts = emit_value_with_attr(&def);
        let src = ts.to_string();
        assert!(src.contains("Option < CurrencyCode >"), "src = {src}");
        assert!(src.contains("skip_serializing_if"), "src = {src}");
    }

    #[test]
    fn value_with_attr_is_valid_rust() {
        let def = ValueWithAttrDef {
            name: "ActiveCurrencyAndAmount".to_owned(),
            value_type: TypeRef::Builtin(RustType::Decimal),
            attributes: vec![attr(
                "Ccy",
                "ccy",
                TypeRef::Named("ActiveCurrencyCode".to_owned()),
                true,
            )],
        };
        let ts = emit_value_with_attr(&def);
        let src = ts.to_string();
        syn::parse_file(&src).expect("must be parseable Rust");
    }

    #[test]
    fn value_with_no_attrs() {
        let def = ValueWithAttrDef {
            name: "SimpleValue".to_owned(),
            value_type: TypeRef::Builtin(RustType::String),
            attributes: vec![],
        };
        let ts = emit_value_with_attr(&def);
        let src = ts.to_string();
        assert!(src.contains("\"$value\""), "src = {src}");
        syn::parse_file(&src).expect("must be parseable Rust");
    }

    #[test]
    fn value_with_multiple_attrs() {
        let def = ValueWithAttrDef {
            name: "ComplexValue".to_owned(),
            value_type: TypeRef::Builtin(RustType::String),
            attributes: vec![
                attr("Attr1", "attr1", TypeRef::Named("Type1".to_owned()), true),
                attr("Attr2", "attr2", TypeRef::Named("Type2".to_owned()), false),
            ],
        };
        let ts = emit_value_with_attr(&def);
        let src = ts.to_string();
        assert!(src.contains("\"@Attr1\""), "src = {src}");
        assert!(src.contains("\"@Attr2\""), "src = {src}");
        assert!(src.contains("pub attr1 : Type1"), "src = {src}");
        assert!(src.contains("Option < Type2 >"), "src = {src}");
        syn::parse_file(&src).expect("must be parseable Rust");
    }
}
