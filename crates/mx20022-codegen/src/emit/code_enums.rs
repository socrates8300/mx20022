//! Emit [`CodeEnumDef`] as a string-valued Rust enum.

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::types::CodeEnumDef;

use super::util::{make_ident, standard_derives};

/// Emit a single [`CodeEnumDef`] as a `pub enum` with unit variants.
///
/// Each variant carries a `#[serde(rename = "XML_VALUE")]` attribute so that
/// serde serializes/deserializes the raw XSD code string (e.g. `"ADDR"`).
pub fn emit_code_enum(def: &CodeEnumDef) -> TokenStream {
    let derives = standard_derives();
    let name = make_ident(&def.name);

    let variants: TokenStream = def
        .codes
        .iter()
        .map(|code| {
            let xml_value = &code.xml_value;
            let variant_name = make_ident(&code.rust_name);
            quote! {
                #[serde(rename = #xml_value)]
                #variant_name,
            }
        })
        .collect();

    quote! {
        #derives
        pub enum #name {
            #variants
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{CodeEnumDef, CodeValue};

    fn code(xml_value: &str, rust_name: &str) -> CodeValue {
        CodeValue {
            xml_value: xml_value.to_owned(),
            rust_name: rust_name.to_owned(),
        }
    }

    #[test]
    fn code_enum_basic() {
        let def = CodeEnumDef {
            name: "AddressType2Code".to_owned(),
            codes: vec![
                code("ADDR", "Addr"),
                code("PBOX", "Pbox"),
                code("HOME", "Home"),
                code("BIZZ", "Bizz"),
                code("MLTO", "Mlto"),
                code("DLVY", "Dlvy"),
            ],
        };
        let ts = emit_code_enum(&def);
        let src = ts.to_string();
        assert!(src.contains("pub enum AddressType2Code"), "src = {src}");
        assert!(src.contains("\"ADDR\""), "src = {src}");
        assert!(src.contains("\"PBOX\""), "src = {src}");
        assert!(src.contains("Addr ,"), "src = {src}");
        assert!(src.contains("Pbox ,"), "src = {src}");
    }

    #[test]
    fn code_enum_is_valid_rust() {
        let def = CodeEnumDef {
            name: "Priority2Code".to_owned(),
            codes: vec![code("HIGH", "High"), code("NORM", "Norm")],
        };
        let ts = emit_code_enum(&def);
        let src = ts.to_string();
        syn::parse_file(&src).expect("must be parseable Rust");
    }

    #[test]
    fn code_enum_single_variant() {
        let def = CodeEnumDef {
            name: "SingleCode".to_owned(),
            codes: vec![code("ONLY", "Only")],
        };
        let ts = emit_code_enum(&def);
        let src = ts.to_string();
        assert!(src.contains("Only"), "src = {src}");
        assert!(src.contains("\"ONLY\""), "src = {src}");
    }

    #[test]
    fn code_enum_serde_derives() {
        let def = CodeEnumDef {
            name: "MyCode".to_owned(),
            codes: vec![code("VAL", "Val")],
        };
        let ts = emit_code_enum(&def);
        let src = ts.to_string();
        assert!(src.contains("serde :: Serialize"), "src = {src}");
        assert!(src.contains("serde :: Deserialize"), "src = {src}");
    }
}
