//! XSD parser built on [`quick_xml`] streaming events.
//!
//! The parser drives a simple recursive-descent state machine over the flat
//! event stream produced by [`quick_xml::Reader`].  No DOM tree is built; each
//! recognised element is accumulated directly into the [`Schema`] being
//! constructed.
//!
//! # Supported constructs
//!
//! | XSD construct | Rust mapping |
//! |---|---|
//! | `<xs:element name="…" type="…"/>` (top-level) | [`Element`] |
//! | `<xs:simpleType>` + `<xs:restriction>` | [`SimpleType`] / [`Restriction`] |
//! | Facets: `enumeration`, `pattern`, `minLength`, `maxLength`, `minInclusive`, `maxInclusive`, `totalDigits`, `fractionDigits` | [`Facet`] variants |
//! | `<xs:complexType>` + `<xs:sequence>` | `ComplexContent::Sequence` |
//! | `<xs:complexType>` + `<xs:choice>` | `ComplexContent::Choice` |
//! | `<xs:complexType>` + `<xs:simpleContent><xs:extension>` | `ComplexContent::SimpleContent` |
//! | `<xs:complexType>` + `<xs:any>` | `ComplexContent::Any` |

use std::io::BufRead;

use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};

use super::types::{
    Attribute, ChoiceVariant, ComplexContent, ComplexType, Element, Facet, MaxOccurs, Restriction,
    Schema, SequenceElement, SimpleType,
};

/// Errors that can occur while parsing an XSD file.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// An I/O or encoding error from quick-xml.
    #[error("xml error: {0}")]
    Xml(#[from] quick_xml::Error),

    /// A required XML attribute was missing from an element.
    #[error("missing required attribute '{attr}' on <{element}>")]
    MissingAttribute {
        /// The XSD element local name.
        element: &'static str,
        /// The attribute name that was expected.
        attr: &'static str,
    },

    /// An attribute value could not be parsed as the expected type.
    #[error("invalid value '{value}' for attribute '{attr}': {reason}")]
    InvalidAttributeValue {
        /// The raw attribute value.
        value: String,
        /// The attribute name.
        attr: &'static str,
        /// Human-readable explanation.
        reason: &'static str,
    },

    /// The `<xs:schema>` root element was not found.
    #[error("missing <xs:schema> root element")]
    MissingSchemaRoot,

    /// UTF-8 decoding failure for an attribute value.
    #[error("utf-8 error in attribute value: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

// ---------------------------------------------------------------------------
// Internal state machine types
// ---------------------------------------------------------------------------

/// High-level parser context — what the parser is currently building.
#[derive(Debug)]
enum Context {
    /// At the top level of `<xs:schema>`.
    Schema,
    /// Inside a `<xs:simpleType>`.
    SimpleType(SimpleTypeBuilder),
    /// Inside a `<xs:complexType>`.
    ComplexType(ComplexTypeBuilder),
}

/// Builder state for a `<xs:simpleType>` definition.
///
/// `nesting_depth` counts how many Start events have been seen since we entered
/// the `<xs:simpleType>` element.  The matching `</xs:simpleType>` End event is
/// recognised by `nesting_depth` reaching 0.
#[derive(Debug, Default)]
struct SimpleTypeBuilder {
    name: String,
    base: String,
    facets: Vec<Facet>,
    /// Number of open child Start tags inside this type context.
    nesting_depth: u32,
}

/// Builder state for a `<xs:complexType>` definition.
///
/// Same depth-tracking convention as [`SimpleTypeBuilder`].
#[derive(Debug, Default)]
struct ComplexTypeBuilder {
    name: String,
    content: ComplexContentBuilder,
    /// Number of open child Start tags inside this type context.
    nesting_depth: u32,
}

/// The content model being assembled inside a [`ComplexTypeBuilder`].
#[derive(Debug, Default)]
enum ComplexContentBuilder {
    #[default]
    Empty,
    Sequence(Vec<SequenceElement>),
    Choice(Vec<ChoiceVariant>),
    SimpleContent {
        base: String,
        attributes: Vec<Attribute>,
        /// True once the `<xs:extension>` open-tag has been seen.
        in_extension: bool,
    },
    Any {
        namespace: Option<String>,
    },
}

// ---------------------------------------------------------------------------
// Public entry points
// ---------------------------------------------------------------------------

/// Parse an XSD schema from any [`BufRead`] source.
///
/// # Errors
///
/// Returns [`ParseError`] if the XML is malformed or a required XSD attribute
/// is missing.
///
/// # Example
///
/// ```no_run
/// use std::{fs::File, io::BufReader};
/// use mx20022_codegen::xsd::parser::parse;
///
/// let file = File::open("schemas/head/head.001.001.04.xsd").unwrap();
/// let schema = parse(BufReader::new(file)).unwrap();
/// println!("namespace: {}", schema.target_namespace);
/// ```
pub fn parse<R: BufRead>(reader: R) -> Result<Schema, ParseError> {
    parse_internal(Reader::from_reader(reader))
}

/// Parse an XSD schema from a string slice.
///
/// Primarily useful in tests.
///
/// # Errors
///
/// Returns [`ParseError`] on malformed XML or missing required attributes.
pub fn parse_str(xml: &str) -> Result<Schema, ParseError> {
    parse_internal(Reader::from_str(xml))
}

// ---------------------------------------------------------------------------
// Core parser loop
// ---------------------------------------------------------------------------

fn parse_internal<R: BufRead>(mut reader: Reader<R>) -> Result<Schema, ParseError> {
    reader.config_mut().trim_text(true);

    let mut schema = Schema {
        target_namespace: String::new(),
        elements: Vec::new(),
        simple_types: Vec::new(),
        complex_types: Vec::new(),
    };

    let mut buf = Vec::new();
    let mut ctx_stack: Vec<Context> = Vec::new();
    let mut found_schema_root = false;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => {
                on_start(e, &mut ctx_stack, &mut schema, &mut found_schema_root)?;
            }
            Event::Empty(ref e) => {
                on_empty(e, &mut ctx_stack, &mut schema, &mut found_schema_root)?;
            }
            Event::End(_) => {
                on_end(&mut ctx_stack, &mut schema);
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    if !found_schema_root {
        return Err(ParseError::MissingSchemaRoot);
    }

    Ok(schema)
}

// ---------------------------------------------------------------------------
// Event: Start (opening tag with children)
// ---------------------------------------------------------------------------

fn on_start(
    e: &BytesStart<'_>,
    ctx_stack: &mut Vec<Context>,
    schema: &mut Schema,
    found_schema_root: &mut bool,
) -> Result<(), ParseError> {
    let name_binding = e.name();
    let local = local_name(name_binding.as_ref());

    match ctx_stack.last_mut() {
        // ---- No context yet (or schema root already open) ----
        None => {
            if local == "schema" {
                *found_schema_root = true;
                schema.target_namespace = attr_value(e, "targetNamespace")?.unwrap_or_default();
                ctx_stack.push(Context::Schema);
            }
            // Everything else at the root level before <xs:schema> is ignored.
        }

        // ---- Inside <xs:schema> ----
        Some(Context::Schema) => match local {
            "simpleType" => {
                let name = require_attr(e, "name", "xs:simpleType")?;
                ctx_stack.push(Context::SimpleType(SimpleTypeBuilder {
                    name,
                    ..Default::default()
                }));
            }
            "complexType" => {
                let name = require_attr(e, "name", "xs:complexType")?;
                ctx_stack.push(Context::ComplexType(ComplexTypeBuilder {
                    name,
                    ..Default::default()
                }));
            }
            "element" => {
                // Top-level element: `<xs:element name="…" type="…">` (with children — unusual).
                // We still capture it but then push a dummy skip context so End pops cleanly.
                on_top_level_element(e, schema)?;
                // Push Schema context so the End event doesn't disturb us — actually we
                // need depth tracking. Increment schema "skip" would break things. Instead
                // we track this via an incremented nesting on the parent. Since Schema doesn't
                // have depth, we just note: the matching End will hit on_end with Some(Schema)
                // and we'll do nothing (Schema never pops on End).
                // Actually — on_end only pops SimpleType/ComplexType contexts.  Schema just
                // returns.  So we're fine: the End of <xs:element> inside Schema is a no-op.
            }
            _ => {
                // Annotation, import, etc. — ignore.  Their End is also a no-op.
            }
        },

        // ---- Inside <xs:simpleType> ----
        Some(Context::SimpleType(ref mut b)) => {
            // Every Start inside a simpleType increments nesting depth.
            b.nesting_depth += 1;

            // Only act on the direct children (nesting_depth == 1).
            if b.nesting_depth == 1 && local == "restriction" {
                b.base = attr_value(e, "base")?.unwrap_or_default();
            }
            // Facets (minLength etc.) are self-closing (Empty events) so we
            // don't need Start handlers for them.
        }

        // ---- Inside <xs:complexType> ----
        Some(Context::ComplexType(ref mut b)) => {
            b.nesting_depth += 1;

            // Act on direct children (nesting_depth == 1) to set up the content model,
            // and on grand-children (nesting_depth == 2) for extension inside simpleContent.
            match (b.nesting_depth, local) {
                (1, "sequence") => {
                    b.content = ComplexContentBuilder::Sequence(Vec::new());
                }
                (1, "choice") => {
                    b.content = ComplexContentBuilder::Choice(Vec::new());
                }
                (1, "simpleContent") => {
                    b.content = ComplexContentBuilder::SimpleContent {
                        base: String::new(),
                        attributes: Vec::new(),
                        in_extension: false,
                    };
                }
                (2, "extension") => {
                    if let ComplexContentBuilder::SimpleContent {
                        ref mut base,
                        ref mut in_extension,
                        ..
                    } = b.content
                    {
                        *base = attr_value(e, "base")?.unwrap_or_default();
                        *in_extension = true;
                    }
                }
                // Element inside sequence/choice: handled as Empty in ISO 20022 XSDs
                // but guard for Start+End form too.
                (2, "element") => {
                    on_element_inside_complex(e, b)?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Event: Empty (self-closing tag)
// ---------------------------------------------------------------------------

fn on_empty(
    e: &BytesStart<'_>,
    ctx_stack: &mut [Context],
    schema: &mut Schema,
    found_schema_root: &mut bool,
) -> Result<(), ParseError> {
    let name_binding = e.name();
    let local = local_name(name_binding.as_ref());

    match ctx_stack.last_mut() {
        None => {
            if local == "schema" {
                *found_schema_root = true;
                schema.target_namespace = attr_value(e, "targetNamespace")?.unwrap_or_default();
            }
        }

        Some(Context::Schema) => {
            if local == "element" {
                on_top_level_element(e, schema)?;
            }
            // simpleType/complexType as empties would have no content, skip them.
        }

        Some(Context::SimpleType(ref mut b)) => {
            // Self-closing tags inside a simpleType: facets and bare restriction.
            match local {
                "restriction" => {
                    // <xs:restriction base="xs:boolean"/> with no facets.
                    b.base = attr_value(e, "base")?.unwrap_or_default();
                }
                "enumeration" => {
                    let v = attr_value(e, "value")?.unwrap_or_default();
                    b.facets.push(Facet::Enumeration(v));
                }
                "pattern" => {
                    let v = attr_value(e, "value")?.unwrap_or_default();
                    b.facets.push(Facet::Pattern(v));
                }
                "minLength" => {
                    let v = attr_value(e, "value")?.unwrap_or_default();
                    b.facets.push(Facet::MinLength(parse_u64(&v, "minLength")?));
                }
                "maxLength" => {
                    let v = attr_value(e, "value")?.unwrap_or_default();
                    b.facets.push(Facet::MaxLength(parse_u64(&v, "maxLength")?));
                }
                "minInclusive" => {
                    let v = attr_value(e, "value")?.unwrap_or_default();
                    b.facets.push(Facet::MinInclusive(v));
                }
                "maxInclusive" => {
                    let v = attr_value(e, "value")?.unwrap_or_default();
                    b.facets.push(Facet::MaxInclusive(v));
                }
                "totalDigits" => {
                    let v = attr_value(e, "value")?.unwrap_or_default();
                    b.facets
                        .push(Facet::TotalDigits(parse_u32(&v, "totalDigits")?));
                }
                "fractionDigits" => {
                    let v = attr_value(e, "value")?.unwrap_or_default();
                    b.facets
                        .push(Facet::FractionDigits(parse_u32(&v, "fractionDigits")?));
                }
                _ => {}
            }
        }

        Some(Context::ComplexType(ref mut b)) => match local {
            "element" => {
                on_element_inside_complex(e, b)?;
            }
            "any" => {
                let namespace = attr_value(e, "namespace")?;
                b.content = ComplexContentBuilder::Any { namespace };
            }
            "attribute" => {
                on_attribute_inside_complex(e, b)?;
            }
            _ => {}
        },
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Event: End (closing tag)
// ---------------------------------------------------------------------------

/// Called for every closing tag.
///
/// The context stack only has `SimpleType` and `ComplexType` entries while we
/// are inside those types.  We use `nesting_depth` to distinguish between end
/// tags of children (decrement) vs. the end tag of the type itself (pop and emit).
fn on_end(ctx_stack: &mut Vec<Context>, schema: &mut Schema) {
    match ctx_stack.last_mut() {
        Some(Context::SimpleType(ref mut b)) => {
            if b.nesting_depth > 0 {
                b.nesting_depth -= 1;
                // Child's end tag — stay in context.
                return;
            }
            // nesting_depth == 0 means this is </xs:simpleType> itself.
        }
        Some(Context::ComplexType(ref mut b)) => {
            if b.nesting_depth > 0 {
                b.nesting_depth -= 1;
                return;
            }
            // nesting_depth == 0 means this is </xs:complexType> itself.
        }
        // Schema end tag — pop the Schema context.
        Some(Context::Schema) => {
            ctx_stack.pop();
            return;
        }
        None => return,
    }

    // Pop the context and emit the finished type.
    let ctx = ctx_stack.pop().expect("checked above");
    match ctx {
        Context::SimpleType(b) => {
            schema.simple_types.push(SimpleType {
                name: b.name,
                restriction: Restriction {
                    base: b.base,
                    facets: b.facets,
                },
            });
        }
        Context::ComplexType(b) => {
            let content = finish_complex_content(b.content);
            schema.complex_types.push(ComplexType {
                name: b.name,
                content,
            });
        }
        Context::Schema => unreachable!("handled above"),
    }
}

// ---------------------------------------------------------------------------
// Sub-handlers
// ---------------------------------------------------------------------------

/// Process a top-level `<xs:element name="…" type="…">` declaration.
fn on_top_level_element(e: &BytesStart<'_>, schema: &mut Schema) -> Result<(), ParseError> {
    if let (Some(name), Some(type_name)) = (attr_value(e, "name")?, attr_value(e, "type")?) {
        schema.elements.push(Element { name, type_name });
    }
    Ok(())
}

/// Process an `<xs:element>` inside a `<xs:sequence>` or `<xs:choice>`.
fn on_element_inside_complex(
    e: &BytesStart<'_>,
    b: &mut ComplexTypeBuilder,
) -> Result<(), ParseError> {
    match b.content {
        ComplexContentBuilder::Sequence(ref mut seq) => {
            if let (Some(name), Some(type_name)) = (attr_value(e, "name")?, attr_value(e, "type")?)
            {
                let min_occurs = parse_min_occurs(e)?;
                let max_occurs = parse_max_occurs(e)?;
                seq.push(SequenceElement {
                    name,
                    type_name,
                    min_occurs,
                    max_occurs,
                });
            }
        }
        ComplexContentBuilder::Choice(ref mut variants) => {
            if let (Some(name), Some(type_name)) = (attr_value(e, "name")?, attr_value(e, "type")?)
            {
                variants.push(ChoiceVariant { name, type_name });
            }
        }
        _ => {}
    }
    Ok(())
}

/// Process an `<xs:attribute>` inside a `<xs:extension>`.
fn on_attribute_inside_complex(
    e: &BytesStart<'_>,
    b: &mut ComplexTypeBuilder,
) -> Result<(), ParseError> {
    if let ComplexContentBuilder::SimpleContent {
        ref mut attributes,
        in_extension,
        ..
    } = b.content
    {
        if in_extension {
            if let (Some(name), Some(type_name)) = (attr_value(e, "name")?, attr_value(e, "type")?)
            {
                let required = attr_value(e, "use")?.is_some_and(|v| v == "required");
                attributes.push(Attribute {
                    name,
                    type_name,
                    required,
                });
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Attribute helpers
// ---------------------------------------------------------------------------

/// Return an attribute value as an owned `String`, or `None` if absent.
fn attr_value(e: &BytesStart<'_>, name: &str) -> Result<Option<String>, ParseError> {
    for attr in e.attributes().flatten() {
        if local_name(attr.key.as_ref()) == name {
            let val = std::str::from_utf8(attr.value.as_ref())?.to_owned();
            return Ok(Some(val));
        }
    }
    Ok(None)
}

/// Return an attribute value, returning `MissingAttribute` if absent.
fn require_attr(
    e: &BytesStart<'_>,
    attr: &'static str,
    element: &'static str,
) -> Result<String, ParseError> {
    attr_value(e, attr)?.ok_or(ParseError::MissingAttribute { element, attr })
}

/// Parse the `minOccurs` attribute, defaulting to `1`.
fn parse_min_occurs(e: &BytesStart<'_>) -> Result<u32, ParseError> {
    match attr_value(e, "minOccurs")? {
        None => Ok(1),
        Some(v) => parse_u32(&v, "minOccurs"),
    }
}

/// Parse the `maxOccurs` attribute, defaulting to `Bounded(1)`.
fn parse_max_occurs(e: &BytesStart<'_>) -> Result<MaxOccurs, ParseError> {
    match attr_value(e, "maxOccurs")? {
        None => Ok(MaxOccurs::Bounded(1)),
        Some(ref v) if v == "unbounded" => Ok(MaxOccurs::Unbounded),
        Some(v) => parse_u32(&v, "maxOccurs").map(MaxOccurs::Bounded),
    }
}

// ---------------------------------------------------------------------------
// Numeric parsing helpers
// ---------------------------------------------------------------------------

fn parse_u32(s: &str, attr: &'static str) -> Result<u32, ParseError> {
    s.parse::<u32>()
        .map_err(|_| ParseError::InvalidAttributeValue {
            value: s.to_owned(),
            attr,
            reason: "expected non-negative integer",
        })
}

fn parse_u64(s: &str, attr: &'static str) -> Result<u64, ParseError> {
    s.parse::<u64>()
        .map_err(|_| ParseError::InvalidAttributeValue {
            value: s.to_owned(),
            attr,
            reason: "expected non-negative integer",
        })
}

// ---------------------------------------------------------------------------
// Misc helpers
// ---------------------------------------------------------------------------

/// Strip the namespace prefix from a qualified name (e.g. `"xs:element"` → `"element"`).
fn local_name(name: &[u8]) -> &str {
    let s = std::str::from_utf8(name).unwrap_or("");
    s.rfind(':').map_or(s, |pos| &s[pos + 1..])
}

/// Convert a [`ComplexContentBuilder`] into the final [`ComplexContent`].
fn finish_complex_content(b: ComplexContentBuilder) -> ComplexContent {
    match b {
        ComplexContentBuilder::Sequence(seq) => ComplexContent::Sequence(seq),
        ComplexContentBuilder::Choice(ch) => ComplexContent::Choice(ch),
        ComplexContentBuilder::SimpleContent {
            base, attributes, ..
        } => ComplexContent::SimpleContent { base, attributes },
        ComplexContentBuilder::Any { namespace } => ComplexContent::Any { namespace },
        ComplexContentBuilder::Empty => ComplexContent::Any { namespace: None },
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn wrap(body: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<xs:schema
    xmlns:xs="http://www.w3.org/2001/XMLSchema"
    targetNamespace="urn:test"
    elementFormDefault="qualified">
{body}
</xs:schema>"#
        )
    }

    // -----------------------------------------------------------------------
    // Basic schema structure
    // -----------------------------------------------------------------------

    #[test]
    fn empty_schema_parses() {
        let xml = wrap("");
        let schema = parse_str(&xml).unwrap();
        assert_eq!(schema.target_namespace, "urn:test");
        assert!(schema.elements.is_empty());
        assert!(schema.simple_types.is_empty());
        assert!(schema.complex_types.is_empty());
    }

    #[test]
    fn missing_schema_root_errors() {
        let err = parse_str("<root/>").unwrap_err();
        assert!(matches!(err, ParseError::MissingSchemaRoot));
    }

    // -----------------------------------------------------------------------
    // Top-level elements
    // -----------------------------------------------------------------------

    #[test]
    fn top_level_element() {
        let xml = wrap(r#"<xs:element name="Document" type="Document"/>"#);
        let schema = parse_str(&xml).unwrap();
        assert_eq!(schema.elements.len(), 1);
        let el = &schema.elements[0];
        assert_eq!(el.name, "Document");
        assert_eq!(el.type_name, "Document");
    }

    #[test]
    fn multiple_top_level_elements() {
        let xml = wrap(
            r#"
            <xs:element name="AppHdr" type="BusinessApplicationHeaderV04"/>
            <xs:element name="Document" type="Document"/>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        assert_eq!(schema.elements.len(), 2);
        assert_eq!(schema.elements[0].name, "AppHdr");
        assert_eq!(schema.elements[1].name, "Document");
    }

    // -----------------------------------------------------------------------
    // Simple types
    // -----------------------------------------------------------------------

    #[test]
    fn simple_type_string_min_max_length() {
        let xml = wrap(
            r#"
            <xs:simpleType name="Max35Text">
                <xs:restriction base="xs:string">
                    <xs:minLength value="1"/>
                    <xs:maxLength value="35"/>
                </xs:restriction>
            </xs:simpleType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        assert_eq!(schema.simple_types.len(), 1);
        let st = &schema.simple_types[0];
        assert_eq!(st.name, "Max35Text");
        assert_eq!(st.restriction.base, "xs:string");
        assert_eq!(
            st.restriction.facets,
            vec![Facet::MinLength(1), Facet::MaxLength(35)]
        );
    }

    #[test]
    fn simple_type_enumeration() {
        let xml = wrap(
            r#"
            <xs:simpleType name="AddressType2Code">
                <xs:restriction base="xs:string">
                    <xs:enumeration value="ADDR"/>
                    <xs:enumeration value="PBOX"/>
                    <xs:enumeration value="HOME"/>
                </xs:restriction>
            </xs:simpleType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        let st = &schema.simple_types[0];
        assert_eq!(
            st.restriction.facets,
            vec![
                Facet::Enumeration("ADDR".into()),
                Facet::Enumeration("PBOX".into()),
                Facet::Enumeration("HOME".into()),
            ]
        );
    }

    #[test]
    fn simple_type_pattern() {
        let xml = wrap(
            r#"
            <xs:simpleType name="CountryCode">
                <xs:restriction base="xs:string">
                    <xs:pattern value="[A-Z]{2,2}"/>
                </xs:restriction>
            </xs:simpleType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        let st = &schema.simple_types[0];
        assert_eq!(
            st.restriction.facets,
            vec![Facet::Pattern("[A-Z]{2,2}".into())]
        );
    }

    #[test]
    fn simple_type_decimal_restriction() {
        let xml = wrap(
            r#"
            <xs:simpleType name="ActiveCurrencyAndAmount_SimpleType">
                <xs:restriction base="xs:decimal">
                    <xs:fractionDigits value="5"/>
                    <xs:totalDigits value="18"/>
                    <xs:minInclusive value="0"/>
                </xs:restriction>
            </xs:simpleType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        let st = &schema.simple_types[0];
        assert_eq!(st.restriction.base, "xs:decimal");
        assert_eq!(
            st.restriction.facets,
            vec![
                Facet::FractionDigits(5),
                Facet::TotalDigits(18),
                Facet::MinInclusive("0".into()),
            ]
        );
    }

    #[test]
    fn simple_type_boolean_no_facets() {
        let xml = wrap(
            r#"
            <xs:simpleType name="YesNoIndicator">
                <xs:restriction base="xs:boolean"/>
            </xs:simpleType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        let st = &schema.simple_types[0];
        assert_eq!(st.name, "YesNoIndicator");
        assert_eq!(st.restriction.base, "xs:boolean");
        assert!(st.restriction.facets.is_empty());
    }

    #[test]
    fn simple_type_empty_string_restriction() {
        // <xs:restriction base="xs:string"/> with no facets — used for
        // unconstrained code types like BusinessMessagePriorityCode.
        let xml = wrap(
            r#"
            <xs:simpleType name="BusinessMessagePriorityCode">
                <xs:restriction base="xs:string"/>
            </xs:simpleType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        let st = &schema.simple_types[0];
        assert_eq!(st.restriction.base, "xs:string");
        assert!(st.restriction.facets.is_empty());
    }

    #[test]
    fn simple_type_date_restriction() {
        let xml = wrap(
            r#"
            <xs:simpleType name="ISODate">
                <xs:restriction base="xs:date"/>
            </xs:simpleType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        let st = &schema.simple_types[0];
        assert_eq!(st.restriction.base, "xs:date");
    }

    // -----------------------------------------------------------------------
    // Complex types — sequence
    // -----------------------------------------------------------------------

    #[test]
    fn complex_type_sequence_required_optional() {
        let xml = wrap(
            r#"
            <xs:complexType name="BranchData5">
                <xs:sequence>
                    <xs:element name="FinInstnId" type="FinancialInstitutionIdentification23"/>
                    <xs:element maxOccurs="1" minOccurs="0" name="BrnchId" type="BranchData5"/>
                </xs:sequence>
            </xs:complexType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        assert_eq!(schema.complex_types.len(), 1);
        let ct = &schema.complex_types[0];
        assert_eq!(ct.name, "BranchData5");

        if let ComplexContent::Sequence(ref seq) = ct.content {
            assert_eq!(seq.len(), 2);
            assert_eq!(seq[0].name, "FinInstnId");
            assert_eq!(seq[0].min_occurs, 1);
            assert_eq!(seq[0].max_occurs, MaxOccurs::Bounded(1));

            assert_eq!(seq[1].name, "BrnchId");
            assert_eq!(seq[1].min_occurs, 0);
            assert_eq!(seq[1].max_occurs, MaxOccurs::Bounded(1));
        } else {
            panic!("expected Sequence, got {:?}", ct.content);
        }
    }

    #[test]
    fn complex_type_sequence_unbounded() {
        let xml = wrap(
            r#"
            <xs:complexType name="BusinessApplicationHeaderV04">
                <xs:sequence>
                    <xs:element name="Fr" type="Party51Choice"/>
                    <xs:element maxOccurs="unbounded" minOccurs="0" name="Rltd" type="BusinessApplicationHeader8"/>
                </xs:sequence>
            </xs:complexType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        let ct = &schema.complex_types[0];
        if let ComplexContent::Sequence(ref seq) = ct.content {
            assert_eq!(seq[1].max_occurs, MaxOccurs::Unbounded);
        } else {
            panic!("expected Sequence");
        }
    }

    #[test]
    fn complex_type_sequence_bounded_max() {
        // AdrLine maxOccurs="7" from PostalAddress27
        let xml = wrap(
            r#"
            <xs:complexType name="PostalAddress27">
                <xs:sequence>
                    <xs:element maxOccurs="7" minOccurs="0" name="AdrLine" type="Max70Text"/>
                </xs:sequence>
            </xs:complexType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        let ct = &schema.complex_types[0];
        if let ComplexContent::Sequence(ref seq) = ct.content {
            assert_eq!(seq[0].max_occurs, MaxOccurs::Bounded(7));
        } else {
            panic!("expected Sequence");
        }
    }

    // -----------------------------------------------------------------------
    // Complex types — choice
    // -----------------------------------------------------------------------

    #[test]
    fn complex_type_choice() {
        let xml = wrap(
            r#"
            <xs:complexType name="Party51Choice">
                <xs:choice>
                    <xs:element name="OrgId" type="PartyIdentification272"/>
                    <xs:element name="FIId" type="BranchAndFinancialInstitutionIdentification8"/>
                </xs:choice>
            </xs:complexType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        let ct = &schema.complex_types[0];
        assert_eq!(ct.name, "Party51Choice");

        if let ComplexContent::Choice(ref variants) = ct.content {
            assert_eq!(variants.len(), 2);
            assert_eq!(variants[0].name, "OrgId");
            assert_eq!(variants[0].type_name, "PartyIdentification272");
            assert_eq!(variants[1].name, "FIId");
        } else {
            panic!("expected Choice, got {:?}", ct.content);
        }
    }

    // -----------------------------------------------------------------------
    // Complex types — simpleContent / extension
    // -----------------------------------------------------------------------

    #[test]
    fn complex_type_simple_content() {
        let xml = wrap(
            r#"
            <xs:complexType name="ActiveCurrencyAndAmount">
                <xs:simpleContent>
                    <xs:extension base="ActiveCurrencyAndAmount_SimpleType">
                        <xs:attribute name="Ccy" type="ActiveCurrencyCode" use="required"/>
                    </xs:extension>
                </xs:simpleContent>
            </xs:complexType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        let ct = &schema.complex_types[0];
        assert_eq!(ct.name, "ActiveCurrencyAndAmount");

        if let ComplexContent::SimpleContent {
            ref base,
            ref attributes,
        } = ct.content
        {
            assert_eq!(base, "ActiveCurrencyAndAmount_SimpleType");
            assert_eq!(attributes.len(), 1);
            assert_eq!(attributes[0].name, "Ccy");
            assert_eq!(attributes[0].type_name, "ActiveCurrencyCode");
            assert!(attributes[0].required);
        } else {
            panic!("expected SimpleContent, got {:?}", ct.content);
        }
    }

    #[test]
    fn simple_content_optional_attribute() {
        let xml = wrap(
            r#"
            <xs:complexType name="FooAmount">
                <xs:simpleContent>
                    <xs:extension base="FooAmount_SimpleType">
                        <xs:attribute name="Ccy" type="CurrencyCode" use="optional"/>
                    </xs:extension>
                </xs:simpleContent>
            </xs:complexType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        let ct = &schema.complex_types[0];
        if let ComplexContent::SimpleContent { ref attributes, .. } = ct.content {
            assert!(!attributes[0].required);
        } else {
            panic!("expected SimpleContent");
        }
    }

    // -----------------------------------------------------------------------
    // Complex types — xs:any
    // -----------------------------------------------------------------------

    #[test]
    fn complex_type_any_with_namespace() {
        let xml = wrap(
            r#"
            <xs:complexType name="SignatureEnvelope">
                <xs:sequence>
                    <xs:any namespace="http://www.w3.org/2000/09/xmldsig#" processContents="lax"/>
                </xs:sequence>
            </xs:complexType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        let ct = &schema.complex_types[0];
        if let ComplexContent::Any { ref namespace } = ct.content {
            assert_eq!(
                namespace.as_deref(),
                Some("http://www.w3.org/2000/09/xmldsig#")
            );
        } else {
            panic!("expected Any, got {:?}", ct.content);
        }
    }

    // -----------------------------------------------------------------------
    // Mixed schema (multiple types)
    // -----------------------------------------------------------------------

    #[test]
    fn mixed_schema_counts() {
        let xml = wrap(
            r#"
            <xs:element name="AppHdr" type="BusinessApplicationHeaderV04"/>
            <xs:simpleType name="Max35Text">
                <xs:restriction base="xs:string">
                    <xs:minLength value="1"/>
                    <xs:maxLength value="35"/>
                </xs:restriction>
            </xs:simpleType>
            <xs:complexType name="BranchData5">
                <xs:sequence>
                    <xs:element maxOccurs="1" minOccurs="0" name="Id" type="Max35Text"/>
                </xs:sequence>
            </xs:complexType>
            "#,
        );
        let schema = parse_str(&xml).unwrap();
        assert_eq!(schema.elements.len(), 1);
        assert_eq!(schema.simple_types.len(), 1);
        assert_eq!(schema.complex_types.len(), 1);
    }

    // -----------------------------------------------------------------------
    // Real schema: head.001.001.04.xsd
    // -----------------------------------------------------------------------

    #[test]
    fn parse_head_001_001_04() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schemas/head/head.001.001.04.xsd"
        );
        let file = std::fs::File::open(path)
            .expect("head.001.001.04.xsd not found — run scripts/download-schemas.sh");
        let schema = parse(std::io::BufReader::new(file)).unwrap();

        assert_eq!(
            schema.target_namespace,
            "urn:iso:std:iso:20022:tech:xsd:head.001.001.04"
        );

        // Must have at least one top-level element.
        assert!(!schema.elements.is_empty(), "no top-level elements parsed");

        let app_hdr = schema.elements.iter().find(|e| e.name == "AppHdr");
        assert!(app_hdr.is_some(), "AppHdr element not found");
        assert_eq!(app_hdr.unwrap().type_name, "BusinessApplicationHeaderV04");

        // Spot-check simple types.
        assert!(
            schema.simple_types.iter().any(|s| s.name == "Max35Text"),
            "Max35Text not found"
        );
        let max35 = schema
            .simple_types
            .iter()
            .find(|s| s.name == "Max35Text")
            .unwrap();
        assert!(max35.restriction.facets.contains(&Facet::MinLength(1)));
        assert!(max35.restriction.facets.contains(&Facet::MaxLength(35)));

        let country = schema
            .simple_types
            .iter()
            .find(|s| s.name == "CountryCode")
            .unwrap();
        assert!(country
            .restriction
            .facets
            .iter()
            .any(|f| matches!(f, Facet::Pattern(_))));

        let yes_no = schema
            .simple_types
            .iter()
            .find(|s| s.name == "YesNoIndicator")
            .unwrap();
        assert_eq!(yes_no.restriction.base, "xs:boolean");

        // Spot-check complex types.
        let party51 = schema
            .complex_types
            .iter()
            .find(|c| c.name == "Party51Choice")
            .unwrap();
        assert!(
            matches!(party51.content, ComplexContent::Choice(_)),
            "Party51Choice should be Choice"
        );

        let sig_env = schema
            .complex_types
            .iter()
            .find(|c| c.name == "SignatureEnvelope")
            .unwrap();
        assert!(
            matches!(sig_env.content, ComplexContent::Any { .. }),
            "SignatureEnvelope should be Any"
        );

        let branch = schema
            .complex_types
            .iter()
            .find(|c| c.name == "BranchData5")
            .unwrap();
        if let ComplexContent::Sequence(ref seq) = branch.content {
            assert!(!seq.is_empty());
        } else {
            panic!("BranchData5 should be Sequence");
        }
    }

    // -----------------------------------------------------------------------
    // Real schema: pacs.008.001.10.xsd
    // -----------------------------------------------------------------------

    #[test]
    fn parse_pacs_008_001_10() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schemas/pacs/pacs.008.001.10.xsd"
        );
        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            // Schema file may not be present in CI — skip gracefully.
            Err(_) => return,
        };
        let schema = parse(std::io::BufReader::new(file)).unwrap();

        assert_eq!(
            schema.target_namespace,
            "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.10"
        );

        // ActiveCurrencyAndAmount_SimpleType uses xs:decimal with facets.
        let decimal_st = schema
            .simple_types
            .iter()
            .find(|s| s.name == "ActiveCurrencyAndAmount_SimpleType")
            .unwrap();
        assert_eq!(decimal_st.restriction.base, "xs:decimal");
        assert!(decimal_st
            .restriction
            .facets
            .contains(&Facet::FractionDigits(5)));
        assert!(decimal_st
            .restriction
            .facets
            .contains(&Facet::TotalDigits(18)));
        assert!(decimal_st
            .restriction
            .facets
            .contains(&Facet::MinInclusive("0".into())));

        // ActiveCurrencyAndAmount is a simpleContent type.
        let amount_ct = schema
            .complex_types
            .iter()
            .find(|c| c.name == "ActiveCurrencyAndAmount")
            .unwrap();
        if let ComplexContent::SimpleContent {
            ref base,
            ref attributes,
        } = amount_ct.content
        {
            assert_eq!(base, "ActiveCurrencyAndAmount_SimpleType");
            assert_eq!(attributes.len(), 1);
            assert_eq!(attributes[0].name, "Ccy");
            assert!(attributes[0].required);
        } else {
            panic!(
                "ActiveCurrencyAndAmount should be SimpleContent, got {:?}",
                amount_ct.content
            );
        }

        // Document element must be present.
        assert!(schema.elements.iter().any(|e| e.name == "Document"));
    }
}
