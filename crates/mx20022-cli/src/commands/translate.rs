//! Implementation of the `translate` subcommand.
//!
//! Translates between SWIFT MT messages and ISO 20022 MX messages.
//!
//! Supported conversions:
//!
//! | `--to` value | Input         | Output           |
//! |---|---|---|
//! | `pacs008`    | MT103 text    | pacs.008.001.13 XML |
//! | `mt103`      | pacs.008 XML  | MT103 text       |
//! | `pacs009`    | MT202 text    | pacs.009.001.10 XML |
//! | `mt202`      | pacs.009 XML  | MT202 text       |
//! | `camt053`    | MT940 text    | camt.053.001.11 XML |
//! | `mt940`      | camt.053 XML  | MT940 text       |

use std::path::Path;

use mx20022_model::generated::{
    camt::camt_053_001_11 as camt053,
    pacs::{pacs_008_001_13 as pacs008, pacs_009_001_10 as pacs009},
};
use mx20022_translate::mappings::{
    camt053_to_mt940::camt053_to_mt940, mt103_to_pacs008::mt103_to_pacs008,
    mt202_to_pacs009::mt202_to_pacs009, mt940_to_camt053::mt940_to_camt053,
    pacs008_to_mt103::pacs008_to_mt103, pacs009_to_mt202::pacs009_to_mt202,
};
use mx20022_translate::mappings::{TranslationError, TranslationWarnings};
use mx20022_translate::mt::{
    fields::mt103::parse_mt103, fields::mt202::parse_mt202, fields::mt940::parse_mt940,
    parser::parse,
};

/// Error type returned by the translate command.
#[derive(Debug)]
pub enum TranslateError {
    /// An I/O error reading the input file.
    Io(std::io::Error),
    /// Translation failed.
    Translation(TranslationError),
    /// An XML parse or serialisation error.
    Xml(mx20022_parse::ParseError),
    /// The file exceeds the maximum allowed size.
    FileTooLarge { size: u64, max: u64 },
    /// The requested target format is not recognised.
    UnknownTarget(String),
}

impl std::fmt::Display for TranslateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslateError::Io(e) => write!(f, "I/O error: {e}"),
            TranslateError::Translation(e) => write!(f, "translation error: {e}"),
            TranslateError::Xml(e) => write!(f, "XML error: {e}"),
            TranslateError::FileTooLarge { size, max } => {
                write!(
                    f,
                    "file is too large ({size} bytes); maximum allowed is {max} bytes"
                )
            }
            TranslateError::UnknownTarget(t) => write!(
                f,
                "unknown target format '{t}' — valid values: pacs008, mt103, pacs009, mt202, camt053, mt940"
            ),
        }
    }
}

impl std::error::Error for TranslateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TranslateError::Io(e) => Some(e),
            TranslateError::Translation(e) => Some(e),
            TranslateError::Xml(e) => Some(e),
            TranslateError::FileTooLarge { .. } | TranslateError::UnknownTarget(_) => None,
        }
    }
}

impl From<std::io::Error> for TranslateError {
    fn from(e: std::io::Error) -> Self {
        TranslateError::Io(e)
    }
}

impl From<TranslationError> for TranslateError {
    fn from(e: TranslationError) -> Self {
        TranslateError::Translation(e)
    }
}

impl From<mx20022_parse::ParseError> for TranslateError {
    fn from(e: mx20022_parse::ParseError) -> Self {
        TranslateError::Xml(e)
    }
}

/// Print any warnings to stderr.
fn print_warnings(warnings: &TranslationWarnings) {
    for w in &warnings.warnings {
        eprintln!("warning [{}]: {}", w.field, w.message);
    }
}

/// Produce a deterministic message-ID from the input path stem.
fn derive_msg_id(file: &Path) -> String {
    file.file_stem()
        .and_then(|s| s.to_str())
        .map_or_else(|| "MSG001".to_owned(), str::to_uppercase)
}

/// Run the `translate` subcommand.
///
/// # Arguments
///
/// * `file` — path to the input message file (MT text or MX XML)
/// * `to` — target format identifier (e.g. `"pacs008"`, `"mt103"`)
/// * `output` — optional path to write the result; stdout if `None`
/// * `msg_id` — optional message ID override; derived from filename if `None`
/// * `creation_time` — optional ISO-8601 creation timestamp; required for MT→MX
///   conversions when `msg_id` is supplied
///
/// # Errors
///
/// Returns an error if the file cannot be read, the translation fails, or the
/// target format is not recognised.
/// Maximum file size accepted by the translate command (10 MB).
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

pub fn run(
    file: &Path,
    to: &str,
    output: Option<&Path>,
    msg_id: Option<&str>,
    creation_time: Option<&str>,
) -> Result<(), TranslateError> {
    let meta = std::fs::metadata(file)?;
    if meta.len() > MAX_FILE_SIZE {
        return Err(TranslateError::FileTooLarge {
            size: meta.len(),
            max: MAX_FILE_SIZE,
        });
    }
    let input = std::fs::read_to_string(file)?;
    let effective_msg_id = msg_id.map_or_else(|| derive_msg_id(file), str::to_owned);
    let effective_ts = creation_time.unwrap_or("2000-01-01T00:00:00").to_owned();

    let output_text = match to {
        // ----------------------------------------------------------------
        // MT → MX
        // ----------------------------------------------------------------
        "pacs008" => {
            let msg = parse(&input).map_err(TranslationError::MtParse)?;
            let mt103 = parse_mt103(&msg.block4).map_err(TranslationError::MtParse)?;
            let result = mt103_to_pacs008(&mt103, &effective_msg_id, &effective_ts)?;
            print_warnings(&result.warnings);
            mx20022_parse::ser::to_string_with_declaration(&result.message)?
        }
        "pacs009" => {
            let msg = parse(&input).map_err(TranslationError::MtParse)?;
            let mt202 = parse_mt202(&msg.block4).map_err(TranslationError::MtParse)?;
            let result = mt202_to_pacs009(&mt202, &effective_msg_id, &effective_ts)?;
            print_warnings(&result.warnings);
            mx20022_parse::ser::to_string_with_declaration(&result.message)?
        }
        "camt053" => {
            let msg = parse(&input).map_err(TranslationError::MtParse)?;
            let mt940 = parse_mt940(&msg.block4).map_err(TranslationError::MtParse)?;
            let result = mt940_to_camt053(&mt940, &effective_msg_id, &effective_ts)?;
            print_warnings(&result.warnings);
            mx20022_parse::ser::to_string_with_declaration(&result.message)?
        }
        // ----------------------------------------------------------------
        // MX → MT
        // ----------------------------------------------------------------
        "mt103" => {
            let doc: pacs008::Document = mx20022_parse::de::from_str(&input)?;
            let result = pacs008_to_mt103(&doc)?;
            print_warnings(&result.warnings);
            result.message
        }
        "mt202" => {
            let doc: pacs009::Document = mx20022_parse::de::from_str(&input)?;
            let result = pacs009_to_mt202(&doc)?;
            print_warnings(&result.warnings);
            result.message
        }
        "mt940" => {
            let doc: camt053::Document = mx20022_parse::de::from_str(&input)?;
            let result = camt053_to_mt940(&doc)?;
            print_warnings(&result.warnings);
            result.message
        }
        other => return Err(TranslateError::UnknownTarget(other.to_owned())),
    };

    match output {
        Some(path) => std::fs::write(path, &output_text)?,
        None => print!("{output_text}"),
    }

    Ok(())
}
