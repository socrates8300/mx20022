//! Payment scheme-specific validation.
//!
//! Each payment scheme (`FedNow`, SEPA, `CBPR+`) has additional rules beyond the
//! base ISO 20022 schema. This module provides validators that enforce these
//! scheme-specific constraints.
//!
//! # Design
//!
//! Scheme validators operate on **raw XML strings** rather than typed model
//! structs, keeping the `mx20022-validate` crate decoupled from any particular
//! message version. Field extraction is done with lightweight string scanning
//! (see [`xml_scan`]), not full XML parsing.
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

use crate::error::ValidationResult;

/// A scheme-specific validator applied to raw XML message content.
///
/// Implement this trait to add support for new payment schemes or to create
/// custom scheme variants.
///
/// # Contract
///
/// - `validate` **must** return an empty `ValidationResult` (no errors, no
///   warnings) for message types not listed in `supported_messages`.
/// - `validate` should not panic; callers provide raw XML that may be
///   malformed.
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
    /// `message_type` is the full ISO 20022 message type detected from the
    /// XML namespace (e.g. `"pacs.008.001.13"`).  The validator is responsible
    /// for deriving the short type and returning early for unsupported types.
    fn validate(&self, xml: &str, message_type: &str) -> ValidationResult;
}
