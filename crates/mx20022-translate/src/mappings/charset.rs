//! SWIFT FIN character-set conversion utilities.
//!
//! The SWIFT FIN character set is a restricted subset of ASCII.  ISO 20022
//! messages use UTF-8 and may contain characters that cannot be transmitted
//! in a SWIFT FIN message without replacement.

/// The SWIFT FIN character set: letters, digits and the punctuation symbols
/// that are explicitly permitted in MT message text blocks.
///
/// Allowed: `A-Z`, `a-z`, `0-9`, `/ - ? : ( ) . , ' + { } CR LF Space`
///
/// # Examples
///
/// ```rust
/// use mx20022_translate::mappings::charset::is_swift_safe;
/// assert!(is_swift_safe('A'));
/// assert!(is_swift_safe('/'));
/// assert!(!is_swift_safe('€'));
/// ```
#[must_use]
pub fn is_swift_safe(c: char) -> bool {
    matches!(
        c,
        'A'..='Z'
            | 'a'..='z'
            | '0'..='9'
            | ' '
            | '/'
            | '-'
            | '?'
            | ':'
            | '('
            | ')'
            | '.'
            | ','
            | '\''
            | '+'
            | '{'
            | '}'
            | '\r'
            | '\n'
    )
}

/// Convert a UTF-8 string to the SWIFT FIN character set.
///
/// Characters outside the SWIFT charset are replaced with a close ASCII
/// approximation when one exists, or with a space otherwise.
///
/// Returns `(converted_string, had_replacements)`.  The boolean is `true`
/// when at least one character was replaced.
///
/// # Examples
///
/// ```rust
/// use mx20022_translate::mappings::charset::to_swift_charset;
/// let (s, replaced) = to_swift_charset("Müller");
/// assert_eq!(s, "Muller");
/// assert!(replaced);
/// ```
pub fn to_swift_charset(s: &str) -> (String, bool) {
    let mut out = String::with_capacity(s.len());
    let mut had_replacements = false;

    for c in s.chars() {
        if is_swift_safe(c) {
            out.push(c);
        } else {
            had_replacements = true;
            let replacement = approximate(c);
            out.push_str(replacement);
        }
    }

    (out, had_replacements)
}

/// Return an ASCII approximation for non-SWIFT characters.
///
/// Covers the most common European diacritics and Unicode punctuation.
fn approximate(c: char) -> &'static str {
    match c {
        // Latin extended — uppercase
        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => "A",
        'Æ' => "AE",
        'Ç' => "C",
        'È' | 'É' | 'Ê' | 'Ë' => "E",
        'Ì' | 'Í' | 'Î' | 'Ï' => "I",
        'Ð' => "D",
        'Ñ' => "N",
        'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => "O",
        'Ù' | 'Ú' | 'Û' | 'Ü' => "U",
        'Ý' => "Y",
        'Þ' => "TH",
        'ß' => "ss",
        // Latin extended — lowercase
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => "a",
        'æ' => "ae",
        'ç' => "c",
        'è' | 'é' | 'ê' | 'ë' => "e",
        'ì' | 'í' | 'î' | 'ï' => "i",
        'ð' => "d",
        'ñ' => "n",
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' => "o",
        'ù' | 'ú' | 'û' | 'ü' => "u",
        'ý' | 'ÿ' => "y",
        'þ' => "th",
        // Currency and common symbols
        '€' => "EUR",
        '£' => "GBP",
        '¥' => "JPY",
        // Typographic punctuation — replace with safe equivalents
        // Single/double quotation marks → apostrophe
        '\u{2018}' | '\u{2019}' | '\u{201C}' | '\u{201D}' => "'",
        '\u{2013}' | '\u{2014}' => "-", // en-dash / em-dash
        '\u{2026}' => "...",            // horizontal ellipsis
        // Copyright/trademark symbols, tabs and everything else → space
        _ => " ",
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_letters_are_safe() {
        for c in 'A'..='Z' {
            assert!(is_swift_safe(c));
        }
        for c in 'a'..='z' {
            assert!(is_swift_safe(c));
        }
    }

    #[test]
    fn test_digits_are_safe() {
        for c in '0'..='9' {
            assert!(is_swift_safe(c));
        }
    }

    #[test]
    fn test_swift_punctuation_safe() {
        for c in [
            ' ', '/', '-', '?', ':', '(', ')', '.', ',', '\'', '+', '{', '}',
        ] {
            assert!(is_swift_safe(c), "expected '{c}' to be SWIFT-safe");
        }
    }

    #[test]
    fn test_non_swift_chars_not_safe() {
        assert!(!is_swift_safe('€'));
        assert!(!is_swift_safe('ü'));
        assert!(!is_swift_safe('ñ'));
    }

    #[test]
    fn test_pure_ascii_no_replacement() {
        let (s, replaced) = to_swift_charset("HELLO WORLD 123");
        assert_eq!(s, "HELLO WORLD 123");
        assert!(!replaced);
    }

    #[test]
    fn test_umlaut_replacement() {
        let (s, replaced) = to_swift_charset("Müller");
        assert_eq!(s, "Muller");
        assert!(replaced);
    }

    #[test]
    fn test_euro_sign_replacement() {
        let (s, replaced) = to_swift_charset("100€");
        assert_eq!(s, "100EUR");
        assert!(replaced);
    }

    #[test]
    fn test_empty_string() {
        let (s, replaced) = to_swift_charset("");
        assert_eq!(s, "");
        assert!(!replaced);
    }
}
