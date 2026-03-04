//! Schema and business rule validation for ISO 20022 financial messages.
//!
//! # Overview
//!
//! This crate provides layered validation for ISO 20022 messages:
//!
//! 1. **Rule layer** ([`rules`]) — individual, reusable validators (IBAN, BIC,
//!    length, regex pattern).
//! 2. **Schema layer** ([`schema`]) — orchestrates rules against field constraints
//!    derived from XSD facets.
//!
//! # Quick start
//!
//! ```rust
//! use mx20022_validate::rules::RuleRegistry;
//!
//! let registry = RuleRegistry::with_defaults();
//!
//! // Validate an IBAN
//! let errors = registry.validate_field(
//!     "GB82WEST12345698765432",
//!     "/Document/CdtTrfTxInf/CdtrAcct/Id/IBAN",
//!     &["IBAN_CHECK"],
//! );
//! assert!(errors.is_empty());
//!
//! // Validate a BIC
//! let errors = registry.validate_field(
//!     "AAAAGB2L",
//!     "/Document/CdtTrfTxInf/CdtrAgt/FinInstnId/BICFI",
//!     &["BIC_CHECK"],
//! );
//! assert!(errors.is_empty());
//! ```

pub mod error;
pub mod rules;
pub mod schema;
pub mod schemes;
pub mod typed;

// Convenience re-exports
pub use error::{Severity, ValidationError, ValidationResult};
pub use rules::RuleRegistry;
pub use schema::SchemaValidator;
