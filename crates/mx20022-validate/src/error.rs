//! Validation error types for ISO 20022 message validation.

/// A single validation finding at a specific location in a message.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// XPath-like location in the message (e.g. `/Document/FIToFICstmrCdtTrf/GrpHdr/MsgId`).
    pub path: String,
    /// Severity of the finding.
    pub severity: Severity,
    /// Rule identifier (e.g. `IBAN_CHECK`, `MAX_LENGTH_35`).
    pub rule_id: String,
    /// Human-readable description.
    pub message: String,
}

impl ValidationError {
    /// Create a new [`ValidationError`].
    ///
    /// # Examples
    ///
    /// ```
    /// use mx20022_validate::error::{ValidationError, Severity};
    ///
    /// let err = ValidationError::new("/Document/MsgId", Severity::Error, "MAX_LENGTH_35", "Value exceeds maximum length of 35");
    /// assert_eq!(err.severity, Severity::Error);
    /// ```
    pub fn new(
        path: impl Into<String>,
        severity: Severity,
        rule_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            severity,
            rule_id: rule_id.into(),
            message: message.into(),
        }
    }
}

/// Severity level of a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// A hard error — the message is not valid.
    Error,
    /// A warning — the message may be valid but has a notable issue.
    Warning,
    /// Informational — for diagnostics only.
    Info,
}

/// The aggregated result of validating an ISO 20022 message.
///
/// # Examples
///
/// ```
/// use mx20022_validate::error::{ValidationResult, ValidationError, Severity};
///
/// let result = ValidationResult::new(vec![]);
/// assert!(result.is_valid());
/// assert_eq!(result.error_count(), 0);
/// assert_eq!(result.warning_count(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// All findings produced during validation.
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    /// Create a new `ValidationResult` from a list of findings.
    pub fn new(errors: Vec<ValidationError>) -> Self {
        Self { errors }
    }

    /// Returns `true` if there are no findings with [`Severity::Error`].
    pub fn is_valid(&self) -> bool {
        !self.errors.iter().any(|e| e.severity == Severity::Error)
    }

    /// Count of findings with [`Severity::Error`].
    pub fn error_count(&self) -> usize {
        self.errors
            .iter()
            .filter(|e| e.severity == Severity::Error)
            .count()
    }

    /// Count of findings with [`Severity::Warning`].
    pub fn warning_count(&self) -> usize {
        self.errors
            .iter()
            .filter(|e| e.severity == Severity::Warning)
            .count()
    }

    /// Merge another `ValidationResult` into this one.
    pub fn merge(&mut self, other: ValidationResult) {
        self.errors.extend(other.errors);
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_with_no_errors() {
        let result = ValidationResult::new(vec![]);
        assert!(result.is_valid());
    }

    #[test]
    fn is_invalid_with_error() {
        let result = ValidationResult::new(vec![ValidationError::new(
            "/path",
            Severity::Error,
            "RULE_1",
            "Some error",
        )]);
        assert!(!result.is_valid());
    }

    #[test]
    fn is_valid_with_only_warning() {
        let result = ValidationResult::new(vec![ValidationError::new(
            "/path",
            Severity::Warning,
            "RULE_1",
            "Some warning",
        )]);
        assert!(result.is_valid());
        assert_eq!(result.warning_count(), 1);
        assert_eq!(result.error_count(), 0);
    }

    #[test]
    fn counts_are_correct() {
        let result = ValidationResult::new(vec![
            ValidationError::new("/a", Severity::Error, "R1", "e1"),
            ValidationError::new("/b", Severity::Error, "R2", "e2"),
            ValidationError::new("/c", Severity::Warning, "R3", "w1"),
            ValidationError::new("/d", Severity::Info, "R4", "i1"),
        ]);
        assert_eq!(result.error_count(), 2);
        assert_eq!(result.warning_count(), 1);
        assert!(!result.is_valid());
    }

    #[test]
    fn merge_combines_findings() {
        let mut a = ValidationResult::new(vec![ValidationError::new(
            "/a",
            Severity::Error,
            "R1",
            "e1",
        )]);
        let b = ValidationResult::new(vec![ValidationError::new(
            "/b",
            Severity::Warning,
            "R2",
            "w1",
        )]);
        a.merge(b);
        assert_eq!(a.errors.len(), 2);
        assert_eq!(a.error_count(), 1);
        assert_eq!(a.warning_count(), 1);
    }
}
