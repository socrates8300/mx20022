//! Translation from MT940 Customer Statement Message to camt.053.001.11.

use mx20022_model::common::ChoiceWrapper;
use mx20022_model::generated::camt::camt_053_001_11 as camt053;

use crate::mappings::error::{TranslationError, TranslationResult, TranslationWarnings};
use crate::mt::fields::mt940::{Balance, Mt940, StatementLine};

/// Translate an `Mt940` into a `camt.053.001.11` `Document`.
///
/// # Errors
///
/// Returns [`TranslationError`] when a mandatory field cannot be mapped.
pub fn mt940_to_camt053(
    mt940: &Mt940,
    msg_id: &str,
    creation_time: &str,
) -> Result<TranslationResult<camt053::Document>, TranslationError> {
    let mut warnings = TranslationWarnings::default();

    // ------------------------------------------------------------------
    // GroupHeader81
    // ------------------------------------------------------------------
    let grp_hdr = camt053::GroupHeader81::builder()
        .msg_id(camt053::Max35Text(msg_id.to_string()))
        .cre_dt_tm(camt053::ISODateTime(creation_time.to_string()))
        .build()?;

    // ------------------------------------------------------------------
    // Account identification (:25:)
    // ------------------------------------------------------------------
    let acct_id = build_account_id(&mt940.account_id);
    let acct = camt053::CashAccount41 {
        id: Some(acct_id),
        tp: None,
        ccy: None,
        nm: None,
        prxy: None,
        ownr: None,
        svcr: None,
    };

    // ------------------------------------------------------------------
    // Statement ID (:20:) and sequence number (:28C:)
    // ------------------------------------------------------------------
    let stmt_id = camt053::Max35Text(mt940.transaction_reference.clone());

    // Try to parse the statement number from :28C: (format: NNN/NNN)
    let elctrnc_seq_nb: Option<camt053::Number> = parse_statement_number(&mt940.statement_number);

    // ------------------------------------------------------------------
    // Balances (:60F/M: opening, :62F/M: closing, :64: closing available)
    // ------------------------------------------------------------------
    let mut bal_vec: Vec<camt053::CashBalance8> = Vec::new();

    // Opening balance type: OPBD (final) or ITBD (intermediate)
    let opening_type_code = if mt940.opening_balance.balance_type == 'F' {
        "OPBD"
    } else {
        "ITBD"
    };
    bal_vec.push(build_balance(&mt940.opening_balance, opening_type_code)?);

    // Closing balance type: CLBD (final) or ITCL (intermediate)
    let closing_type_code = if mt940.closing_balance.balance_type == 'F' {
        "CLBD"
    } else {
        "ITCL"
    };
    bal_vec.push(build_balance(&mt940.closing_balance, closing_type_code)?);

    // Closing available balance (:64:)
    if let Some(ca) = &mt940.closing_available {
        bal_vec.push(build_balance(ca, "CLAV")?);
    }

    // Forward available balances (:65:)
    for fa in &mt940.forward_available {
        bal_vec.push(build_balance(fa, "FWAV")?);
    }

    // ------------------------------------------------------------------
    // Statement entries (:61: + :86:)
    // ------------------------------------------------------------------
    let mut ntry_vec: Vec<camt053::ReportEntry13> = Vec::new();
    for sl in &mt940.statement_lines {
        match build_entry(sl) {
            Ok(entry) => ntry_vec.push(entry),
            Err(e) => {
                warnings.add(":61:", format!("skipped statement line: {e}"));
            }
        }
    }

    // ------------------------------------------------------------------
    // AccountStatement12
    // ------------------------------------------------------------------
    let stmt = camt053::AccountStatement12 {
        id: stmt_id,
        stmt_pgntn: None,
        elctrnc_seq_nb,
        rptg_seq: None,
        lgl_seq_nb: None,
        cre_dt_tm: None,
        fr_to_dt: None,
        cpy_dplct_ind: None,
        rptg_src: None,
        acct,
        rltd_acct: None,
        intrst: vec![],
        bal: bal_vec,
        txs_summry: None,
        ntry: ntry_vec,
        addtl_stmt_inf: None,
    };

    let bk_to_cstmr_stmt = camt053::BankToCustomerStatementV11 {
        grp_hdr,
        stmt: vec![stmt],
        splmtry_data: vec![],
    };

    let doc = camt053::Document { bk_to_cstmr_stmt };

    Ok(TranslationResult {
        message: doc,
        warnings,
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build an `AccountIdentification4Choice` wrapped in a `ChoiceWrapper` from
/// the raw MT940 `:25:` account ID string.
fn build_account_id(account_id: &str) -> ChoiceWrapper<camt053::AccountIdentification4Choice> {
    let trimmed = account_id.trim();
    // Check if it looks like an IBAN (country code prefix, >= 15 chars)
    if is_iban_like(trimmed) {
        ChoiceWrapper::new(camt053::AccountIdentification4Choice::IBAN(
            camt053::IBAN2007Identifier(trimmed.to_string()),
        ))
    } else {
        ChoiceWrapper::new(camt053::AccountIdentification4Choice::Othr(
            camt053::GenericAccountIdentification1 {
                id: camt053::Max34Text(trimmed.to_string()),
                schme_nm: None,
                issr: None,
            },
        ))
    }
}

/// Heuristic IBAN check: two uppercase letters followed by digits, length 15-34.
fn is_iban_like(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    chars.len() >= 15
        && chars.len() <= 34
        && chars[0].is_ascii_uppercase()
        && chars[1].is_ascii_uppercase()
        && chars[2..].iter().all(char::is_ascii_alphanumeric)
}

/// Parse the statement / sequence number from the `:28C:` value.
///
/// Format: `NNN/NNN` — returns the first number.
fn parse_statement_number(s: &str) -> Option<camt053::Number> {
    let first = s.split('/').next().unwrap_or("").trim();
    if first.chars().all(|c| c.is_ascii_digit()) && !first.is_empty() {
        Some(camt053::Number(first.to_string()))
    } else {
        None
    }
}

/// Convert an MT `Balance` struct into a `camt.053` `CashBalance8`.
///
/// `balance_type_code` must be an ISO 20022 External Balance Type code
/// (`OPBD`, `CLBD`, `ITBD`, `ITCL`, `CLAV`, `FWAV`).
fn build_balance(
    bal: &Balance,
    balance_type_code: &str,
) -> Result<camt053::CashBalance8, TranslationError> {
    let cdt_dbt_ind = match bal.dc_indicator {
        'C' => camt053::CreditDebitCode::Crdt,
        'D' => camt053::CreditDebitCode::Dbit,
        other => {
            return Err(TranslationError::InvalidFieldValue {
                field: "balance.dc_indicator".into(),
                detail: format!("expected C or D, got '{other}'"),
            });
        }
    };

    let amt = camt053::ActiveOrHistoricCurrencyAndAmount {
        value: camt053::ActiveOrHistoricCurrencyAndAmountSimpleType(bal.amount.clone()),
        ccy: camt053::ActiveOrHistoricCurrencyCode(bal.currency.clone()),
    };

    let bal_type = camt053::BalanceType13 {
        cd_or_prtry: ChoiceWrapper::new(camt053::BalanceType10Choice::Cd(
            camt053::ExternalBalanceType1Code(balance_type_code.to_string()),
        )),
        sub_tp: None,
    };

    let dt = ChoiceWrapper::new(camt053::DateAndDateTime2Choice::Dt(camt053::ISODate(
        bal.date.clone(),
    )));

    camt053::CashBalance8::builder()
        .tp(bal_type)
        .amt(amt)
        .cdt_dbt_ind(cdt_dbt_ind)
        .dt(dt)
        .build()
        .map_err(TranslationError::Builder)
}

/// Build a `ReportEntry13` from a `StatementLine`.
fn build_entry(sl: &StatementLine) -> Result<camt053::ReportEntry13, TranslationError> {
    let is_credit = sl.dc_mark == "C" || sl.dc_mark == "RC";
    let cdt_dbt_ind = if is_credit {
        camt053::CreditDebitCode::Crdt
    } else {
        camt053::CreditDebitCode::Dbit
    };

    let amt = camt053::ActiveOrHistoricCurrencyAndAmount {
        value: camt053::ActiveOrHistoricCurrencyAndAmountSimpleType(sl.amount.clone()),
        // Currency is not present at the entry level in MT940; use an empty placeholder.
        ccy: camt053::ActiveOrHistoricCurrencyCode(String::new()),
    };

    let val_dt = ChoiceWrapper::new(camt053::DateAndDateTime2Choice::Dt(camt053::ISODate(
        sl.value_date.clone(),
    )));

    let sts = ChoiceWrapper::new(camt053::EntryStatus1Choice::Cd(
        camt053::ExternalEntryStatus1Code("BOOK".to_string()),
    ));

    // Use the transaction type code as a proprietary bank transaction code.
    let bk_tx_cd = camt053::BankTransactionCodeStructure4 {
        domn: None,
        prtry: Some(camt053::ProprietaryBankTransactionCodeStructure1 {
            cd: camt053::Max35Text(sl.transaction_type.clone()),
            issr: None,
        }),
    };

    let mut builder = camt053::ReportEntry13::builder()
        .ntry_ref(camt053::Max35Text(sl.reference.clone()))
        .amt(amt)
        .cdt_dbt_ind(cdt_dbt_ind)
        .sts(sts)
        .val_dt(val_dt)
        .bk_tx_cd(bk_tx_cd);

    if let Some(info) = &sl.information {
        builder = builder.addtl_ntry_inf(camt053::Max500Text(info.clone()));
    }

    builder.build().map_err(TranslationError::Builder)
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_mt940_to_camt053_basic() {
        let msg = parse(MT940_RAW).unwrap();
        let mt940 = parse_mt940(&msg.block4).unwrap();
        let result = mt940_to_camt053(&mt940, "MSG001", "2023-06-15T10:00:00").unwrap();
        let doc = &result.message;

        let stmt = &doc.bk_to_cstmr_stmt.stmt[0];
        assert_eq!(stmt.id.0, "STMT-REF-001");
    }

    #[test]
    fn test_mt940_to_camt053_balances() {
        let msg = parse(MT940_RAW).unwrap();
        let mt940 = parse_mt940(&msg.block4).unwrap();
        let result = mt940_to_camt053(&mt940, "MSG001", "2023-06-15T10:00:00").unwrap();
        let stmt = &result.message.bk_to_cstmr_stmt.stmt[0];

        // Opening + closing = 2 balances
        assert!(stmt.bal.len() >= 2);
        // Opening balance: credit, 10000.00
        let ob = &stmt.bal[0];
        assert!(matches!(ob.cdt_dbt_ind, camt053::CreditDebitCode::Crdt));
        assert_eq!(ob.amt.value.0, "10000.00");
    }

    #[test]
    fn test_mt940_to_camt053_entries() {
        let msg = parse(MT940_RAW).unwrap();
        let mt940 = parse_mt940(&msg.block4).unwrap();
        let result = mt940_to_camt053(&mt940, "MSG001", "2023-06-15T10:00:00").unwrap();
        let stmt = &result.message.bk_to_cstmr_stmt.stmt[0];

        assert_eq!(stmt.ntry.len(), 1);
        let entry = &stmt.ntry[0];
        assert!(matches!(entry.cdt_dbt_ind, camt053::CreditDebitCode::Dbit));
        assert_eq!(entry.amt.value.0, "1000.50");
        assert_eq!(
            entry.addtl_ntry_inf.as_ref().map(|s| s.0.as_str()),
            Some("Payment to supplier")
        );
    }

    #[test]
    fn test_mt940_to_camt053_iban_account() {
        let msg = parse(MT940_RAW).unwrap();
        let mt940 = parse_mt940(&msg.block4).unwrap();
        let result = mt940_to_camt053(&mt940, "MSG001", "2023-06-15T10:00:00").unwrap();
        let stmt = &result.message.bk_to_cstmr_stmt.stmt[0];
        if let Some(ChoiceWrapper {
            inner: camt053::AccountIdentification4Choice::IBAN(iban),
        }) = &stmt.acct.id
        {
            assert_eq!(iban.0, "NL91ABNA0417164300");
        } else {
            panic!("expected IBAN account identification");
        }
    }
}
