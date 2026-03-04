//! Runtime tests for newtype constraint validation via `new()` / `TryFrom`.
//!
//! These tests verify that the code-generated validators correctly accept and
//! reject values according to their XSD facets (Pattern, MinLength, MaxLength).

use mx20022_model::common::validate::ConstraintKind;
use mx20022_model::generated::pacs::pacs_008_001_13::{
    ActiveCurrencyCode, BICFIDec2014Identifier, CountryCode, IBAN2007Identifier, Max35Text,
    UUIDv4Identifier,
};

// ---------------------------------------------------------------------------
// ActiveCurrencyCode — Pattern: [A-Z]{3,3}
// ---------------------------------------------------------------------------

#[test]
fn active_currency_code_valid() {
    assert!(ActiveCurrencyCode::new("USD").is_ok());
    assert!(ActiveCurrencyCode::new("EUR").is_ok());
    assert!(ActiveCurrencyCode::new("GBP").is_ok());
    assert!(ActiveCurrencyCode::try_from("JPY".to_owned()).is_ok());
}

#[test]
fn active_currency_code_too_short() {
    let err = ActiveCurrencyCode::new("US").unwrap_err();
    assert_eq!(err.kind, ConstraintKind::Pattern);
}

#[test]
fn active_currency_code_too_long() {
    let err = ActiveCurrencyCode::new("USDD").unwrap_err();
    assert_eq!(err.kind, ConstraintKind::Pattern);
}

#[test]
fn active_currency_code_wrong_chars() {
    assert!(ActiveCurrencyCode::new("us1").is_err());
    assert!(ActiveCurrencyCode::new("123").is_err());
    assert!(ActiveCurrencyCode::new("usd").is_err());
}

#[test]
fn active_currency_code_empty() {
    let err = ActiveCurrencyCode::new("").unwrap_err();
    assert_eq!(err.kind, ConstraintKind::Pattern);
}

// ---------------------------------------------------------------------------
// CountryCode — Pattern: [A-Z]{2,2}
// ---------------------------------------------------------------------------

#[test]
fn country_code_valid() {
    assert!(CountryCode::new("US").is_ok());
    assert!(CountryCode::new("GB").is_ok());
    assert!(CountryCode::try_from("DE".to_owned()).is_ok());
}

#[test]
fn country_code_invalid() {
    assert!(CountryCode::new("").is_err());
    assert!(CountryCode::new("U").is_err());
    assert!(CountryCode::new("USA").is_err());
    assert!(CountryCode::new("us").is_err());
    assert!(CountryCode::new("12").is_err());
}

// ---------------------------------------------------------------------------
// BICFIDec2014Identifier — Pattern: [A-Z0-9]{4}[A-Z]{2}[A-Z0-9]{2}([A-Z0-9]{3}){0,1}
// ---------------------------------------------------------------------------

#[test]
fn bic_valid_8_char() {
    assert!(BICFIDec2014Identifier::new("AAAAGB2L").is_ok());
    assert!(BICFIDec2014Identifier::new("DEUTDEFF").is_ok());
}

#[test]
fn bic_valid_11_char() {
    assert!(BICFIDec2014Identifier::new("AAAAGB2LXXX").is_ok());
    assert!(BICFIDec2014Identifier::new("DEUTDEFF500").is_ok());
}

#[test]
fn bic_invalid_9_char() {
    // 9 chars: too long for 8, too short for 11
    let err = BICFIDec2014Identifier::new("AAAAGB2LX").unwrap_err();
    assert_eq!(err.kind, ConstraintKind::Pattern);
}

#[test]
fn bic_invalid_12_char() {
    let err = BICFIDec2014Identifier::new("AAAAGB2LXXXX").unwrap_err();
    assert_eq!(err.kind, ConstraintKind::Pattern);
}

#[test]
fn bic_invalid_lowercase() {
    let err = BICFIDec2014Identifier::new("aaaagb2l").unwrap_err();
    assert_eq!(err.kind, ConstraintKind::Pattern);
}

#[test]
fn bic_empty() {
    assert!(BICFIDec2014Identifier::new("").is_err());
}

// ---------------------------------------------------------------------------
// IBAN2007Identifier — Pattern: [A-Z]{2}[0-9]{2}[a-zA-Z0-9]{1,30}
// ---------------------------------------------------------------------------

#[test]
fn iban_valid() {
    assert!(IBAN2007Identifier::new("GB82WEST12345698765432").is_ok());
    assert!(IBAN2007Identifier::new("DE89370400440532013000").is_ok());
    // Minimal: 2 alpha + 2 digit + 1 alnum = 5 chars
    assert!(IBAN2007Identifier::new("AB12x").is_ok());
}

#[test]
fn iban_valid_max_length() {
    // 2 + 2 + 30 = 34 chars total
    let max = format!("AB12{}", "a".repeat(30));
    assert!(IBAN2007Identifier::new(&max).is_ok());
}

#[test]
fn iban_too_short() {
    // Missing the alnum trailing part
    assert!(IBAN2007Identifier::new("AB12").is_err());
    assert!(IBAN2007Identifier::new("AB").is_err());
    assert!(IBAN2007Identifier::new("").is_err());
}

#[test]
fn iban_too_long() {
    // 2 + 2 + 31 = 35 chars — exceeds max 30 alnum
    let too_long = format!("AB12{}", "a".repeat(31));
    assert!(IBAN2007Identifier::new(&too_long).is_err());
}

#[test]
fn iban_wrong_prefix() {
    // Lowercase country code
    assert!(IBAN2007Identifier::new("gb82WEST12345698765432").is_err());
    // Digits in country code position
    assert!(IBAN2007Identifier::new("12XX1234").is_err());
}

// ---------------------------------------------------------------------------
// Max35Text — MinLength 1, MaxLength 35
// ---------------------------------------------------------------------------

#[test]
fn max35text_valid() {
    assert!(Max35Text::new("hello").is_ok());
    assert!(Max35Text::new("a]").is_ok());
}

#[test]
fn max35text_min_boundary() {
    // Exactly 1 char — should pass
    assert!(Max35Text::new("a").is_ok());
}

#[test]
fn max35text_max_boundary() {
    // Exactly 35 chars — should pass
    assert!(Max35Text::new(&"a".repeat(35)).is_ok());
}

#[test]
fn max35text_empty_rejected() {
    let err = Max35Text::new("").unwrap_err();
    assert_eq!(err.kind, ConstraintKind::MinLength);
}

#[test]
fn max35text_too_long() {
    let err = Max35Text::new(&"a".repeat(36)).unwrap_err();
    assert_eq!(err.kind, ConstraintKind::MaxLength);
}

#[test]
fn max35text_unicode_counted_by_chars() {
    // Each emoji is 1 char but multiple bytes. 35 emoji = 35 chars = OK
    let emoji_35 = "\u{1F600}".repeat(35);
    assert!(Max35Text::new(&emoji_35).is_ok());

    let emoji_36 = "\u{1F600}".repeat(36);
    assert!(Max35Text::new(&emoji_36).is_err());
}

// ---------------------------------------------------------------------------
// UUIDv4Identifier — Pattern: [a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}
// ---------------------------------------------------------------------------

#[test]
fn uuid_v4_valid() {
    assert!(UUIDv4Identifier::new("550e8400-e29b-41d3-a456-426614174000").is_ok());
    assert!(UUIDv4Identifier::new("12345678-abcd-4ef0-8000-000000000000").is_ok());
}

#[test]
fn uuid_v4_all_variant_bits() {
    // Byte 19 must be one of: 8, 9, a, b
    assert!(UUIDv4Identifier::new("00000000-0000-4000-8000-000000000000").is_ok());
    assert!(UUIDv4Identifier::new("00000000-0000-4000-9000-000000000000").is_ok());
    assert!(UUIDv4Identifier::new("00000000-0000-4000-a000-000000000000").is_ok());
    assert!(UUIDv4Identifier::new("00000000-0000-4000-b000-000000000000").is_ok());
}

#[test]
fn uuid_v4_wrong_variant_bit() {
    // 'c' is not in [89ab]
    let err = UUIDv4Identifier::new("00000000-0000-4000-c000-000000000000").unwrap_err();
    assert_eq!(err.kind, ConstraintKind::Pattern);
}

#[test]
fn uuid_v4_wrong_version() {
    // Version digit must be '4', not '5'
    let err = UUIDv4Identifier::new("550e8400-e29b-51d3-a456-426614174000").unwrap_err();
    assert_eq!(err.kind, ConstraintKind::Pattern);
}

#[test]
fn uuid_v4_uppercase_rejected() {
    // Pattern requires lowercase hex
    assert!(UUIDv4Identifier::new("550E8400-E29B-41D3-A456-426614174000").is_err());
}

#[test]
fn uuid_v4_too_short() {
    assert!(UUIDv4Identifier::new("550e8400-e29b-41d3").is_err());
    assert!(UUIDv4Identifier::new("").is_err());
}

#[test]
fn uuid_v4_no_dashes() {
    assert!(UUIDv4Identifier::new("550e8400e29b41d3a456426614174000").is_err());
}

// ---------------------------------------------------------------------------
// Cross-cutting: TryFrom<String> and From<T> for String round-trip
// ---------------------------------------------------------------------------

#[test]
fn round_trip_via_string() {
    let code = ActiveCurrencyCode::new("USD").unwrap();
    let s: String = code.into();
    assert_eq!(s, "USD");
}

#[test]
fn constraint_error_is_std_error() {
    let err = ActiveCurrencyCode::new("").unwrap_err();
    // ConstraintError implements std::error::Error
    let _: &dyn std::error::Error = &err;
    assert!(!err.to_string().is_empty());
}
