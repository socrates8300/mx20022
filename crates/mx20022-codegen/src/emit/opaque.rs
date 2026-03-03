//! Emit [`OpaqueDef`] as a Rust struct wrapping `String`.
//!
//! Types generated from `xs:any` content hold raw XML as a string value.
//! quick-xml's `"$value"` rename convention is used so the struct round-trips
//! correctly through serde.

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::types::OpaqueDef;

use super::util::{make_ident, standard_derives};

/// Emit a single [`OpaqueDef`] as a `pub struct` wrapping `String`.
///
/// If the XSD `<xs:any>` element specified a `namespace` attribute, it is
/// preserved in a doc comment.
pub fn emit_opaque(def: &OpaqueDef) -> TokenStream {
    let derives = standard_derives();
    let name = make_ident(&def.name);

    let namespace_doc: TokenStream = if let Some(ns) = &def.namespace {
        let doc = format!(" Accepts content from namespace: `{ns}`");
        quote! { #[doc = #doc] }
    } else {
        TokenStream::new()
    };

    quote! {
        #namespace_doc
        #derives
        pub struct #name {
            #[serde(rename = "$value")]
            pub value: String,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::OpaqueDef;

    #[test]
    fn opaque_with_namespace() {
        let def = OpaqueDef {
            name: "SignatureEnvelope".to_owned(),
            namespace: Some("##other".to_owned()),
        };
        let ts = emit_opaque(&def);
        let src = ts.to_string();
        assert!(src.contains("pub struct SignatureEnvelope"), "src = {src}");
        assert!(src.contains("\"$value\""), "src = {src}");
        assert!(src.contains("##other"), "src = {src}");
        assert!(src.contains("pub value : String"), "src = {src}");
    }

    #[test]
    fn opaque_without_namespace() {
        let def = OpaqueDef {
            name: "AnyContent".to_owned(),
            namespace: None,
        };
        let ts = emit_opaque(&def);
        let src = ts.to_string();
        assert!(src.contains("pub struct AnyContent"), "src = {src}");
        assert!(src.contains("\"$value\""), "src = {src}");
        // No namespace doc comment.
        assert!(!src.contains("namespace"), "src = {src}");
    }

    #[test]
    fn opaque_is_valid_rust() {
        let def = OpaqueDef {
            name: "SignatureEnvelope".to_owned(),
            namespace: Some("##other".to_owned()),
        };
        let ts = emit_opaque(&def);
        let src = ts.to_string();
        syn::parse_file(&src).expect("must be parseable Rust");
    }

    #[test]
    fn opaque_serde_derives() {
        let def = OpaqueDef {
            name: "RawXml".to_owned(),
            namespace: None,
        };
        let ts = emit_opaque(&def);
        let src = ts.to_string();
        assert!(src.contains("serde :: Serialize"), "src = {src}");
        assert!(src.contains("serde :: Deserialize"), "src = {src}");
    }
}
