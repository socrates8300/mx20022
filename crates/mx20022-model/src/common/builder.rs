//! Builder error type for generated builder pattern implementations.

/// Error returned when a builder cannot construct a valid type because one or
/// more required fields were not set.
///
/// # Example
///
/// ```
/// use mx20022_model::common::BuilderError;
///
/// let err = BuilderError {
///     type_name: "BusinessApplicationHeaderV04".to_owned(),
///     missing_fields: vec!["fr".to_owned(), "biz_msg_idr".to_owned()],
/// };
/// assert!(err.to_string().contains("BusinessApplicationHeaderV04"));
/// assert!(err.to_string().contains("fr"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuilderError {
    /// The type being built (e.g. `"BusinessApplicationHeaderV04"`).
    pub type_name: String,
    /// Names of required fields that were not set before calling `.build()`.
    pub missing_fields: Vec<String>,
}

impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cannot build `{}`: missing required fields: [{}]",
            self.type_name,
            self.missing_fields.join(", ")
        )
    }
}

impl std::error::Error for BuilderError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_single_field() {
        let err = BuilderError {
            type_name: "Foo".to_owned(),
            missing_fields: vec!["bar".to_owned()],
        };
        let s = err.to_string();
        assert!(s.contains("Foo"), "type name in message: {s}");
        assert!(s.contains("bar"), "field name in message: {s}");
    }

    #[test]
    fn display_multiple_fields() {
        let err = BuilderError {
            type_name: "Foo".to_owned(),
            missing_fields: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
        };
        let s = err.to_string();
        assert!(s.contains("a"), "{s}");
        assert!(s.contains("b"), "{s}");
        assert!(s.contains("c"), "{s}");
    }

    #[test]
    fn error_trait_implemented() {
        let err = BuilderError {
            type_name: "T".to_owned(),
            missing_fields: vec![],
        };
        // std::error::Error is object-safe and the impl compiles
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn clone_and_eq() {
        let err = BuilderError {
            type_name: "T".to_owned(),
            missing_fields: vec!["x".to_owned()],
        };
        assert_eq!(err.clone(), err);
    }
}
