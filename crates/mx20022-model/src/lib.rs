//! Generated and hand-written types for ISO 20022 financial messages.
//!
//! # Feature flags
//!
//! Each message family is behind a feature flag:
//! - `head` — Business Application Header (`head.001`)
//! - `pacs` — Payments Clearing and Settlement (`pacs.002`, `pacs.008`)
//! - `pain` — Payment Initiation (future)
//! - `camt` — Cash Management (future)
//! - `all` — enables all families

pub mod common;
pub mod generated;
