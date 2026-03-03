//! Field-level parser for MT940 Customer Statement Messages.
//!
//! MT940 is used to convey detailed information about entries booked to an
//! account.  The `:61:` (Statement Line) field has a packed positional format
//! that requires careful character-level parsing.

use crate::mt::error::MtError;
use crate::mt::types::{Block4, TagField};

use super::common::{parse_amount, parse_date_mmdd, parse_date_yymmdd};

// ---------------------------------------------------------------------------
// Parsed representation
// ---------------------------------------------------------------------------

/// A fully parsed MT940 Customer Statement Message.
#[derive(Debug, Clone, PartialEq)]
pub struct Mt940 {
    /// `:20:` — Transaction Reference Number.
    pub transaction_reference: String,
    /// `:25/25P:` — Account Identification.
    pub account_id: String,
    /// `:28C:` — Statement Number / Sequence Number.
    pub statement_number: String,
    /// `:60F/60M:` — Opening Balance.
    pub opening_balance: Balance,
    /// `:62F/62M:` — Closing Balance.
    pub closing_balance: Balance,
    // Optional fields
    /// `:21:` — Related Reference.
    pub related_reference: Option<String>,
    /// `:61:` + optional `:86:` pairs — Statement Lines.
    pub statement_lines: Vec<StatementLine>,
    /// `:64:` — Closing Available Balance.
    pub closing_available: Option<Balance>,
    /// `:65:` — Forward Available Balances (repeatable).
    pub forward_available: Vec<Balance>,
    /// Standalone `:86:` not immediately following a `:61:`.
    pub account_owner_info: Option<String>,
}

/// A balance field (`:60F/M:`, `:62F/M:`, `:64:`, `:65:`).
#[derive(Debug, Clone, PartialEq)]
pub struct Balance {
    /// `D` (debit) or `C` (credit).
    pub dc_indicator: char,
    /// Date in ISO `YYYY-MM-DD` format.
    pub date: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Amount as a decimal string.
    pub amount: String,
    /// `F` (first/final) or `M` (intermediate).
    pub balance_type: char,
}

/// A single `:61:` statement line together with its optional `:86:` narrative.
#[derive(Debug, Clone, PartialEq)]
pub struct StatementLine {
    /// Value date in ISO `YYYY-MM-DD` format.
    pub value_date: String,
    /// Entry date in ISO `YYYY-MM-DD` format (from the optional `MMDD`).
    pub entry_date: Option<String>,
    /// Debit/Credit mark: `D`, `C`, `RD`, or `RC`.
    pub dc_mark: String,
    /// Optional funds code letter immediately following the D/C mark.
    pub funds_code: Option<char>,
    /// Amount as a decimal string.
    pub amount: String,
    /// Transaction type identifier (letter + 3 chars, e.g. `NTRN`).
    pub transaction_type: String,
    /// Customer reference.
    pub reference: String,
    /// Institution reference (after `//`).
    pub institution_reference: Option<String>,
    /// Supplementary details line.
    pub supplementary: Option<String>,
    /// `:86:` narrative immediately following this `:61:`.
    pub information: Option<String>,
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

/// Parse an [`Mt940`] from the already-parsed [`Block4`].
///
/// # Errors
///
/// Returns [`MtError::MissingField`] for mandatory fields or
/// [`MtError::InvalidFieldValue`] when a field cannot be decoded.
pub fn parse_mt940(block4: &Block4) -> Result<Mt940, MtError> {
    let mt = "940";

    let transaction_reference = require_field(block4, "20", mt)?.value.clone();

    // :25: or :25P:
    let account_id = block4
        .fields
        .iter()
        .find(|f| f.tag == "25" || f.tag == "25P")
        .map(|f| f.value.clone())
        .ok_or_else(|| MtError::MissingField {
            tag: "25".into(),
            message_type: mt.into(),
        })?;

    let statement_number = require_field(block4, "28C", mt)?.value.clone();

    // Opening balance: :60F: or :60M:
    let opening_balance = block4
        .fields
        .iter()
        .find(|f| f.tag == "60F" || f.tag == "60M")
        .ok_or_else(|| MtError::MissingField {
            tag: "60F/60M".into(),
            message_type: mt.into(),
        })
        .and_then(parse_balance)?;

    // Closing balance: :62F: or :62M:
    let closing_balance = block4
        .fields
        .iter()
        .find(|f| f.tag == "62F" || f.tag == "62M")
        .ok_or_else(|| MtError::MissingField {
            tag: "62F/62M".into(),
            message_type: mt.into(),
        })
        .and_then(parse_balance)?;

    // Optional
    let related_reference = block4.get("21").map(|f| f.value.clone());

    // Statement lines: interleaved :61: and :86: fields
    let statement_lines = parse_statement_lines(block4)?;

    // :64: closing available
    let closing_available = block4
        .fields
        .iter()
        .find(|f| f.tag == "64")
        .map(parse_balance)
        .transpose()?;

    // :65: repeatable
    let forward_available = block4
        .get_all("65")
        .into_iter()
        .map(parse_balance)
        .collect::<Result<Vec<_>, _>>()?;

    // Standalone :86: (not following a :61:) — take the first one that was not consumed.
    let account_owner_info = find_standalone_86(block4, &statement_lines);

    Ok(Mt940 {
        transaction_reference,
        account_id,
        statement_number,
        opening_balance,
        closing_balance,
        related_reference,
        statement_lines,
        closing_available,
        forward_available,
        account_owner_info,
    })
}

// ---------------------------------------------------------------------------
// Statement line parsing
// ---------------------------------------------------------------------------

fn parse_statement_lines(block4: &Block4) -> Result<Vec<StatementLine>, MtError> {
    let mut lines: Vec<StatementLine> = Vec::new();
    let fields = &block4.fields;
    let mut i = 0;

    while i < fields.len() {
        let field = &fields[i];
        if field.tag == "61" {
            let mut sl = parse_61(&field.value)?;
            // Look ahead for a :86: immediately following.
            if i + 1 < fields.len() && fields[i + 1].tag == "86" {
                sl.information = Some(fields[i + 1].value.clone());
                i += 1; // skip the :86: — it belongs to this :61:
            }
            lines.push(sl);
        }
        i += 1;
    }

    Ok(lines)
}

/// Find a standalone `:86:` field that was NOT consumed by a `:61:`.
///
/// Strategy: any `:86:` that is NOT immediately preceded by a `:61:` is
/// standalone.
fn find_standalone_86(block4: &Block4, consumed_lines: &[StatementLine]) -> Option<String> {
    // Count how many :86: fields were consumed as statement-line information.
    let consumed_count = consumed_lines
        .iter()
        .filter(|sl| sl.information.is_some())
        .count();

    let all_86: Vec<&str> = block4
        .get_all("86")
        .into_iter()
        .map(|f| f.value.as_str())
        .collect();

    // Standalone ones are those beyond the consumed count.
    if all_86.len() > consumed_count {
        Some(all_86[consumed_count..].join("\n"))
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Balance field parser
// ---------------------------------------------------------------------------

fn parse_balance(field: &TagField) -> Result<Balance, MtError> {
    let tag = &field.tag;
    let s = field.value.trim();

    // balance_type is encoded in the tag suffix (F or M).
    let balance_type = tag
        .chars()
        .last()
        .filter(|c| *c == 'F' || *c == 'M')
        .unwrap_or('F');

    // Format: D/C indicator (1 char) + date (YYMMDD, 6 chars) + currency (3 chars) + amount
    if s.len() < 11 {
        return Err(MtError::InvalidFieldValue {
            tag: tag.clone(),
            detail: format!("balance too short: '{s}'"),
        });
    }

    let dc_indicator = s.chars().next().ok_or_else(|| MtError::InvalidFieldValue {
        tag: tag.clone(),
        detail: "empty balance".into(),
    })?;

    if dc_indicator != 'D' && dc_indicator != 'C' {
        return Err(MtError::InvalidFieldValue {
            tag: tag.clone(),
            detail: format!("expected D or C, got '{dc_indicator}'"),
        });
    }

    let date = parse_date_yymmdd(&s[1..7]).map_err(|_| MtError::InvalidFieldValue {
        tag: tag.clone(),
        detail: format!("invalid date in balance '{s}'"),
    })?;

    let rest = &s[7..];
    let amt = parse_amount(rest)?;

    Ok(Balance {
        dc_indicator,
        date,
        currency: amt.currency,
        amount: amt.value,
        balance_type,
    })
}

// ---------------------------------------------------------------------------
// :61: Statement Line parser
// ---------------------------------------------------------------------------

/// Parse a single `:61:` field value into a [`StatementLine`].
///
/// Format: `YYMMDD[MMDD]D/C[letter]Amount,NType Reference[//Institution][newline supplementary]`
fn parse_61(s: &str) -> Result<StatementLine, MtError> {
    let tag = "61";

    // Split on newlines to separate the first-line packed data from supplementary.
    let mut line_iter = s.lines();
    let first_line = line_iter.next().unwrap_or("").trim();
    let supplementary_lines: Vec<&str> = line_iter.collect();
    let supplementary = if supplementary_lines.is_empty() {
        None
    } else {
        Some(supplementary_lines.join("\n"))
    };

    if first_line.len() < 7 {
        return Err(MtError::InvalidFieldValue {
            tag: tag.to_string(),
            detail: format!("statement line too short: '{first_line}'"),
        });
    }

    let mut pos = 0usize;

    // Value date: YYMMDD (6 chars)
    let value_date_str = &first_line[pos..pos + 6];
    let value_date = parse_date_yymmdd(value_date_str).map_err(|_| MtError::InvalidFieldValue {
        tag: tag.to_string(),
        detail: format!("invalid value date '{value_date_str}'"),
    })?;
    // Keep the year component for entry date resolution.
    let value_year: u32 = {
        let yy: u32 = value_date_str[..2].parse().unwrap_or(0);
        if yy >= 80 {
            1900 + yy
        } else {
            2000 + yy
        }
    };
    pos += 6;

    // Optional entry date: MMDD (4 chars) — present when next chars are digits before D/C.
    let entry_date = if first_line.len() > pos + 4 {
        let maybe_mmdd = &first_line[pos..pos + 4];
        if maybe_mmdd.chars().all(|c| c.is_ascii_digit())
            && first_line.len() > pos + 4
            && matches!(first_line.chars().nth(pos + 4), Some('D' | 'C' | 'R'))
        {
            let ed = parse_date_mmdd(value_year, maybe_mmdd).ok();
            if ed.is_some() {
                pos += 4;
            }
            ed
        } else {
            None
        }
    } else {
        None
    };

    // D/C mark: D, C, RD, or RC
    let dc_mark = if first_line[pos..].starts_with("RD") {
        pos += 2;
        "RD".to_string()
    } else if first_line[pos..].starts_with("RC") {
        pos += 2;
        "RC".to_string()
    } else if first_line[pos..].starts_with('D') {
        pos += 1;
        "D".to_string()
    } else if first_line[pos..].starts_with('C') {
        pos += 1;
        "C".to_string()
    } else {
        return Err(MtError::InvalidFieldValue {
            tag: tag.to_string(),
            detail: format!("expected D/C mark at position {pos} in '{first_line}'"),
        });
    };

    // Optional funds code: single letter that is NOT a digit and NOT a comma
    // and NOT the start of the amount (which follows currency digits).
    // The funds code appears between D/C and the amount; it is a single
    // alphabetic character that does NOT belong to the amount.
    let funds_code = if pos < first_line.len() {
        let ch = first_line.chars().nth(pos).unwrap();
        if ch.is_ascii_alphabetic() {
            // Peek ahead: if the char after this is a digit, it's the funds code.
            let next = first_line.chars().nth(pos + 1);
            if matches!(next, Some(c) if c.is_ascii_digit()) {
                pos += 1;
                Some(ch)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // Amount: digits and comma (e.g. `1000,50`), ends before the transaction type.
    let amount_start = pos;
    while pos < first_line.len() {
        let ch = first_line.chars().nth(pos).unwrap();
        if ch.is_ascii_digit() || ch == ',' {
            pos += 1;
        } else {
            break;
        }
    }
    let raw_amount = &first_line[amount_start..pos];
    let amount = raw_amount.replace(',', ".");

    // Transaction type: letter + 3 chars (e.g. `NTRN`, `NMSC`).
    if pos + 4 > first_line.len() {
        return Err(MtError::InvalidFieldValue {
            tag: tag.to_string(),
            detail: format!("missing transaction type in '{first_line}'"),
        });
    }
    let transaction_type = first_line[pos..pos + 4].to_string();
    pos += 4;

    // Reference: up to `//` or end of line.
    let rest = &first_line[pos..];
    let (reference, institution_reference) = if let Some(idx) = rest.find("//") {
        (rest[..idx].to_string(), Some(rest[idx + 2..].to_string()))
    } else {
        (rest.to_string(), None)
    };

    Ok(StatementLine {
        value_date,
        entry_date,
        dc_mark,
        funds_code,
        amount,
        transaction_type,
        reference,
        institution_reference,
        supplementary,
        information: None, // filled in by the caller if :86: follows
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn require_field<'a>(
    block4: &'a Block4,
    tag: &str,
    message_type: &str,
) -> Result<&'a crate::mt::types::TagField, MtError> {
    block4.get(tag).ok_or_else(|| MtError::MissingField {
        tag: tag.to_string(),
        message_type: message_type.to_string(),
    })
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mt::parser::parse;

    const MT940_RAW: &str = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:O9401200230615BANKBEBBAXXX00000000002306151200N}\
{3:{108:MT940REF}}\
{4:
:20:STMT-REF-001
:25:NL91ABNA0417164300
:28C:1/1
:60F:C230614EUR10000,00
:61:2306150615DN1000,50NTRFREF103-001//INSTREF001
:86:Payment to supplier
:61:2306150615CR2500,00NMSCREF940-001
:86:Incoming transfer
:62F:C230615EUR11499,50
:64:C230615EUR11499,50
-}\
{5:{CHK:GHI12345678}}";

    #[test]
    fn test_parse_mt940_required_fields() {
        let msg = parse(MT940_RAW).unwrap();
        let mt = parse_mt940(&msg.block4).unwrap();
        assert_eq!(mt.transaction_reference, "STMT-REF-001");
        assert_eq!(mt.account_id, "NL91ABNA0417164300");
        assert_eq!(mt.statement_number, "1/1");
    }

    #[test]
    fn test_parse_mt940_opening_balance() {
        let msg = parse(MT940_RAW).unwrap();
        let mt = parse_mt940(&msg.block4).unwrap();
        let ob = &mt.opening_balance;
        assert_eq!(ob.dc_indicator, 'C');
        assert_eq!(ob.date, "2023-06-14");
        assert_eq!(ob.currency, "EUR");
        assert_eq!(ob.amount, "10000.00");
        assert_eq!(ob.balance_type, 'F');
    }

    #[test]
    fn test_parse_mt940_closing_balance() {
        let msg = parse(MT940_RAW).unwrap();
        let mt = parse_mt940(&msg.block4).unwrap();
        let cb = &mt.closing_balance;
        assert_eq!(cb.dc_indicator, 'C');
        assert_eq!(cb.date, "2023-06-15");
        assert_eq!(cb.amount, "11499.50");
    }

    #[test]
    fn test_parse_mt940_statement_lines() {
        let msg = parse(MT940_RAW).unwrap();
        let mt = parse_mt940(&msg.block4).unwrap();
        assert_eq!(mt.statement_lines.len(), 2);

        let sl0 = &mt.statement_lines[0];
        assert_eq!(sl0.value_date, "2023-06-15");
        assert_eq!(sl0.dc_mark, "D");
        assert_eq!(sl0.amount, "1000.50");
        assert_eq!(sl0.transaction_type, "NTRF");
        assert_eq!(sl0.reference, "REF103-001");
        assert_eq!(sl0.institution_reference.as_deref(), Some("INSTREF001"));
        assert_eq!(sl0.information.as_deref(), Some("Payment to supplier"));

        let sl1 = &mt.statement_lines[1];
        assert_eq!(sl1.dc_mark, "C");
        assert_eq!(sl1.amount, "2500.00");
        assert_eq!(sl1.information.as_deref(), Some("Incoming transfer"));
    }

    #[test]
    fn test_parse_mt940_closing_available() {
        let msg = parse(MT940_RAW).unwrap();
        let mt = parse_mt940(&msg.block4).unwrap();
        let ca = mt.closing_available.unwrap();
        assert_eq!(ca.dc_indicator, 'C');
        assert_eq!(ca.amount, "11499.50");
    }

    #[test]
    fn test_parse_mt940_missing_25_fails() {
        let raw = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:O9401200230615BANKBEBBAXXX00000000002306151200N}\
{3:}\
{4:
:20:REF
:28C:1/1
:60F:C230614EUR1000,00
:62F:C230614EUR1000,00
-}";
        let msg = parse(raw).unwrap();
        let err = parse_mt940(&msg.block4).unwrap_err();
        assert!(matches!(err, MtError::MissingField { tag, .. } if tag == "25"));
    }

    #[test]
    fn test_parse_balance_debit() {
        let field = crate::mt::types::TagField {
            tag: "60F".into(),
            value: "D230614EUR500,00".into(),
        };
        let b = parse_balance(&field).unwrap();
        assert_eq!(b.dc_indicator, 'D');
        assert_eq!(b.amount, "500.00");
    }

    #[test]
    fn test_parse_61_with_entry_date() {
        // Entry date present: 2306150615 → value=230615, entry=0615
        let sl = parse_61("2306150615CR2500,00NMSCREF940-001").unwrap();
        assert_eq!(sl.value_date, "2023-06-15");
        assert_eq!(sl.entry_date.as_deref(), Some("2023-06-15"));
        assert_eq!(sl.dc_mark, "C");
    }
}
