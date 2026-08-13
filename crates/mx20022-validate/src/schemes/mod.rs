//! Payment scheme-specific validation.
//!
//! Each payment scheme (`FedNow`, SEPA, `CBPR+`) has additional rules beyond the
//! base ISO 20022 schema. This module provides validators that enforce these
//! scheme-specific constraints.
//!
//! # Design
//!
//! [`SchemeValidator::validate`] is a parse-and-delegate adapter. For
//! `pacs.008.001.13`, it extracts the `Document`, detects the authoritative
//! message namespace, deserializes the generated model, and delegates all
//! field rules to [`SchemeValidator::validate_typed`]. Older `pacs.008`
//! versions use the same version-neutral rule inputs and add an untyped-version
//! warning. Raw XML is retained only for envelope, byte-size, and
//! control-character checks.
//!
//! # Error Paths
//!
//! Error paths in [`ValidationError`](crate::error::ValidationError) follow
//! XPath-like conventions:
//!
//! | Style | Example | When |
//! |---|---|---|
//! | Absolute | `/Document/FIToFICstmrCdtTrf/GrpHdr/MsgId` | Typed path (field known) |
//! | Abbreviated | `//BICFI` | Rule applies to repeated typed fields |
//! | Root element | `/AppHdr` | Envelope-level checks |
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
pub(crate) mod common;
pub mod fednow;
mod pacs008;
mod raw;
pub mod sepa;

use std::{any::Any, borrow::Cow};

use mx20022_model::generated::pacs::pacs_008_001_13;
use mx20022_parse::envelope::parse_namespace;
use mx20022_parse::ParseError;
use quick_xml::events::Event;
use quick_xml::name::ResolveResult;
use quick_xml::reader::NsReader;

use crate::error::{Severity, ValidationError, ValidationResult};

/// A scheme-specific validator for ISO 20022 payment messages.
///
/// Provides a parse-and-delegate XML path
/// ([`validate`](SchemeValidator::validate)) and a typed path
/// ([`validate_typed`](SchemeValidator::validate_typed)).
///
/// # Contract
///
/// - `validate` **must** return an empty [`ValidationResult`] (no errors,
///   no warnings) for message types not listed in
///   [`supported_messages`](SchemeValidator::supported_messages).
/// - `validate_typed` returns `None` for unsupported message types or
///   failed downcasts, and `Some(result)` for actual validation.
/// - Malformed supported `pacs.008.001.13` XML produces one `SCHEME_PARSE`
///   finding. Independent raw findings for the caller's supported short type
///   are preserved; field findings are suppressed.
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
    /// Supported `pacs.008.001.13` messages are deserialized and delegated to
    /// [`validate_typed`](SchemeValidator::validate_typed). Other `pacs.008`
    /// versions receive raw findings, version-neutral field findings, and
    /// `SCHEME_UNTYPED_VERSION`.
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

/// Central parse-and-delegate adapter used by the built-in schemes.
fn run_xml<V, R, U>(
    validator: &V,
    xml: &str,
    caller_message_type: &str,
    raw_prepass: R,
    untyped_rules: U,
) -> ValidationResult
where
    V: SchemeValidator + ?Sized,
    R: FnOnce(&str, &str) -> ValidationResult,
    U: FnOnce(&pacs008::Facts) -> ValidationResult,
{
    let caller_short_type = short_message_type(caller_message_type);
    if !validator
        .supported_messages()
        .contains(&caller_short_type.as_str())
    {
        return ValidationResult::default();
    }

    if caller_short_type != "pacs.008" {
        return raw_prepass(xml, &caller_short_type);
    }

    let mut result = raw_prepass(xml, &caller_short_type);
    let document_xml = match mx20022_parse::de::document_xml(xml) {
        Ok(document) => document,
        Err(error) => return with_scheme_parse(result, &error),
    };
    let authoritative_type = match document_message_type(xml) {
        Ok(message_type) => message_type,
        Err(error) => return with_scheme_parse(result, &error),
    };
    let authoritative_short_type = short_message_type(&authoritative_type);
    if authoritative_short_type != "pacs.008" {
        return with_scheme_parse(
            result,
            &ParseError::InvalidEnvelope(format!(
                "Document namespace identifies {authoritative_type}, not pacs.008"
            )),
        );
    }

    if authoritative_type != "pacs.008.001.13" {
        let facts = match pacs008::Facts::from_xml(document_xml) {
            Ok(facts) => facts,
            Err(error) => return with_scheme_parse(result, &error),
        };
        result.merge(untyped_rules(&facts));
        result.errors.push(ValidationError::new(
            "/Document",
            Severity::Warning,
            "SCHEME_UNTYPED_VERSION",
            format!(
                "{} applies version-neutral field checks to {authoritative_type}; typed deserialization requires pacs.008.001.13",
                validator.name()
            ),
        ));
        return result;
    }

    let typed_xml = match document_for_typed_validation(document_xml) {
        Ok(document) => document,
        Err(error) => return with_scheme_parse(result, &error),
    };
    let document: pacs_008_001_13::Document = match mx20022_parse::de::from_str(&typed_xml) {
        Ok(document) => document,
        Err(error) => return with_scheme_parse(result, &error),
    };

    let Some(typed_result) = validator.validate_typed(&document, &authoritative_type) else {
        return with_scheme_parse(
            result,
            &ParseError::InvalidEnvelope(format!(
                "{} could not route pacs.008.001.13 to typed validation",
                validator.name()
            )),
        );
    };
    result.merge(typed_result);
    result
}

/// Build the XML view consumed by the generated type.
///
/// ISO 20022 supplementary envelopes accept `##any`, while the generated
/// model represents that payload as a string. `quick-xml` cannot deserialize
/// nested elements into that string. Scheme rules do not inspect supplementary
/// data, so remove only the opaque contents of `SplmtryData/Envlp`, allocating
/// when such content exists and otherwise retaining the borrowed Document.
fn document_for_typed_validation(document: &str) -> Result<Cow<'_, str>, ParseError> {
    let mut reader = quick_xml::Reader::from_str(document);
    let mut stack = Vec::<Vec<u8>>::new();
    let mut active_envelope: Option<(usize, usize)> = None;
    let mut opaque_ranges = Vec::<(usize, usize)>::new();

    loop {
        let event_start = usize::try_from(reader.buffer_position()).map_err(|_| {
            ParseError::InvalidEnvelope(
                "Document position exceeds addressable input size".to_owned(),
            )
        })?;
        let event = reader.read_event().map_err(|error| {
            ParseError::InvalidEnvelope(format!("malformed Document XML: {error}"))
        })?;

        match event {
            Event::Start(element) => {
                let local_name = element.local_name().as_ref().to_vec();
                if active_envelope.is_none()
                    && local_name == b"Envlp"
                    && stack.last().is_some_and(|parent| parent == b"SplmtryData")
                {
                    let content_start =
                        usize::try_from(reader.buffer_position()).map_err(|_| {
                            ParseError::InvalidEnvelope(
                                "Document position exceeds addressable input size".to_owned(),
                            )
                        })?;
                    active_envelope = Some((stack.len(), content_start));
                }
                stack.push(local_name);
            }
            Event::End(_) => {
                let Some(depth) = stack.len().checked_sub(1) else {
                    return Err(ParseError::InvalidEnvelope(
                        "Document contains an unmatched closing element".to_owned(),
                    ));
                };
                if let Some((envelope_depth, content_start)) = active_envelope {
                    if envelope_depth == depth {
                        opaque_ranges.push((content_start, event_start));
                        active_envelope = None;
                    }
                }
                stack.pop();
            }
            Event::Eof => break,
            _ => {}
        }
    }

    if opaque_ranges.is_empty() {
        return Ok(Cow::Borrowed(document));
    }

    let mut view = String::with_capacity(document.len());
    let mut cursor = 0usize;
    for (start, end) in opaque_ranges {
        view.push_str(&document[cursor..start]);
        view.push_str("supplementary-data-omitted-for-scheme-validation");
        cursor = end;
    }
    view.push_str(&document[cursor..]);
    Ok(Cow::Owned(view))
}

/// Read the namespace bound to the `Document` element in the original XML.
///
/// Reading the full stream preserves namespace declarations inherited from a
/// `BizMsgEnvlp` ancestor when `Document` itself is prefixed.
fn document_message_type(xml: &str) -> Result<String, ParseError> {
    let mut reader = NsReader::from_str(xml);

    loop {
        let (namespace, event) = reader.read_resolved_event().map_err(|error| {
            ParseError::InvalidEnvelope(format!("malformed Document XML: {error}"))
        })?;
        match event {
            Event::Start(element) | Event::Empty(element) => {
                if element.local_name().as_ref() != b"Document" {
                    continue;
                }
                return match namespace {
                    ResolveResult::Bound(namespace) => {
                        let namespace =
                            std::str::from_utf8(namespace.as_ref()).map_err(|error| {
                                ParseError::InvalidEnvelope(format!(
                                    "Document namespace is not UTF-8: {error}"
                                ))
                            })?;
                        parse_namespace(namespace).map(|message_id| message_id.dotted())
                    }
                    ResolveResult::Unbound => Err(ParseError::InvalidEnvelope(
                        "Document root has no namespace".to_owned(),
                    )),
                    ResolveResult::Unknown(prefix) => Err(ParseError::InvalidEnvelope(format!(
                        "Document namespace prefix is not declared: {}",
                        String::from_utf8_lossy(&prefix)
                    ))),
                };
            }
            Event::Eof => {
                return Err(ParseError::InvalidEnvelope(
                    "Document element is missing".to_owned(),
                ));
            }
            _ => {}
        }
    }
}

fn scheme_parse_error(error: &ParseError) -> ValidationResult {
    ValidationResult::new(vec![ValidationError::new(
        "/Document",
        Severity::Error,
        "SCHEME_PARSE",
        error.to_string(),
    )])
}

fn with_scheme_parse(mut raw_result: ValidationResult, error: &ParseError) -> ValidationResult {
    raw_result.merge(scheme_parse_error(error));
    raw_result
}
