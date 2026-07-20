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

    // Validate via byte class checks rather than slicing the &str directly: a fixed
    // byte offset like `&canonical[..2]` panics if it lands inside a multibyte UTF-8
    // character (e.g. 5 CJK characters pass the 5..=34 byte-length gate above but
    // byte index 2 sits mid-character). Byte slicing on a `&[u8]` has no char
    // boundaries and never panics. Once every byte is known ASCII, the mod-97
    // rearrangement below is char-boundary safe. Mirrors the pattern in `bic.rs`.
    let bytes = canonical.as_bytes();

    // First two bytes must be uppercase ASCII letters (country code).
    if !bytes[..2].iter().all(u8::is_ascii_uppercase) {
        let got: String = canonical.chars().take(2).collect();
        return Err(format!(
            "IBAN country code must be 2 uppercase letters, got `{got}`"
        ));
    }

    // Bytes 3–4 must be decimal digits (check digits).
    if !bytes[2..4].iter().all(u8::is_ascii_digit) {
        let got: String = canonical.chars().skip(2).take(2).collect();
        return Err(format!(
            "IBAN check digits must be 2 decimal digits, got `{got}`"
        ));
    }

    // Remaining bytes (BBAN) must be ASCII alphanumeric.
    if !bytes[4..].iter().all(u8::is_ascii_alphanumeric) {
        let got: String = canonical.chars().skip(4).collect();
        return Err(format!("IBAN BBAN must be alphanumeric, got `{got}`"));
    }

    // Mod-97 check: rearrange (move first 4 chars to end), expand letters to digits,
    // compute mod 97. All bytes are validated ASCII above, so iterating as `b as
    // char` is safe and yields one character per byte.
    let rearranged: String = bytes[4..]
        .iter()
        .chain(&bytes[..4])
        .map(|&b| b as char)
        .collect();
    let numeric = alpha_to_numeric(&rearranged);
    let remainder = mod97(&numeric);
    if remainder != 1 {
        return Err(format!(
            "IBAN check digit verification failed (mod-97 = {remainder}): `{iban}`"
        ));
    }

    Ok(())
}

use super::checkdigit::{alpha_to_numeric, mod97};

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

    // Regression: multibyte UTF-8 input must not panic.
    // Previously the byte-slice boundaries `&canonical[..2]` / `[2..4]` / `[4..]`
    // panicked when a multibyte char straddled byte 2 or 4, because the length gate
    // checked byte length, not char count. See RUSTSEC-style hardening: the
    // SchemeValidator contract (schemes/mod.rs:71-72) forbids panics from Rule::validate.
    #[test]
    fn multibyte_input_returns_error_not_panic() {
        let rule = IbanRule;
        // 5 CJK characters = 15 bytes, passes the 5..=34 byte-length gate,
        // but byte index 2 lands inside the first character.
        let result = std::panic::catch_unwind(|| rule.validate("中中中中中", "/test"));
        let errors = result.expect("validate must not panic on multibyte input");
        assert!(
            !errors.is_empty(),
            "multibyte input should produce a validation error"
        );
        assert!(errors[0].message.contains("country code"));
    }

    #[test]
    fn multibyte_after_country_code_returns_error_not_panic() {
        let rule = IbanRule;
        // "GB" is valid ASCII country, but bytes 2..4 land mid-character,
        // exercising the check-digit boundary specifically.
        let result = std::panic::catch_unwind(|| rule.validate("GB中中中中", "/test"));
        let errors = result.expect("validate must not panic on multibyte input");
        assert!(
            !errors.is_empty(),
            "multibyte input should produce a validation error"
        );
        assert!(errors[0].message.contains("check digits"));
    }

    #[test]
    fn emoji_and_4_byte_chars_return_error_not_panic() {
        let rule = IbanRule;
        // 4-byte UTF-8 sequences: 5 of them = 20 bytes, passes the length gate,
        // and byte boundaries 2/4 fall deep inside surrogate pairs.
        let result = std::panic::catch_unwind(|| rule.validate("🚀🚀🚀🚀🚀", "/test"));
        let errors = result.expect("validate must not panic on 4-byte UTF-8 input");
        assert!(!errors.is_empty());
    }
}
