//! Regex pattern validation rule derived from XSD `pattern` facets.
//!
//! Uses the [`regex`] crate to compile and match patterns. Patterns are anchored
//! (full-string match) to match XSD semantics where the pattern must match the
//! entire value.

use crate::error::{Severity, ValidationError};
use crate::rules::Rule;
use regex::Regex;

/// Validates a string against a compiled regular expression (full-string match).
///
/// The pattern is anchored with `^...$` so that it must match the entire value,
/// mirroring XSD `<xs:pattern>` behaviour.
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::pattern::PatternRule;
/// use mx20022_validate::rules::Rule;
///
/// let rule = PatternRule::new("COUNTRY_CODE", "[A-Z]{2}").unwrap();
/// assert!(rule.validate("GB", "/path").is_empty());
/// assert!(!rule.validate("gb", "/path").is_empty());
/// assert!(!rule.validate("GBR", "/path").is_empty());
/// ```
pub struct PatternRule {
    rule_id: String,
    pattern: String,
    regex: Regex,
}

impl PatternRule {
    /// Build a new `PatternRule` with the given rule id and regex pattern.
    ///
    /// The pattern is automatically anchored — do **not** include `^` / `$` yourself.
    ///
    /// # Errors
    ///
    /// Returns an error if `pattern` is not a valid regular expression.
    pub fn new(rule_id: impl Into<String>, pattern: &str) -> Result<Self, regex::Error> {
        let rule_id = rule_id.into();
        // Anchor the pattern to enforce full-string matching (XSD semantics).
        let anchored = format!("^(?:{pattern})$");
        let regex = Regex::new(&anchored)?;
        Ok(Self {
            rule_id,
            pattern: pattern.to_owned(),
            regex,
        })
    }

    /// The raw (unanchored) pattern string used to construct this rule.
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

impl Rule for PatternRule {
    fn id(&self) -> &str {
        &self.rule_id
    }

    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
        if self.regex.is_match(value) {
            vec![]
        } else {
            vec![ValidationError::new(
                path,
                Severity::Error,
                &self.rule_id,
                format!("Value `{value}` does not match pattern `{}`", self.pattern),
            )]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Rule;

    #[test]
    fn two_letter_country_code_passes() {
        let rule = PatternRule::new("COUNTRY_CODE", "[A-Z]{2}").unwrap();
        assert!(rule.validate("GB", "/p").is_empty());
        assert!(rule.validate("US", "/p").is_empty());
    }

    #[test]
    fn two_letter_country_code_rejects_lowercase() {
        let rule = PatternRule::new("COUNTRY_CODE", "[A-Z]{2}").unwrap();
        let errors = rule.validate("gb", "/p");
        assert!(!errors.is_empty());
    }

    #[test]
    fn two_letter_country_code_rejects_extra_chars() {
        let rule = PatternRule::new("COUNTRY_CODE", "[A-Z]{2}").unwrap();
        let errors = rule.validate("GBR", "/p");
        assert!(!errors.is_empty());
    }

    #[test]
    fn bic_pattern_passes() {
        let rule = PatternRule::new(
            "BIC_PATTERN",
            "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}",
        )
        .unwrap();
        assert!(rule.validate("AAAAGB2L", "/p").is_empty());
        assert!(rule.validate("AAAAGB2LXXX", "/p").is_empty());
    }

    #[test]
    fn bic_pattern_rejects_short() {
        let rule = PatternRule::new(
            "BIC_PATTERN",
            "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}",
        )
        .unwrap();
        let errors = rule.validate("AAAA", "/p");
        assert!(!errors.is_empty());
    }

    #[test]
    fn error_contains_pattern_and_rule_id() {
        let rule = PatternRule::new("MY_RULE", "[A-Z]{3}").unwrap();
        let errors = rule.validate("abc", "/some/path");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "MY_RULE");
        assert_eq!(errors[0].path, "/some/path");
        assert!(errors[0].message.contains("[A-Z]{3}"));
    }

    #[test]
    fn invalid_regex_returns_error() {
        let result = PatternRule::new("BAD", "[unclosed");
        assert!(result.is_err());
    }

    #[test]
    fn empty_string_matches_empty_pattern() {
        let rule = PatternRule::new("EMPTY", "").unwrap();
        assert!(rule.validate("", "/p").is_empty());
        // A non-empty value should fail a pattern that only matches empty
        let errors = rule.validate("x", "/p");
        assert!(!errors.is_empty());
    }

    #[test]
    fn pattern_is_full_string_match_not_partial() {
        // Pattern [A-Z]{2} should NOT match "ABCD" even though "AB" is a substring
        let rule = PatternRule::new("R", "[A-Z]{2}").unwrap();
        let errors = rule.validate("ABCD", "/p");
        assert!(!errors.is_empty(), "Partial match should be rejected");
    }
}
