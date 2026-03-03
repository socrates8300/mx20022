//! Error types for SWIFT MT message parsing.

use thiserror::Error;

/// Errors that can occur when parsing SWIFT MT messages.
#[derive(Debug, Error)]
pub enum MtError {
    /// The top-level block structure is malformed (e.g., unmatched braces).
    #[error("invalid block structure: {0}")]
    InvalidBlockStructure(String),

    /// The content of a specific block cannot be parsed.
    #[error("invalid block {block} content: {detail}")]
    InvalidBlockContent { block: u8, detail: String },

    /// A required block is absent from the message.
    #[error("missing required block: {0}")]
    MissingBlock(u8),

    /// A field tag does not match the expected format.
    #[error("invalid field tag: {0}")]
    InvalidFieldTag(String),

    /// A required field is missing from the message body.
    #[error("missing required field: {tag} in MT{message_type}")]
    MissingField { tag: String, message_type: String },

    /// A field's value cannot be interpreted according to the SWIFT spec.
    #[error("invalid field value for {tag}: {detail}")]
    InvalidFieldValue { tag: String, detail: String },
}
