//! ISO 8601 date and datetime validation rules for ISO 20022 messages.
//!
//! ISO 20022 uses two date-like types:
//! - `ISODateTime`: `YYYY-MM-DDThh:mm:ss[.sss][Z|+hh:mm|-hh:mm]`
//! - `ISODate`:     `YYYY-MM-DD`
//!
//! Structural validation is performed without an external calendar library:
//! ranges are enforced for each field, but calendar semantics (e.g. Feb 30)
//! are intentionally not checked, as ISO 20022 schemas also don't prohibit
//! them at the lexical level.

use crate::error::{Severity, ValidationError};
use crate::rules::Rule;

// ---------------------------------------------------------------------------
// IsoDateTimeRule
// ---------------------------------------------------------------------------

/// Validates a value as an ISO 8601 datetime string in the format used by
/// ISO 20022: `YYYY-MM-DDThh:mm:ss[.sss][Z|+hh:mm|-hh:mm]`.
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::datetime::IsoDateTimeRule;
/// use mx20022_validate::rules::Rule;
///
/// let rule = IsoDateTimeRule;
///
/// let errors = rule.validate("2024-01-01T12:00:00Z", "/path");
/// assert!(errors.is_empty(), "Valid datetime should produce no errors");
///
/// let errors = rule.validate("2024-13-01T00:00:00Z", "/path");
/// assert!(!errors.is_empty(), "Month 13 should be rejected");
///
/// let errors = rule.validate("not-a-date", "/path");
/// assert!(!errors.is_empty(), "Non-date string should be rejected");
/// ```
pub struct IsoDateTimeRule;

impl Rule for IsoDateTimeRule {
    fn id(&self) -> &'static str {
        "DATETIME_CHECK"
    }

    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
        match validate_iso_datetime(value) {
            Ok(()) => vec![],
            Err(msg) => {
                vec![ValidationError::new(
                    path,
                    Severity::Error,
                    "DATETIME_CHECK",
                    msg,
                )]
            }
        }
    }
}

// ---------------------------------------------------------------------------
// IsoDateRule
// ---------------------------------------------------------------------------

/// Validates a value as an ISO 8601 date string in the format used by
/// ISO 20022: `YYYY-MM-DD`.
///
/// # Examples
///
/// ```
/// use mx20022_validate::rules::datetime::IsoDateRule;
/// use mx20022_validate::rules::Rule;
///
/// let rule = IsoDateRule;
///
/// let errors = rule.validate("2024-01-15", "/path");
/// assert!(errors.is_empty(), "Valid date should produce no errors");
///
/// let errors = rule.validate("2024-00-15", "/path");
/// assert!(!errors.is_empty(), "Month 00 should be rejected");
/// ```
pub struct IsoDateRule;

impl Rule for IsoDateRule {
    fn id(&self) -> &'static str {
        "DATE_CHECK"
    }

    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError> {
        match validate_iso_date(value) {
            Ok(()) => vec![],
            Err(msg) => {
                vec![ValidationError::new(
                    path,
                    Severity::Error,
                    "DATE_CHECK",
                    msg,
                )]
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

/// Parse a 2-digit decimal field from a byte slice at `offset` and validate
/// that it falls in `[min, max]`.  Returns `Err` with a descriptive message
/// on failure.
fn parse_two_digits(s: &[u8], offset: usize, field: &str, min: u8, max: u8) -> Result<u8, String> {
    let a = s[offset];
    let b = s[offset + 1];
    if !a.is_ascii_digit() || !b.is_ascii_digit() {
        return Err(format!("{field} must be two decimal digits"));
    }
    let value = (a - b'0') * 10 + (b - b'0');
    if value < min || value > max {
        return Err(format!(
            "{field} must be in range [{min}, {max}], got {value}"
        ));
    }
    Ok(value)
}

/// Parse a 4-digit year and validate range [1900, 2099].
fn parse_year(s: &[u8], offset: usize) -> Result<u16, String> {
    let digits: Vec<u8> = s[offset..offset + 4].to_vec();
    for d in &digits {
        if !d.is_ascii_digit() {
            return Err("Year must be four decimal digits".to_owned());
        }
    }
    let year: u16 = u16::from(digits[0] - b'0') * 1000
        + u16::from(digits[1] - b'0') * 100
        + u16::from(digits[2] - b'0') * 10
        + u16::from(digits[3] - b'0');
    if !(1900..=2099).contains(&year) {
        return Err(format!("Year must be in range [1900, 2099], got {year}"));
    }
    Ok(year)
}

/// Validate the date portion `YYYY-MM-DD` given as a byte slice at offset 0.
/// The slice must be exactly 10 bytes.
fn validate_date_part(bytes: &[u8], original: &str) -> Result<(), String> {
    if bytes.len() < 10 {
        return Err(format!("Value is too short to be a date: `{original}`"));
    }
    parse_year(bytes, 0)?;
    if bytes[4] != b'-' {
        return Err(format!(
            "Expected '-' after year in `{original}`, got `{}`",
            bytes[4] as char
        ));
    }
    parse_two_digits(bytes, 5, "Month", 1, 12)?;
    if bytes[7] != b'-' {
        return Err(format!(
            "Expected '-' after month in `{original}`, got `{}`",
            bytes[7] as char
        ));
    }
    parse_two_digits(bytes, 8, "Day", 1, 31)?;
    Ok(())
}

/// Validate an ISO 20022 date string `YYYY-MM-DD`.
fn validate_iso_date(value: &str) -> Result<(), String> {
    let bytes = value.as_bytes();
    if bytes.len() != 10 {
        return Err(format!(
            "Date must be exactly 10 characters (YYYY-MM-DD), got {}: `{value}`",
            bytes.len()
        ));
    }
    validate_date_part(bytes, value)
}

/// Validate an ISO 20022 datetime string `YYYY-MM-DDThh:mm:ss[.sss][Z|+hh:mm|-hh:mm]`.
fn validate_iso_datetime(value: &str) -> Result<(), String> {
    let bytes = value.as_bytes();

    // Minimum: YYYY-MM-DDThh:mm:ssZ = 20 chars
    if bytes.len() < 20 {
        return Err(format!(
            "Datetime is too short (minimum 20 characters), got {}: `{value}`",
            bytes.len()
        ));
    }

    // Validate date portion (bytes 0..10)
    validate_date_part(bytes, value)?;

    // 'T' separator at position 10
    if bytes[10] != b'T' {
        return Err(format!(
            "Expected 'T' date/time separator at position 11 in `{value}`, got `{}`",
            bytes[10] as char
        ));
    }

    // Time: hh:mm:ss starting at offset 11
    parse_two_digits(bytes, 11, "Hour", 0, 23)?;
    if bytes[13] != b':' {
        return Err(format!(
            "Expected ':' after hour in `{value}`, got `{}`",
            bytes[13] as char
        ));
    }
    parse_two_digits(bytes, 14, "Minute", 0, 59)?;
    if bytes[16] != b':' {
        return Err(format!(
            "Expected ':' after minute in `{value}`, got `{}`",
            bytes[16] as char
        ));
    }
    parse_two_digits(bytes, 17, "Second", 0, 59)?;

    // Optional fractional seconds and timezone at offset 19
    let mut pos = 19;

    // Optional fractional seconds: .sss (1-9 digits)
    if pos < bytes.len() && bytes[pos] == b'.' {
        pos += 1;
        let frac_start = pos;
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
        if pos == frac_start {
            return Err(format!("Expected fractional digits after '.' in `{value}`"));
        }
    }

    // Timezone: Z, +hh:mm, or -hh:mm
    if pos >= bytes.len() {
        return Err(format!(
            "Missing timezone designator (Z, +hh:mm, or -hh:mm) in `{value}`"
        ));
    }

    match bytes[pos] {
        b'Z' => {
            pos += 1;
        }
        b'+' | b'-' => {
            // Need at least 6 more characters: hh:mm
            if bytes.len() < pos + 6 {
                return Err(format!("Timezone offset is truncated in `{value}`"));
            }
            pos += 1;
            parse_two_digits(bytes, pos, "Timezone hour", 0, 23)?;
            pos += 2;
            if bytes[pos] != b':' {
                return Err(format!(
                    "Expected ':' in timezone offset in `{value}`, got `{}`",
                    bytes[pos] as char
                ));
            }
            pos += 1;
            parse_two_digits(bytes, pos, "Timezone minute", 0, 59)?;
            pos += 2;
        }
        other => {
            return Err(format!(
                "Invalid timezone designator `{}` in `{value}`; expected Z, +, or -",
                other as char
            ));
        }
    }

    // No trailing garbage
    if pos != bytes.len() {
        return Err(format!(
            "Unexpected trailing characters in datetime `{value}`"
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Rule;

    // ----- IsoDateTimeRule -------------------------------------------------

    const VALID_DATETIMES: &[&str] = &[
        "2024-01-01T12:00:00Z",
        "2024-01-15T23:59:59Z",
        "1900-01-01T00:00:00Z",
        "2099-12-31T23:59:59Z",
        "2024-06-15T08:30:00+05:30",
        "2024-06-15T08:30:00-07:00",
        "2024-01-01T12:00:00.000Z",
        "2024-01-01T12:00:00.123Z",
        "2024-01-01T12:00:00.123456789Z",
        "2024-01-01T00:00:00+00:00",
    ];

    const INVALID_DATETIMES: &[&str] = &[
        "2024-13-01T00:00:00Z",      // month 13
        "2024-00-01T00:00:00Z",      // month 0
        "2024-01-32T00:00:00Z",      // day 32
        "2024-01-00T00:00:00Z",      // day 0
        "2024-01-01T24:00:00Z",      // hour 24
        "2024-01-01T00:60:00Z",      // minute 60
        "2024-01-01T00:00:60Z",      // second 60
        "1899-12-31T00:00:00Z",      // year out of range
        "2100-01-01T00:00:00Z",      // year out of range
        "2024-01-01T00:00:00",       // missing timezone
        "2024-01-01 00:00:00Z",      // space instead of T
        "not-a-date",                // garbage
        "",                          // empty
        "2024-01-01T12:00:00.Z",     // trailing dot before Z
        "2024-01-01T12:00:00+25:00", // tz hour 25
    ];

    #[test]
    fn valid_datetimes_pass() {
        let rule = IsoDateTimeRule;
        for dt in VALID_DATETIMES {
            let errors = rule.validate(dt, "/test");
            assert!(
                errors.is_empty(),
                "Expected no errors for `{dt}`, got: {errors:?}"
            );
        }
    }

    #[test]
    fn invalid_datetimes_fail() {
        let rule = IsoDateTimeRule;
        for dt in INVALID_DATETIMES {
            let errors = rule.validate(dt, "/test");
            assert!(!errors.is_empty(), "Expected errors for `{dt}`");
        }
    }

    #[test]
    fn datetime_error_has_correct_rule_id() {
        let rule = IsoDateTimeRule;
        let errors = rule.validate("not-a-date", "/Document/CreDtTm");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "DATETIME_CHECK");
        assert_eq!(errors[0].path, "/Document/CreDtTm");
        assert_eq!(errors[0].severity, Severity::Error);
    }

    #[test]
    fn datetime_rule_id_is_datetime_check() {
        assert_eq!(IsoDateTimeRule.id(), "DATETIME_CHECK");
    }

    // ----- IsoDateRule -----------------------------------------------------

    const VALID_DATES: &[&str] = &[
        "2024-01-01",
        "2024-12-31",
        "1900-01-01",
        "2099-12-31",
        "2024-02-29", // structural only — no calendar check
        "2024-06-15",
    ];

    const INVALID_DATES: &[&str] = &[
        "2024-13-01",  // month 13
        "2024-00-01",  // month 0
        "2024-01-32",  // day 32
        "2024-01-00",  // day 0
        "1899-12-31",  // year out of range
        "2100-01-01",  // year out of range
        "2024/01/01",  // wrong separator
        "24-01-01",    // 2-digit year
        "2024-1-1",    // no zero-padding
        "not-a-date",  // garbage
        "",            // empty
        "2024-01-01T", // has time component
        "20240101",    // no separators
    ];

    #[test]
    fn valid_dates_pass() {
        let rule = IsoDateRule;
        for d in VALID_DATES {
            let errors = rule.validate(d, "/test");
            assert!(
                errors.is_empty(),
                "Expected no errors for `{d}`, got: {errors:?}"
            );
        }
    }

    #[test]
    fn invalid_dates_fail() {
        let rule = IsoDateRule;
        for d in INVALID_DATES {
            let errors = rule.validate(d, "/test");
            assert!(!errors.is_empty(), "Expected errors for `{d}`");
        }
    }

    #[test]
    fn date_error_has_correct_rule_id() {
        let rule = IsoDateRule;
        let errors = rule.validate("not-a-date", "/Document/IntrBkSttlmDt");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "DATE_CHECK");
        assert_eq!(errors[0].path, "/Document/IntrBkSttlmDt");
        assert_eq!(errors[0].severity, Severity::Error);
    }

    #[test]
    fn date_rule_id_is_date_check() {
        assert_eq!(IsoDateRule.id(), "DATE_CHECK");
    }
}
