//! Integration tests for MT ↔ MX translation.
//!
//! These tests exercise the full translation pipeline: parsing a raw MT
//! message, translating it to an MX document, and verifying key fields.
//! Roundtrip tests additionally translate back to MT and re-parse.

use mx20022_model::generated::pacs::pacs_008_001_13 as pacs008;
use mx20022_translate::mappings::charset::{is_swift_safe, to_swift_charset};
use mx20022_translate::mappings::TranslationError;
use mx20022_translate::mappings::{
    camt053_to_mt940::camt053_to_mt940, mt103_to_pacs008::mt103_to_pacs008,
    mt202_to_pacs009::mt202_to_pacs009, mt940_to_camt053::mt940_to_camt053,
    pacs008_to_mt103::pacs008_to_mt103, pacs009_to_mt202::pacs009_to_mt202,
};
use mx20022_translate::mt::fields::mt103::parse_mt103;
use mx20022_translate::mt::fields::mt202::parse_mt202;
use mx20022_translate::mt::fields::mt940::parse_mt940;
use mx20022_translate::mt::parser::parse;

// ---------------------------------------------------------------------------
// Fixture strings
// ---------------------------------------------------------------------------

const MT103_RAW: &str = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:I103BANKDEFFXXXXN}\
{3:{108:MT103REF}}\
{4:
:20:REFERENCE123
:23B:CRED
:32A:230615EUR1000,50
:50K:/DE89370400440532013000
JOHN DOE
123 MAIN STREET
:59:/GB29NWBK60161331926819
JANE SMITH
456 HIGH STREET
:71A:SHA
:70:INVOICE 12345
-}{5:{CHK:ABC12345678}}";

const MT202_RAW: &str = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:I202BANKDEFFXXXXN}\
{3:{108:MT202REF}}\
{4:
:20:TXN-REF-202-001
:21:REL-REF-001
:32A:230615USD50000,00
:58A:CHASUS33XXX
-}{5:{CHK:DEF12345678}}";

const MT940_RAW: &str = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:O9401200230615BANKBEBBAXXX00000000002306151200N}\
{3:{108:MT940REF}}\
{4:
:20:STMT-REF-001
:25:NL91ABNA0417164300
:28C:1/1
:60F:C230614EUR10000,00
:61:2306150615DN1000,50NTRFREF103-001//INSTREF001
:86:Payment to supplier
:62F:C230615EUR8999,50
-}{5:{CHK:GHI12345678}}";

// ---------------------------------------------------------------------------
// MT → MX tests
// ---------------------------------------------------------------------------

#[test]
fn test_mt103_to_pacs008() {
    let msg = parse(MT103_RAW).unwrap();
    let mt103 = parse_mt103(&msg.block4).unwrap();
    let result = mt103_to_pacs008(&mt103, "MSG001", "2023-06-15T10:00:00").unwrap();
    let doc = &result.message;

    let tx = &doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf[0];
    assert_eq!(tx.pmt_id.end_to_end_id.0, "REFERENCE123");
    assert_eq!(tx.intr_bk_sttlm_amt.value.0, "1000.50");
    assert_eq!(tx.intr_bk_sttlm_amt.ccy.0, "EUR");
    assert!(matches!(tx.chrg_br, pacs008::ChargeBearerType1Code::Shar));
    assert_eq!(tx.dbtr.nm.as_ref().map(|n| n.0.as_str()), Some("JOHN DOE"));
    assert_eq!(
        tx.cdtr.nm.as_ref().map(|n| n.0.as_str()),
        Some("JANE SMITH")
    );
}

#[test]
fn test_mt202_to_pacs009() {
    let msg = parse(MT202_RAW).unwrap();
    let mt202 = parse_mt202(&msg.block4).unwrap();
    let result = mt202_to_pacs009(&mt202, "MSG001", "2023-06-15T10:00:00").unwrap();
    let doc = &result.message;

    let tx = &doc.fi_cdt_trf.cdt_trf_tx_inf[0];
    assert_eq!(tx.pmt_id.end_to_end_id.0, "TXN-REF-202-001");
    assert_eq!(tx.intr_bk_sttlm_amt.value.0, "50000.00");
    assert_eq!(tx.intr_bk_sttlm_amt.ccy.0, "USD");
}

#[test]
fn test_mt940_to_camt053() {
    let msg = parse(MT940_RAW).unwrap();
    let mt940 = parse_mt940(&msg.block4).unwrap();
    let result = mt940_to_camt053(&mt940, "MSG001", "2023-06-15T10:00:00").unwrap();
    let doc = &result.message;

    let stmt = &doc.bk_to_cstmr_stmt.stmt[0];
    assert_eq!(stmt.id.0, "STMT-REF-001");
    assert!(stmt.bal.len() >= 2);
    assert_eq!(stmt.ntry.len(), 1);
    assert_eq!(stmt.ntry[0].amt.value.0, "1000.50");
}

// ---------------------------------------------------------------------------
// MX → MT tests
// ---------------------------------------------------------------------------

#[test]
fn test_pacs008_to_mt103() {
    let msg = parse(MT103_RAW).unwrap();
    let mt103 = parse_mt103(&msg.block4).unwrap();
    let doc = mt103_to_pacs008(&mt103, "MSG001", "2023-06-15T10:00:00")
        .unwrap()
        .message;
    let result = pacs008_to_mt103(&doc).unwrap();
    assert!(result.message.contains(":20:REFERENCE123"));
    assert!(result.message.contains(":32A:230615EUR1000,50"));
    assert!(result.message.contains(":71A:SHA"));
}

#[test]
fn test_pacs009_to_mt202() {
    let msg = parse(MT202_RAW).unwrap();
    let mt202 = parse_mt202(&msg.block4).unwrap();
    let doc = mt202_to_pacs009(&mt202, "MSG001", "2023-06-15T10:00:00")
        .unwrap()
        .message;
    let result = pacs009_to_mt202(&doc).unwrap();
    assert!(result.message.contains(":20:TXN-REF-202-001"));
    assert!(result.message.contains(":32A:230615USD50000,00"));
}

#[test]
fn test_camt053_to_mt940() {
    let msg = parse(MT940_RAW).unwrap();
    let mt940 = parse_mt940(&msg.block4).unwrap();
    let doc = mt940_to_camt053(&mt940, "MSG001", "2023-06-15T10:00:00")
        .unwrap()
        .message;
    let result = camt053_to_mt940(&doc).unwrap();
    assert!(
        result.message.contains(":20:STMT-REF-001"),
        "result: {}",
        result.message
    );
    assert!(
        result.message.contains(":25:NL91ABNA0417164300"),
        "result: {}",
        result.message
    );
}

// ---------------------------------------------------------------------------
// Roundtrip tests: MT → MX → MT
// ---------------------------------------------------------------------------

#[test]
fn test_mt103_roundtrip() {
    let msg = parse(MT103_RAW).unwrap();
    let mt103 = parse_mt103(&msg.block4).unwrap();

    // MT → pacs.008
    let doc = mt103_to_pacs008(&mt103, "MSG001", "2023-06-15T10:00:00")
        .unwrap()
        .message;

    // pacs.008 → MT text
    let result = pacs008_to_mt103(&doc).unwrap();
    let mt_text = result.message;

    // Re-parse the generated MT text
    let reparsed_msg = parse(&mt_text).unwrap();
    let reparsed = parse_mt103(&reparsed_msg.block4).unwrap();

    assert_eq!(reparsed.senders_reference, mt103.senders_reference);
    assert_eq!(
        reparsed.interbank_settled_amount,
        mt103.interbank_settled_amount
    );
    assert_eq!(reparsed.currency, mt103.currency);
    assert_eq!(reparsed.value_date, mt103.value_date);
    assert_eq!(reparsed.details_of_charges, mt103.details_of_charges);
}

#[test]
fn test_mt202_roundtrip() {
    let msg = parse(MT202_RAW).unwrap();
    let mt202 = parse_mt202(&msg.block4).unwrap();

    let doc = mt202_to_pacs009(&mt202, "MSG001", "2023-06-15T10:00:00")
        .unwrap()
        .message;

    let result = pacs009_to_mt202(&doc).unwrap();
    let mt_text = result.message;

    let reparsed_msg = parse(&mt_text).unwrap();
    let reparsed = parse_mt202(&reparsed_msg.block4).unwrap();

    assert_eq!(reparsed.transaction_reference, mt202.transaction_reference);
    assert_eq!(reparsed.amount, mt202.amount);
    assert_eq!(reparsed.currency, mt202.currency);
    assert_eq!(reparsed.value_date, mt202.value_date);
}

#[test]
fn test_mt940_roundtrip() {
    let msg = parse(MT940_RAW).unwrap();
    let mt940 = parse_mt940(&msg.block4).unwrap();

    let doc = mt940_to_camt053(&mt940, "MSG001", "2023-06-15T10:00:00")
        .unwrap()
        .message;

    let result = camt053_to_mt940(&doc).unwrap();
    let mt_text = result.message;

    // The roundtripped MT940 should contain the transaction reference and account ID.
    assert!(
        mt_text.contains(":20:STMT-REF-001"),
        "roundtrip: {}",
        mt_text
    );
    assert!(
        mt_text.contains(":25:NL91ABNA0417164300"),
        "roundtrip: {}",
        mt_text
    );
}

// ---------------------------------------------------------------------------
// MX → MT roundtrip (re-parse the generated MT)
// ---------------------------------------------------------------------------

#[test]
fn test_pacs008_to_mt103_reparseable() {
    let msg = parse(MT103_RAW).unwrap();
    let mt103 = parse_mt103(&msg.block4).unwrap();
    let doc = mt103_to_pacs008(&mt103, "MSG001", "2023-06-15T10:00:00")
        .unwrap()
        .message;
    let result = pacs008_to_mt103(&doc).unwrap();

    // The MT text must be re-parseable.
    let reparsed = parse(&result.message).expect("generated MT103 text must be parseable");
    let reparsed_mt = parse_mt103(&reparsed.block4).expect("generated block4 must parse as MT103");

    assert_eq!(reparsed_mt.senders_reference, "REFERENCE123");
}

// ---------------------------------------------------------------------------
// Character set tests
// ---------------------------------------------------------------------------

#[test]
fn test_swift_charset_basic() {
    assert!(is_swift_safe('A'));
    assert!(is_swift_safe('z'));
    assert!(is_swift_safe('0'));
    assert!(is_swift_safe('/'));
    assert!(is_swift_safe('-'));
    assert!(is_swift_safe(' '));
}

#[test]
fn test_swift_charset_replacement() {
    let (s, replaced) = to_swift_charset("Müller & Co.");
    assert!(replaced, "umlaut should have been replaced");
    assert!(!s.contains('ü'), "ü should not appear in output");
    assert!(
        s.chars().all(|c| is_swift_safe(c) || c == '&'),
        "output should be SWIFT-safe (& is not safe but we test the letter)"
    );
}

#[test]
fn test_swift_charset_pure_ascii() {
    let (s, replaced) = to_swift_charset("HELLO WORLD/123");
    assert_eq!(s, "HELLO WORLD/123");
    assert!(!replaced);
}

// ---------------------------------------------------------------------------
// Warning tests
// ---------------------------------------------------------------------------

#[test]
fn test_unmappable_field_warning() {
    // MT103 with :72: sender-to-receiver info produces a warning in pacs.008
    let raw = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:I103BANKDEFFXXXXN}\
{3:}\
{4:
:20:REF001
:23B:CRED
:32A:230615EUR1000,00
:50K:ACME CORP
:59:BENEFICIARY
:71A:SHA
:72:/ACC/EXTRA INFO
-}";
    let msg = parse(raw).unwrap();
    let mt103 = parse_mt103(&msg.block4).unwrap();
    let result = mt103_to_pacs008(&mt103, "MSG", "2023-06-15T10:00:00").unwrap();
    assert!(
        !result.warnings.is_empty(),
        "expected warnings for :72: field"
    );
}

#[test]
fn test_multiple_tx_warning_pacs008() {
    // Build a document with two CdtTrfTxInf entries
    let msg = parse(MT103_RAW).unwrap();
    let mt103 = parse_mt103(&msg.block4).unwrap();
    let mut doc = mt103_to_pacs008(&mt103, "MSG001", "2023-06-15T10:00:00")
        .unwrap()
        .message;

    // Duplicate the transaction
    let tx_clone = doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf[0].clone();
    doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf.push(tx_clone);

    let result = pacs008_to_mt103(&doc).unwrap();
    assert!(
        !result.warnings.is_empty(),
        "expected warning for multiple transactions"
    );
}

// ---------------------------------------------------------------------------
// Error tests
// ---------------------------------------------------------------------------

#[test]
fn test_missing_required_fields_pacs008() {
    // An empty pacs.008 document with no transactions
    let doc = pacs008::Document {
        fi_to_fi_cstmr_cdt_trf: pacs008::FIToFICustomerCreditTransferV13 {
            grp_hdr: pacs008::GroupHeader131::builder()
                .msg_id(pacs008::Max35Text("X".into()))
                .cre_dt_tm(pacs008::ISODateTime("2023-01-01T00:00:00".into()))
                .nb_of_txs(pacs008::Max15NumericText("0".into()))
                .sttlm_inf(pacs008::SettlementInstruction15 {
                    sttlm_mtd: pacs008::SettlementMethod1Code::Inda,
                    sttlm_acct: None,
                    clr_sys: None,
                    instg_rmbrsmnt_agt: None,
                    instg_rmbrsmnt_agt_acct: None,
                    instd_rmbrsmnt_agt: None,
                    instd_rmbrsmnt_agt_acct: None,
                    thrd_rmbrsmnt_agt: None,
                    thrd_rmbrsmnt_agt_acct: None,
                })
                .build()
                .unwrap(),
            cdt_trf_tx_inf: vec![],
            splmtry_data: vec![],
        },
    };

    let err = pacs008_to_mt103(&doc).unwrap_err();
    assert!(
        matches!(err, TranslationError::MissingField { .. }),
        "expected MissingField error, got: {err}"
    );
}
