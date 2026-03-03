//! Translation from MT202 Bank-to-Bank Credit Transfer to pacs.009.001.10.

use mx20022_model::generated::pacs::pacs_009_001_10 as pacs009;

use crate::mappings::error::{TranslationError, TranslationResult, TranslationWarnings};
use crate::mt::fields::common::PartyInfo;
use crate::mt::fields::mt202::Mt202;

/// Translate an [`Mt202`] into a `pacs.009.001.10` [`Document`].
///
/// # Errors
///
/// Returns [`TranslationError`] when a mandatory field cannot be mapped.
pub fn mt202_to_pacs009(
    mt202: &Mt202,
    msg_id: &str,
    creation_time: &str,
) -> Result<TranslationResult<pacs009::Document>, TranslationError> {
    let mut warnings = TranslationWarnings::default();

    // ------------------------------------------------------------------
    // GroupHeader96
    // ------------------------------------------------------------------
    let sttlm_inf = pacs009::SettlementInstruction11 {
        sttlm_mtd: pacs009::SettlementMethod1Code::Inda,
        sttlm_acct: None,
        clr_sys: None,
        instg_rmbrsmnt_agt: None,
        instg_rmbrsmnt_agt_acct: None,
        instd_rmbrsmnt_agt: None,
        instd_rmbrsmnt_agt_acct: None,
        thrd_rmbrsmnt_agt: None,
        thrd_rmbrsmnt_agt_acct: None,
    };

    let grp_hdr = pacs009::GroupHeader96::builder()
        .msg_id(pacs009::Max35Text(msg_id.to_string()))
        .cre_dt_tm(pacs009::ISODateTime(creation_time.to_string()))
        .nb_of_txs(pacs009::Max15NumericText("1".to_string()))
        .sttlm_inf(sttlm_inf)
        .build()?;

    // ------------------------------------------------------------------
    // PaymentIdentification13
    // ------------------------------------------------------------------
    let pmt_id = pacs009::PaymentIdentification13 {
        instr_id: Some(pacs009::Max35Text(mt202.transaction_reference.clone())),
        end_to_end_id: pacs009::Max35Text(mt202.transaction_reference.clone()),
        tx_id: Some(pacs009::Max35Text(mt202.related_reference.clone())),
        uetr: None,
        clr_sys_ref: None,
    };

    // ------------------------------------------------------------------
    // IntrBkSttlmAmt + IntrBkSttlmDt
    // ------------------------------------------------------------------
    let intr_bk_sttlm_amt = pacs009::ActiveCurrencyAndAmount {
        value: pacs009::ActiveCurrencyAndAmountSimpleType(mt202.amount.clone()),
        ccy: pacs009::ActiveCurrencyCode(mt202.currency.clone()),
    };
    let intr_bk_sttlm_dt = Some(pacs009::ISODate(mt202.value_date.clone()));

    // ------------------------------------------------------------------
    // Dbtr (ordering institution)
    // ------------------------------------------------------------------
    let dbtr = mt202
        .ordering_institution
        .as_ref()
        .map_or_else(empty_fi6, party_info_to_fi6);

    // ------------------------------------------------------------------
    // DbtrAgt — not present in MT202 directly
    // ------------------------------------------------------------------

    // ------------------------------------------------------------------
    // CdtrAgt — not directly represented; beneficiary institution goes to Cdtr
    // ------------------------------------------------------------------

    // ------------------------------------------------------------------
    // Cdtr (beneficiary institution — :58A/D:)
    // ------------------------------------------------------------------
    let cdtr = party_info_to_fi6(&mt202.beneficiary_institution);

    // ------------------------------------------------------------------
    // IntrmyAgt1 (intermediary — :56A/D:)
    // ------------------------------------------------------------------
    let intrmy_agt1 = mt202.intermediary.as_ref().map(party_info_to_fi6);

    // ------------------------------------------------------------------
    // Warn about unmapped fields
    // ------------------------------------------------------------------
    if mt202.senders_correspondent.is_some() {
        warnings.add(":53:", "senders_correspondent not mapped to pacs.009");
    }
    if mt202.receivers_correspondent.is_some() {
        warnings.add(":54:", "receivers_correspondent not mapped to pacs.009");
    }
    if mt202.sender_to_receiver_info.is_some() {
        warnings.add(":72:", "sender_to_receiver_info not mapped");
    }

    // ------------------------------------------------------------------
    // Assemble CreditTransferTransaction56
    // ------------------------------------------------------------------
    let tx = pacs009::CreditTransferTransaction56 {
        pmt_id,
        pmt_tp_inf: None,
        intr_bk_sttlm_amt,
        intr_bk_sttlm_dt,
        sttlm_prty: None,
        sttlm_tm_indctn: None,
        sttlm_tm_req: None,
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
        dbtr,
        dbtr_acct: None,
        dbtr_agt: None,
        dbtr_agt_acct: None,
        cdtr_agt: None,
        cdtr_agt_acct: None,
        cdtr,
        cdtr_acct: None,
        ultmt_cdtr: None,
        instr_for_cdtr_agt: vec![],
        instr_for_nxt_agt: vec![],
        purp: None,
        rmt_inf: None,
        undrlyg_cstmr_cdt_trf: None,
        splmtry_data: vec![],
    };

    let fi_cdt_trf = pacs009::FinancialInstitutionCreditTransferV10 {
        grp_hdr,
        cdt_trf_tx_inf: vec![tx],
        splmtry_data: vec![],
    };

    let doc = pacs009::Document { fi_cdt_trf };

    Ok(TranslationResult {
        message: doc,
        warnings,
    })
}

// ---------------------------------------------------------------------------
// Helpers (pacs009-specific types)
// ---------------------------------------------------------------------------

fn party_info_to_fi6(party: &PartyInfo) -> pacs009::BranchAndFinancialInstitutionIdentification6 {
    let mut bicfi: Option<pacs009::BICFIDec2014Identifier> = None;
    let mut nm: Option<pacs009::Max140Text> = None;

    if let Some(acct) = &party.account {
        if let Some(bic) = &acct.bic {
            bicfi = Some(pacs009::BICFIDec2014Identifier(bic.clone()));
        }
    }

    if bicfi.is_none() {
        if let Some(n) = &party.name {
            nm = Some(pacs009::Max140Text(n.clone()));
        }
    }

    pacs009::BranchAndFinancialInstitutionIdentification6 {
        fin_instn_id: pacs009::FinancialInstitutionIdentification18 {
            bicfi,
            clr_sys_mmb_id: None,
            lei: None,
            nm,
            pstl_adr: None,
            othr: None,
        },
        brnch_id: None,
    }
}

fn empty_fi6() -> pacs009::BranchAndFinancialInstitutionIdentification6 {
    pacs009::BranchAndFinancialInstitutionIdentification6 {
        fin_instn_id: pacs009::FinancialInstitutionIdentification18 {
            bicfi: None,
            clr_sys_mmb_id: None,
            lei: None,
            nm: None,
            pstl_adr: None,
            othr: None,
        },
        brnch_id: None,
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mt::fields::mt202::parse_mt202;
    use crate::mt::parser::parse;

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

    #[test]
    fn test_mt202_to_pacs009_basic() {
        let msg = parse(MT202_RAW).unwrap();
        let mt202 = parse_mt202(&msg.block4).unwrap();
        let result = mt202_to_pacs009(&mt202, "MSG001", "2023-06-15T10:00:00").unwrap();
        let doc = &result.message;
        let tx = &doc.fi_cdt_trf.cdt_trf_tx_inf[0];

        assert_eq!(doc.fi_cdt_trf.grp_hdr.msg_id.0, "MSG001");
        assert_eq!(tx.pmt_id.end_to_end_id.0, "TXN-REF-202-001");
        assert_eq!(tx.intr_bk_sttlm_amt.value.0, "50000.00");
        assert_eq!(tx.intr_bk_sttlm_amt.ccy.0, "USD");
        assert_eq!(
            tx.intr_bk_sttlm_dt.as_ref().map(|d| d.0.as_str()),
            Some("2023-06-15")
        );
    }

    #[test]
    fn test_mt202_to_pacs009_cdtr_name() {
        let msg = parse(MT202_RAW).unwrap();
        let mt202 = parse_mt202(&msg.block4).unwrap();
        let result = mt202_to_pacs009(&mt202, "MSG002", "2023-06-15T10:00:00").unwrap();
        let tx = &result.message.fi_cdt_trf.cdt_trf_tx_inf[0];
        // :58A: has value "CHASUS33XXX" which is parsed as a name (no leading /)
        assert!(
            tx.cdtr.fin_instn_id.nm.as_ref().map(|n| n.0.as_str()) == Some("CHASUS33XXX")
                || tx.cdtr.fin_instn_id.bicfi.is_some()
        );
    }
}
