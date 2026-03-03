//! Error and result types for MT↔MX translation.

use thiserror::Error;

/// Errors that can occur during translation between MT and MX messages.
#[derive(Debug, Error)]
pub enum TranslationError {
    /// The message type is not supported by this translation layer.
    #[error("unsupported message type: {0}")]
    UnsupportedMessageType(String),

    /// A field required for translation is absent from the source message.
    #[error("missing required field: {field} for {context}")]
    MissingField { field: String, context: String },

    /// A field's value cannot be mapped to the target format.
    #[error("invalid field value: {field}: {detail}")]
    InvalidFieldValue { field: String, detail: String },

    /// The source MT text could not be parsed.
    #[error("MT parse error: {0}")]
    MtParse(#[from] crate::mt::MtError),

    /// The source MX XML could not be parsed.
    #[error("MX parse error: {0}")]
    MxParse(#[from] mx20022_parse::ParseError),

    /// A model builder rejected its inputs.
    #[error("builder error: {0}")]
    Builder(#[from] mx20022_model::common::BuilderError),
}

// ---------------------------------------------------------------------------
// TranslationWarning
// ---------------------------------------------------------------------------

/// A single field-level data-loss warning produced during translation.
///
/// Warnings indicate that a field was truncated, approximated, or could not
/// be fully represented in the target format but did not prevent translation
/// from completing.
#[derive(Debug, Clone, PartialEq)]
pub struct TranslationWarning {
    /// The source or target field name, e.g. `":72:"` or `"CdtTrfTxInf.RmtInf"`.
    pub field: String,
    /// A human-readable description of what was lost or approximated.
    pub message: String,
}

/// A collection of [`TranslationWarning`]s accumulated during a single
/// translation call.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TranslationWarnings {
    /// The warnings in the order they were recorded.
    pub warnings: Vec<TranslationWarning>,
}

impl TranslationWarnings {
    /// Append a new warning.
    pub fn add(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.warnings.push(TranslationWarning {
            field: field.into(),
            message: message.into(),
        });
    }

    /// Returns `true` when no warnings have been recorded.
    pub fn is_empty(&self) -> bool {
        self.warnings.is_empty()
    }
}

// ---------------------------------------------------------------------------
// TranslationResult
// ---------------------------------------------------------------------------

/// The outcome of a successful translation.
///
/// `T` is the translated message type (`Document`, `String`, …).
/// `warnings` contains any data-loss notes that did not prevent the
/// translation from completing.
#[derive(Debug)]
pub struct TranslationResult<T: std::fmt::Debug> {
    /// The translated message.
    pub message: T,
    /// Any warnings about data that was truncated, approximated, or lost.
    pub warnings: TranslationWarnings,
}
