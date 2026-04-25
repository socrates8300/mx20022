//! Field-level parser for MT103 Customer Credit Transfer messages.

use crate::mt::error::MtError;
use crate::mt::types::Block4;

use super::common::{parse_32a, parse_amount, parse_party_value, require_field, Amount, PartyInfo};

// ---------------------------------------------------------------------------
// Parsed representation
// ---------------------------------------------------------------------------

/// A fully parsed MT103 Customer Credit Transfer.
#[derive(Debug, Clone, PartialEq)]
pub struct Mt103 {
    /// `:20:` — Sender's Reference.
    pub senders_reference: String,
    /// `:23B:` — Bank Operation Code (e.g. `CRED`).
    pub bank_operation_code: String,
    /// `:32A:` — Value date in ISO format `YYYY-MM-DD`.
    pub value_date: String,
    /// `:32A:` — ISO 4217 currency code.
    pub currency: String,
    /// `:32A:` — Interbank settled amount as a decimal string.
    pub interbank_settled_amount: String,
    /// `:50A/F/K:` — Ordering customer.
    pub ordering_customer: PartyInfo,
    /// `:59/59A/59F:` — Beneficiary customer.
    pub beneficiary: PartyInfo,
    /// `:71A:` — Details of charges (`SHA`, `OUR`, `BEN`).
    pub details_of_charges: String,
    // Optional fields
    /// `:13C:` — Time indication.
    pub time_indication: Option<String>,
    /// `:21:` — Related reference.
    pub related_reference: Option<String>,
    /// `:33B:` — Currency/instructed amount.
    pub instructed_amount: Option<Amount>,
    /// `:36:` — Exchange rate.
    pub exchange_rate: Option<String>,
    /// `:52A/D:` — Ordering institution.
    pub ordering_institution: Option<PartyInfo>,
    /// `:53A/B/D:` — Sender's correspondent.
    pub senders_correspondent: Option<PartyInfo>,
    /// `:54A/B/D:` — Receiver's correspondent.
    pub receivers_correspondent: Option<PartyInfo>,
    /// `:56A/C/D:` — Intermediary institution.
    pub intermediary: Option<PartyInfo>,
    /// `:57A/B/C/D:` — Account with institution.
    pub account_with_institution: Option<PartyInfo>,
    /// `:70:` — Remittance information.
    pub remittance_info: Option<String>,
    /// `:71F:` — Sender's charges (repeatable).
    pub senders_charges: Vec<Amount>,
    /// `:71G:` — Receiver's charges.
    pub receivers_charges: Option<Amount>,
    /// `:72:` — Sender to receiver information.
    pub sender_to_receiver_info: Option<String>,
    /// `:77B:` — Regulatory reporting.
    pub regulatory_reporting: Option<String>,
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

/// Parse an [`Mt103`] from the already-parsed [`Block4`].
///
/// # Errors
///
/// Returns [`MtError::MissingField`] for any mandatory field absent from
/// `block4`, or [`MtError::InvalidFieldValue`] when a field cannot be
/// decoded.
pub fn parse_mt103(block4: &Block4) -> Result<Mt103, MtError> {
    let mt = "103";

    let senders_reference = require_field(block4, "20", mt)?.value.clone();
    let bank_operation_code = require_field(block4, "23B", mt)?.value.clone();

    let field_32a = require_field(block4, "32A", mt)?.value.clone();
    let (value_date, currency, interbank_settled_amount) = parse_32a(&field_32a, "32A")?;

    // :50A/F/K: — accept any suffix
    let ordering_customer = block4
        .fields
        .iter()
        .find(|f| f.tag == "50A" || f.tag == "50F" || f.tag == "50K" || f.tag == "50")
        .map(|f| parse_party_value(&f.value))
        .ok_or_else(|| MtError::MissingField {
            tag: "50A/F/K".into(),
            message_type: mt.into(),
        })?;

    // :59/59A/59F:
    let beneficiary = block4
        .fields
        .iter()
        .find(|f| f.tag == "59" || f.tag == "59A" || f.tag == "59F")
        .map(|f| parse_party_value(&f.value))
        .ok_or_else(|| MtError::MissingField {
            tag: "59/59A/59F".into(),
            message_type: mt.into(),
        })?;

    let details_of_charges = require_field(block4, "71A", mt)?.value.trim().to_string();

    // Optional fields
    let time_indication = block4.get("13C").map(|f| f.value.clone());
    let related_reference = block4.get("21").map(|f| f.value.clone());

    let instructed_amount = block4
        .get("33B")
        .map(|f| parse_amount(&f.value))
        .transpose()?;

    let exchange_rate = block4.get("36").map(|f| f.value.clone());

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
        .find(|f| f.tag == "56A" || f.tag == "56C" || f.tag == "56D")
        .map(|f| parse_party_value(&f.value));

    let account_with_institution = block4
        .fields
        .iter()
        .find(|f| f.tag == "57A" || f.tag == "57B" || f.tag == "57C" || f.tag == "57D")
        .map(|f| parse_party_value(&f.value));

    let remittance_info = block4.get("70").map(|f| f.value.clone());

    let senders_charges = block4
        .get_all("71F")
        .into_iter()
        .map(|f| parse_amount(&f.value))
        .collect::<Result<Vec<_>, _>>()?;

    let receivers_charges = block4
        .get("71G")
        .map(|f| parse_amount(&f.value))
        .transpose()?;

    let sender_to_receiver_info = block4.get("72").map(|f| f.value.clone());
    let regulatory_reporting = block4.get("77B").map(|f| f.value.clone());

    Ok(Mt103 {
        senders_reference,
        bank_operation_code,
        value_date,
        currency,
        interbank_settled_amount,
        ordering_customer,
        beneficiary,
        details_of_charges,
        time_indication,
        related_reference,
        instructed_amount,
        exchange_rate,
        ordering_institution,
        senders_correspondent,
        receivers_correspondent,
        intermediary,
        account_with_institution,
        remittance_info,
        senders_charges,
        receivers_charges,
        sender_to_receiver_info,
        regulatory_reporting,
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mt::parser::parse;

    const MT103_RAW: &str = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:I103BANKDEFFXXXXN}\
{3:{108:MT103REF}}\
{4:
:20:REFERENCE123
:23B:CRED
:32A:230615EUR1000,50
:50K:/DE89370400440532013000
JOHN DOE
123 MAIN STREET
ANYTOWN
:59:/GB29NWBK60161331926819
JANE SMITH
456 HIGH STREET
LONDON
:71A:SHA
-}\
{5:{CHK:ABC12345678}}";

    #[test]
    fn test_parse_mt103_required_fields() {
        let msg = parse(MT103_RAW).unwrap();
        let mt = parse_mt103(&msg.block4).unwrap();
        assert_eq!(mt.senders_reference, "REFERENCE123");
        assert_eq!(mt.bank_operation_code, "CRED");
        assert_eq!(mt.value_date, "2023-06-15");
        assert_eq!(mt.currency, "EUR");
        assert_eq!(mt.interbank_settled_amount, "1000.50");
        assert_eq!(mt.details_of_charges, "SHA");
    }

    #[test]
    fn test_parse_mt103_ordering_customer() {
        let msg = parse(MT103_RAW).unwrap();
        let mt = parse_mt103(&msg.block4).unwrap();
        let oc = &mt.ordering_customer;
        assert_eq!(
            oc.account.as_ref().and_then(|a| a.iban.as_deref()),
            Some("DE89370400440532013000")
        );
        assert_eq!(oc.name.as_deref(), Some("JOHN DOE"));
    }

    #[test]
    fn test_parse_mt103_beneficiary() {
        let msg = parse(MT103_RAW).unwrap();
        let mt = parse_mt103(&msg.block4).unwrap();
        let ben = &mt.beneficiary;
        assert_eq!(
            ben.account.as_ref().and_then(|a| a.iban.as_deref()),
            Some("GB29NWBK60161331926819")
        );
        assert_eq!(ben.name.as_deref(), Some("JANE SMITH"));
    }

    #[test]
    fn test_parse_mt103_senders_charges() {
        let raw = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:O1031200230615CITIUS33XXXX00000000002306151200N}\
{3:{108:MT103OUT}}\
{4:
:20:OUTREF-001
:23B:CRED
:32A:230615USD25000,00
:50K:/US33XXXX01234567890
ACME CORP
:59:/DE89370400440532013000
SUPPLIER GMBH
:71A:OUR
:71F:USD10,00
-}\
{5:{CHK:JKL12345678}}";
        let msg = parse(raw).unwrap();
        let mt = parse_mt103(&msg.block4).unwrap();
        assert_eq!(mt.senders_charges.len(), 1);
        assert_eq!(mt.senders_charges[0].currency, "USD");
        assert_eq!(mt.senders_charges[0].value, "10.00");
    }

    #[test]
    fn test_parse_mt103_missing_required_fails() {
        // Block 4 without :71A:
        let raw = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:I103BANKDEFFXXXXN}\
{3:}\
{4:
:20:REF
:23B:CRED
:32A:230615EUR100,00
:50K:ACME
:59:SMITH
-}";
        let msg = parse(raw).unwrap();
        let err = parse_mt103(&msg.block4).unwrap_err();
        assert!(matches!(err, MtError::MissingField { tag, .. } if tag == "71A"));
    }
}
