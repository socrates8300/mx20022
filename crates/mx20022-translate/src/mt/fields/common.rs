//! Shared field-parsing utilities used by all MT message parsers.
//!
//! These helpers convert SWIFT-encoded values (comma-decimal amounts, YYMMDD
//! dates, account strings, party blocks) into normalised Rust types.

use super::super::error::MtError;
use super::super::types::{Block4, TagField};

// ---------------------------------------------------------------------------
// Value types
// ---------------------------------------------------------------------------

/// A currency/amount pair extracted from a SWIFT amount field.
///
/// The amount is stored as a normalised decimal string using a full stop (`.`)
/// as the decimal separator, matching ISO 20022 conventions.
///
/// # Example
///
/// ```rust
/// use mx20022_translate::mt::fields::common::{Amount, parse_amount};
/// let a = parse_amount("EUR1000,50").unwrap();
/// assert_eq!(a.currency, "EUR");
/// assert_eq!(a.value, "1000.50");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Amount {
    /// ISO 4217 currency code.
    pub currency: String,
    /// Decimal amount string, e.g. `"1000.50"`.
    pub value: String,
}

/// A parsed SWIFT account identifier.
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    /// IBAN extracted from a `/IBAN` prefix, if present.
    pub iban: Option<String>,
    /// BIC extracted from a `//BIC/` prefix, if present.
    pub bic: Option<String>,
    /// Raw account number (anything that is neither IBAN nor BIC).
    pub account: Option<String>,
}

/// Name and address information for a party (ordering customer, beneficiary, etc.).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyInfo {
    /// Parsed account identifier from the first line when it starts with `/`.
    pub account: Option<Account>,
    /// Party name (first non-account line).
    pub name: Option<String>,
    /// Any additional address lines.
    pub address_lines: Vec<String>,
}

// ---------------------------------------------------------------------------
// Amount parsing
// ---------------------------------------------------------------------------

/// Parse a SWIFT currency/amount string such as `"EUR1000,50"` into an
/// [`Amount`].
///
/// SWIFT uses a comma as the decimal separator; this function converts it to a
/// full stop.
///
/// # Errors
///
/// Returns [`MtError::InvalidFieldValue`] when the string is shorter than
/// four characters or the currency code is not exactly three letters.
pub fn parse_amount(s: &str) -> Result<Amount, MtError> {
    if s.len() < 4 {
        return Err(MtError::InvalidFieldValue {
            tag: "amount".into(),
            detail: format!("too short to contain currency + amount: '{s}'"),
        });
    }

    let currency = s[..3].to_string();
    if !currency.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(MtError::InvalidFieldValue {
            tag: "amount".into(),
            detail: format!("invalid currency code '{currency}'"),
        });
    }

    let raw_amount = &s[3..];
    let value = raw_amount.replace(',', ".");

    Ok(Amount { currency, value })
}

// ---------------------------------------------------------------------------
// Date parsing
// ---------------------------------------------------------------------------

/// Convert a SWIFT six-digit date (`YYMMDD`) into a full ISO 8601 date string
/// (`YYYY-MM-DD`).
///
/// The century is inferred using the conventional SWIFT rule:
/// - `YY >= 80` → 19xx
/// - `YY < 80`  → 20xx
///
/// # Errors
///
/// Returns [`MtError::InvalidFieldValue`] when the string is not exactly six
/// ASCII digits.
///
/// # Panics
///
/// Does not panic in practice; the `expect` is unreachable after the length
/// and digit validation above.
///
/// # Example
///
/// ```rust
/// use mx20022_translate::mt::fields::common::parse_date_yymmdd;
/// assert_eq!(parse_date_yymmdd("230615").unwrap(), "2023-06-15");
/// assert_eq!(parse_date_yymmdd("991231").unwrap(), "1999-12-31");
/// ```
pub fn parse_date_yymmdd(s: &str) -> Result<String, MtError> {
    if s.len() != 6 || !s.chars().all(|c| c.is_ascii_digit()) {
        return Err(MtError::InvalidFieldValue {
            tag: "date".into(),
            detail: format!("expected 6-digit YYMMDD date, got '{s}'"),
        });
    }

    let yy: u32 = s[..2].parse().expect("checked: string is 6 ASCII digits");
    let mm = &s[2..4];
    let dd = &s[4..6];
    let century = if yy >= 80 { 1900u32 } else { 2000u32 };
    let year = century + yy;

    Ok(format!("{year:04}-{mm}-{dd}"))
}

/// Convert a SWIFT four-digit month-day (`MMDD`) into a full ISO 8601 date
/// string using the supplied year.
pub fn parse_date_mmdd(year: u32, s: &str) -> Result<String, MtError> {
    if s.len() != 4 || !s.chars().all(|c| c.is_ascii_digit()) {
        return Err(MtError::InvalidFieldValue {
            tag: "date".into(),
            detail: format!("expected 4-digit MMDD date, got '{s}'"),
        });
    }
    let mm = &s[..2];
    let dd = &s[2..4];
    Ok(format!("{year:04}-{mm}-{dd}"))
}

// ---------------------------------------------------------------------------
// Account parsing
// ---------------------------------------------------------------------------

/// Parse a SWIFT account identification string.
///
/// Recognised prefixes:
/// - `/IBAN` — a single leading `/` followed by the IBAN.
/// - `//BIC/ACCT` — double leading `//` means a BIC, followed by `/` and the
///   account number.
/// - Anything else is stored as a raw account number.
pub fn parse_account(s: &str) -> Result<Account, MtError> {
    if let Some(rest) = s.strip_prefix("//") {
        // //BIC or //BIC/account
        if let Some((bic, acct)) = rest.split_once('/') {
            Ok(Account {
                iban: None,
                bic: Some(bic.to_string()),
                account: Some(acct.to_string()),
            })
        } else {
            Ok(Account {
                iban: None,
                bic: Some(rest.to_string()),
                account: None,
            })
        }
    } else if let Some(rest) = s.strip_prefix('/') {
        // /IBAN  (most common in MT103/MT940)
        Ok(Account {
            iban: Some(rest.to_string()),
            bic: None,
            account: None,
        })
    } else {
        Ok(Account {
            iban: None,
            bic: None,
            account: Some(s.to_string()),
        })
    }
}

// ---------------------------------------------------------------------------
// Party info parsing
// ---------------------------------------------------------------------------

/// Parse a multiline party block into a [`PartyInfo`].
///
/// The first line is examined for a leading `/` which signals an account
/// identifier.  Subsequent lines are treated as name (first) then address.
///
/// # Panics
///
/// Does not panic in practice; the `expect` is unreachable after the
/// `is_empty` guard above.
pub fn parse_party_lines(lines: &[&str]) -> PartyInfo {
    if lines.is_empty() {
        return PartyInfo::default();
    }

    let mut iter = lines.iter().peekable();
    let first = *iter.next().expect("checked: lines is non-empty");

    let (account, name_from_first) = if first.starts_with('/') {
        let acct = parse_account(first).ok();
        (acct, None)
    } else {
        (None, Some(first.to_string()))
    };

    // Collect remaining lines.
    let mut remaining: Vec<String> = iter.map(|l| (*l).to_string()).collect();

    let name = if let Some(n) = name_from_first {
        Some(n)
    } else if !remaining.is_empty() {
        Some(remaining.remove(0))
    } else {
        None
    };

    PartyInfo {
        account,
        name,
        address_lines: remaining,
    }
}

/// Split a tagged field value on newlines and delegate to [`parse_party_lines`].
pub fn parse_party_value(value: &str) -> PartyInfo {
    let lines: Vec<&str> = value.lines().collect();
    parse_party_lines(&lines)
}

// ---------------------------------------------------------------------------
// Shared field lookup + parsing helpers
// ---------------------------------------------------------------------------

/// Look up a required field in `block4` by tag name.
///
/// Returns [`MtError::MissingField`] when the tag is absent.
pub(crate) fn require_field<'a>(
    block4: &'a Block4,
    tag: &str,
    message_type: &str,
) -> Result<&'a TagField, MtError> {
    block4.get(tag).ok_or_else(|| MtError::MissingField {
        tag: tag.to_string(),
        message_type: message_type.to_string(),
    })
}

/// Parse a `:32A:` value: `YYMMDD` + currency (3) + amount.
///
/// Returns `(date, currency, amount)` on success.
pub(crate) fn parse_32a(s: &str, tag: &str) -> Result<(String, String, String), MtError> {
    if s.len() < 10 {
        return Err(MtError::InvalidFieldValue {
            tag: tag.to_string(),
            detail: format!("too short: '{s}'"),
        });
    }
    let date = parse_date_yymmdd(&s[..6]).map_err(|_| MtError::InvalidFieldValue {
        tag: tag.to_string(),
        detail: format!("invalid date in '{s}'"),
    })?;
    let rest = &s[6..];
    let amt = parse_amount(rest)?;
    Ok((date, amt.currency, amt.value))
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_amount_basic() {
        let a = parse_amount("EUR1000,50").unwrap();
        assert_eq!(a.currency, "EUR");
        assert_eq!(a.value, "1000.50");
    }

    #[test]
    fn test_parse_amount_no_decimal() {
        let a = parse_amount("USD50000,00").unwrap();
        assert_eq!(a.currency, "USD");
        assert_eq!(a.value, "50000.00");
    }

    #[test]
    fn test_parse_amount_short_fails() {
        assert!(parse_amount("EU").is_err());
    }

    #[test]
    fn test_parse_date_yymmdd_century_20() {
        assert_eq!(parse_date_yymmdd("230615").unwrap(), "2023-06-15");
    }

    #[test]
    fn test_parse_date_yymmdd_century_19() {
        assert_eq!(parse_date_yymmdd("991231").unwrap(), "1999-12-31");
    }

    #[test]
    fn test_parse_date_yymmdd_boundary() {
        // yy = 80 → 1980
        assert_eq!(parse_date_yymmdd("800101").unwrap(), "1980-01-01");
        // yy = 79 → 2079
        assert_eq!(parse_date_yymmdd("790101").unwrap(), "2079-01-01");
    }

    #[test]
    fn test_parse_date_invalid() {
        assert!(parse_date_yymmdd("2306").is_err());
        assert!(parse_date_yymmdd("23061X").is_err());
    }

    #[test]
    fn test_parse_account_iban() {
        let a = parse_account("/DE89370400440532013000").unwrap();
        assert_eq!(a.iban.as_deref(), Some("DE89370400440532013000"));
        assert!(a.bic.is_none());
    }

    #[test]
    fn test_parse_account_bic_account() {
        let a = parse_account("//CHASUS33/1234567890").unwrap();
        assert_eq!(a.bic.as_deref(), Some("CHASUS33"));
        assert_eq!(a.account.as_deref(), Some("1234567890"));
    }

    #[test]
    fn test_parse_account_raw() {
        let a = parse_account("1234567890").unwrap();
        assert_eq!(a.account.as_deref(), Some("1234567890"));
    }

    #[test]
    fn test_parse_party_lines_with_account() {
        let lines = vec!["/DE89370400440532013000", "JOHN DOE", "123 MAIN ST"];
        let p = parse_party_lines(&lines);
        assert!(p.account.is_some());
        assert_eq!(p.name.as_deref(), Some("JOHN DOE"));
        assert_eq!(p.address_lines, vec!["123 MAIN ST"]);
    }

    #[test]
    fn test_parse_party_lines_name_only() {
        let lines = vec!["JOHN DOE"];
        let p = parse_party_lines(&lines);
        assert!(p.account.is_none());
        assert_eq!(p.name.as_deref(), Some("JOHN DOE"));
        assert!(p.address_lines.is_empty());
    }

    #[test]
    fn test_parse_party_empty() {
        let p = parse_party_lines(&[]);
        assert!(p.account.is_none());
        assert!(p.name.is_none());
    }
}
