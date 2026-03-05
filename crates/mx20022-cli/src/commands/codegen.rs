//! Implementation of the `codegen` subcommand.
//!
//! Reads an XSD schema file and generates idiomatic Rust types using the
//! `mx20022-codegen` pipeline: parse → lower to IR → emit Rust source.

use std::io::{BufReader, Write as IoWrite};
use std::path::Path;

use mx20022_codegen::{emit, ir, xsd};

/// Error type returned by the codegen command.
#[derive(Debug)]
pub enum CodegenError {
    /// The XSD file could not be read.
    Io(std::io::Error),
    /// The XSD could not be parsed.
    Xsd(xsd::ParseError),
    /// The XSD could not be lowered to IR.
    Lower(ir::LowerError),
    /// The file exceeds the maximum allowed size.
    FileTooLarge { size: u64, max: u64 },
}

impl std::fmt::Display for CodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodegenError::Io(e) => write!(f, "I/O error: {e}"),
            CodegenError::Xsd(e) => write!(f, "XSD parse error: {e}"),
            CodegenError::Lower(e) => write!(f, "IR lowering error: {e}"),
            CodegenError::FileTooLarge { size, max } => {
                write!(
                    f,
                    "file is too large ({size} bytes); maximum allowed is {max} bytes"
                )
            }
        }
    }
}

impl From<std::io::Error> for CodegenError {
    fn from(e: std::io::Error) -> Self {
        CodegenError::Io(e)
    }
}

impl From<xsd::ParseError> for CodegenError {
    fn from(e: xsd::ParseError) -> Self {
        CodegenError::Xsd(e)
    }
}

impl From<ir::LowerError> for CodegenError {
    fn from(e: ir::LowerError) -> Self {
        CodegenError::Lower(e)
    }
}

impl std::error::Error for CodegenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CodegenError::Io(e) => Some(e),
            CodegenError::Xsd(e) => Some(e),
            CodegenError::Lower(e) => Some(e),
            CodegenError::FileTooLarge { .. } => None,
        }
    }
}

/// Run the `codegen` subcommand.
///
/// Parses the XSD at `file`, lowers it to the IR, emits Rust source, and
/// writes it to `output` (or stdout if `output` is `None`).
///
/// # Errors
///
/// Returns an error if the XSD cannot be read, parsed, or lowered.
/// Maximum file size accepted by the codegen command (10 MB).
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

pub fn run(file: &Path, output: Option<&Path>) -> Result<(), CodegenError> {
    let meta = std::fs::metadata(file)?;
    if meta.len() > MAX_FILE_SIZE {
        return Err(CodegenError::FileTooLarge {
            size: meta.len(),
            max: MAX_FILE_SIZE,
        });
    }
    let f = std::fs::File::open(file)?;
    let schema = xsd::parse(BufReader::new(f))?;
    let graph = ir::lower(&schema)?;
    let rust_source = emit::emit(&graph);

    match output {
        Some(out_path) => {
            std::fs::write(out_path, rust_source.as_bytes())?;
            eprintln!(
                "Generated {} bytes → {}",
                rust_source.len(),
                out_path.display()
            );
        }
        None => {
            std::io::stdout()
                .write_all(rust_source.as_bytes())
                .map_err(CodegenError::Io)?;
        }
    }

    Ok(())
}
