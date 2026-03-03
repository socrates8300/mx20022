//! Translation from MT103 Customer Credit Transfer to pacs.008.001.13.

use mx20022_model::generated::pacs::pacs_008_001_13 as pacs008;

use crate::mappings::{
    error::{TranslationError, TranslationResult, TranslationWarnings},
    helpers::{
        account_to_cash_account, charges_to_code, empty_fi_id, party_to_fi_id, party_to_party_id,
    },
};
use crate::mt::fields::mt103::Mt103;

/// Translate an [`Mt103`] into a `pacs.008.001.13` [`Document`].
///
/// `msg_id` is used for `GrpHdr.MsgId` (max 35 chars).
/// `creation_time` is used for `GrpHdr.CreDtTm` (ISO datetime, e.g.
/// `"2023-06-15T10:00:00"`).
///
/// # Errors
///
/// Returns [`TranslationError`] when a mandatory field cannot be mapped.
pub fn mt103_to_pacs008(
    mt103: &Mt103,
    msg_id: &str,
    creation_time: &str,
) -> Result<TranslationResult<pacs008::Document>, TranslationError> {
    let mut warnings = TranslationWarnings::default();

    // ------------------------------------------------------------------
    // GroupHeader131
    // ------------------------------------------------------------------
    let sttlm_inf = pacs008::SettlementInstruction15 {
        sttlm_mtd: pacs008::SettlementMethod1Code::Inda,
        sttlm_acct: None,
        clr_sys: None,
        instg_rmbrsmnt_agt: None,
        instg_rmbrsmnt_agt_acct: None,
        instd_rmbrsmnt_agt: None,
        instd_rmbrsmnt_agt_acct: None,
        thrd_rmbrsmnt_agt: None,
        thrd_rmbrsmnt_agt_acct: None,
    };

    let grp_hdr = pacs008::GroupHeader131::builder()
        .msg_id(pacs008::Max35Text(msg_id.to_string()))
        .cre_dt_tm(pacs008::ISODateTime(creation_time.to_string()))
        .nb_of_txs(pacs008::Max15NumericText("1".to_string()))
        .sttlm_inf(sttlm_inf)
        .build()?;

    // ------------------------------------------------------------------
    // PaymentIdentification13
    // ------------------------------------------------------------------
    let pmt_id = pacs008::PaymentIdentification13 {
        instr_id: Some(pacs008::Max35Text(mt103.senders_reference.clone())),
        end_to_end_id: pacs008::Max35Text(mt103.senders_reference.clone()),
        tx_id: None,
        uetr: None,
        clr_sys_ref: None,
    };

    // ------------------------------------------------------------------
    // IntrBkSttlmAmt
    // ------------------------------------------------------------------
    let intr_bk_sttlm_amt = pacs008::ActiveCurrencyAndAmount {
        value: pacs008::ActiveCurrencyAndAmountSimpleType(mt103.interbank_settled_amount.clone()),
        ccy: pacs008::ActiveCurrencyCode(mt103.currency.clone()),
    };

    // ------------------------------------------------------------------
    // IntrBkSttlmDt
    // ------------------------------------------------------------------
    let intr_bk_sttlm_dt = Some(pacs008::ISODate(mt103.value_date.clone()));

    // ------------------------------------------------------------------
    // Charge bearer
    // ------------------------------------------------------------------
    let chrg_br = charges_to_code(&mt103.details_of_charges);

    // ------------------------------------------------------------------
    // Dbtr (ordering customer)
    // ------------------------------------------------------------------
    let dbtr = party_to_party_id(&mt103.ordering_customer);
    let dbtr_acct = mt103
        .ordering_customer
        .account
        .as_ref()
        .and_then(account_to_cash_account);

    // ------------------------------------------------------------------
    // DbtrAgt (ordering institution / fallback to empty)
    // ------------------------------------------------------------------
    let dbtr_agt = mt103
        .ordering_institution
        .as_ref()
        .map_or_else(empty_fi_id, party_to_fi_id);

    // ------------------------------------------------------------------
    // CdtrAgt (account with institution)
    // ------------------------------------------------------------------
    let cdtr_agt = mt103
        .account_with_institution
        .as_ref()
        .map_or_else(empty_fi_id, party_to_fi_id);

    // ------------------------------------------------------------------
    // Cdtr (beneficiary)
    // ------------------------------------------------------------------
    let cdtr = party_to_party_id(&mt103.beneficiary);
    let cdtr_acct = mt103
        .beneficiary
        .account
        .as_ref()
        .and_then(account_to_cash_account);

    // ------------------------------------------------------------------
    // IntrmyAgt1 (intermediary)
    // ------------------------------------------------------------------
    let intrmy_agt1 = mt103.intermediary.as_ref().map(party_to_fi_id);

    // ------------------------------------------------------------------
    // InstdAmt (:33B:)
    // ------------------------------------------------------------------
    let instd_amt =
        mt103
            .instructed_amount
            .as_ref()
            .map(|a| pacs008::ActiveOrHistoricCurrencyAndAmount {
                value: pacs008::ActiveOrHistoricCurrencyAndAmountSimpleType(a.value.clone()),
                ccy: pacs008::ActiveOrHistoricCurrencyCode(a.currency.clone()),
            });

    // ------------------------------------------------------------------
    // XchgRate (:36:)
    // ------------------------------------------------------------------
    let xchg_rate = mt103
        .exchange_rate
        .as_ref()
        .map(|r| pacs008::BaseOneRate(r.clone()));

    // ------------------------------------------------------------------
    // RmtInf (:70:)
    // ------------------------------------------------------------------
    let rmt_inf = mt103.remittance_info.as_ref().map(|info| {
        // Split into 140-char chunks for Ustrd lines
        let chunks: Vec<pacs008::Max140Text> = info
            .chars()
            .collect::<Vec<_>>()
            .chunks(140)
            .map(|c| pacs008::Max140Text(c.iter().collect()))
            .collect();
        pacs008::RemittanceInformation22 {
            ustrd: chunks,
            strd: vec![],
        }
    });

    // ------------------------------------------------------------------
    // Warn about unmapped fields
    // ------------------------------------------------------------------
    if mt103.senders_correspondent.is_some() {
        warnings.add(":53:", "senders_correspondent not mapped to pacs.008");
    }
    if mt103.receivers_correspondent.is_some() {
        warnings.add(":54:", "receivers_correspondent not mapped to pacs.008");
    }
    if mt103.sender_to_receiver_info.is_some() {
        warnings.add(":72:", "sender_to_receiver_info not mapped");
    }
    if !mt103.senders_charges.is_empty() || mt103.receivers_charges.is_some() {
        warnings.add(":71F/71G:", "senders/receivers charges not mapped");
    }

    // ------------------------------------------------------------------
    // Assemble CreditTransferTransaction70
    // ------------------------------------------------------------------
    let tx = pacs008::CreditTransferTransaction70 {
        pmt_id,
        pmt_tp_inf: None,
        intr_bk_sttlm_amt,
        intr_bk_sttlm_dt,
        sttlm_prty: None,
        sttlm_tm_indctn: None,
        sttlm_tm_req: None,
        addtl_dt_tm: None,
        instd_amt,
        xchg_rate,
        agrd_rate: None,
        chrg_br,
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
        intrmy_agt1,
        intrmy_agt1acct: None,
        intrmy_agt2: None,
        intrmy_agt2acct: None,
        intrmy_agt3: None,
        intrmy_agt3acct: None,
        ultmt_dbtr: None,
        initg_pty: None,
        dbtr,
        dbtr_acct,
        dbtr_agt,
        dbtr_agt_acct: None,
        cdtr_agt,
        cdtr_agt_acct: None,
        cdtr,
        cdtr_acct,
        ultmt_cdtr: None,
        instr_for_cdtr_agt: vec![],
        instr_for_nxt_agt: vec![],
        purp: None,
        rgltry_rptg: vec![],
        tax: None,
        rltd_rmt_inf: vec![],
        rmt_inf,
        splmtry_data: vec![],
    };

    let fi_to_fi = pacs008::FIToFICustomerCreditTransferV13 {
        grp_hdr,
        cdt_trf_tx_inf: vec![tx],
        splmtry_data: vec![],
    };

    let doc = pacs008::Document {
        fi_to_fi_cstmr_cdt_trf: fi_to_fi,
    };

    Ok(TranslationResult {
        message: doc,
        warnings,
    })
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mt::fields::mt103::parse_mt103;
    use crate::mt::parser::parse;

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

    #[test]
    fn test_mt103_to_pacs008_basic() {
        let msg = parse(MT103_RAW).unwrap();
        let mt103 = parse_mt103(&msg.block4).unwrap();
        let result = mt103_to_pacs008(&mt103, "MSG001", "2023-06-15T10:00:00").unwrap();
        let doc = &result.message;
        let tx = &doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf[0];

        assert_eq!(doc.fi_to_fi_cstmr_cdt_trf.grp_hdr.msg_id.0, "MSG001");
        assert_eq!(tx.pmt_id.end_to_end_id.0, "REFERENCE123");
        assert_eq!(tx.intr_bk_sttlm_amt.value.0, "1000.50");
        assert_eq!(tx.intr_bk_sttlm_amt.ccy.0, "EUR");
        assert_eq!(
            tx.intr_bk_sttlm_dt.as_ref().map(|d| d.0.as_str()),
            Some("2023-06-15")
        );
        assert!(matches!(tx.chrg_br, pacs008::ChargeBearerType1Code::Shar));
        assert_eq!(tx.dbtr.nm.as_ref().map(|n| n.0.as_str()), Some("JOHN DOE"));
        assert_eq!(
            tx.cdtr.nm.as_ref().map(|n| n.0.as_str()),
            Some("JANE SMITH")
        );
    }

    #[test]
    fn test_mt103_to_pacs008_remittance_info() {
        let msg = parse(MT103_RAW).unwrap();
        let mt103 = parse_mt103(&msg.block4).unwrap();
        let result = mt103_to_pacs008(&mt103, "MSG002", "2023-06-15T10:00:00").unwrap();
        let tx = &result.message.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf[0];
        let rmt = tx.rmt_inf.as_ref().unwrap();
        assert!(!rmt.ustrd.is_empty());
        assert_eq!(rmt.ustrd[0].0, "INVOICE 12345");
    }

    #[test]
    fn test_mt103_to_pacs008_sha_charge_bearer() {
        let msg = parse(MT103_RAW).unwrap();
        let mt103 = parse_mt103(&msg.block4).unwrap();
        let result = mt103_to_pacs008(&mt103, "MSG003", "2023-06-15T10:00:00").unwrap();
        let tx = &result.message.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf[0];
        assert!(matches!(tx.chrg_br, pacs008::ChargeBearerType1Code::Shar));
    }

    #[test]
    fn test_mt103_to_pacs008_dbtr_iban() {
        let msg = parse(MT103_RAW).unwrap();
        let mt103 = parse_mt103(&msg.block4).unwrap();
        let result = mt103_to_pacs008(&mt103, "MSG004", "2023-06-15T10:00:00").unwrap();
        let tx = &result.message.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf[0];
        let dbtr_acct = tx.dbtr_acct.as_ref().unwrap();
        if let Some(mx20022_model::common::ChoiceWrapper {
            inner: pacs008::AccountIdentification4Choice::IBAN(iban),
        }) = &dbtr_acct.id
        {
            assert_eq!(iban.0, "DE89370400440532013000");
        } else {
            panic!("expected IBAN account");
        }
    }
}
