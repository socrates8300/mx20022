//! Translation from pacs.009.001.10 to MT202 Bank-to-Bank Credit Transfer text.

use mx20022_model::generated::pacs::pacs_009_001_10 as pacs009;

use crate::mappings::{
    error::{TranslationError, TranslationResult, TranslationWarnings},
    helpers::{format_mt_message, iso_date_to_yymmdd},
};

/// Translate a `pacs.009.001.10` `Document` to an MT202 message string.
///
/// # Errors
///
/// Returns [`TranslationError::MissingField`] when required pacs.009 fields
/// are absent.
pub fn pacs009_to_mt202(
    doc: &pacs009::Document,
) -> Result<TranslationResult<String>, TranslationError> {
    let mut warnings = TranslationWarnings::default();

    let fi_cdt_trf = &doc.fi_cdt_trf;

    if fi_cdt_trf.cdt_trf_tx_inf.len() > 1 {
        warnings.add(
            "CdtTrfTxInf",
            "document contains multiple transactions; only the first is translated",
        );
    }

    let tx = fi_cdt_trf
        .cdt_trf_tx_inf
        .first()
        .ok_or_else(|| TranslationError::MissingField {
            field: "CdtTrfTxInf".into(),
            context: "pacs009_to_mt202".into(),
        })?;

    // :20:
    let reference = &tx.pmt_id.end_to_end_id.0;

    // :21: related reference
    let related_ref = tx.pmt_id.tx_id.as_ref().map_or("NONREF", |t| t.0.as_str());

    // :32A:
    let value_date_iso = tx
        .intr_bk_sttlm_dt
        .as_ref()
        .map_or("000101", |d| d.0.as_str());
    let value_date_swift = iso_date_to_yymmdd(value_date_iso)?;
    let ccy = &tx.intr_bk_sttlm_amt.ccy.0;
    let amt_dot = &tx.intr_bk_sttlm_amt.value.0;
    let amt_swift = amt_dot.replace('.', ",");
    let field_32a = format!("{value_date_swift}{ccy}{amt_swift}");

    // :58A/D: beneficiary institution (Cdtr)
    let field_58 = extract_fi_field(&tx.cdtr);

    let fields: Vec<(String, String)> = vec![
        ("20".into(), reference.clone()),
        ("21".into(), related_ref.to_string()),
        ("32A".into(), field_32a),
        ("58A".into(), field_58),
    ];

    let sender_bic = extract_bic_from_fi6(&tx.dbtr).unwrap_or_else(|| {
        warnings.add(
            "Dbtr/FinInstnId/BICFI",
            "BIC unavailable; using placeholder",
        );
        "UNKNOWNXXXXX".to_string()
    });
    let receiver_bic = extract_bic_from_fi6(&tx.cdtr).unwrap_or_else(|| {
        warnings.add(
            "Cdtr/FinInstnId/BICFI",
            "BIC unavailable; using placeholder",
        );
        "UNKNOWNXXXXX".to_string()
    });

    let mt_text = format_mt_message("202", &sender_bic, &receiver_bic, &fields);

    Ok(TranslationResult {
        message: mt_text,
        warnings,
    })
}

fn extract_fi_field(fi: &pacs009::BranchAndFinancialInstitutionIdentification6) -> String {
    if let Some(bic) = &fi.fin_instn_id.bicfi {
        bic.0.clone()
    } else if let Some(nm) = &fi.fin_instn_id.nm {
        nm.0.clone()
    } else {
        String::new()
    }
}

fn extract_bic_from_fi6(
    fi: &pacs009::BranchAndFinancialInstitutionIdentification6,
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
    use crate::mappings::mt202_to_pacs009::mt202_to_pacs009;
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

    fn roundtrip_doc() -> pacs009::Document {
        let msg = parse(MT202_RAW).unwrap();
        let mt202 = parse_mt202(&msg.block4).unwrap();
        mt202_to_pacs009(&mt202, "MSG001", "2023-06-15T10:00:00")
            .unwrap()
            .message
    }

    #[test]
    fn test_pacs009_to_mt202_contains_reference() {
        let doc = roundtrip_doc();
        let result = pacs009_to_mt202(&doc).unwrap();
        assert!(
            result.message.contains(":20:TXN-REF-202-001"),
            "result: {}",
            result.message
        );
    }

    #[test]
    fn test_pacs009_to_mt202_contains_32a() {
        let doc = roundtrip_doc();
        let result = pacs009_to_mt202(&doc).unwrap();
        assert!(
            result.message.contains(":32A:230615USD50000,00"),
            "result: {}",
            result.message
        );
    }

    #[test]
    fn test_pacs009_to_mt202_contains_21() {
        let doc = roundtrip_doc();
        let result = pacs009_to_mt202(&doc).unwrap();
        // :21: carries the related reference (from :21:REL-REF-001 in the MT source),
        // which is stored in tx_id and roundtrips correctly.
        assert!(
            result.message.contains(":21:REL-REF-001"),
            "result: {}",
            result.message
        );
    }
}
