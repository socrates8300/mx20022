//! Private generic field extraction for the version-agnostic CLI checks.

pub(super) fn extract_element<'a>(xml: &'a str, tag: &str) -> Option<&'a str> {
    let close = format!("</{tag}>");
    let prefix_bare = format!("<{tag}>");
    let prefix_attr = format!("<{tag} ");
    let tag_start = xml
        .find(prefix_bare.as_str())
        .or_else(|| xml.find(prefix_attr.as_str()))?;
    let opening_end = xml[tag_start..].find('>')?;
    let content_start = tag_start + opening_end + 1;
    let content_end = xml[content_start..].find(&close)?;
    Some(xml[content_start..content_start + content_end].trim())
}

pub(super) fn extract_all_elements<'a>(xml: &'a str, tag: &str) -> Vec<&'a str> {
    let close = format!("</{tag}>");
    let prefix_bare = format!("<{tag}>");
    let prefix_attr = format!("<{tag} ");
    let mut results = Vec::new();
    let mut remaining = xml;

    loop {
        let bare = remaining.find(prefix_bare.as_str());
        let attributed = remaining.find(prefix_attr.as_str());
        let tag_start = match (bare, attributed) {
            (None, None) => break,
            (Some(position), None) | (None, Some(position)) => position,
            (Some(bare), Some(attributed)) => bare.min(attributed),
        };
        let Some(opening_end) = remaining[tag_start..].find('>') else {
            break;
        };
        let content_start = tag_start + opening_end + 1;
        let tail = &remaining[content_start..];
        let Some(content_end) = tail.find(&close) else {
            break;
        };
        results.push(tail[..content_end].trim());
        remaining = &tail[content_end + close.len()..];
    }

    results
}

pub(super) fn extract_all_attributes<'a>(xml: &'a str, attribute: &str) -> Vec<&'a str> {
    let needle = format!("{attribute}=\"");
    let mut results = Vec::new();
    let mut remaining = xml;

    while let Some(position) = remaining.find(&needle) {
        let value_start = position + needle.len();
        let tail = &remaining[value_start..];
        let Some(value_end) = tail.find('"') else {
            break;
        };
        results.push(tail[..value_end].trim());
        remaining = &tail[value_end + 1..];
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_generic_elements_and_attributes() {
        let xml = r#"<Root><IBAN>one</IBAN><IBAN>two</IBAN><Amt Ccy="EUR">1</Amt></Root>"#;
        assert_eq!(extract_element(xml, "IBAN"), Some("one"));
        assert_eq!(extract_all_elements(xml, "IBAN"), vec!["one", "two"]);
        assert_eq!(extract_all_attributes(xml, "Ccy"), vec!["EUR"]);
    }
}
