//! Deserialization of ISO 20022 XML messages.
//!
//! Thin wrappers around [`quick_xml::de`] that translate errors into
//! [`ParseError`].
//!
//! # Examples
//!
//! ```no_run
//! # use mx20022_parse::de::from_str;
//! # use mx20022_model::generated::head::BusinessApplicationHeaderV04;
//! let xml = r#"<AppHdr xmlns="urn:iso:std:iso:20022:tech:xsd:head.001.001.04">...</AppHdr>"#;
//! let _hdr: BusinessApplicationHeaderV04 = from_str(xml).unwrap();
//! ```

use serde::de::DeserializeOwned;

use crate::{envelope::detect_message_type, ParseError};

/// Deserialize an ISO 20022 XML message from a string slice.
///
/// The root element name must match the serde rename of the target type.
///
/// # Errors
///
/// Returns [`ParseError::Deserialize`] if the XML is malformed or does not
/// match the expected schema.
pub fn from_str<T: DeserializeOwned>(xml: &str) -> Result<T, ParseError> {
    quick_xml::de::from_str(xml).map_err(ParseError::Deserialize)
}

/// Deserialize an ISO 20022 XML message from a buffered reader.
///
/// # Caution
///
/// This reads until EOF and imposes no upper bound on input size, element
/// depth, or memory use. When deserializing from an untrusted or
/// size-unbounded source (e.g. a network stream), wrap the reader with
/// [`std::io::Read::take`] or read into a pre-sized buffer and use
/// [`from_str`] instead, so that a single oversized document cannot exhaust
/// process memory.
///
/// # Errors
///
/// Returns [`ParseError::Deserialize`] if the XML is malformed or does not
/// match the expected schema.
pub fn from_reader<R: std::io::BufRead, T: DeserializeOwned>(reader: R) -> Result<T, ParseError> {
    quick_xml::de::from_reader(reader).map_err(ParseError::Deserialize)
}

/// Deserialize an ISO 20022 XML message and tag any deserialization
/// failure with the detected message identifier.
///
/// Behaves like [`from_str`] on success. On failure, if the envelope
/// detection succeeded the error is returned as
/// [`ParseError::DeserializeIn`] carrying the dotted message ID
/// (e.g. `"pacs.008.001.13"`); if envelope detection itself failed,
/// the underlying `quick_xml` error is returned via
/// [`ParseError::Deserialize`] unchanged.
///
/// Useful when the XML may be one of several pacs/pain/camt/head
/// messages and an opaque `quick_xml` diagnostic alone is not enough
/// to tell the user *which* schema mismatched.
///
/// # Errors
///
/// Returns [`ParseError::DeserializeIn`] when envelope detection
/// succeeded but deserialization failed; returns
/// [`ParseError::Deserialize`] otherwise.
pub fn from_str_in_envelope<T: DeserializeOwned>(xml: &str) -> Result<T, ParseError> {
    let detected = detect_message_type(xml).ok();
    quick_xml::de::from_str(xml).map_err(|e| match detected {
        Some(id) => ParseError::DeserializeIn {
            context: id.dotted(),
            source: e,
        },
        None => ParseError::Deserialize(e),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A struct that requires XML element structure to deserialize.
    #[derive(serde::Deserialize, Debug)]
    struct Foo {
        #[allow(dead_code)]
        x: u32,
    }

    #[test]
    fn from_str_invalid_xml_returns_error() {
        // "<<<garbage>>>" is invalid XML and cannot match the Foo struct shape.
        let result: Result<Foo, _> = from_str("<<<garbage>>>");
        assert!(result.is_err(), "malformed XML must return an error");
    }

    #[test]
    fn from_reader_invalid_xml_returns_error() {
        let xml = b"<<<garbage>>>";
        let result: Result<Foo, _> = from_reader(xml.as_ref());
        assert!(result.is_err(), "malformed XML must return an error");
    }

    #[test]
    fn from_str_in_envelope_includes_detected_message_id_on_failure() {
        // Valid pacs.008 envelope, but the body shape does not match Foo
        // (no <x> element).
        let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13"><Other/></Document>"#;
        let err = from_str_in_envelope::<Foo>(xml).unwrap_err();
        match err {
            ParseError::DeserializeIn { ref context, .. } => {
                assert_eq!(context, "pacs.008.001.13");
                let rendered = err.to_string();
                assert!(
                    rendered.contains("pacs.008.001.13"),
                    "Display should mention the detected envelope, got: {rendered}"
                );
            }
            other => panic!("expected DeserializeIn, got: {other:?}"),
        }
    }

    #[test]
    fn from_str_in_envelope_falls_back_to_plain_deserialize_when_envelope_unknown() {
        // No ISO 20022 namespace → detection fails, error stays as
        // ParseError::Deserialize so existing match-arms keep working.
        let xml = r#"<Document><Other/></Document>"#;
        let err = from_str_in_envelope::<Foo>(xml).unwrap_err();
        assert!(
            matches!(err, ParseError::Deserialize(_)),
            "expected Deserialize without context, got: {err:?}"
        );
    }

    #[test]
    fn from_str_in_envelope_succeeds_passes_through() {
        // Foo has a single u32 field `x`. Wrap it in a pacs.008 envelope.
        let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13"><x>42</x></Document>"#;
        // Foo's deserializer reads <x>; whether quick_xml accepts the
        // shape depends on the structural decisions of serde-xml-rs vs
        // quick_xml. The behaviour we want to assert is "no
        // DeserializeIn was raised when deserialization succeeded" — the
        // call returns Ok or returns Deserialize/DeserializeIn but never
        // panics. In practice the body matches, so:
        let v: Foo = from_str_in_envelope(xml).expect("body matches Foo");
        assert_eq!(v.x, 42);
    }
}
