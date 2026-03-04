//! Payment scheme-specific validation.
//!
//! Each payment scheme (`FedNow`, SEPA, `CBPR+`) has additional rules beyond the
//! base ISO 20022 schema. This module provides validators that enforce these
//! scheme-specific constraints.
//!
//! # Design
//!
//! Scheme validators support two validation paths:
//!
//! 1. **XML-based** ([`SchemeValidator::validate`]) — operates on raw XML
//!    strings using lightweight string scanning ([`xml_scan`]).
//! 2. **Typed** ([`SchemeValidator::validate_typed`]) — operates on
//!    deserialized message structs via `std::any::Any` downcasting.
//!
//! The typed path is preferred when the caller has already deserialized the
//! message. It avoids fragile XML string scanning and catches field-level
//! issues at compile time (within the validator implementation).
//!
//! # Error Paths
//!
//! Error paths in [`ValidationError`](crate::error::ValidationError) follow
//! XPath-like conventions:
//!
//! | Style | Example | When |
//! |---|---|---|
//! | Absolute | `/Document/FIToFICstmrCdtTrf/GrpHdr/MsgId` | Typed path (field known) |
//! | Abbreviated | `//BICFI` | XML scan (element found anywhere) |
//! | Root element | `/AppHdr` | Envelope-level checks |
//!
//! **New validators should prefer absolute paths** when the field location is
//! known (always the case for typed validators). Abbreviated `//Element`
//! paths are acceptable for XML-scan checks that match elements regardless
//! of position.
//!
//! # Usage
//!
//! ```rust
//! use mx20022_validate::schemes::fednow::FedNowValidator;
//! use mx20022_validate::schemes::SchemeValidator;
//!
//! let validator = FedNowValidator::new();
//! let xml = r#"<?xml version="1.0"?><Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13"></Document>"#;
//! let result = validator.validate(xml, "pacs.008.001.13");
//! // Result may contain errors for missing mandatory fields.
//! println!("{} error(s)", result.error_count());
//! ```

pub mod cbpr;
pub mod fednow;
pub mod sepa;
pub mod xml_scan;

use std::any::Any;

use crate::error::ValidationResult;

/// A scheme-specific validator for ISO 20022 payment messages.
///
/// Provides two validation paths: XML-based ([`validate`](SchemeValidator::validate))
/// and typed ([`validate_typed`](SchemeValidator::validate_typed)).
///
/// # Contract
///
/// - `validate` **must** return an empty [`ValidationResult`] (no errors,
///   no warnings) for message types not listed in
///   [`supported_messages`](SchemeValidator::supported_messages).
/// - `validate_typed` returns `None` for unsupported message types or
///   failed downcasts, and `Some(result)` for actual validation.
/// - Neither method should panic; callers may provide malformed XML or
///   unrecognised types.
/// - Implementations should be `Send + Sync` so they can be stored in
///   `Arc<dyn SchemeValidator>`.
pub trait SchemeValidator: Send + Sync {
    /// Human-readable name of the scheme (e.g. `"FedNow"`, `"SEPA"`, `"CBPR+"`).
    fn name(&self) -> &'static str;

    /// Short message type identifiers supported by this scheme.
    ///
    /// Each entry is a two-segment dot-separated identifier such as
    /// `"pacs.008"` or `"camt.056"`.  The validator should ignore messages
    /// whose type does not appear in this list.
    fn supported_messages(&self) -> &[&str];

    /// Validate raw XML content against this scheme's rules.
    ///
    /// # Migration
    ///
    /// This method operates on raw XML strings using fragile string scanning.
    /// **New callers should use [`validate_typed`](SchemeValidator::validate_typed)**
    /// which operates on deserialized message structs and catches field-level
    /// issues at compile time.
    ///
    /// The XML-based path remains available for cases where raw XML is the
    /// only input (e.g. CLI validation without prior deserialization), or for
    /// checks that inherently require raw XML (message size, `AppHdr` envelope,
    /// control characters).
    ///
    /// `message_type` is the full ISO 20022 message type detected from the
    /// XML namespace (e.g. `"pacs.008.001.13"`).  The validator is responsible
    /// for deriving the short type and returning early for unsupported types.
    fn validate(&self, xml: &str, message_type: &str) -> ValidationResult;

    /// Validate a typed (deserialized) message against this scheme's rules.
    ///
    /// `msg` is a reference to the deserialized message struct (e.g.
    /// `pacs_008_001_13::Document`). Implementations downcast via
    /// `Any::downcast_ref` to the concrete types they support.
    ///
    /// `message_type` is the full ISO 20022 message type (e.g.
    /// `"pacs.008.001.13"`), used to route to the appropriate validation logic.
    ///
    /// Returns `Some(result)` when the validator supports the given message
    /// type and the downcast succeeds. Returns `None` for unsupported message
    /// types or failed downcasts, allowing callers to distinguish "valid with
    /// no errors" from "not applicable".
    fn validate_typed(&self, msg: &dyn Any, message_type: &str) -> Option<ValidationResult> {
        let _ = (msg, message_type);
        None
    }
}

/// Extract the short message type (e.g. `"pacs.008"`) from a full type
/// string like `"pacs.008.001.13"`.
pub fn short_message_type(message_type: &str) -> String {
    message_type
        .splitn(3, '.')
        .take(2)
        .collect::<Vec<_>>()
        .join(".")
}
