//! XSD AST → IR lowering pass.
//!
//! The single public entry point is [`lower`], which converts a parsed
//! [`xsd::Schema`] into a [`TypeGraph`].
//!
//! # Name-conversion rules
//!
//! | Source | Rule | Example |
//! |---|---|---|
//! | XSD type name | kept verbatim (already PascalCase) | `BusinessApplicationHeaderV04` |
//! | Sequence element name → struct field | [`xml_to_snake_case`] | `BizMsgIdr` → `biz_msg_idr` |
//! | Choice element name → variant name | kept verbatim | `OrgId` → `OrgId` |
//! | Enumeration value → code variant | [`code_to_pascal_case`] | `"ADDR"` → `Addr` |
//! | Attribute name → field name | [`xml_to_snake_case`] | `Ccy` → `ccy` |

use indexmap::IndexMap;

use crate::xsd::{self, ComplexContent, Facet, MaxOccurs, Restriction, Schema, SequenceElement};

use super::types::{
    AttrDef, Cardinality, CodeEnumDef, CodeValue, Constraint, EnumDef, FieldDef, NewtypeDef,
    OpaqueDef, RootElement, RustType, StructDef, TypeDef, TypeGraph, TypeRef, ValueWithAttrDef,
    VariantDef,
};

// ── Error type ────────────────────────────────────────────────────────────────

/// Errors that can occur during the lowering pass.
#[derive(Debug, thiserror::Error)]
pub enum LowerError {
    /// A `xs:simpleType` restriction base was not a recognized built-in XSD
    /// type and did not refer to a known user-defined type.
    #[error("unknown base type '{base}' in simple type '{context}'")]
    UnknownBase { base: String, context: String },
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Lower an XSD [`Schema`] into a [`TypeGraph`].
///
/// # Errors
///
/// Returns [`LowerError`] if an unresolvable type reference is encountered.
pub fn lower(schema: &Schema) -> Result<TypeGraph, LowerError> {
    let mut types: IndexMap<String, TypeDef> = IndexMap::new();

    for st in &schema.simple_types {
        let def = lower_simple_type(st)?;
        let name = normalize_type_name(&st.name);
        types.insert(name, def);
    }

    for ct in &schema.complex_types {
        let def = lower_complex_type(ct);
        let name = normalize_type_name(&ct.name);
        types.insert(name, def);
    }

    let root_elements = schema
        .elements
        .iter()
        .map(|e| RootElement {
            xml_name: e.name.clone(),
            type_name: e.type_name.clone(),
        })
        .collect();

    Ok(TypeGraph {
        namespace: schema.target_namespace.clone(),
        root_elements,
        types,
    })
}

// ── Simple type lowering ──────────────────────────────────────────────────────

fn lower_simple_type(st: &xsd::SimpleType) -> Result<TypeDef, LowerError> {
    let restriction = &st.restriction;

    // If every facet is an Enumeration, produce a CodeEnum.
    let all_enums = !restriction.facets.is_empty()
        && restriction
            .facets
            .iter()
            .all(|f| matches!(f, Facet::Enumeration(_)));

    if all_enums {
        return Ok(TypeDef::CodeEnum(lower_code_enum(&st.name, restriction)));
    }

    // Otherwise produce a Newtype with constraints.
    lower_newtype(&st.name, restriction).map(TypeDef::Newtype)
}

fn lower_code_enum(name: &str, restriction: &Restriction) -> CodeEnumDef {
    let codes = restriction
        .facets
        .iter()
        .filter_map(|f| {
            if let Facet::Enumeration(v) = f {
                Some(CodeValue {
                    xml_value: v.clone(),
                    rust_name: code_to_pascal_case(v),
                })
            } else {
                None
            }
        })
        .collect();

    CodeEnumDef {
        name: normalize_type_name(name),
        codes,
    }
}

fn lower_newtype(name: &str, restriction: &Restriction) -> Result<NewtypeDef, LowerError> {
    let inner = map_builtin(&restriction.base).ok_or_else(|| LowerError::UnknownBase {
        base: restriction.base.clone(),
        context: name.to_owned(),
    })?;

    let constraints = restriction.facets.iter().filter_map(lower_facet).collect();

    Ok(NewtypeDef {
        name: normalize_type_name(name),
        inner,
        constraints,
    })
}

fn lower_facet(facet: &Facet) -> Option<Constraint> {
    match facet {
        Facet::Enumeration(_) => None, // handled separately
        Facet::Pattern(v) => Some(Constraint::Pattern(v.clone())),
        Facet::MinLength(v) => Some(Constraint::MinLength(*v)),
        Facet::MaxLength(v) => Some(Constraint::MaxLength(*v)),
        Facet::MinInclusive(v) => Some(Constraint::MinInclusive(v.clone())),
        Facet::MaxInclusive(v) => Some(Constraint::MaxInclusive(v.clone())),
        Facet::TotalDigits(v) => Some(Constraint::TotalDigits(*v)),
        Facet::FractionDigits(v) => Some(Constraint::FractionDigits(*v)),
    }
}

// ── Complex type lowering ─────────────────────────────────────────────────────

fn lower_complex_type(ct: &xsd::ComplexType) -> TypeDef {
    match &ct.content {
        ComplexContent::Sequence(elements) => TypeDef::Struct(lower_struct(&ct.name, elements)),
        ComplexContent::Choice(variants) => TypeDef::Enum(lower_enum(&ct.name, variants)),
        ComplexContent::SimpleContent { base, attributes } => {
            TypeDef::ValueWithAttr(lower_value_with_attr(&ct.name, base, attributes))
        }
        ComplexContent::Any { namespace } => TypeDef::Opaque(OpaqueDef {
            name: normalize_type_name(&ct.name),
            namespace: namespace.clone(),
        }),
    }
}

fn lower_struct(name: &str, elements: &[SequenceElement]) -> StructDef {
    let fields = elements.iter().map(lower_field).collect();
    StructDef {
        name: normalize_type_name(name),
        fields,
    }
}

fn lower_field(el: &SequenceElement) -> FieldDef {
    let type_ref = resolve_type_ref(&el.type_name);
    let cardinality = lower_cardinality(el.min_occurs, &el.max_occurs);

    FieldDef {
        xml_name: el.name.clone(),
        rust_name: xml_to_snake_case(&el.name),
        type_ref,
        cardinality,
    }
}

fn lower_cardinality(min_occurs: u32, max_occurs: &MaxOccurs) -> Cardinality {
    match (min_occurs, max_occurs) {
        (_, MaxOccurs::Unbounded) => Cardinality::Vec,
        (_, MaxOccurs::Bounded(n)) if *n > 1 => Cardinality::BoundedVec(*n),
        (0, MaxOccurs::Bounded(1)) => Cardinality::Optional,
        _ => Cardinality::Required,
    }
}

fn lower_enum(name: &str, variants: &[xsd::ChoiceVariant]) -> EnumDef {
    let variants = variants
        .iter()
        .map(|v| VariantDef {
            xml_name: v.name.clone(),
            // Variant names are kept as-is; they are already PascalCase in
            // ISO 20022 XSD files.
            rust_name: v.name.clone(),
            type_ref: resolve_type_ref(&v.type_name),
        })
        .collect();

    EnumDef {
        name: normalize_type_name(name),
        variants,
    }
}

fn lower_value_with_attr(
    name: &str,
    base: &str,
    attributes: &[xsd::Attribute],
) -> ValueWithAttrDef {
    let value_type = resolve_type_ref(base);

    let attrs = attributes
        .iter()
        .map(|a| AttrDef {
            xml_name: a.name.clone(),
            rust_name: xml_to_snake_case(&a.name),
            type_ref: resolve_type_ref(&a.type_name),
            required: a.required,
        })
        .collect();

    ValueWithAttrDef {
        name: normalize_type_name(name),
        value_type,
        attributes: attrs,
    }
}

// ── Type-reference resolution ─────────────────────────────────────────────────

/// Normalize an XSD type name to a valid Rust `PascalCase` identifier.
///
/// ISO 20022 type names are already `PascalCase` with one exception: types like
/// `ActiveCurrencyAndAmount_SimpleType` contain underscores, which trigger
/// `non_camel_case_types` warnings.  This function strips underscores and joins
/// the segments.
fn normalize_type_name(name: &str) -> String {
    if name.contains('_') {
        name.split('_')
            .map(|seg| {
                let mut c = seg.chars();
                match c.next() {
                    None => String::new(),
                    Some(first) => {
                        let mut s = first.to_uppercase().to_string();
                        s.extend(c);
                        s
                    }
                }
            })
            .collect()
    } else {
        name.to_owned()
    }
}

/// Resolve an XSD type name to a [`TypeRef`].
///
/// If the name is a known XSD built-in it becomes a [`TypeRef::Builtin`];
/// otherwise it becomes a [`TypeRef::Named`] referencing another type in the
/// graph.
fn resolve_type_ref(type_name: &str) -> TypeRef {
    if let Some(rt) = map_builtin(type_name) {
        TypeRef::Builtin(rt)
    } else {
        TypeRef::Named(normalize_type_name(type_name))
    }
}

/// Map an XSD built-in type name to a [`RustType`].
///
/// Returns `None` for user-defined type names.
fn map_builtin(name: &str) -> Option<RustType> {
    // Strip namespace prefix if present (e.g. "xs:string" or "xsd:string").
    let local = name.split(':').next_back().unwrap_or(name);

    match local {
        "string" | "normalizedString" | "token" | "ID" | "IDREF" | "NMTOKEN" | "anyURI"
        | "language" | "Name" | "NCName" | "base64Binary" | "hexBinary" | "time" | "gYear"
        | "gYearMonth" | "gMonth" | "gMonthDay" | "gDay" | "duration" => Some(RustType::String),
        "boolean" => Some(RustType::Bool),
        "decimal" | "integer" | "int" | "long" | "short" | "byte" | "nonNegativeInteger"
        | "positiveInteger" | "unsignedInt" | "unsignedLong" | "unsignedShort" | "unsignedByte"
        | "nonPositiveInteger" | "negativeInteger" | "float" | "double" => Some(RustType::Decimal),
        "date" => Some(RustType::Date),
        "dateTime" => Some(RustType::DateTime),
        _ => None,
    }
}

// ── Name conversion utilities ─────────────────────────────────────────────────

/// Convert an ISO 20022 XML element name (`PascalCase` / mixed-case / all-caps)
/// to a `snake_case` Rust identifier.
///
/// # Algorithm
///
/// The function inserts an underscore before each uppercase letter that
/// immediately follows a lowercase letter, or before an uppercase letter that
/// is immediately followed by a lowercase letter when the preceding character
/// is also uppercase (to handle runs of capitals such as `"BICFI"` or the
/// transition in `"FIToFI"`).
///
/// # Examples
///
/// ```
/// use mx20022_codegen::ir::lower::xml_to_snake_case;
///
/// assert_eq!(xml_to_snake_case("BizMsgIdr"),      "biz_msg_idr");
/// assert_eq!(xml_to_snake_case("FIToFICstmrCdtTrf"), "fi_to_fi_cstmr_cdt_trf");
/// assert_eq!(xml_to_snake_case("BICFI"),           "bicfi");
/// assert_eq!(xml_to_snake_case("LEI"),             "lei");
/// assert_eq!(xml_to_snake_case("Id"),              "id");
/// assert_eq!(xml_to_snake_case("URLAdr"),          "url_adr");
/// assert_eq!(xml_to_snake_case("CreDt"),           "cre_dt");
/// ```
pub fn xml_to_snake_case(name: &str) -> String {
    if name.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = name.chars().collect();
    let n = chars.len();
    let mut out = String::with_capacity(n + 4);

    for i in 0..n {
        let c = chars[i];
        if c.is_uppercase() {
            let prev_lower = i > 0 && chars[i - 1].is_lowercase();
            let next_lower = i + 1 < n && chars[i + 1].is_lowercase();
            let prev_upper = i > 0 && chars[i - 1].is_uppercase();

            // Insert underscore when:
            //  1. preceded by a lowercase letter  (e.g. "Msg|I|dr")
            //  2. preceded by uppercase AND followed by lowercase, meaning we
            //     are at the start of a new word within an all-caps run
            //     (e.g. "BICF|I|" — no, but "URL|A|dr" yes: prev=L upper, next=d lower)
            if i > 0 && (prev_lower || (prev_upper && next_lower)) {
                out.push('_');
            }
            out.push(c.to_lowercase().next().unwrap_or(c));
        } else {
            out.push(c);
        }
    }

    out
}

/// Convert an XSD enumeration value (typically all-caps) to a `PascalCase` Rust
/// variant name.
///
/// # Rules
///
/// - If the value is already mixed-case, preserve it verbatim (the XSD author
///   chose the casing intentionally).
/// - If the value is all-uppercase (optionally with digits/hyphens), convert
///   the first character to uppercase and the rest to lowercase, splitting on
///   hyphens if present.
///
/// # Examples
///
/// ```
/// use mx20022_codegen::ir::lower::code_to_pascal_case;
///
/// assert_eq!(code_to_pascal_case("ADDR"),  "Addr");
/// assert_eq!(code_to_pascal_case("CODU"),  "Codu");
/// assert_eq!(code_to_pascal_case("DUPL"),  "Dupl");
/// assert_eq!(code_to_pascal_case("HIGH"),  "High");
/// assert_eq!(code_to_pascal_case("NORM"),  "Norm");
/// assert_eq!(code_to_pascal_case("TEST-VALUE"), "TestValue");
/// ```
pub fn code_to_pascal_case(value: &str) -> String {
    // Split on hyphens and process each segment.
    value
        .split('-')
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let upper = first.to_uppercase().to_string();
                    let rest: String = chars.collect::<String>().to_lowercase();
                    upper + &rest
                }
            }
        })
        .collect()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{Cardinality, CodeValue, Constraint, RustType, TypeDef, TypeRef};
    use crate::xsd::{
        Attribute, ChoiceVariant, ComplexContent, ComplexType, Element, Facet, MaxOccurs,
        Restriction, Schema, SequenceElement, SimpleType,
    };

    // ── Helper builders ───────────────────────────────────────────────────────

    fn make_schema(
        elements: Vec<Element>,
        simple_types: Vec<SimpleType>,
        complex_types: Vec<ComplexType>,
    ) -> Schema {
        Schema {
            target_namespace: "urn:test".to_owned(),
            elements,
            simple_types,
            complex_types,
        }
    }

    fn seq_el(name: &str, type_name: &str, min: u32, max: MaxOccurs) -> SequenceElement {
        SequenceElement {
            name: name.to_owned(),
            type_name: type_name.to_owned(),
            min_occurs: min,
            max_occurs: max,
        }
    }

    fn restriction(base: &str, facets: Vec<Facet>) -> Restriction {
        Restriction {
            base: base.to_owned(),
            facets,
        }
    }

    // ── Snake-case conversion ─────────────────────────────────────────────────

    #[test]
    fn snake_case_basic_pascal() {
        assert_eq!(xml_to_snake_case("BizMsgIdr"), "biz_msg_idr");
    }

    #[test]
    fn snake_case_fi_to_fi() {
        assert_eq!(
            xml_to_snake_case("FIToFICstmrCdtTrf"),
            "fi_to_fi_cstmr_cdt_trf"
        );
    }

    #[test]
    fn snake_case_all_caps() {
        assert_eq!(xml_to_snake_case("BICFI"), "bicfi");
        assert_eq!(xml_to_snake_case("LEI"), "lei");
    }

    #[test]
    fn snake_case_two_chars() {
        assert_eq!(xml_to_snake_case("Id"), "id");
    }

    #[test]
    fn snake_case_url_prefix() {
        assert_eq!(xml_to_snake_case("URLAdr"), "url_adr");
    }

    #[test]
    fn snake_case_cre_dt() {
        assert_eq!(xml_to_snake_case("CreDt"), "cre_dt");
    }

    #[test]
    fn snake_case_single_lower() {
        assert_eq!(xml_to_snake_case("a"), "a");
    }

    #[test]
    fn snake_case_empty() {
        assert_eq!(xml_to_snake_case(""), "");
    }

    // ── Code-to-PascalCase ────────────────────────────────────────────────────

    #[test]
    fn code_pascal_addr() {
        assert_eq!(code_to_pascal_case("ADDR"), "Addr");
    }

    #[test]
    fn code_pascal_codu() {
        assert_eq!(code_to_pascal_case("CODU"), "Codu");
    }

    #[test]
    fn code_pascal_hyphen() {
        assert_eq!(code_to_pascal_case("TEST-VALUE"), "TestValue");
    }

    // ── CodeEnum lowering ─────────────────────────────────────────────────────

    #[test]
    fn lower_code_enum_basic() {
        let schema = make_schema(
            vec![],
            vec![SimpleType {
                name: "AddressType2Code".to_owned(),
                restriction: restriction(
                    "xs:string",
                    vec![
                        Facet::Enumeration("ADDR".to_owned()),
                        Facet::Enumeration("PBOX".to_owned()),
                        Facet::Enumeration("HOME".to_owned()),
                    ],
                ),
            }],
            vec![],
        );

        let graph = lower(&schema).unwrap();
        let def = graph.types.get("AddressType2Code").unwrap();
        let TypeDef::CodeEnum(ce) = def else {
            panic!("expected CodeEnum, got {:?}", def);
        };

        assert_eq!(ce.name, "AddressType2Code");
        assert_eq!(ce.codes.len(), 3);
        assert_eq!(
            ce.codes[0],
            CodeValue {
                xml_value: "ADDR".to_owned(),
                rust_name: "Addr".to_owned(),
            }
        );
        assert_eq!(
            ce.codes[1],
            CodeValue {
                xml_value: "PBOX".to_owned(),
                rust_name: "Pbox".to_owned(),
            }
        );
    }

    // ── Newtype lowering ──────────────────────────────────────────────────────

    #[test]
    fn lower_newtype_with_pattern() {
        let schema = make_schema(
            vec![],
            vec![SimpleType {
                name: "BICFIDec2014Identifier".to_owned(),
                restriction: restriction(
                    "xs:string",
                    vec![Facet::Pattern(
                        "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}".to_owned(),
                    )],
                ),
            }],
            vec![],
        );

        let graph = lower(&schema).unwrap();
        let def = graph.types.get("BICFIDec2014Identifier").unwrap();
        let TypeDef::Newtype(nt) = def else {
            panic!("expected Newtype");
        };

        assert_eq!(nt.inner, RustType::String);
        assert_eq!(nt.constraints.len(), 1);
        assert!(matches!(&nt.constraints[0], Constraint::Pattern(_)));
    }

    #[test]
    fn lower_newtype_with_length_bounds() {
        let schema = make_schema(
            vec![],
            vec![SimpleType {
                name: "Max35Text".to_owned(),
                restriction: restriction(
                    "xs:string",
                    vec![Facet::MinLength(1), Facet::MaxLength(35)],
                ),
            }],
            vec![],
        );

        let graph = lower(&schema).unwrap();
        let def = graph.types.get("Max35Text").unwrap();
        let TypeDef::Newtype(nt) = def else {
            panic!("expected Newtype");
        };

        assert_eq!(nt.constraints.len(), 2);
        assert_eq!(nt.constraints[0], Constraint::MinLength(1));
        assert_eq!(nt.constraints[1], Constraint::MaxLength(35));
    }

    #[test]
    fn lower_newtype_no_facets() {
        // A simpleType with no facets at all — e.g. BusinessMessagePriorityCode.
        let schema = make_schema(
            vec![],
            vec![SimpleType {
                name: "BusinessMessagePriorityCode".to_owned(),
                restriction: restriction("xs:string", vec![]),
            }],
            vec![],
        );

        let graph = lower(&schema).unwrap();
        let def = graph.types.get("BusinessMessagePriorityCode").unwrap();
        let TypeDef::Newtype(nt) = def else {
            panic!("expected Newtype");
        };
        assert_eq!(nt.inner, RustType::String);
        assert!(nt.constraints.is_empty());
    }

    // ── Struct lowering ───────────────────────────────────────────────────────

    #[test]
    fn lower_struct_basic() {
        let schema = make_schema(
            vec![],
            vec![],
            vec![ComplexType {
                name: "BusinessApplicationHeaderV04".to_owned(),
                content: ComplexContent::Sequence(vec![
                    seq_el("BizMsgIdr", "Max35Text", 1, MaxOccurs::Bounded(1)),
                    seq_el("Fr", "Party51Choice", 1, MaxOccurs::Bounded(1)),
                    seq_el(
                        "Rltd",
                        "BusinessApplicationHeader8",
                        0,
                        MaxOccurs::Unbounded,
                    ),
                ]),
            }],
        );

        let graph = lower(&schema).unwrap();
        let def = graph.types.get("BusinessApplicationHeaderV04").unwrap();
        let TypeDef::Struct(sd) = def else {
            panic!("expected Struct");
        };

        assert_eq!(sd.name, "BusinessApplicationHeaderV04");
        assert_eq!(sd.fields.len(), 3);

        let f0 = &sd.fields[0];
        assert_eq!(f0.xml_name, "BizMsgIdr");
        assert_eq!(f0.rust_name, "biz_msg_idr");
        assert_eq!(f0.type_ref, TypeRef::Named("Max35Text".to_owned()));
        assert_eq!(f0.cardinality, Cardinality::Required);

        let f2 = &sd.fields[2];
        assert_eq!(f2.cardinality, Cardinality::Vec);
    }

    // ── Cardinality mapping ───────────────────────────────────────────────────

    #[test]
    fn cardinality_required() {
        let c = lower_cardinality(1, &MaxOccurs::Bounded(1));
        assert_eq!(c, Cardinality::Required);
    }

    #[test]
    fn cardinality_optional() {
        let c = lower_cardinality(0, &MaxOccurs::Bounded(1));
        assert_eq!(c, Cardinality::Optional);
    }

    #[test]
    fn cardinality_vec_unbounded() {
        let c = lower_cardinality(0, &MaxOccurs::Unbounded);
        assert_eq!(c, Cardinality::Vec);
    }

    #[test]
    fn cardinality_bounded_vec() {
        let c = lower_cardinality(0, &MaxOccurs::Bounded(5));
        assert_eq!(c, Cardinality::BoundedVec(5));
    }

    #[test]
    fn cardinality_unbounded_required_min() {
        // min=1, max=unbounded → Vec (we don't differentiate non-empty vecs in
        // the IR; that can be a constraint in the validate crate).
        let c = lower_cardinality(1, &MaxOccurs::Unbounded);
        assert_eq!(c, Cardinality::Vec);
    }

    // ── Enum (choice) lowering ────────────────────────────────────────────────

    #[test]
    fn lower_choice_enum() {
        let schema = make_schema(
            vec![],
            vec![],
            vec![ComplexType {
                name: "AddressType3Choice".to_owned(),
                content: ComplexContent::Choice(vec![
                    ChoiceVariant {
                        name: "Cd".to_owned(),
                        type_name: "AddressType2Code".to_owned(),
                    },
                    ChoiceVariant {
                        name: "Prtry".to_owned(),
                        type_name: "GenericIdentification30".to_owned(),
                    },
                ]),
            }],
        );

        let graph = lower(&schema).unwrap();
        let def = graph.types.get("AddressType3Choice").unwrap();
        let TypeDef::Enum(ed) = def else {
            panic!("expected Enum");
        };

        assert_eq!(ed.name, "AddressType3Choice");
        assert_eq!(ed.variants.len(), 2);
        assert_eq!(ed.variants[0].xml_name, "Cd");
        assert_eq!(ed.variants[0].rust_name, "Cd");
        assert_eq!(
            ed.variants[0].type_ref,
            TypeRef::Named("AddressType2Code".to_owned())
        );
    }

    // ── SimpleContent → ValueWithAttr ─────────────────────────────────────────

    #[test]
    fn lower_simple_content() {
        let schema = make_schema(
            vec![],
            vec![],
            vec![ComplexType {
                name: "ActiveCurrencyAndAmount".to_owned(),
                content: ComplexContent::SimpleContent {
                    base: "xs:decimal".to_owned(),
                    attributes: vec![Attribute {
                        name: "Ccy".to_owned(),
                        type_name: "ActiveCurrencyCode".to_owned(),
                        required: true,
                    }],
                },
            }],
        );

        let graph = lower(&schema).unwrap();
        let def = graph.types.get("ActiveCurrencyAndAmount").unwrap();
        let TypeDef::ValueWithAttr(vwa) = def else {
            panic!("expected ValueWithAttr");
        };

        assert_eq!(vwa.name, "ActiveCurrencyAndAmount");
        assert_eq!(vwa.value_type, TypeRef::Builtin(RustType::Decimal));
        assert_eq!(vwa.attributes.len(), 1);
        assert_eq!(vwa.attributes[0].xml_name, "Ccy");
        assert_eq!(vwa.attributes[0].rust_name, "ccy");
        assert_eq!(
            vwa.attributes[0].type_ref,
            TypeRef::Named("ActiveCurrencyCode".to_owned())
        );
        assert!(vwa.attributes[0].required);
    }

    // ── xs:any → Opaque ───────────────────────────────────────────────────────

    #[test]
    fn lower_any_opaque() {
        let schema = make_schema(
            vec![],
            vec![],
            vec![ComplexType {
                name: "SignatureEnvelope".to_owned(),
                content: ComplexContent::Any {
                    namespace: Some("##other".to_owned()),
                },
            }],
        );

        let graph = lower(&schema).unwrap();
        let def = graph.types.get("SignatureEnvelope").unwrap();
        let TypeDef::Opaque(op) = def else {
            panic!("expected Opaque");
        };

        assert_eq!(op.name, "SignatureEnvelope");
        assert_eq!(op.namespace, Some("##other".to_owned()));
    }

    #[test]
    fn lower_any_opaque_no_namespace() {
        let schema = make_schema(
            vec![],
            vec![],
            vec![ComplexType {
                name: "AnyContent".to_owned(),
                content: ComplexContent::Any { namespace: None },
            }],
        );

        let graph = lower(&schema).unwrap();
        let def = graph.types.get("AnyContent").unwrap();
        let TypeDef::Opaque(op) = def else {
            panic!("expected Opaque");
        };
        assert!(op.namespace.is_none());
    }

    // ── Built-in type mapping ─────────────────────────────────────────────────

    #[test]
    fn builtin_mapping_string() {
        assert_eq!(map_builtin("xs:string"), Some(RustType::String));
        assert_eq!(map_builtin("string"), Some(RustType::String));
    }

    #[test]
    fn builtin_mapping_boolean() {
        assert_eq!(map_builtin("xs:boolean"), Some(RustType::Bool));
    }

    #[test]
    fn builtin_mapping_decimal() {
        assert_eq!(map_builtin("xs:decimal"), Some(RustType::Decimal));
    }

    #[test]
    fn builtin_mapping_date() {
        assert_eq!(map_builtin("xs:date"), Some(RustType::Date));
    }

    #[test]
    fn builtin_mapping_datetime() {
        assert_eq!(map_builtin("xs:dateTime"), Some(RustType::DateTime));
    }

    #[test]
    fn builtin_mapping_unknown() {
        assert_eq!(map_builtin("Max35Text"), None);
    }

    // ── Root elements ─────────────────────────────────────────────────────────

    #[test]
    fn lower_root_elements() {
        let schema = make_schema(
            vec![Element {
                name: "AppHdr".to_owned(),
                type_name: "BusinessApplicationHeaderV04".to_owned(),
            }],
            vec![],
            vec![],
        );

        let graph = lower(&schema).unwrap();
        assert_eq!(graph.root_elements.len(), 1);
        assert_eq!(graph.root_elements[0].xml_name, "AppHdr");
        assert_eq!(
            graph.root_elements[0].type_name,
            "BusinessApplicationHeaderV04"
        );
    }

    // ── Integration: real head.001.001.04 schema ──────────────────────────────

    #[test]
    fn integration_lower_head_001() {
        let xsd_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schemas/head/head.001.001.04.xsd"
        );

        let file = std::fs::File::open(xsd_path).expect("head.001.001.04.xsd not found");
        let schema = crate::xsd::parse(std::io::BufReader::new(file)).expect("XSD parse failed");

        let graph = lower(&schema).unwrap();

        // The schema has one root element (AppHdr).
        assert_eq!(graph.root_elements.len(), 1, "expected 1 root element");
        assert_eq!(graph.root_elements[0].xml_name, "AppHdr");

        // There should be a reasonable number of types (the schema has ~50+).
        assert!(
            graph.types.len() >= 30,
            "expected at least 30 types, got {}",
            graph.types.len()
        );

        // The root type should be a Struct.
        let root_type_name = &graph.root_elements[0].type_name;
        let root = graph.types.get(root_type_name).expect("root type missing");
        assert!(
            matches!(root, TypeDef::Struct(_)),
            "root type should be a Struct"
        );

        // CodeEnum: AddressType2Code should have 6 variants.
        let addr_code = graph
            .types
            .get("AddressType2Code")
            .expect("AddressType2Code missing");
        let TypeDef::CodeEnum(ce) = addr_code else {
            panic!("AddressType2Code should be CodeEnum");
        };
        assert_eq!(ce.codes.len(), 6);

        // Newtype: BICFIDec2014Identifier should have a pattern constraint.
        let bic = graph
            .types
            .get("BICFIDec2014Identifier")
            .expect("BICFIDec2014Identifier missing");
        let TypeDef::Newtype(nt) = bic else {
            panic!("BICFIDec2014Identifier should be Newtype");
        };
        assert!(nt
            .constraints
            .iter()
            .any(|c| matches!(c, Constraint::Pattern(_))));

        // Enum (choice): AddressType3Choice.
        let choice = graph
            .types
            .get("AddressType3Choice")
            .expect("AddressType3Choice missing");
        assert!(matches!(choice, TypeDef::Enum(_)));

        // Opaque: SignatureEnvelope (xs:any).
        let sig = graph
            .types
            .get("SignatureEnvelope")
            .expect("SignatureEnvelope missing");
        assert!(matches!(sig, TypeDef::Opaque(_)));
    }
}
