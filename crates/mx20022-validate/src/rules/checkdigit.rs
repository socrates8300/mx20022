/// Compute `numeric_string mod 97` using chunked arithmetic (handles arbitrary length).
pub(crate) fn mod97(numeric: &str) -> u64 {
    let mut remainder: u64 = 0;
    for ch in numeric.chars() {
        let digit = ch as u64 - '0' as u64;
        remainder = (remainder * 10 + digit) % 97;
    }
    remainder
}
