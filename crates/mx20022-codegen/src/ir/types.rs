//! Intermediate representation types.
//!
//! The IR sits between the raw XSD AST ([`crate::xsd::Schema`]) and the final
//! emitted Rust token stream.  It is closer to Rust than XSD: XML names have
//! been mapped to Rust identifiers, cardinality has been resolved into
//! [`Cardinality`] variants, and built-in XSD types have been mapped to
//! [`RustType`].
//!
//! # Entry point
//!
//! See [`TypeGraph`] for the top-level structure produced by
//! [`crate::ir::lower::lower`].

use indexmap::IndexMap;

/// The complete intermediate representation of a single XSD schema.
///
/// # Example
///
/// ```no_run
/// use mx20022_codegen::{xsd, ir};
///
/// let schema = xsd::parse_str(r#"<xs:schema
///     xmlns:xs="http://www.w3.org/2001/XMLSchema"
///     targetNamespace="urn:example">
///   <xs:element name="Root" type="RootType"/>
///   <xs:complexType name="RootType">
///     <xs:sequence>
///       <xs:element name="Id" type="xs:string"/>
///     </xs:sequence>
///   </xs:complexType>
/// </xs:schema>"#).unwrap();
///
/// let graph = ir::lower::lower(&schema).unwrap();
/// assert_eq!(graph.root_elements.len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TypeGraph {
    /// The `targetNamespace` from the XSD schema element.
    pub namespace: String,
    /// Top-level document element declarations (e.g. `<xs:element name="AppHdr" …/>`).
    pub root_elements: Vec<RootElement>,
    /// All type definitions keyed by their XSD/Rust name, in insertion order.
    pub types: IndexMap<String, TypeDef>,
}

/// A top-level element declaration referencing a named type.
#[derive(Debug, Clone, PartialEq)]
pub struct RootElement {
    /// The XML element name (e.g. `"AppHdr"`).
    pub xml_name: String,
    /// The type name the element uses (e.g. `"BusinessApplicationHeaderV04"`).
    pub type_name: String,
}

/// A single type definition in the IR.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeDef {
    /// A struct with named fields — from `xs:complexType` with `xs:sequence`.
    Struct(StructDef),
    /// An enum with typed variants — from `xs:complexType` with `xs:choice`.
    Enum(EnumDef),
    /// A newtype wrapping a built-in Rust type — from `xs:simpleType` with
    /// non-enumeration facets (pattern, length, etc.).
    Newtype(NewtypeDef),
    /// A string-valued code enum — from `xs:simpleType` whose restriction
    /// contains only `xs:enumeration` facets.
    CodeEnum(CodeEnumDef),
    /// A value with XML attributes — from `xs:complexType` with
    /// `xs:simpleContent` / `xs:extension`.
    ValueWithAttr(ValueWithAttrDef),
    /// An opaque wrapper for `xs:any` content.
    Opaque(OpaqueDef),
}

impl TypeDef {
    /// Return the name of this type definition.
    pub fn name(&self) -> &str {
        match self {
            TypeDef::Struct(d) => &d.name,
            TypeDef::Enum(d) => &d.name,
            TypeDef::Newtype(d) => &d.name,
            TypeDef::CodeEnum(d) => &d.name,
            TypeDef::ValueWithAttr(d) => &d.name,
            TypeDef::Opaque(d) => &d.name,
        }
    }
}

// ── Struct ────────────────────────────────────────────────────────────────────

/// A struct generated from an `xs:complexType` with an `xs:sequence`.
#[derive(Debug, Clone, PartialEq)]
pub struct StructDef {
    /// The Rust struct name (identical to the XSD type name; already `PascalCase`).
    pub name: String,
    /// Ordered list of struct fields.
    pub fields: Vec<FieldDef>,
}

/// A single field inside a [`StructDef`].
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDef {
    /// Original XML element name (e.g. `"BizMsgIdr"`).
    pub xml_name: String,
    /// `snake_case` Rust field name (e.g. `"biz_msg_idr"`).
    pub rust_name: String,
    /// Resolved type reference.
    pub type_ref: TypeRef,
    /// Field cardinality, derived from `minOccurs`/`maxOccurs`.
    pub cardinality: Cardinality,
}

/// How many times a field may appear.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cardinality {
    /// `minOccurs=1`, `maxOccurs=1` — plain field.
    Required,
    /// `minOccurs=0`, `maxOccurs=1` — `Option<T>`.
    Optional,
    /// `maxOccurs="unbounded"` — `Vec<T>`.
    Vec,
    /// `maxOccurs=N` where `N > 1` — `Vec<T>` with a documented upper bound.
    BoundedVec(u32),
}

// ── Enum ──────────────────────────────────────────────────────────────────────

/// An enum generated from an `xs:complexType` with an `xs:choice`.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDef {
    /// The Rust enum name.
    pub name: String,
    /// Ordered list of enum variants.
    pub variants: Vec<VariantDef>,
}

/// A single variant inside an [`EnumDef`].
#[derive(Debug, Clone, PartialEq)]
pub struct VariantDef {
    /// Original XML element name used as the variant label (e.g. `"OrgId"`).
    pub xml_name: String,
    /// Rust variant name — kept as `PascalCase` since that is already idiomatic
    /// for enum variants (e.g. `"OrgId"`).
    pub rust_name: String,
    /// The inner type carried by this variant.
    pub type_ref: TypeRef,
}

// ── Newtype ───────────────────────────────────────────────────────────────────

/// A newtype wrapping a primitive type with optional validation constraints.
///
/// Generated from `xs:simpleType` restrictions that do **not** consist solely
/// of `xs:enumeration` facets.
#[derive(Debug, Clone, PartialEq)]
pub struct NewtypeDef {
    /// The Rust struct name.
    pub name: String,
    /// The wrapped primitive Rust type.
    pub inner: RustType,
    /// Zero or more constraints derived from XSD facets.
    pub constraints: Vec<Constraint>,
}

// ── CodeEnum ──────────────────────────────────────────────────────────────────

/// A string-code enum generated from `xs:simpleType` with `xs:enumeration`
/// facets.
///
/// # Example
///
/// XSD `AddressType2Code` with values `"ADDR"`, `"PBOX"`, … becomes:
/// ```text
/// pub enum AddressType2Code { Addr, Pbox, … }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CodeEnumDef {
    /// The Rust enum name.
    pub name: String,
    /// The enumerated code values.
    pub codes: Vec<CodeValue>,
}

/// A single code value within a [`CodeEnumDef`].
#[derive(Debug, Clone, PartialEq)]
pub struct CodeValue {
    /// The raw XML string value (e.g. `"ADDR"`).
    pub xml_value: String,
    /// The Rust variant name in `PascalCase` (e.g. `"Addr"`).
    pub rust_name: String,
}

// ── ValueWithAttr ─────────────────────────────────────────────────────────────

/// A value type that also carries XML attributes.
///
/// Generated from `xs:complexType` with `xs:simpleContent` / `xs:extension`.
#[derive(Debug, Clone, PartialEq)]
pub struct ValueWithAttrDef {
    /// The Rust struct name.
    pub name: String,
    /// The base value type (the `xs:extension base` resolved to an IR ref).
    pub value_type: TypeRef,
    /// XML attributes on this element.
    pub attributes: Vec<AttrDef>,
}

/// A single XML attribute inside a [`ValueWithAttrDef`].
#[derive(Debug, Clone, PartialEq)]
pub struct AttrDef {
    /// Original XML attribute name (e.g. `"Ccy"`).
    pub xml_name: String,
    /// `snake_case` Rust field name (e.g. `"ccy"`).
    pub rust_name: String,
    /// Resolved type reference.
    pub type_ref: TypeRef,
    /// Whether `use="required"` was specified on the attribute.
    pub required: bool,
}

// ── Opaque ────────────────────────────────────────────────────────────────────

/// An opaque wrapper generated from `xs:complexType` with `xs:any`.
#[derive(Debug, Clone, PartialEq)]
pub struct OpaqueDef {
    /// The Rust struct name.
    pub name: String,
    /// The `namespace` attribute of `<xs:any>`, if present.
    pub namespace: Option<String>,
}

// ── Shared references and primitives ─────────────────────────────────────────

/// A reference to a type in the IR.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeRef {
    /// Reference to another named type in the [`TypeGraph`].
    Named(String),
    /// A built-in primitive Rust type.
    Builtin(RustType),
}

/// Primitive Rust types that XSD built-in types map to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustType {
    /// `xs:string`, `xs:normalizedString`, `xs:token`, etc. → `String`
    String,
    /// `xs:boolean` → `bool`
    Bool,
    /// `xs:decimal` → stored as `String` for now; can be refined to
    /// `rust_decimal::Decimal` later.
    Decimal,
    /// `xs:date` → stored as `String`.
    Date,
    /// `xs:dateTime` → stored as `String`.
    DateTime,
}

/// A constraint derived from an XSD facet applied to a [`NewtypeDef`].
#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    /// `xs:minLength`
    MinLength(u64),
    /// `xs:maxLength`
    MaxLength(u64),
    /// `xs:pattern`
    Pattern(String),
    /// `xs:minInclusive`
    MinInclusive(String),
    /// `xs:maxInclusive`
    MaxInclusive(String),
    /// `xs:totalDigits`
    TotalDigits(u32),
    /// `xs:fractionDigits`
    FractionDigits(u32),
}
