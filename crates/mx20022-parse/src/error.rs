//! Error types for XML parsing and serialization.

use thiserror::Error;

/// Errors that can occur during ISO 20022 XML parsing or serialization.
#[derive(Debug, Error)]
pub enum ParseError {
    /// XML deserialization failed.
    #[error("XML deserialization error: {0}")]
    Deserialize(#[from] quick_xml::DeError),

    /// XML serialization failed.
    #[error("XML serialization error: {0}")]
    Serialize(#[from] quick_xml::SeError),

    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The XML document does not have a valid ISO 20022 envelope.
    #[error("invalid envelope: {0}")]
    InvalidEnvelope(String),
}
