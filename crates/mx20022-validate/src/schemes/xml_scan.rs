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
/// assert_eq!(extract_element(xml, "NbOfTxs"), Some("1"));
/// assert_eq!(extract_element(xml, "Missing"), None);
/// ```
pub fn extract_element<'a>(xml: &'a str, tag: &str) -> Option<&'a str> {
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
    Some(xml[content_start..content_start + end].trim())
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
pub fn extract_all_elements<'a>(xml: &'a str, tag: &str) -> Vec<&'a str> {
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
            results.push(tail[..end_pos].trim());
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
///     extract_attribute(xml, "IntrBkSttlmAmt", "Ccy"),
///     Some("USD")
/// );
/// ```
pub fn extract_attribute<'a>(xml: &'a str, tag: &str, attr: &str) -> Option<&'a str> {
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
    Some(&attrs_str[value_start..value_start + value_end])
}

/// Extract all values of a named XML attribute (e.g. `Ccy="USD"`) from `xml`.
///
/// Scans for every occurrence of `attr_name="value"` and returns the values.
/// Handles both regular (`>`) and self-closing (`/>`) element forms.
///
/// # Examples
///
/// ```
/// use mx20022_validate::schemes::xml_scan::extract_all_attributes;
///
/// let xml = r#"<A Ccy="USD">1</A><B Ccy="EUR">2</B>"#;
/// assert_eq!(extract_all_attributes(xml, "Ccy"), vec!["USD", "EUR"]);
/// ```
pub fn extract_all_attributes<'a>(xml: &'a str, attr_name: &str) -> Vec<&'a str> {
    let needle = format!("{attr_name}=\"");
    let mut results = Vec::new();
    let mut remaining = xml;
    while let Some(pos) = remaining.find(&needle) {
        let after_eq = pos + needle.len();
        let tail = &remaining[after_eq..];
        if let Some(end_pos) = tail.find('"') {
            results.push(tail[..end_pos].trim());
            remaining = &tail[end_pos + 1..];
        } else {
            break;
        }
    }
    results
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
    let bare = format!("<{tag}>");
    let self_close = format!("<{tag}/>");
    let with_attr = format!("<{tag} ");
    xml.contains(&bare) || xml.contains(&self_close) || xml.contains(&with_attr)
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
        assert_eq!(extract_element(xml, "MsgId"), Some("ABC123"));
    }

    #[test]
    fn extract_element_trims_whitespace() {
        let xml = "<Root><MsgId>  ABC123  </MsgId></Root>";
        assert_eq!(extract_element(xml, "MsgId"), Some("ABC123"));
    }

    #[test]
    fn extract_element_missing_returns_none() {
        let xml = "<Root><MsgId>ABC123</MsgId></Root>";
        assert_eq!(extract_element(xml, "NbOfTxs"), None);
    }

    #[test]
    fn extract_element_returns_first_match() {
        let xml = "<Root><Nm>Alice</Nm><Nm>Bob</Nm></Root>";
        assert_eq!(extract_element(xml, "Nm"), Some("Alice"));
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
        assert_eq!(extract_attribute(xml, "Amount", "Ccy"), Some("USD"));
    }

    #[test]
    fn extract_attribute_self_closing() {
        let xml = r#"<Amt Ccy="EUR"/>"#;
        assert_eq!(extract_attribute(xml, "Amt", "Ccy"), Some("EUR"));
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
