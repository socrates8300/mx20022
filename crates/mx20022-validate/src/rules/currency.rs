//! ISO 4217 currency code validation rule.
//!
//! Validates that a 3-letter alphabetic code is a recognised ISO 4217
//! currency code.  The set covers all actively-traded currencies plus the
//! major precious-metal and testing codes (XAU, XAG, XXX, …).

use crate::error::{Severity, ValidationError};
use crate::rules::Rule;

/// Static set of recognised ISO 4217 currency codes.
///
/// Sources: ISO 4217 Maintenance Agency list (2024 edition).
/// Codes are 3-letter uppercase ASCII.
static VALID_CURRENCIES: &[&str] = &[
    // Major traded currencies
    "AED", "AFN", "ALL", "AMD", "ANG", "AOA", "ARS", "AUD", "AWG", "AZN", "BAM", "BBD", "BDT",
    "BGN", "BHD", "BMD", "BND", "BOB", "BOV", "BRL", "BSD", "BTN", "BWP", "BYN", "BZD", "CAD",
    "CDF", "CHE", "CHF", "CHW", "CLF", "CLP", "CNY", "COP", "COU", "CRC", "CUP", "CVE", "CZK",
    "DJF", "DKK", "DOP", "DZD", "EGP", "ERN", "ETB", "EUR", "FJD", "FKP", "GBP", "GEL", "GHS",
    "GIP", "GMD", "GNF", "GTQ", "GYD", "HKD", "HNL", "HTG", "HUF", "IDR", "ILS", "INR", "IQD",
    "IRR", "ISK", "JMD", "JOD", "JPY", "KES", "KGS", "KHR", "KMF", "KPW", "KRW", "KWD", "KYD",
    "KZT", "LAK", "LBP", "LKR", "LRD", "LSL", "LYD", "MAD", "MDL", "MGA", "MKD", "MMK", "MNT",
    "MOP", "MRU", "MUR", "MVR", "MWK", "MXN", "MXV", "MYR", "MZN", "NAD", "NGN", "NIO", "NOK",
    "NPR", "NZD", "OMR", "PAB", "PEN", "PGK", "PHP", "PKR", "PLN", "PYG", "QAR", "RON", "RSD",
    "RUB", "RWF", "SAR", "SBD", "SCR", "SDG", "SEK", "SGD", "SHP", "SLE", "SOS", "SRD", "SSP",
    "STN", "SVC", "SYP", "SZL", "THB", "TJS", "TMT", "TND", "TOP", "TRY", "TTD", "TWD", "TZS",
    "UAH", "UGX", "USD", "USN", "UYI", "UYU", "UYW", "UZS", "VED", "VES", "VND", "VUV", "WST",
    "XAF", "XAG", "XAU", "XBA", "XBB", "XBC", "XBD", "XCD", "XDR", "XOF", "XPD", "XPF", "XPT",
    "XSU", "XTS", "XUA", "XXX", "YER", "ZAR", "ZMW", "ZWG",
    // Historical codes still appearing in legacy ISO 20022 messages
    "HRK", "SLL", "STD", "VEF", "MRO", "BYR",
];

/// Validates a value as an ISO 4217 currency code.
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::currency::CurrencyRule;
/// use mx20022_validate::rules::Rule;
///
/// let rule = CurrencyRule;
///
/// let errors = rule.validate("USD", "/path");
/// assert!(errors.is_empty(), "USD is a valid currency code");
///
/// let errors = rule.validate("XYZ", "/path");
/// assert!(!errors.is_empty(), "XYZ is not a valid ISO 4217 code");
///
/// let errors = rule.validate("usd", "/path");
/// assert!(!errors.is_empty(), "Lowercase codes are rejected");
/// ```
pub struct CurrencyRule;

impl Rule for CurrencyRule {
    fn id(&self) -> &'static str {
        "CURRENCY_CHECK"
    }

    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
        if VALID_CURRENCIES.contains(&value) {
            return vec![];
        }

        let msg = if value.is_empty() {
            "Currency code must not be empty".to_owned()
        } else if value.len() != 3 {
            format!(
                "Currency code must be exactly 3 characters, got {}: `{value}`",
                value.len()
            )
        } else if !value.chars().all(|c| c.is_ascii_alphabetic()) {
            format!("Currency code must be 3 alphabetic characters, got: `{value}`")
        } else if !value.chars().all(|c| c.is_ascii_uppercase()) {
            format!("Currency code must be uppercase, got: `{value}`")
        } else {
            format!("Unrecognised ISO 4217 currency code: `{value}`")
        };

        vec![ValidationError::new(
            path,
            Severity::Error,
            "CURRENCY_CHECK",
            msg,
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Rule;

    // A representative sample of valid ISO 4217 codes
    const VALID_CODES: &[&str] = &[
        "USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "NZD", "CNY", "INR", "BRL", "MXN", "SGD",
        "HKD", "KRW", "ZAR", "SEK", "NOK", "DKK", "PLN", "CZK", "HUF", "TRY", "THB", "MYR", "IDR",
        "PHP", "AED", "SAR", "QAR", "KWD", "BHD", "OMR", "EGP", "ILS", "TWD", "ARS", "CLP", "COP",
        "PEN", "NGN", "KES", "GHS", "TZS", "RUB", "UAH", "RON", "BGN", "ISK",
        // ISO special codes
        "XAU", "XAG", "XDR", "XXX", "XAF", "XOF", // Historical legacy codes
        "HRK",
    ];

    const INVALID_CODES: &[&str] = &[
        "usd",  // lowercase
        "Usd",  // mixed case
        "US",   // too short (2 chars)
        "USDX", // too long (4 chars)
        "123",  // numeric
        "U1D",  // contains digit
        "",     // empty
        "ZZZ",  // fictional (not in set — note XTS/XXX are valid testing codes but ZZZ is not)
        "ABC",  // fictional
    ];

    #[test]
    fn valid_currency_codes_pass() {
        let rule = CurrencyRule;
        for code in VALID_CODES {
            let errors = rule.validate(code, "/test");
            assert!(
                errors.is_empty(),
                "Expected no errors for valid code `{code}`, got: {errors:?}"
            );
        }
    }

    #[test]
    fn invalid_currency_codes_fail() {
        let rule = CurrencyRule;
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
        let rule = CurrencyRule;
        let errors = rule.validate("ABC", "/Document/Ccy");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "CURRENCY_CHECK");
        assert_eq!(errors[0].path, "/Document/Ccy");
        assert_eq!(errors[0].severity, Severity::Error);
    }

    #[test]
    fn rule_id_is_currency_check() {
        assert_eq!(CurrencyRule.id(), "CURRENCY_CHECK");
    }

    #[test]
    fn lowercase_code_rejected_with_descriptive_message() {
        let rule = CurrencyRule;
        let errors = rule.validate("usd", "/test");
        assert!(!errors.is_empty());
        assert!(
            errors[0].message.contains("uppercase") || errors[0].message.contains("Unrecognised"),
            "Expected message about case, got: {}",
            errors[0].message
        );
    }

    #[test]
    fn two_char_code_rejected_with_length_message() {
        let rule = CurrencyRule;
        let errors = rule.validate("US", "/test");
        assert!(!errors.is_empty());
        assert!(
            errors[0].message.contains("3 characters") || errors[0].message.contains("exactly"),
            "Expected length message, got: {}",
            errors[0].message
        );
    }

    #[test]
    fn empty_code_rejected() {
        let rule = CurrencyRule;
        let errors = rule.validate("", "/test");
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("empty"));
    }

    #[test]
    fn numeric_code_rejected() {
        let rule = CurrencyRule;
        let errors = rule.validate("123", "/test");
        assert!(!errors.is_empty());
    }
}
