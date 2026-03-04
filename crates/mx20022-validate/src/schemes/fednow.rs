//! `FedNow` payment scheme validator.
//!
//! The Federal Reserve's real-time gross settlement service imposes
//! additional constraints on top of the base ISO 20022 schema:
//!
//! - Only USD transactions are accepted.
//! - Settlement method must be `CLRG`.
//! - Charges bearer must be `SLEV`.
//! - A single transaction per group (`NbOfTxs = "1"`).
//! - UETR is mandatory (UUID v4 format).
//! - End-to-end ID is mandatory (≤ 35 characters).
//! - Amount in `[0.01, 500_000.00]` USD (the upper bound is configurable up to
//!   25,000,000.00 USD for high-value participants).
//! - Message size limits: 64 KB for pacs.008 / pacs.004, 32 KB for pacs.028.

use std::any::Any;
use std::sync::OnceLock;

use regex::Regex;

use super::xml_scan::{extract_attribute, extract_element, has_element, xml_byte_size};
use super::SchemeValidator;
use crate::error::{Severity, ValidationError, ValidationResult};

/// `FedNow` scheme validator.
///
/// # Examples
///
/// ```
/// use mx20022_validate::schemes::fednow::FedNowValidator;
/// use mx20022_validate::schemes::SchemeValidator;
///
/// let validator = FedNowValidator::new();
/// assert_eq!(validator.name(), "FedNow");
/// assert!(validator.supported_messages().contains(&"pacs.008"));
/// ```
pub struct FedNowValidator {
    /// Maximum permitted settlement amount in USD.
    max_amount: f64,
}

impl FedNowValidator {
    /// Create a validator with the standard 500,000 USD limit.
    pub fn new() -> Self {
        Self {
            max_amount: 500_000.0,
        }
    }

    /// Create a validator with a custom maximum amount (e.g. 25,000,000 USD for
    /// high-value participants).
    pub fn with_max_amount(max_amount: f64) -> Self {
        Self { max_amount }
    }
}

impl Default for FedNowValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Compiled UUID v4 regex, cached for the lifetime of the process.
fn uetr_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$")
            .expect("valid regex")
    })
}

/// UUID v4 pattern (8-4-4-4-12 hex groups).
fn is_valid_uetr(value: &str) -> bool {
    uetr_re().is_match(value)
}

impl SchemeValidator for FedNowValidator {
    fn name(&self) -> &'static str {
        "FedNow"
    }

    fn supported_messages(&self) -> &[&str] {
        &[
            "pacs.008", "pacs.002", "pacs.004", "pacs.028", "camt.056", "pain.013",
        ]
    }

    fn validate(&self, xml: &str, message_type: &str) -> ValidationResult {
        let short_type = super::short_message_type(message_type);

        if !self.supported_messages().contains(&short_type.as_str()) {
            return ValidationResult::default();
        }

        let mut errors: Vec<ValidationError> = Vec::new();

        // --- Message size ---------------------------------------------------
        let size = xml_byte_size(xml);
        let size_limit: usize = if short_type == "pacs.028" {
            32 * 1024
        } else {
            64 * 1024
        };
        if size > size_limit {
            errors.push(ValidationError::new(
                "/Document",
                Severity::Error,
                "FEDNOW_MSG_SIZE",
                format!(
                    "Message size {size} bytes exceeds FedNow limit of {size_limit} bytes for {short_type}"
                ),
            ));
        }

        // The remaining checks are pacs.008-specific field rules.
        if short_type != "pacs.008" {
            return ValidationResult::new(errors);
        }

        // --- NbOfTxs must be "1" -------------------------------------------
        if let Some(nb) = extract_element(xml, "NbOfTxs") {
            if nb != "1" {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/GrpHdr/NbOfTxs",
                    Severity::Error,
                    "FEDNOW_SINGLE_TX",
                    format!(
                        "FedNow requires exactly one transaction per group (NbOfTxs = \"1\"), got \"{nb}\""
                    ),
                ));
            }
        }

        // --- Settlement method must be CLRG ---------------------------------
        if let Some(sttlm_mtd) = extract_element(xml, "SttlmMtd") {
            if sttlm_mtd != "CLRG" {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/GrpHdr/SttlmInf/SttlmMtd",
                    Severity::Error,
                    "FEDNOW_STTLM_MTD",
                    format!("FedNow requires SttlmMtd = \"CLRG\", got \"{sttlm_mtd}\""),
                ));
            }
        }

        // --- ChrgBr must be SLEV --------------------------------------------
        if let Some(chrg_br) = extract_element(xml, "ChrgBr") {
            if chrg_br != "SLEV" {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr",
                    Severity::Error,
                    "FEDNOW_CHRGBR",
                    format!("FedNow requires ChrgBr = \"SLEV\", got \"{chrg_br}\""),
                ));
            }
        }

        // --- Currency must be USD -------------------------------------------
        if let Some(ccy) = extract_attribute(xml, "IntrBkSttlmAmt", "Ccy") {
            if ccy != "USD" {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt/@Ccy",
                    Severity::Error,
                    "FEDNOW_CURRENCY",
                    format!("FedNow only accepts USD transactions; found currency \"{ccy}\""),
                ));
            }
        }

        // --- Amount range ---------------------------------------------------
        if let Some(amt_str) = extract_element(xml, "IntrBkSttlmAmt") {
            self.validate_amount(
                &amt_str,
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt",
                &mut errors,
            );
        }

        // --- UETR is required and must be UUID v4 ---------------------------
        if let Some(uetr) = extract_element(xml, "UETR") {
            if !is_valid_uetr(&uetr) {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/UETR",
                    Severity::Error,
                    "FEDNOW_UETR_FORMAT",
                    format!("UETR must be a valid UUID v4; got \"{uetr}\""),
                ));
            }
        } else {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/UETR",
                Severity::Error,
                "FEDNOW_UETR_REQUIRED",
                "FedNow requires a UETR (UUID v4) in PmtId",
            ));
        }

        // --- End-to-end ID is required and max 35 chars ---------------------
        if let Some(e2e) = extract_element(xml, "EndToEndId") {
            if e2e.len() > 35 {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/EndToEndId",
                    Severity::Error,
                    "FEDNOW_E2E_LENGTH",
                    format!(
                        "EndToEndId must be at most 35 characters; got {} characters",
                        e2e.len()
                    ),
                ));
            }
        } else {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/EndToEndId",
                Severity::Error,
                "FEDNOW_E2E_REQUIRED",
                "FedNow requires an EndToEndId in PmtId",
            ));
        }

        // --- Debtor name max 140 chars --------------------------------------
        // We check the first <Nm> inside <Dbtr>. A simple heuristic: scan for
        // the Dbtr block and extract the Nm within it.
        check_name_length(xml, "Dbtr", &mut errors, "FEDNOW_DBTR_NM_LENGTH");
        check_name_length(xml, "Cdtr", &mut errors, "FEDNOW_CDTR_NM_LENGTH");

        // --- RmtInf/Ustrd max 140 chars per element -------------------------
        for ustrd in super::xml_scan::extract_all_elements(xml, "Ustrd") {
            if ustrd.len() > 140 {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/RmtInf/Ustrd",
                    Severity::Error,
                    "FEDNOW_USTRD_LENGTH",
                    format!(
                        "Ustrd element must be at most 140 characters; got {} characters",
                        ustrd.len()
                    ),
                ));
            }
        }

        // --- AppHdr presence is a soft check (not required but common) ------
        if !has_element(xml, "AppHdr") && !has_element(xml, "BizMsgIdr") {
            errors.push(ValidationError::new(
                "/AppHdr",
                Severity::Warning,
                "FEDNOW_APPHDR_MISSING",
                "Business Application Header (AppHdr) is recommended for FedNow messages",
            ));
        }

        ValidationResult::new(errors)
    }

    fn validate_typed(&self, msg: &dyn Any, message_type: &str) -> Option<ValidationResult> {
        use mx20022_model::generated::pacs::pacs_008_001_13;

        let short_type = super::short_message_type(message_type);
        if !self.supported_messages().contains(&short_type.as_str()) {
            return None;
        }

        // Only pacs.008 has typed field-level checks.
        if short_type != "pacs.008" {
            return None;
        }

        let doc = msg.downcast_ref::<pacs_008_001_13::Document>()?;

        Some(self.validate_pacs008_typed(doc))
    }
}

impl FedNowValidator {
    fn validate_amount(&self, amt_str: &str, path: &str, errors: &mut Vec<ValidationError>) {
        let decimal_ok = amt_str
            .find('.')
            .is_some_and(|dot| amt_str.len() - dot - 1 == 2);
        if !decimal_ok {
            errors.push(ValidationError::new(
                path,
                Severity::Error,
                "FEDNOW_AMOUNT_DECIMALS",
                format!("FedNow amounts must have exactly 2 decimal places; got \"{amt_str}\""),
            ));
        }
        match parse_amount_cents(amt_str) {
            Some(cents) => {
                if cents < 1 {
                    errors.push(ValidationError::new(
                        path,
                        Severity::Error,
                        "FEDNOW_AMOUNT_MIN",
                        format!("FedNow minimum amount is 0.01 USD; got \"{amt_str}\""),
                    ));
                }
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let max_cents = (self.max_amount * 100.0) as u64;
                if cents > max_cents {
                    errors.push(ValidationError::new(
                        path,
                        Severity::Error,
                        "FEDNOW_AMOUNT_LIMIT",
                        format!(
                            "FedNow maximum amount is {:.2} USD; got \"{amt_str}\"",
                            self.max_amount
                        ),
                    ));
                }
            }
            None => {
                errors.push(ValidationError::new(
                    path,
                    Severity::Error,
                    "FEDNOW_AMOUNT_FORMAT",
                    format!("Cannot parse amount as a number: \"{amt_str}\""),
                ));
            }
        }
    }

    /// Typed validation for pacs.008 messages.
    fn validate_pacs008_typed(
        &self,
        doc: &mx20022_model::generated::pacs::pacs_008_001_13::Document,
    ) -> ValidationResult {
        use mx20022_model::generated::pacs::pacs_008_001_13::{
            ChargeBearerType1Code, SettlementMethod1Code,
        };

        let mut errors: Vec<ValidationError> = Vec::new();
        let msg = &doc.fi_to_fi_cstmr_cdt_trf;

        // --- NbOfTxs must be "1" -------------------------------------------
        if msg.grp_hdr.nb_of_txs.0 != "1" {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/GrpHdr/NbOfTxs",
                Severity::Error,
                "FEDNOW_SINGLE_TX",
                format!(
                    "FedNow requires exactly one transaction per group (NbOfTxs = \"1\"), got \"{}\"",
                    msg.grp_hdr.nb_of_txs.0
                ),
            ));
        }

        // --- Settlement method must be CLRG ---------------------------------
        if msg.grp_hdr.sttlm_inf.sttlm_mtd != SettlementMethod1Code::Clrg {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/GrpHdr/SttlmInf/SttlmMtd",
                Severity::Error,
                "FEDNOW_STTLM_MTD",
                format!(
                    "FedNow requires SttlmMtd = \"CLRG\", got {:?}",
                    msg.grp_hdr.sttlm_inf.sttlm_mtd
                ),
            ));
        }

        // Validate each credit transfer transaction.
        for tx in &msg.cdt_trf_tx_inf {
            // --- ChrgBr must be SLEV ----------------------------------------
            if tx.chrg_br != ChargeBearerType1Code::Slev {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr",
                    Severity::Error,
                    "FEDNOW_CHRGBR",
                    format!("FedNow requires ChrgBr = \"SLEV\", got {:?}", tx.chrg_br),
                ));
            }

            // --- Currency must be USD ---------------------------------------
            let ccy = &tx.intr_bk_sttlm_amt.ccy.0;
            if ccy != "USD" {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt/@Ccy",
                    Severity::Error,
                    "FEDNOW_CURRENCY",
                    format!("FedNow only accepts USD transactions; found currency \"{ccy}\""),
                ));
            }

            // --- Amount range -----------------------------------------------
            let amt_str = &tx.intr_bk_sttlm_amt.value.0;
            self.validate_amount(
                amt_str,
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt",
                &mut errors,
            );

            // --- UETR is required and must be UUID v4 -----------------------
            match &tx.pmt_id.uetr {
                Some(uetr) if !is_valid_uetr(&uetr.0) => {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/UETR",
                        Severity::Error,
                        "FEDNOW_UETR_FORMAT",
                        format!("UETR must be a valid UUID v4; got \"{}\"", uetr.0),
                    ));
                }
                None => {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/UETR",
                        Severity::Error,
                        "FEDNOW_UETR_REQUIRED",
                        "FedNow requires a UETR (UUID v4) in PmtId",
                    ));
                }
                Some(_) => {} // Valid UETR
            }

            // --- End-to-end ID is required (length covered by XSD) ----------
            // Max35Text is the type — XSD validation handles the 35-char limit.
            // We only need to check it's not empty/whitespace for FedNow.
            if tx.pmt_id.end_to_end_id.0.trim().is_empty() {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/EndToEndId",
                    Severity::Error,
                    "FEDNOW_E2E_REQUIRED",
                    "FedNow requires a non-empty EndToEndId in PmtId",
                ));
            }

            // --- Debtor name max 140 chars ----------------------------------
            if let Some(nm) = &tx.dbtr.nm {
                if nm.0.len() > 140 {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Dbtr/Nm",
                        Severity::Error,
                        "FEDNOW_DBTR_NM_LENGTH",
                        format!(
                            "Dbtr/Nm must be at most 140 characters; got {} characters",
                            nm.0.len()
                        ),
                    ));
                }
            }

            // --- Creditor name max 140 chars --------------------------------
            if let Some(nm) = &tx.cdtr.nm {
                if nm.0.len() > 140 {
                    errors.push(ValidationError::new(
                        "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Cdtr/Nm",
                        Severity::Error,
                        "FEDNOW_CDTR_NM_LENGTH",
                        format!(
                            "Cdtr/Nm must be at most 140 characters; got {} characters",
                            nm.0.len()
                        ),
                    ));
                }
            }

            // --- RmtInf/Ustrd max 140 chars per element ---------------------
            if let Some(rmt_inf) = &tx.rmt_inf {
                for ustrd in &rmt_inf.ustrd {
                    if ustrd.0.len() > 140 {
                        errors.push(ValidationError::new(
                            "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/RmtInf/Ustrd",
                            Severity::Error,
                            "FEDNOW_USTRD_LENGTH",
                            format!(
                                "Ustrd element must be at most 140 characters; got {} characters",
                                ustrd.0.len()
                            ),
                        ));
                    }
                }
            }
        }

        // Note: AppHdr check and message size check require raw XML and are
        // not covered by the typed path. Those remain in the XML-based
        // `validate` method.

        ValidationResult::new(errors)
    }
}

/// Parse a decimal amount string like `"1000.50"` into integer cents (`100050`).
///
/// # Contract
///
/// The input **must** contain exactly one `.` followed by exactly **2** decimal
/// digits (e.g. `"100.50"`, `"0.01"`). This matches the format enforced by the
/// XSD `ActiveCurrencyAndAmount` type that all callers validate against before
/// reaching this function.
///
/// Returns `None` for non-conforming input:
/// - No decimal point (e.g. `"100"`)
/// - Integer or fractional part fails `u64` parsing (e.g. `"abc.50"`, `"100.ab"`)
///
/// A `debug_assert!` fires in test/dev builds if the fractional part is not
/// exactly 2 digits, catching misuse early without penalising release builds.
pub(crate) fn parse_amount_cents(s: &str) -> Option<u64> {
    let dot = s.find('.')?;
    let integer: u64 = s[..dot].parse().ok()?;
    let frac_str = &s[dot + 1..];
    debug_assert!(
        frac_str.len() == 2,
        "parse_amount_cents expects exactly 2 decimal digits, got {frac_str:?}"
    );
    let frac: u64 = frac_str.parse().ok()?;
    Some(integer * 100 + frac)
}

/// Like [`parse_amount_cents`], but accepts 0–2 decimal digits.
///
/// - `"1000"` → `Some(100_000)`
/// - `"1000.5"` → `Some(100_050)`
/// - `"1000.50"` → `Some(100_050)`
/// - `"1000.500"` → `None` (>2 decimal digits)
/// - `"abc"` → `None`
pub(crate) fn parse_amount_cents_lenient(s: &str) -> Option<u64> {
    if let Some(dot) = s.find('.') {
        let integer: u64 = s[..dot].parse().ok()?;
        let frac_str = &s[dot + 1..];
        let frac: u64 = match frac_str.len() {
            0 => 0,
            1 => frac_str.parse::<u64>().ok()? * 10,
            2 => frac_str.parse().ok()?,
            _ => return None,
        };
        Some(integer * 100 + frac)
    } else {
        let integer: u64 = s.parse().ok()?;
        Some(integer * 100)
    }
}

/// Check that the `<Nm>` child inside `<parent_tag>` does not exceed 140 chars.
fn check_name_length(
    xml: &str,
    parent_tag: &str,
    errors: &mut Vec<ValidationError>,
    rule_id: &str,
) {
    // Find the parent element, then look for a Nm within its content.
    let open = format!("<{parent_tag}>");
    let close = format!("</{parent_tag}>");
    let Some(start) = xml.find(&open) else { return };
    let after = start + open.len();
    let Some(end) = xml[after..].find(&close) else {
        return;
    };
    let block = &xml[after..after + end];
    if let Some(nm) = extract_element(block, "Nm") {
        if nm.len() > 140 {
            errors.push(ValidationError::new(
                format!("/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/{parent_tag}/Nm"),
                Severity::Error,
                rule_id,
                format!(
                    "{parent_tag}/Nm must be at most 140 characters; got {} characters",
                    nm.len()
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_is_fednow() {
        assert_eq!(FedNowValidator::new().name(), "FedNow");
    }

    #[test]
    fn supports_pacs008() {
        let v = FedNowValidator::new();
        assert!(v.supported_messages().contains(&"pacs.008"));
    }

    #[test]
    fn unsupported_message_returns_empty() {
        let v = FedNowValidator::new();
        let result = v.validate("<xml/>", "pacs.009.001.10");
        assert!(result.errors.is_empty());
    }

    #[test]
    fn valid_uetr_accepted() {
        assert!(is_valid_uetr("97ed4827-7b6f-4491-a06f-b548d5a7512d"));
    }

    #[test]
    fn invalid_uetr_rejected() {
        assert!(!is_valid_uetr("not-a-uuid"));
        assert!(!is_valid_uetr("97ed4827-7b6f-3491-a06f-b548d5a7512d")); // version 3, not 4
    }

    #[test]
    fn default_max_amount_is_500k() {
        let v = FedNowValidator::default();
        assert!((v.max_amount - 500_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn custom_max_amount() {
        let v = FedNowValidator::with_max_amount(25_000_000.0);
        assert!((v.max_amount - 25_000_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn parse_amount_cents_normal() {
        assert_eq!(parse_amount_cents("100.50"), Some(10050));
    }

    #[test]
    fn parse_amount_cents_minimum() {
        assert_eq!(parse_amount_cents("0.01"), Some(1));
    }

    #[test]
    fn parse_amount_cents_large() {
        assert_eq!(parse_amount_cents("999999.99"), Some(99999999));
    }

    #[test]
    fn parse_amount_cents_no_dot() {
        assert_eq!(parse_amount_cents("100"), None);
    }

    #[test]
    fn parse_amount_cents_bad_integer() {
        assert_eq!(parse_amount_cents("abc.50"), None);
    }

    #[test]
    fn parse_amount_cents_bad_fraction() {
        assert_eq!(parse_amount_cents("100.ab"), None);
    }
}
