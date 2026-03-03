//! XSD to Rust code generator for ISO 20022 message types.
//!
//! This crate reads ISO 20022 XSD schema files and generates idiomatic Rust
//! type definitions suitable for serialization with `quick-xml` and `serde`.
//!
//! # Architecture
//!
//! 1. **`xsd`** — Parse XSD files into an AST ([`xsd::Schema`]).
//! 2. **`ir`** — Lower the AST into a typed intermediate representation
//!    ([`ir::TypeGraph`]).
//! 3. **`emit`** — Render the IR to formatted Rust source code.
//!
//! # Quick start
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
//! let rust_source = emit::emit(&graph);
//! println!("{rust_source}");
//! ```

pub mod emit;
pub mod ir;
pub mod xsd;
