//! Intermediate representation (IR) for ISO 20022 schema types.
//!
//! The IR is the bridge between the raw XSD AST produced by [`crate::xsd`]
//! and the Rust token stream emitted by [`crate::emit`].  It is deliberately
//! closer to Rust than to XSD, making the emitter simple.
//!
//! # Entry points
//!
//! - [`lower::lower`] — convert an [`xsd::Schema`] into a [`TypeGraph`].
//! - [`TypeGraph`], [`TypeDef`] and related types — the IR data model.
//!
//! [`xsd::Schema`]: crate::xsd::Schema

pub mod lower;
pub mod types;

pub use lower::{lower, LowerError};
pub use types::{
    AttrDef, Cardinality, CodeEnumDef, CodeValue, Constraint, EnumDef, FieldDef, NewtypeDef,
    OpaqueDef, RootElement, RustType, StructDef, TypeDef, TypeGraph, TypeRef, ValueWithAttrDef,
    VariantDef,
};
