//! MT ↔ MX field mapping rules.
//!
//! Each submodule handles translation between a specific MT message type
//! and its ISO 20022 MX equivalent.
//!
//! # Supported translations
//!
//! | Direction | Source | Target |
//! |---|---|---|
//! | MT → MX | MT103 | pacs.008.001.13 |
//! | MX → MT | pacs.008.001.13 | MT103 |
//! | MT → MX | MT202 | pacs.009.001.10 |
//! | MX → MT | pacs.009.001.10 | MT202 |
//! | MT → MX | MT940 | camt.053.001.11 |
//! | MX → MT | camt.053.001.11 | MT940 |

pub mod camt053_to_mt940;
pub mod charset;
pub mod error;
pub mod helpers;
pub mod mt103_to_pacs008;
pub mod mt202_to_pacs009;
pub mod mt940_to_camt053;
pub mod pacs008_to_mt103;
pub mod pacs009_to_mt202;

pub use error::{TranslationError, TranslationResult, TranslationWarning, TranslationWarnings};
