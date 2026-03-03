//! Serialization of ISO 20022 message types to XML.
//!
//! Thin wrappers around [`quick_xml::se`] that translate errors into
//! [`ParseError`].
//!
//! # Examples
//!
//! ```no_run
//! # use mx20022_parse::ser::to_string;
//! # use mx20022_model::generated::head::BusinessApplicationHeaderV04;
//! # let hdr: BusinessApplicationHeaderV04 = unimplemented!();
//! let xml = to_string(&hdr).unwrap();
//! ```

use serde::Serialize;

use crate::ParseError;

/// XML declaration prepended when using [`to_string_with_declaration`].
const XML_DECLARATION: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

/// Serialize an ISO 20022 message type to an XML string.
///
/// The root element name is derived from the struct name (or `#[serde(rename)]`).
/// No XML declaration is prepended; use [`to_string_with_declaration`] if needed.
///
/// # Errors
///
/// Returns [`ParseError::Serialize`] if the value cannot be serialized.
pub fn to_string<T: Serialize>(value: &T) -> Result<String, ParseError> {
    quick_xml::se::to_string(value).map_err(ParseError::Serialize)
}

/// Serialize an ISO 20022 message type to an XML string with an XML declaration.
///
/// Prepends `<?xml version="1.0" encoding="UTF-8"?>` before the XML body.
///
/// # Errors
///
/// Returns [`ParseError::Serialize`] if the value cannot be serialized.
pub fn to_string_with_declaration<T: Serialize>(value: &T) -> Result<String, ParseError> {
    let body = quick_xml::se::to_string(value).map_err(ParseError::Serialize)?;
    Ok(format!("{XML_DECLARATION}{body}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_with_declaration_prepends_declaration() {
        #[derive(serde::Serialize)]
        struct Msg {
            value: u32,
        }

        let result = to_string_with_declaration(&Msg { value: 42 }).unwrap();
        assert!(
            result.starts_with(r#"<?xml version="1.0" encoding="UTF-8"?>"#),
            "expected XML declaration at start, got: {result}"
        );
        assert!(
            result.contains("<value>42</value>"),
            "expected body in output"
        );
    }

    #[test]
    fn to_string_no_declaration() {
        #[derive(serde::Serialize)]
        struct Msg {
            value: u32,
        }

        let result = to_string(&Msg { value: 7 }).unwrap();
        assert!(
            !result.starts_with("<?xml"),
            "to_string must not include XML declaration"
        );
        assert!(result.contains("<value>7</value>"));
    }
}
