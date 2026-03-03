//! String length validation rules derived from XSD `minLength` / `maxLength` facets.

use crate::error::{Severity, ValidationError};
use crate::rules::Rule;

/// Validates that a string meets a minimum length requirement.
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::length::MinLengthRule;
/// use mx20022_validate::rules::Rule;
///
/// let rule = MinLengthRule::new(1);
/// assert!(rule.validate("a", "/path").is_empty());
/// assert!(!rule.validate("", "/path").is_empty());
/// ```
pub struct MinLengthRule {
    min: usize,
}

impl MinLengthRule {
    /// Create a new minimum-length rule.
    pub fn new(min: usize) -> Self {
        Self { min }
    }
}

impl Rule for MinLengthRule {
    fn id(&self) -> &'static str {
        "MIN_LENGTH"
    }

    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
        // XSD minLength operates on the number of Unicode code points (characters), not bytes.
        let len = value.chars().count();
        if len < self.min {
            vec![ValidationError::new(
                path,
                Severity::Error,
                "MIN_LENGTH",
                format!(
                    "Value length {len} is less than minimum length {}",
                    self.min
                ),
            )]
        } else {
            vec![]
        }
    }
}

/// Validates that a string does not exceed a maximum length.
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::length::MaxLengthRule;
/// use mx20022_validate::rules::Rule;
///
/// let rule = MaxLengthRule::new(35);
/// assert!(rule.validate("short text", "/path").is_empty());
/// assert!(!rule.validate(&"x".repeat(36), "/path").is_empty());
/// ```
pub struct MaxLengthRule {
    max: usize,
}

impl MaxLengthRule {
    /// Create a new maximum-length rule.
    pub fn new(max: usize) -> Self {
        Self { max }
    }
}

impl Rule for MaxLengthRule {
    fn id(&self) -> &'static str {
        "MAX_LENGTH"
    }

    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
        let len = value.chars().count();
        if len > self.max {
            vec![ValidationError::new(
                path,
                Severity::Error,
                "MAX_LENGTH",
                format!("Value length {len} exceeds maximum length {}", self.max),
            )]
        } else {
            vec![]
        }
    }
}

/// Validates that a string's length is within an inclusive range `[min, max]`.
///
/// This is a convenience wrapper that combines [`MinLengthRule`] and [`MaxLengthRule`].
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::length::LengthRangeRule;
/// use mx20022_validate::rules::Rule;
///
/// let rule = LengthRangeRule::new(1, 35);
/// assert!(rule.validate("hello", "/path").is_empty());
/// assert!(!rule.validate("", "/path").is_empty());
/// assert!(!rule.validate(&"x".repeat(36), "/path").is_empty());
/// ```
pub struct LengthRangeRule {
    min: usize,
    max: usize,
}

impl LengthRangeRule {
    /// Create a new length range rule.
    pub fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }
}

impl Rule for LengthRangeRule {
    fn id(&self) -> &'static str {
        "LENGTH_RANGE"
    }

    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
        let len = value.chars().count();
        let mut errors = vec![];
        if len < self.min {
            errors.push(ValidationError::new(
                path,
                Severity::Error,
                "MIN_LENGTH",
                format!(
                    "Value length {len} is less than minimum length {}",
                    self.min
                ),
            ));
        }
        if len > self.max {
            errors.push(ValidationError::new(
                path,
                Severity::Error,
                "MAX_LENGTH",
                format!("Value length {len} exceeds maximum length {}", self.max),
            ));
        }
        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Rule;

    // --- MinLengthRule ---

    #[test]
    fn min_length_passes_when_equal() {
        let rule = MinLengthRule::new(3);
        assert!(rule.validate("abc", "/p").is_empty());
    }

    #[test]
    fn min_length_passes_when_longer() {
        let rule = MinLengthRule::new(3);
        assert!(rule.validate("abcdef", "/p").is_empty());
    }

    #[test]
    fn min_length_fails_when_shorter() {
        let rule = MinLengthRule::new(3);
        let errors = rule.validate("ab", "/p");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "MIN_LENGTH");
    }

    #[test]
    fn min_length_zero_always_passes() {
        let rule = MinLengthRule::new(0);
        assert!(rule.validate("", "/p").is_empty());
    }

    // --- MaxLengthRule ---

    #[test]
    fn max_length_passes_when_equal() {
        let rule = MaxLengthRule::new(35);
        assert!(rule.validate(&"x".repeat(35), "/p").is_empty());
    }

    #[test]
    fn max_length_passes_when_shorter() {
        let rule = MaxLengthRule::new(35);
        assert!(rule.validate("short", "/p").is_empty());
    }

    #[test]
    fn max_length_fails_when_longer() {
        let rule = MaxLengthRule::new(35);
        let errors = rule.validate(&"x".repeat(36), "/p");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "MAX_LENGTH");
    }

    #[test]
    fn max_length_empty_string_passes_when_max_positive() {
        let rule = MaxLengthRule::new(5);
        assert!(rule.validate("", "/p").is_empty());
    }

    // --- LengthRangeRule ---

    #[test]
    fn range_passes_within_bounds() {
        let rule = LengthRangeRule::new(1, 35);
        assert!(rule.validate("hello world", "/p").is_empty());
    }

    #[test]
    fn range_fails_below_min() {
        let rule = LengthRangeRule::new(1, 35);
        let errors = rule.validate("", "/p");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "MIN_LENGTH");
    }

    #[test]
    fn range_fails_above_max() {
        let rule = LengthRangeRule::new(1, 5);
        let errors = rule.validate("toolong", "/p");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "MAX_LENGTH");
    }

    #[test]
    fn length_counts_unicode_code_points_not_bytes() {
        // "é" is 2 UTF-8 bytes but 1 code point; max=1 should pass
        let rule = MaxLengthRule::new(1);
        assert!(rule.validate("é", "/p").is_empty());
    }
}
