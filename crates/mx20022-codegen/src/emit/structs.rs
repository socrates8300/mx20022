//! Emit [`StructDef`] as a Rust struct.

use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::types::{Cardinality, FieldDef, StructDef, TypeRef};

use super::util::{apply_cardinality, make_ident, standard_derives};

/// Emit a single [`StructDef`] as a `pub struct` definition.
///
/// `choice_types` is the set of type names that are xs:choice enums
/// ([`crate::ir::types::EnumDef`]).  Fields whose type is in this set are
/// emitted using [`mx20022_model::common::ChoiceWrapper`] so that
/// `quick-xml` + serde can round-trip them correctly.
///
/// # Serde annotations
///
/// - Per-field `#[serde(rename = "XmlName")]` is always emitted, because
///   XML element names follow no single renaming convention.
/// - `#[serde(skip_serializing_if = "Option::is_none")]` is added for
///   `Optional` fields.
/// - `#[serde(default)]` and `#[serde(skip_serializing_if = "Vec::is_empty")]`
///   are added for `Vec` / `BoundedVec` fields.
///
/// # Upper-bound doc comment
///
/// When a field uses `Cardinality::BoundedVec(n)`, a doc comment noting the
/// maximum is emitted before the field.
pub fn emit_struct(def: &StructDef, choice_types: &HashSet<String>) -> TokenStream {
    let derives = standard_derives();
    let name = make_ident(&def.name);

    let fields: TokenStream = def
        .fields
        .iter()
        .map(|f| emit_field(f, choice_types))
        .collect();

    quote! {
        #derives
        pub struct #name {
            #fields
        }
    }
}

fn is_choice(type_ref: &TypeRef, choice_types: &HashSet<String>) -> bool {
    match type_ref {
        TypeRef::Named(n) => choice_types.contains(n),
        TypeRef::Builtin(_) => false,
    }
}

fn emit_field(field: &FieldDef, choice_types: &HashSet<String>) -> TokenStream {
    let xml_name = &field.xml_name;
    let rust_name = make_ident(&field.rust_name);

    // Resolve the base Rust type token stream.  For xs:choice fields, wrap
    // the enum in `crate::common::ChoiceWrapper<T>` so that `quick-xml` can
    // serialize/deserialize newtype enum variants correctly.
    let base_type: TokenStream = if is_choice(&field.type_ref, choice_types) {
        let inner = super::util::type_ref_tokens(&field.type_ref);
        quote! { crate::common::ChoiceWrapper<#inner> }
    } else {
        super::util::type_ref_tokens(&field.type_ref)
    };

    let full_type = apply_cardinality(base_type, &field.cardinality);

    // Build serde attributes.
    let rename_attr = quote! { #[serde(rename = #xml_name)] };

    let mut extra_attrs = TokenStream::new();
    match &field.cardinality {
        Cardinality::Optional => {
            extra_attrs.extend(quote! {
                #[serde(skip_serializing_if = "Option::is_none")]
            });
        }
        Cardinality::Vec => {
            extra_attrs.extend(quote! {
                #[serde(default)]
                #[serde(skip_serializing_if = "Vec::is_empty")]
            });
        }
        Cardinality::BoundedVec(n) => {
            let doc = format!(" Maximum {n} occurrences.");
            extra_attrs.extend(quote! {
                #[doc = #doc]
                #[serde(default)]
                #[serde(skip_serializing_if = "Vec::is_empty")]
            });
        }
        Cardinality::Required => {}
    }

    quote! {
        #rename_attr
        #extra_attrs
        pub #rust_name: #full_type,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{
        Cardinality, EnumDef, FieldDef, RustType, StructDef, TypeRef, VariantDef,
    };

    fn field(
        xml_name: &str,
        rust_name: &str,
        type_ref: TypeRef,
        cardinality: Cardinality,
    ) -> FieldDef {
        FieldDef {
            xml_name: xml_name.to_owned(),
            rust_name: rust_name.to_owned(),
            type_ref,
            cardinality,
        }
    }

    fn empty_choices() -> HashSet<String> {
        HashSet::new()
    }

    fn choice_set(names: &[&str]) -> HashSet<String> {
        names.iter().map(|&s| s.to_owned()).collect()
    }

    #[test]
    fn struct_required_field() {
        let def = StructDef {
            name: "MyStruct".to_owned(),
            fields: vec![field(
                "BizMsgIdr",
                "biz_msg_idr",
                TypeRef::Named("Max35Text".to_owned()),
                Cardinality::Required,
            )],
        };
        let ts = emit_struct(&def, &empty_choices());
        let src = ts.to_string();
        assert!(src.contains("pub struct MyStruct"));
        assert!(src.contains("pub biz_msg_idr : Max35Text"));
        assert!(src.contains("\"BizMsgIdr\""));
    }

    #[test]
    fn struct_optional_field() {
        let def = StructDef {
            name: "MyStruct".to_owned(),
            fields: vec![field(
                "Dt",
                "dt",
                TypeRef::Builtin(RustType::String),
                Cardinality::Optional,
            )],
        };
        let ts = emit_struct(&def, &empty_choices());
        let src = ts.to_string();
        assert!(src.contains("Option < String >"), "src = {src}");
        assert!(src.contains("skip_serializing_if"));
    }

    #[test]
    fn struct_vec_field() {
        let def = StructDef {
            name: "MyStruct".to_owned(),
            fields: vec![field(
                "Items",
                "items",
                TypeRef::Named("ItemType".to_owned()),
                Cardinality::Vec,
            )],
        };
        let ts = emit_struct(&def, &empty_choices());
        let src = ts.to_string();
        assert!(src.contains("Vec < ItemType >"), "src = {src}");
        assert!(src.contains("Vec::is_empty"));
        assert!(src.contains("default"));
    }

    #[test]
    fn struct_bounded_vec_field() {
        let def = StructDef {
            name: "MyStruct".to_owned(),
            fields: vec![field(
                "Items",
                "items",
                TypeRef::Named("ItemType".to_owned()),
                Cardinality::BoundedVec(5),
            )],
        };
        let ts = emit_struct(&def, &empty_choices());
        let src = ts.to_string();
        assert!(src.contains("Vec < ItemType >"), "src = {src}");
        // Doc comment about the upper bound.
        assert!(src.contains("5"), "src = {src}");
    }

    #[test]
    fn struct_keyword_field_name() {
        // Field named "type" (a Rust keyword) must be emitted as r#type.
        let def = StructDef {
            name: "MyStruct".to_owned(),
            fields: vec![field(
                "Type",
                "type",
                TypeRef::Builtin(RustType::String),
                Cardinality::Required,
            )],
        };
        let ts = emit_struct(&def, &empty_choices());
        let src = ts.to_string();
        // proc_macro2 formats raw idents as "r#type".
        assert!(src.contains("r#type"), "src = {src}");
    }

    #[test]
    fn struct_bool_field() {
        let def = StructDef {
            name: "MyStruct".to_owned(),
            fields: vec![field(
                "Active",
                "active",
                TypeRef::Builtin(RustType::Bool),
                Cardinality::Required,
            )],
        };
        let ts = emit_struct(&def, &empty_choices());
        let src = ts.to_string();
        assert!(src.contains("pub active : bool"), "src = {src}");
    }

    #[test]
    fn struct_empty_fields_is_valid_rust() {
        let def = StructDef {
            name: "EmptyStruct".to_owned(),
            fields: vec![],
        };
        let ts = emit_struct(&def, &empty_choices());
        let file_str = ts.to_string();
        syn::parse_file(&file_str).expect("must be parseable Rust");
    }

    #[test]
    fn struct_choice_field_uses_choice_wrapper() {
        // Set up a choice type named "Party51Choice"
        let def = StructDef {
            name: "BusinessApplicationHeader".to_owned(),
            fields: vec![field(
                "Fr",
                "fr",
                TypeRef::Named("Party51Choice".to_owned()),
                Cardinality::Required,
            )],
        };
        let choices = choice_set(&["Party51Choice"]);
        let ts = emit_struct(&def, &choices);
        let src = ts.to_string();
        // Should contain ChoiceWrapper<Party51Choice>
        assert!(
            src.contains("ChoiceWrapper < Party51Choice >"),
            "xs:choice field must use ChoiceWrapper; src = {src}"
        );
        assert!(
            !src.contains("pub fr : Party51Choice"),
            "bare enum must not appear; src = {src}"
        );
    }

    #[test]
    fn struct_optional_choice_field() {
        let def = StructDef {
            name: "Foo".to_owned(),
            fields: vec![field(
                "Choice",
                "choice",
                TypeRef::Named("MyChoice".to_owned()),
                Cardinality::Optional,
            )],
        };
        let choices = choice_set(&["MyChoice"]);
        let ts = emit_struct(&def, &choices);
        let src = ts.to_string();
        assert!(
            src.contains("Option < crate :: common :: ChoiceWrapper < MyChoice > >"),
            "optional xs:choice field must be Option<ChoiceWrapper<T>>; src = {src}"
        );
    }

    #[allow(dead_code)]
    fn _make_enum_def() -> EnumDef {
        EnumDef {
            name: "Party51Choice".to_owned(),
            variants: vec![VariantDef {
                xml_name: "FIId".to_owned(),
                rust_name: "FIId".to_owned(),
                type_ref: TypeRef::Named("SomeType".to_owned()),
            }],
        }
    }
}
