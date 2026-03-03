//! Validate an ISO 20022 message with built-in rules and scheme-specific checks.
//!
//! Run with: `cargo run -p mx20022 --example validate_message`

use mx20022::validate::{
    rules::RuleRegistry,
    schemes::{fednow::FedNowValidator, SchemeValidator},
};

fn main() {
    let xml = include_str!("../../../testdata/xml/pacs/pacs_008_001_13_minimal.xml");

    // -----------------------------------------------------------------------
    // Field-level rule validation
    // -----------------------------------------------------------------------
    let registry = RuleRegistry::with_defaults();

    println!("--- Field-level rule validation ---");

    let iban = "GB82WEST12345698765432";
    let iban_errors = registry.validate_field(iban, "/Document/DbtrAcct/Id/IBAN", &["IBAN_CHECK"]);
    if iban_errors.is_empty() {
        println!("  IBAN {} is valid", iban);
    } else {
        for e in &iban_errors {
            println!("  IBAN error [{}]: {}", e.rule_id, e.message);
        }
    }

    let bic = "AAAAGB2LXXX";
    let bic_errors =
        registry.validate_field(bic, "/Document/DbtrAgt/FinInstnId/BICFI", &["BIC_CHECK"]);
    if bic_errors.is_empty() {
        println!("  BIC  {} is valid", bic);
    } else {
        for e in &bic_errors {
            println!("  BIC  error [{}]: {}", e.rule_id, e.message);
        }
    }

    // -----------------------------------------------------------------------
    // Scheme-specific validation — FedNow
    // -----------------------------------------------------------------------
    let fednow = FedNowValidator::new();
    println!("\n--- FedNow scheme validation ---");

    let result = fednow.validate(xml, "pacs.008.001.13");
    if result.is_valid() {
        println!(
            "  Message is valid for FedNow ({} warning(s))",
            result.warning_count()
        );
    } else {
        println!(
            "  Message has {} error(s) and {} warning(s)",
            result.error_count(),
            result.warning_count()
        );
    }
    for e in &result.errors {
        println!(
            "  [{:?}] [{}] {} — {}",
            e.severity, e.rule_id, e.path, e.message
        );
    }

    // -----------------------------------------------------------------------
    // Validate the known-valid FedNow fixture directly
    // -----------------------------------------------------------------------
    let valid_xml = include_str!("../../../testdata/schemes/fednow/valid_pacs008.xml");
    let valid_result = fednow.validate(valid_xml, "pacs.008.001.13");
    println!("\n--- FedNow valid fixture ---");
    if valid_result.is_valid() {
        println!("  valid_pacs008.xml passes FedNow validation");
    } else {
        println!(
            "  valid_pacs008.xml has {} error(s)",
            valid_result.error_count()
        );
        for e in &valid_result.errors {
            println!("  [{:?}] [{}]: {}", e.severity, e.rule_id, e.message);
        }
    }
}
