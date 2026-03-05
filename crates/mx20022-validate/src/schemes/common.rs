//! Shared helpers for scheme validators.
//!
//! This module consolidates utility functions used across multiple scheme
//! validators (`FedNow`, SEPA, `CBPR+`), reducing code duplication and ensuring
//! consistent behaviour for common operations like amount parsing and
//! parent-block extraction.

use super::xml_scan::{extract_element, has_element};
use crate::error::{Severity, ValidationError};

/// Extract the content between `<parent_tag>...</parent_tag>`.
///
/// Returns `None` and pushes an error if:
/// - `require_parent` is `true` and the parent tag is missing.
/// - The parent tag exists but is unclosed (malformed XML).
///
/// Returns `None` silently if the parent tag is absent and `require_parent`
/// is `false`.
pub(crate) fn extract_parent_block<'a>(
    xml: &'a str,
    parent_tag: &str,
    path_prefix: &str,
    rule_id: &str,
    errors: &mut Vec<ValidationError>,
    require_parent: bool,
) -> Option<&'a str> {
    let open = format!("<{parent_tag}>");
    let close = format!("</{parent_tag}>");
    let Some(start) = xml.find(&open) else {
        if require_parent {
            errors.push(ValidationError::new(
                path_prefix,
                Severity::Error,
                rule_id,
                format!("{parent_tag} element is missing"),
            ));
        }
        return None;
    };
    let after = start + open.len();
    let Some(end) = xml[after..].find(&close) else {
        errors.push(ValidationError::new(
            path_prefix,
            Severity::Error,
            rule_id,
            format!("{parent_tag} element is unclosed; XML structure invalid"),
        ));
        return None;
    };
    Some(&xml[after..after + end])
}

/// Check that a `<Nm>` element exists inside a parent block and optionally
/// enforce a maximum length.
///
/// If `max_len` is `Some(n)`, the name length is checked against `n`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn check_name_in_parent(
    xml: &str,
    parent_tag: &str,
    max_len: Option<usize>,
    path_prefix: &str,
    rule_id: &str,
    scheme_name: &str,
    errors: &mut Vec<ValidationError>,
    require_name: bool,
) {
    let Some(block) = extract_parent_block(
        xml,
        parent_tag,
        path_prefix,
        rule_id,
        errors,
        true, // parent is always required when checking names
    ) else {
        return;
    };

    match extract_element(block, "Nm") {
        None if require_name => {
            errors.push(ValidationError::new(
                format!("{path_prefix}/Nm"),
                Severity::Error,
                rule_id,
                format!("{parent_tag}/Nm is required for {scheme_name}"),
            ));
        }
        Some(nm) if max_len.is_some_and(|max| nm.len() > max) => {
            let max = max_len.unwrap();
            errors.push(ValidationError::new(
                format!("{path_prefix}/Nm"),
                Severity::Error,
                rule_id,
                format!(
                    "{parent_tag}/Nm must be at most {max} characters; got {} characters",
                    nm.len()
                ),
            ));
        }
        _ => {}
    }
}

/// Check that a `BICFI` element exists inside a parent block.
pub(crate) fn check_bic_in_parent(
    xml: &str,
    parent_tag: &str,
    path: &str,
    rule_id: &str,
    scheme_name: &str,
    errors: &mut Vec<ValidationError>,
) {
    let Some(block) = extract_parent_block(xml, parent_tag, path, rule_id, errors, true) else {
        return;
    };
    if !has_element(block, "BICFI") {
        errors.push(ValidationError::new(
            path,
            Severity::Error,
            rule_id,
            format!("{parent_tag}/FinInstnId/BICFI is required for {scheme_name}"),
        ));
    }
}

/// Parse a decimal amount string like `"1000.50"` into integer cents (`100050`).
///
/// # Contract
///
/// The input **must** contain exactly one `.` followed by exactly **2** decimal
/// digits (e.g. `"100.50"`, `"0.01"`). This matches the format enforced by the
/// XSD `ActiveCurrencyAndAmount` type that all callers validate against before
/// reaching this function.
///
/// Returns `None` for non-conforming input:
/// - No decimal point (e.g. `"100"`)
/// - Integer or fractional part fails `u64` parsing (e.g. `"abc.50"`, `"100.ab"`)
///
/// A `debug_assert!` fires in test/dev builds if the fractional part is not
/// exactly 2 digits, catching misuse early without penalising release builds.
pub(crate) fn parse_amount_cents(s: &str) -> Option<u64> {
    let dot = s.find('.')?;
    let integer: u64 = s[..dot].parse().ok()?;
    let frac_str = &s[dot + 1..];
    debug_assert!(
        frac_str.len() == 2,
        "parse_amount_cents expects exactly 2 decimal digits, got {frac_str:?}"
    );
    let frac: u64 = frac_str.parse().ok()?;
    Some(integer * 100 + frac)
}

/// Like [`parse_amount_cents`], but accepts 0–2 decimal digits.
///
/// - `"1000"` → `Some(100_000)`
/// - `"1000.5"` → `Some(100_050)`
/// - `"1000.50"` → `Some(100_050)`
/// - `"1000.500"` → `None` (>2 decimal digits)
/// - `"abc"` → `None`
pub(crate) fn parse_amount_cents_lenient(s: &str) -> Option<u64> {
    if let Some(dot) = s.find('.') {
        let integer: u64 = s[..dot].parse().ok()?;
        let frac_str = &s[dot + 1..];
        let frac: u64 = match frac_str.len() {
            0 => 0,
            1 => frac_str.parse::<u64>().ok()? * 10,
            2 => frac_str.parse().ok()?,
            _ => return None,
        };
        Some(integer * 100 + frac)
    } else {
        let integer: u64 = s.parse().ok()?;
        Some(integer * 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_amount_cents_normal() {
        assert_eq!(parse_amount_cents("100.50"), Some(10050));
    }

    #[test]
    fn parse_amount_cents_minimum() {
        assert_eq!(parse_amount_cents("0.01"), Some(1));
    }

    #[test]
    fn parse_amount_cents_large() {
        assert_eq!(parse_amount_cents("999999.99"), Some(99999999));
    }

    #[test]
    fn parse_amount_cents_no_dot() {
        assert_eq!(parse_amount_cents("100"), None);
    }

    #[test]
    fn parse_amount_cents_bad_integer() {
        assert_eq!(parse_amount_cents("abc.50"), None);
    }

    #[test]
    fn parse_amount_cents_bad_fraction() {
        assert_eq!(parse_amount_cents("100.ab"), None);
    }

    #[test]
    fn lenient_no_dot() {
        assert_eq!(parse_amount_cents_lenient("1000"), Some(100_000));
    }

    #[test]
    fn lenient_one_decimal() {
        assert_eq!(parse_amount_cents_lenient("1000.5"), Some(100_050));
    }

    #[test]
    fn lenient_two_decimals() {
        assert_eq!(parse_amount_cents_lenient("1000.50"), Some(100_050));
    }

    #[test]
    fn lenient_three_decimals() {
        assert_eq!(parse_amount_cents_lenient("1000.500"), None);
    }

    #[test]
    fn extract_parent_block_missing_optional() {
        let mut errors = Vec::new();
        let result = extract_parent_block("<xml/>", "Dbtr", "/path", "RULE", &mut errors, false);
        assert!(result.is_none());
        assert!(errors.is_empty(), "should not push error when not required");
    }

    #[test]
    fn extract_parent_block_missing_required() {
        let mut errors = Vec::new();
        let result = extract_parent_block("<xml/>", "Dbtr", "/path", "RULE", &mut errors, true);
        assert!(result.is_none());
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn extract_parent_block_unclosed() {
        let mut errors = Vec::new();
        let result = extract_parent_block(
            "<Dbtr><Nm>Alice</Nm>",
            "Dbtr",
            "/path",
            "RULE",
            &mut errors,
            false,
        );
        assert!(result.is_none());
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("unclosed"));
    }
}
