//! Schema-level validation.
//!
//! This module provides the thin orchestration layer that drives field-level
//! constraint checking through the [`RuleRegistry`]. The actual rule logic lives
//! in [`crate::rules`]; the constraint definitions for a particular message schema
//! are expressed as [`constraints::ConstraintSet`] instances.
//!
//! # Design
//!
//! ```text
//! ConstraintSet ──► RuleRegistry ──► Rule::validate()
//!                                         │
//!                                         ▼
//!                                  Vec<ValidationError>
//! ```
//!
//! [`SchemaValidator`] wraps a [`ConstraintSet`] + [`RuleRegistry`] pair and
//! exposes a single [`SchemaValidator::validate_field`] entry-point.

pub mod constraints;

use crate::error::ValidationResult;
use crate::rules::RuleRegistry;
use constraints::ConstraintSet;

/// Orchestrates schema-level validation for a message type.
///
/// # Examples
///
/// ```
/// use mx20022_validate::schema::{SchemaValidator};
/// use mx20022_validate::schema::constraints::{ConstraintSet, FieldConstraint};
/// use mx20022_validate::rules::RuleRegistry;
///
/// let mut cs = ConstraintSet::new();
/// cs.add(FieldConstraint::new("/Document/GrpHdr/MsgId", ["MAX_LENGTH"]));
///
/// let validator = SchemaValidator::new(cs, RuleRegistry::with_defaults());
/// let result = validator.validate_field("/Document/GrpHdr/MsgId", "ABC123");
/// assert!(result.is_valid());
/// ```
pub struct SchemaValidator {
    constraints: ConstraintSet,
    registry: RuleRegistry,
}

impl SchemaValidator {
    /// Create a new `SchemaValidator` from a constraint set and rule registry.
    pub fn new(constraints: ConstraintSet, registry: RuleRegistry) -> Self {
        Self {
            constraints,
            registry,
        }
    }

    /// Create a `SchemaValidator` with an empty constraint set and the default rule registry.
    pub fn with_defaults() -> Self {
        Self::new(ConstraintSet::new(), RuleRegistry::with_defaults())
    }

    /// Validate a single field value at the given path.
    ///
    /// Returns a [`ValidationResult`] with any findings from constraints registered
    /// for this path. If no constraints are registered for the path the result is
    /// unconditionally valid.
    pub fn validate_field(&self, path: &str, value: &str) -> ValidationResult {
        self.constraints.validate_field(path, value, &self.registry)
    }

    /// Access the underlying registry (e.g. to register additional rules).
    pub fn registry_mut(&mut self) -> &mut RuleRegistry {
        &mut self.registry
    }

    /// Access the underlying constraint set (e.g. to add constraints at runtime).
    pub fn constraints_mut(&mut self) -> &mut ConstraintSet {
        &mut self.constraints
    }

    /// Validate multiple `(path, value)` pairs and merge all findings.
    pub fn validate_fields<'a>(
        &self,
        fields: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> ValidationResult {
        let mut result = ValidationResult::default();
        for (path, value) in fields {
            result.merge(self.validate_field(path, value));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use constraints::FieldConstraint;

    #[test]
    fn with_defaults_produces_valid_for_unconstrained_path() {
        let validator = SchemaValidator::with_defaults();
        let result = validator.validate_field("/any/path", "any value");
        assert!(result.is_valid());
    }

    #[test]
    fn validate_field_iban() {
        let mut cs = ConstraintSet::new();
        cs.add(FieldConstraint::new("/iban", ["IBAN_CHECK"]));
        let validator = SchemaValidator::new(cs, RuleRegistry::with_defaults());

        assert!(validator
            .validate_field("/iban", "GB82WEST12345698765432")
            .is_valid());
        assert!(!validator.validate_field("/iban", "NOTANIBAN").is_valid());
    }

    #[test]
    fn validate_fields_merges_results() {
        let mut cs = ConstraintSet::new();
        cs.add(FieldConstraint::new("/iban", ["IBAN_CHECK"]));
        cs.add(FieldConstraint::new("/bic", ["BIC_CHECK"]));
        let validator = SchemaValidator::new(cs, RuleRegistry::with_defaults());

        let result = validator.validate_fields([("/iban", "NOTANIBAN"), ("/bic", "NOTABIC")]);
        // Both fields should produce errors
        assert!(!result.is_valid());
        assert!(result.error_count() >= 2);
    }

    #[test]
    fn validate_fields_all_valid() {
        let mut cs = ConstraintSet::new();
        cs.add(FieldConstraint::new("/iban", ["IBAN_CHECK"]));
        cs.add(FieldConstraint::new("/bic", ["BIC_CHECK"]));
        let validator = SchemaValidator::new(cs, RuleRegistry::with_defaults());

        let result =
            validator.validate_fields([("/iban", "GB82WEST12345698765432"), ("/bic", "AAAAGB2L")]);
        assert!(result.is_valid());
    }
}
