//! LEI (Legal Entity Identifier) validation rule.
//!
//! Validates per ISO 17442:
//! - Exactly 20 characters, all uppercase alphanumeric ASCII
//! - Characters 1–4: LOU prefix (alphanumeric)
//! - Characters 5–18: entity identifier (alphanumeric)
//! - Characters 19–20: 2-digit MOD 97-10 check digits
//!
//! The check-digit algorithm is the same MOD 97-10 used by IBAN:
//! letters are expanded (A=10, B=11, …, Z=35) before computing the remainder.

use crate::error::{Severity, ValidationError};
use crate::rules::Rule;

/// Validates a value as an ISO 17442 Legal Entity Identifier (LEI).
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::lei::LeiRule;
/// use mx20022_validate::rules::Rule;
///
/// let rule = LeiRule;
///
/// // Valid LEI (verified public registration, mod-97 == 1)
/// let errors = rule.validate("7ZW8QJWVPR4P1S5PX088", "/path");
/// assert!(errors.is_empty(), "Valid LEI should produce no errors");
///
/// let errors = rule.validate("TOOSHORT", "/path");
/// assert!(!errors.is_empty(), "Wrong-length LEI should produce errors");
/// ```
pub struct LeiRule;

impl Rule for LeiRule {
    fn id(&self) -> &'static str {
        "LEI_CHECK"
    }

    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
        match validate_lei(value) {
            Ok(()) => vec![],
            Err(msg) => vec![ValidationError::new(
                path,
                Severity::Error,
                "LEI_CHECK",
                msg,
            )],
        }
    }
}

fn validate_lei(lei: &str) -> Result<(), String> {
    // Length must be exactly 20
    if lei.len() != 20 {
        return Err(format!(
            "LEI must be exactly 20 characters, got {}: `{lei}`",
            lei.len()
        ));
    }

    // All characters must be uppercase alphanumeric ASCII
    for (i, c) in lei.chars().enumerate() {
        if !c.is_ascii_uppercase() && !c.is_ascii_digit() {
            return Err(format!(
                "LEI must contain only uppercase alphanumeric characters; \
                 character {} (`{c}`) is invalid in `{lei}`",
                i + 1
            ));
        }
    }

    // Last 2 characters must be decimal digits (check digits)
    let check_str = &lei[18..20];
    if !check_str.chars().all(|c| c.is_ascii_digit()) {
        return Err(format!(
            "LEI check digits (characters 19-20) must be decimal digits, \
             got `{check_str}` in `{lei}`"
        ));
    }

    // MOD 97-10 check per ISO 17442:
    // Expand all 20 characters (A=10, B=11, …, Z=35; digits stay) to a numeric
    // string, then compute mod 97.  A valid LEI yields remainder 1.
    // (No rearrangement — the full 20-char string is used as-is, unlike IBAN.)
    let numeric = lei_to_numeric(lei);
    let remainder = mod97(&numeric);
    if remainder != 1 {
        return Err(format!(
            "LEI check digit verification failed (mod-97 = {remainder}): `{lei}`"
        ));
    }

    Ok(())
}

/// Expand alphanumeric string for MOD 97-10: digits stay, letters become A=10…Z=35.
fn lei_to_numeric(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if c.is_ascii_digit() {
            out.push(c);
        } else {
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

    // Valid LEIs — each verified to produce mod-97 remainder == 1.
    //
    // Derivation: for a LEI with prefix P (18 chars), check digits are:
    //   98 - (numeric(P + "00") mod 97)
    // These were computed from confirmed-valid public LEI registrations.
    const VALID_LEIS: &[&str] = &[
        // 7ZW8QJWVPR4P1S5PX0 prefix → check digits 88
        "7ZW8QJWVPR4P1S5PX088",
        // 5493001KJTIIGC8Y1R prefix — verified public LEI
        "5493001KJTIIGC8Y1R12",
        // 213800WSGIIZCXF1P5 prefix — verified public LEI
        "213800WSGIIZCXF1P572",
    ];

    const INVALID_LEIS: &[&str] = &[
        "TOOSHORT",               // too short
        "7ZW8QJWVPR4P1S5PX08800", // too long (22 chars)
        "7ZW8QJWVPR4P1S5PX0!8",   // invalid character '!'
        "7zw8QJWVPR4P1S5PX088",   // lowercase 'z'
        "7ZW8QJWVPR4P1S5PX0AA",   // non-digit check digits
        "7ZW8QJWVPR4P1S5PX099",   // wrong check digits (mod-97 != 1)
        "",                       // empty
    ];

    #[test]
    fn valid_leis_pass() {
        let rule = LeiRule;
        for lei in VALID_LEIS {
            let errors = rule.validate(lei, "/test");
            assert!(
                errors.is_empty(),
                "Expected no errors for valid LEI `{lei}`, got: {errors:?}"
            );
        }
    }

    #[test]
    fn invalid_leis_fail() {
        let rule = LeiRule;
        for lei in INVALID_LEIS {
            let errors = rule.validate(lei, "/test");
            assert!(
                !errors.is_empty(),
                "Expected errors for invalid LEI `{lei}`"
            );
        }
    }

    #[test]
    fn error_has_correct_rule_id_and_path() {
        let rule = LeiRule;
        let errors = rule.validate("TOOSHORT", "/Document/LEI");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "LEI_CHECK");
        assert_eq!(errors[0].path, "/Document/LEI");
        assert_eq!(errors[0].severity, Severity::Error);
    }

    #[test]
    fn rule_id_is_lei_check() {
        assert_eq!(LeiRule.id(), "LEI_CHECK");
    }

    #[test]
    fn wrong_length_produces_length_message() {
        let rule = LeiRule;
        let errors = rule.validate("TOOSHORT", "/test");
        assert!(!errors.is_empty());
        assert!(
            errors[0].message.contains("20 characters"),
            "Expected length message, got: {}",
            errors[0].message
        );
    }

    #[test]
    fn bad_check_digits_produces_mod97_message() {
        let rule = LeiRule;
        // Valid format (20 chars, uppercase alphanumeric, numeric last 2) but wrong check digits.
        // 7ZW8QJWVPR4P1S5PX099 has '99' instead of the correct '88', so mod-97 != 1.
        let errors = rule.validate("7ZW8QJWVPR4P1S5PX099", "/test");
        assert!(!errors.is_empty());
        assert!(
            errors[0].message.contains("mod-97") || errors[0].message.contains("check digit"),
            "Expected mod-97 message, got: {}",
            errors[0].message
        );
    }

    #[test]
    fn lowercase_characters_rejected() {
        let rule = LeiRule;
        let errors = rule.validate("7zw8QJWVPR4P1S5PX085", "/test");
        assert!(!errors.is_empty());
    }
}
