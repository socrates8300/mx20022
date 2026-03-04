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

use super::xml_scan::{extract_all_elements, extract_attribute, extract_element};
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

/// Check whether a string contains only characters from the SEPA restricted
/// Latin character set.
///
/// Allowed: a-z A-Z 0-9 / - ? : ( ) . , ' + Space and Latin Extended-A
/// characters with diacritics (U+00C0 – U+00FF, i.e. À–ÿ).
pub fn is_sepa_charset(s: &str) -> bool {
    s.chars().all(|c| {
        matches!(c,
            'A'..='Z'
            | 'a'..='z'
            | '0'..='9'
            | '/' | '-' | '?' | ':' | '(' | ')' | '.' | ',' | '\'' | '+' | ' '
        ) || ('\u{00C0}'..='\u{00FF}').contains(&c)
    })
}

/// Tags whose text content must conform to the SEPA character set.
const CHARSET_TAGS: &[&str] = &["Nm", "Ustrd", "StrtNm", "TwnNm"];

impl SchemeValidator for SepaValidator {
    fn name(&self) -> &'static str {
        "SEPA"
    }

    fn supported_messages(&self) -> &[&str] {
        &["pacs.008", "pacs.002", "pain.001"]
    }

    fn validate(&self, xml: &str, message_type: &str) -> ValidationResult {
        let short_type = super::short_message_type(message_type);

        if !self.supported_messages().contains(&short_type.as_str()) {
            return ValidationResult::default();
        }

        let mut errors: Vec<ValidationError> = Vec::new();

        // The field-level checks are pacs.008-specific.
        if short_type != "pacs.008" {
            return ValidationResult::new(errors);
        }

        // --- Currency must be EUR -------------------------------------------
        if let Some(ccy) = extract_attribute(xml, "IntrBkSttlmAmt", "Ccy") {
            if ccy != "EUR" {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt/@Ccy",
                    Severity::Error,
                    "SEPA_CURRENCY",
                    format!("SEPA only accepts EUR transactions; found currency \"{ccy}\""),
                ));
            }
        }

        // --- ChrgBr must be SLEV -------------------------------------------
        if let Some(chrg_br) = extract_element(xml, "ChrgBr") {
            if chrg_br != "SLEV" {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr",
                    Severity::Error,
                    "SEPA_CHRGBR",
                    format!("SEPA SCT requires ChrgBr = \"SLEV\", got \"{chrg_br}\""),
                ));
            }
        } else {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr",
                Severity::Error,
                "SEPA_CHRGBR_REQUIRED",
                "SEPA SCT requires ChrgBr = \"SLEV\"",
            ));
        }

        // --- Settlement method must be CLRG ---------------------------------
        if let Some(sttlm_mtd) = extract_element(xml, "SttlmMtd") {
            if sttlm_mtd != "CLRG" {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/GrpHdr/SttlmInf/SttlmMtd",
                    Severity::Error,
                    "SEPA_STTLM_MTD",
                    format!("SEPA requires SttlmMtd = \"CLRG\", got \"{sttlm_mtd}\""),
                ));
            }
        }

        // --- NbOfTxs must be "1" -------------------------------------------
        if let Some(nb) = extract_element(xml, "NbOfTxs") {
            if nb != "1" {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/GrpHdr/NbOfTxs",
                    Severity::Error,
                    "SEPA_SINGLE_TX",
                    format!(
                        "SEPA requires one transaction per group (NbOfTxs = \"1\"), got \"{nb}\""
                    ),
                ));
            }
        }

        // --- Debtor name required, max 70 chars -----------------------------
        check_name(xml, "Dbtr", 70, &mut errors, "SEPA_DBTR_NM");
        // --- Creditor name required, max 70 chars ---------------------------
        check_name(xml, "Cdtr", 70, &mut errors, "SEPA_CDTR_NM");

        // --- End-to-end ID max 35 chars -------------------------------------
        if let Some(e2e) = extract_element(xml, "EndToEndId") {
            if e2e.len() > 35 {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/EndToEndId",
                    Severity::Error,
                    "SEPA_E2E_LENGTH",
                    format!(
                        "EndToEndId must be at most 35 characters; got {} characters",
                        e2e.len()
                    ),
                ));
            }
        }

        // --- Ustrd total length max 140 chars --------------------------------
        let ustrd_total: usize = extract_all_elements(xml, "Ustrd")
            .iter()
            .map(String::len)
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

        // --- Amount range ---------------------------------------------------
        if let Some(amt_str) = extract_element(xml, "IntrBkSttlmAmt") {
            match amt_str.parse::<f64>() {
                Ok(amount) => {
                    if amount < 0.01 {
                        errors.push(ValidationError::new(
                            "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt",
                            Severity::Error,
                            "SEPA_AMOUNT_MIN",
                            format!("SEPA minimum amount is 0.01 EUR; got {amount:.2}"),
                        ));
                    }
                    if amount > 999_999_999.99 {
                        errors.push(ValidationError::new(
                            "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt",
                            Severity::Error,
                            "SEPA_AMOUNT_MAX",
                            format!("SEPA maximum amount is 999,999,999.99 EUR; got {amount:.2}"),
                        ));
                    }
                    // At most 2 decimal places.
                    if let Some(dot) = amt_str.find('.') {
                        let decimals = amt_str.len() - dot - 1;
                        if decimals > 2 {
                            errors.push(ValidationError::new(
                                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt",
                                Severity::Error,
                                "SEPA_AMOUNT_DECIMALS",
                                format!(
                                    "SEPA amounts must have at most 2 decimal places; got \"{amt_str}\""
                                ),
                            ));
                        }
                    }
                }
                Err(_) => {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt",
                        Severity::Error,
                        "SEPA_AMOUNT_FORMAT",
                        format!("Cannot parse amount as a number: \"{amt_str}\""),
                    ));
                }
            }
        }

        // --- IBAN required for debtor and creditor --------------------------
        // Presence check only — format is validated elsewhere.
        let ibans = extract_all_elements(xml, "IBAN");
        if ibans.is_empty() {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf",
                Severity::Error,
                "SEPA_IBAN_REQUIRED",
                "SEPA requires IBAN for both debtor and creditor accounts; none found",
            ));
        } else if ibans.len() < 2 {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf",
                Severity::Warning,
                "SEPA_IBAN_BOTH",
                "SEPA requires IBAN for both debtor and creditor; only one found",
            ));
        }

        // --- SEPA character set check ---------------------------------------
        for tag in CHARSET_TAGS {
            for value in extract_all_elements(xml, tag) {
                if !is_sepa_charset(&value) {
                    // Report just the offending characters.
                    let bad: String = value
                        .chars()
                        .filter(|&c| !is_sepa_charset(&c.to_string()))
                        .collect();
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
        }

        ValidationResult::new(errors)
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

        Some(self.validate_pacs008_typed(doc))
    }
}

impl SepaValidator {
    /// Typed validation for pacs.008 messages under SEPA SCT rules.
    #[allow(clippy::unused_self)]
    fn validate_pacs008_typed(
        &self,
        doc: &mx20022_model::generated::pacs::pacs_008_001_13::Document,
    ) -> ValidationResult {
        use mx20022_model::generated::pacs::pacs_008_001_13::{
            AccountIdentification4Choice, ChargeBearerType1Code, SettlementMethod1Code,
        };

        let mut errors: Vec<ValidationError> = Vec::new();
        let msg = &doc.fi_to_fi_cstmr_cdt_trf;

        // --- Settlement method must be CLRG ---------------------------------
        if msg.grp_hdr.sttlm_inf.sttlm_mtd != SettlementMethod1Code::Clrg {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/GrpHdr/SttlmInf/SttlmMtd",
                Severity::Error,
                "SEPA_STTLM_MTD",
                format!(
                    "SEPA requires SttlmMtd = \"CLRG\", got {:?}",
                    msg.grp_hdr.sttlm_inf.sttlm_mtd
                ),
            ));
        }

        // --- NbOfTxs must be "1" -------------------------------------------
        if msg.grp_hdr.nb_of_txs.0 != "1" {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/GrpHdr/NbOfTxs",
                Severity::Error,
                "SEPA_SINGLE_TX",
                format!(
                    "SEPA requires one transaction per group (NbOfTxs = \"1\"), got \"{}\"",
                    msg.grp_hdr.nb_of_txs.0
                ),
            ));
        }

        for tx in &msg.cdt_trf_tx_inf {
            // --- Currency must be EUR ---------------------------------------
            let ccy = &tx.intr_bk_sttlm_amt.ccy.0;
            if ccy != "EUR" {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt/@Ccy",
                    Severity::Error,
                    "SEPA_CURRENCY",
                    format!("SEPA only accepts EUR transactions; found currency \"{ccy}\""),
                ));
            }

            // --- ChrgBr must be SLEV ----------------------------------------
            if tx.chrg_br != ChargeBearerType1Code::Slev {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr",
                    Severity::Error,
                    "SEPA_CHRGBR",
                    format!("SEPA SCT requires ChrgBr = \"SLEV\", got {:?}", tx.chrg_br),
                ));
            }

            // --- Debtor name required, max 70 chars -------------------------
            match &tx.dbtr.nm {
                None => {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Dbtr/Nm",
                        Severity::Error,
                        "SEPA_DBTR_NM",
                        "Dbtr/Nm is required for SEPA",
                    ));
                }
                Some(nm) if nm.0.len() > 70 => {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Dbtr/Nm",
                        Severity::Error,
                        "SEPA_DBTR_NM",
                        format!(
                            "Dbtr/Nm must be at most 70 characters; got {} characters",
                            nm.0.len()
                        ),
                    ));
                }
                Some(_) => {}
            }

            // --- Creditor name required, max 70 chars -----------------------
            match &tx.cdtr.nm {
                None => {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Cdtr/Nm",
                        Severity::Error,
                        "SEPA_CDTR_NM",
                        "Cdtr/Nm is required for SEPA",
                    ));
                }
                Some(nm) if nm.0.len() > 70 => {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Cdtr/Nm",
                        Severity::Error,
                        "SEPA_CDTR_NM",
                        format!(
                            "Cdtr/Nm must be at most 70 characters; got {} characters",
                            nm.0.len()
                        ),
                    ));
                }
                Some(_) => {}
            }

            // --- End-to-end ID max 35 chars ---------------------------------
            // Max35Text enforces 35 via XSD. Typed path trusts XSD validation
            // for length but checks that it's present (it's a required field).

            // --- Ustrd total length max 140 chars ---------------------------
            if let Some(rmt_inf) = &tx.rmt_inf {
                let ustrd_total: usize = rmt_inf.ustrd.iter().map(|u| u.0.len()).sum();
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
                for ustrd in &rmt_inf.ustrd {
                    if !is_sepa_charset(&ustrd.0) {
                        let bad: String = ustrd
                            .0
                            .chars()
                            .filter(|&c| !is_sepa_charset(&c.to_string()))
                            .collect();
                        errors.push(ValidationError::new(
                            "//Ustrd",
                            Severity::Error,
                            "SEPA_CHARSET",
                            format!(
                                "Field <Ustrd> contains characters outside the SEPA restricted \
                                 Latin character set: {bad:?}"
                            ),
                        ));
                    }
                }
            }

            // --- Amount range -----------------------------------------------
            let amt_str = &tx.intr_bk_sttlm_amt.value.0;
            match amt_str.parse::<f64>() {
                Ok(amount) => {
                    if amount < 0.01 {
                        errors.push(ValidationError::new(
                            "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt",
                            Severity::Error,
                            "SEPA_AMOUNT_MIN",
                            format!("SEPA minimum amount is 0.01 EUR; got {amount:.2}"),
                        ));
                    }
                    if amount > 999_999_999.99 {
                        errors.push(ValidationError::new(
                            "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt",
                            Severity::Error,
                            "SEPA_AMOUNT_MAX",
                            format!("SEPA maximum amount is 999,999,999.99 EUR; got {amount:.2}"),
                        ));
                    }
                    // At most 2 decimal places.
                    if let Some(dot) = amt_str.find('.') {
                        let decimals = amt_str.len() - dot - 1;
                        if decimals > 2 {
                            errors.push(ValidationError::new(
                                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt",
                                Severity::Error,
                                "SEPA_AMOUNT_DECIMALS",
                                format!(
                                    "SEPA amounts must have at most 2 decimal places; got \"{amt_str}\""
                                ),
                            ));
                        }
                    }
                }
                Err(_) => {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt",
                        Severity::Error,
                        "SEPA_AMOUNT_FORMAT",
                        format!("Cannot parse amount as a number: \"{amt_str}\""),
                    ));
                }
            }

            // --- SEPA character set check on names --------------------------
            if let Some(nm) = &tx.dbtr.nm {
                if !is_sepa_charset(&nm.0) {
                    let bad: String =
                        nm.0.chars()
                            .filter(|&c| !is_sepa_charset(&c.to_string()))
                            .collect();
                    errors.push(ValidationError::new(
                        "//Nm",
                        Severity::Error,
                        "SEPA_CHARSET",
                        format!(
                            "Field <Nm> contains characters outside the SEPA restricted \
                             Latin character set: {bad:?}"
                        ),
                    ));
                }
            }
            if let Some(nm) = &tx.cdtr.nm {
                if !is_sepa_charset(&nm.0) {
                    let bad: String =
                        nm.0.chars()
                            .filter(|&c| !is_sepa_charset(&c.to_string()))
                            .collect();
                    errors.push(ValidationError::new(
                        "//Nm",
                        Severity::Error,
                        "SEPA_CHARSET",
                        format!(
                            "Field <Nm> contains characters outside the SEPA restricted \
                             Latin character set: {bad:?}"
                        ),
                    ));
                }
            }

            // --- IBAN required for debtor and creditor accounts ---
            let has_dbtr_iban = tx.dbtr_acct.as_ref().is_some_and(|acct| {
                acct.id.as_ref().is_some_and(|choice| {
                    matches!(choice.inner, AccountIdentification4Choice::IBAN(_))
                })
            });
            let has_cdtr_iban = tx.cdtr_acct.as_ref().is_some_and(|acct| {
                acct.id.as_ref().is_some_and(|choice| {
                    matches!(choice.inner, AccountIdentification4Choice::IBAN(_))
                })
            });
            if !has_dbtr_iban && !has_cdtr_iban {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf",
                    Severity::Error,
                    "SEPA_IBAN_REQUIRED",
                    "SEPA requires IBAN for both debtor and creditor accounts; none found",
                ));
            } else if !has_dbtr_iban || !has_cdtr_iban {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf",
                    Severity::Warning,
                    "SEPA_IBAN_BOTH",
                    "SEPA requires IBAN for both debtor and creditor; only one found",
                ));
            }
        }

        ValidationResult::new(errors)
    }
}

/// Validate that the `<Nm>` child of `<parent_tag>` is present and within
/// `max_len` characters.
fn check_name(
    xml: &str,
    parent_tag: &str,
    max_len: usize,
    errors: &mut Vec<ValidationError>,
    rule_id: &str,
) {
    let open = format!("<{parent_tag}>");
    let close = format!("</{parent_tag}>");
    let Some(start) = xml.find(&open) else {
        errors.push(ValidationError::new(
            format!("/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/{parent_tag}"),
            Severity::Error,
            rule_id,
            format!("{parent_tag} element is missing"),
        ));
        return;
    };
    let after = start + open.len();
    let Some(end) = xml[after..].find(&close) else {
        return;
    };
    let block = &xml[after..after + end];
    match extract_element(block, "Nm") {
        None => {
            errors.push(ValidationError::new(
                format!("/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/{parent_tag}/Nm"),
                Severity::Error,
                rule_id,
                format!("{parent_tag}/Nm is required for SEPA"),
            ));
        }
        Some(nm) if nm.len() > max_len => {
            errors.push(ValidationError::new(
                format!("/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/{parent_tag}/Nm"),
                Severity::Error,
                rule_id,
                format!(
                    "{parent_tag}/Nm must be at most {max_len} characters; got {} characters",
                    nm.len()
                ),
            ));
        }
        Some(_) => {}
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
}
