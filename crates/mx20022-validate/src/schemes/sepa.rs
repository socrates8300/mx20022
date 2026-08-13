//! SEPA (Single Euro Payments Area) scheme validator.
//!
//! Enforces European Payments Council (EPC) usage guidelines for the SEPA
//! Credit Transfer (SCT) scheme:
//!
//! - Only EUR transactions.
//! - Settlement method must be `CLRG`; charges bearer must be `SLEV`.
//! - Single transaction per group (`NbOfTxs = "1"`).
//! - Debtor and creditor names are required (≤ 70 characters each).
//! - End-to-end ID ≤ 35 characters.
//! - `RmtInf/Ustrd` ≤ 140 characters in total.
//! - Amount in `[0.01, 999_999_999.99]` EUR with at most 2 decimal places.
//! - IBAN is required for both debtor and creditor accounts.
//! - SEPA restricted Latin character set on name and address fields.

use std::any::Any;

use super::SchemeValidator;
use crate::error::{Severity, ValidationError, ValidationResult};

/// SEPA Credit Transfer scheme validator.
///
/// # Examples
///
/// ```
/// use mx20022_validate::schemes::sepa::SepaValidator;
/// use mx20022_validate::schemes::SchemeValidator;
///
/// let validator = SepaValidator::new();
/// assert_eq!(validator.name(), "SEPA");
/// assert!(validator.supported_messages().contains(&"pacs.008"));
/// ```
pub struct SepaValidator;

impl SepaValidator {
    /// Create a new `SepaValidator`.
    pub fn new() -> Self {
        Self
    }
}

impl Default for SepaValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns `true` if `c` is in the SEPA restricted Latin character set.
fn is_sepa_char(c: char) -> bool {
    matches!(c,
        'A'..='Z'
        | 'a'..='z'
        | '0'..='9'
        | '/' | '-' | '?' | ':' | '(' | ')' | '.' | ',' | '\'' | '+' | ' '
    ) || ('\u{00C0}'..='\u{00FF}').contains(&c)
}

/// Check whether a string contains only characters from the SEPA restricted
/// Latin character set.
///
/// Allowed: a-z A-Z 0-9 / - ? : ( ) . , ' + Space and Latin Extended-A
/// characters with diacritics (U+00C0 – U+00FF, i.e. À–ÿ).
pub fn is_sepa_charset(s: &str) -> bool {
    s.chars().all(is_sepa_char)
}

impl SchemeValidator for SepaValidator {
    fn name(&self) -> &'static str {
        "SEPA"
    }

    fn supported_messages(&self) -> &[&str] {
        &["pacs.008", "pacs.002", "pain.001"]
    }

    fn validate(&self, xml: &str, message_type: &str) -> ValidationResult {
        super::run_xml(
            self,
            xml,
            message_type,
            |_, _| ValidationResult::default(),
            |facts| self.validate_pacs008(facts),
        )
    }

    fn validate_typed(&self, msg: &dyn Any, message_type: &str) -> Option<ValidationResult> {
        use mx20022_model::generated::pacs::pacs_008_001_13;

        let short_type = super::short_message_type(message_type);
        if !self.supported_messages().contains(&short_type.as_str()) {
            return None;
        }

        if short_type != "pacs.008" {
            return None;
        }

        let doc = msg.downcast_ref::<pacs_008_001_13::Document>()?;

        let facts = match super::pacs008::Facts::from_typed_with_additional_charset_fields(doc) {
            Ok(facts) => facts,
            Err(error) => return Some(super::scheme_parse_error(&error)),
        };

        Some(self.validate_pacs008(&facts))
    }
}

impl SepaValidator {
    /// Validate a SEPA amount string: decimal places, min, max.
    fn validate_sepa_amount(amt_str: &str, path: &str, errors: &mut Vec<ValidationError>) {
        let decimals = amt_str.find('.').map_or(0, |dot| amt_str.len() - dot - 1);
        if decimals > 2 {
            errors.push(ValidationError::new(
                path,
                Severity::Error,
                "SEPA_AMOUNT_DECIMALS",
                format!("SEPA amounts must have at most 2 decimal places; got \"{amt_str}\""),
            ));
        }
        match super::common::parse_amount_cents_lenient(amt_str) {
            Some(cents) => {
                if cents < 1 {
                    errors.push(ValidationError::new(
                        path,
                        Severity::Error,
                        "SEPA_AMOUNT_MIN",
                        format!("SEPA minimum amount is 0.01 EUR; got \"{amt_str}\""),
                    ));
                }
                if cents > 99_999_999_999 {
                    errors.push(ValidationError::new(
                        path,
                        Severity::Error,
                        "SEPA_AMOUNT_MAX",
                        format!("SEPA maximum amount is 999,999,999.99 EUR; got \"{amt_str}\""),
                    ));
                }
            }
            None => {
                errors.push(ValidationError::new(
                    path,
                    Severity::Error,
                    "SEPA_AMOUNT_FORMAT",
                    format!("Cannot parse amount as a number: \"{amt_str}\""),
                ));
            }
        }
    }

    /// Check that a field conforms to the SEPA restricted Latin character set.
    fn check_sepa_text(tag: &str, value: &str, errors: &mut Vec<ValidationError>) {
        if !is_sepa_charset(value) {
            let bad: String = value.chars().filter(|&c| !is_sepa_char(c)).collect();
            errors.push(ValidationError::new(
                format!("//{tag}"),
                Severity::Error,
                "SEPA_CHARSET",
                format!(
                    "Field <{tag}> contains characters outside the SEPA restricted \
                     Latin character set: {bad:?}"
                ),
            ));
        }
    }

    /// Validate version-neutral pacs.008 facts under SEPA SCT rules.
    #[allow(clippy::unused_self)]
    fn validate_pacs008(&self, facts: &super::pacs008::Facts) -> ValidationResult {
        let mut errors: Vec<ValidationError> = Vec::new();

        // --- Settlement method must be CLRG ---------------------------------
        if let Some(settlement_method) = &facts.settlement_method {
            if settlement_method != "CLRG" {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/GrpHdr/SttlmInf/SttlmMtd",
                    Severity::Error,
                    "SEPA_STTLM_MTD",
                    format!("SEPA requires SttlmMtd = \"CLRG\", got \"{settlement_method}\""),
                ));
            }
        }

        // --- Declared and actual transaction count must both be one ---------
        let actual_transactions = facts.transactions.len();
        if facts.nb_of_txs.as_deref() != Some("1") || actual_transactions != 1 {
            let declared_transactions = facts.nb_of_txs.as_deref().unwrap_or("<missing>");
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/GrpHdr/NbOfTxs",
                Severity::Error,
                "SEPA_SINGLE_TX",
                format!(
                    "SEPA requires exactly one transaction per group (NbOfTxs = \"1\" and one CdtTrfTxInf); declared \"{declared_transactions}\", found {actual_transactions} CdtTrfTxInf element(s)"
                ),
            ));
        }

        for tx in &facts.transactions {
            // --- Currency must be EUR ---------------------------------------
            if let Some(currency) = &tx.currency {
                if currency != "EUR" {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt/@Ccy",
                        Severity::Error,
                        "SEPA_CURRENCY",
                        format!(
                            "SEPA only accepts EUR transactions; found currency \"{currency}\""
                        ),
                    ));
                }
            }

            // --- ChrgBr must be SLEV ----------------------------------------
            match tx.charge_bearer.as_deref() {
                Some("SLEV") => {}
                Some(charge_bearer) => errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr",
                    Severity::Error,
                    "SEPA_CHRGBR",
                    format!("SEPA SCT requires ChrgBr = \"SLEV\", got \"{charge_bearer}\""),
                )),
                None => errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr",
                    Severity::Error,
                    "SEPA_CHRGBR_REQUIRED",
                    "SEPA SCT requires ChrgBr = \"SLEV\"",
                )),
            }

            // --- Debtor name required, max 70 chars -------------------------
            match &tx.debtor_name {
                None => {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Dbtr/Nm",
                        Severity::Error,
                        "SEPA_DBTR_NM",
                        "Dbtr/Nm is required for SEPA",
                    ));
                }
                Some(name) if name.chars().count() > 70 => {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Dbtr/Nm",
                        Severity::Error,
                        "SEPA_DBTR_NM",
                        format!(
                            "Dbtr/Nm must be at most 70 characters; got {} characters",
                            name.chars().count()
                        ),
                    ));
                }
                Some(_) => {}
            }

            // --- Creditor name required, max 70 chars -----------------------
            match &tx.creditor_name {
                None => {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Cdtr/Nm",
                        Severity::Error,
                        "SEPA_CDTR_NM",
                        "Cdtr/Nm is required for SEPA",
                    ));
                }
                Some(name) if name.chars().count() > 70 => {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Cdtr/Nm",
                        Severity::Error,
                        "SEPA_CDTR_NM",
                        format!(
                            "Cdtr/Nm must be at most 70 characters; got {} characters",
                            name.chars().count()
                        ),
                    ));
                }
                Some(_) => {}
            }

            // --- End-to-end ID max 35 Unicode scalars -----------------------
            if let Some(end_to_end_id) = &tx.end_to_end_id {
                let end_to_end_length = end_to_end_id.chars().count();
                if end_to_end_length > 35 {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/EndToEndId",
                        Severity::Error,
                        "SEPA_E2E_LENGTH",
                        format!(
                            "EndToEndId must be at most 35 characters; got {end_to_end_length} characters"
                        ),
                    ));
                }
            }

            // --- Ustrd total length max 140 chars ---------------------------
            if !tx.unstructured_remittance.is_empty() {
                let ustrd_total: usize = tx
                    .unstructured_remittance
                    .iter()
                    .map(|value| value.chars().count())
                    .sum();
                if ustrd_total > 140 {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/RmtInf/Ustrd",
                        Severity::Error,
                        "SEPA_USTRD_LENGTH",
                        format!(
                            "RmtInf/Ustrd total length must not exceed 140 characters; got {ustrd_total}"
                        ),
                    ));
                }

                // SEPA character set check on Ustrd.
                for unstructured in &tx.unstructured_remittance {
                    Self::check_sepa_text("Ustrd", unstructured, &mut errors);
                }
            }

            // --- Amount range -----------------------------------------------
            if let Some(amount) = &tx.amount {
                Self::validate_sepa_amount(
                    amount,
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt",
                    &mut errors,
                );
            }

            // --- SEPA character set check on names --------------------------
            if let Some(name) = &tx.debtor_name {
                Self::check_sepa_text("Nm", name, &mut errors);
            }
            if let Some(name) = &tx.creditor_name {
                Self::check_sepa_text("Nm", name, &mut errors);
            }

            // --- IBAN required for debtor and creditor accounts ---
            if !tx.has_debtor_iban && !tx.has_creditor_iban {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf",
                    Severity::Error,
                    "SEPA_IBAN_REQUIRED",
                    "SEPA requires IBAN for both debtor and creditor accounts; none found",
                ));
            } else if !tx.has_debtor_iban || !tx.has_creditor_iban {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf",
                    Severity::Warning,
                    "SEPA_IBAN_BOTH",
                    "SEPA requires IBAN for both debtor and creditor; only one found",
                ));
            }
        }

        // --- SEPA character set check on other names and addresses ---------
        for field in &facts.additional_charset_fields {
            Self::check_sepa_text(field.tag, &field.value, &mut errors);
        }

        ValidationResult::new(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_is_sepa() {
        assert_eq!(SepaValidator::new().name(), "SEPA");
    }

    #[test]
    fn supports_pacs008() {
        let v = SepaValidator::new();
        assert!(v.supported_messages().contains(&"pacs.008"));
    }

    #[test]
    fn unsupported_message_returns_empty() {
        let v = SepaValidator::new();
        let result = v.validate("<xml/>", "pacs.009.001.10");
        assert!(result.errors.is_empty());
    }

    #[test]
    fn sepa_charset_ascii_allowed() {
        assert!(is_sepa_charset("Alice Smith / 123"));
    }

    #[test]
    fn sepa_charset_diacritics_allowed() {
        assert!(is_sepa_charset("Müller")); // ü is U+00FC, in range
    }

    #[test]
    fn sepa_charset_control_chars_rejected() {
        assert!(!is_sepa_charset("Alice\x01Smith"));
    }

    #[test]
    fn sepa_charset_cyrillic_rejected() {
        assert!(!is_sepa_charset("Алиса")); // Cyrillic
    }

    fn validate_amount(amount: &str) -> ValidationResult {
        let mut errors = Vec::new();
        SepaValidator::validate_sepa_amount(amount, "/Document/Amount", &mut errors);
        ValidationResult::new(errors)
    }

    fn has_error(result: &ValidationResult, code: &str) -> bool {
        result.errors.iter().any(|e| e.rule_id == code)
    }

    #[test]
    fn sepa_amount_at_max_boundary() {
        let result = validate_amount("999999999.99");
        assert!(
            !has_error(&result, "SEPA_AMOUNT_MAX"),
            "999999999.99 should be within SEPA max; errors: {:?}",
            result.errors
        );
    }

    #[test]
    fn sepa_amount_just_under_max() {
        let result = validate_amount("999999999.98");
        assert!(!has_error(&result, "SEPA_AMOUNT_MAX"));
    }

    #[test]
    fn sepa_amount_exceeds_max() {
        let result = validate_amount("1000000000.00");
        assert!(
            has_error(&result, "SEPA_AMOUNT_MAX"),
            "1000000000.00 should exceed SEPA max"
        );
    }

    #[test]
    fn sepa_amount_at_min_boundary() {
        let result = validate_amount("0.01");
        assert!(
            !has_error(&result, "SEPA_AMOUNT_MIN"),
            "0.01 should be within SEPA min"
        );
    }

    #[test]
    fn sepa_amount_below_min() {
        let result = validate_amount("0.00");
        assert!(
            has_error(&result, "SEPA_AMOUNT_MIN"),
            "0.00 should be below SEPA min"
        );
    }
}
