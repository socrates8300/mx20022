//! XSD Abstract Syntax Tree types.
//!
//! These types represent the subset of XSD constructs found in ISO 20022 schemas.
//! The design is intentionally narrow: only the patterns that actually appear in
//! the `head`, `pacs`, `pain`, and `camt` schema families are modelled.

/// A parsed XSD schema file.
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    /// The `targetNamespace` attribute of the `<xs:schema>` element.
    pub target_namespace: String,
    /// Top-level `<xs:element>` declarations.
    pub elements: Vec<Element>,
    /// Top-level `<xs:simpleType>` definitions.
    pub simple_types: Vec<SimpleType>,
    /// Top-level `<xs:complexType>` definitions.
    pub complex_types: Vec<ComplexType>,
}

/// A top-level element declaration: `<xs:element name="…" type="…"/>`.
#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    /// The element name (e.g. `"Document"`, `"AppHdr"`).
    pub name: String,
    /// The declared type name (e.g. `"Document"`, `"BusinessApplicationHeaderV04"`).
    pub type_name: String,
}

/// A `<xs:simpleType>` definition, always restriction-based in ISO 20022 schemas.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleType {
    /// The type name.
    pub name: String,
    /// The restriction that defines the type's value space.
    pub restriction: Restriction,
}

/// The `<xs:restriction>` element inside a `<xs:simpleType>`.
#[derive(Debug, Clone, PartialEq)]
pub struct Restriction {
    /// The `base` attribute (e.g. `"xs:string"`, `"xs:decimal"`, `"xs:boolean"`).
    pub base: String,
    /// Zero or more constraining facets.
    pub facets: Vec<Facet>,
}

/// A single XSD constraining facet.
#[derive(Debug, Clone, PartialEq)]
pub enum Facet {
    /// `<xs:enumeration value="…"/>`
    Enumeration(String),
    /// `<xs:pattern value="…"/>`
    Pattern(String),
    /// `<xs:minLength value="…"/>`
    MinLength(u64),
    /// `<xs:maxLength value="…"/>`
    MaxLength(u64),
    /// `<xs:minInclusive value="…"/>`
    MinInclusive(String),
    /// `<xs:maxInclusive value="…"/>`
    MaxInclusive(String),
    /// `<xs:totalDigits value="…"/>`
    TotalDigits(u32),
    /// `<xs:fractionDigits value="…"/>`
    FractionDigits(u32),
}

/// A `<xs:complexType>` definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexType {
    /// The type name.
    pub name: String,
    /// The content model of the complex type.
    pub content: ComplexContent,
}

/// The content model of a complex type.
#[derive(Debug, Clone, PartialEq)]
pub enum ComplexContent {
    /// `<xs:sequence>` — maps to a struct with ordered fields.
    Sequence(Vec<SequenceElement>),
    /// `<xs:choice>` — maps to a Rust enum.
    Choice(Vec<ChoiceVariant>),
    /// `<xs:simpleContent><xs:extension base="…">` — a value type with attributes.
    SimpleContent {
        /// The base type referenced in `<xs:extension base="…">`.
        base: String,
        /// Attributes declared inside the extension.
        attributes: Vec<Attribute>,
    },
    /// `<xs:any>` wildcard or empty body — used for opaque wrapper types.
    Any {
        /// The `namespace` attribute of `<xs:any>`, if present.
        namespace: Option<String>,
    },
}

/// An element inside a `<xs:sequence>`.
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceElement {
    /// The element name.
    pub name: String,
    /// The element type name.
    pub type_name: String,
    /// Minimum occurrences (default `1`).
    pub min_occurs: u32,
    /// Maximum occurrences (default `Bounded(1)`).
    pub max_occurs: MaxOccurs,
}

/// The `maxOccurs` attribute on a sequence element.
#[derive(Debug, Clone, PartialEq)]
pub enum MaxOccurs {
    /// A finite upper bound.
    Bounded(u32),
    /// `maxOccurs="unbounded"`.
    Unbounded,
}

/// A variant inside a `<xs:choice>`.
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceVariant {
    /// The element name used as the variant label.
    pub name: String,
    /// The element type name.
    pub type_name: String,
}

/// An `<xs:attribute>` declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    /// The attribute name.
    pub name: String,
    /// The attribute type name.
    pub type_name: String,
    /// Whether `use="required"` was specified.
    pub required: bool,
}
