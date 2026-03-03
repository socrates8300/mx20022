//! ISO 20022 amount format validation rule.
//!
//! ISO 20022 decimal amounts (e.g. `ActiveCurrencyAndAmount`) must be:
//! - A positive decimal number in string form
//! - Greater than zero
//! - At most 18 integer digits
//! - At most 5 fractional digits

use crate::error::{Severity, ValidationError};
use crate::rules::Rule;

/// Validates a decimal amount string per ISO 20022 constraints.
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::amount::AmountFormatRule;
/// use mx20022_validate::rules::Rule;
///
/// let rule = AmountFormatRule;
///
/// let errors = rule.validate("1000.00", "/path");
/// assert!(errors.is_empty(), "Valid amount should produce no errors");
///
/// let errors = rule.validate("-1.00", "/path");
/// assert!(!errors.is_empty(), "Negative amount should be rejected");
///
/// let errors = rule.validate("0", "/path");
/// assert!(!errors.is_empty(), "Zero amount should be rejected");
///
/// let errors = rule.validate("1.123456", "/path");
/// assert!(!errors.is_empty(), "Too many fractional digits should be rejected");
/// ```
pub struct AmountFormatRule;

/// Maximum allowed integer digits (ISO 20022 constraint).
const MAX_INTEGER_DIGITS: usize = 18;
/// Maximum allowed fractional digits (ISO 20022 constraint).
const MAX_FRACTIONAL_DIGITS: usize = 5;

impl Rule for AmountFormatRule {
    fn id(&self) -> &'static str {
        "AMOUNT_FORMAT"
    }

    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
        match validate_amount(value) {
            Ok(()) => vec![],
            Err(msg) => {
                vec![ValidationError::new(
                    path,
                    Severity::Error,
                    "AMOUNT_FORMAT",
                    msg,
                )]
            }
        }
    }
}

fn validate_amount(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("Amount must not be empty".to_owned());
    }

    // Reject anything that doesn't look like a positive decimal number.
    // Accept optional leading sign only if it's '+' (no '-' allowed).
    let stripped = value.strip_prefix('+').unwrap_or(value);

    // Must not start with '-'
    if value.starts_with('-') {
        return Err(format!("Amount must be positive (> 0), got: `{value}`"));
    }

    // Split into integer and fractional parts
    let (integer_part, fractional_part) = match stripped.split_once('.') {
        Some((int, frac)) => (int, Some(frac)),
        None => (stripped, None),
    };

    // Validate integer part: must be non-empty and all digits
    if integer_part.is_empty() {
        return Err(format!("Amount has no integer part: `{value}`"));
    }

    if !integer_part.chars().all(|c| c.is_ascii_digit()) {
        return Err(format!(
            "Amount contains non-numeric characters in integer part: `{value}`"
        ));
    }

    // Count significant integer digits (strip leading zeros for the count, but
    // ISO 20022 measures total digits before the decimal point)
    let integer_digits = integer_part.len();
    if integer_digits > MAX_INTEGER_DIGITS {
        return Err(format!(
            "Amount integer part has {integer_digits} digits, maximum is {MAX_INTEGER_DIGITS}: `{value}`"
        ));
    }

    // Validate fractional part if present
    if let Some(frac) = fractional_part {
        if frac.is_empty() {
            return Err(format!("Amount has trailing decimal point: `{value}`"));
        }
        if !frac.chars().all(|c| c.is_ascii_digit()) {
            return Err(format!(
                "Amount contains non-numeric characters in fractional part: `{value}`"
            ));
        }
        if frac.len() > MAX_FRACTIONAL_DIGITS {
            return Err(format!(
                "Amount has {} fractional digits, maximum is {MAX_FRACTIONAL_DIGITS}: `{value}`",
                frac.len()
            ));
        }
    }

    // Must be > 0: parse as f64 and check (safe since we've already validated
    // the format — we only need to distinguish zero from positive)
    let parsed: f64 = value
        .parse()
        .map_err(|_| format!("Amount is not a valid decimal number: `{value}`"))?;
    if parsed <= 0.0 {
        return Err(format!("Amount must be greater than zero, got: `{value}`"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Rule;

    const VALID_AMOUNTS: &[&str] = &[
        "1",
        "1.0",
        "1.00",
        "1000.00",
        "999999999999999999", // 18 integer digits
        "0.12345",            // 5 fractional digits
        "1000000000.12345",
        "0.00001",
        "+1.00", // explicit positive sign
        "1.1",
    ];

    const INVALID_AMOUNTS: &[&str] = &[
        "-1.00",               // negative
        "0",                   // zero (integer)
        "0.00",                // zero (decimal)
        "0.00000",             // zero with max fractional
        "1.123456",            // 6 fractional digits
        "1234567890123456789", // 19 integer digits
        "abc",                 // non-numeric
        "1.2.3",               // multiple decimal points
        "",                    // empty
        "1.",                  // trailing decimal point
        ".5",                  // no integer part
        "-0.01",               // negative
    ];

    #[test]
    fn valid_amounts_pass() {
        let rule = AmountFormatRule;
        for amount in VALID_AMOUNTS {
            let errors = rule.validate(amount, "/test");
            assert!(
                errors.is_empty(),
                "Expected no errors for valid amount `{amount}`, got: {errors:?}"
            );
        }
    }

    #[test]
    fn invalid_amounts_fail() {
        let rule = AmountFormatRule;
        for amount in INVALID_AMOUNTS {
            let errors = rule.validate(amount, "/test");
            assert!(
                !errors.is_empty(),
                "Expected errors for invalid amount `{amount}`"
            );
        }
    }

    #[test]
    fn error_has_correct_rule_id_and_path() {
        let rule = AmountFormatRule;
        let errors = rule.validate("-1.00", "/Document/Amt");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "AMOUNT_FORMAT");
        assert_eq!(errors[0].path, "/Document/Amt");
        assert_eq!(errors[0].severity, Severity::Error);
    }

    #[test]
    fn rule_id_is_amount_format() {
        assert_eq!(AmountFormatRule.id(), "AMOUNT_FORMAT");
    }

    #[test]
    fn zero_is_rejected() {
        let rule = AmountFormatRule;
        let errors = rule.validate("0", "/test");
        assert!(!errors.is_empty());
        assert!(
            errors[0].message.contains("greater than zero") || errors[0].message.contains("> 0"),
            "Expected zero message, got: {}",
            errors[0].message
        );
    }

    #[test]
    fn negative_is_rejected() {
        let rule = AmountFormatRule;
        let errors = rule.validate("-100.00", "/test");
        assert!(!errors.is_empty());
        assert!(
            errors[0].message.contains("positive"),
            "Expected positive message, got: {}",
            errors[0].message
        );
    }

    #[test]
    fn too_many_fractional_digits_rejected() {
        let rule = AmountFormatRule;
        let errors = rule.validate("1.123456", "/test");
        assert!(!errors.is_empty());
        assert!(
            errors[0].message.contains("fractional"),
            "Expected fractional message, got: {}",
            errors[0].message
        );
    }

    #[test]
    fn too_many_integer_digits_rejected() {
        let rule = AmountFormatRule;
        let errors = rule.validate("1234567890123456789", "/test"); // 19 digits
        assert!(!errors.is_empty());
        assert!(
            errors[0].message.contains("integer"),
            "Expected integer digits message, got: {}",
            errors[0].message
        );
    }

    #[test]
    fn non_numeric_rejected() {
        let rule = AmountFormatRule;
        let errors = rule.validate("abc", "/test");
        assert!(!errors.is_empty());
    }

    #[test]
    fn empty_rejected() {
        let rule = AmountFormatRule;
        let errors = rule.validate("", "/test");
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("empty"));
    }

    #[test]
    fn exactly_18_integer_digits_passes() {
        let rule = AmountFormatRule;
        let errors = rule.validate("999999999999999999", "/test"); // 18 digits
        assert!(errors.is_empty(), "18 integer digits should be valid");
    }

    #[test]
    fn exactly_5_fractional_digits_passes() {
        let rule = AmountFormatRule;
        let errors = rule.validate("1.12345", "/test"); // 5 fractional digits
        assert!(errors.is_empty(), "5 fractional digits should be valid");
    }
}
