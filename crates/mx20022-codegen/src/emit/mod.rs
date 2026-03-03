//! Code emission: IR [`TypeGraph`] → Rust source string.
//!
//! The main entry point is [`emit`], which takes a fully-lowered [`TypeGraph`]
//! and returns a formatted Rust source file as a `String`.
//!
//! # Pipeline
//!
//! 1. Each [`TypeDef`] variant is dispatched to the appropriate submodule.
//! 2. Each submodule returns a `proc_macro2::TokenStream`.
//! 3. All token streams are collected and formatted with `prettyplease`.
//!
//! # Example
//!
//! ```no_run
//! use mx20022_codegen::{xsd, ir, emit};
//!
//! let schema = xsd::parse_str(r#"<xs:schema
//!     xmlns:xs="http://www.w3.org/2001/XMLSchema"
//!     targetNamespace="urn:example">
//!   <xs:element name="Root" type="RootType"/>
//!   <xs:complexType name="RootType">
//!     <xs:sequence>
//!       <xs:element name="Id" type="xs:string"/>
//!     </xs:sequence>
//!   </xs:complexType>
//! </xs:schema>"#).unwrap();
//!
//! let graph = ir::lower(&schema).unwrap();
//! let code = emit::emit(&graph);
//! assert!(code.contains("pub struct RootType"));
//! ```

mod builders;
mod code_enums;
mod enums;
mod newtypes;
mod opaque;
mod structs;
mod validate;
mod value_with_attr;

pub(crate) mod util;

use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::types::{TypeDef, TypeGraph};

/// Emit a complete Rust source file from the given [`TypeGraph`].
///
/// The returned string is formatted with `prettyplease` and is guaranteed to
/// be parseable by `syn::parse_file`.
///
/// xs:choice types (IR [`crate::ir::types::EnumDef`]) that appear as struct
/// field values are automatically wrapped in
/// `crate::common::ChoiceWrapper<T>` so that `quick-xml` + serde can
/// round-trip them correctly.
///
/// # Panics
///
/// Panics if the generated token stream cannot be parsed by `syn`. This
/// indicates a bug in the emitter (not in user input).
pub fn emit(graph: &TypeGraph) -> String {
    // Build the set of xs:choice type names (all EnumDef names in the graph).
    let choice_types: HashSet<String> = graph
        .types
        .values()
        .filter_map(|t| {
            if let TypeDef::Enum(e) = t {
                Some(e.name.clone())
            } else {
                None
            }
        })
        .collect();

    let mut tokens = TokenStream::new();

    // Module-level doc comment with namespace.
    let ns = &graph.namespace;
    let doc = format!(" Generated from ISO 20022 XSD schema.\n Namespace: `{ns}`");
    tokens.extend(quote! {
        #![doc = #doc]
    });

    for type_def in graph.types.values() {
        let type_tokens = match type_def {
            TypeDef::Struct(d) => {
                let mut ts = structs::emit_struct(d, &choice_types);
                ts.extend(builders::emit_builder(d, &choice_types));
                ts
            }
            TypeDef::Enum(d) => enums::emit_enum(d),
            TypeDef::Newtype(d) => newtypes::emit_newtype(d),
            TypeDef::CodeEnum(d) => code_enums::emit_code_enum(d),
            TypeDef::ValueWithAttr(d) => value_with_attr::emit_value_with_attr(d),
            TypeDef::Opaque(d) => opaque::emit_opaque(d),
        };
        tokens.extend(type_tokens);
    }

    // Emit `impl Validatable` for all types and `impl IsoMessage` for
    // document types.
    tokens.extend(validate::emit_validatable_impls(graph, &choice_types));

    let file_str = tokens.to_string();
    let parsed = syn::parse_file(&file_str)
        .unwrap_or_else(|e| panic!("emitter produced invalid Rust (syn error: {e}):\n{file_str}"));
    prettyplease::unparse(&parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{
        AttrDef, Cardinality, CodeEnumDef, CodeValue, EnumDef, FieldDef, NewtypeDef, OpaqueDef,
        RootElement, RustType, StructDef, TypeDef, TypeGraph, TypeRef, ValueWithAttrDef,
        VariantDef,
    };
    use indexmap::IndexMap;

    fn make_graph(types: Vec<(String, TypeDef)>) -> TypeGraph {
        TypeGraph {
            namespace: "urn:test:namespace".to_owned(),
            root_elements: vec![RootElement {
                xml_name: "Root".to_owned(),
                type_name: "RootType".to_owned(),
            }],
            types: types.into_iter().collect::<IndexMap<_, _>>(),
        }
    }

    #[test]
    fn emit_empty_graph_is_valid_rust() {
        let graph = TypeGraph {
            namespace: "urn:empty".to_owned(),
            root_elements: vec![],
            types: IndexMap::new(),
        };
        let code = emit(&graph);
        syn::parse_file(&code).expect("must be valid Rust");
    }

    #[test]
    fn emit_simple_struct() {
        let graph = make_graph(vec![(
            "MyStruct".to_owned(),
            TypeDef::Struct(StructDef {
                name: "MyStruct".to_owned(),
                fields: vec![FieldDef {
                    xml_name: "Name".to_owned(),
                    rust_name: "name".to_owned(),
                    type_ref: TypeRef::Builtin(RustType::String),
                    cardinality: Cardinality::Required,
                }],
            }),
        )]);
        let code = emit(&graph);
        syn::parse_file(&code).expect("must be valid Rust");
        assert!(code.contains("pub struct MyStruct"));
        assert!(code.contains("pub name: String"));
    }

    #[test]
    fn emit_newtype() {
        let graph = make_graph(vec![(
            "Max35Text".to_owned(),
            TypeDef::Newtype(NewtypeDef {
                name: "Max35Text".to_owned(),
                inner: RustType::String,
                constraints: vec![],
            }),
        )]);
        let code = emit(&graph);
        syn::parse_file(&code).expect("must be valid Rust");
        assert!(code.contains("pub struct Max35Text"));
    }

    #[test]
    fn emit_code_enum() {
        let graph = make_graph(vec![(
            "AddressType2Code".to_owned(),
            TypeDef::CodeEnum(CodeEnumDef {
                name: "AddressType2Code".to_owned(),
                codes: vec![
                    CodeValue {
                        xml_value: "ADDR".to_owned(),
                        rust_name: "Addr".to_owned(),
                    },
                    CodeValue {
                        xml_value: "PBOX".to_owned(),
                        rust_name: "Pbox".to_owned(),
                    },
                ],
            }),
        )]);
        let code = emit(&graph);
        syn::parse_file(&code).expect("must be valid Rust");
        assert!(code.contains("pub enum AddressType2Code"));
        assert!(code.contains("Addr"));
        assert!(code.contains("Pbox"));
    }

    #[test]
    fn emit_choice_enum() {
        let graph = make_graph(vec![(
            "AddressType3Choice".to_owned(),
            TypeDef::Enum(EnumDef {
                name: "AddressType3Choice".to_owned(),
                variants: vec![
                    VariantDef {
                        xml_name: "Cd".to_owned(),
                        rust_name: "Cd".to_owned(),
                        type_ref: TypeRef::Named("AddressType2Code".to_owned()),
                    },
                    VariantDef {
                        xml_name: "Prtry".to_owned(),
                        rust_name: "Prtry".to_owned(),
                        type_ref: TypeRef::Named("GenericIdentification30".to_owned()),
                    },
                ],
            }),
        )]);
        let code = emit(&graph);
        syn::parse_file(&code).expect("must be valid Rust");
        assert!(code.contains("pub enum AddressType3Choice"));
    }

    #[test]
    fn emit_value_with_attr() {
        let graph = make_graph(vec![(
            "ActiveCurrencyAndAmount".to_owned(),
            TypeDef::ValueWithAttr(ValueWithAttrDef {
                name: "ActiveCurrencyAndAmount".to_owned(),
                value_type: TypeRef::Builtin(RustType::Decimal),
                attributes: vec![AttrDef {
                    xml_name: "Ccy".to_owned(),
                    rust_name: "ccy".to_owned(),
                    type_ref: TypeRef::Named("ActiveCurrencyCode".to_owned()),
                    required: true,
                }],
            }),
        )]);
        let code = emit(&graph);
        syn::parse_file(&code).expect("must be valid Rust");
        assert!(code.contains("pub struct ActiveCurrencyAndAmount"));
        assert!(code.contains("$value"));
        assert!(code.contains("@Ccy"));
    }

    #[test]
    fn emit_opaque() {
        let graph = make_graph(vec![(
            "SignatureEnvelope".to_owned(),
            TypeDef::Opaque(OpaqueDef {
                name: "SignatureEnvelope".to_owned(),
                namespace: Some("##other".to_owned()),
            }),
        )]);
        let code = emit(&graph);
        syn::parse_file(&code).expect("must be valid Rust");
        assert!(code.contains("pub struct SignatureEnvelope"));
    }

    #[test]
    fn emit_namespace_doc_comment() {
        let graph = TypeGraph {
            namespace: "urn:iso:std:iso:20022:tech:xsd:head.001.001.04".to_owned(),
            root_elements: vec![],
            types: IndexMap::new(),
        };
        let code = emit(&graph);
        assert!(code.contains("head.001.001.04"));
    }

    #[test]
    fn emit_head_001_e2e() {
        let xsd_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schemas/head/head.001.001.04.xsd"
        );
        let file = std::fs::File::open(xsd_path).expect("head.001.001.04.xsd not found");
        let schema = crate::xsd::parse(std::io::BufReader::new(file)).expect("XSD parse failed");
        let graph = crate::ir::lower(&schema).unwrap();
        let code = emit(&graph);

        // Must be parseable by syn (i.e., valid Rust).
        syn::parse_file(&code).expect("generated code must be valid Rust");

        // Must contain expected types.
        assert!(code.contains("pub struct BusinessApplicationHeaderV04"));
        assert!(code.contains("pub enum AddressType3Choice"));
        assert!(code.contains("pub enum AddressType2Code"));
        assert!(code.contains("pub struct Max35Text"));
        assert!(code.contains("pub struct SignatureEnvelope"));
    }
}
