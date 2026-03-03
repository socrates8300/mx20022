//! `mx20022-cli` — ISO 20022 message toolkit command-line interface.

use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "mx20022-cli", about = "ISO 20022 message toolkit", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Inspect an ISO 20022 XML message and display its structure
    Inspect {
        /// Path to the XML file
        file: std::path::PathBuf,
    },
    /// Validate an ISO 20022 XML message against schema rules
    ///
    /// Optionally validates against payment-scheme-specific rules with --scheme.
    /// Allowed values for --scheme: fednow, sepa, cbpr
    Validate {
        /// Path to the XML file
        file: std::path::PathBuf,
        /// Payment scheme for additional validation (fednow | sepa | cbpr)
        #[arg(long, value_name = "SCHEME")]
        scheme: Option<String>,
    },
    /// Generate Rust types from an XSD schema
    Codegen {
        /// Path to the XSD file
        file: std::path::PathBuf,
        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<std::path::PathBuf>,
    },
    /// Translate between SWIFT MT and ISO 20022 MX messages
    ///
    /// Supported --to values:
    ///   pacs008  — MT103  → pacs.008.001.13 XML
    ///   mt103    — pacs.008 XML → MT103 text
    ///   pacs009  — MT202  → pacs.009.001.10 XML
    ///   mt202    — pacs.009 XML → MT202 text
    ///   camt053  — MT940  → camt.053.001.11 XML
    ///   mt940    — camt.053 XML → MT940 text
    Translate {
        /// Path to the input message file (MT text or MX XML)
        file: std::path::PathBuf,
        /// Target format (pacs008 | mt103 | pacs009 | mt202 | camt053 | mt940)
        #[arg(long)]
        to: String,
        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<std::path::PathBuf>,
        /// Override the MX message ID (derived from filename by default)
        #[arg(long)]
        msg_id: Option<String>,
        /// Override the MX creation timestamp (ISO 8601, e.g. 2023-06-15T10:00:00)
        #[arg(long)]
        creation_time: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let result: Result<(), Box<dyn std::error::Error>> = match cli.command {
        Command::Inspect { file } => {
            commands::inspect::run(&file).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        Command::Validate { file, scheme } => commands::validate::run(&file, scheme.as_deref())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>),
        Command::Codegen { file, output } => commands::codegen::run(&file, output.as_deref())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>),
        Command::Translate {
            file,
            to,
            output,
            msg_id,
            creation_time,
        } => commands::translate::run(
            &file,
            &to,
            output.as_deref(),
            msg_id.as_deref(),
            creation_time.as_deref(),
        )
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>),
    };
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
