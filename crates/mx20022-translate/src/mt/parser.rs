//! Block-level parser for SWIFT MT messages.
//!
//! Parses the five-block structure (`{1:...}{2:...}{3:...}{4:\n...-}{5:...}`)
//! without relying on regular expressions for block extraction — nested braces
//! inside blocks 3 and 5 make regex-based extraction fragile.

use regex::Regex;
use std::sync::OnceLock;

use super::error::MtError;
use super::types::{
    Block1, Block2, Block2Input, Block2Output, Block3, Block4, Block5, MtMessage, TagField,
};

// Regex matching `:tag:` at the start of a line in block 4.
// Tag is 2–3 chars: two digits optionally followed by one uppercase letter.
fn field_tag_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^:(\d{2}[A-Z]?):(.*)$").expect("valid regex"))
}

/// Parse a raw SWIFT MT message string into an [`MtMessage`].
///
/// Block 4 is required; all other blocks are optional but returned as `None`
/// when absent rather than returning an error.
///
/// # Errors
///
/// Returns [`MtError`] when:
/// - The block structure has unmatched braces.
/// - A block's content violates the SWIFT spec.
/// - Block 4 is entirely absent.
pub fn parse(input: &str) -> Result<MtMessage, MtError> {
    let input = input.trim();

    let raw_blocks = extract_blocks(input)?;

    let block1 = raw_blocks.get(&1).map(|s| parse_block1(s)).transpose()?;

    let block2 = raw_blocks.get(&2).map(|s| parse_block2(s)).transpose()?;

    let block3 = raw_blocks.get(&3).map(|s| parse_block3(s)).transpose()?;

    let block4_raw = raw_blocks.get(&4).ok_or(MtError::MissingBlock(4))?;
    let block4 = parse_block4(block4_raw);

    let block5 = raw_blocks.get(&5).map(|s| parse_block5(s)).transpose()?;

    Ok(MtMessage {
        block1,
        block2,
        block3,
        block4,
        block5,
    })
}

// ---------------------------------------------------------------------------
// Block extraction
// ---------------------------------------------------------------------------

/// Scan `input` and return a map from block number → block content string.
///
/// Uses brace-depth counting so nested `{...}` inside blocks 3 and 5 are
/// handled correctly. The returned strings are the raw content between the
/// outer `{N:` and its matching `}`.
fn extract_blocks(input: &str) -> Result<std::collections::HashMap<u8, String>, MtError> {
    let mut blocks: std::collections::HashMap<u8, String> = std::collections::HashMap::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut pos = 0;

    while pos < len {
        // Skip whitespace between blocks.
        while pos < len && chars[pos].is_whitespace() {
            pos += 1;
        }
        if pos >= len {
            break;
        }

        // Every block starts with `{`.
        if chars[pos] != '{' {
            return Err(MtError::InvalidBlockStructure(format!(
                "expected '{{' at position {pos}, got '{}'",
                chars[pos]
            )));
        }
        pos += 1; // consume '{'

        // Next character must be the block number digit.
        if pos >= len || !chars[pos].is_ascii_digit() {
            return Err(MtError::InvalidBlockStructure(format!(
                "expected block number digit at position {pos}"
            )));
        }
        // Safe: we checked is_ascii_digit above, and single decimal digit 0-9 fits in u8.
        #[allow(clippy::cast_possible_truncation)]
        let block_num: u8 = chars[pos].to_digit(10).expect("checked: is_ascii_digit") as u8;
        pos += 1;

        // Then a colon.
        if pos >= len || chars[pos] != ':' {
            return Err(MtError::InvalidBlockStructure(format!(
                "expected ':' after block number at position {pos}"
            )));
        }
        pos += 1; // consume ':'

        // Now collect everything up to the matching closing brace, tracking depth.
        let content_start = pos;
        let mut depth = 1usize; // we are inside the outer '{'
        while pos < len {
            match chars[pos] {
                '{' => {
                    depth += 1;
                    pos += 1;
                }
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        pos += 1; // consume closing '}'
                        break;
                    }
                    pos += 1;
                }
                _ => {
                    pos += 1;
                }
            }
        }

        if depth != 0 {
            return Err(MtError::InvalidBlockStructure(format!(
                "unmatched braces in block {block_num}"
            )));
        }

        // The content is chars[content_start .. pos-1] (pos already advanced past '}'.
        let content: String = chars[content_start..pos - 1].iter().collect();
        blocks.insert(block_num, content);
    }

    Ok(blocks)
}

// ---------------------------------------------------------------------------
// Block 1 parser
// ---------------------------------------------------------------------------

fn parse_block1(content: &str) -> Result<Block1, MtError> {
    // Minimum: app_id(1) + service_id(2) + lt_address(12) + session(4) + sequence(6) = 25 chars
    if content.len() < 25 {
        return Err(MtError::InvalidBlockContent {
            block: 1,
            detail: format!("too short: expected ≥25 chars, got {}", content.len()),
        });
    }

    let mut chars = content.chars();

    let app_id = chars.next().ok_or_else(|| MtError::InvalidBlockContent {
        block: 1,
        detail: "missing app_id".into(),
    })?;

    let service_id: String = chars.by_ref().take(2).collect();
    let lt_address: String = chars.by_ref().take(12).collect();
    let session_number: String = chars.by_ref().take(4).collect();
    let sequence_number: String = chars.by_ref().take(6).collect();

    Ok(Block1 {
        app_id,
        service_id,
        lt_address,
        session_number,
        sequence_number,
    })
}

// ---------------------------------------------------------------------------
// Block 2 parser
// ---------------------------------------------------------------------------

fn parse_block2(content: &str) -> Result<Block2, MtError> {
    let mut chars = content.chars();
    let direction = chars.next().ok_or_else(|| MtError::InvalidBlockContent {
        block: 2,
        detail: "empty block 2".into(),
    })?;

    match direction {
        'I' => parse_block2_input(&content[1..]),
        'O' => parse_block2_output(&content[1..]),
        other => Err(MtError::InvalidBlockContent {
            block: 2,
            detail: format!("unknown direction '{other}'; expected I or O"),
        }),
    }
}

fn parse_block2_input(content: &str) -> Result<Block2, MtError> {
    // Minimum: message_type(3) + destination(12) = 15 chars
    if content.len() < 15 {
        return Err(MtError::InvalidBlockContent {
            block: 2,
            detail: format!(
                "input too short: expected ≥15 chars after 'I', got {}",
                content.len()
            ),
        });
    }

    let message_type = content[..3].to_string();
    let destination = content[3..15].to_string();

    let remaining = &content[15..];
    let mut chars = remaining.chars();

    let priority = chars.next().filter(|c| matches!(c, 'S' | 'N' | 'U'));
    let delivery_monitoring = if priority.is_some() {
        chars.next().filter(char::is_ascii_digit)
    } else {
        None
    };
    let obsolescence_period: Option<String> = if delivery_monitoring.is_some() {
        let s: String = chars.take(3).collect();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    } else {
        None
    };

    Ok(Block2::Input(Block2Input {
        message_type,
        destination,
        priority,
        delivery_monitoring,
        obsolescence_period,
    }))
}

fn parse_block2_output(content: &str) -> Result<Block2, MtError> {
    // O + message_type(3) + input_time(4) + MIR(28) + output_date(6) + output_time(4) + priority(1)
    // minimum without priority = 3+4+28+6+4 = 45
    if content.len() < 45 {
        return Err(MtError::InvalidBlockContent {
            block: 2,
            detail: format!(
                "output too short: expected ≥45 chars after 'O', got {}",
                content.len()
            ),
        });
    }

    let message_type = content[..3].to_string();
    let input_time = content[3..7].to_string();
    let mir = content[7..35].to_string();
    let input_date = mir[..6].to_string();
    let output_date = content[35..41].to_string();
    let output_time = content[41..45].to_string();

    let priority = content
        .chars()
        .nth(45)
        .filter(|c| matches!(c, 'S' | 'N' | 'U'));

    Ok(Block2::Output(Block2Output {
        message_type,
        input_time,
        input_date,
        mir,
        output_date,
        output_time,
        priority,
    }))
}

// ---------------------------------------------------------------------------
// Block 3 parser
// ---------------------------------------------------------------------------

fn parse_block3(content: &str) -> Result<Block3, MtError> {
    if content.is_empty() {
        return Ok(Block3::default());
    }
    let tags = parse_tag_value_pairs(content, 3)?;
    Ok(Block3 { tags })
}

// ---------------------------------------------------------------------------
// Block 4 parser
// ---------------------------------------------------------------------------

fn parse_block4(content: &str) -> Block4 {
    // Content may begin with an optional newline (SWIFT spec allows `{4:\n...`).
    let text = content.trim_start_matches('\n').trim_start_matches('\r');

    let re = field_tag_re();
    let mut fields: Vec<TagField> = Vec::new();

    for line in text.lines() {
        // End-of-block marker.
        if line == "-" {
            break;
        }

        if let Some(caps) = re.captures(line) {
            let tag = caps[1].to_string();
            let value = caps[2].to_string();
            fields.push(TagField { tag, value });
        } else {
            // Continuation line — append to the most-recent field.
            if let Some(last) = fields.last_mut() {
                last.value.push('\n');
                last.value.push_str(line);
            }
            // Lines before any field tag are silently ignored.
        }
    }

    Block4 { fields }
}

// ---------------------------------------------------------------------------
// Block 5 parser
// ---------------------------------------------------------------------------

fn parse_block5(content: &str) -> Result<Block5, MtError> {
    if content.is_empty() {
        return Ok(Block5::default());
    }
    let tags = parse_tag_value_pairs(content, 5)?;
    Ok(Block5 { tags })
}

// ---------------------------------------------------------------------------
// Shared helper — parse `{tag:value}` pairs inside blocks 3 and 5
// ---------------------------------------------------------------------------

fn parse_tag_value_pairs(content: &str, block: u8) -> Result<Vec<(String, String)>, MtError> {
    let mut pairs: Vec<(String, String)> = Vec::new();
    let chars: Vec<char> = content.chars().collect();
    let len = chars.len();
    let mut pos = 0;

    while pos < len {
        // Skip whitespace.
        while pos < len && chars[pos].is_whitespace() {
            pos += 1;
        }
        if pos >= len {
            break;
        }

        if chars[pos] != '{' {
            return Err(MtError::InvalidBlockContent {
                block,
                detail: format!("expected '{{' at position {pos}, got '{}'", chars[pos]),
            });
        }
        pos += 1; // consume '{'

        // Read the tag — everything up to the first ':'.
        let tag_start = pos;
        while pos < len && chars[pos] != ':' && chars[pos] != '}' {
            pos += 1;
        }
        let tag: String = chars[tag_start..pos].iter().collect();

        if pos >= len || chars[pos] != ':' {
            // Empty tag or tag without value — insert empty value.
            if pos < len && chars[pos] == '}' {
                pos += 1;
            }
            pairs.push((tag, String::new()));
            continue;
        }
        pos += 1; // consume ':'

        // Collect value up to matching '}'.
        let val_start = pos;
        let mut depth = 1usize;
        while pos < len {
            match chars[pos] {
                '{' => {
                    depth += 1;
                    pos += 1;
                }
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        pos += 1;
                        break;
                    }
                    pos += 1;
                }
                _ => {
                    pos += 1;
                }
            }
        }

        let value: String = chars[val_start..pos - 1].iter().collect();
        pairs.push((tag, value));
    }

    Ok(pairs)
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mt::types::Block2;

    const MT103_RAW: &str = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:I103BANKDEFFXXXXN}\
{3:{108:MT103REF}}\
{4:\n\
:20:REFERENCE123\n\
:23B:CRED\n\
:32A:230615EUR1000,50\n\
:50K:/DE89370400440532013000\n\
JOHN DOE\n\
:59:/GB29NWBK60161331926819\n\
JANE SMITH\n\
:71A:SHA\n\
-}\
{5:{CHK:ABC12345678}}";

    #[test]
    fn test_extract_five_blocks() {
        let blocks = extract_blocks(MT103_RAW).unwrap();
        assert!(blocks.contains_key(&1));
        assert!(blocks.contains_key(&2));
        assert!(blocks.contains_key(&3));
        assert!(blocks.contains_key(&4));
        assert!(blocks.contains_key(&5));
    }

    #[test]
    fn test_block1_parsed() {
        let msg = parse(MT103_RAW).unwrap();
        let b1 = msg.block1.unwrap();
        assert_eq!(b1.app_id, 'F');
        assert_eq!(b1.service_id, "01");
        assert_eq!(b1.lt_address, "BANKBEBBAXXX");
        assert_eq!(b1.session_number, "0000");
        assert_eq!(b1.sequence_number, "000000");
    }

    #[test]
    fn test_block2_input_parsed() {
        let msg = parse(MT103_RAW).unwrap();
        let Block2::Input(b2) = msg.block2.unwrap() else {
            panic!("expected Input variant");
        };
        assert_eq!(b2.message_type, "103");
        assert_eq!(b2.destination, "BANKDEFFXXXX");
        assert_eq!(b2.priority, Some('N'));
    }

    #[test]
    fn test_block3_tag_lookup() {
        let msg = parse(MT103_RAW).unwrap();
        let b3 = msg.block3.unwrap();
        assert_eq!(b3.get("108"), Some("MT103REF"));
    }

    #[test]
    fn test_block4_field_count() {
        let msg = parse(MT103_RAW).unwrap();
        // :20: :23B: :32A: :50K: :59: :71A: — continuation lines are folded
        // into the preceding field, so we expect exactly 6 tag-field entries.
        assert_eq!(msg.block4.fields.len(), 6);
    }

    #[test]
    fn test_block4_multiline_field() {
        let msg = parse(MT103_RAW).unwrap();
        let f = msg.block4.get("50K").unwrap();
        assert!(f.value.contains("JOHN DOE"), "expected continuation line");
    }

    #[test]
    fn test_block5_checksum() {
        let msg = parse(MT103_RAW).unwrap();
        let b5 = msg.block5.unwrap();
        assert_eq!(b5.get("CHK"), Some("ABC12345678"));
    }

    #[test]
    fn test_message_type() {
        let msg = parse(MT103_RAW).unwrap();
        assert_eq!(msg.message_type(), Some("103"));
    }

    #[test]
    fn test_missing_block4_returns_error() {
        let raw = "{1:F01BANKBEBBAXXX0000000000}{2:I103BANKDEFFXXXXN}";
        assert!(parse(raw).is_err());
    }

    #[test]
    fn test_empty_block3() {
        let raw =
            "{1:F01BANKBEBBAXXX0000000000}{2:I103BANKDEFFXXXXN}{3:}{4:\n:20:REF\n:23B:CRED\n-}";
        let msg = parse(raw).unwrap();
        // Empty block 3 present → Some(Block3 { tags: [] })
        assert_eq!(msg.block3.unwrap().tags.len(), 0);
    }
}
