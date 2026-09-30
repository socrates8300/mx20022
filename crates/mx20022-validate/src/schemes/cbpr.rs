//! CBPR+ (Cross-Border Payments and Reporting Plus) scheme validator.
//!
//! Enforces Swift's CBPR+ usage guidelines for pacs.008 and related messages:
//!
//! - Business Application Header (`AppHdr`) is mandatory.
//! - Instructing and instructed agent BICs are required.
//! - Debtor agent and creditor agent BICs are required.
//! - Debtor and creditor names are required.
//! - UETR is mandatory in `PmtId`.
//! - End-to-end ID is mandatory.
//! - Charges bearer (`ChrgBr`) is required and must be one of: CRED, DEBT,
//!   SHAR, SLEV.
//! - Interbank settlement date is required.
//! - All BICs should be 11 characters (8-char BICs generate a warning).
//! - UTF-8 only; no control characters other than LF, CR, TAB.

use std::any::Any;

use super::SchemeValidator;
use crate::error::{Severity, ValidationError, ValidationResult};

/// CBPR+ scheme validator.
///
/// # Examples
///
/// ```
/// use mx20022_validate::schemes::cbpr::CbprPlusValidator;
/// use mx20022_validate::schemes::SchemeValidator;
///
/// let validator = CbprPlusValidator::new();
/// assert_eq!(validator.name(), "CBPR+");
/// assert!(validator.supported_messages().contains(&"pacs.008"));
/// ```
pub struct CbprPlusValidator;

impl CbprPlusValidator {
    /// Create a new `CbprPlusValidator`.
    pub fn new() -> Self {
        Self
    }
}

impl Default for CbprPlusValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemeValidator for CbprPlusValidator {
    fn name(&self) -> &'static str {
        "CBPR+"
    }

    fn supported_messages(&self) -> &[&str] {
        &[
            "pacs.008", "pacs.009", "pacs.002", "pacs.004", "camt.056", "camt.029",
        ]
    }

    fn validate(&self, xml: &str, message_type: &str) -> ValidationResult {
        super::run_xml(
            self,
            xml,
            message_type,
            |xml, _| Self::validate_raw(xml),
            Self::validate_pacs008,
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

        Some(Self::validate_pacs008(&super::pacs008::Facts::from(doc)))
    }
}

impl CbprPlusValidator {
    /// Validate rules that require the original XML envelope.
    fn validate_raw(xml: &str) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some((offset, character)) = super::raw::first_disallowed_control(xml) {
            errors.push(ValidationError::new(
                "/Document",
                Severity::Error,
                "CBPR_CONTROL_CHAR",
                format!(
                    "Disallowed control character U+{:04X} at byte offset {offset}",
                    character as u32
                ),
            ));
        }

        let (has_app_header, has_business_message_id) = super::raw::header_presence(xml);
        if !has_app_header && !has_business_message_id {
            errors.push(ValidationError::new(
                "/AppHdr",
                Severity::Error,
                "CBPR_BAH_REQUIRED",
                "CBPR+ requires a Business Application Header (AppHdr / BizMsgIdr)",
            ));
        }

        ValidationResult::new(errors)
    }

    /// Validate version-neutral pacs.008 facts under CBPR+ rules.
    fn validate_pacs008(facts: &super::pacs008::Facts) -> ValidationResult {
        let mut errors: Vec<ValidationError> = Vec::new();

        // --- Instructing agent BIC required (GrpHdr level) ------------------
        if facts.instg_agent_bic.is_none() {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/GrpHdr/InstgAgt/FinInstnId/BICFI",
                Severity::Error,
                "CBPR_INSTG_AGT_BIC",
                "InstgAgt/FinInstnId/BICFI is required for CBPR+",
            ));
        }

        // --- Instructed agent BIC required (GrpHdr level) -------------------
        if facts.instd_agent_bic.is_none() {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/GrpHdr/InstdAgt/FinInstnId/BICFI",
                Severity::Error,
                "CBPR_INSTD_AGT_BIC",
                "InstdAgt/FinInstnId/BICFI is required for CBPR+",
            ));
        }

        // --- BIC padding check for GrpHdr-level agents ---
        for bic in [
            facts.instg_agent_bic.as_deref(),
            facts.instd_agent_bic.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            if bic.len() == 8 {
                errors.push(ValidationError::new(
                    "//BICFI",
                    Severity::Warning,
                    "CBPR_BIC_PADDING",
                    format!(
                        "CBPR+ recommends 11-character BICs; \"{bic}\" is 8 characters (pad with XXX)"
                    ),
                ));
            }
        }

        for tx in &facts.transactions {
            // --- Debtor agent BIC required ----------------------------------
            if tx.debtor_agent_bic.is_none() {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/DbtrAgt/FinInstnId/BICFI",
                    Severity::Error,
                    "CBPR_DBTR_AGT_BIC",
                    "DbtrAgt/FinInstnId/BICFI is required for CBPR+",
                ));
            }

            // --- Creditor agent BIC required --------------------------------
            if tx.creditor_agent_bic.is_none() {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/CdtrAgt/FinInstnId/BICFI",
                    Severity::Error,
                    "CBPR_CDTR_AGT_BIC",
                    "CdtrAgt/FinInstnId/BICFI is required for CBPR+",
                ));
            }

            // --- BIC padding check (8-char BICs should be 11) ---------------
            for bic in [
                tx.debtor_agent_bic.as_deref(),
                tx.creditor_agent_bic.as_deref(),
            ]
            .into_iter()
            .flatten()
            {
                if bic.len() == 8 {
                    errors.push(ValidationError::new(
                        "//BICFI",
                        Severity::Warning,
                        "CBPR_BIC_PADDING",
                        format!(
                            "CBPR+ recommends 11-character BICs; \"{bic}\" is 8 characters (pad with XXX)"
                        ),
                    ));
                }
            }

            // --- Debtor name required ---------------------------------------
            if tx.debtor_name.is_none() {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Dbtr/Nm",
                    Severity::Error,
                    "CBPR_DBTR_NM_REQUIRED",
                    "Dbtr/Nm is required for CBPR+",
                ));
            }

            // --- Creditor name required -------------------------------------
            if tx.creditor_name.is_none() {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Cdtr/Nm",
                    Severity::Error,
                    "CBPR_CDTR_NM_REQUIRED",
                    "Cdtr/Nm is required for CBPR+",
                ));
            }

            // --- UETR required ----------------------------------------------
            if tx.uetr.is_none() {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/UETR",
                    Severity::Error,
                    "CBPR_UETR_REQUIRED",
                    "CBPR+ requires a UETR in PmtId",
                ));
            }

            // --- End-to-end ID required -------------------------------------
            if tx.end_to_end_id.is_none() {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/EndToEndId",
                    Severity::Error,
                    "CBPR_E2E_REQUIRED",
                    "CBPR+ requires an EndToEndId in PmtId",
                ));
            }

            // --- Charges bearer required and schema-valid ------------------
            match tx.charge_bearer.as_deref() {
                Some("CRED" | "DEBT" | "SHAR" | "SLEV") => {}
                Some(charge_bearer) => errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr",
                    Severity::Error,
                    "CBPR_CHRGBR_VALUE",
                    format!(
                        "ChrgBr must be one of CRED, DEBT, SHAR, SLEV; got \"{charge_bearer}\""
                    ),
                )),
                None => errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr",
                    Severity::Error,
                    "CBPR_CHRGBR_REQUIRED",
                    "CBPR+ requires ChrgBr (one of CRED, DEBT, SHAR, SLEV)",
                )),
            }

            // --- Interbank settlement date required -------------------------
            if !tx.has_settlement_date {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmDt",
                    Severity::Error,
                    "CBPR_STTLM_DT_REQUIRED",
                    "CBPR+ requires IntrBkSttlmDt",
                ));
            }
        }

        ValidationResult::new(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_is_cbpr_plus() {
        assert_eq!(CbprPlusValidator::new().name(), "CBPR+");
    }

    #[test]
    fn supports_pacs008() {
        let v = CbprPlusValidator::new();
        assert!(v.supported_messages().contains(&"pacs.008"));
    }

    #[test]
    fn unsupported_message_returns_empty() {
        let v = CbprPlusValidator::new();
        let result = v.validate("<xml/>", "pain.001.001.09");
        assert!(result.errors.is_empty());
    }

    #[test]
    fn control_character_produces_error() {
        let result = CbprPlusValidator::validate_raw("hello\x01world");
        assert_eq!(result.errors[0].rule_id, "CBPR_CONTROL_CHAR");
    }

    #[test]
    fn allowed_whitespace_is_fine() {
        let result = CbprPlusValidator::validate_raw(
            "<AppHdr><BizMsgIdr>id</BizMsgIdr>hello\nworld\r\n\t</AppHdr>",
        );
        assert!(result.errors.is_empty());
    }
}
