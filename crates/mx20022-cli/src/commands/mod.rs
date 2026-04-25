//! Subcommand implementations for the `mx20022-cli` tool.

pub mod codegen;
pub mod inspect;
pub mod translate;
pub mod validate;

/// Maximum file size accepted by any command (10 MB).
pub(crate) const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
