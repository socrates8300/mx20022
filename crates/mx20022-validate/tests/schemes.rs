//! Integration tests for scheme-specific validators.
//!
//! Fixtures live under `testdata/schemes/{scheme}/` relative to the workspace
//! root.  `CARGO_MANIFEST_DIR` points at the crate root
//! (`crates/mx20022-validate/`), so we navigate up two levels.

use mx20022_validate::schemes::{
    cbpr::CbprPlusValidator, fednow::FedNowValidator, sepa::SepaValidator, SchemeValidator,
};
use mx20022_validate::Severity;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn read_fixture(relative: &str) -> String {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let path = format!("{manifest}/../../testdata/schemes/{relative}");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Cannot read fixture `{path}`: {e}"))
}

fn has_error_with_rule(result: &mx20022_validate::ValidationResult, rule_id: &str) -> bool {
    result
        .errors
        .iter()
        .any(|e| e.rule_id == rule_id && e.severity == Severity::Error)
}

fn has_warning_with_rule(result: &mx20022_validate::ValidationResult, rule_id: &str) -> bool {
    result
        .errors
        .iter()
        .any(|e| e.rule_id == rule_id && e.severity == Severity::Warning)
}

// ---------------------------------------------------------------------------
// FedNow tests
// ---------------------------------------------------------------------------

#[test]
fn fednow_valid_pacs008() {
    let xml = read_fixture("fednow/valid_pacs008.xml");
    let v = FedNowValidator::new();
    let result = v.validate(&xml, "pacs.008.001.13");
    assert!(
        result.is_valid(),
        "Expected valid FedNow pacs.008 to pass; errors: {:?}",
        result.errors
    );
}

#[test]
fn fednow_invalid_currency() {
    let xml = read_fixture("fednow/invalid_eur.xml");
    let v = FedNowValidator::new();
    let result = v.validate(&xml, "pacs.008.001.13");
    assert!(
        has_error_with_rule(&result, "FEDNOW_CURRENCY"),
        "Expected FEDNOW_CURRENCY error for EUR currency; got: {:?}",
        result.errors
    );
}

#[test]
fn fednow_over_amount_limit() {
    let xml = read_fixture("fednow/over_limit.xml");
    let v = FedNowValidator::new();
    let result = v.validate(&xml, "pacs.008.001.13");
    assert!(
        has_error_with_rule(&result, "FEDNOW_AMOUNT_LIMIT"),
        "Expected FEDNOW_AMOUNT_LIMIT error for 750,000 USD; got: {:?}",
        result.errors
    );
}

#[test]
fn fednow_custom_amount_limit_passes() {
    // With a 25M limit, 750,000 should be valid (amount-wise).
    let xml = read_fixture("fednow/over_limit.xml");
    let v = FedNowValidator::with_max_amount(25_000_000.0);
    let result = v.validate(&xml, "pacs.008.001.13");
    assert!(
        !has_error_with_rule(&result, "FEDNOW_AMOUNT_LIMIT"),
        "Expected no FEDNOW_AMOUNT_LIMIT error with 25M limit; got: {:?}",
        result.errors
    );
}

#[test]
fn fednow_multi_transaction() {
    // Inline XML: NbOfTxs = "3" should fail.
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13">
  <FIToFICstmrCdtTrf>
    <GrpHdr>
      <MsgId>MULTI-TX-001</MsgId>
      <CreDtTm>2024-01-01T12:00:00Z</CreDtTm>
      <NbOfTxs>3</NbOfTxs>
      <SttlmInf><SttlmMtd>CLRG</SttlmMtd></SttlmInf>
    </GrpHdr>
    <CdtTrfTxInf>
      <PmtId>
        <EndToEndId>E2E-001</EndToEndId>
        <UETR>97ed4827-7b6f-4491-a06f-b548d5a7512d</UETR>
      </PmtId>
      <IntrBkSttlmAmt Ccy="USD">100.00</IntrBkSttlmAmt>
      <IntrBkSttlmDt>2024-01-01</IntrBkSttlmDt>
      <ChrgBr>SLEV</ChrgBr>
      <Dbtr><Nm>Alice</Nm></Dbtr>
      <DbtrAgt><FinInstnId><BICFI>AAAAGB2LXXX</BICFI></FinInstnId></DbtrAgt>
      <CdtrAgt><FinInstnId><BICFI>BBBBUS33XXX</BICFI></FinInstnId></CdtrAgt>
      <Cdtr><Nm>Bob</Nm></Cdtr>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#;
    let v = FedNowValidator::new();
    let result = v.validate(xml, "pacs.008.001.13");
    assert!(
        has_error_with_rule(&result, "FEDNOW_SINGLE_TX"),
        "Expected FEDNOW_SINGLE_TX error for NbOfTxs=3; got: {:?}",
        result.errors
    );
}

#[test]
fn fednow_missing_uetr() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13">
  <FIToFICstmrCdtTrf>
    <GrpHdr>
      <MsgId>NO-UETR-001</MsgId>
      <CreDtTm>2024-01-01T12:00:00Z</CreDtTm>
      <NbOfTxs>1</NbOfTxs>
      <SttlmInf><SttlmMtd>CLRG</SttlmMtd></SttlmInf>
    </GrpHdr>
    <CdtTrfTxInf>
      <PmtId>
        <EndToEndId>E2E-NO-UETR</EndToEndId>
      </PmtId>
      <IntrBkSttlmAmt Ccy="USD">100.00</IntrBkSttlmAmt>
      <IntrBkSttlmDt>2024-01-01</IntrBkSttlmDt>
      <ChrgBr>SLEV</ChrgBr>
      <Dbtr><Nm>Alice</Nm></Dbtr>
      <DbtrAgt><FinInstnId><BICFI>AAAAGB2LXXX</BICFI></FinInstnId></DbtrAgt>
      <CdtrAgt><FinInstnId><BICFI>BBBBUS33XXX</BICFI></FinInstnId></CdtrAgt>
      <Cdtr><Nm>Bob</Nm></Cdtr>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#;
    let v = FedNowValidator::new();
    let result = v.validate(xml, "pacs.008.001.13");
    assert!(
        has_error_with_rule(&result, "FEDNOW_UETR_REQUIRED"),
        "Expected FEDNOW_UETR_REQUIRED; got: {:?}",
        result.errors
    );
}

#[test]
fn fednow_non_clrg_settlement() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13">
  <FIToFICstmrCdtTrf>
    <GrpHdr>
      <MsgId>NON-CLRG-001</MsgId>
      <CreDtTm>2024-01-01T12:00:00Z</CreDtTm>
      <NbOfTxs>1</NbOfTxs>
      <SttlmInf><SttlmMtd>INGA</SttlmMtd></SttlmInf>
    </GrpHdr>
    <CdtTrfTxInf>
      <PmtId>
        <EndToEndId>E2E-NON-CLRG</EndToEndId>
        <UETR>97ed4827-7b6f-4491-a06f-b548d5a7512d</UETR>
      </PmtId>
      <IntrBkSttlmAmt Ccy="USD">100.00</IntrBkSttlmAmt>
      <IntrBkSttlmDt>2024-01-01</IntrBkSttlmDt>
      <ChrgBr>SLEV</ChrgBr>
      <Dbtr><Nm>Alice</Nm></Dbtr>
      <DbtrAgt><FinInstnId><BICFI>AAAAGB2LXXX</BICFI></FinInstnId></DbtrAgt>
      <CdtrAgt><FinInstnId><BICFI>BBBBUS33XXX</BICFI></FinInstnId></CdtrAgt>
      <Cdtr><Nm>Bob</Nm></Cdtr>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#;
    let v = FedNowValidator::new();
    let result = v.validate(xml, "pacs.008.001.13");
    assert!(
        has_error_with_rule(&result, "FEDNOW_STTLM_MTD"),
        "Expected FEDNOW_STTLM_MTD error; got: {:?}",
        result.errors
    );
}

// ---------------------------------------------------------------------------
// SEPA tests
// ---------------------------------------------------------------------------

#[test]
fn sepa_valid_pacs008() {
    let xml = read_fixture("sepa/valid_pacs008.xml");
    let v = SepaValidator::new();
    let result = v.validate(&xml, "pacs.008.001.13");
    assert!(
        result.is_valid(),
        "Expected valid SEPA pacs.008 to pass; errors: {:?}",
        result.errors
    );
}

#[test]
fn sepa_invalid_currency() {
    let xml = read_fixture("sepa/invalid_usd.xml");
    let v = SepaValidator::new();
    let result = v.validate(&xml, "pacs.008.001.13");
    assert!(
        has_error_with_rule(&result, "SEPA_CURRENCY"),
        "Expected SEPA_CURRENCY error for USD; got: {:?}",
        result.errors
    );
}

#[test]
fn sepa_invalid_charset() {
    // The <Nm> field contains Cyrillic characters.
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13">
  <FIToFICstmrCdtTrf>
    <GrpHdr>
      <MsgId>SEPA-CHARSET-001</MsgId>
      <CreDtTm>2024-01-01T09:00:00Z</CreDtTm>
      <NbOfTxs>1</NbOfTxs>
      <SttlmInf><SttlmMtd>CLRG</SttlmMtd></SttlmInf>
    </GrpHdr>
    <CdtTrfTxInf>
      <PmtId>
        <EndToEndId>E2E-CHARSET-001</EndToEndId>
        <UETR>97ed4827-7b6f-4491-a06f-b548d5a7512d</UETR>
      </PmtId>
      <IntrBkSttlmAmt Ccy="EUR">100.00</IntrBkSttlmAmt>
      <IntrBkSttlmDt>2024-01-01</IntrBkSttlmDt>
      <ChrgBr>SLEV</ChrgBr>
      <Dbtr><Nm>Алиса Смит</Nm></Dbtr>
      <DbtrAcct><Id><IBAN>DE89370400440532013000</IBAN></Id></DbtrAcct>
      <DbtrAgt><FinInstnId><BICFI>DEUTDEDBXXX</BICFI></FinInstnId></DbtrAgt>
      <CdtrAgt><FinInstnId><BICFI>BNPAFRPPXXX</BICFI></FinInstnId></CdtrAgt>
      <Cdtr><Nm>Marie Dupont</Nm></Cdtr>
      <CdtrAcct><Id><IBAN>FR7630006000011234567890189</IBAN></Id></CdtrAcct>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#;
    let v = SepaValidator::new();
    let result = v.validate(xml, "pacs.008.001.13");
    assert!(
        has_error_with_rule(&result, "SEPA_CHARSET"),
        "Expected SEPA_CHARSET error for Cyrillic name; got: {:?}",
        result.errors
    );
}

#[test]
fn sepa_amount_too_high() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13">
  <FIToFICstmrCdtTrf>
    <GrpHdr>
      <MsgId>SEPA-OVER-MAX-001</MsgId>
      <CreDtTm>2024-01-01T09:00:00Z</CreDtTm>
      <NbOfTxs>1</NbOfTxs>
      <SttlmInf><SttlmMtd>CLRG</SttlmMtd></SttlmInf>
    </GrpHdr>
    <CdtTrfTxInf>
      <PmtId>
        <EndToEndId>E2E-OVER-MAX-001</EndToEndId>
        <UETR>97ed4827-7b6f-4491-a06f-b548d5a7512d</UETR>
      </PmtId>
      <IntrBkSttlmAmt Ccy="EUR">1000000000.00</IntrBkSttlmAmt>
      <IntrBkSttlmDt>2024-01-01</IntrBkSttlmDt>
      <ChrgBr>SLEV</ChrgBr>
      <Dbtr><Nm>Hans Muller</Nm></Dbtr>
      <DbtrAcct><Id><IBAN>DE89370400440532013000</IBAN></Id></DbtrAcct>
      <DbtrAgt><FinInstnId><BICFI>DEUTDEDBXXX</BICFI></FinInstnId></DbtrAgt>
      <CdtrAgt><FinInstnId><BICFI>BNPAFRPPXXX</BICFI></FinInstnId></CdtrAgt>
      <Cdtr><Nm>Marie Dupont</Nm></Cdtr>
      <CdtrAcct><Id><IBAN>FR7630006000011234567890189</IBAN></Id></CdtrAcct>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#;
    let v = SepaValidator::new();
    let result = v.validate(xml, "pacs.008.001.13");
    assert!(
        has_error_with_rule(&result, "SEPA_AMOUNT_MAX"),
        "Expected SEPA_AMOUNT_MAX error for 1,000,000,000 EUR; got: {:?}",
        result.errors
    );
}

#[test]
fn sepa_slev_required() {
    // ChrgBr = SHAR is invalid for SEPA SCT.
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13">
  <FIToFICstmrCdtTrf>
    <GrpHdr>
      <MsgId>SEPA-NON-SLEV-001</MsgId>
      <CreDtTm>2024-01-01T09:00:00Z</CreDtTm>
      <NbOfTxs>1</NbOfTxs>
      <SttlmInf><SttlmMtd>CLRG</SttlmMtd></SttlmInf>
    </GrpHdr>
    <CdtTrfTxInf>
      <PmtId>
        <EndToEndId>E2E-NON-SLEV-001</EndToEndId>
        <UETR>97ed4827-7b6f-4491-a06f-b548d5a7512d</UETR>
      </PmtId>
      <IntrBkSttlmAmt Ccy="EUR">100.00</IntrBkSttlmAmt>
      <IntrBkSttlmDt>2024-01-01</IntrBkSttlmDt>
      <ChrgBr>SHAR</ChrgBr>
      <Dbtr><Nm>Hans Muller</Nm></Dbtr>
      <DbtrAcct><Id><IBAN>DE89370400440532013000</IBAN></Id></DbtrAcct>
      <DbtrAgt><FinInstnId><BICFI>DEUTDEDBXXX</BICFI></FinInstnId></DbtrAgt>
      <CdtrAgt><FinInstnId><BICFI>BNPAFRPPXXX</BICFI></FinInstnId></CdtrAgt>
      <Cdtr><Nm>Marie Dupont</Nm></Cdtr>
      <CdtrAcct><Id><IBAN>FR7630006000011234567890189</IBAN></Id></CdtrAcct>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#;
    let v = SepaValidator::new();
    let result = v.validate(xml, "pacs.008.001.13");
    assert!(
        has_error_with_rule(&result, "SEPA_CHRGBR"),
        "Expected SEPA_CHRGBR error for ChrgBr=SHAR; got: {:?}",
        result.errors
    );
}

// ---------------------------------------------------------------------------
// CBPR+ tests
// ---------------------------------------------------------------------------

#[test]
fn cbpr_valid_pacs008() {
    let xml = read_fixture("cbpr/valid_pacs008.xml");
    let v = CbprPlusValidator::new();
    let result = v.validate(&xml, "pacs.008.001.13");
    assert!(
        result.is_valid(),
        "Expected valid CBPR+ pacs.008 to pass; errors: {:?}",
        result.errors
    );
}

#[test]
fn cbpr_missing_instg_agt_bic() {
    let xml = read_fixture("cbpr/missing_bic.xml");
    let v = CbprPlusValidator::new();
    let result = v.validate(&xml, "pacs.008.001.13");
    assert!(
        has_error_with_rule(&result, "CBPR_INSTG_AGT_BIC"),
        "Expected CBPR_INSTG_AGT_BIC error; got: {:?}",
        result.errors
    );
}

#[test]
fn cbpr_missing_uetr() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<BizMsgEnvlp>
  <AppHdr><BizMsgIdr>BAH-NO-UETR-001</BizMsgIdr></AppHdr>
  <Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13">
    <FIToFICstmrCdtTrf>
      <GrpHdr>
        <MsgId>NO-UETR-001</MsgId>
        <CreDtTm>2024-01-01T12:00:00Z</CreDtTm>
        <NbOfTxs>1</NbOfTxs>
        <SttlmInf><SttlmMtd>INGA</SttlmMtd></SttlmInf>
        <InstgAgt><FinInstnId><BICFI>AAAAGB2LXXX</BICFI></FinInstnId></InstgAgt>
        <InstdAgt><FinInstnId><BICFI>BBBBUS33XXX</BICFI></FinInstnId></InstdAgt>
      </GrpHdr>
      <CdtTrfTxInf>
        <PmtId>
          <EndToEndId>E2E-NO-UETR</EndToEndId>
        </PmtId>
        <IntrBkSttlmAmt Ccy="USD">100.00</IntrBkSttlmAmt>
        <IntrBkSttlmDt>2024-01-01</IntrBkSttlmDt>
        <ChrgBr>SHAR</ChrgBr>
        <Dbtr><Nm>Alice</Nm></Dbtr>
        <DbtrAgt><FinInstnId><BICFI>AAAAGB2LXXX</BICFI></FinInstnId></DbtrAgt>
        <CdtrAgt><FinInstnId><BICFI>BBBBUS33XXX</BICFI></FinInstnId></CdtrAgt>
        <Cdtr><Nm>Bob</Nm></Cdtr>
      </CdtTrfTxInf>
    </FIToFICstmrCdtTrf>
  </Document>
</BizMsgEnvlp>"#;
    let v = CbprPlusValidator::new();
    let result = v.validate(xml, "pacs.008.001.13");
    assert!(
        has_error_with_rule(&result, "CBPR_UETR_REQUIRED"),
        "Expected CBPR_UETR_REQUIRED; got: {:?}",
        result.errors
    );
}

#[test]
fn cbpr_missing_debtor_name() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<BizMsgEnvlp>
  <AppHdr><BizMsgIdr>BAH-NO-DBT-NM-001</BizMsgIdr></AppHdr>
  <Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13">
    <FIToFICstmrCdtTrf>
      <GrpHdr>
        <MsgId>NO-DBT-NM-001</MsgId>
        <CreDtTm>2024-01-01T12:00:00Z</CreDtTm>
        <NbOfTxs>1</NbOfTxs>
        <SttlmInf><SttlmMtd>INGA</SttlmMtd></SttlmInf>
        <InstgAgt><FinInstnId><BICFI>AAAAGB2LXXX</BICFI></FinInstnId></InstgAgt>
        <InstdAgt><FinInstnId><BICFI>BBBBUS33XXX</BICFI></FinInstnId></InstdAgt>
      </GrpHdr>
      <CdtTrfTxInf>
        <PmtId>
          <EndToEndId>E2E-NO-DBT-NM</EndToEndId>
          <UETR>97ed4827-7b6f-4491-a06f-b548d5a7512d</UETR>
        </PmtId>
        <IntrBkSttlmAmt Ccy="USD">100.00</IntrBkSttlmAmt>
        <IntrBkSttlmDt>2024-01-01</IntrBkSttlmDt>
        <ChrgBr>SHAR</ChrgBr>
        <Dbtr>
          <!-- Nm intentionally missing -->
          <Id><OrgId><AnyBIC>AAAAGB2LXXX</AnyBIC></OrgId></Id>
        </Dbtr>
        <DbtrAgt><FinInstnId><BICFI>AAAAGB2LXXX</BICFI></FinInstnId></DbtrAgt>
        <CdtrAgt><FinInstnId><BICFI>BBBBUS33XXX</BICFI></FinInstnId></CdtrAgt>
        <Cdtr><Nm>Bob Jones</Nm></Cdtr>
      </CdtTrfTxInf>
    </FIToFICstmrCdtTrf>
  </Document>
</BizMsgEnvlp>"#;
    let v = CbprPlusValidator::new();
    let result = v.validate(xml, "pacs.008.001.13");
    assert!(
        has_error_with_rule(&result, "CBPR_DBTR_NM_REQUIRED"),
        "Expected CBPR_DBTR_NM_REQUIRED; got: {:?}",
        result.errors
    );
}

#[test]
fn cbpr_bic_padding_warning() {
    // 8-char BICs should produce a warning (not an error).
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<BizMsgEnvlp>
  <AppHdr><BizMsgIdr>BAH-SHORT-BIC-001</BizMsgIdr></AppHdr>
  <Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13">
    <FIToFICstmrCdtTrf>
      <GrpHdr>
        <MsgId>SHORT-BIC-001</MsgId>
        <CreDtTm>2024-01-01T12:00:00Z</CreDtTm>
        <NbOfTxs>1</NbOfTxs>
        <SttlmInf><SttlmMtd>INGA</SttlmMtd></SttlmInf>
        <InstgAgt><FinInstnId><BICFI>AAAAGB2L</BICFI></FinInstnId></InstgAgt>
        <InstdAgt><FinInstnId><BICFI>BBBBUS33</BICFI></FinInstnId></InstdAgt>
      </GrpHdr>
      <CdtTrfTxInf>
        <PmtId>
          <EndToEndId>E2E-SHORT-BIC</EndToEndId>
          <UETR>97ed4827-7b6f-4491-a06f-b548d5a7512d</UETR>
        </PmtId>
        <IntrBkSttlmAmt Ccy="USD">100.00</IntrBkSttlmAmt>
        <IntrBkSttlmDt>2024-01-01</IntrBkSttlmDt>
        <ChrgBr>SHAR</ChrgBr>
        <Dbtr><Nm>Alice</Nm></Dbtr>
        <DbtrAgt><FinInstnId><BICFI>AAAAGB2L</BICFI></FinInstnId></DbtrAgt>
        <CdtrAgt><FinInstnId><BICFI>BBBBUS33</BICFI></FinInstnId></CdtrAgt>
        <Cdtr><Nm>Bob</Nm></Cdtr>
      </CdtTrfTxInf>
    </FIToFICstmrCdtTrf>
  </Document>
</BizMsgEnvlp>"#;
    let v = CbprPlusValidator::new();
    let result = v.validate(xml, "pacs.008.001.13");
    // Should be valid (no Errors) but have warnings for 8-char BICs.
    assert!(
        result.is_valid(),
        "8-char BIC should be a warning, not an error; errors: {:?}",
        result.errors
    );
    let has_bic_warning = result
        .errors
        .iter()
        .any(|e| e.rule_id == "CBPR_BIC_PADDING" && e.severity == Severity::Warning);
    assert!(
        has_bic_warning,
        "Expected CBPR_BIC_PADDING warning for 8-char BIC; got: {:?}",
        result.errors
    );
}

// ---------------------------------------------------------------------------
// Cross-scheme tests
// ---------------------------------------------------------------------------

#[test]
fn scheme_names() {
    assert_eq!(FedNowValidator::new().name(), "FedNow");
    assert_eq!(SepaValidator::new().name(), "SEPA");
    assert_eq!(CbprPlusValidator::new().name(), "CBPR+");
}

#[test]
fn supported_messages_are_non_empty() {
    assert!(!FedNowValidator::new().supported_messages().is_empty());
    assert!(!SepaValidator::new().supported_messages().is_empty());
    assert!(!CbprPlusValidator::new().supported_messages().is_empty());
}

#[test]
fn unsupported_message_returns_empty_fednow() {
    let result = FedNowValidator::new().validate("<xml/>", "pain.001.001.09");
    assert!(result.errors.is_empty());
}

#[test]
fn unsupported_message_returns_empty_sepa() {
    let result = SepaValidator::new().validate("<xml/>", "camt.056.001.11");
    assert!(result.errors.is_empty());
}

#[test]
fn unsupported_message_returns_empty_cbpr() {
    let result = CbprPlusValidator::new().validate("<xml/>", "pain.001.001.09");
    assert!(result.errors.is_empty());
}

#[test]
fn validators_are_send_sync() {
    // Compile-time check: ensure validators satisfy Send + Sync so they can be
    // used in Arc<dyn SchemeValidator>.
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FedNowValidator>();
    assert_send_sync::<SepaValidator>();
    assert_send_sync::<CbprPlusValidator>();
}

// ---------------------------------------------------------------------------
// Typed validation integration tests
// ---------------------------------------------------------------------------

use mx20022_model::generated::pacs::pacs_008_001_13;

/// Deserialize a pacs.008 fixture XML into a typed Document.
///
/// If the XML is wrapped in an envelope (e.g. `<BizMsgEnvlp>`), extracts the
/// inner `<Document>` element first.
fn parse_pacs008(xml: &str) -> pacs_008_001_13::Document {
    let doc_xml = extract_document_fragment(xml);
    mx20022_parse::de::from_str(doc_xml).expect("fixture must deserialize into pacs.008 Document")
}

/// Extract the `<Document ...>...</Document>` fragment from possibly-enveloped XML.
fn extract_document_fragment(xml: &str) -> &str {
    if let Some(start) = xml.find("<Document") {
        if let Some(end) = xml.rfind("</Document>") {
            return &xml[start..end + "</Document>".len()];
        }
    }
    xml
}

#[test]
fn fednow_typed_valid_pacs008() {
    let xml = read_fixture("fednow/valid_pacs008.xml");
    let doc = parse_pacs008(&xml);
    let v = FedNowValidator::new();
    let result = v
        .validate_typed(&doc, "pacs.008.001.13")
        .expect("should support pacs.008");
    assert!(
        result.is_valid(),
        "Expected valid FedNow pacs.008 to pass typed validation; errors: {:?}",
        result.errors
    );
}

#[test]
fn fednow_typed_invalid_currency() {
    let xml = read_fixture("fednow/invalid_eur.xml");
    let doc = parse_pacs008(&xml);
    let v = FedNowValidator::new();
    let result = v
        .validate_typed(&doc, "pacs.008.001.13")
        .expect("should support pacs.008");
    assert!(
        has_error_with_rule(&result, "FEDNOW_CURRENCY"),
        "Expected FEDNOW_CURRENCY error for EUR currency in typed path; got: {:?}",
        result.errors
    );
}

#[test]
fn fednow_typed_over_amount_limit() {
    let xml = read_fixture("fednow/over_limit.xml");
    let doc = parse_pacs008(&xml);
    let v = FedNowValidator::new();
    let result = v
        .validate_typed(&doc, "pacs.008.001.13")
        .expect("should support pacs.008");
    assert!(
        has_error_with_rule(&result, "FEDNOW_AMOUNT_LIMIT"),
        "Expected FEDNOW_AMOUNT_LIMIT in typed path; got: {:?}",
        result.errors
    );
}

#[test]
fn fednow_typed_unsupported_message_returns_none() {
    let xml = read_fixture("fednow/valid_pacs008.xml");
    let doc = parse_pacs008(&xml);
    let v = FedNowValidator::new();
    // Pass wrong message type — should return None.
    let result = v.validate_typed(&doc, "pacs.009.001.10");
    assert!(result.is_none());
}

#[test]
fn fednow_typed_wrong_type_returns_none() {
    // Pass a non-Document type — downcast should fail gracefully.
    let v = FedNowValidator::new();
    let bogus = String::from("not a Document");
    let result = v.validate_typed(&bogus, "pacs.008.001.13");
    assert!(result.is_none());
}

#[test]
fn sepa_typed_valid_pacs008() {
    let xml = read_fixture("sepa/valid_pacs008.xml");
    let doc = parse_pacs008(&xml);
    let v = SepaValidator::new();
    let result = v
        .validate_typed(&doc, "pacs.008.001.13")
        .expect("should support pacs.008");
    assert!(
        result.is_valid(),
        "Expected valid SEPA pacs.008 to pass typed validation; errors: {:?}",
        result.errors
    );
}

#[test]
fn sepa_typed_invalid_currency() {
    let xml = read_fixture("sepa/invalid_usd.xml");
    let doc = parse_pacs008(&xml);
    let v = SepaValidator::new();
    let result = v
        .validate_typed(&doc, "pacs.008.001.13")
        .expect("should support pacs.008");
    assert!(
        has_error_with_rule(&result, "SEPA_CURRENCY"),
        "Expected SEPA_CURRENCY error for USD currency in typed path; got: {:?}",
        result.errors
    );
}

#[test]
fn sepa_typed_missing_iban() {
    // Construct a SEPA pacs.008 without IBAN accounts — typed path should
    // produce SEPA_IBAN_REQUIRED error.
    let xml = read_fixture("sepa/valid_pacs008.xml");
    let mut doc = parse_pacs008(&xml);

    // Remove debtor and creditor account IBANs by clearing the accounts.
    for tx in &mut doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf {
        tx.dbtr_acct = None;
        tx.cdtr_acct = None;
    }

    let v = SepaValidator::new();
    let result = v
        .validate_typed(&doc, "pacs.008.001.13")
        .expect("should support pacs.008");
    assert!(
        has_error_with_rule(&result, "SEPA_IBAN_REQUIRED"),
        "Expected SEPA_IBAN_REQUIRED when both IBANs are missing; got: {:?}",
        result.errors
    );
}

#[test]
fn sepa_typed_partial_iban() {
    // Only one of debtor/creditor has IBAN — typed path should produce
    // SEPA_IBAN_BOTH warning.
    let xml = read_fixture("sepa/valid_pacs008.xml");
    let mut doc = parse_pacs008(&xml);

    // Remove only debtor IBAN, keep creditor.
    for tx in &mut doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf {
        tx.dbtr_acct = None;
    }

    let v = SepaValidator::new();
    let result = v
        .validate_typed(&doc, "pacs.008.001.13")
        .expect("should support pacs.008");
    assert!(
        has_warning_with_rule(&result, "SEPA_IBAN_BOTH"),
        "Expected SEPA_IBAN_BOTH warning when only one IBAN present; got: {:?}",
        result.errors
    );
}

#[test]
fn cbpr_typed_valid_pacs008() {
    let xml = read_fixture("cbpr/valid_pacs008.xml");
    let doc = parse_pacs008(&xml);
    let v = CbprPlusValidator::new();
    let result = v
        .validate_typed(&doc, "pacs.008.001.13")
        .expect("should support pacs.008");
    assert!(
        result.is_valid(),
        "Expected valid CBPR+ pacs.008 to pass typed validation; errors: {:?}",
        result.errors
    );
}

#[test]
fn cbpr_typed_missing_bic() {
    let xml = read_fixture("cbpr/missing_bic.xml");
    let doc = parse_pacs008(&xml);
    let v = CbprPlusValidator::new();
    let result = v
        .validate_typed(&doc, "pacs.008.001.13")
        .expect("should support pacs.008");
    // Should detect missing BIC (at least one CBPR_*_BIC error).
    let has_bic_error = result.errors.iter().any(|e| e.rule_id.contains("_BIC"));
    assert!(
        has_bic_error,
        "Expected at least one BIC-related error in typed path; got: {:?}",
        result.errors
    );
}

#[test]
fn cbpr_typed_grphdr_bic_padding_warning() {
    // 8-char BICs on GrpHdr InstgAgt/InstdAgt should produce warnings.
    let xml = read_fixture("cbpr/valid_pacs008.xml");
    let mut doc = parse_pacs008(&xml);

    // Shorten GrpHdr agent BICs to 8 chars to trigger padding warnings.
    if let Some(ref mut agt) = doc.fi_to_fi_cstmr_cdt_trf.grp_hdr.instg_agt {
        if let Some(ref mut bic) = agt.fin_instn_id.bicfi {
            bic.0 = "AAAAGB2L".to_string();
        }
    }
    if let Some(ref mut agt) = doc.fi_to_fi_cstmr_cdt_trf.grp_hdr.instd_agt {
        if let Some(ref mut bic) = agt.fin_instn_id.bicfi {
            bic.0 = "BBBBUS33".to_string();
        }
    }

    let v = CbprPlusValidator::new();
    let result = v
        .validate_typed(&doc, "pacs.008.001.13")
        .expect("should support pacs.008");
    let bic_warnings: Vec<_> = result
        .errors
        .iter()
        .filter(|e| e.rule_id == "CBPR_BIC_PADDING" && e.severity == Severity::Warning)
        .collect();
    assert!(
        bic_warnings.len() >= 2,
        "Expected at least 2 CBPR_BIC_PADDING warnings for GrpHdr agents; got {}: {:?}",
        bic_warnings.len(),
        bic_warnings
    );
}

#[test]
fn vec_index_in_validation_paths() {
    // Test that Vec items inside a Vec field get indexed paths like
    // "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf[0]/PmtId/EndToEndId".
    use mx20022_model::common::validate::Validatable;

    let xml = read_fixture("fednow/valid_pacs008.xml");
    let mut doc = parse_pacs008(&xml);

    // Force a MaxLength violation on EndToEndId (Max35Text) inside the
    // first CdtTrfTxInf element to guarantee a violation with a Vec index.
    doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf[0]
        .pmt_id
        .end_to_end_id
        .0 = "A".repeat(50); // 50 chars > 35 limit

    let mut violations = Vec::new();
    doc.validate_constraints("/Document", &mut violations);

    // At least one violation should have a path containing "[0]".
    let indexed: Vec<_> = violations
        .iter()
        .filter(|v| v.path.contains("[0]"))
        .collect();
    assert!(
        !indexed.is_empty(),
        "Expected at least one violation path with [0] index; all paths: {:?}",
        violations.iter().map(|v| &v.path).collect::<Vec<_>>()
    );
    // Verify the path includes the Vec field name with index.
    assert!(
        indexed.iter().any(|v| v.path.contains("CdtTrfTxInf[0]")),
        "Expected path to contain 'CdtTrfTxInf[0]'; indexed paths: {:?}",
        indexed.iter().map(|v| &v.path).collect::<Vec<_>>()
    );
}

#[test]
fn typed_validate_constraints_integration() {
    use mx20022_model::common::validate::Validatable;

    let xml = read_fixture("fednow/valid_pacs008.xml");
    let doc = parse_pacs008(&xml);
    let mut violations = Vec::new();
    doc.validate_constraints("/Document", &mut violations);

    assert!(
        violations.is_empty(),
        "Valid pacs.008 fixture should have zero constraint violations, got {}: {:?}",
        violations.len(),
        violations.iter().map(|v| &v.path).collect::<Vec<_>>()
    );
}

#[test]
fn typed_validate_bridge() {
    let xml = read_fixture("fednow/valid_pacs008.xml");
    let doc = parse_pacs008(&xml);
    let result = mx20022_validate::typed::validate_constraints(&doc, "/Document");

    assert!(
        result.is_valid(),
        "Valid pacs.008 fixture should produce a valid result from typed bridge, got {} errors",
        result.error_count()
    );
}
