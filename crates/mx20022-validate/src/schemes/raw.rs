//! Raw XML checks that cannot be expressed on a deserialized message body.

use quick_xml::events::Event;
use quick_xml::Reader;

/// Return the UTF-8 byte length of an XML string.
pub(crate) fn byte_len(xml: &str) -> usize {
    xml.len()
}

/// Detect `AppHdr` and `BizMsgIdr` elements by local name.
///
/// Prefixes are ignored, so namespace-qualified headers are recognized. A
/// malformed stream yields the presence observed before the parser error so
/// the adapter can preserve independent header findings with `SCHEME_PARSE`.
pub(crate) fn header_presence(xml: &str) -> (bool, bool) {
    let mut reader = Reader::from_str(xml);
    let mut app_header = false;
    let mut business_message_id = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(element) | Event::Empty(element)) => {
                let local_name = element.local_name();
                match local_name.as_ref() {
                    b"AppHdr" => app_header = true,
                    b"BizMsgIdr" => business_message_id = true,
                    _ => {}
                }
                if app_header && business_message_id {
                    break;
                }
            }
            Ok(Event::Eof) | Err(_) => break,
            Ok(_) => {}
        }
    }

    (app_header, business_message_id)
}

/// Return the first disallowed control character and its UTF-8 byte offset.
pub(crate) fn first_disallowed_control(xml: &str) -> Option<(usize, char)> {
    xml.char_indices()
        .find(|(_, character)| character.is_control() && !matches!(character, '\n' | '\r' | '\t'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_len_counts_utf8_bytes() {
        assert_eq!(byte_len("é"), 2);
    }

    #[test]
    fn header_presence_uses_local_names() {
        let xml = r#"<env:Envelope xmlns:env="urn:env" xmlns:h="urn:head"><h:AppHdr><h:BizMsgIdr>id</h:BizMsgIdr></h:AppHdr></env:Envelope>"#;
        assert_eq!(header_presence(xml), (true, true));
    }

    #[test]
    fn first_control_reports_byte_offset() {
        assert_eq!(first_disallowed_control("é\u{1}"), Some((2, '\u{1}')));
        assert_eq!(first_disallowed_control("ok\n\t\r"), None);
    }
}
