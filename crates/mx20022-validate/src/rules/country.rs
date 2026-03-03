//! ISO 3166-1 alpha-2 country code validation rule.
//!
//! Validates that a 2-letter uppercase code is a recognised ISO 3166-1 alpha-2
//! country code.  The static list covers all 249 codes in the current
//! ISO 3166 Maintenance Agency publication (2024 edition), plus the
//! exceptional reservations commonly encountered in financial messages
//! (XK for Kosovo).

use crate::error::{Severity, ValidationError};
use crate::rules::Rule;

/// Static set of recognised ISO 3166-1 alpha-2 country codes.
static VALID_COUNTRIES: &[&str] = &[
    "AD", "AE", "AF", "AG", "AI", "AL", "AM", "AO", "AQ", "AR", "AS", "AT", "AU", "AW", "AX", "AZ",
    "BA", "BB", "BD", "BE", "BF", "BG", "BH", "BI", "BJ", "BL", "BM", "BN", "BO", "BQ", "BR", "BS",
    "BT", "BV", "BW", "BY", "BZ", "CA", "CC", "CD", "CF", "CG", "CH", "CI", "CK", "CL", "CM", "CN",
    "CO", "CR", "CU", "CV", "CW", "CX", "CY", "CZ", "DE", "DJ", "DK", "DM", "DO", "DZ", "EC", "EE",
    "EG", "EH", "ER", "ES", "ET", "FI", "FJ", "FK", "FM", "FO", "FR", "GA", "GB", "GD", "GE", "GF",
    "GG", "GH", "GI", "GL", "GM", "GN", "GP", "GQ", "GR", "GS", "GT", "GU", "GW", "GY", "HK", "HM",
    "HN", "HR", "HT", "HU", "ID", "IE", "IL", "IM", "IN", "IO", "IQ", "IR", "IS", "IT", "JE", "JM",
    "JO", "JP", "KE", "KG", "KH", "KI", "KM", "KN", "KP", "KR", "KW", "KY", "KZ", "LA", "LB", "LC",
    "LI", "LK", "LR", "LS", "LT", "LU", "LV", "LY", "MA", "MC", "MD", "ME", "MF", "MG", "MH", "MK",
    "ML", "MM", "MN", "MO", "MP", "MQ", "MR", "MS", "MT", "MU", "MV", "MW", "MX", "MY", "MZ", "NA",
    "NC", "NE", "NF", "NG", "NI", "NL", "NO", "NP", "NR", "NU", "NZ", "OM", "PA", "PE", "PF", "PG",
    "PH", "PK", "PL", "PM", "PN", "PR", "PS", "PT", "PW", "PY", "QA", "RE", "RO", "RS", "RU", "RW",
    "SA", "SB", "SC", "SD", "SE", "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SR", "SS",
    "ST", "SV", "SX", "SY", "SZ", "TC", "TD", "TF", "TG", "TH", "TJ", "TK", "TL", "TM", "TN", "TO",
    "TR", "TT", "TV", "TW", "TZ", "UA", "UG", "UM", "US", "UY", "UZ", "VA", "VC", "VE", "VG", "VI",
    "VN", "VU", "WF", "WS", "YE", "YT", "ZA", "ZM", "ZW",
    // User-assigned / exceptional reservations commonly used in finance
    "XK", // Kosovo (SWIFT, EU usage)
];

/// Validates a value as an ISO 3166-1 alpha-2 country code.
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::country::CountryCodeRule;
/// use mx20022_validate::rules::Rule;
///
/// let rule = CountryCodeRule;
///
/// let errors = rule.validate("GB", "/path");
/// assert!(errors.is_empty(), "GB is a valid country code");
///
/// let errors = rule.validate("gb", "/path");
/// assert!(!errors.is_empty(), "Lowercase codes are rejected");
///
/// let errors = rule.validate("XX", "/path");
/// assert!(!errors.is_empty(), "XX is not a valid ISO 3166-1 code");
/// ```
pub struct CountryCodeRule;

impl Rule for CountryCodeRule {
    fn id(&self) -> &'static str {
        "COUNTRY_CHECK"
    }

    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
        if VALID_COUNTRIES.contains(&value) {
            return vec![];
        }

        let msg = if value.is_empty() {
            "Country code must not be empty".to_owned()
        } else if value.len() != 2 {
            format!(
                "Country code must be exactly 2 characters, got {}: `{value}`",
                value.len()
            )
        } else if !value.chars().all(|c| c.is_ascii_alphabetic()) {
            format!("Country code must be 2 alphabetic characters, got: `{value}`")
        } else if !value.chars().all(|c| c.is_ascii_uppercase()) {
            format!("Country code must be uppercase, got: `{value}`")
        } else {
            format!("Unrecognised ISO 3166-1 alpha-2 country code: `{value}`")
        };

        vec![ValidationError::new(
            path,
            Severity::Error,
            "COUNTRY_CHECK",
            msg,
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Rule;

    const VALID_CODES: &[&str] = &[
        "US", "GB", "DE", "FR", "JP", "CN", "AU", "CA", "IN", "BR", "MX", "RU", "KR", "ZA", "SG",
        "HK", "NL", "SE", "NO", "DK", "FI", "PL", "CZ", "HU", "RO", "UA", "TR", "SA", "AE", "QA",
        "KW", "BH", "OM", "EG", "NG", "KE", "GH", "TZ", "MY", "ID", "TH", "PH", "VN", "PK", "BD",
        "NZ", "CH", "AT", "BE", "IE", "PT", "ES", "IT", "GR", "IL", "AR", "CL", "CO", "PE", "VE",
        "XK", // Kosovo exceptional reservation
    ];

    const INVALID_CODES: &[&str] = &[
        "us",  // lowercase
        "Gb",  // mixed case
        "USA", // too long (3 chars)
        "U",   // too short (1 char)
        "12",  // numeric
        "U1",  // contains digit
        "",    // empty
        "XX",  // unassigned
        "AA",  // unassigned
        "QQ",  // fictional
    ];

    #[test]
    fn valid_country_codes_pass() {
        let rule = CountryCodeRule;
        for code in VALID_CODES {
            let errors = rule.validate(code, "/test");
            assert!(
                errors.is_empty(),
                "Expected no errors for valid code `{code}`, got: {errors:?}"
            );
        }
    }

    #[test]
    fn invalid_country_codes_fail() {
        let rule = CountryCodeRule;
        for code in INVALID_CODES {
            let errors = rule.validate(code, "/test");
            assert!(
                !errors.is_empty(),
                "Expected errors for invalid code `{code}`"
            );
        }
    }

    #[test]
    fn error_has_correct_rule_id_and_path() {
        let rule = CountryCodeRule;
        let errors = rule.validate("XX", "/Document/Ctry");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "COUNTRY_CHECK");
        assert_eq!(errors[0].path, "/Document/Ctry");
        assert_eq!(errors[0].severity, Severity::Error);
    }

    #[test]
    fn rule_id_is_country_check() {
        assert_eq!(CountryCodeRule.id(), "COUNTRY_CHECK");
    }

    #[test]
    fn empty_code_rejected() {
        let rule = CountryCodeRule;
        let errors = rule.validate("", "/test");
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("empty"));
    }

    #[test]
    fn three_char_code_rejected_with_length_message() {
        let rule = CountryCodeRule;
        let errors = rule.validate("USA", "/test");
        assert!(!errors.is_empty());
        assert!(
            errors[0].message.contains("2 characters") || errors[0].message.contains("exactly"),
            "Expected length message, got: {}",
            errors[0].message
        );
    }

    #[test]
    fn lowercase_code_rejected() {
        let rule = CountryCodeRule;
        let errors = rule.validate("us", "/test");
        assert!(!errors.is_empty());
    }
}
