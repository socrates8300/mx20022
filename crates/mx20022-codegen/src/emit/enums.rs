//! Emit [`EnumDef`] (choice types) as a Rust enum.

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::types::EnumDef;

use super::util::{make_ident, standard_derives, type_ref_tokens};

/// Emit a single [`EnumDef`] as a `pub enum` definition.
///
/// Each variant carries its inner type as a newtype-style tuple variant.
/// A `#[serde(rename = "XmlName")]` attribute is emitted for each variant
/// so that quick-xml can match the correct XML element name.
///
/// `#[allow(clippy::large_enum_variant)]` is emitted unconditionally because
/// ISO 20022 choice types often hold large structs; boxing the variants would
/// break the `quick-xml` serde mapping.
pub fn emit_enum(def: &EnumDef) -> TokenStream {
    let derives = standard_derives();
    let name = make_ident(&def.name);

    let variants: TokenStream = def
        .variants
        .iter()
        .map(|v| {
            let xml_name = &v.xml_name;
            let variant_name = make_ident(&v.rust_name);
            let inner_type = type_ref_tokens(&v.type_ref);
            quote! {
                #[serde(rename = #xml_name)]
                #variant_name(#inner_type),
            }
        })
        .collect();

    quote! {
        #[allow(clippy::large_enum_variant)]
        #derives
        pub enum #name {
            #variants
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{EnumDef, RustType, TypeRef, VariantDef};

    fn variant(xml_name: &str, rust_name: &str, type_ref: TypeRef) -> VariantDef {
        VariantDef {
            xml_name: xml_name.to_owned(),
            rust_name: rust_name.to_owned(),
            type_ref,
        }
    }

    #[test]
    fn enum_named_variants() {
        let def = EnumDef {
            name: "AddressType3Choice".to_owned(),
            variants: vec![
                variant("Cd", "Cd", TypeRef::Named("AddressType2Code".to_owned())),
                variant(
                    "Prtry",
                    "Prtry",
                    TypeRef::Named("GenericIdentification30".to_owned()),
                ),
            ],
        };
        let ts = emit_enum(&def);
        let src = ts.to_string();
        assert!(src.contains("pub enum AddressType3Choice"), "src = {src}");
        assert!(src.contains("\"Cd\""), "src = {src}");
        assert!(src.contains("\"Prtry\""), "src = {src}");
        assert!(src.contains("Cd (AddressType2Code)"), "src = {src}");
        assert!(
            src.contains("Prtry (GenericIdentification30)"),
            "src = {src}"
        );
    }

    #[test]
    fn enum_builtin_variant() {
        let def = EnumDef {
            name: "MyChoice".to_owned(),
            variants: vec![variant("Val", "Val", TypeRef::Builtin(RustType::String))],
        };
        let ts = emit_enum(&def);
        let src = ts.to_string();
        assert!(src.contains("Val (String)"), "src = {src}");
    }

    #[test]
    fn enum_is_valid_rust() {
        let def = EnumDef {
            name: "Party51Choice".to_owned(),
            variants: vec![
                variant(
                    "OrgId",
                    "OrgId",
                    TypeRef::Named("PartyIdentification272".to_owned()),
                ),
                variant(
                    "FIId",
                    "FIId",
                    TypeRef::Named("BranchAndFinancialInstitutionIdentification8".to_owned()),
                ),
            ],
        };
        let ts = emit_enum(&def);
        let src = ts.to_string();
        syn::parse_file(&src).expect("must be parseable Rust");
    }

    #[test]
    fn enum_serde_rename_on_each_variant() {
        let def = EnumDef {
            name: "MyEnum".to_owned(),
            variants: vec![
                variant("XmlName1", "Variant1", TypeRef::Builtin(RustType::String)),
                variant("XmlName2", "Variant2", TypeRef::Builtin(RustType::Bool)),
            ],
        };
        let ts = emit_enum(&def);
        let src = ts.to_string();
        assert!(src.contains("\"XmlName1\""), "src = {src}");
        assert!(src.contains("\"XmlName2\""), "src = {src}");
    }
}
