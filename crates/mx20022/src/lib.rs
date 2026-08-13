//! ISO 20022 financial message toolkit.
//!
//! `mx20022` is an umbrella crate that re-exports the four workspace
//! components needed to read, write, validate, and translate
//! ISO 20022 messages and SWIFT MT messages:
//!
//! - [`model`] — strongly-typed message structs generated from the
//!   official XSD schemas (`pacs`, `pain`, `camt`, `head`).
//! - [`parse`] — XML deserialization, serialization, and envelope
//!   message-type detection.
//! - [`validate`] — field-level rule, schema, and scheme (`FedNow` / SEPA /
//!   CBPR+) validation.
//! - [`translate`] — bidirectional MT ↔ MX translation for the supported
//!   message pairs (MT103 ↔ pacs.008, MT202 ↔ pacs.009, MT940 ↔ camt.053).
//!
//! See the per-crate docs for full API surface; the [`prelude`] re-exports
//! the items most users need for typical workflows.
//!
//! # Quick start: parse an ISO 20022 XML message
//!
//! ```rust
//! use mx20022::model::generated::pacs::pacs_008_001_13::Document;
//! use mx20022::parse::{de, envelope};
//!
//! // This fixture deliberately declares the payment namespace on its wrapper.
//! let xml = include_str!("../tests/fixtures/inherited_namespace_pacs008.xml");
//! let id = envelope::detect_message_type(xml).unwrap();
//! assert_eq!(id.family, "pacs");
//! assert_eq!(id.msg_id, "008");
//! assert_eq!(id.version, "13");
//!
//! let document_xml = de::document_xml(xml).unwrap();
//! let document: Document = de::from_str(document_xml).unwrap();
//! assert_eq!(document.fi_to_fi_cstmr_cdt_trf.grp_hdr.msg_id.0, "README-INHERITED-NS");
//! ```
//!
//! # Quick start: translate MT103 to pacs.008
//!
//! ```rust
//! use mx20022::prelude::*;
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
//! let mt = mt::parse(raw).unwrap();
//! let mt103 = mt::fields::mt103::parse_mt103(&mt.block4).unwrap();
//! let result = mt103_to_pacs008(&mt103, "MSG-1", "2023-06-15T10:00:00").unwrap();
//! assert!(result.warnings.is_empty() || !result.warnings.is_empty());
//! ```
//!
//! # Quick start: validate a single field
//!
//! ```rust
//! use mx20022::prelude::*;
//!
//! let registry = RuleRegistry::with_defaults();
//! let errors = registry.validate_field(
//!     "GB82WEST12345698765432",
//!     "/Document/CdtTrfTxInf/CdtrAcct/Id/IBAN",
//!     &["IBAN_CHECK"],
//! );
//! assert!(errors.is_empty());
//! ```

pub use mx20022_model as model;
pub use mx20022_parse as parse;
pub use mx20022_translate as translate;
pub use mx20022_validate as validate;

/// Curated re-exports for typical workflows.
///
/// Glob-import this module to get the headline error types, the
/// translation entry-point functions, the SWIFT MT parser, and the
/// validation registry without remembering each sub-crate's path.
///
/// ```
/// use mx20022::prelude::*;
/// ```
pub mod prelude {
    // ---- Errors ----
    pub use crate::model::common::BuilderError;
    pub use crate::parse::ParseError;
    pub use crate::translate::mappings::{
        TranslationError, TranslationResult, TranslationWarning, TranslationWarnings,
    };
    pub use crate::translate::mt::MtError;
    pub use crate::validate::{Severity, ValidationError, ValidationResult};

    // ---- Common ----
    pub use crate::model::common::ChoiceWrapper;

    // ---- Parsing / serialization ----
    pub use crate::parse::envelope::{detect_message_type, MessageId};

    // ---- Validation ----
    pub use crate::validate::{RuleRegistry, SchemaValidator};

    // ---- MT (re-export the module so users can name `mt::parse`, `mt::fields::*`) ----
    pub use crate::translate::mt;

    // ---- MT ↔ MX translation entry points ----
    pub use crate::translate::mappings::camt053_to_mt940::camt053_to_mt940;
    pub use crate::translate::mappings::mt103_to_pacs008::mt103_to_pacs008;
    pub use crate::translate::mappings::mt202_to_pacs009::mt202_to_pacs009;
    pub use crate::translate::mappings::mt940_to_camt053::mt940_to_camt053;
    pub use crate::translate::mappings::pacs008_to_mt103::pacs008_to_mt103;
    pub use crate::translate::mappings::pacs009_to_mt202::pacs009_to_mt202;
}
