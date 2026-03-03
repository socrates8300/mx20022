//! Integration tests for the generated builder API.
//!
//! These tests verify that:
//! - `BusinessApplicationHeaderV04::builder()` builds successfully with all
//!   required fields set.
//! - `BuilderError` is returned when required fields are missing.
//! - Chaining syntax works end-to-end.

use mx20022_model::common::{BuilderError, ChoiceWrapper};
use mx20022_model::generated::head::{
    BICFIDec2014Identifier, BranchAndFinancialInstitutionIdentification8,
    BusinessApplicationHeaderV04, FinancialInstitutionIdentification23, ISODateTime, Max35Text,
    Party51Choice,
};

// ── Helpers ───────────────────────────────────────────────────────────────────

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

fn make_party(bic: &str) -> ChoiceWrapper<Party51Choice> {
    ChoiceWrapper::new(Party51Choice::FIId(make_fi_id(bic)))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn builder_succeeds_with_all_required_fields() {
    let bah = BusinessApplicationHeaderV04::builder()
        .fr(make_party("AAAAGB2LXXX"))
        .to(make_party("BBBBUS33XXX"))
        .biz_msg_idr(Max35Text("MSG-001".to_owned()))
        .msg_def_idr(Max35Text("pacs.008.001.13".to_owned()))
        .cre_dt(ISODateTime("2024-01-01T12:00:00Z".to_owned()))
        .build();

    let bah = bah.expect("build must succeed with all required fields");
    assert_eq!(bah.biz_msg_idr.0, "MSG-001");
    assert_eq!(bah.msg_def_idr.0, "pacs.008.001.13");
    assert_eq!(bah.cre_dt.0, "2024-01-01T12:00:00Z");
    assert!(bah.char_set.is_none());
    assert!(bah.rltd.is_empty());
}

#[test]
fn builder_fails_when_required_field_missing() {
    // Missing: fr, to, biz_msg_idr, msg_def_idr, cre_dt
    let result = BusinessApplicationHeaderV04::builder().build();

    match result {
        Err(BuilderError {
            ref type_name,
            ref missing_fields,
        }) => {
            assert_eq!(type_name, "BusinessApplicationHeaderV04");
            // All five required fields should be reported.
            assert!(
                missing_fields.contains(&"fr".to_owned()),
                "expected `fr` in missing: {missing_fields:?}"
            );
            assert!(
                missing_fields.contains(&"to".to_owned()),
                "expected `to` in missing: {missing_fields:?}"
            );
            assert!(
                missing_fields.contains(&"biz_msg_idr".to_owned()),
                "expected `biz_msg_idr` in missing: {missing_fields:?}"
            );
            assert!(
                missing_fields.contains(&"msg_def_idr".to_owned()),
                "expected `msg_def_idr` in missing: {missing_fields:?}"
            );
            assert!(
                missing_fields.contains(&"cre_dt".to_owned()),
                "expected `cre_dt` in missing: {missing_fields:?}"
            );
        }
        Ok(_) => panic!("expected build to fail with missing fields"),
    }
}

#[test]
fn builder_fails_with_partially_missing_fields() {
    // Provide some but not all required fields.
    let result = BusinessApplicationHeaderV04::builder()
        .fr(make_party("AAAAGB2LXXX"))
        .biz_msg_idr(Max35Text("MSG-001".to_owned()))
        .build();

    let err = result.expect_err("must fail — to, msg_def_idr, cre_dt are missing");
    assert_eq!(err.type_name, "BusinessApplicationHeaderV04");
    assert!(
        err.missing_fields.contains(&"to".to_owned()),
        "{:?}",
        err.missing_fields
    );
    assert!(
        err.missing_fields.contains(&"msg_def_idr".to_owned()),
        "{:?}",
        err.missing_fields
    );
    assert!(
        err.missing_fields.contains(&"cre_dt".to_owned()),
        "{:?}",
        err.missing_fields
    );
    // `fr` and `biz_msg_idr` were set, so they must not appear in missing.
    assert!(
        !err.missing_fields.contains(&"fr".to_owned()),
        "{:?}",
        err.missing_fields
    );
    assert!(
        !err.missing_fields.contains(&"biz_msg_idr".to_owned()),
        "{:?}",
        err.missing_fields
    );
}

#[test]
fn builder_optional_fields_default_to_none() {
    let bah = BusinessApplicationHeaderV04::builder()
        .fr(make_party("AAAAGB2LXXX"))
        .to(make_party("BBBBUS33XXX"))
        .biz_msg_idr(Max35Text("MSG-001".to_owned()))
        .msg_def_idr(Max35Text("pacs.008.001.13".to_owned()))
        .cre_dt(ISODateTime("2024-01-01T12:00:00Z".to_owned()))
        .build()
        .unwrap();

    assert!(bah.char_set.is_none(), "char_set must default to None");
    assert!(bah.biz_svc.is_none(), "biz_svc must default to None");
    assert!(bah.mkt_prctc.is_none(), "mkt_prctc must default to None");
    assert!(
        bah.biz_prcg_dt.is_none(),
        "biz_prcg_dt must default to None"
    );
    assert!(bah.cpy_dplct.is_none(), "cpy_dplct must default to None");
    assert!(
        bah.pssbl_dplct.is_none(),
        "pssbl_dplct must default to None"
    );
    assert!(bah.prty.is_none(), "prty must default to None");
    assert!(bah.sgntr.is_none(), "sgntr must default to None");
}

#[test]
fn builder_vec_field_defaults_to_empty() {
    let bah = BusinessApplicationHeaderV04::builder()
        .fr(make_party("AAAAGB2LXXX"))
        .to(make_party("BBBBUS33XXX"))
        .biz_msg_idr(Max35Text("MSG-001".to_owned()))
        .msg_def_idr(Max35Text("pacs.008.001.13".to_owned()))
        .cre_dt(ISODateTime("2024-01-01T12:00:00Z".to_owned()))
        .build()
        .unwrap();

    assert!(bah.rltd.is_empty(), "rltd vec must default to empty");
}

#[test]
fn builder_output_matches_struct_literal() {
    // Build via builder.
    let via_builder = BusinessApplicationHeaderV04::builder()
        .fr(make_party("AAAAGB2LXXX"))
        .to(make_party("BBBBUS33XXX"))
        .biz_msg_idr(Max35Text("MSG-20240101-001".to_owned()))
        .msg_def_idr(Max35Text("pacs.008.001.13".to_owned()))
        .cre_dt(ISODateTime("2024-01-01T12:00:00Z".to_owned()))
        .build()
        .unwrap();

    // Build via struct literal.
    let via_literal = BusinessApplicationHeaderV04 {
        char_set: None,
        fr: make_party("AAAAGB2LXXX"),
        to: make_party("BBBBUS33XXX"),
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
    };

    assert_eq!(
        via_builder, via_literal,
        "builder output must match struct literal"
    );
}

#[test]
fn builder_error_display_is_informative() {
    let err = BusinessApplicationHeaderV04::builder().build().unwrap_err();
    let s = err.to_string();
    assert!(s.contains("BusinessApplicationHeaderV04"), "{s}");
    assert!(s.contains("fr"), "{s}");
}
