/// Compute `numeric_string mod 97` using chunked arithmetic (handles arbitrary length).
pub(crate) fn mod97(numeric: &str) -> u64 {
    let mut remainder: u64 = 0;
    for ch in numeric.chars() {
        let digit = ch as u64 - '0' as u64;
        remainder = (remainder * 10 + digit) % 97;
    }
    remainder
}

/// Expand an alphanumeric string for MOD 97-10: digits stay, letters become A=10…Z=35.
///
/// Used by both IBAN and LEI check-digit verification.
pub(crate) fn alpha_to_numeric(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if c.is_ascii_digit() {
            out.push(c);
        } else {
            let n = (c as u32) - ('A' as u32) + 10;
            out.push_str(&n.to_string());
        }
    }
    out
}
