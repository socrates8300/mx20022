//! Implementation of the `inspect` subcommand.
//!
//! Reads an ISO 20022 XML file, detects its message type from the namespace,
//! and prints a human-readable summary of the envelope.

use std::path::Path;

use super::MAX_FILE_SIZE;
use mx20022_parse::envelope::detect_message_type;

/// Error type returned by the inspect command.
#[derive(Debug)]
pub enum InspectError {
    /// The file could not be read.
    Io(std::io::Error),
    /// The XML does not contain a valid ISO 20022 envelope.
    Parse(mx20022_parse::ParseError),
    /// The file exceeds the maximum allowed size.
    FileTooLarge { size: u64, max: u64 },
}

impl std::fmt::Display for InspectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InspectError::Io(e) => write!(f, "I/O error: {e}"),
            InspectError::Parse(e) => write!(f, "parse error: {e}"),
            InspectError::FileTooLarge { size, max } => {
                write!(
                    f,
                    "file is too large ({size} bytes); maximum allowed is {max} bytes"
                )
            }
        }
    }
}

impl From<std::io::Error> for InspectError {
    fn from(e: std::io::Error) -> Self {
        InspectError::Io(e)
    }
}

impl From<mx20022_parse::ParseError> for InspectError {
    fn from(e: mx20022_parse::ParseError) -> Self {
        InspectError::Parse(e)
    }
}

impl std::error::Error for InspectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InspectError::Io(e) => Some(e),
            InspectError::Parse(e) => Some(e),
            InspectError::FileTooLarge { .. } => None,
        }
    }
}

/// Run the `inspect` subcommand.
///
/// # Errors
///
/// Returns an error if the file cannot be read or has no recognisable ISO 20022
/// namespace.
pub fn run(file: &Path) -> Result<(), InspectError> {
    let meta = std::fs::metadata(file)?;
    if meta.len() > MAX_FILE_SIZE {
        return Err(InspectError::FileTooLarge {
            size: meta.len(),
            max: MAX_FILE_SIZE,
        });
    }
    let xml = std::fs::read_to_string(file)?;

    println!("File: {}", file.display());
    println!("Size: {} bytes", xml.len());
    println!();

    let msg_id = match detect_message_type(&xml) {
        Ok(id) => id,
        Err(e) => {
            println!("Message type: (unrecognised — {e})");
            return Ok(());
        }
    };

    println!("Message type : {}", msg_id.dotted());
    println!("  Family     : {}", msg_id.family);
    println!("  Message ID : {}", msg_id.msg_id);
    println!("  Variant    : {}", msg_id.variant);
    println!("  Version    : {}", msg_id.version);
    println!(
        "  Namespace  : urn:iso:std:iso:20022:tech:xsd:{}",
        msg_id.dotted()
    );

    Ok(())
}
