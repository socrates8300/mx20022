//! Payments Clearing and Settlement (pacs) message types.
//!
//! Types are scoped by message version to avoid name collisions between
//! shared type names (e.g. `PostalAddress27` appears in both pacs.002 and
//! pacs.008).  Import the specific version module you need:
//!
//! ```ignore
//! use mx20022_model::generated::pacs::pacs_008_001_13::Document;
//! ```

pub mod pacs_002_001_14;
pub mod pacs_004_001_11;
pub mod pacs_008_001_13;
pub mod pacs_009_001_10;
pub mod pacs_028_001_05;
