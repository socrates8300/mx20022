//! Translation from pacs.008.001.13 to MT103 Customer Credit Transfer text.

use mx20022_model::generated::pacs::pacs_008_001_13 as pacs008;

use crate::mappings::{
    error::{TranslationError, TranslationResult, TranslationWarnings},
    helpers::{code_to_charges, format_mt_message, iso_date_to_yymmdd},
};

/// Translate a `pacs.008.001.13` [`Document`] to an MT103 message string.
///
/// Only the first `CdtTrfTxInf` entry is translated.  A warning is added when
/// the document contains more than one transaction.
///
/// # Errors
///
/// Returns [`TranslationError::MissingField`] when a required pacs.008 field
/// is absent.
pub fn pacs008_to_mt103(
    doc: &pacs008::Document,
) -> Result<TranslationResult<String>, TranslationError> {
    let mut warnings = TranslationWarnings::default();

    let fi_to_fi = &doc.fi_to_fi_cstmr_cdt_trf;

    if fi_to_fi.cdt_trf_tx_inf.len() > 1 {
        warnings.add(
            "CdtTrfTxInf",
            "document contains multiple transactions; only the first is translated",
        );
    }

    let tx = fi_to_fi
        .cdt_trf_tx_inf
        .first()
        .ok_or_else(|| TranslationError::MissingField {
            field: "CdtTrfTxInf".into(),
            context: "pacs008_to_mt103".into(),
        })?;

    // :20: — sender's reference from EndToEndId
    let reference = &tx.pmt_id.end_to_end_id.0;

    // :23B:
    let bank_op_code = "CRED";

    // :32A: value date + currency + amount
    let value_date_iso = tx
        .intr_bk_sttlm_dt
        .as_ref()
        .map_or("000101", |d| d.0.as_str());
    let value_date_swift = iso_date_to_yymmdd(value_date_iso)?;
    let ccy = &tx.intr_bk_sttlm_amt.ccy.0;
    let amt_dot = &tx.intr_bk_sttlm_amt.value.0;
    let amt_swift = amt_dot.replace('.', ",");
    let sttlm_field = format!("{value_date_swift}{ccy}{amt_swift}");

    // :50K: ordering customer (Dbtr + DbtrAcct)
    let ordering_party = build_party_field(&tx.dbtr, tx.dbtr_acct.as_ref());

    // :59: beneficiary (Cdtr + CdtrAcct)
    let beneficiary = build_party_field(&tx.cdtr, tx.cdtr_acct.as_ref());

    // :71A: charge bearer
    let charge_bearer = code_to_charges(&tx.chrg_br);

    // Build the field list
    let mut fields: Vec<(String, String)> = vec![
        ("20".into(), reference.clone()),
        ("23B".into(), bank_op_code.into()),
        ("32A".into(), sttlm_field),
        ("50K".into(), ordering_party),
        ("59".into(), beneficiary),
        ("71A".into(), charge_bearer.into()),
    ];

    // :70: remittance info (optional)
    if let Some(rmt) = &tx.rmt_inf {
        if !rmt.ustrd.is_empty() {
            let text: String = rmt
                .ustrd
                .iter()
                .map(|t| t.0.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            fields.push(("70".into(), text));
        }
    }

    // Derive sender/receiver BICs from agents where available
    let sender_bic = extract_bic_from_fi(&tx.dbtr_agt).unwrap_or_else(|| {
        warnings.add(
            "DbtrAgt/FinInstnId/BICFI",
            "BIC unavailable; using placeholder",
        );
        "UNKNOWNXXXXX".to_string()
    });
    let receiver_bic = extract_bic_from_fi(&tx.cdtr_agt).unwrap_or_else(|| {
        warnings.add(
            "CdtrAgt/FinInstnId/BICFI",
            "BIC unavailable; using placeholder",
        );
        "UNKNOWNXXXXX".to_string()
    });

    let mt_text = format_mt_message("103", &sender_bic, &receiver_bic, &fields);

    Ok(TranslationResult {
        message: mt_text,
        warnings,
    })
}

/// Build an MT party field value from a `PartyIdentification272` and optional
/// `CashAccount40`.
///
/// Format:
/// ```text
/// /IBAN_OR_ACCOUNT   (if account present)
/// PARTY NAME
/// ```
fn build_party_field(
    party: &pacs008::PartyIdentification272,
    acct: Option<&pacs008::CashAccount40>,
) -> String {
    let mut lines: Vec<String> = Vec::new();

    if let Some(a) = acct {
        if let Some(id) = &a.id {
            match &id.inner {
                pacs008::AccountIdentification4Choice::IBAN(iban) => {
                    lines.push(format!("/{}", iban.0));
                }
                pacs008::AccountIdentification4Choice::Othr(othr) => {
                    lines.push(format!("/{}", othr.id.0));
                }
            }
        }
    }

    if let Some(nm) = &party.nm {
        lines.push(nm.0.clone());
    }

    lines.join("\n")
}

/// Extract the BIC string from a `BranchAndFinancialInstitutionIdentification8`.
///
/// Returns `None` when neither `BICFI` nor `Nm` is available.
fn extract_bic_from_fi(
    fi: &pacs008::BranchAndFinancialInstitutionIdentification8,
) -> Option<String> {
    fi.fin_instn_id
        .bicfi
        .as_ref()
        .map(|b| b.0.clone())
        .or_else(|| fi.fin_instn_id.nm.as_ref().map(|n| n.0.clone()))
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mappings::mt103_to_pacs008::mt103_to_pacs008;
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

    fn roundtrip_doc() -> pacs008::Document {
        let msg = parse(MT103_RAW).unwrap();
        let mt103 = parse_mt103(&msg.block4).unwrap();
        mt103_to_pacs008(&mt103, "MSG001", "2023-06-15T10:00:00")
            .unwrap()
            .message
    }

    #[test]
    fn test_pacs008_to_mt103_contains_reference() {
        let doc = roundtrip_doc();
        let result = pacs008_to_mt103(&doc).unwrap();
        assert!(result.message.contains(":20:REFERENCE123"));
    }

    #[test]
    fn test_pacs008_to_mt103_contains_32a() {
        let doc = roundtrip_doc();
        let result = pacs008_to_mt103(&doc).unwrap();
        assert!(result.message.contains(":32A:230615EUR1000,50"));
    }

    #[test]
    fn test_pacs008_to_mt103_contains_charge_bearer() {
        let doc = roundtrip_doc();
        let result = pacs008_to_mt103(&doc).unwrap();
        assert!(result.message.contains(":71A:SHA"));
    }

    #[test]
    fn test_pacs008_to_mt103_contains_parties() {
        let doc = roundtrip_doc();
        let result = pacs008_to_mt103(&doc).unwrap();
        assert!(result.message.contains("JOHN DOE"));
        assert!(result.message.contains("JANE SMITH"));
    }

    #[test]
    fn test_pacs008_to_mt103_empty_cdt_trf_tx() {
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
        assert!(matches!(err, TranslationError::MissingField { .. }));
    }
}
