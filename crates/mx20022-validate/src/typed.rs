//! Typed validation bridge.
//!
//! Converts [`ConstraintViolation`]s from the model crate's [`Validatable`]
//! trait into the validate crate's [`ValidationResult`].
//!
//! # Example
//!
//! ```no_run
//! use mx20022_validate::typed::validate_constraints;
//!
//! // Given a type that implements Validatable:
//! // let result = validate_constraints(&my_message, "/Document");
//! // assert!(result.is_valid());
//! ```

use mx20022_model::common::validate::{
    ConstraintKind, ConstraintViolation, IsoMessage, Validatable,
};

use crate::error::{Severity, ValidationError, ValidationResult};

/// Validate XSD constraints on a typed value, producing a [`ValidationResult`].
///
/// Calls [`Validatable::validate_constraints`] on `msg` and maps each
/// [`ConstraintViolation`] to a [`ValidationError`] with `Severity::Error`.
pub fn validate_constraints<T: Validatable>(msg: &T, base_path: &str) -> ValidationResult {
    let mut violations = Vec::new();
    msg.validate_constraints(base_path, &mut violations);
    ValidationResult::new(violations.into_iter().map(violation_to_error).collect())
}

/// Validate a complete ISO 20022 message using its own root path.
///
/// Convenience wrapper that extracts the root path from [`IsoMessage`] and
/// calls [`validate_constraints`].
pub fn validate_message<T: IsoMessage>(msg: &T) -> ValidationResult {
    validate_constraints(msg, msg.root_path())
}

/// Map a [`ConstraintViolation`] to a [`ValidationError`].
fn violation_to_error(v: ConstraintViolation) -> ValidationError {
    ValidationError::new(
        v.path,
        Severity::Error,
        constraint_rule_id(v.kind),
        v.message,
    )
}

/// Map a [`ConstraintKind`] to a stable rule ID string.
fn constraint_rule_id(kind: ConstraintKind) -> &'static str {
    match kind {
        ConstraintKind::MinLength => "XSD_MIN_LENGTH",
        ConstraintKind::MaxLength => "XSD_MAX_LENGTH",
        ConstraintKind::Pattern => "XSD_PATTERN",
        ConstraintKind::MinInclusive => "XSD_MIN_INCLUSIVE",
        ConstraintKind::MaxInclusive => "XSD_MAX_INCLUSIVE",
        ConstraintKind::TotalDigits => "XSD_TOTAL_DIGITS",
        ConstraintKind::FractionDigits => "XSD_FRACTION_DIGITS",
    }
}
