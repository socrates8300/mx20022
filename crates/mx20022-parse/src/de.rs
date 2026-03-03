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

use crate::ParseError;

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
/// # Errors
///
/// Returns [`ParseError::Deserialize`] if the XML is malformed or does not
/// match the expected schema.
pub fn from_reader<R: std::io::BufRead, T: DeserializeOwned>(reader: R) -> Result<T, ParseError> {
    quick_xml::de::from_reader(reader).map_err(ParseError::Deserialize)
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
}
