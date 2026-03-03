//! Validation rule registry and built-in rule implementations.
//!
//! # Overview
//!
//! Rules are stateless validators that inspect a string value at a named path and
//! produce zero or more [`ValidationError`]s. They are registered in a
//! [`RuleRegistry`] and looked up by ID when validating fields.
//!
//! # Built-in rules
//!
//! | Rule ID           | Module          | Description                              |
//! |-------------------|-----------------|------------------------------------------|
//! | `IBAN_CHECK`      | [`iban`]        | ISO 13616 IBAN format + mod-97 check     |
//! | `BIC_CHECK`       | [`bic`]         | ISO 9362 BIC/SWIFT code format           |
//! | `CURRENCY_CHECK`  | [`currency`]    | ISO 4217 currency code                   |
//! | `COUNTRY_CHECK`   | [`country`]     | ISO 3166-1 alpha-2 country code          |
//! | `LEI_CHECK`       | [`lei`]         | ISO 17442 LEI format + mod-97 check      |
//! | `AMOUNT_FORMAT`   | [`amount`]      | ISO 20022 decimal amount format          |
//! | `DATETIME_CHECK`  | [`datetime`]    | ISO 8601 datetime (ISO 20022 subset)     |
//! | `DATE_CHECK`      | [`datetime`]    | ISO 8601 date (ISO 20022 subset)         |
//! | `MIN_LENGTH`      | [`length`]      | Minimum string length (XSD `minLength`)  |
//! | `MAX_LENGTH`      | [`length`]      | Maximum string length (XSD `maxLength`)  |
//! | `LENGTH_RANGE`    | [`length`]      | Combined min/max range                   |
//! | `*` (custom)      | [`pattern`]     | Regex pattern (XSD `pattern` facet)      |

pub mod amount;
pub mod bic;
pub mod country;
pub mod currency;
pub mod datetime;
pub mod iban;
pub mod lei;
pub mod length;
pub mod pattern;

use crate::error::ValidationError;
use std::collections::HashMap;

/// A validation rule that can be applied to a string value at a given path.
///
/// Implement this trait to create custom validation rules.
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::Rule;
/// use mx20022_validate::error::{ValidationError, Severity};
///
/// struct NonEmptyRule;
///
/// impl Rule for NonEmptyRule {
///     fn id(&self) -> &'static str { "NON_EMPTY" }
///
///     fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
///         if value.is_empty() {
///             vec![ValidationError::new(path, Severity::Error, "NON_EMPTY", "Value must not be empty")]
///         } else {
///             vec![]
///         }
///     }
/// }
/// ```
pub trait Rule: Send + Sync {
    /// Unique identifier for this rule (e.g. `"IBAN_CHECK"`).
    fn id(&self) -> &str;

    /// Run the rule against `value` at the given `path` and return any findings.
    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError>;
}

/// A registry of named validation rules.
///
/// Rules are stored by their [`Rule::id`]. When multiple rules share an ID the
/// last one registered wins (use unique IDs unless intentional override is needed).
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::RuleRegistry;
///
/// let registry = RuleRegistry::with_defaults();
/// let errors = registry.validate_field("GB82WEST12345698765432", "/path/iban", &["IBAN_CHECK"]);
/// assert!(errors.is_empty());
/// ```
pub struct RuleRegistry {
    rules: HashMap<String, Box<dyn Rule>>,
}

impl RuleRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Create a registry pre-populated with all built-in rules.
    ///
    /// Built-in rules included:
    /// - `IBAN_CHECK`      — IBAN format + mod-97
    /// - `BIC_CHECK`       — BIC/SWIFT format
    /// - `CURRENCY_CHECK`  — ISO 4217 currency code
    /// - `COUNTRY_CHECK`   — ISO 3166-1 alpha-2 country code
    /// - `LEI_CHECK`       — ISO 17442 LEI format + mod-97
    /// - `AMOUNT_FORMAT`   — ISO 20022 decimal amount format
    /// - `DATETIME_CHECK`  — ISO 8601 datetime
    /// - `DATE_CHECK`      — ISO 8601 date
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(iban::IbanRule));
        registry.register(Box::new(bic::BicRule));
        registry.register(Box::new(currency::CurrencyRule));
        registry.register(Box::new(country::CountryCodeRule));
        registry.register(Box::new(lei::LeiRule));
        registry.register(Box::new(amount::AmountFormatRule));
        registry.register(Box::new(datetime::IsoDateTimeRule));
        registry.register(Box::new(datetime::IsoDateRule));
        registry
    }

    /// Register a rule. If a rule with the same ID already exists it is replaced.
    pub fn register(&mut self, rule: Box<dyn Rule>) {
        self.rules.insert(rule.id().to_owned(), rule);
    }

    /// Look up a registered rule by ID.
    pub fn get(&self, rule_id: &str) -> Option<&dyn Rule> {
        self.rules.get(rule_id).map(std::convert::AsRef::as_ref)
    }

    /// Run a specific subset of rules (identified by `rule_ids`) against `value`
    /// at `path` and return all findings.
    ///
    /// Rules whose IDs are not present in the registry are silently skipped.
    ///
    /// # Examples
    ///
    /// ```
    /// use mx20022_validate::rules::RuleRegistry;
    ///
    /// let registry = RuleRegistry::with_defaults();
    /// let errors = registry.validate_field("NOT_AN_IBAN", "/doc/iban", &["IBAN_CHECK"]);
    /// assert!(!errors.is_empty());
    /// ```
    pub fn validate_field(
        &self,
        value: &str,
        path: &str,
        rule_ids: &[&str],
    ) -> Vec<ValidationError> {
        rule_ids
            .iter()
            .filter_map(|id| self.rules.get(*id))
            .flat_map(|rule| rule.validate(value, path))
            .collect()
    }

    /// Run **all** registered rules against `value` at `path`.
    pub fn validate_all(&self, value: &str, path: &str) -> Vec<ValidationError> {
        self.rules
            .values()
            .flat_map(|rule| rule.validate(value, path))
            .collect()
    }
}

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Severity;

    struct AlwaysFailRule;
    impl Rule for AlwaysFailRule {
        fn id(&self) -> &'static str {
            "ALWAYS_FAIL"
        }
        fn validate(&self, _value: &str, path: &str) -> Vec<ValidationError> {
            vec![ValidationError::new(
                path,
                Severity::Error,
                "ALWAYS_FAIL",
                "always fails",
            )]
        }
    }

    #[test]
    fn empty_registry_produces_no_errors() {
        let registry = RuleRegistry::new();
        let errors = registry.validate_field("any", "/p", &["IBAN_CHECK"]);
        assert!(errors.is_empty());
    }

    #[test]
    fn registered_rule_is_invoked() {
        let mut registry = RuleRegistry::new();
        registry.register(Box::new(AlwaysFailRule));
        let errors = registry.validate_field("any", "/p", &["ALWAYS_FAIL"]);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn unknown_rule_id_is_skipped() {
        let registry = RuleRegistry::with_defaults();
        let errors = registry.validate_field("any", "/p", &["NO_SUCH_RULE"]);
        assert!(errors.is_empty());
    }

    #[test]
    fn with_defaults_includes_iban_check() {
        let registry = RuleRegistry::with_defaults();
        assert!(registry.get("IBAN_CHECK").is_some());
    }

    #[test]
    fn with_defaults_includes_bic_check() {
        let registry = RuleRegistry::with_defaults();
        assert!(registry.get("BIC_CHECK").is_some());
    }

    #[test]
    fn with_defaults_includes_all_new_rules() {
        let registry = RuleRegistry::with_defaults();
        assert!(registry.get("CURRENCY_CHECK").is_some());
        assert!(registry.get("COUNTRY_CHECK").is_some());
        assert!(registry.get("LEI_CHECK").is_some());
        assert!(registry.get("AMOUNT_FORMAT").is_some());
        assert!(registry.get("DATETIME_CHECK").is_some());
        assert!(registry.get("DATE_CHECK").is_some());
    }

    #[test]
    fn registering_replaces_existing_rule() {
        let mut registry = RuleRegistry::new();
        registry.register(Box::new(AlwaysFailRule));
        registry.register(Box::new(AlwaysFailRule)); // register again — should not panic
        let errors = registry.validate_field("any", "/p", &["ALWAYS_FAIL"]);
        // Should still be exactly 1 (last-write-wins, same rule)
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn validate_field_with_multiple_rules() {
        let mut registry = RuleRegistry::new();
        registry.register(Box::new(AlwaysFailRule));
        registry.register(Box::new(iban::IbanRule));
        // ALWAYS_FAIL will fire; IBAN_CHECK will also fire for "NOTANIBAN"
        let errors = registry.validate_field("NOTANIBAN", "/p", &["ALWAYS_FAIL", "IBAN_CHECK"]);
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn default_registry_is_with_defaults() {
        let registry = RuleRegistry::default();
        assert!(registry.get("IBAN_CHECK").is_some());
        assert!(registry.get("BIC_CHECK").is_some());
    }

    #[test]
    fn valid_iban_through_registry() {
        let registry = RuleRegistry::with_defaults();
        let errors = registry.validate_field("GB82WEST12345698765432", "/path", &["IBAN_CHECK"]);
        assert!(errors.is_empty());
    }

    #[test]
    fn valid_bic_through_registry() {
        let registry = RuleRegistry::with_defaults();
        let errors = registry.validate_field("AAAAGB2L", "/path", &["BIC_CHECK"]);
        assert!(errors.is_empty());
    }
}
