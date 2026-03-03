//! Generated ISO 20022 message types.
//!
//! Each sub-module corresponds to a message family (e.g. `head`, `pacs`, `pain`, `camt`).
//! Within each family, modules are named by their full message identifier
//! (e.g. `head_001_001_04` for `head.001.001.04`).
//!
//! **Do not edit these files by hand.** They are produced by `mx20022-codegen`.

#[cfg(feature = "camt")]
pub mod camt;

#[cfg(feature = "head")]
pub mod head;

#[cfg(feature = "pacs")]
pub mod pacs;

#[cfg(feature = "pain")]
pub mod pain;
