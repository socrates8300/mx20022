//! Utilities for detecting ISO 20022 message type from raw XML.
//!
//! ISO 20022 messages are wrapped in a root element (e.g. `<Document>` or
//! `<AppHdr>`) whose `xmlns` attribute carries the message namespace in the
//! form:
//!
//! ```text
//! urn:iso:std:iso:20022:tech:xsd:{family}.{msg}.{variant}.{version}
//! ```
//!
//! For example `urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13` yields:
//! - family   → `"pacs"`
//! - `msg_id`   → `"008"`
//! - variant  → `"001"`
//! - version  → `"13"`

use quick_xml::events::Event;
use quick_xml::name::ResolveResult;
use quick_xml::reader::NsReader;

use crate::ParseError;

/// Message type identifier extracted from an ISO 20022 XML namespace URI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageId {
    /// Message family, e.g. `"pacs"`, `"pain"`, `"camt"`, `"head"`.
    pub family: String,
    /// Message identifier number, e.g. `"008"`.
    pub msg_id: String,
    /// Variant number, e.g. `"001"`.
    pub variant: String,
    /// Version number, e.g. `"13"`.
    pub version: String,
}

impl MessageId {
    /// Returns the canonical dotted identifier, e.g. `"pacs.008.001.13"`.
    pub fn dotted(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.family, self.msg_id, self.variant, self.version
        )
    }
}

/// Prefix that every ISO 20022 namespace begins with.
const NS_PREFIX: &str = "urn:iso:std:iso:20022:tech:xsd:";

/// Parse an ISO 20022 namespace URI into a [`MessageId`].
///
/// Expects the form `urn:iso:std:iso:20022:tech:xsd:{family}.{msg}.{variant}.{version}`.
///
/// # Errors
///
/// Returns [`ParseError::InvalidEnvelope`] if the namespace does not match the
/// expected pattern.
pub fn parse_namespace(ns: &str) -> Result<MessageId, ParseError> {
    let suffix = ns.strip_prefix(NS_PREFIX).ok_or_else(|| {
        ParseError::InvalidEnvelope(format!(
            "namespace does not start with \"{NS_PREFIX}\": {ns}"
        ))
    })?;

    // suffix should be e.g. "pacs.008.001.13"
    let parts: Vec<&str> = suffix.splitn(4, '.').collect();
    if parts.len() != 4 {
        return Err(ParseError::InvalidEnvelope(format!(
            "expected 4 dot-separated components in namespace suffix \"{suffix}\""
        )));
    }

    Ok(MessageId {
        family: parts[0].to_owned(),
        msg_id: parts[1].to_owned(),
        variant: parts[2].to_owned(),
        version: parts[3].to_owned(),
    })
}

/// Extract the authoritative ISO 20022 message type from XML namespace data.
///
/// When the stream contains a `Document` element, its resolved namespace wins
/// over namespaces used by transport metadata such as `AppHdr`. This also
/// handles namespaces inherited from wrapper elements. For messages without a
/// `Document`, the first ISO 20022 element namespace is returned.
///
/// # Errors
///
/// Returns [`ParseError::InvalidEnvelope`] if no matching namespace is found or
/// the namespace is malformed.
///
/// # Examples
///
/// ```
/// # use mx20022_parse::envelope::detect_message_type;
/// let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13"><FIToFICstmrCdtTrf/></Document>"#;
/// let id = detect_message_type(xml).unwrap();
/// assert_eq!(id.family,  "pacs");
/// assert_eq!(id.msg_id,  "008");
/// assert_eq!(id.variant, "001");
/// assert_eq!(id.version, "13");
/// ```
pub fn detect_message_type(xml: &str) -> Result<MessageId, ParseError> {
    let mut reader = NsReader::from_str(xml);
    let mut first_iso_namespace = None;

    loop {
        let (namespace, event) = reader.read_resolved_event().map_err(|error| {
            ParseError::InvalidEnvelope(format!("malformed XML envelope: {error}"))
        })?;

        match event {
            Event::Start(element) | Event::Empty(element) => {
                let is_document = element.local_name().as_ref() == b"Document";
                match namespace {
                    ResolveResult::Bound(namespace) => {
                        let namespace =
                            std::str::from_utf8(namespace.as_ref()).map_err(|error| {
                                ParseError::InvalidEnvelope(format!(
                                    "element namespace is not UTF-8: {error}"
                                ))
                            })?;
                        if is_document {
                            return parse_namespace(namespace);
                        }
                        if first_iso_namespace.is_none() && namespace.starts_with(NS_PREFIX) {
                            first_iso_namespace = Some(parse_namespace(namespace)?);
                        }
                    }
                    ResolveResult::Unbound if is_document => {
                        return Err(ParseError::InvalidEnvelope(
                            "Document element has no namespace".to_owned(),
                        ));
                    }
                    ResolveResult::Unknown(prefix) if is_document => {
                        return Err(ParseError::InvalidEnvelope(format!(
                            "Document namespace prefix is not declared: {}",
                            String::from_utf8_lossy(&prefix)
                        )));
                    }
                    ResolveResult::Unbound | ResolveResult::Unknown(_) => {}
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    first_iso_namespace.ok_or_else(|| {
        ParseError::InvalidEnvelope("no ISO 20022 namespace found in XML document".to_owned())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_namespace_pacs_008() {
        let id = parse_namespace("urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13").unwrap();
        assert_eq!(id.family, "pacs");
        assert_eq!(id.msg_id, "008");
        assert_eq!(id.variant, "001");
        assert_eq!(id.version, "13");
        assert_eq!(id.dotted(), "pacs.008.001.13");
    }

    #[test]
    fn parse_namespace_head_001() {
        let id = parse_namespace("urn:iso:std:iso:20022:tech:xsd:head.001.001.04").unwrap();
        assert_eq!(id.family, "head");
        assert_eq!(id.msg_id, "001");
        assert_eq!(id.variant, "001");
        assert_eq!(id.version, "04");
    }

    #[test]
    fn parse_namespace_invalid_prefix() {
        let err = parse_namespace("urn:something:else:pacs.008.001.13").unwrap_err();
        assert!(matches!(err, ParseError::InvalidEnvelope(_)));
    }

    #[test]
    fn parse_namespace_wrong_component_count() {
        let err = parse_namespace("urn:iso:std:iso:20022:tech:xsd:pacs.008.001").unwrap_err();
        assert!(matches!(err, ParseError::InvalidEnvelope(_)));
    }

    #[test]
    fn detect_message_type_document_root() {
        let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13"><FIToFICstmrCdtTrf/></Document>"#;
        let id = detect_message_type(xml).unwrap();
        assert_eq!(id.dotted(), "pacs.008.001.13");
    }

    #[test]
    fn detect_message_type_apphdr_root() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<AppHdr xmlns="urn:iso:std:iso:20022:tech:xsd:head.001.001.04">
  <Fr/>
</AppHdr>"#;
        let id = detect_message_type(xml).unwrap();
        assert_eq!(id.dotted(), "head.001.001.04");
    }

    #[test]
    fn detect_message_type_no_namespace_returns_error() {
        let xml = r#"<Document><FIToFICstmrCdtTrf/></Document>"#;
        assert!(detect_message_type(xml).is_err());
    }

    #[test]
    fn detect_message_type_non_iso_namespace_returns_error() {
        let xml = r#"<root xmlns="http://example.com/other"/>"#;
        assert!(detect_message_type(xml).is_err());
    }

    #[test]
    fn detect_message_type_pacs_002() {
        let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.002.001.14"/>"#;
        let id = detect_message_type(xml).unwrap();
        assert_eq!(id.family, "pacs");
        assert_eq!(id.msg_id, "002");
        assert_eq!(id.version, "14");
    }

    #[test]
    fn detect_message_type_prefers_document_over_apphdr() {
        let xml = r#"<Envelope><AppHdr xmlns="urn:iso:std:iso:20022:tech:xsd:head.001.001.04"/><Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13"/></Envelope>"#;
        assert_eq!(
            detect_message_type(xml).unwrap().dotted(),
            "pacs.008.001.13"
        );
    }

    #[test]
    fn detect_message_type_resolves_inherited_document_namespace() {
        let xml = r#"<Envelope xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13"><Document/></Envelope>"#;
        assert_eq!(
            detect_message_type(xml).unwrap().dotted(),
            "pacs.008.001.13"
        );
    }
}
