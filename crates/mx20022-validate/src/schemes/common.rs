//! Amount parsers shared by typed scheme validators.

/// Parse a decimal amount string like `"1000.50"` into integer cents (`100050`).
///
/// # Contract
///
/// The input **must** contain exactly one `.` followed by exactly **2** decimal
/// digits (e.g. `"100.50"`, `"0.01"`). Scheme validation deliberately does
/// not run generated XSD constraints, so this parser rejects invalid lengths
/// and arithmetic overflow itself.
///
/// Returns `None` for non-conforming input:
/// - No decimal point (e.g. `"100"`)
/// - Integer or fractional part fails `u64` parsing (e.g. `"abc.50"`, `"100.ab"`)
///
pub(crate) fn parse_amount_cents(s: &str) -> Option<u64> {
    let dot = s.find('.')?;
    let integer: u64 = s[..dot].parse().ok()?;
    let frac_str = &s[dot + 1..];
    if frac_str.len() != 2 {
        return None;
    }
    let frac: u64 = frac_str.parse().ok()?;
    integer.checked_mul(100)?.checked_add(frac)
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
        integer.checked_mul(100)?.checked_add(frac)
    } else {
        let integer: u64 = s.parse().ok()?;
        integer.checked_mul(100)
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
    fn parse_amount_cents_rejects_wrong_fraction_length_and_overflow() {
        assert_eq!(parse_amount_cents("100.5"), None);
        assert_eq!(parse_amount_cents("100.500"), None);
        assert_eq!(parse_amount_cents("184467440737095517.00"), None);
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
    fn lenient_rejects_overflow() {
        assert_eq!(parse_amount_cents_lenient("184467440737095517"), None);
    }
}
