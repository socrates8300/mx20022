//! Integration test: Parse → Validate pipeline.
//!
//! Deserializes a `pacs.008.001.13` XML fixture using `mx20022_parse`, then
//! extracts fields from the strongly-typed struct and validates them with the
//! `mx20022_validate` rule registry.

use mx20022_model::generated::pacs::pacs_008_001_13::{AccountIdentification4Choice, Document};
use mx20022_parse::de::from_str;
use mx20022_validate::rules::RuleRegistry;

/// Absolute path to the shared testdata directory.
///
/// The integration test binary is run from the workspace root, but
/// `env!("CARGO_MANIFEST_DIR")` resolves to the crate directory, so we
/// navigate up two levels to reach the workspace root.
fn testdata_path(relative: &str) -> std::path::PathBuf {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let workspace_root = std::path::Path::new(manifest)
        .parent() // crates/
        .and_then(|p| p.parent()) // workspace root
        .expect("Could not determine workspace root from CARGO_MANIFEST_DIR");
    workspace_root.join("testdata").join(relative)
}

/// Deserialize the minimal `pacs.008.001.13` XML fixture into a typed `Document`.
fn load_pacs008_minimal() -> Document {
    let path = testdata_path("xml/pacs/pacs_008_001_13_minimal.xml");
    let xml = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Could not read {}: {e}", path.display()));
    from_str::<Document>(&xml)
        .unwrap_or_else(|e| panic!("Could not deserialise pacs.008 fixture: {e}"))
}

// ---------------------------------------------------------------------------
// Valid-scenario tests
// ---------------------------------------------------------------------------

#[test]
fn pacs008_bicfi_values_are_valid() {
    let doc = load_pacs008_minimal();
    let registry = RuleRegistry::with_defaults();
    let txns = &doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf;

    assert!(
        !txns.is_empty(),
        "Fixture must have at least one transaction"
    );

    for (i, txn) in txns.iter().enumerate() {
        let path_dbtr = format!("//CdtTrfTxInf[{}]/DbtrAgt/FinInstnId/BICFI", i + 1);
        let path_cdtr = format!("//CdtTrfTxInf[{}]/CdtrAgt/FinInstnId/BICFI", i + 1);

        if let Some(bic) = &txn.dbtr_agt.fin_instn_id.bicfi {
            let errors = registry.validate_field(&bic.0, &path_dbtr, &["BIC_CHECK"]);
            assert!(
                errors.is_empty(),
                "DbtrAgt BIC `{}` failed: {errors:?}",
                bic.0
            );
        }

        if let Some(bic) = &txn.cdtr_agt.fin_instn_id.bicfi {
            let errors = registry.validate_field(&bic.0, &path_cdtr, &["BIC_CHECK"]);
            assert!(
                errors.is_empty(),
                "CdtrAgt BIC `{}` failed: {errors:?}",
                bic.0
            );
        }
    }
}

#[test]
fn pacs008_iban_values_are_valid() {
    let doc = load_pacs008_minimal();
    let registry = RuleRegistry::with_defaults();
    let txns = &doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf;

    for (i, txn) in txns.iter().enumerate() {
        // Debtor account IBAN
        if let Some(acct) = &txn.dbtr_acct {
            if let Some(id_wrapper) = &acct.id {
                if let AccountIdentification4Choice::IBAN(iban) = &id_wrapper.inner {
                    let path = format!("//CdtTrfTxInf[{}]/DbtrAcct/Id/IBAN", i + 1);
                    let errors = registry.validate_field(&iban.0, &path, &["IBAN_CHECK"]);
                    assert!(
                        errors.is_empty(),
                        "DbtrAcct IBAN `{}` failed: {errors:?}",
                        iban.0
                    );
                }
            }
        }

        // Creditor account IBAN
        if let Some(acct) = &txn.cdtr_acct {
            if let Some(id_wrapper) = &acct.id {
                if let AccountIdentification4Choice::IBAN(iban) = &id_wrapper.inner {
                    let path = format!("//CdtTrfTxInf[{}]/CdtrAcct/Id/IBAN", i + 1);
                    let errors = registry.validate_field(&iban.0, &path, &["IBAN_CHECK"]);
                    assert!(
                        errors.is_empty(),
                        "CdtrAcct IBAN `{}` failed: {errors:?}",
                        iban.0
                    );
                }
            }
        }
    }
}

#[test]
fn pacs008_currency_is_valid() {
    let doc = load_pacs008_minimal();
    let registry = RuleRegistry::with_defaults();
    let txns = &doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf;

    for (i, txn) in txns.iter().enumerate() {
        let ccy = &txn.intr_bk_sttlm_amt.ccy.0;
        let path = format!("//CdtTrfTxInf[{}]/IntrBkSttlmAmt/@Ccy", i + 1);
        let errors = registry.validate_field(ccy, &path, &["CURRENCY_CHECK"]);
        assert!(
            errors.is_empty(),
            "IntrBkSttlmAmt currency `{ccy}` failed: {errors:?}"
        );
    }
}

#[test]
fn pacs008_interbank_amount_is_valid() {
    let doc = load_pacs008_minimal();
    let registry = RuleRegistry::with_defaults();
    let txns = &doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf;

    for (i, txn) in txns.iter().enumerate() {
        let amt = &txn.intr_bk_sttlm_amt.value.0;
        let path = format!("//CdtTrfTxInf[{}]/IntrBkSttlmAmt", i + 1);
        let errors = registry.validate_field(amt, &path, &["AMOUNT_FORMAT"]);
        assert!(
            errors.is_empty(),
            "IntrBkSttlmAmt `{amt}` failed: {errors:?}"
        );
    }
}

#[test]
fn pacs008_grphdr_cre_dt_tm_is_valid() {
    let doc = load_pacs008_minimal();
    let registry = RuleRegistry::with_defaults();
    let cre_dt_tm = &doc.fi_to_fi_cstmr_cdt_trf.grp_hdr.cre_dt_tm.0;
    let errors = registry.validate_field(cre_dt_tm, "//GrpHdr/CreDtTm", &["DATETIME_CHECK"]);
    assert!(
        errors.is_empty(),
        "GrpHdr/CreDtTm `{cre_dt_tm}` failed: {errors:?}"
    );
}

#[test]
fn pacs008_settlement_date_is_valid() {
    let doc = load_pacs008_minimal();
    let registry = RuleRegistry::with_defaults();
    let txns = &doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf;

    for (i, txn) in txns.iter().enumerate() {
        if let Some(dt) = &txn.intr_bk_sttlm_dt {
            let path = format!("//CdtTrfTxInf[{}]/IntrBkSttlmDt", i + 1);
            let errors = registry.validate_field(&dt.0, &path, &["DATE_CHECK"]);
            assert!(
                errors.is_empty(),
                "IntrBkSttlmDt `{}` failed: {errors:?}",
                dt.0
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Invalid-scenario tests — validators catch bad values
// ---------------------------------------------------------------------------

#[test]
fn invalid_currency_code_is_caught() {
    let registry = RuleRegistry::with_defaults();
    // "XYZ" is not an ISO 4217 code
    let errors = registry.validate_field("XYZ", "//IntrBkSttlmAmt/@Ccy", &["CURRENCY_CHECK"]);
    assert!(
        !errors.is_empty(),
        "Fictional currency XYZ must be rejected"
    );
    assert_eq!(errors[0].rule_id, "CURRENCY_CHECK");
}

#[test]
fn invalid_iban_is_caught() {
    let registry = RuleRegistry::with_defaults();
    let errors = registry.validate_field(
        "GB82WEST1234569876543X",
        "//DbtrAcct/Id/IBAN",
        &["IBAN_CHECK"],
    );
    assert!(!errors.is_empty(), "Malformed IBAN must be rejected");
    assert_eq!(errors[0].rule_id, "IBAN_CHECK");
}

#[test]
fn invalid_bic_is_caught() {
    let registry = RuleRegistry::with_defaults();
    let errors = registry.validate_field("BADFORMAT", "//BICFI", &["BIC_CHECK"]);
    assert!(!errors.is_empty(), "Malformed BIC must be rejected");
    assert_eq!(errors[0].rule_id, "BIC_CHECK");
}

#[test]
fn invalid_lei_is_caught() {
    let registry = RuleRegistry::with_defaults();
    // Wrong length — must be exactly 20 chars
    let errors = registry.validate_field("TOOSHORT", "//LEI", &["LEI_CHECK"]);
    assert!(!errors.is_empty(), "Wrong-length LEI must be rejected");
    assert_eq!(errors[0].rule_id, "LEI_CHECK");
}

#[test]
fn valid_lei_passes() {
    let registry = RuleRegistry::with_defaults();
    // 5493001KJTIIGC8Y1R12 — verified public LEI, mod-97 == 1
    let errors = registry.validate_field("5493001KJTIIGC8Y1R12", "//LEI", &["LEI_CHECK"]);
    assert!(errors.is_empty(), "Valid LEI must pass: {errors:?}");
}

#[test]
fn negative_amount_is_caught() {
    let registry = RuleRegistry::with_defaults();
    let errors = registry.validate_field("-100.00", "//IntrBkSttlmAmt", &["AMOUNT_FORMAT"]);
    assert!(!errors.is_empty(), "Negative amount must be rejected");
    assert_eq!(errors[0].rule_id, "AMOUNT_FORMAT");
}

#[test]
fn invalid_datetime_is_caught() {
    let registry = RuleRegistry::with_defaults();
    let errors = registry.validate_field(
        "2024-13-01T00:00:00Z",
        "//GrpHdr/CreDtTm",
        &["DATETIME_CHECK"],
    );
    assert!(!errors.is_empty(), "Month 13 must be rejected");
    assert_eq!(errors[0].rule_id, "DATETIME_CHECK");
}

#[test]
fn invalid_date_is_caught() {
    let registry = RuleRegistry::with_defaults();
    let errors = registry.validate_field("2024-00-01", "//IntrBkSttlmDt", &["DATE_CHECK"]);
    assert!(!errors.is_empty(), "Month 00 must be rejected");
    assert_eq!(errors[0].rule_id, "DATE_CHECK");
}

#[test]
fn invalid_country_code_is_caught() {
    let registry = RuleRegistry::with_defaults();
    let errors = registry.validate_field("XX", "//Ctry", &["COUNTRY_CHECK"]);
    assert!(
        !errors.is_empty(),
        "Unassigned country code must be rejected"
    );
    assert_eq!(errors[0].rule_id, "COUNTRY_CHECK");
}
