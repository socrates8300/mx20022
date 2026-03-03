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

use super::xml_scan::{extract_all_elements, extract_element, has_element};
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

/// Valid `ChrgBr` values under CBPR+.
const VALID_CHRGBR: &[&str] = &["CRED", "DEBT", "SHAR", "SLEV"];

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
        let short_type = message_type
            .splitn(3, '.')
            .take(2)
            .collect::<Vec<_>>()
            .join(".");

        if !self.supported_messages().contains(&short_type.as_str()) {
            return ValidationResult::default();
        }

        let mut errors: Vec<ValidationError> = Vec::new();

        // --- UTF-8 control character check ----------------------------------
        // The XML string is already valid UTF-8 (Rust strings), so we only need
        // to look for disallowed control characters.
        check_control_characters(xml, &mut errors);

        // --- Business Application Header ------------------------------------
        if !has_element(xml, "AppHdr") && !has_element(xml, "BizMsgIdr") {
            errors.push(ValidationError::new(
                "/AppHdr",
                Severity::Error,
                "CBPR_BAH_REQUIRED",
                "CBPR+ requires a Business Application Header (AppHdr / BizMsgIdr)",
            ));
        }

        // Field-level checks are pacs.008-specific.
        if short_type != "pacs.008" {
            return ValidationResult::new(errors);
        }

        // --- Mandatory BICs -------------------------------------------------
        let bic_fields: &[(&str, &str, &str)] = &[
            (
                "InstgAgt",
                "CBPR_INSTG_AGT_BIC",
                "/Document/FIToFICstmrCdtTrf/GrpHdr/InstgAgt/FinInstnId/BICFI",
            ),
            (
                "InstdAgt",
                "CBPR_INSTD_AGT_BIC",
                "/Document/FIToFICstmrCdtTrf/GrpHdr/InstdAgt/FinInstnId/BICFI",
            ),
            (
                "DbtrAgt",
                "CBPR_DBTR_AGT_BIC",
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/DbtrAgt/FinInstnId/BICFI",
            ),
            (
                "CdtrAgt",
                "CBPR_CDTR_AGT_BIC",
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/CdtrAgt/FinInstnId/BICFI",
            ),
        ];
        for (parent, rule_id, path) in bic_fields {
            check_bic_in_parent(xml, parent, path, rule_id, &mut errors);
        }

        // --- Debtor and creditor names required -----------------------------
        check_name_required(xml, "Dbtr", "CBPR_DBTR_NM_REQUIRED", &mut errors);
        check_name_required(xml, "Cdtr", "CBPR_CDTR_NM_REQUIRED", &mut errors);

        // --- UETR required --------------------------------------------------
        if !has_element(xml, "UETR") {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/UETR",
                Severity::Error,
                "CBPR_UETR_REQUIRED",
                "CBPR+ requires a UETR in PmtId",
            ));
        }

        // --- End-to-end ID required -----------------------------------------
        if !has_element(xml, "EndToEndId") {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/EndToEndId",
                Severity::Error,
                "CBPR_E2E_REQUIRED",
                "CBPR+ requires an EndToEndId in PmtId",
            ));
        }

        // --- ChrgBr required and must be valid ------------------------------
        if let Some(chrg_br) = extract_element(xml, "ChrgBr") {
            if !VALID_CHRGBR.contains(&chrg_br.as_str()) {
                errors.push(ValidationError::new(
                    "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr",
                    Severity::Error,
                    "CBPR_CHRGBR_VALUE",
                    format!("ChrgBr must be one of CRED, DEBT, SHAR, SLEV; got \"{chrg_br}\""),
                ));
            }
        } else {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr",
                Severity::Error,
                "CBPR_CHRGBR_REQUIRED",
                "CBPR+ requires ChrgBr (one of CRED, DEBT, SHAR, SLEV)",
            ));
        }

        // --- Interbank settlement date required -----------------------------
        if !has_element(xml, "IntrBkSttlmDt") {
            errors.push(ValidationError::new(
                "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmDt",
                Severity::Error,
                "CBPR_STTLM_DT_REQUIRED",
                "CBPR+ requires IntrBkSttlmDt",
            ));
        }

        // --- BIC padding check (8-char BICs should be 11) ------------------
        for bic in extract_all_elements(xml, "BICFI") {
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

        ValidationResult::new(errors)
    }
}

/// Check for disallowed control characters in the XML string.
///
/// CBPR+ requires UTF-8 encoding with no control characters except:
/// - LF (0x0A)
/// - CR (0x0D)
/// - TAB (0x09)
fn check_control_characters(xml: &str, errors: &mut Vec<ValidationError>) {
    for (i, c) in xml.char_indices() {
        if c.is_control() && !matches!(c, '\n' | '\r' | '\t') {
            errors.push(ValidationError::new(
                "/Document",
                Severity::Error,
                "CBPR_CONTROL_CHAR",
                format!(
                    "Disallowed control character U+{:04X} at byte offset {i}",
                    c as u32
                ),
            ));
            // Report only the first occurrence to avoid noise.
            break;
        }
    }
}

/// Check that a `BICFI` element exists inside a parent block.
fn check_bic_in_parent(
    xml: &str,
    parent_tag: &str,
    path: &str,
    rule_id: &str,
    errors: &mut Vec<ValidationError>,
) {
    let open = format!("<{parent_tag}>");
    let close = format!("</{parent_tag}>");
    let Some(start) = xml.find(&open) else {
        errors.push(ValidationError::new(
            path,
            Severity::Error,
            rule_id,
            format!("{parent_tag}/FinInstnId/BICFI is required for CBPR+ but the parent element is missing"),
        ));
        return;
    };
    let after = start + open.len();
    let Some(end) = xml[after..].find(&close) else {
        return;
    };
    let block = &xml[after..after + end];
    if !has_element(block, "BICFI") {
        errors.push(ValidationError::new(
            path,
            Severity::Error,
            rule_id,
            format!("{parent_tag}/FinInstnId/BICFI is required for CBPR+"),
        ));
    }
}

/// Check that a `Nm` element exists inside a parent block.
fn check_name_required(
    xml: &str,
    parent_tag: &str,
    rule_id: &str,
    errors: &mut Vec<ValidationError>,
) {
    let open = format!("<{parent_tag}>");
    let close = format!("</{parent_tag}>");
    let Some(start) = xml.find(&open) else {
        errors.push(ValidationError::new(
            format!("/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/{parent_tag}/Nm"),
            Severity::Error,
            rule_id,
            format!("{parent_tag}/Nm is required for CBPR+ but {parent_tag} element is missing"),
        ));
        return;
    };
    let after = start + open.len();
    let Some(end) = xml[after..].find(&close) else {
        return;
    };
    let block = &xml[after..after + end];
    if !has_element(block, "Nm") && super::xml_scan::extract_element(block, "Nm").is_none() {
        errors.push(ValidationError::new(
            format!("/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/{parent_tag}/Nm"),
            Severity::Error,
            rule_id,
            format!("{parent_tag}/Nm is required for CBPR+"),
        ));
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
        let mut errors = Vec::new();
        check_control_characters("hello\x01world", &mut errors);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "CBPR_CONTROL_CHAR");
    }

    #[test]
    fn allowed_whitespace_is_fine() {
        let mut errors = Vec::new();
        check_control_characters("hello\nworld\r\n\t", &mut errors);
        assert!(errors.is_empty());
    }
}
