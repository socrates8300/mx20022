//! Lightweight XML element and attribute extraction utilities.
//!
//! These functions use simple string scanning rather than full XML parsing.
//! They are intentionally limited: they handle the flat, predictable element
//! structures found in ISO 20022 messages but do not handle CDATA, comments,
//! or namespace-qualified attributes.
//!
//! # Migration to typed validation
//!
//! **New code should prefer the typed validation path**
//! ([`SchemeValidator::validate_typed`](super::SchemeValidator::validate_typed))
//! which accesses fields directly on deserialized message structs. The
//! `xml_scan` utilities remain available for cases where raw XML is the only
//! input (e.g. CLI validation without deserialization).
//!
//! # Limitations
//!
//! - Element extraction is first-match only (use [`extract_all_elements`] for
//!   repeated elements).
//! - Namespace prefixes on tags are not handled — match against the local name
//!   only.
//! - Attribute extraction searches the whole document, not a specific element.

/// Extract the text content of the first `<tag>value</tag>` in `xml`.
///
/// Returns `None` if the tag is absent or has no closing counterpart.
///
/// # Examples
///
/// ```
/// use mx20022_validate::schemes::xml_scan::extract_element;
///
/// let xml = "<Doc><NbOfTxs>1</NbOfTxs></Doc>";
/// assert_eq!(extract_element(xml, "NbOfTxs").as_deref(), Some("1"));
/// assert_eq!(extract_element(xml, "Missing"), None);
/// ```
pub fn extract_element(xml: &str, tag: &str) -> Option<String> {
    // Match both `<Tag>` and `<Tag attr="val">` by searching for the tag prefix
    // and then finding the `>` that ends the opening tag.
    let close = format!("</{tag}>");
    let prefix_bare = format!("<{tag}>");
    let prefix_attr = format!("<{tag} ");

    // Find the opening tag (bare or with attributes).
    let tag_start = xml
        .find(prefix_bare.as_str())
        .or_else(|| xml.find(prefix_attr.as_str()))?;

    // Find the `>` that closes the opening tag.
    let gt = xml[tag_start..].find('>')?;
    let content_start = tag_start + gt + 1;

    let end = xml[content_start..].find(&close)?;
    Some(xml[content_start..content_start + end].trim().to_owned())
}

/// Extract the text content of every `<tag>value</tag>` in `xml`.
///
/// Returns an empty `Vec` if the tag is absent.
///
/// # Examples
///
/// ```
/// use mx20022_validate::schemes::xml_scan::extract_all_elements;
///
/// let xml = "<D><Nm>Alice</Nm><Nm>Bob</Nm></D>";
/// let names = extract_all_elements(xml, "Nm");
/// assert_eq!(names, vec!["Alice", "Bob"]);
/// ```
pub fn extract_all_elements(xml: &str, tag: &str) -> Vec<String> {
    let close = format!("</{tag}>");
    let prefix_bare = format!("<{tag}>");
    let prefix_attr = format!("<{tag} ");
    let mut results = Vec::new();
    let mut remaining = xml;
    loop {
        // Find the next opening tag (bare or with attributes).
        let pos_bare = remaining.find(prefix_bare.as_str());
        let pos_attr = remaining.find(prefix_attr.as_str());
        let tag_start = match (pos_bare, pos_attr) {
            (None, None) => break,
            (Some(a), None) => a,
            (None, Some(b)) => b,
            (Some(a), Some(b)) => a.min(b),
        };
        // Advance past the `>` that ends the opening tag.
        let Some(gt) = remaining[tag_start..].find('>') else {
            break;
        };
        let content_start = tag_start + gt + 1;
        let tail = &remaining[content_start..];
        if let Some(end_pos) = tail.find(&close) {
            results.push(tail[..end_pos].trim().to_owned());
            remaining = &tail[end_pos + close.len()..];
        } else {
            break;
        }
    }
    results
}

/// Extract the value of a named attribute on a specific element tag.
///
/// Finds the first occurrence of `<tag ...attr="value"...>` and returns
/// `value`. Handles both self-closing (`/>`) and regular (`>`) forms.
///
/// # Examples
///
/// ```
/// use mx20022_validate::schemes::xml_scan::extract_attribute;
///
/// let xml = r#"<IntrBkSttlmAmt Ccy="USD">1000.00</IntrBkSttlmAmt>"#;
/// assert_eq!(
///     extract_attribute(xml, "IntrBkSttlmAmt", "Ccy").as_deref(),
///     Some("USD")
/// );
/// ```
pub fn extract_attribute(xml: &str, tag: &str, attr: &str) -> Option<String> {
    // Find `<tag ` (must have a space after the tag name for attributes).
    let prefix = format!("<{tag} ");
    let tag_start = xml.find(&prefix)?;
    let tag_content = &xml[tag_start + prefix.len()..];
    // Find the closing `>` (or `/>`) of the opening tag.
    let gt_pos = tag_content.find('>')?;
    let attrs_str = &tag_content[..gt_pos];
    // Find `attr="` within the attribute region.
    let attr_prefix = format!("{attr}=\"");
    let attr_start = attrs_str.find(&attr_prefix)?;
    let value_start = attr_start + attr_prefix.len();
    let value_end = attrs_str[value_start..].find('"')?;
    Some(attrs_str[value_start..value_start + value_end].to_owned())
}

/// Return `true` if the given tag appears anywhere in `xml`.
///
/// # Examples
///
/// ```
/// use mx20022_validate::schemes::xml_scan::has_element;
///
/// let xml = "<Doc><UETR>some-uuid</UETR></Doc>";
/// assert!(has_element(xml, "UETR"));
/// assert!(!has_element(xml, "AppHdr"));
/// ```
pub fn has_element(xml: &str, tag: &str) -> bool {
    xml.contains(&format!("<{tag}>"))
        || xml.contains(&format!("<{tag}/>"))
        || xml.contains(&format!("<{tag} "))
}

/// Return the UTF-8 byte length of the XML string.
///
/// # Examples
///
/// ```
/// use mx20022_validate::schemes::xml_scan::xml_byte_size;
///
/// assert_eq!(xml_byte_size("hello"), 5);
/// ```
pub fn xml_byte_size(xml: &str) -> usize {
    xml.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_element_simple() {
        let xml = "<Root><MsgId>ABC123</MsgId></Root>";
        assert_eq!(extract_element(xml, "MsgId").as_deref(), Some("ABC123"));
    }

    #[test]
    fn extract_element_trims_whitespace() {
        let xml = "<Root><MsgId>  ABC123  </MsgId></Root>";
        assert_eq!(extract_element(xml, "MsgId").as_deref(), Some("ABC123"));
    }

    #[test]
    fn extract_element_missing_returns_none() {
        let xml = "<Root><MsgId>ABC123</MsgId></Root>";
        assert_eq!(extract_element(xml, "NbOfTxs"), None);
    }

    #[test]
    fn extract_element_returns_first_match() {
        let xml = "<Root><Nm>Alice</Nm><Nm>Bob</Nm></Root>";
        assert_eq!(extract_element(xml, "Nm").as_deref(), Some("Alice"));
    }

    #[test]
    fn extract_all_elements_multiple() {
        let xml = "<D><Nm>Alice</Nm><Nm>Bob</Nm><Nm>Carol</Nm></D>";
        let result = extract_all_elements(xml, "Nm");
        assert_eq!(result, vec!["Alice", "Bob", "Carol"]);
    }

    #[test]
    fn extract_all_elements_none_returns_empty() {
        let xml = "<D><MsgId>123</MsgId></D>";
        let result = extract_all_elements(xml, "Nm");
        assert!(result.is_empty());
    }

    #[test]
    fn extract_attribute_finds_value() {
        let xml = r#"<Amount Ccy="USD">100.00</Amount>"#;
        assert_eq!(
            extract_attribute(xml, "Amount", "Ccy").as_deref(),
            Some("USD")
        );
    }

    #[test]
    fn extract_attribute_self_closing() {
        let xml = r#"<Amt Ccy="EUR"/>"#;
        assert_eq!(extract_attribute(xml, "Amt", "Ccy").as_deref(), Some("EUR"));
    }

    #[test]
    fn extract_attribute_missing_returns_none() {
        let xml = r#"<Amount>100.00</Amount>"#;
        assert_eq!(extract_attribute(xml, "Amount", "Ccy"), None);
    }

    #[test]
    fn has_element_present() {
        let xml = "<Doc><UETR>uuid-here</UETR></Doc>";
        assert!(has_element(xml, "UETR"));
    }

    #[test]
    fn has_element_self_closing() {
        let xml = "<Doc><Empty/></Doc>";
        assert!(has_element(xml, "Empty"));
    }

    #[test]
    fn has_element_absent() {
        let xml = "<Doc><MsgId>123</MsgId></Doc>";
        assert!(!has_element(xml, "AppHdr"));
    }

    #[test]
    fn xml_byte_size_ascii() {
        assert_eq!(xml_byte_size("hello"), 5);
    }

    #[test]
    fn xml_byte_size_utf8_multibyte() {
        // "é" is 2 bytes in UTF-8
        assert_eq!(xml_byte_size("é"), 2);
    }
}
