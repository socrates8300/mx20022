//! XSD parsing module.
//!
//! Provides types and a streaming parser for the subset of XSD constructs used
//! in ISO 20022 schema files.
//!
//! # Quick start
//!
//! ```no_run
//! use std::{fs::File, io::BufReader};
//! use mx20022_codegen::xsd::parser::parse;
//!
//! let file = File::open("schemas/head/head.001.001.04.xsd").unwrap();
//! let schema = parse(BufReader::new(file)).unwrap();
//! println!("namespace: {}", schema.target_namespace);
//! println!("{} complex types", schema.complex_types.len());
//! ```

pub mod parser;
pub mod types;

pub use parser::{parse, parse_str, ParseError};
pub use types::{
    Attribute, ChoiceVariant, ComplexContent, ComplexType, Element, Facet, MaxOccurs, Restriction,
    Schema, SequenceElement, SimpleType,
};
