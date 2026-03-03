//! BIC (Bank Identifier Code) / SWIFT code validation rule.
//!
//! Validates per ISO 9362:2022:
//! - 4 alpha-only characters: institution code
//! - 2 alpha-only characters: country code (ISO 3166-1 alpha-2)
//! - 2 alphanumeric characters: location code
//! - Optional 3 alphanumeric characters: branch code
//!
//! Total length: 8 or 11 characters.
//!
//! The pattern also matches the generated type annotation:
//! `[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}`

use crate::error::{Severity, ValidationError};
use crate::rules::Rule;

/// Validates a value as a BIC/SWIFT code.
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::bic::BicRule;
/// use mx20022_validate::rules::Rule;
///
/// let rule = BicRule;
/// let errors = rule.validate("AAAAGB2L", "/path");
/// assert!(errors.is_empty(), "Valid BIC-8 should produce no errors");
///
/// let errors = rule.validate("AAAAGB2LXXX", "/path");
/// assert!(errors.is_empty(), "Valid BIC-11 should produce no errors");
///
/// let errors = rule.validate("INVALID", "/path");
/// assert!(!errors.is_empty(), "Invalid BIC should produce errors");
/// ```
pub struct BicRule;

impl Rule for BicRule {
    fn id(&self) -> &'static str {
        "BIC_CHECK"
    }

    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
        match validate_bic(value) {
            Ok(()) => vec![],
            Err(msg) => vec![ValidationError::new(
                path,
                Severity::Error,
                "BIC_CHECK",
                msg,
            )],
        }
    }
}

fn validate_bic(bic: &str) -> Result<(), String> {
    let len = bic.len();
    if len != 8 && len != 11 {
        return Err(format!(
            "BIC must be 8 or 11 characters, got {len}: `{bic}`"
        ));
    }

    let bytes = bic.as_bytes();

    // Characters 1-4: institution code — uppercase alpha only
    for (i, &b) in bytes[..4].iter().enumerate() {
        if !b.is_ascii_uppercase() {
            return Err(format!(
                "BIC institution code (chars 1-4) must be uppercase letters; \
                 char {} is `{}`",
                i + 1,
                char::from(b)
            ));
        }
    }

    // Characters 5-6: country code — uppercase alpha only
    for (i, &b) in bytes[4..6].iter().enumerate() {
        if !b.is_ascii_uppercase() {
            return Err(format!(
                "BIC country code (chars 5-6) must be uppercase letters; \
                 char {} is `{}`",
                i + 5,
                char::from(b)
            ));
        }
    }

    // Characters 7-8: location code — alphanumeric (uppercase)
    for (i, &b) in bytes[6..8].iter().enumerate() {
        if !b.is_ascii_uppercase() && !b.is_ascii_digit() {
            return Err(format!(
                "BIC location code (chars 7-8) must be uppercase alphanumeric; \
                 char {} is `{}`",
                i + 7,
                char::from(b)
            ));
        }
    }

    // Characters 9-11 (optional): branch code — alphanumeric (uppercase)
    if len == 11 {
        for (i, &b) in bytes[8..11].iter().enumerate() {
            if !b.is_ascii_uppercase() && !b.is_ascii_digit() {
                return Err(format!(
                    "BIC branch code (chars 9-11) must be uppercase alphanumeric; \
                     char {} is `{}`",
                    i + 9,
                    char::from(b)
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Rule;

    const VALID_BICS: &[&str] = &[
        "AAAAGB2L",    // 8-char BIC
        "AAAAGB2LXXX", // 11-char BIC (XXX = primary office)
        "DEUTDEDB",    // Deutsche Bank Frankfurt
        "DEUTDEDBFRA", // Deutsche Bank Frankfurt with branch
        "BOFAUS3N",    // Bank of America
        "BOFAUS3NXXX", // Bank of America with branch
        "CHASUS33",    // JPMorgan Chase
        "CHASUS33XXX", // JPMorgan Chase with branch
    ];

    const INVALID_BICS: &[&str] = &[
        "AAAA",          // too short
        "AAAAGB2LXXXXX", // too long (13 chars)
        "1AAAGB2L",      // institution code has digit
        "AAAA1B2L",      // country code has digit
        "AAAAgb2L",      // lowercase
        "AAAAGB2l",      // lowercase location
        "",              // empty
        "AAAA GB2L",     // contains space
    ];

    #[test]
    fn valid_bics_pass() {
        let rule = BicRule;
        for bic in VALID_BICS {
            let errors = rule.validate(bic, "/test");
            assert!(
                errors.is_empty(),
                "Expected no errors for valid BIC `{bic}`, got: {errors:?}"
            );
        }
    }

    #[test]
    fn invalid_bics_fail() {
        let rule = BicRule;
        for bic in INVALID_BICS {
            let errors = rule.validate(bic, "/test");
            assert!(
                !errors.is_empty(),
                "Expected errors for invalid BIC `{bic}`"
            );
        }
    }

    #[test]
    fn error_has_correct_rule_id_and_path() {
        let rule = BicRule;
        let errors = rule.validate("INVALID", "/some/path");
        assert_eq!(errors[0].rule_id, "BIC_CHECK");
        assert_eq!(errors[0].path, "/some/path");
    }

    #[test]
    fn rule_id_is_bic_check() {
        assert_eq!(BicRule.id(), "BIC_CHECK");
    }
}
