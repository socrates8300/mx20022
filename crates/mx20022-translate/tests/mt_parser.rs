//! Integration tests for the SWIFT MT message parser.

use mx20022_translate::mt::fields::common::{
    parse_account, parse_amount, parse_date_yymmdd, parse_party_lines,
};
use mx20022_translate::mt::fields::mt103::parse_mt103;
use mx20022_translate::mt::fields::mt202::parse_mt202;
use mx20022_translate::mt::fields::mt940::parse_mt940;
use mx20022_translate::mt::types::{Block2, Block4, TagField};
use mx20022_translate::mt::MtError;
use mx20022_translate::mt::{self, MtMessage};
use pretty_assertions::assert_eq;

// ---------------------------------------------------------------------------
// Fixture helpers
// ---------------------------------------------------------------------------

fn read_fixture(name: &str) -> String {
    let path = format!("{}/../../testdata/mt/{}", env!("CARGO_MANIFEST_DIR"), name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read fixture '{path}': {e}"))
}

fn parse_fixture(name: &str) -> MtMessage {
    let raw = read_fixture(name);
    mt::parse(&raw).unwrap_or_else(|e| panic!("failed to parse fixture '{name}': {e}"))
}

// ---------------------------------------------------------------------------
// Block-level parsing — MT103
// ---------------------------------------------------------------------------

#[test]
fn test_parse_mt103_blocks_present() {
    let msg = parse_fixture("mt103.txt");
    assert!(msg.block1.is_some(), "block1 should be present");
    assert!(msg.block2.is_some(), "block2 should be present");
    assert!(msg.block3.is_some(), "block3 should be present");
    assert!(msg.block5.is_some(), "block5 should be present");
}

#[test]
fn test_parse_mt103_block1() {
    let msg = parse_fixture("mt103.txt");
    let b1 = msg.block1.unwrap();
    assert_eq!(b1.app_id, 'F');
    assert_eq!(b1.service_id, "01");
    assert_eq!(b1.lt_address, "BANKBEBBAXXX");
    assert_eq!(b1.session_number, "0000");
    assert_eq!(b1.sequence_number, "000000");
}

#[test]
fn test_parse_mt103_block2_input() {
    let msg = parse_fixture("mt103.txt");
    let Block2::Input(b2) = msg.block2.unwrap() else {
        panic!("expected Input variant for MT103");
    };
    assert_eq!(b2.message_type, "103");
    assert_eq!(b2.destination, "BANKDEFFXXXX");
    assert_eq!(b2.priority, Some('N'));
}

#[test]
fn test_parse_mt202_block2_input() {
    let msg = parse_fixture("mt202.txt");
    let Block2::Input(b2) = msg.block2.unwrap() else {
        panic!("expected Input variant for MT202");
    };
    assert_eq!(b2.message_type, "202");
    assert_eq!(b2.priority, Some('N'));
}

#[test]
fn test_parse_mt940_block2_output() {
    let msg = parse_fixture("mt940.txt");
    let Block2::Output(b2) = msg.block2.unwrap() else {
        panic!("expected Output variant for MT940");
    };
    assert_eq!(b2.message_type, "940");
    assert_eq!(b2.input_time, "1200");
    assert_eq!(b2.output_time, "1200");
    assert_eq!(b2.priority, Some('N'));
    // MIR starts with the input date.
    assert!(
        b2.mir.starts_with("230615"),
        "MIR should start with input date 230615"
    );
}

#[test]
fn test_parse_block3_tags() {
    let msg = parse_fixture("mt103.txt");
    let b3 = msg.block3.unwrap();
    assert_eq!(b3.get("108"), Some("MT103REF"));
    // The fixture also has tag 121 (UUID).
    assert!(b3.get("121").is_some());
}

#[test]
fn test_parse_block4_fields_count() {
    let msg = parse_fixture("mt103.txt");
    // :20: :23B: :32A: :50K: :59: :71A: = 6 distinct tag entries
    // :50K: spans multiple lines but is a single field.
    assert!(msg.block4.fields.len() >= 6);
}

#[test]
fn test_parse_block4_multiline_field() {
    let msg = parse_fixture("mt103.txt");
    let f50k = msg.block4.get("50K").expect(":50K: must be present");
    // Should contain the account line AND name line AND address lines.
    assert!(
        f50k.value.contains("JOHN DOE"),
        ":50K: must include name continuation"
    );
    assert!(
        f50k.value.contains("123 MAIN STREET"),
        ":50K: must include address"
    );
}

#[test]
fn test_parse_block5_tags() {
    let msg = parse_fixture("mt103.txt");
    let b5 = msg.block5.unwrap();
    assert_eq!(b5.get("CHK"), Some("ABC12345678"));
}

// ---------------------------------------------------------------------------
// Message type detection
// ---------------------------------------------------------------------------

#[test]
fn test_message_type_mt103() {
    let msg = parse_fixture("mt103.txt");
    assert_eq!(msg.message_type(), Some("103"));
}

#[test]
fn test_message_type_mt202() {
    let msg = parse_fixture("mt202.txt");
    assert_eq!(msg.message_type(), Some("202"));
}

#[test]
fn test_message_type_mt940() {
    let msg = parse_fixture("mt940.txt");
    assert_eq!(msg.message_type(), Some("940"));
}

#[test]
fn test_message_type_mt103_output() {
    let msg = parse_fixture("mt103_output.txt");
    assert_eq!(msg.message_type(), Some("103"));
}

// ---------------------------------------------------------------------------
// Field-level parsing — MT103
// ---------------------------------------------------------------------------

#[test]
fn test_mt103_fields() {
    let msg = parse_fixture("mt103.txt");
    let mt = parse_mt103(&msg.block4).unwrap();

    assert_eq!(mt.senders_reference, "REFERENCE123");
    assert_eq!(mt.bank_operation_code, "CRED");
    assert_eq!(mt.value_date, "2023-06-15");
    assert_eq!(mt.currency, "EUR");
    assert_eq!(mt.interbank_settled_amount, "1000.50");
    assert_eq!(mt.details_of_charges, "SHA");

    // Ordering customer
    let oc = &mt.ordering_customer;
    assert_eq!(
        oc.account.as_ref().and_then(|a| a.iban.as_deref()),
        Some("DE89370400440532013000")
    );
    assert_eq!(oc.name.as_deref(), Some("JOHN DOE"));
    assert!(oc.address_lines.contains(&"123 MAIN STREET".to_string()));

    // Beneficiary
    let ben = &mt.beneficiary;
    assert_eq!(
        ben.account.as_ref().and_then(|a| a.iban.as_deref()),
        Some("GB29NWBK60161331926819")
    );
    assert_eq!(ben.name.as_deref(), Some("JANE SMITH"));
}

#[test]
fn test_mt103_output_variant() {
    let msg = parse_fixture("mt103_output.txt");
    // Block2 should be Output variant.
    assert!(matches!(msg.block2, Some(Block2::Output(_))));

    let mt = parse_mt103(&msg.block4).unwrap();
    assert_eq!(mt.senders_reference, "OUTREF-001");
    assert_eq!(mt.currency, "USD");
    assert_eq!(mt.interbank_settled_amount, "25000.00");
    assert_eq!(mt.details_of_charges, "OUR");

    // :71F: sender charges
    assert_eq!(mt.senders_charges.len(), 1);
    assert_eq!(mt.senders_charges[0].value, "10.00");

    // :72: sender to receiver info
    assert_eq!(
        mt.sender_to_receiver_info.as_deref(),
        Some("/REC/PAYMENT FOR INVOICE 12345")
    );
}

// ---------------------------------------------------------------------------
// Field-level parsing — MT202
// ---------------------------------------------------------------------------

#[test]
fn test_mt202_fields() {
    let msg = parse_fixture("mt202.txt");
    let mt = parse_mt202(&msg.block4).unwrap();

    assert_eq!(mt.transaction_reference, "TXN-REF-202-001");
    assert_eq!(mt.related_reference, "REL-REF-001");
    assert_eq!(mt.value_date, "2023-06-15");
    assert_eq!(mt.currency, "USD");
    assert_eq!(mt.amount, "50000.00");

    // :58A: stored as raw name (no leading '/')
    assert_eq!(
        mt.beneficiary_institution.name.as_deref(),
        Some("CHASUS33XXX")
    );
}

// ---------------------------------------------------------------------------
// Field-level parsing — MT940
// ---------------------------------------------------------------------------

#[test]
fn test_mt940_fields() {
    let msg = parse_fixture("mt940.txt");
    let mt = parse_mt940(&msg.block4).unwrap();

    assert_eq!(mt.transaction_reference, "STMT-REF-001");
    assert_eq!(mt.account_id, "NL91ABNA0417164300");
    assert_eq!(mt.statement_number, "1/1");
}

#[test]
fn test_mt940_balances() {
    let msg = parse_fixture("mt940.txt");
    let mt = parse_mt940(&msg.block4).unwrap();

    let ob = &mt.opening_balance;
    assert_eq!(ob.dc_indicator, 'C');
    assert_eq!(ob.date, "2023-06-14");
    assert_eq!(ob.currency, "EUR");
    assert_eq!(ob.amount, "10000.00");
    assert_eq!(ob.balance_type, 'F');

    let cb = &mt.closing_balance;
    assert_eq!(cb.dc_indicator, 'C');
    assert_eq!(cb.date, "2023-06-15");
    assert_eq!(cb.amount, "11499.50");

    // Closing available
    let ca = mt.closing_available.as_ref().unwrap();
    assert_eq!(ca.amount, "11499.50");
}

#[test]
fn test_mt940_statement_lines() {
    let msg = parse_fixture("mt940.txt");
    let mt = parse_mt940(&msg.block4).unwrap();

    assert_eq!(mt.statement_lines.len(), 2);

    let sl0 = &mt.statement_lines[0];
    assert_eq!(sl0.value_date, "2023-06-15");
    assert_eq!(sl0.dc_mark, "D");
    assert_eq!(sl0.amount, "1000.50");
    assert_eq!(sl0.transaction_type, "NTRF");
    assert_eq!(sl0.reference, "REF103-001");
    assert_eq!(sl0.institution_reference.as_deref(), Some("INSTREF001"));
    assert_eq!(sl0.information.as_deref(), Some("Payment to supplier"));

    let sl1 = &mt.statement_lines[1];
    assert_eq!(sl1.dc_mark, "C");
    assert_eq!(sl1.amount, "2500.00");
    assert_eq!(sl1.transaction_type, "NMSC");
    assert_eq!(sl1.reference, "REF940-001");
    assert_eq!(sl1.information.as_deref(), Some("Incoming transfer"));
}

// ---------------------------------------------------------------------------
// Common field utilities
// ---------------------------------------------------------------------------

#[test]
fn test_parse_amount_eur() {
    let a = parse_amount("EUR1000,50").unwrap();
    assert_eq!(a.currency, "EUR");
    assert_eq!(a.value, "1000.50");
}

#[test]
fn test_parse_amount_usd_whole() {
    let a = parse_amount("USD50000,00").unwrap();
    assert_eq!(a.currency, "USD");
    assert_eq!(a.value, "50000.00");
}

#[test]
fn test_parse_amount_invalid() {
    assert!(parse_amount("EU").is_err());
    assert!(parse_amount("").is_err());
}

#[test]
fn test_parse_date_century_20() {
    assert_eq!(parse_date_yymmdd("230615").unwrap(), "2023-06-15");
}

#[test]
fn test_parse_date_century_19() {
    assert_eq!(parse_date_yymmdd("991231").unwrap(), "1999-12-31");
}

#[test]
fn test_parse_date_invalid() {
    assert!(parse_date_yymmdd("23061X").is_err());
    assert!(parse_date_yymmdd("2306").is_err());
}

#[test]
fn test_parse_account_iban() {
    let a = parse_account("/GB29NWBK60161331926819").unwrap();
    assert_eq!(a.iban.as_deref(), Some("GB29NWBK60161331926819"));
    assert!(a.bic.is_none());
    assert!(a.account.is_none());
}

#[test]
fn test_parse_account_bic_account() {
    let a = parse_account("//CHASUS33/123456789").unwrap();
    assert_eq!(a.bic.as_deref(), Some("CHASUS33"));
    assert_eq!(a.account.as_deref(), Some("123456789"));
}

#[test]
fn test_parse_account_raw() {
    let a = parse_account("PLAINACCT123").unwrap();
    assert_eq!(a.account.as_deref(), Some("PLAINACCT123"));
    assert!(a.iban.is_none());
    assert!(a.bic.is_none());
}

#[test]
fn test_parse_party_lines_full() {
    let lines = vec![
        "/DE89370400440532013000",
        "JOHN DOE",
        "123 MAIN STREET",
        "BERLIN",
    ];
    let p = parse_party_lines(&lines);
    assert_eq!(
        p.account.as_ref().and_then(|a| a.iban.as_deref()),
        Some("DE89370400440532013000")
    );
    assert_eq!(p.name.as_deref(), Some("JOHN DOE"));
    assert_eq!(p.address_lines, vec!["123 MAIN STREET", "BERLIN"]);
}

#[test]
fn test_parse_party_lines_name_only() {
    let lines = vec!["ACME CORP"];
    let p = parse_party_lines(&lines);
    assert!(p.account.is_none());
    assert_eq!(p.name.as_deref(), Some("ACME CORP"));
    assert!(p.address_lines.is_empty());
}

// ---------------------------------------------------------------------------
// Error cases
// ---------------------------------------------------------------------------

#[test]
fn test_empty_input() {
    let err = mt::parse("").unwrap_err();
    assert!(matches!(err, MtError::MissingBlock(4)));
}

#[test]
fn test_missing_block4() {
    let raw = "{1:F01BANKBEBBAXXX0000000000}{2:I103BANKDEFFXXXXN}";
    let err = mt::parse(raw).unwrap_err();
    assert!(matches!(err, MtError::MissingBlock(4)));
}

#[test]
fn test_invalid_block_structure_no_brace() {
    let raw = "NOTABLOCK";
    let err = mt::parse(raw).unwrap_err();
    assert!(matches!(err, MtError::InvalidBlockStructure(_)));
}

#[test]
fn test_invalid_block1_too_short() {
    // Only 5 chars after "1:" — far too short for a valid block 1.
    let raw = "{1:F0100}{4:\n:20:REF\n:23B:CRED\n-}";
    let err = mt::parse(raw).unwrap_err();
    assert!(matches!(err, MtError::InvalidBlockContent { block: 1, .. }));
}

#[test]
fn test_mt103_missing_required_field() {
    // Missing :71A:
    let raw = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:I103BANKDEFFXXXXN}\
{3:}\
{4:
:20:REF
:23B:CRED
:32A:230615EUR100,00
:50K:ACME
:59:SMITH
-}";
    let msg = mt::parse(raw).unwrap();
    let err = parse_mt103(&msg.block4).unwrap_err();
    assert!(
        matches!(&err, MtError::MissingField { tag, message_type } if tag == "71A" && message_type == "103"),
        "unexpected error: {err}"
    );
}

#[test]
fn test_mt202_missing_21() {
    let raw = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:I202BANKDEFFXXXXN}\
{3:}\
{4:
:20:REF
:32A:230615USD1000,00
:58A:CHASUS33
-}";
    let msg = mt::parse(raw).unwrap();
    let err = parse_mt202(&msg.block4).unwrap_err();
    assert!(matches!(&err, MtError::MissingField { tag, .. } if tag == "21"));
}

#[test]
fn test_mt940_missing_account() {
    let raw = "\
{1:F01BANKBEBBAXXX0000000000}\
{2:O9401200230615BANKBEBBAXXX00000000002306151200N}\
{3:}\
{4:
:20:REF
:28C:1/1
:60F:C230614EUR1000,00
:62F:C230614EUR1000,00
-}";
    let msg = mt::parse(raw).unwrap();
    let err = parse_mt940(&msg.block4).unwrap_err();
    assert!(matches!(&err, MtError::MissingField { tag, .. } if tag == "25"));
}

// ---------------------------------------------------------------------------
// Block4 API tests
// ---------------------------------------------------------------------------

#[test]
fn test_block4_get_all_repeatable() {
    // Build a synthetic block with two :71F: fields.
    let block4 = Block4 {
        fields: vec![
            TagField {
                tag: "20".into(),
                value: "REF".into(),
            },
            TagField {
                tag: "71F".into(),
                value: "USD5,00".into(),
            },
            TagField {
                tag: "71F".into(),
                value: "EUR3,00".into(),
            },
        ],
    };
    let all_71f = block4.get_all("71F");
    assert_eq!(all_71f.len(), 2);
    assert_eq!(all_71f[0].value, "USD5,00");
    assert_eq!(all_71f[1].value, "EUR3,00");
}

#[test]
fn test_block4_get_returns_first() {
    let block4 = Block4 {
        fields: vec![
            TagField {
                tag: "86".into(),
                value: "first".into(),
            },
            TagField {
                tag: "86".into(),
                value: "second".into(),
            },
        ],
    };
    assert_eq!(block4.get("86").unwrap().value, "first");
}
