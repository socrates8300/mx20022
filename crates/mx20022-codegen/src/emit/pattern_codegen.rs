//! XSD pattern → Rust inline validation code generator.
//!
//! Compiles XSD `xs:pattern` facets into `TokenStream` boolean expressions that
//! evaluate to `true` when a `value: &str` does **not** match the pattern
//! (i.e., `true` = violation).
//!
//! This avoids a runtime `regex` dependency on the model crate by emitting
//! byte-level checks at codegen time.

use proc_macro2::TokenStream;
use quote::quote;

// ── Public API ───────────────────────────────────────────────────────────────

/// Compile an XSD pattern string into a violation-check `TokenStream`.
///
/// Returns `Some(ts)` where `ts` is a boolean expression operating on
/// `value: &str` that is `true` when the value **violates** the pattern.
///
/// Returns `None` if the pattern cannot be compiled (unsupported syntax).
pub fn emit_pattern_check(pattern: &str) -> Option<TokenStream> {
    let segments = parse_pattern(pattern)?;
    Some(emit_segments(&segments))
}

// ── Internal AST ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Segment {
    CharClass {
        ranges: Vec<CharRange>,
        quantifier: Quantifier,
    },
    Literal(u8),
    Group {
        segments: Vec<Segment>,
        quantifier: Quantifier,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum CharRange {
    Range(u8, u8),
    Single(u8),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Quantifier {
    Exact(usize),
    Range(usize, usize),
    OneOrMore,
}

// ── Parser ───────────────────────────────────────────────────────────────────

fn parse_pattern(pattern: &str) -> Option<Vec<Segment>> {
    let bytes = pattern.as_bytes();
    let mut pos = 0;
    let mut segments = Vec::new();

    while pos < bytes.len() {
        match bytes[pos] {
            b'[' => {
                let (ranges, end) = parse_char_class(bytes, pos)?;
                pos = end;
                let quantifier = parse_quantifier(bytes, &mut pos);
                segments.push(Segment::CharClass { ranges, quantifier });
            }
            b'(' => {
                let (inner, end) = parse_group(bytes, pos)?;
                pos = end;
                let quantifier = parse_quantifier(bytes, &mut pos);
                segments.push(Segment::Group {
                    segments: inner,
                    quantifier,
                });
            }
            b'\\' => {
                pos += 1;
                if pos >= bytes.len() {
                    return None;
                }
                let ch = bytes[pos];
                pos += 1;
                let quantifier = parse_quantifier(bytes, &mut pos);
                match quantifier {
                    Quantifier::Exact(1) => segments.push(Segment::Literal(ch)),
                    _ => {
                        segments.push(Segment::CharClass {
                            ranges: vec![CharRange::Single(ch)],
                            quantifier,
                        });
                    }
                }
            }
            ch => {
                pos += 1;
                let quantifier = parse_quantifier(bytes, &mut pos);
                match quantifier {
                    Quantifier::Exact(1) => segments.push(Segment::Literal(ch)),
                    _ => {
                        segments.push(Segment::CharClass {
                            ranges: vec![CharRange::Single(ch)],
                            quantifier,
                        });
                    }
                }
            }
        }
    }

    Some(segments)
}

fn parse_char_class(bytes: &[u8], start: usize) -> Option<(Vec<CharRange>, usize)> {
    debug_assert_eq!(bytes[start], b'[');
    let mut pos = start + 1;
    let mut ranges = Vec::new();

    while pos < bytes.len() && bytes[pos] != b']' {
        if bytes[pos] == b'\\' {
            pos += 1;
            if pos >= bytes.len() {
                return None;
            }
            let ch = bytes[pos];
            pos += 1;
            if pos < bytes.len()
                && bytes[pos] == b'-'
                && pos + 1 < bytes.len()
                && bytes[pos + 1] != b']'
            {
                pos += 1; // skip '-'
                let end_ch = if bytes[pos] == b'\\' {
                    pos += 1;
                    if pos >= bytes.len() {
                        return None;
                    }
                    let c = bytes[pos];
                    pos += 1;
                    c
                } else {
                    let c = bytes[pos];
                    pos += 1;
                    c
                };
                ranges.push(CharRange::Range(ch, end_ch));
            } else {
                ranges.push(CharRange::Single(ch));
            }
        } else {
            let ch = bytes[pos];
            pos += 1;
            if pos < bytes.len()
                && bytes[pos] == b'-'
                && pos + 1 < bytes.len()
                && bytes[pos + 1] != b']'
            {
                pos += 1; // skip '-'
                let end_ch = if bytes[pos] == b'\\' {
                    pos += 1;
                    if pos >= bytes.len() {
                        return None;
                    }
                    let c = bytes[pos];
                    pos += 1;
                    c
                } else {
                    let c = bytes[pos];
                    pos += 1;
                    c
                };
                ranges.push(CharRange::Range(ch, end_ch));
            } else {
                ranges.push(CharRange::Single(ch));
            }
        }
    }

    if pos >= bytes.len() {
        return None; // unclosed bracket
    }
    pos += 1; // skip ']'
    Some((ranges, pos))
}

fn parse_group(bytes: &[u8], start: usize) -> Option<(Vec<Segment>, usize)> {
    debug_assert_eq!(bytes[start], b'(');
    let mut depth = 1;
    let mut pos = start + 1;

    while pos < bytes.len() && depth > 0 {
        match bytes[pos] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b'\\' => pos += 1, // skip escaped char
            _ => {}
        }
        if depth > 0 {
            pos += 1;
        }
    }

    if depth != 0 {
        return None;
    }

    let inner_bytes = &bytes[start + 1..pos];
    // Parse the inner content as a pattern
    let inner_str = std::str::from_utf8(inner_bytes).ok()?;
    let inner_segments = parse_pattern(inner_str)?;
    pos += 1; // skip closing ')'
    Some((inner_segments, pos))
}

fn parse_quantifier(bytes: &[u8], pos: &mut usize) -> Quantifier {
    if *pos >= bytes.len() {
        return Quantifier::Exact(1);
    }

    match bytes[*pos] {
        b'{' => {
            let start = *pos + 1;
            let mut end = start;
            while end < bytes.len() && bytes[end] != b'}' {
                end += 1;
            }
            if end >= bytes.len() {
                return Quantifier::Exact(1);
            }
            let content = std::str::from_utf8(&bytes[start..end]).unwrap_or("");
            *pos = end + 1;

            if let Some(comma_pos) = content.find(',') {
                let min_str = &content[..comma_pos];
                let max_str = &content[comma_pos + 1..];
                let min = min_str.parse::<usize>().unwrap_or(0);
                let max = max_str.parse::<usize>().unwrap_or(min);
                if min == max {
                    Quantifier::Exact(min)
                } else {
                    Quantifier::Range(min, max)
                }
            } else {
                let n = content.parse::<usize>().unwrap_or(1);
                Quantifier::Exact(n)
            }
        }
        b'+' => {
            *pos += 1;
            Quantifier::OneOrMore
        }
        b'*' => {
            *pos += 1;
            Quantifier::Range(0, usize::MAX)
        }
        b'?' => {
            *pos += 1;
            Quantifier::Range(0, 1)
        }
        _ => Quantifier::Exact(1),
    }
}

// ── Code emission ────────────────────────────────────────────────────────────

/// Emit a boolean expression that is `true` when `value` violates the pattern.
fn emit_segments(segments: &[Segment]) -> TokenStream {
    // Compute length constraints from the segments.
    let (min_len, max_len) = compute_length_bounds(segments);

    // If min == max (fixed length), emit a simple length check + per-byte validation.
    // Otherwise emit a cursor-based validator.
    if min_len == max_len {
        emit_fixed_length(segments, min_len)
    } else {
        emit_variable_length(segments, min_len, max_len)
    }
}

fn compute_length_bounds(segments: &[Segment]) -> (usize, usize) {
    let mut min_total = 0usize;
    let mut max_total = 0usize;

    for seg in segments {
        let (seg_min, seg_max) = match seg {
            Segment::Literal(_) => (1, 1),
            Segment::CharClass { quantifier, .. } => quantifier_bounds(*quantifier),
            Segment::Group {
                segments: inner,
                quantifier,
            } => {
                let (inner_min, inner_max) = compute_length_bounds(inner);
                let (q_min, q_max) = quantifier_bounds(*quantifier);
                (
                    inner_min.saturating_mul(q_min),
                    inner_max.saturating_mul(q_max),
                )
            }
        };
        min_total = min_total.saturating_add(seg_min);
        max_total = max_total.saturating_add(seg_max);
    }

    (min_total, max_total)
}

fn quantifier_bounds(q: Quantifier) -> (usize, usize) {
    match q {
        Quantifier::Exact(n) => (n, n),
        Quantifier::Range(min, max) => (min, max),
        Quantifier::OneOrMore => (1, usize::MAX),
    }
}

/// Emit a negated char predicate (true when char does NOT match).
/// Avoids `!(b == X)` patterns that clippy flags.
fn emit_char_predicate_negated(ranges: &[CharRange]) -> TokenStream {
    let checks: Vec<TokenStream> = ranges
        .iter()
        .map(|r| match r {
            CharRange::Range(lo, hi) => {
                let lo_lit = *lo;
                let hi_lit = *hi;
                quote! { !(#lo_lit..=#hi_lit).contains(&b) }
            }
            CharRange::Single(ch) => {
                let ch_lit = *ch;
                quote! { b != #ch_lit }
            }
        })
        .collect();

    if checks.len() == 1 {
        checks.into_iter().next().unwrap()
    } else {
        // De Morgan's: NOT (A || B || C) = !A && !B && !C
        quote! { #(#checks)&&* }
    }
}

fn emit_fixed_length(segments: &[Segment], len: usize) -> TokenStream {
    // Build per-byte checks.
    let mut byte_checks = Vec::new();
    let mut offset = 0usize;
    emit_fixed_checks(segments, &mut offset, &mut byte_checks);

    let len_lit = len;

    if byte_checks.is_empty() {
        // Length-only check
        quote! {
            {
                let bytes = value.as_bytes();
                bytes.len() != #len_lit
            }
        }
    } else {
        quote! {
            {
                let bytes = value.as_bytes();
                bytes.len() != #len_lit || #(#byte_checks)||*
            }
        }
    }
}

fn emit_fixed_checks(segments: &[Segment], offset: &mut usize, checks: &mut Vec<TokenStream>) {
    for seg in segments {
        match seg {
            Segment::Literal(ch) => {
                let off = *offset;
                let ch_lit = *ch;
                checks.push(quote! { bytes[#off] != #ch_lit });
                *offset += 1;
            }
            Segment::CharClass {
                ranges, quantifier, ..
            } => {
                let count = match quantifier {
                    Quantifier::Exact(n) => *n,
                    _ => unreachable!("fixed-length path only for exact quantifiers"),
                };
                let neg_pred = emit_char_predicate_negated(ranges);
                for _ in 0..count {
                    let off = *offset;
                    // Wrap in parens so `(...) || (...)` is unambiguous
                    // (bare `{...} || {..}` parses `||` as a closure).
                    let check = quote! { ({ let b = bytes[#off]; #neg_pred }) };
                    checks.push(check);
                    *offset += 1;
                }
            }
            Segment::Group {
                segments: inner,
                quantifier,
            } => {
                let count = match quantifier {
                    Quantifier::Exact(n) => *n,
                    _ => unreachable!("fixed-length path only for exact quantifiers"),
                };
                for _ in 0..count {
                    emit_fixed_checks(inner, offset, checks);
                }
            }
        }
    }
}

fn emit_variable_length(segments: &[Segment], min_len: usize, max_len: usize) -> TokenStream {
    // Emit cursor-based validation.
    let mut stmts = Vec::new();

    // Length bounds check.
    if max_len < usize::MAX {
        stmts.push(quote! {
            if !(#min_len..=#max_len).contains(&len) {
                return true;
            }
        });
    } else {
        stmts.push(quote! {
            if len < #min_len {
                return true;
            }
        });
    }

    // Build cursor-advancing statements.
    emit_cursor_stmts(segments, &mut stmts);

    // Final check: cursor must have consumed all bytes.
    stmts.push(quote! {
        if pos != len {
            return true;
        }
    });

    quote! {
        {
            let bytes = value.as_bytes();
            let len = bytes.len();
            let result: bool = (|| -> bool {
                let mut pos: usize = 0;
                #(#stmts)*
                false
            })();
            result
        }
    }
}

fn emit_cursor_stmts(segments: &[Segment], stmts: &mut Vec<TokenStream>) {
    for seg in segments {
        match seg {
            Segment::Literal(ch) => {
                let ch_lit = *ch;
                stmts.push(quote! {
                    if pos >= len || bytes[pos] != #ch_lit {
                        return true;
                    }
                    pos += 1;
                });
            }
            Segment::CharClass {
                ranges, quantifier, ..
            } => {
                let neg_pred = emit_char_predicate_negated(ranges);
                match quantifier {
                    Quantifier::Exact(n) => {
                        let n_lit = *n;
                        stmts.push(quote! {
                            {
                                let end = pos + #n_lit;
                                if end > len {
                                    return true;
                                }
                                for &b in &bytes[pos..end] {
                                    if #neg_pred {
                                        return true;
                                    }
                                }
                                pos = end;
                            }
                        });
                    }
                    Quantifier::Range(min, max) => {
                        let min_lit = *min;
                        let max_lit = *max;
                        let (start_binding, min_guard) = if min_lit > 0 {
                            (
                                quote! { let start = pos; },
                                quote! {
                                    let matched = pos - start;
                                    if matched < #min_lit {
                                        return true;
                                    }
                                },
                            )
                        } else {
                            (TokenStream::new(), TokenStream::new())
                        };
                        if *max == usize::MAX {
                            stmts.push(quote! {
                                {
                                    #start_binding
                                    while pos < len {
                                        let b = bytes[pos];
                                        if #neg_pred {
                                            break;
                                        }
                                        pos += 1;
                                    }
                                    #min_guard
                                }
                            });
                        } else {
                            stmts.push(quote! {
                                {
                                    #start_binding
                                    let limit = if pos + #max_lit < len { pos + #max_lit } else { len };
                                    while pos < limit {
                                        let b = bytes[pos];
                                        if #neg_pred {
                                            break;
                                        }
                                        pos += 1;
                                    }
                                    #min_guard
                                }
                            });
                        }
                    }
                    Quantifier::OneOrMore => {
                        let min_lit = 1usize;
                        stmts.push(quote! {
                            {
                                let start = pos;
                                while pos < len {
                                    let b = bytes[pos];
                                    if #neg_pred {
                                        break;
                                    }
                                    pos += 1;
                                }
                                let matched = pos - start;
                                if matched < #min_lit {
                                    return true;
                                }
                            }
                        });
                    }
                }
            }
            Segment::Group {
                segments: inner,
                quantifier,
            } => {
                match quantifier {
                    Quantifier::Exact(n) => {
                        // For fixed-count groups, unroll
                        for _ in 0..*n {
                            emit_cursor_stmts(inner, stmts);
                        }
                    }
                    Quantifier::Range(min, max) => {
                        // For {0,1} (optional group), emit a try-match
                        let (_inner_min_len, _inner_max_len) = compute_length_bounds(inner);
                        let min_lit = *min;

                        if *min == 0 && *max == 1 {
                            // Optional group: try to match if enough chars remain
                            let mut inner_stmts = Vec::new();
                            emit_cursor_stmts(inner, &mut inner_stmts);
                            stmts.push(quote! {
                                {
                                    let saved = pos;
                                    let matched: bool = (|| -> bool {
                                        #(#inner_stmts)*
                                        false
                                    })();
                                    if matched {
                                        pos = saved; // revert — optional group didn't match
                                    }
                                }
                            });
                        } else {
                            // General range group — match min..=max repetitions
                            let max_lit = *max;
                            let mut inner_stmts = Vec::new();
                            emit_cursor_stmts(inner, &mut inner_stmts);
                            // inner_min_len / inner_max_len available for future bounds reasoning
                            let min_guard = if min_lit > 0 {
                                quote! {
                                    if count < #min_lit {
                                        return true;
                                    }
                                }
                            } else {
                                TokenStream::new()
                            };
                            stmts.push(quote! {
                                {
                                    let mut count = 0usize;
                                    while count < #max_lit {
                                        let saved = pos;
                                        let failed: bool = (|| -> bool {
                                            #(#inner_stmts)*
                                            false
                                        })();
                                        if failed {
                                            pos = saved;
                                            break;
                                        }
                                        count += 1;
                                    }
                                    #min_guard
                                }
                            });
                        }
                    }
                    Quantifier::OneOrMore => {
                        let mut inner_stmts = Vec::new();
                        emit_cursor_stmts(inner, &mut inner_stmts);
                        stmts.push(quote! {
                            {
                                let mut count = 0usize;
                                loop {
                                    let saved = pos;
                                    let failed: bool = (|| -> bool {
                                        #(#inner_stmts)*
                                        false
                                    })();
                                    if failed {
                                        pos = saved;
                                        break;
                                    }
                                    count += 1;
                                }
                                if count < 1 {
                                    return true;
                                }
                            }
                        });
                    }
                }
            }
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_parses(pattern: &str) {
        let ts = emit_pattern_check(pattern)
            .unwrap_or_else(|| panic!("emit_pattern_check returned None for: {pattern}"));
        let full = quote! {
            fn check(value: &str) -> bool {
                #ts
            }
        };
        let src = full.to_string();
        syn::parse_file(&src).unwrap_or_else(|e| {
            panic!("invalid Rust for pattern '{pattern}':\n{src}\nerror: {e}");
        });
    }

    // ── Parser unit tests ────────────────────────────────────────────────────

    #[test]
    fn parse_simple_char_class() {
        let segs = parse_pattern("[A-Z]{3,3}").unwrap();
        assert_eq!(segs.len(), 1);
        assert!(matches!(
            &segs[0],
            Segment::CharClass {
                ranges,
                quantifier: Quantifier::Exact(3)
            } if ranges.len() == 1
        ));
    }

    #[test]
    fn parse_literal() {
        let segs = parse_pattern("-").unwrap();
        assert_eq!(segs.len(), 1);
        assert!(matches!(&segs[0], Segment::Literal(b'-')));
    }

    #[test]
    fn parse_escaped_plus() {
        let segs = parse_pattern(r"[\+]{0,1}").unwrap();
        assert_eq!(segs.len(), 1);
        assert!(matches!(
            &segs[0],
            Segment::CharClass {
                quantifier: Quantifier::Range(0, 1),
                ..
            }
        ));
    }

    #[test]
    fn parse_group() {
        let segs = parse_pattern("([A-Z]{2}){3}").unwrap();
        assert_eq!(segs.len(), 1);
        assert!(matches!(
            &segs[0],
            Segment::Group {
                quantifier: Quantifier::Exact(3),
                ..
            }
        ));
    }

    #[test]
    fn parse_one_or_more() {
        let segs = parse_pattern("[0-9a-fA-F]+").unwrap();
        assert_eq!(segs.len(), 1);
        assert!(matches!(
            &segs[0],
            Segment::CharClass {
                quantifier: Quantifier::OneOrMore,
                ..
            }
        ));
    }

    // ── All 22 patterns must emit valid Rust ─────────────────────────────────

    #[test]
    fn pattern_01_three_uppercase() {
        assert_parses("[A-Z]{3,3}");
    }

    #[test]
    fn pattern_02_two_uppercase() {
        assert_parses("[A-Z]{2,2}");
    }

    #[test]
    fn pattern_03_two_lowercase() {
        assert_parses("[a-z]{2,2}");
    }

    #[test]
    fn pattern_04_one_digit() {
        assert_parses("[0-9]");
    }

    #[test]
    fn pattern_05_two_digits() {
        assert_parses("[0-9]{2}");
    }

    #[test]
    fn pattern_06_three_digits() {
        assert_parses("[0-9]{3}");
    }

    #[test]
    fn pattern_07_one_to_three_digits() {
        assert_parses("[0-9]{1,3}");
    }

    #[test]
    fn pattern_08_one_to_five_digits() {
        assert_parses("[0-9]{1,5}");
    }

    #[test]
    fn pattern_09_two_to_three_digits() {
        assert_parses("[0-9]{2,3}");
    }

    #[test]
    fn pattern_10_three_to_four_digits() {
        assert_parses("[0-9]{3,4}");
    }

    #[test]
    fn pattern_11_one_to_fifteen_digits() {
        assert_parses("[0-9]{1,15}");
    }

    #[test]
    fn pattern_12_eight_to_twentyeight_digits() {
        assert_parses("[0-9]{8,28}");
    }

    #[test]
    fn pattern_13_four_alnum() {
        assert_parses("[a-zA-Z0-9]{4}");
    }

    #[test]
    fn pattern_14_hex_plus() {
        assert_parses("[0-9a-fA-F]+");
    }

    #[test]
    fn pattern_15_iban() {
        assert_parses("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}");
    }

    #[test]
    fn pattern_16_lei() {
        assert_parses("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}");
    }

    #[test]
    fn pattern_17_eighteen_alnum_two_digit() {
        assert_parses("[A-Z0-9]{18,18}[0-9]{2,2}");
    }

    #[test]
    fn pattern_18_bic() {
        assert_parses("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}");
    }

    #[test]
    fn pattern_19_64_hex_upper() {
        assert_parses("([0-9A-F][0-9A-F]){32}");
    }

    #[test]
    fn pattern_20_uuid_v4() {
        assert_parses("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}");
    }

    #[test]
    fn pattern_21_optional_plus_digits() {
        assert_parses(r"[\+]{0,1}[0-9]{1,15}");
    }

    #[test]
    fn pattern_22_phone() {
        assert_parses(r"\+[0-9]{1,3}-[0-9()+\-]{1,30}");
    }

    // ── Emission structure tests ─────────────────────────────────────────────

    #[test]
    fn fixed_length_pattern_emits_exact_len_check() {
        let ts = emit_pattern_check("[A-Z]{3,3}").unwrap();
        let src = ts.to_string();
        // Fixed-length: should check bytes.len() != 3
        assert!(src.contains("!= 3"), "should check exact length 3: {src}");
    }

    #[test]
    fn variable_length_pattern_uses_cursor() {
        let ts = emit_pattern_check("[0-9]{1,3}").unwrap();
        let src = ts.to_string();
        // Variable-length: should use cursor-based validation
        assert!(src.contains("pos"), "should use cursor variable: {src}");
    }

    #[test]
    fn unsupported_pattern_returns_none() {
        // An unclosed bracket should fail to parse
        assert!(emit_pattern_check("[A-Z").is_none());
    }

    #[test]
    fn empty_pattern_returns_some() {
        // Empty pattern matches everything — no violation
        let ts = emit_pattern_check("").unwrap();
        assert!(!ts.is_empty());
    }

    #[test]
    fn bic_pattern_handles_optional_suffix() {
        let ts =
            emit_pattern_check("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
        let src = ts.to_string();
        // Should have cursor-based validation (variable length: 8 or 11)
        assert!(src.contains("pos"), "BIC pattern should use cursor: {src}");
    }
}
