//! SWIFT MT message parser.
//!
//! Parses SWIFT FIN MT messages (MT103, MT202, MT940) from their text
//! representation into structured Rust types.
//!
//! # Quick Start
//!
//! ```rust
//! use mx20022_translate::mt;
//! use mx20022_translate::mt::fields::mt103::parse_mt103;
//!
//! let raw = "\
//! {1:F01BANKBEBBAXXX0000000000}\
//! {2:I103BANKDEFFXXXXN}\
//! {3:{108:MYREF}}\
//! {4:\n\
//! :20:REF123\n\
//! :23B:CRED\n\
//! :32A:230615EUR1000,00\n\
//! :50K:ACME CORP\n\
//! :59:JANE SMITH\n\
//! :71A:SHA\n\
//! -}\
//! {5:{CHK:ABCDEF1234}}";
//!
//! let msg = mt::parse(raw).unwrap();
//! assert_eq!(msg.message_type(), Some("103"));
//! let mt103 = parse_mt103(&msg.block4).unwrap();
//! assert_eq!(mt103.senders_reference, "REF123");
//! ```

pub mod error;
pub mod fields;
pub mod parser;
pub mod types;

pub use error::MtError;
pub use parser::parse;
pub use types::MtMessage;
