//! Roundtrip integration tests for all 10 newly generated ISO 20022 message types.
//!
//! For each message type this module:
//! 1. Constructs a minimal Document struct in Rust.
//! 2. Serializes it to an XML string via `mx20022_parse::ser::to_string`.
//! 3. Deserializes back to a Document struct via `mx20022_parse::de::from_str`.
//! 4. Asserts that the result equals the original (roundtrip identity).
//!
//! Fixture-file-based envelope detection tests verify that the namespace URIs
//! in the minimal XML fixtures are correctly parsed.

use mx20022_parse::{de, envelope, ser};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Read a fixture file relative to the workspace root.
fn fixture(path: &str) -> String {
    // Integration tests run with cwd = package root (crates/mx20022-parse),
    // so we navigate up two levels to reach the workspace root.
    let full_path = format!("../../testdata/xml/{path}");
    std::fs::read_to_string(&full_path)
        .unwrap_or_else(|e| panic!("failed to read fixture {full_path}: {e}"))
}

// ─────────────────────────────────────────────────────────────────────────────
// pacs.002.001.14 — FI to FI Payment Status Report
// ─────────────────────────────────────────────────────────────────────────────

mod pacs_002 {
    use super::*;
    use mx20022_model::generated::pacs::pacs_002_001_14::{
        Document, FIToFIPaymentStatusReportV14, GroupHeader120, ISODateTime, Max35Text,
        OriginalGroupHeader22,
    };

    fn make_doc() -> Document {
        Document {
            fi_to_fi_pmt_sts_rpt: FIToFIPaymentStatusReportV14 {
                grp_hdr: GroupHeader120 {
                    msg_id: Max35Text("PACS002-TEST-001".to_owned()),
                    cre_dt_tm: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
                    instg_agt: None,
                    instd_agt: None,
                    orgnl_biz_qry: None,
                },
                orgnl_grp_inf_and_sts: vec![OriginalGroupHeader22 {
                    orgnl_msg_id: Max35Text("PACS008-20231215-001".to_owned()),
                    orgnl_msg_nm_id: Max35Text("pacs.008.001.13".to_owned()),
                    orgnl_cre_dt_tm: None,
                    orgnl_nb_of_txs: None,
                    orgnl_ctrl_sum: None,
                    grp_sts: None,
                    sts_rsn_inf: vec![],
                    nb_of_txs_per_sts: vec![],
                }],
                tx_inf_and_sts: vec![],
                splmtry_data: vec![],
            },
        }
    }

    #[test]
    fn roundtrip() {
        let original = make_doc();
        let xml = ser::to_string(&original).expect("serialization must succeed");
        assert!(
            xml.contains("PACS002-TEST-001"),
            "MsgId must appear in XML: {xml}"
        );
        let roundtripped: Document = de::from_str(&xml).expect("deserialization must succeed");
        assert_eq!(
            original, roundtripped,
            "pacs.002 roundtrip must be identity"
        );
    }

    #[test]
    fn envelope_detect() {
        let xml = fixture("pacs/pacs_002_001_14_minimal.xml");
        let id = envelope::detect_message_type(&xml).expect("must detect namespace");
        assert_eq!(id.dotted(), "pacs.002.001.14");
        assert_eq!(id.family, "pacs");
        assert_eq!(id.msg_id, "002");
        assert_eq!(id.version, "14");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// pacs.008.001.13 — FI to FI Customer Credit Transfer
// ─────────────────────────────────────────────────────────────────────────────

mod pacs_008 {
    use super::*;
    use mx20022_model::generated::pacs::pacs_008_001_13::{
        ActiveCurrencyAndAmount, ActiveCurrencyAndAmountSimpleType, ActiveCurrencyCode,
        BICFIDec2014Identifier, BranchAndFinancialInstitutionIdentification8,
        ChargeBearerType1Code, CreditTransferTransaction70, Document,
        FIToFICustomerCreditTransferV13, FinancialInstitutionIdentification23, GroupHeader131,
        ISODateTime, Max140Text, Max15NumericText, Max35Text, PartyIdentification272,
        PaymentIdentification13, SettlementInstruction15, SettlementMethod1Code,
    };

    fn make_fi(bic: &str) -> BranchAndFinancialInstitutionIdentification8 {
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

    fn make_party(name: &str) -> PartyIdentification272 {
        PartyIdentification272 {
            nm: Some(Max140Text(name.to_owned())),
            pstl_adr: None,
            id: None,
            ctry_of_res: None,
            ctct_dtls: None,
        }
    }

    fn make_doc() -> Document {
        Document {
            fi_to_fi_cstmr_cdt_trf: FIToFICustomerCreditTransferV13 {
                grp_hdr: GroupHeader131 {
                    msg_id: Max35Text("PACS008-TEST-001".to_owned()),
                    cre_dt_tm: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
                    xpry_dt_tm: None,
                    btch_bookg: None,
                    nb_of_txs: Max15NumericText("1".to_owned()),
                    ctrl_sum: None,
                    ttl_intr_bk_sttlm_amt: None,
                    intr_bk_sttlm_dt: None,
                    sttlm_inf: SettlementInstruction15 {
                        sttlm_mtd: SettlementMethod1Code::Clrg,
                        sttlm_acct: None,
                        clr_sys: None,
                        instg_rmbrsmnt_agt: None,
                        instg_rmbrsmnt_agt_acct: None,
                        instd_rmbrsmnt_agt: None,
                        instd_rmbrsmnt_agt_acct: None,
                        thrd_rmbrsmnt_agt: None,
                        thrd_rmbrsmnt_agt_acct: None,
                    },
                    pmt_tp_inf: None,
                    instg_agt: None,
                    instd_agt: None,
                },
                cdt_trf_tx_inf: vec![CreditTransferTransaction70 {
                    pmt_id: PaymentIdentification13 {
                        instr_id: None,
                        end_to_end_id: Max35Text("E2E-TEST-001".to_owned()),
                        tx_id: None,
                        uetr: None,
                        clr_sys_ref: None,
                    },
                    pmt_tp_inf: None,
                    intr_bk_sttlm_amt: ActiveCurrencyAndAmount {
                        value: ActiveCurrencyAndAmountSimpleType("1000.00".to_owned()),
                        ccy: ActiveCurrencyCode("USD".to_owned()),
                    },
                    intr_bk_sttlm_dt: None,
                    sttlm_prty: None,
                    sttlm_tm_indctn: None,
                    sttlm_tm_req: None,
                    addtl_dt_tm: None,
                    instd_amt: None,
                    xchg_rate: None,
                    agrd_rate: None,
                    chrg_br: ChargeBearerType1Code::Slev,
                    chrgs_inf: vec![],
                    mndt_rltd_inf: None,
                    pmt_sgntr: None,
                    prvs_instg_agt1: None,
                    prvs_instg_agt1acct: None,
                    prvs_instg_agt2: None,
                    prvs_instg_agt2acct: None,
                    prvs_instg_agt3: None,
                    prvs_instg_agt3acct: None,
                    instg_agt: None,
                    instd_agt: None,
                    intrmy_agt1: None,
                    intrmy_agt1acct: None,
                    intrmy_agt2: None,
                    intrmy_agt2acct: None,
                    intrmy_agt3: None,
                    intrmy_agt3acct: None,
                    ultmt_dbtr: None,
                    initg_pty: None,
                    dbtr: make_party("Alice Smith"),
                    dbtr_acct: None,
                    dbtr_agt: make_fi("AAAAGB2LXXX"),
                    dbtr_agt_acct: None,
                    cdtr_agt: make_fi("BBBBUS33XXX"),
                    cdtr_agt_acct: None,
                    cdtr: make_party("Bob Jones"),
                    cdtr_acct: None,
                    ultmt_cdtr: None,
                    instr_for_cdtr_agt: vec![],
                    instr_for_nxt_agt: vec![],
                    purp: None,
                    rgltry_rptg: vec![],
                    tax: None,
                    rltd_rmt_inf: vec![],
                    rmt_inf: None,
                    splmtry_data: vec![],
                }],
                splmtry_data: vec![],
            },
        }
    }

    #[test]
    fn roundtrip() {
        let original = make_doc();
        let xml = ser::to_string(&original).expect("serialization must succeed");
        assert!(
            xml.contains("PACS008-TEST-001"),
            "MsgId must appear in XML: {xml}"
        );
        assert!(
            xml.contains("AAAAGB2LXXX"),
            "debtor agent BIC must appear in XML: {xml}"
        );
        let roundtripped: Document = de::from_str(&xml).expect("deserialization must succeed");
        assert_eq!(
            original, roundtripped,
            "pacs.008 roundtrip must be identity"
        );
    }

    #[test]
    fn envelope_detect() {
        let xml = fixture("pacs/pacs_008_001_13_minimal.xml");
        let id = envelope::detect_message_type(&xml).expect("must detect namespace");
        assert_eq!(id.dotted(), "pacs.008.001.13");
        assert_eq!(id.family, "pacs");
        assert_eq!(id.msg_id, "008");
        assert_eq!(id.version, "13");
    }

    #[test]
    fn fixture_parses() {
        let xml = fixture("pacs/pacs_008_001_13_minimal.xml");
        let doc: Document = de::from_str(&xml).expect("pacs.008 fixture must parse");
        assert_eq!(
            doc.fi_to_fi_cstmr_cdt_trf.grp_hdr.msg_id.0,
            "PACS008-20240101-001"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// pacs.004.001.11 — Payment Return
// ─────────────────────────────────────────────────────────────────────────────

mod pacs_004 {
    use super::*;
    use mx20022_model::generated::pacs::pacs_004_001_11::{
        Document, GroupHeader99, ISODateTime, Max15NumericText, Max35Text, PaymentReturnV11,
        SettlementInstruction11, SettlementMethod1Code,
    };

    fn make_doc() -> Document {
        Document {
            pmt_rtr: PaymentReturnV11 {
                grp_hdr: GroupHeader99 {
                    msg_id: Max35Text("PACS004-TEST-001".to_owned()),
                    cre_dt_tm: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
                    authstn: vec![],
                    btch_bookg: None,
                    nb_of_txs: Max15NumericText("1".to_owned()),
                    ctrl_sum: None,
                    grp_rtr: None,
                    ttl_rtrd_intr_bk_sttlm_amt: None,
                    intr_bk_sttlm_dt: None,
                    sttlm_inf: SettlementInstruction11 {
                        sttlm_mtd: SettlementMethod1Code::Clrg,
                        sttlm_acct: None,
                        clr_sys: None,
                        instg_rmbrsmnt_agt: None,
                        instg_rmbrsmnt_agt_acct: None,
                        instd_rmbrsmnt_agt: None,
                        instd_rmbrsmnt_agt_acct: None,
                        thrd_rmbrsmnt_agt: None,
                        thrd_rmbrsmnt_agt_acct: None,
                    },
                    pmt_tp_inf: None,
                    instg_agt: None,
                    instd_agt: None,
                },
                orgnl_grp_inf: None,
                tx_inf: vec![],
                splmtry_data: vec![],
            },
        }
    }

    #[test]
    fn roundtrip() {
        let original = make_doc();
        let xml = ser::to_string(&original).expect("serialization must succeed");
        assert!(
            xml.contains("PACS004-TEST-001"),
            "MsgId must appear in XML: {xml}"
        );
        let roundtripped: Document = de::from_str(&xml).expect("deserialization must succeed");
        assert_eq!(
            original, roundtripped,
            "pacs.004 roundtrip must be identity"
        );
    }

    #[test]
    fn envelope_detect() {
        let xml = fixture("pacs/pacs_004_001_11_minimal.xml");
        let id = envelope::detect_message_type(&xml).expect("must detect namespace");
        assert_eq!(id.dotted(), "pacs.004.001.11");
        assert_eq!(id.family, "pacs");
        assert_eq!(id.msg_id, "004");
        assert_eq!(id.version, "11");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// pacs.009.001.10 — FI Credit Transfer
// ─────────────────────────────────────────────────────────────────────────────

mod pacs_009 {
    use super::*;
    use mx20022_model::generated::pacs::pacs_009_001_10::{
        ActiveCurrencyAndAmount, ActiveCurrencyAndAmountSimpleType, ActiveCurrencyCode,
        BICFIDec2014Identifier, BranchAndFinancialInstitutionIdentification6,
        CreditTransferTransaction56, Document, FinancialInstitutionCreditTransferV10,
        FinancialInstitutionIdentification18, GroupHeader96, ISODateTime, Max15NumericText,
        Max35Text, PaymentIdentification13, SettlementInstruction11, SettlementMethod1Code,
    };

    fn make_fi(bic: &str) -> BranchAndFinancialInstitutionIdentification6 {
        BranchAndFinancialInstitutionIdentification6 {
            fin_instn_id: FinancialInstitutionIdentification18 {
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

    fn make_doc() -> Document {
        Document {
            fi_cdt_trf: FinancialInstitutionCreditTransferV10 {
                grp_hdr: GroupHeader96 {
                    msg_id: Max35Text("PACS009-TEST-001".to_owned()),
                    cre_dt_tm: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
                    btch_bookg: None,
                    nb_of_txs: Max15NumericText("1".to_owned()),
                    ctrl_sum: None,
                    ttl_intr_bk_sttlm_amt: None,
                    intr_bk_sttlm_dt: None,
                    sttlm_inf: SettlementInstruction11 {
                        sttlm_mtd: SettlementMethod1Code::Clrg,
                        sttlm_acct: None,
                        clr_sys: None,
                        instg_rmbrsmnt_agt: None,
                        instg_rmbrsmnt_agt_acct: None,
                        instd_rmbrsmnt_agt: None,
                        instd_rmbrsmnt_agt_acct: None,
                        thrd_rmbrsmnt_agt: None,
                        thrd_rmbrsmnt_agt_acct: None,
                    },
                    pmt_tp_inf: None,
                    instg_agt: None,
                    instd_agt: None,
                },
                cdt_trf_tx_inf: vec![CreditTransferTransaction56 {
                    pmt_id: PaymentIdentification13 {
                        instr_id: None,
                        end_to_end_id: Max35Text("E2E-TEST-001".to_owned()),
                        tx_id: None,
                        uetr: None,
                        clr_sys_ref: None,
                    },
                    pmt_tp_inf: None,
                    intr_bk_sttlm_amt: ActiveCurrencyAndAmount {
                        value: ActiveCurrencyAndAmountSimpleType("5000.00".to_owned()),
                        ccy: ActiveCurrencyCode("USD".to_owned()),
                    },
                    intr_bk_sttlm_dt: None,
                    sttlm_prty: None,
                    sttlm_tm_indctn: None,
                    sttlm_tm_req: None,
                    prvs_instg_agt1: None,
                    prvs_instg_agt1acct: None,
                    prvs_instg_agt2: None,
                    prvs_instg_agt2acct: None,
                    prvs_instg_agt3: None,
                    prvs_instg_agt3acct: None,
                    instg_agt: Some(make_fi("AAAAGB2LXXX")),
                    instd_agt: Some(make_fi("BBBBUS33XXX")),
                    intrmy_agt1: None,
                    intrmy_agt1acct: None,
                    intrmy_agt2: None,
                    intrmy_agt2acct: None,
                    intrmy_agt3: None,
                    intrmy_agt3acct: None,
                    ultmt_dbtr: None,
                    dbtr: make_fi("AAAAGB2LXXX"),
                    dbtr_acct: None,
                    dbtr_agt: None,
                    dbtr_agt_acct: None,
                    cdtr_agt: None,
                    cdtr_agt_acct: None,
                    cdtr: make_fi("BBBBUS33XXX"),
                    cdtr_acct: None,
                    ultmt_cdtr: None,
                    instr_for_cdtr_agt: vec![],
                    instr_for_nxt_agt: vec![],
                    purp: None,
                    rmt_inf: None,
                    undrlyg_cstmr_cdt_trf: None,
                    splmtry_data: vec![],
                }],
                splmtry_data: vec![],
            },
        }
    }

    #[test]
    fn roundtrip() {
        let original = make_doc();
        let xml = ser::to_string(&original).expect("serialization must succeed");
        assert!(
            xml.contains("PACS009-TEST-001"),
            "MsgId must appear in XML: {xml}"
        );
        assert!(
            xml.contains("AAAAGB2LXXX"),
            "sender BIC must appear in XML: {xml}"
        );
        let roundtripped: Document = de::from_str(&xml).expect("deserialization must succeed");
        assert_eq!(
            original, roundtripped,
            "pacs.009 roundtrip must be identity"
        );
    }

    #[test]
    fn envelope_detect() {
        let xml = fixture("pacs/pacs_009_001_10_minimal.xml");
        let id = envelope::detect_message_type(&xml).expect("must detect namespace");
        assert_eq!(id.dotted(), "pacs.009.001.10");
        assert_eq!(id.family, "pacs");
        assert_eq!(id.msg_id, "009");
        assert_eq!(id.version, "10");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// pacs.028.001.05 — FI to FI Payment Status Request
// ─────────────────────────────────────────────────────────────────────────────

mod pacs_028 {
    use super::*;
    use mx20022_model::generated::pacs::pacs_028_001_05::{
        Document, FIToFIPaymentStatusRequestV05, GroupHeader91, ISODateTime, Max35Text,
    };

    fn make_doc() -> Document {
        Document {
            fi_to_fi_pmt_sts_req: FIToFIPaymentStatusRequestV05 {
                grp_hdr: GroupHeader91 {
                    msg_id: Max35Text("PACS028-TEST-001".to_owned()),
                    cre_dt_tm: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
                    instg_agt: None,
                    instd_agt: None,
                },
                orgnl_grp_inf: vec![],
                tx_inf: vec![],
                splmtry_data: vec![],
            },
        }
    }

    #[test]
    fn roundtrip() {
        let original = make_doc();
        let xml = ser::to_string(&original).expect("serialization must succeed");
        assert!(
            xml.contains("PACS028-TEST-001"),
            "MsgId must appear in XML: {xml}"
        );
        let roundtripped: Document = de::from_str(&xml).expect("deserialization must succeed");
        assert_eq!(
            original, roundtripped,
            "pacs.028 roundtrip must be identity"
        );
    }

    #[test]
    fn envelope_detect() {
        let xml = fixture("pacs/pacs_028_001_05_minimal.xml");
        let id = envelope::detect_message_type(&xml).expect("must detect namespace");
        assert_eq!(id.dotted(), "pacs.028.001.05");
        assert_eq!(id.family, "pacs");
        assert_eq!(id.msg_id, "028");
        assert_eq!(id.version, "05");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// pain.001.001.11 — Customer Credit Transfer Initiation
// ─────────────────────────────────────────────────────────────────────────────

mod pain_001 {
    use super::*;
    use mx20022_model::common::ChoiceWrapper;
    use mx20022_model::generated::pain::pain_001_001_11::{
        AccountIdentification4Choice, ActiveOrHistoricCurrencyAndAmount,
        ActiveOrHistoricCurrencyAndAmountSimpleType, ActiveOrHistoricCurrencyCode,
        BICFIDec2014Identifier, BranchAndFinancialInstitutionIdentification6, CashAccount40,
        CreditTransferTransaction54, CustomerCreditTransferInitiationV11, DateAndDateTime2Choice,
        Document, FinancialInstitutionIdentification18, GroupHeader95, IBAN2007Identifier, ISODate,
        ISODateTime, Max140Text, Max15NumericText, Max35Text, PartyIdentification135,
        PaymentIdentification6, PaymentInstruction40, PaymentMethod3Code,
    };

    fn make_fi(bic: &str) -> BranchAndFinancialInstitutionIdentification6 {
        BranchAndFinancialInstitutionIdentification6 {
            fin_instn_id: FinancialInstitutionIdentification18 {
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

    fn make_account(iban: &str) -> CashAccount40 {
        CashAccount40 {
            id: Some(ChoiceWrapper::new(AccountIdentification4Choice::IBAN(
                IBAN2007Identifier(iban.to_owned()),
            ))),
            tp: None,
            ccy: None,
            nm: None,
            prxy: None,
        }
    }

    fn make_party(name: &str) -> PartyIdentification135 {
        PartyIdentification135 {
            nm: Some(Max140Text(name.to_owned())),
            pstl_adr: None,
            id: None,
            ctry_of_res: None,
            ctct_dtls: None,
        }
    }

    fn make_doc() -> Document {
        Document {
            cstmr_cdt_trf_initn: CustomerCreditTransferInitiationV11 {
                grp_hdr: GroupHeader95 {
                    msg_id: Max35Text("PAIN001-TEST-001".to_owned()),
                    cre_dt_tm: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
                    authstn: vec![],
                    nb_of_txs: Max15NumericText("1".to_owned()),
                    ctrl_sum: None,
                    initg_pty: make_party("ACME Corp"),
                    fwdg_agt: None,
                    initn_src: None,
                },
                pmt_inf: vec![PaymentInstruction40 {
                    pmt_inf_id: Max35Text("PMTINF-TEST-001".to_owned()),
                    pmt_mtd: PaymentMethod3Code::Trf,
                    reqd_advc_tp: None,
                    btch_bookg: None,
                    nb_of_txs: None,
                    ctrl_sum: None,
                    pmt_tp_inf: None,
                    reqd_exctn_dt: ChoiceWrapper::new(DateAndDateTime2Choice::Dt(ISODate(
                        "2024-01-02".to_owned(),
                    ))),
                    poolg_adjstmnt_dt: None,
                    dbtr: make_party("Alice Smith"),
                    dbtr_acct: make_account("GB82WEST12345698765432"),
                    dbtr_agt: make_fi("AAAAGB2LXXX"),
                    dbtr_agt_acct: None,
                    instr_for_dbtr_agt: None,
                    ultmt_dbtr: None,
                    chrg_br: None,
                    chrgs_acct: None,
                    chrgs_acct_agt: None,
                    cdt_trf_tx_inf: vec![CreditTransferTransaction54 {
                        pmt_id: PaymentIdentification6 {
                            instr_id: None,
                            end_to_end_id: Max35Text("E2E-TEST-001".to_owned()),
                            uetr: None,
                        },
                        pmt_tp_inf: None,
                        amt: ChoiceWrapper::new(
                            mx20022_model::generated::pain::pain_001_001_11::AmountType4Choice::InstdAmt(
                                ActiveOrHistoricCurrencyAndAmount {
                                    value: ActiveOrHistoricCurrencyAndAmountSimpleType(
                                        "1000.00".to_owned(),
                                    ),
                                    ccy: ActiveOrHistoricCurrencyCode("USD".to_owned()),
                                },
                            ),
                        ),
                        xchg_rate_inf: None,
                        chrg_br: None,
                        mndt_rltd_inf: None,
                        chq_instr: None,
                        ultmt_dbtr: None,
                        intrmy_agt1: None,
                        intrmy_agt1acct: None,
                        intrmy_agt2: None,
                        intrmy_agt2acct: None,
                        intrmy_agt3: None,
                        intrmy_agt3acct: None,
                        cdtr_agt: None,
                        cdtr_agt_acct: None,
                        cdtr: Some(make_party("Bob Jones")),
                        cdtr_acct: Some(make_account("DE89370400440532013000")),
                        ultmt_cdtr: None,
                        instr_for_cdtr_agt: vec![],
                        instr_for_dbtr_agt: None,
                        purp: None,
                        rgltry_rptg: vec![],
                        tax: None,
                        rltd_rmt_inf: vec![],
                        rmt_inf: None,
                        splmtry_data: vec![],
                    }],
                }],
                splmtry_data: vec![],
            },
        }
    }

    #[test]
    fn roundtrip() {
        let original = make_doc();
        let xml = ser::to_string(&original).expect("serialization must succeed");
        assert!(
            xml.contains("PAIN001-TEST-001"),
            "MsgId must appear in XML: {xml}"
        );
        assert!(
            xml.contains("Alice Smith"),
            "debtor name must appear in XML: {xml}"
        );
        let roundtripped: Document = de::from_str(&xml).expect("deserialization must succeed");
        assert_eq!(
            original, roundtripped,
            "pain.001 roundtrip must be identity"
        );
    }

    #[test]
    fn envelope_detect() {
        let xml = fixture("pain/pain_001_001_11_minimal.xml");
        let id = envelope::detect_message_type(&xml).expect("must detect namespace");
        assert_eq!(id.dotted(), "pain.001.001.11");
        assert_eq!(id.family, "pain");
        assert_eq!(id.msg_id, "001");
        assert_eq!(id.version, "11");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// pain.002.001.13 — Customer Payment Status Report
// ─────────────────────────────────────────────────────────────────────────────

mod pain_002 {
    use super::*;
    use mx20022_model::generated::pain::pain_002_001_13::{
        CustomerPaymentStatusReportV13, Document, GroupHeader86, ISODateTime, Max35Text,
        OriginalGroupHeader17,
    };

    fn make_doc() -> Document {
        Document {
            cstmr_pmt_sts_rpt: CustomerPaymentStatusReportV13 {
                grp_hdr: GroupHeader86 {
                    msg_id: Max35Text("PAIN002-TEST-001".to_owned()),
                    cre_dt_tm: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
                    initg_pty: None,
                    fwdg_agt: None,
                    dbtr_agt: None,
                    cdtr_agt: None,
                },
                orgnl_grp_inf_and_sts: OriginalGroupHeader17 {
                    orgnl_msg_id: Max35Text("PAIN001-20231201-001".to_owned()),
                    orgnl_msg_nm_id: Max35Text("pain.001.001.11".to_owned()),
                    orgnl_cre_dt_tm: None,
                    orgnl_nb_of_txs: None,
                    orgnl_ctrl_sum: None,
                    grp_sts: None,
                    sts_rsn_inf: vec![],
                    nb_of_txs_per_sts: vec![],
                },
                orgnl_pmt_inf_and_sts: vec![],
                splmtry_data: vec![],
            },
        }
    }

    #[test]
    fn roundtrip() {
        let original = make_doc();
        let xml = ser::to_string(&original).expect("serialization must succeed");
        assert!(
            xml.contains("PAIN002-TEST-001"),
            "MsgId must appear in XML: {xml}"
        );
        let roundtripped: Document = de::from_str(&xml).expect("deserialization must succeed");
        assert_eq!(
            original, roundtripped,
            "pain.002 roundtrip must be identity"
        );
    }

    #[test]
    fn envelope_detect() {
        let xml = fixture("pain/pain_002_001_13_minimal.xml");
        let id = envelope::detect_message_type(&xml).expect("must detect namespace");
        assert_eq!(id.dotted(), "pain.002.001.13");
        assert_eq!(id.family, "pain");
        assert_eq!(id.msg_id, "002");
        assert_eq!(id.version, "13");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// pain.013.001.09 — Creditor Payment Activation Request
// ─────────────────────────────────────────────────────────────────────────────

mod pain_013 {
    use super::*;
    use mx20022_model::common::ChoiceWrapper;
    use mx20022_model::generated::pain::pain_013_001_09::{
        BICFIDec2014Identifier, BranchAndFinancialInstitutionIdentification6,
        CreditorPaymentActivationRequestV09, DateAndDateTime2Choice, Document,
        FinancialInstitutionIdentification18, GroupHeader78, ISODate, ISODateTime, Max140Text,
        Max15NumericText, Max35Text, PartyIdentification135, PaymentInstruction41,
        PaymentMethod7Code,
    };

    fn make_fi(bic: &str) -> BranchAndFinancialInstitutionIdentification6 {
        BranchAndFinancialInstitutionIdentification6 {
            fin_instn_id: FinancialInstitutionIdentification18 {
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

    fn make_doc() -> Document {
        Document {
            cdtr_pmt_actvtn_req: CreditorPaymentActivationRequestV09 {
                grp_hdr: GroupHeader78 {
                    msg_id: Max35Text("PAIN013-TEST-001".to_owned()),
                    cre_dt_tm: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
                    nb_of_txs: Max15NumericText("1".to_owned()),
                    ctrl_sum: None,
                    initg_pty: PartyIdentification135 {
                        nm: Some(Max140Text("Bob Jones".to_owned())),
                        pstl_adr: None,
                        id: None,
                        ctry_of_res: None,
                        ctct_dtls: None,
                    },
                },
                pmt_inf: vec![PaymentInstruction41 {
                    pmt_inf_id: None,
                    pmt_mtd: PaymentMethod7Code::Trf,
                    reqd_advc_tp: None,
                    pmt_tp_inf: None,
                    reqd_exctn_dt: ChoiceWrapper::new(DateAndDateTime2Choice::Dt(ISODate(
                        "2024-01-02".to_owned(),
                    ))),
                    xpry_dt: None,
                    pmt_cond: None,
                    dbtr: PartyIdentification135 {
                        nm: Some(Max140Text("Bob Jones".to_owned())),
                        pstl_adr: None,
                        id: None,
                        ctry_of_res: None,
                        ctct_dtls: None,
                    },
                    dbtr_acct: None,
                    dbtr_agt: make_fi("AAAAGB2LXXX"),
                    ultmt_dbtr: None,
                    chrg_br: None,
                    cdt_trf_tx: vec![],
                }],
                splmtry_data: vec![],
            },
        }
    }

    #[test]
    fn roundtrip() {
        let original = make_doc();
        let xml = ser::to_string(&original).expect("serialization must succeed");
        assert!(
            xml.contains("PAIN013-TEST-001"),
            "MsgId must appear in XML: {xml}"
        );
        let roundtripped: Document = de::from_str(&xml).expect("deserialization must succeed");
        assert_eq!(
            original, roundtripped,
            "pain.013 roundtrip must be identity"
        );
    }

    #[test]
    fn envelope_detect() {
        let xml = fixture("pain/pain_013_001_09_minimal.xml");
        let id = envelope::detect_message_type(&xml).expect("must detect namespace");
        assert_eq!(id.dotted(), "pain.013.001.09");
        assert_eq!(id.family, "pain");
        assert_eq!(id.msg_id, "013");
        assert_eq!(id.version, "09");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// camt.053.001.11 — Bank to Customer Statement
// ─────────────────────────────────────────────────────────────────────────────

mod camt_053 {
    use super::*;
    use mx20022_model::generated::camt::camt_053_001_11::{
        BankToCustomerStatementV11, Document, GroupHeader81, ISODateTime, Max35Text,
    };

    fn make_doc() -> Document {
        Document {
            bk_to_cstmr_stmt: BankToCustomerStatementV11 {
                grp_hdr: GroupHeader81 {
                    msg_id: Max35Text("CAMT053-TEST-001".to_owned()),
                    cre_dt_tm: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
                    msg_rcpt: None,
                    msg_pgntn: None,
                    orgnl_biz_qry: None,
                    addtl_inf: None,
                },
                stmt: vec![],
                splmtry_data: vec![],
            },
        }
    }

    #[test]
    fn roundtrip() {
        let original = make_doc();
        let xml = ser::to_string(&original).expect("serialization must succeed");
        assert!(
            xml.contains("CAMT053-TEST-001"),
            "MsgId must appear in XML: {xml}"
        );
        let roundtripped: Document = de::from_str(&xml).expect("deserialization must succeed");
        assert_eq!(
            original, roundtripped,
            "camt.053 roundtrip must be identity"
        );
    }

    #[test]
    fn envelope_detect() {
        let xml = fixture("camt/camt_053_001_11_minimal.xml");
        let id = envelope::detect_message_type(&xml).expect("must detect namespace");
        assert_eq!(id.dotted(), "camt.053.001.11");
        assert_eq!(id.family, "camt");
        assert_eq!(id.msg_id, "053");
        assert_eq!(id.version, "11");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// camt.054.001.11 — Bank to Customer Notification
// ─────────────────────────────────────────────────────────────────────────────

mod camt_054 {
    use super::*;
    use mx20022_model::generated::camt::camt_054_001_11::{
        BankToCustomerDebitCreditNotificationV11, Document, GroupHeader81, ISODateTime, Max35Text,
    };

    fn make_doc() -> Document {
        Document {
            bk_to_cstmr_dbt_cdt_ntfctn: BankToCustomerDebitCreditNotificationV11 {
                grp_hdr: GroupHeader81 {
                    msg_id: Max35Text("CAMT054-TEST-001".to_owned()),
                    cre_dt_tm: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
                    msg_rcpt: None,
                    msg_pgntn: None,
                    orgnl_biz_qry: None,
                    addtl_inf: None,
                },
                ntfctn: vec![],
                splmtry_data: vec![],
            },
        }
    }

    #[test]
    fn roundtrip() {
        let original = make_doc();
        let xml = ser::to_string(&original).expect("serialization must succeed");
        assert!(
            xml.contains("CAMT054-TEST-001"),
            "MsgId must appear in XML: {xml}"
        );
        let roundtripped: Document = de::from_str(&xml).expect("deserialization must succeed");
        assert_eq!(
            original, roundtripped,
            "camt.054 roundtrip must be identity"
        );
    }

    #[test]
    fn envelope_detect() {
        let xml = fixture("camt/camt_054_001_11_minimal.xml");
        let id = envelope::detect_message_type(&xml).expect("must detect namespace");
        assert_eq!(id.dotted(), "camt.054.001.11");
        assert_eq!(id.family, "camt");
        assert_eq!(id.msg_id, "054");
        assert_eq!(id.version, "11");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// camt.056.001.11 — Payment Cancellation Request
// ─────────────────────────────────────────────────────────────────────────────

mod camt_056 {
    use super::*;
    use mx20022_model::common::ChoiceWrapper;
    use mx20022_model::generated::camt::camt_056_001_11::{
        BICFIDec2014Identifier, BranchAndFinancialInstitutionIdentification8, CaseAssignment6,
        Document, FIToFIPaymentCancellationRequestV11, FinancialInstitutionIdentification23,
        ISODateTime, Max35Text, Party50Choice,
    };

    fn make_fi_agent(bic: &str) -> ChoiceWrapper<Party50Choice> {
        ChoiceWrapper::new(Party50Choice::Agt(
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
            },
        ))
    }

    fn make_doc() -> Document {
        Document {
            fi_to_fi_pmt_cxl_req: FIToFIPaymentCancellationRequestV11 {
                assgnmt: CaseAssignment6 {
                    id: Max35Text("CXLREQ-TEST-001".to_owned()),
                    assgnr: make_fi_agent("AAAAGB2LXXX"),
                    assgne: make_fi_agent("BBBBUS33XXX"),
                    cre_dt_tm: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
                },
                case: None,
                ctrl_data: None,
                undrlyg: vec![],
                splmtry_data: vec![],
            },
        }
    }

    #[test]
    fn roundtrip() {
        let original = make_doc();
        let xml = ser::to_string(&original).expect("serialization must succeed");
        assert!(
            xml.contains("CXLREQ-TEST-001"),
            "assignment ID must appear in XML: {xml}"
        );
        assert!(
            xml.contains("AAAAGB2LXXX"),
            "assigner BIC must appear in XML: {xml}"
        );
        let roundtripped: Document = de::from_str(&xml).expect("deserialization must succeed");
        assert_eq!(
            original, roundtripped,
            "camt.056 roundtrip must be identity"
        );
    }

    #[test]
    fn envelope_detect() {
        let xml = fixture("camt/camt_056_001_11_minimal.xml");
        let id = envelope::detect_message_type(&xml).expect("must detect namespace");
        assert_eq!(id.dotted(), "camt.056.001.11");
        assert_eq!(id.family, "camt");
        assert_eq!(id.msg_id, "056");
        assert_eq!(id.version, "11");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// camt.029.001.12 — Resolution of Investigation
// ─────────────────────────────────────────────────────────────────────────────

mod camt_029 {
    use super::*;
    use mx20022_model::common::ChoiceWrapper;
    use mx20022_model::generated::camt::camt_029_001_12::{
        BICFIDec2014Identifier, BranchAndFinancialInstitutionIdentification6, CaseAssignment5,
        Document, FinancialInstitutionIdentification18, ISODateTime, InvestigationStatus5Choice,
        Max35Text, Party40Choice, ResolutionOfInvestigationV12,
    };

    fn make_fi_agent(bic: &str) -> ChoiceWrapper<Party40Choice> {
        ChoiceWrapper::new(Party40Choice::Agt(
            BranchAndFinancialInstitutionIdentification6 {
                fin_instn_id: FinancialInstitutionIdentification18 {
                    bicfi: Some(BICFIDec2014Identifier(bic.to_owned())),
                    clr_sys_mmb_id: None,
                    lei: None,
                    nm: None,
                    pstl_adr: None,
                    othr: None,
                },
                brnch_id: None,
            },
        ))
    }

    fn make_doc() -> Document {
        Document {
            rsltn_of_invstgtn: ResolutionOfInvestigationV12 {
                assgnmt: CaseAssignment5 {
                    id: Max35Text("RSLINV-TEST-001".to_owned()),
                    assgnr: make_fi_agent("AAAAGB2LXXX"),
                    assgne: make_fi_agent("BBBBUS33XXX"),
                    cre_dt_tm: ISODateTime("2024-01-01T12:00:00Z".to_owned()),
                },
                rslvd_case: None,
                sts: ChoiceWrapper::new(InvestigationStatus5Choice::AssgnmtCxlConf(
                    mx20022_model::generated::camt::camt_029_001_12::YesNoIndicator(true),
                )),
                cxl_dtls: vec![],
                mod_dtls: None,
                clm_non_rct_dtls: None,
                stmt_dtls: None,
                crrctn_tx: None,
                rsltn_rltd_inf: None,
                splmtry_data: vec![],
            },
        }
    }

    #[test]
    fn roundtrip() {
        let original = make_doc();
        let xml = ser::to_string(&original).expect("serialization must succeed");
        assert!(
            xml.contains("RSLINV-TEST-001"),
            "assignment ID must appear in XML: {xml}"
        );
        assert!(
            xml.contains("AAAAGB2LXXX"),
            "assigner BIC must appear in XML: {xml}"
        );
        let roundtripped: Document = de::from_str(&xml).expect("deserialization must succeed");
        assert_eq!(
            original, roundtripped,
            "camt.029 roundtrip must be identity"
        );
    }

    #[test]
    fn envelope_detect() {
        let xml = fixture("camt/camt_029_001_12_minimal.xml");
        let id = envelope::detect_message_type(&xml).expect("must detect namespace");
        assert_eq!(id.dotted(), "camt.029.001.12");
        assert_eq!(id.family, "camt");
        assert_eq!(id.msg_id, "029");
        assert_eq!(id.version, "12");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Fixture-file parse tests: verify that the minimal XML fixtures deserialize
// without error (namespace is stripped by quick-xml during deserialization).
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn fixture_pacs_002_parses() {
    use mx20022_model::generated::pacs::pacs_002_001_14::Document;
    let xml = fixture("pacs/pacs_002_001_14_minimal.xml");
    let doc: Document = de::from_str(&xml).expect("pacs.002 fixture must parse");
    assert_eq!(
        doc.fi_to_fi_pmt_sts_rpt.grp_hdr.msg_id.0,
        "PACS002-20240101-001"
    );
}

#[test]
fn fixture_pacs_004_parses() {
    use mx20022_model::generated::pacs::pacs_004_001_11::Document;
    let xml = fixture("pacs/pacs_004_001_11_minimal.xml");
    let doc: Document = de::from_str(&xml).expect("pacs.004 fixture must parse");
    assert_eq!(doc.pmt_rtr.grp_hdr.msg_id.0, "PACS004-20240101-001");
}

#[test]
fn fixture_pacs_009_parses() {
    use mx20022_model::generated::pacs::pacs_009_001_10::Document;
    let xml = fixture("pacs/pacs_009_001_10_minimal.xml");
    let doc: Document = de::from_str(&xml).expect("pacs.009 fixture must parse");
    assert_eq!(doc.fi_cdt_trf.grp_hdr.msg_id.0, "PACS009-20240101-001");
}

#[test]
fn fixture_pacs_028_parses() {
    use mx20022_model::generated::pacs::pacs_028_001_05::Document;
    let xml = fixture("pacs/pacs_028_001_05_minimal.xml");
    let doc: Document = de::from_str(&xml).expect("pacs.028 fixture must parse");
    assert_eq!(
        doc.fi_to_fi_pmt_sts_req.grp_hdr.msg_id.0,
        "PACS028-20240101-001"
    );
}

#[test]
fn fixture_pain_001_parses() {
    use mx20022_model::generated::pain::pain_001_001_11::Document;
    let xml = fixture("pain/pain_001_001_11_minimal.xml");
    let doc: Document = de::from_str(&xml).expect("pain.001 fixture must parse");
    assert_eq!(
        doc.cstmr_cdt_trf_initn.grp_hdr.msg_id.0,
        "PAIN001-20240101-001"
    );
}

#[test]
fn fixture_pain_002_parses() {
    use mx20022_model::generated::pain::pain_002_001_13::Document;
    let xml = fixture("pain/pain_002_001_13_minimal.xml");
    let doc: Document = de::from_str(&xml).expect("pain.002 fixture must parse");
    assert_eq!(
        doc.cstmr_pmt_sts_rpt.grp_hdr.msg_id.0,
        "PAIN002-20240101-001"
    );
}

#[test]
fn fixture_pain_013_parses() {
    use mx20022_model::generated::pain::pain_013_001_09::Document;
    let xml = fixture("pain/pain_013_001_09_minimal.xml");
    let doc: Document = de::from_str(&xml).expect("pain.013 fixture must parse");
    assert_eq!(
        doc.cdtr_pmt_actvtn_req.grp_hdr.msg_id.0,
        "PAIN013-20240101-001"
    );
}

#[test]
fn fixture_camt_053_parses() {
    use mx20022_model::generated::camt::camt_053_001_11::Document;
    let xml = fixture("camt/camt_053_001_11_minimal.xml");
    let doc: Document = de::from_str(&xml).expect("camt.053 fixture must parse");
    assert_eq!(
        doc.bk_to_cstmr_stmt.grp_hdr.msg_id.0,
        "CAMT053-20240101-001"
    );
}

#[test]
fn fixture_camt_054_parses() {
    use mx20022_model::generated::camt::camt_054_001_11::Document;
    let xml = fixture("camt/camt_054_001_11_minimal.xml");
    let doc: Document = de::from_str(&xml).expect("camt.054 fixture must parse");
    assert_eq!(
        doc.bk_to_cstmr_dbt_cdt_ntfctn.grp_hdr.msg_id.0,
        "CAMT054-20240101-001"
    );
}

#[test]
fn fixture_camt_056_parses() {
    use mx20022_model::generated::camt::camt_056_001_11::Document;
    let xml = fixture("camt/camt_056_001_11_minimal.xml");
    let doc: Document = de::from_str(&xml).expect("camt.056 fixture must parse");
    assert_eq!(doc.fi_to_fi_pmt_cxl_req.assgnmt.id.0, "CXLREQ-20240101-001");
}

#[test]
fn fixture_camt_029_parses() {
    use mx20022_model::generated::camt::camt_029_001_12::Document;
    let xml = fixture("camt/camt_029_001_12_minimal.xml");
    let doc: Document = de::from_str(&xml).expect("camt.029 fixture must parse");
    assert_eq!(doc.rsltn_of_invstgtn.assgnmt.id.0, "RSLINV-20240101-001");
}
