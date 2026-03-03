//! Roundtrip integration tests: serialize a type to XML and deserialize back,
//! asserting the result equals the original.
//!
//! All generated xs:choice fields are wrapped in
//! `mx20022_model::common::ChoiceWrapper<T>` so that `quick-xml` + serde can
//! round-trip them correctly.  Access the inner enum value via `.inner` or
//! by dereferencing the wrapper.

use mx20022_model::common::ChoiceWrapper;
use mx20022_model::generated::head::{
    BICFIDec2014Identifier, BranchAndFinancialInstitutionIdentification8,
    BusinessApplicationHeaderV04, FinancialInstitutionIdentification23, ISODateTime, Max35Text,
    Party51Choice,
};
use mx20022_parse::{de, envelope, ser};

// ── Helpers ──────────────────────────────────────────────────────────────────

fn make_fi_id(bic: &str) -> BranchAndFinancialInstitutionIdentification8 {
    BranchAndFinancialInstitutionIdentification8 {
        fin_instn_id: FinancialInstitutionIdentification23 {
            bicfi: Some(BICFIDec2014Identifier(bic.to_owned())),
            clr_sys_mmb_id: None,
            lei: None,
            nm: None,
            pstl_adr: None,
            othr: None,
        },
        brnch_id: None,
    }
}

fn make_bah() -> BusinessApplicationHeaderV04 {
    BusinessApplicationHeaderV04 {
        char_set: None,
        fr: ChoiceWrapper::new(Party51Choice::FIId(make_fi_id("AAAAGB2LXXX"))),
        to: ChoiceWrapper::new(Party51Choice::FIId(make_fi_id("BBBBUS33XXX"))),
        biz_msg_idr: Max35Text("MSG-20240101-001".to_owned()),
        msg_def_idr: Max35Text("pacs.008.001.13".to_owned()),
        biz_svc: None,
        mkt_prctc: None,
        cre_dt: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
        biz_prcg_dt: None,
        cpy_dplct: None,
        pssbl_dplct: None,
        prty: None,
        sgntr: None,
        rltd: vec![],
    }
}

// ── Roundtrip: BusinessApplicationHeaderV04 (full BAH with xs:choice) ────────

#[test]
fn bah_roundtrip() {
    let original = make_bah();

    let xml = ser::to_string(&original).expect("serialization must succeed");

    // Spot-check that key field values appear in the XML
    assert!(
        xml.contains("AAAAGB2LXXX"),
        "sender BIC must appear in XML: {xml}"
    );
    assert!(
        xml.contains("BBBBUS33XXX"),
        "receiver BIC must appear in XML: {xml}"
    );
    assert!(
        xml.contains("MSG-20240101-001"),
        "business message ID must appear in XML: {xml}"
    );
    assert!(
        xml.contains("pacs.008.001.13"),
        "message def ID must appear in XML: {xml}"
    );

    // Deserialize back
    let roundtripped: BusinessApplicationHeaderV04 =
        de::from_str(&xml).expect("deserialization must succeed");

    assert_eq!(original, roundtripped, "roundtrip must be identity");
}

#[test]
fn bah_roundtrip_with_xml_declaration() {
    let original = make_bah();

    let xml = ser::to_string_with_declaration(&original)
        .expect("serialization with declaration must succeed");

    assert!(
        xml.starts_with(r#"<?xml version="1.0" encoding="UTF-8"?>"#),
        "output must start with XML declaration: {xml}"
    );

    let roundtripped: BusinessApplicationHeaderV04 =
        de::from_str(&xml).expect("deserialization with declaration must succeed");

    assert_eq!(
        original, roundtripped,
        "roundtrip with declaration must be identity"
    );
}

#[test]
fn bah_choice_field_access() {
    let bah = make_bah();

    // ChoiceWrapper implements Deref, so `*bah.fr` is `Party51Choice`
    match &*bah.fr {
        Party51Choice::FIId(fi) => {
            assert_eq!(fi.fin_instn_id.bicfi.as_ref().unwrap().0, "AAAAGB2LXXX");
        }
        other => panic!("expected FIId, got {other:?}"),
    }

    // Also accessible via .inner
    match &bah.to.inner {
        Party51Choice::FIId(fi) => {
            assert_eq!(fi.fin_instn_id.bicfi.as_ref().unwrap().0, "BBBBUS33XXX");
        }
        other => panic!("expected FIId, got {other:?}"),
    }
}

// ── Roundtrip: plain struct without xs:choice fields ─────────────────────────

#[test]
fn fi_id_struct_roundtrip() {
    use mx20022_model::generated::head::BranchData5;
    let original = BranchData5 {
        id: Some(Max35Text("BRANCH-001".to_owned())),
        lei: None,
        nm: Some(mx20022_model::generated::head::Max140Text(
            "London Branch".to_owned(),
        )),
        pstl_adr: None,
    };
    let xml = ser::to_string(&original).expect("serialization must succeed");
    let roundtripped: BranchData5 = de::from_str(&xml).expect("deserialization must succeed");
    assert_eq!(original, roundtripped);
}

// ── Envelope detection ───────────────────────────────────────────────────────

#[test]
fn envelope_detect_pacs_008() {
    let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13"/>"#;
    let id = envelope::detect_message_type(xml).expect("must detect namespace");
    assert_eq!(id.family, "pacs");
    assert_eq!(id.msg_id, "008");
    assert_eq!(id.variant, "001");
    assert_eq!(id.version, "13");
    assert_eq!(id.dotted(), "pacs.008.001.13");
}

#[test]
fn envelope_detect_head_001() {
    let xml = r#"<AppHdr xmlns="urn:iso:std:iso:20022:tech:xsd:head.001.001.04">
      <BizMsgIdr>test</BizMsgIdr>
    </AppHdr>"#;
    let id = envelope::detect_message_type(xml).expect("must detect namespace");
    assert_eq!(id.dotted(), "head.001.001.04");
}

#[test]
fn envelope_detect_pacs_002() {
    let xml = r#"<?xml version="1.0"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.002.001.14">
</Document>"#;
    let id = envelope::detect_message_type(xml).unwrap();
    assert_eq!(id.dotted(), "pacs.002.001.14");
}

// ── Builder → serialize → parse roundtrip ────────────────────────────────────

#[test]
fn builder_roundtrip() {
    // Use the generated builder to construct a BusinessApplicationHeaderV04.
    let original = BusinessApplicationHeaderV04::builder()
        .fr(ChoiceWrapper::new(Party51Choice::FIId(make_fi_id(
            "AAAAGB2LXXX",
        ))))
        .to(ChoiceWrapper::new(Party51Choice::FIId(make_fi_id(
            "BBBBUS33XXX",
        ))))
        .biz_msg_idr(Max35Text("MSG-BUILDER-001".to_owned()))
        .msg_def_idr(Max35Text("pacs.008.001.13".to_owned()))
        .cre_dt(ISODateTime("2024-06-01T09:00:00Z".to_owned()))
        .build()
        .expect("all required fields set — build must succeed");

    // Serialize to XML.
    let xml = ser::to_string(&original).expect("serialization must succeed");

    assert!(xml.contains("AAAAGB2LXXX"), "sender BIC in XML: {xml}");
    assert!(xml.contains("BBBBUS33XXX"), "receiver BIC in XML: {xml}");
    assert!(xml.contains("MSG-BUILDER-001"), "msg ID in XML: {xml}");

    // Deserialize back.
    let roundtripped: BusinessApplicationHeaderV04 =
        de::from_str(&xml).expect("deserialization must succeed");

    assert_eq!(original, roundtripped, "builder roundtrip must be identity");
}

#[test]
fn builder_result_matches_struct_literal_for_roundtrip() {
    // Build via builder.
    let via_builder = BusinessApplicationHeaderV04::builder()
        .fr(ChoiceWrapper::new(Party51Choice::FIId(make_fi_id(
            "AAAAGB2LXXX",
        ))))
        .to(ChoiceWrapper::new(Party51Choice::FIId(make_fi_id(
            "BBBBUS33XXX",
        ))))
        .biz_msg_idr(Max35Text("MSG-20240101-001".to_owned()))
        .msg_def_idr(Max35Text("pacs.008.001.13".to_owned()))
        .cre_dt(ISODateTime("2024-01-01T12:00:00Z".to_owned()))
        .build()
        .unwrap();

    // Identical value via struct literal (same as make_bah()).
    let via_literal = make_bah();

    assert_eq!(
        via_builder, via_literal,
        "builder and literal must be equal"
    );

    // Both serialize to the same XML.
    let xml_builder = ser::to_string(&via_builder).unwrap();
    let xml_literal = ser::to_string(&via_literal).unwrap();
    assert_eq!(xml_builder, xml_literal, "XML outputs must be identical");
}
