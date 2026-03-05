//! Translation from camt.053.001.11 to MT940 Customer Statement Message text.

use std::fmt::Write as _;

use mx20022_model::generated::camt::camt_053_001_11 as camt053;

use crate::mappings::error::{TranslationError, TranslationResult, TranslationWarnings};
use crate::mappings::helpers::{iso_date_to_yymmdd, pad_lt_address};

/// Translate a `camt.053.001.11` `Document` to an MT940 message string.
///
/// Only the first `Stmt` entry is translated.  A warning is added when there
/// are multiple statements.
///
/// # Errors
///
/// Returns [`TranslationError::MissingField`] when a required camt.053 field
/// is absent.
pub fn camt053_to_mt940(
    doc: &camt053::Document,
) -> Result<TranslationResult<String>, TranslationError> {
    let mut warnings = TranslationWarnings::default();

    let stmts = &doc.bk_to_cstmr_stmt.stmt;
    if stmts.len() > 1 {
        warnings.add(
            "Stmt",
            "document contains multiple statements; only the first is translated",
        );
    }

    let stmt = stmts
        .first()
        .ok_or_else(|| TranslationError::MissingField {
            field: "Stmt".into(),
            context: "camt053_to_mt940".into(),
        })?;

    // :20: transaction reference
    let stmt_ref = stmt.id.0.clone();

    // :25: account identification
    let account_id = extract_account_id(&stmt.acct);

    // :28C: sequence number
    let seq_num = stmt
        .elctrnc_seq_nb
        .as_ref()
        .map(|n| n.0.as_str())
        .or_else(|| stmt.lgl_seq_nb.as_ref().map(|n| n.0.as_str()))
        .unwrap_or("1");
    let seq_number = format!("{seq_num}/1");

    // Find opening balance (OPBD or ITBD)
    let opening = find_balance(stmt, &["OPBD", "ITBD"]);
    // Find closing balance (CLBD or ITCL)
    let closing = find_balance(stmt, &["CLBD", "ITCL"]);

    let opening_balance = if let Some(bal) = opening {
        balance_to_mt(bal)?
    } else {
        warnings.add("Bal[OPBD]", "no opening balance found; using placeholder");
        "C000101EUR0,00".to_string()
    };

    let closing_balance = if let Some(bal) = closing {
        balance_to_mt(bal)?
    } else {
        warnings.add("Bal[CLBD]", "no closing balance found; using placeholder");
        "C000101EUR0,00".to_string()
    };

    // Build body — camt.053 statements do not carry sender/receiver BICs.
    warnings.add(
        "Stmt/Acct",
        "sender/receiver BIC not available in camt.053; using placeholder",
    );
    let sender_bic = "UNKNOWNXXXXX";

    let mt_text = format_mt940(
        sender_bic,
        &stmt_ref,
        &account_id,
        &seq_number,
        &opening_balance,
        &closing_balance,
        &stmt.ntry,
        find_balance(stmt, &["CLAV"]),
        &mut warnings,
    );

    Ok(TranslationResult {
        message: mt_text,
        warnings,
    })
}

/// Format a complete MT940 message string.
#[allow(clippy::too_many_arguments)]
fn format_mt940(
    sender_bic: &str,
    stmt_ref: &str,
    account_id: &str,
    seq_number: &str,
    opening_balance: &str,
    closing_balance: &str,
    entries: &[camt053::ReportEntry13],
    closing_avail: Option<&camt053::CashBalance8>,
    warnings: &mut crate::mappings::error::TranslationWarnings,
) -> String {
    let sender_lt = pad_lt_address(sender_bic);

    let mut body = String::new();
    let _ = writeln!(body, ":20:{stmt_ref}");
    let _ = writeln!(body, ":25:{account_id}");
    let _ = writeln!(body, ":28C:{seq_number}");
    let _ = writeln!(body, ":60F:{opening_balance}");

    for ntry in entries {
        match entry_to_mt61(ntry) {
            Ok(line_61) => {
                let _ = writeln!(body, ":61:{line_61}");
                if let Some(info) = &ntry.addtl_ntry_inf {
                    if !info.0.is_empty() {
                        let _ = writeln!(body, ":86:{}", info.0);
                    }
                }
            }
            Err(e) => {
                warnings.add(":61:", format!("skipped entry: {e}"));
            }
        }
    }

    let _ = writeln!(body, ":62F:{closing_balance}");

    if let Some(ca) = closing_avail {
        match balance_to_mt(ca) {
            Ok(s) => {
                let _ = writeln!(body, ":64:{s}");
            }
            Err(e) => warnings.add(":64:", format!("could not format closing available: {e}")),
        }
    }

    format!(
        "{{1:F01{sender_lt}0000000000}}{{2:O9400000000000{sender_lt}00000000000000000000N}}{{4:\n{body}-}}{{5:{{CHK:000000000000}}}}"
    )
}

/// Extract a usable account ID string from a [`camt053::CashAccount41`].
fn extract_account_id(acct: &camt053::CashAccount41) -> String {
    if let Some(wrapper) = &acct.id {
        match &wrapper.inner {
            camt053::AccountIdentification4Choice::IBAN(iban) => iban.0.clone(),
            camt053::AccountIdentification4Choice::Othr(othr) => othr.id.0.clone(),
        }
    } else {
        String::new()
    }
}

/// Find the first balance entry matching one of the provided type codes.
fn find_balance<'a>(
    stmt: &'a camt053::AccountStatement12,
    type_codes: &[&str],
) -> Option<&'a camt053::CashBalance8> {
    stmt.bal.iter().find(|b| {
        if let camt053::BalanceType10Choice::Cd(code) = &b.tp.cd_or_prtry.inner {
            type_codes.contains(&code.0.as_str())
        } else {
            false
        }
    })
}

/// Format a balance as an MT940 field value string.
///
/// Output: `{D|C}{YYMMDD}{CCY}{amount,decimal}`
fn balance_to_mt(bal: &camt053::CashBalance8) -> Result<String, TranslationError> {
    let dc = match &bal.cdt_dbt_ind {
        camt053::CreditDebitCode::Crdt => 'C',
        camt053::CreditDebitCode::Dbit => 'D',
    };

    let date_iso = extract_balance_date(bal);
    let date_swift = iso_date_to_yymmdd(&date_iso)?;
    let ccy = &bal.amt.ccy.0;
    let amt_swift = bal.amt.value.0.replace('.', ",");

    Ok(format!("{dc}{date_swift}{ccy}{amt_swift}"))
}

/// Extract the ISO date string from a `CashBalance8`.
fn extract_balance_date(bal: &camt053::CashBalance8) -> String {
    match &bal.dt.inner {
        camt053::DateAndDateTime2Choice::Dt(d) => d.0.clone(),
        camt053::DateAndDateTime2Choice::DtTm(dt) => {
            dt.0.split('T').next().unwrap_or("").to_string()
        }
    }
}

/// Convert a `ReportEntry13` to a MT940 `:61:` field value string.
///
/// Format: `YYMMDD[MMDD]D/C[letter]amount,NtypeREF[//InstREF]`
///
/// # Errors
///
/// Returns an error if the value date cannot be converted to YYMMDD format.
fn entry_to_mt61(entry: &camt053::ReportEntry13) -> Result<String, TranslationError> {
    let dc = match &entry.cdt_dbt_ind {
        camt053::CreditDebitCode::Crdt => 'C',
        camt053::CreditDebitCode::Dbit => 'D',
    };

    let val_date_iso = entry.val_dt.as_ref().map_or_else(
        || "0001-01-01".to_string(),
        |v| match &v.inner {
            camt053::DateAndDateTime2Choice::Dt(d) => d.0.clone(),
            camt053::DateAndDateTime2Choice::DtTm(dt) => {
                dt.0.split('T').next().unwrap_or("").to_string()
            }
        },
    );

    let date_swift = iso_date_to_yymmdd(&val_date_iso)?;
    let amt_swift = entry.amt.value.0.replace('.', ",");

    let tx_code = entry
        .bk_tx_cd
        .prtry
        .as_ref()
        .map_or("NMSC", |p| p.cd.0.as_str());

    let ref_str = entry.ntry_ref.as_ref().map_or("NONREF", |r| r.0.as_str());

    Ok(format!("{date_swift}{dc}{amt_swift}{tx_code}{ref_str}"))
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mappings::mt940_to_camt053::mt940_to_camt053;
    use crate::mt::fields::mt940::parse_mt940;
    use crate::mt::parser::parse;

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

    fn roundtrip_doc() -> camt053::Document {
        let msg = parse(MT940_RAW).unwrap();
        let mt940 = parse_mt940(&msg.block4).unwrap();
        mt940_to_camt053(&mt940, "MSG001", "2023-06-15T10:00:00")
            .unwrap()
            .message
    }

    #[test]
    fn test_camt053_to_mt940_contains_20() {
        let doc = roundtrip_doc();
        let result = camt053_to_mt940(&doc).unwrap();
        assert!(
            result.message.contains(":20:STMT-REF-001"),
            "result: {}",
            result.message
        );
    }

    #[test]
    fn test_camt053_to_mt940_contains_25() {
        let doc = roundtrip_doc();
        let result = camt053_to_mt940(&doc).unwrap();
        assert!(
            result.message.contains(":25:NL91ABNA0417164300"),
            "result: {}",
            result.message
        );
    }

    #[test]
    fn test_camt053_to_mt940_contains_60f() {
        let doc = roundtrip_doc();
        let result = camt053_to_mt940(&doc).unwrap();
        assert!(
            result.message.contains(":60F:"),
            "result: {}",
            result.message
        );
    }

    #[test]
    fn test_camt053_to_mt940_no_stmts() {
        let doc = camt053::Document {
            bk_to_cstmr_stmt: camt053::BankToCustomerStatementV11 {
                grp_hdr: camt053::GroupHeader81::builder()
                    .msg_id(camt053::Max35Text("X".into()))
                    .cre_dt_tm(camt053::ISODateTime("2023-01-01T00:00:00".into()))
                    .build()
                    .unwrap(),
                stmt: vec![],
                splmtry_data: vec![],
            },
        };
        let err = camt053_to_mt940(&doc).unwrap_err();
        assert!(matches!(err, TranslationError::MissingField { .. }));
    }

    #[test]
    fn test_block2_output_header_length_and_parsability() {
        let doc = roundtrip_doc();
        let result = camt053_to_mt940(&doc).unwrap();

        // Extract block 2 content between {2: and }
        let b2_start = result.message.find("{2:").expect("block 2 must exist") + 3;
        let b2_end = result.message[b2_start..]
            .find('}')
            .expect("block 2 closing brace")
            + b2_start;
        let block2 = &result.message[b2_start..b2_end];

        // Block 2 Output must be exactly 47 chars: O(1) + msg_type(3) + input_time(4)
        // + MIR(28) + output_date(6) + output_time(4) + priority(1)
        assert_eq!(
            block2.len(),
            47,
            "block 2 output header must be 47 chars, got {}: '{block2}'",
            block2.len()
        );
        assert!(block2.starts_with('O'), "block 2 must start with 'O'");
        assert!(
            block2.ends_with('N'),
            "block 2 priority must be 'N', got last char '{}'",
            block2.chars().last().unwrap()
        );

        // The full message must re-parse successfully with correct block 2 variant
        let reparsed = parse(&result.message).expect("generated MT940 must parse");
        match reparsed.block2.as_ref().expect("block 2 must be present") {
            crate::mt::types::Block2::Output(out) => {
                assert_eq!(out.message_type, "940");
                assert_eq!(out.priority, Some('N'));
            }
            other => panic!("expected Block2::Output, got {other:?}"),
        }
    }
}
