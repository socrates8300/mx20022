//! Implementation of the `validate` subcommand.
//!
//! Reads an ISO 20022 XML file, detects its message type, and runs
//! IBAN, BIC, currency, LEI, and date/time validation rules against
//! well-known field paths extracted from the raw XML.
//!
//! When `--scheme` is provided (one of `fednow`, `sepa`, `cbpr`), scheme-
//! specific rules are applied in addition to the generic rule checks.
//!
//! Because the CLI does not statically know which message version it is
//! processing, field extraction uses lightweight XML scanning rather than
//! fully-typed deserialization.  This keeps the command free of feature-gated
//! model types while still exercising the rule engine.

use std::path::Path;

use super::MAX_FILE_SIZE;
use mx20022_parse::envelope::detect_message_type;
use mx20022_validate::schemes::{
    cbpr::CbprPlusValidator,
    fednow::FedNowValidator,
    sepa::SepaValidator,
    xml_scan::{extract_all_attributes, extract_all_elements, extract_element},
    SchemeValidator,
};
use mx20022_validate::{RuleRegistry, Severity, ValidationResult};

/// Error type returned by the validate command.
#[derive(Debug)]
pub enum ValidateError {
    /// The file could not be read.
    Io(std::io::Error),
    /// The XML does not contain a valid ISO 20022 envelope.
    Parse(mx20022_parse::ParseError),
    /// The file exceeds the maximum allowed size.
    FileTooLarge { size: u64, max: u64 },
    /// An unknown scheme was specified with `--scheme`.
    UnknownScheme(String),
    /// Validation completed but errors were found.
    ValidationFailed { error_count: usize },
}

impl std::fmt::Display for ValidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidateError::Io(e) => write!(f, "I/O error: {e}"),
            ValidateError::Parse(e) => write!(f, "parse error: {e}"),
            ValidateError::FileTooLarge { size, max } => {
                write!(
                    f,
                    "file is too large ({size} bytes); maximum allowed is {max} bytes"
                )
            }
            ValidateError::UnknownScheme(s) => {
                write!(
                    f,
                    "unknown scheme `{s}`; expected one of: fednow, sepa, cbpr"
                )
            }
            ValidateError::ValidationFailed { error_count } => {
                write!(f, "{error_count} validation error(s) found")
            }
        }
    }
}

impl From<std::io::Error> for ValidateError {
    fn from(e: std::io::Error) -> Self {
        ValidateError::Io(e)
    }
}

impl From<mx20022_parse::ParseError> for ValidateError {
    fn from(e: mx20022_parse::ParseError) -> Self {
        ValidateError::Parse(e)
    }
}

impl std::error::Error for ValidateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ValidateError::Io(e) => Some(e),
            ValidateError::Parse(e) => Some(e),
            ValidateError::FileTooLarge { .. }
            | ValidateError::UnknownScheme(_)
            | ValidateError::ValidationFailed { .. } => None,
        }
    }
}

/// Resolve a scheme name string to a boxed `SchemeValidator`.
///
/// Returns `None` if `scheme` is `None`, or an error for an unrecognised name.
fn resolve_scheme(scheme: Option<&str>) -> Result<Option<Box<dyn SchemeValidator>>, ValidateError> {
    match scheme {
        None => Ok(None),
        Some("fednow") => Ok(Some(Box::new(FedNowValidator::new()))),
        Some("sepa") => Ok(Some(Box::new(SepaValidator::new()))),
        Some("cbpr") => Ok(Some(Box::new(CbprPlusValidator::new()))),
        Some(other) => Err(ValidateError::UnknownScheme(other.to_owned())),
    }
}

/// Run the `validate` subcommand.
///
/// Exits with code 0 on success and 1 if validation errors are found.
/// The exit code is communicated via the caller in `main.rs`.
///
/// When `scheme` is `Some("fednow")`, `Some("sepa")`, or `Some("cbpr")`,
/// scheme-specific validation is performed after the generic rule checks.
///
/// # Errors
///
/// Returns an error if the file cannot be read, has no recognisable ISO 20022
/// namespace, or an unknown scheme is requested.
pub fn run(file: &Path, scheme: Option<&str>) -> Result<(), ValidateError> {
    let meta = std::fs::metadata(file)?;
    if meta.len() > MAX_FILE_SIZE {
        return Err(ValidateError::FileTooLarge {
            size: meta.len(),
            max: MAX_FILE_SIZE,
        });
    }
    let xml = std::fs::read_to_string(file)?;

    let msg_id = detect_message_type(&xml)?;
    println!("Validating: {} ({})", file.display(), msg_id.dotted());
    if let Some(s) = scheme {
        println!("Scheme:     {s}");
    }
    println!();

    let registry = RuleRegistry::with_defaults();
    let mut result = ValidationResult::default();

    // --- BIC fields --------------------------------------------------------
    // BICFI appears in both AppHdr and pacs messages.
    let bic_tags = ["BICFI", "BIC"];
    for tag in bic_tags {
        for (idx, value) in extract_all_elements(&xml, tag).into_iter().enumerate() {
            let path = format!("//{tag}[{}]", idx + 1);
            let errors = registry.validate_field(value, &path, &["BIC_CHECK"]);
            result.errors.extend(errors);
        }
    }

    // --- IBAN fields -------------------------------------------------------
    for (idx, value) in extract_all_elements(&xml, "IBAN").into_iter().enumerate() {
        let path = format!("//IBAN[{}]", idx + 1);
        let errors = registry.validate_field(value, &path, &["IBAN_CHECK"]);
        result.errors.extend(errors);
    }

    // --- Currency codes ----------------------------------------------------
    // Appears as element content (<Ccy>USD</Ccy>) and as an attribute (Ccy="USD").
    for (idx, value) in extract_all_elements(&xml, "Ccy").into_iter().enumerate() {
        let path = format!("//Ccy[{}]", idx + 1);
        let errors = registry.validate_field(value, &path, &["CURRENCY_CHECK"]);
        result.errors.extend(errors);
    }
    for (idx, value) in extract_all_attributes(&xml, "Ccy").into_iter().enumerate() {
        let path = format!("//@Ccy[{}]", idx + 1);
        let errors = registry.validate_field(value, &path, &["CURRENCY_CHECK"]);
        result.errors.extend(errors);
    }

    // --- LEI elements ------------------------------------------------------
    for (idx, value) in extract_all_elements(&xml, "LEI").into_iter().enumerate() {
        let path = format!("//LEI[{}]", idx + 1);
        let errors = registry.validate_field(value, &path, &["LEI_CHECK"]);
        result.errors.extend(errors);
    }

    // --- Datetime fields ---------------------------------------------------
    // CreDtTm appears in GrpHdr; validate as ISO 8601 datetime.
    for (idx, value) in extract_all_elements(&xml, "CreDtTm")
        .into_iter()
        .enumerate()
    {
        let path = format!("//CreDtTm[{}]", idx + 1);
        let errors = registry.validate_field(value, &path, &["DATETIME_CHECK"]);
        result.errors.extend(errors);
    }

    // --- Date fields -------------------------------------------------------
    // IntrBkSttlmDt appears in pacs messages; validate as ISO 8601 date.
    for (idx, value) in extract_all_elements(&xml, "IntrBkSttlmDt")
        .into_iter()
        .enumerate()
    {
        let path = format!("//IntrBkSttlmDt[{}]", idx + 1);
        let errors = registry.validate_field(value, &path, &["DATE_CHECK"]);
        result.errors.extend(errors);
    }

    // --- Message ID presence check -----------------------------------------
    // BizMsgIdr (AppHdr) or MsgId (pacs/pain/camt GrpHdr)
    let msg_id_present = extract_element(&xml, "BizMsgIdr")
        .or_else(|| extract_element(&xml, "MsgId"))
        .is_some();

    if !msg_id_present {
        use mx20022_validate::ValidationError;
        result.errors.push(ValidationError::new(
            "//GrpHdr/MsgId",
            Severity::Warning,
            "MSG_ID_MISSING",
            "No message identifier (BizMsgIdr / MsgId) found in document",
        ));
    }

    // --- Scheme-specific validation ----------------------------------------
    let scheme_validator = resolve_scheme(scheme)?;
    if let Some(validator) = scheme_validator {
        let scheme_result = validator.validate(&xml, &msg_id.dotted());
        result.merge(scheme_result);
    }

    // --- Print results -----------------------------------------------------
    let error_count = result.error_count();
    let warning_count = result.warning_count();

    if result.errors.is_empty() {
        println!("Result: OK — no findings");
    } else {
        for finding in &result.errors {
            let level = match finding.severity {
                Severity::Error => "ERROR  ",
                Severity::Warning => "WARNING",
                Severity::Info => "INFO   ",
            };
            println!(
                "[{level}] {} — {} ({})",
                finding.path, finding.message, finding.rule_id
            );
        }
        println!();
        println!("Result: {error_count} error(s), {warning_count} warning(s)");
    }

    if !result.is_valid() {
        // Signal failure to the caller which will call std::process::exit(1).
        return Err(ValidateError::ValidationFailed { error_count });
    }

    Ok(())
}
