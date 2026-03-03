//! Field-level parser for MT202 Bank-to-Bank Credit Transfer messages.

use crate::mt::error::MtError;
use crate::mt::types::Block4;

use super::common::{parse_amount, parse_date_yymmdd, parse_party_value, PartyInfo};

// ---------------------------------------------------------------------------
// Parsed representation
// ---------------------------------------------------------------------------

/// A fully parsed MT202 Bank-to-Bank Credit Transfer.
#[derive(Debug, Clone, PartialEq)]
pub struct Mt202 {
    /// `:20:` — Transaction Reference Number.
    pub transaction_reference: String,
    /// `:21:` — Related Reference.
    pub related_reference: String,
    /// `:32A:` — Value date in ISO format `YYYY-MM-DD`.
    pub value_date: String,
    /// `:32A:` — ISO 4217 currency code.
    pub currency: String,
    /// `:32A:` — Amount as a decimal string.
    pub amount: String,
    /// `:58A/D:` — Beneficiary institution.
    pub beneficiary_institution: PartyInfo,
    // Optional fields
    /// `:13C:` — Time indication.
    pub time_indication: Option<String>,
    /// `:52A/D:` — Ordering institution.
    pub ordering_institution: Option<PartyInfo>,
    /// `:53A/B/D:` — Sender's correspondent.
    pub senders_correspondent: Option<PartyInfo>,
    /// `:54A/B/D:` — Receiver's correspondent.
    pub receivers_correspondent: Option<PartyInfo>,
    /// `:56A/D:` — Intermediary institution.
    pub intermediary: Option<PartyInfo>,
    /// `:72:` — Sender to receiver information.
    pub sender_to_receiver_info: Option<String>,
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

/// Parse an [`Mt202`] from the already-parsed [`Block4`].
///
/// # Errors
///
/// Returns [`MtError::MissingField`] for mandatory fields or
/// [`MtError::InvalidFieldValue`] when a field value is malformed.
pub fn parse_mt202(block4: &Block4) -> Result<Mt202, MtError> {
    let mt = "202";

    let transaction_reference = require_field(block4, "20", mt)?.value.clone();
    let related_reference = require_field(block4, "21", mt)?.value.clone();

    let field_32a = require_field(block4, "32A", mt)?.value.clone();
    let (value_date, currency, amount) = parse_32a(&field_32a, "32A")?;

    let beneficiary_institution = block4
        .fields
        .iter()
        .find(|f| f.tag == "58A" || f.tag == "58D")
        .map(|f| parse_party_value(&f.value))
        .ok_or_else(|| MtError::MissingField {
            tag: "58A/D".into(),
            message_type: mt.into(),
        })?;

    // Optional
    let time_indication = block4.get("13C").map(|f| f.value.clone());

    let ordering_institution = block4
        .fields
        .iter()
        .find(|f| f.tag == "52A" || f.tag == "52D")
        .map(|f| parse_party_value(&f.value));

    let senders_correspondent = block4
        .fields
        .iter()
        .find(|f| f.tag == "53A" || f.tag == "53B" || f.tag == "53D")
        .map(|f| parse_party_value(&f.value));

    let receivers_correspondent = block4
        .fields
        .iter()
        .find(|f| f.tag == "54A" || f.tag == "54B" || f.tag == "54D")
        .map(|f| parse_party_value(&f.value));

    let intermediary = block4
        .fields
        .iter()
        .find(|f| f.tag == "56A" || f.tag == "56D")
        .map(|f| parse_party_value(&f.value));

    let sender_to_receiver_info = block4.get("72").map(|f| f.value.clone());

    Ok(Mt202 {
        transaction_reference,
        related_reference,
        value_date,
        currency,
        amount,
        beneficiary_institution,
        time_indication,
        ordering_institution,
        senders_correspondent,
        receivers_correspondent,
        intermediary,
        sender_to_receiver_info,
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

fn parse_32a(s: &str, tag: &str) -> Result<(String, String, String), MtError> {
    if s.len() < 10 {
        return Err(MtError::InvalidFieldValue {
            tag: tag.to_string(),
            detail: format!("too short: '{s}'"),
        });
    }
    let date = parse_date_yymmdd(&s[..6]).map_err(|_| MtError::InvalidFieldValue {
        tag: tag.to_string(),
        detail: format!("invalid date in '{s}'"),
    })?;
    let rest = &s[6..];
    let amt = parse_amount(rest)?;
    Ok((date, amt.currency, amt.value))
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mt::parser::parse;

    const MT202_RAW: &str = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:I202BANKDEFFXXXXN}\
{3:{108:MT202REF}}\
{4:
:20:TXN-REF-202-001
:21:REL-REF-001
:32A:230615USD50000,00
:58A:CHASUS33XXX
-}\
{5:{CHK:DEF12345678}}";

    #[test]
    fn test_parse_mt202_required_fields() {
        let msg = parse(MT202_RAW).unwrap();
        let mt = parse_mt202(&msg.block4).unwrap();
        assert_eq!(mt.transaction_reference, "TXN-REF-202-001");
        assert_eq!(mt.related_reference, "REL-REF-001");
        assert_eq!(mt.value_date, "2023-06-15");
        assert_eq!(mt.currency, "USD");
        assert_eq!(mt.amount, "50000.00");
    }

    #[test]
    fn test_parse_mt202_beneficiary() {
        let msg = parse(MT202_RAW).unwrap();
        let mt = parse_mt202(&msg.block4).unwrap();
        // :58A: with BIC-only value — no leading '/' so stored as raw name.
        assert_eq!(
            mt.beneficiary_institution.name.as_deref(),
            Some("CHASUS33XXX")
        );
    }

    #[test]
    fn test_parse_mt202_missing_21_fails() {
        let raw = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:I202BANKDEFFXXXXN}\
{3:}\
{4:
:20:REF
:32A:230615USD1000,00
:58A:CHASUS33
-}";
        let msg = parse(raw).unwrap();
        let err = parse_mt202(&msg.block4).unwrap_err();
        assert!(matches!(err, MtError::MissingField { tag, .. } if tag == "21"));
    }

    #[test]
    fn test_parse_mt202_with_optional_fields() {
        let raw = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:I202BANKDEFFXXXXN}\
{3:}\
{4:
:20:TXN202
:21:RELREF
:32A:230615EUR25000,00
:52A:DEUTDEDB
:58A:CHASUS33
:72:/ACC/ADDITIONAL INFO
-}";
        let msg = parse(raw).unwrap();
        let mt = parse_mt202(&msg.block4).unwrap();
        assert!(mt.ordering_institution.is_some());
        assert_eq!(
            mt.sender_to_receiver_info.as_deref(),
            Some("/ACC/ADDITIONAL INFO")
        );
    }
}
