//! `mx20022-codegen` — XSD → Rust source code generator for ISO 20022.
//!
//! # Usage
//!
//! ```text
//! mx20022-codegen <XSD_FILE> [--output <FILE>]
//! ```
//!
//! Reads the given XSD file, parses it, lowers to the IR, emits Rust source
//! code, and writes it to stdout (or `--output FILE`).
//!
//! # Example
//!
//! ```text
//! cargo run -p mx20022-codegen -- schemas/head/head.001.001.04.xsd
//! cargo run -p mx20022-codegen -- schemas/head/head.001.001.04.xsd --output out.rs
//! ```

use std::fs;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (xsd_path, output_path) = parse_args(&args).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        eprintln!();
        eprintln!("usage: mx20022-codegen <XSD_FILE> [--output <FILE>]");
        process::exit(1);
    });

    // Open and parse the XSD file.
    let file = fs::File::open(&xsd_path).unwrap_or_else(|e| {
        eprintln!("error: cannot open '{}': {e}", xsd_path.display());
        process::exit(1);
    });

    let schema = mx20022_codegen::xsd::parse(BufReader::new(file)).unwrap_or_else(|e| {
        eprintln!("error: XSD parse failed for '{}': {e}", xsd_path.display());
        process::exit(1);
    });

    // Lower to IR.
    let graph = mx20022_codegen::ir::lower(&schema).unwrap_or_else(|e| {
        eprintln!(
            "error: IR lowering failed for '{}': {e}",
            xsd_path.display()
        );
        process::exit(1);
    });

    // Emit Rust source.
    let rust_source = mx20022_codegen::emit::emit(&graph);

    // Write output.
    match output_path {
        Some(path) => {
            fs::write(&path, &rust_source).unwrap_or_else(|e| {
                eprintln!("error: cannot write to '{}': {e}", path.display());
                process::exit(1);
            });
            eprintln!("wrote {} bytes to '{}'", rust_source.len(), path.display());
        }
        None => {
            io::stdout()
                .write_all(rust_source.as_bytes())
                .unwrap_or_else(|e| {
                    eprintln!("error: write to stdout failed: {e}");
                    process::exit(1);
                });
        }
    }
}

/// Parse CLI arguments into (`xsd_path`, optional `output_path`).
fn parse_args(args: &[String]) -> Result<(PathBuf, Option<PathBuf>), String> {
    let mut xsd_path: Option<PathBuf> = None;
    let mut output_path: Option<PathBuf> = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--output" | "-o" => {
                i += 1;
                if i >= args.len() {
                    return Err("--output requires a file path argument".to_owned());
                }
                output_path = Some(PathBuf::from(&args[i]));
            }
            arg if arg.starts_with('-') => {
                return Err(format!("unknown flag: {arg}"));
            }
            arg => {
                if xsd_path.is_some() {
                    return Err(
                        "too many positional arguments; expected exactly one XSD file path"
                            .to_owned(),
                    );
                }
                xsd_path = Some(PathBuf::from(arg));
            }
        }
        i += 1;
    }

    let xsd = xsd_path.ok_or_else(|| "missing required argument: <XSD_FILE>".to_owned())?;
    Ok((xsd, output_path))
}
