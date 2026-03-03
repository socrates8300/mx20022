//! XML parsing and serialization for ISO 20022 messages.
//!
//! This crate wraps [`quick_xml`] with [`serde`] support into a convenient API
//! for reading and writing ISO 20022 financial messages.
//!
//! # Quick start
//!
//! ```no_run
//! use mx20022_parse::{de, ser, envelope};
//! # use mx20022_model::generated::head::BusinessApplicationHeaderV04;
//!
//! // Detect message type from raw XML
//! let raw = r#"<AppHdr xmlns="urn:iso:std:iso:20022:tech:xsd:head.001.001.04">...</AppHdr>"#;
//! let msg_id = envelope::detect_message_type(raw).unwrap();
//! assert_eq!(msg_id.family, "head");
//!
//! // Parse a known type
//! // let hdr: BusinessApplicationHeaderV04 = de::from_str(raw).unwrap();
//!
//! // Serialize back to XML
//! // let xml = ser::to_string(&hdr).unwrap();
//! ```

pub mod de;
pub mod envelope;
pub mod error;
pub mod ser;

pub use error::ParseError;
