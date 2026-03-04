//! IBAN (International Bank Account Number) validation rule.
//!
//! Validates per ISO 13616:
//! - 2-letter country code
//! - 2 decimal check digits
//! - Up to 30 alphanumeric BBAN characters
//! - Total length between 5 and 34 characters
//! - Mod-97 check digit verification

use crate::error::{Severity, ValidationError};
use crate::rules::Rule;

/// Validates a value as an IBAN using format and mod-97 check digit.
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::iban::IbanRule;
/// use mx20022_validate::rules::Rule;
///
/// let rule = IbanRule;
/// let errors = rule.validate("GB82WEST12345698765432", "/path");
/// assert!(errors.is_empty(), "Valid IBAN should produce no errors");
///
/// let errors = rule.validate("INVALID", "/path");
/// assert!(!errors.is_empty(), "Invalid IBAN should produce errors");
/// ```
pub struct IbanRule;

impl Rule for IbanRule {
    fn id(&self) -> &'static str {
        "IBAN_CHECK"
    }

    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
        match validate_iban(value) {
            Ok(()) => vec![],
            Err(msg) => vec![ValidationError::new(
                path,
                Severity::Error,
                "IBAN_CHECK",
                msg,
            )],
        }
    }
}

/// Core IBAN validation logic — returns `Ok(())` on success or an error message.
fn validate_iban(iban: &str) -> Result<(), String> {
    // Strip optional spaces (some representations include spaces every 4 chars)
    let canonical: String = iban.chars().filter(|c| !c.is_whitespace()).collect();

    let len = canonical.len();
    if !(5..=34).contains(&len) {
        return Err(format!(
            "IBAN length {len} is out of range [5, 34]: `{iban}`"
        ));
    }

    // First two characters must be uppercase ASCII letters (country code)
    let country = &canonical[..2];
    if !country.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(format!(
            "IBAN country code must be 2 uppercase letters, got `{country}`"
        ));
    }

    // Characters 3–4 must be decimal digits (check digits)
    let check_str = &canonical[2..4];
    if !check_str.chars().all(|c| c.is_ascii_digit()) {
        return Err(format!(
            "IBAN check digits must be 2 decimal digits, got `{check_str}`"
        ));
    }

    // BBAN: remaining characters must be alphanumeric
    let bban = &canonical[4..];
    if !bban.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(format!("IBAN BBAN must be alphanumeric, got `{bban}`"));
    }

    // Mod-97 check: rearrange (move first 4 chars to end), expand letters to digits, compute mod 97
    let rearranged = format!("{}{}", bban, &canonical[..4]);
    let numeric = iban_to_numeric(&rearranged);
    let remainder = mod97(&numeric);
    if remainder != 1 {
        return Err(format!(
            "IBAN check digit verification failed (mod-97 = {remainder}): `{iban}`"
        ));
    }

    Ok(())
}

/// Convert an IBAN string (letters → digits, A=10, B=11, …, Z=35) to a numeric string.
fn iban_to_numeric(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if c.is_ascii_digit() {
            out.push(c);
        } else {
            // A=10, B=11, …, Z=35
            let n = (c as u32) - ('A' as u32) + 10;
            out.push_str(&n.to_string());
        }
    }
    out
}

use super::checkdigit::mod97;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Rule;

    // Known-valid IBANs from Wikipedia / IBAN registry
    const VALID_IBANS: &[&str] = &[
        "GB82WEST12345698765432",
        "DE89370400440532013000",
        "FR7630006000011234567890189",
        "NL91ABNA0417164300",
        "BE71096123456769",
        "CH9300762011623852957",
        "SE4550000000058398257466",
        "NO9386011117947",
    ];

    const INVALID_IBANS: &[&str] = &[
        "GB82WEST1234569876543X",  // non-alphanumeric BBAN
        "GB82WEST123456987654321", // wrong mod-97
        "12WEST12345698765432",    // non-letter country code
        "GBXWEST12345698765432",   // non-digit check digits
        "GB",                      // too short
        "",                        // empty
        "INVALID",                 // too short and wrong format
    ];

    #[test]
    fn valid_ibans_pass() {
        let rule = IbanRule;
        for iban in VALID_IBANS {
            let errors = rule.validate(iban, "/test");
            assert!(
                errors.is_empty(),
                "Expected no errors for valid IBAN `{iban}`, got: {errors:?}"
            );
        }
    }

    #[test]
    fn invalid_ibans_fail() {
        let rule = IbanRule;
        for iban in INVALID_IBANS {
            let errors = rule.validate(iban, "/test");
            assert!(
                !errors.is_empty(),
                "Expected errors for invalid IBAN `{iban}`"
            );
        }
    }

    #[test]
    fn error_has_correct_rule_id() {
        let rule = IbanRule;
        let errors = rule.validate("INVALID", "/some/path");
        assert_eq!(errors[0].rule_id, "IBAN_CHECK");
        assert_eq!(errors[0].path, "/some/path");
    }

    #[test]
    fn rule_id_is_iban_check() {
        assert_eq!(IbanRule.id(), "IBAN_CHECK");
    }

    #[test]
    fn iban_with_spaces_is_normalised() {
        // "GB82 WEST 1234 5698 7654 32" == "GB82WEST12345698765432"
        let rule = IbanRule;
        let errors = rule.validate("GB82 WEST 1234 5698 7654 32", "/test");
        assert!(errors.is_empty(), "IBAN with spaces should be accepted");
    }
}
