//! Shared field-mapping helpers used across MT↔MX translation modules.
//!
//! The generated MX types are version-specific (e.g.
//! `pacs_008_001_13::PartyIdentification272` vs
//! `pacs_009_001_10::PartyIdentification135`), so helpers that work with
//! pacs.008 types are typed to that module.  Callers dealing with pacs.009
//! or camt.053 can adapt them as needed since the field layout is identical.

use mx20022_model::common::ChoiceWrapper;
use mx20022_model::generated::pacs::pacs_008_001_13 as pacs008;

use crate::mappings::error::TranslationError;
use crate::mt::fields::common::{Account, PartyInfo};

// ---------------------------------------------------------------------------
// Party helpers (pacs.008 types — same layout as pacs.009/camt)
// ---------------------------------------------------------------------------

/// Convert an MT [`PartyInfo`] to a `pacs.008` `PartyIdentification272`.
///
/// The name is mapped to `Nm`.  Address lines are not populated (they do not
/// survive the MT format cleanly enough to be useful).
///
/// # Examples
///
/// ```rust,ignore
/// let p = PartyInfo { name: Some("ACME CORP".into()), ..Default::default() };
/// let id = party_to_party_id(&p);
/// assert_eq!(id.nm.map(|n| n.0), Some("ACME CORP".to_owned()));
/// ```
pub fn party_to_party_id(party: &PartyInfo) -> pacs008::PartyIdentification272 {
    let nm = party
        .name
        .as_deref()
        .map(|n| pacs008::Max140Text(n.to_string()));

    pacs008::PartyIdentification272 {
        nm,
        pstl_adr: None,
        id: None,
        ctry_of_res: None,
        ctct_dtls: None,
    }
}

/// Convert an MT [`PartyInfo`] (representing a financial institution) to a
/// `pacs.008` `BranchAndFinancialInstitutionIdentification8`.
///
/// The account's BIC field is used for `BICFI` if present; otherwise the
/// name is mapped to the FI's `Nm` field.
pub fn party_to_fi_id(party: &PartyInfo) -> pacs008::BranchAndFinancialInstitutionIdentification8 {
    let mut bicfi: Option<pacs008::BICFIDec2014Identifier> = None;
    let mut nm: Option<pacs008::Max140Text> = None;

    if let Some(acct) = &party.account {
        if let Some(bic) = &acct.bic {
            bicfi = Some(pacs008::BICFIDec2014Identifier(bic.clone()));
        }
    }

    if bicfi.is_none() {
        if let Some(n) = &party.name {
            nm = Some(pacs008::Max140Text(n.clone()));
        }
    }

    let fin_instn_id = pacs008::FinancialInstitutionIdentification23 {
        bicfi,
        clr_sys_mmb_id: None,
        lei: None,
        nm,
        pstl_adr: None,
        othr: None,
    };

    pacs008::BranchAndFinancialInstitutionIdentification8 {
        fin_instn_id,
        brnch_id: None,
    }
}

/// Build a minimal `BranchAndFinancialInstitutionIdentification8` from a raw
/// BIC string.
pub fn bic_to_fi_id(bic: &str) -> pacs008::BranchAndFinancialInstitutionIdentification8 {
    let fin_instn_id = pacs008::FinancialInstitutionIdentification23 {
        bicfi: Some(pacs008::BICFIDec2014Identifier(bic.to_string())),
        clr_sys_mmb_id: None,
        lei: None,
        nm: None,
        pstl_adr: None,
        othr: None,
    };
    pacs008::BranchAndFinancialInstitutionIdentification8 {
        fin_instn_id,
        brnch_id: None,
    }
}

/// Build a minimal `BranchAndFinancialInstitutionIdentification8` with no
/// identification (placeholder / unknown agent).
pub fn empty_fi_id() -> pacs008::BranchAndFinancialInstitutionIdentification8 {
    pacs008::BranchAndFinancialInstitutionIdentification8 {
        fin_instn_id: pacs008::FinancialInstitutionIdentification23 {
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
// Account helpers
// ---------------------------------------------------------------------------

/// Convert an MT [`Account`] to a `pacs.008` `CashAccount40`.
///
/// Returns `None` when the account carries no usable identifier.
pub fn account_to_cash_account(acct: &Account) -> Option<pacs008::CashAccount40> {
    let id_choice = acct
        .iban
        .as_ref()
        .map(|iban| {
            ChoiceWrapper::new(pacs008::AccountIdentification4Choice::IBAN(
                pacs008::IBAN2007Identifier(iban.clone()),
            ))
        })
        .or_else(|| {
            acct.account.as_ref().map(|raw| {
                ChoiceWrapper::new(pacs008::AccountIdentification4Choice::Othr(
                    pacs008::GenericAccountIdentification1 {
                        id: pacs008::Max34Text(raw.clone()),
                        schme_nm: None,
                        issr: None,
                    },
                ))
            })
        });

    id_choice.map(|id| pacs008::CashAccount40 {
        id: Some(id),
        tp: None,
        ccy: None,
        nm: None,
        prxy: None,
    })
}

// ---------------------------------------------------------------------------
// Charge-bearer helpers
// ---------------------------------------------------------------------------

/// Map an MT `:71A:` charge code to a `pacs.008` `ChargeBearerType1Code`.
///
/// * `SHA` → `Shar` (shared)
/// * `OUR` → `Debt` (debtor bears all charges)
/// * `BEN` → `Cred` (creditor bears all charges)
/// * anything else → `Shar` (default)
#[allow(clippy::match_same_arms)] // explicit SHA arm for readability
pub fn charges_to_code(charges: &str) -> pacs008::ChargeBearerType1Code {
    match charges.trim() {
        "SHA" => pacs008::ChargeBearerType1Code::Shar,
        "OUR" => pacs008::ChargeBearerType1Code::Debt,
        "BEN" => pacs008::ChargeBearerType1Code::Cred,
        _ => pacs008::ChargeBearerType1Code::Shar,
    }
}

/// Map a `pacs.008` `ChargeBearerType1Code` back to an MT `:71A:` value.
///
/// All arms are explicit so that new codegen variants cause a compile error
/// instead of silently defaulting to `"SHA"`.
#[allow(clippy::match_same_arms)] // exhaustive so codegen additions produce compile errors
pub fn code_to_charges(code: &pacs008::ChargeBearerType1Code) -> &'static str {
    match code {
        pacs008::ChargeBearerType1Code::Debt => "OUR",
        pacs008::ChargeBearerType1Code::Cred => "BEN",
        pacs008::ChargeBearerType1Code::Shar => "SHA",
        pacs008::ChargeBearerType1Code::Slev => "SHA", // no MT equivalent
    }
}

// ---------------------------------------------------------------------------
// Date format helpers
// ---------------------------------------------------------------------------

/// Convert an ISO date string (`YYYY-MM-DD`) to the SWIFT six-digit format
/// (`YYMMDD`).
///
/// # Errors
///
/// Returns [`TranslationError::InvalidFieldValue`] when the string is not a
/// well-formed `YYYY-MM-DD` date.
pub fn iso_date_to_yymmdd(iso: &str) -> Result<String, TranslationError> {
    // Expected: YYYY-MM-DD (10 chars)
    let parts: Vec<&str> = iso.split('-').collect();
    if parts.len() != 3 || parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
        return Err(TranslationError::InvalidFieldValue {
            field: "date".into(),
            detail: format!("expected ISO date YYYY-MM-DD, got '{iso}'"),
        });
    }
    let yy = &parts[0][2..]; // last two digits of the year
    Ok(format!("{}{}{}", yy, parts[1], parts[2]))
}

// ---------------------------------------------------------------------------
// MT message formatter
// ---------------------------------------------------------------------------

/// Format a complete MT message string from its constituent parts.
///
/// Produces the standard SWIFT FIN block structure:
/// ```text
/// {1:F01{sender}0000000000}{2:I{msg_type}{receiver}N}{4:
/// :tag:value
/// ...
/// -}{5:{CHK:000000000000}}
/// ```
pub fn format_mt_message(
    msg_type: &str,
    sender_bic: &str,
    receiver_bic: &str,
    fields: &[(String, String)],
) -> String {
    // Pad BIC to 12 chars (LT address format)
    let sender_lt = pad_lt_address(sender_bic);
    let receiver_lt = pad_lt_address(receiver_bic);

    let mut body = String::new();
    for (tag, value) in fields {
        body.push(':');
        body.push_str(tag);
        body.push(':');
        body.push_str(value);
        body.push('\n');
    }

    format!(
        "{{1:F01{sender_lt}0000000000}}{{2:I{msg_type}{receiver_lt}N}}{{4:\n{body}-}}{{5:{{CHK:000000000000}}}}"
    )
}

/// Pad or truncate a BIC to a 12-character LT address.
fn pad_lt_address(bic: &str) -> String {
    // LT address = BIC8 (4+2+2) + LT letter (X) + branch (XXX) = 12 chars
    let clean: String = bic.chars().filter(char::is_ascii_alphanumeric).collect();
    match clean.len() {
        8 => format!("{clean}XXXX"),
        11 => format!("{clean}X"),
        12 => clean,
        n if n < 8 => format!("{clean:X<12}"), // pad with X
        _ => clean[..12].to_string(),
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_date_to_yymmdd() {
        assert_eq!(iso_date_to_yymmdd("2023-06-15").unwrap(), "230615");
        assert_eq!(iso_date_to_yymmdd("1999-12-31").unwrap(), "991231");
    }

    #[test]
    fn test_iso_date_to_yymmdd_invalid() {
        assert!(iso_date_to_yymmdd("2023-6-15").is_err());
        assert!(iso_date_to_yymmdd("20230615").is_err());
    }

    #[test]
    fn test_charges_to_code() {
        assert!(matches!(
            charges_to_code("SHA"),
            pacs008::ChargeBearerType1Code::Shar
        ));
        assert!(matches!(
            charges_to_code("OUR"),
            pacs008::ChargeBearerType1Code::Debt
        ));
        assert!(matches!(
            charges_to_code("BEN"),
            pacs008::ChargeBearerType1Code::Cred
        ));
    }

    #[test]
    fn test_code_to_charges() {
        assert_eq!(
            code_to_charges(&pacs008::ChargeBearerType1Code::Shar),
            "SHA"
        );
        assert_eq!(
            code_to_charges(&pacs008::ChargeBearerType1Code::Debt),
            "OUR"
        );
        assert_eq!(
            code_to_charges(&pacs008::ChargeBearerType1Code::Cred),
            "BEN"
        );
    }

    #[test]
    fn test_party_to_party_id_name() {
        let party = PartyInfo {
            name: Some("JOHN DOE".into()),
            ..Default::default()
        };
        let id = party_to_party_id(&party);
        assert_eq!(id.nm.as_ref().map(|n| n.0.as_str()), Some("JOHN DOE"));
    }

    #[test]
    fn test_account_to_cash_account_iban() {
        let acct = Account {
            iban: Some("DE89370400440532013000".into()),
            bic: None,
            account: None,
        };
        let ca = account_to_cash_account(&acct).unwrap();
        if let Some(ChoiceWrapper {
            inner: pacs008::AccountIdentification4Choice::IBAN(iban),
        }) = ca.id
        {
            assert_eq!(iban.0, "DE89370400440532013000");
        } else {
            panic!("expected IBAN variant");
        }
    }

    #[test]
    fn test_account_to_cash_account_empty() {
        let acct = Account {
            iban: None,
            bic: None,
            account: None,
        };
        assert!(account_to_cash_account(&acct).is_none());
    }
}
