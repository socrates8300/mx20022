//! Schema-level constraint definitions.
//!
//! Each [`FieldConstraint`] describes what rules apply to a specific field path.
//! The [`ConstraintSet`] holds a collection of constraints and drives validation
//! through the [`RuleRegistry`].

use crate::error::{ValidationError, ValidationResult};
use crate::rules::RuleRegistry;

/// A constraint specification for a single field path.
///
/// A field constraint pairs an XPath-like field path with a list of rule IDs
/// from the [`RuleRegistry`] that must all pass for the field to be valid.
#[derive(Debug, Clone)]
pub struct FieldConstraint {
    /// XPath-like path identifying the field (e.g. `/Document/GrpHdr/MsgId`).
    pub path: String,
    /// Rule IDs to apply. Must be registered in the [`RuleRegistry`] at validation time.
    pub rule_ids: Vec<String>,
}

impl FieldConstraint {
    /// Create a new field constraint.
    pub fn new(
        path: impl Into<String>,
        rule_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            path: path.into(),
            rule_ids: rule_ids.into_iter().map(Into::into).collect(),
        }
    }
}

/// A set of field constraints for a message type.
///
/// Typically one `ConstraintSet` is constructed per message schema (e.g. pacs.008)
/// and reused across many validations.
///
/// # Examples
///
/// ```
/// use mx20022_validate::schema::constraints::{ConstraintSet, FieldConstraint};
/// use mx20022_validate::rules::RuleRegistry;
///
/// let mut cs = ConstraintSet::new();
/// cs.add(FieldConstraint::new(
///     "/Document/GrpHdr/MsgId",
///     ["MAX_LENGTH"],
/// ));
/// ```
#[derive(Debug, Default)]
pub struct ConstraintSet {
    constraints: Vec<FieldConstraint>,
}

impl ConstraintSet {
    /// Create an empty constraint set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a constraint to the set.
    pub fn add(&mut self, constraint: FieldConstraint) {
        self.constraints.push(constraint);
    }

    /// Look up all constraints for a given path.
    pub fn for_path(&self, path: &str) -> Vec<&FieldConstraint> {
        self.constraints.iter().filter(|c| c.path == path).collect()
    }

    /// Validate a `(path, value)` pair against all matching constraints using `registry`.
    ///
    /// Returns a [`ValidationResult`] aggregating any findings.
    pub fn validate_field(
        &self,
        path: &str,
        value: &str,
        registry: &RuleRegistry,
    ) -> ValidationResult {
        let rule_ids: Vec<&str> = self
            .for_path(path)
            .into_iter()
            .flat_map(|c| c.rule_ids.iter().map(String::as_str))
            .collect();

        if rule_ids.is_empty() {
            return ValidationResult::default();
        }

        let errors: Vec<ValidationError> = registry.validate_field(value, path, &rule_ids);
        ValidationResult::new(errors)
    }

    /// Returns the number of registered constraints.
    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    /// Returns `true` if the set contains no constraints.
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::RuleRegistry;

    #[test]
    fn empty_constraint_set_produces_no_errors() {
        let cs = ConstraintSet::new();
        let registry = RuleRegistry::with_defaults();
        let result = cs.validate_field("/some/path", "value", &registry);
        assert!(result.is_valid());
    }

    #[test]
    fn constraint_set_with_iban_validates() {
        let mut cs = ConstraintSet::new();
        cs.add(FieldConstraint::new("/iban", ["IBAN_CHECK"]));
        let registry = RuleRegistry::with_defaults();

        // Valid IBAN
        let ok = cs.validate_field("/iban", "GB82WEST12345698765432", &registry);
        assert!(ok.is_valid());

        // Invalid IBAN
        let fail = cs.validate_field("/iban", "NOTANIBAN", &registry);
        assert!(!fail.is_valid());
    }

    #[test]
    fn constraint_set_with_bic_validates() {
        let mut cs = ConstraintSet::new();
        cs.add(FieldConstraint::new("/bic", ["BIC_CHECK"]));
        let registry = RuleRegistry::with_defaults();

        let ok = cs.validate_field("/bic", "AAAAGB2L", &registry);
        assert!(ok.is_valid());

        let fail = cs.validate_field("/bic", "bad", &registry);
        assert!(!fail.is_valid());
    }

    #[test]
    fn unknown_rule_id_does_not_panic() {
        let mut cs = ConstraintSet::new();
        cs.add(FieldConstraint::new("/f", ["NO_SUCH_RULE"]));
        let registry = RuleRegistry::with_defaults();
        // Should silently skip unknown rules and produce no errors
        let result = cs.validate_field("/f", "anything", &registry);
        assert!(result.is_valid());
    }

    #[test]
    fn for_path_returns_correct_constraints() {
        let mut cs = ConstraintSet::new();
        cs.add(FieldConstraint::new("/a", ["IBAN_CHECK"]));
        cs.add(FieldConstraint::new("/b", ["BIC_CHECK"]));
        cs.add(FieldConstraint::new("/a", ["BIC_CHECK"]));

        let a_constraints = cs.for_path("/a");
        assert_eq!(a_constraints.len(), 2);

        let b_constraints = cs.for_path("/b");
        assert_eq!(b_constraints.len(), 1);

        let c_constraints = cs.for_path("/c");
        assert!(c_constraints.is_empty());
    }

    #[test]
    fn len_and_is_empty() {
        let mut cs = ConstraintSet::new();
        assert!(cs.is_empty());
        assert_eq!(cs.len(), 0);
        cs.add(FieldConstraint::new("/a", ["R1"]));
        assert!(!cs.is_empty());
        assert_eq!(cs.len(), 1);
    }
}
