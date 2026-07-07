//! Hand-rolled CMake lexer.
//!
//! Produces the exact `(Result<Token, ()>, Range)` stream the previous
//! logos-generated DFA produced (including error spans), but drives scanning
//! with byte dispatch, a stop-byte table for unquoted runs, and `memchr` for
//! delimiter searches. Parity is enforced by the differential tests below.

use std::ops::Range;

use super::token::Token;

/// Bytes that terminate an `UnquotedText` run: ` \t\r\n()#"\`.
/// (`[` and `]` are allowed in continuation position.)
static UNQUOTED_STOP: [bool; 256] = {
    let mut t = [false; 256];
    t[b' ' as usize] = true;
    t[b'\t' as usize] = true;
    t[b'\r' as usize] = true;
    t[b'\n' as usize] = true;
    t[b'(' as usize] = true;
    t[b')' as usize] = true;
    t[b'#' as usize] = true;
    t[b'"' as usize] = true;
    t[b'\\' as usize] = true;
    t
};

/// Streaming lexer over `source` bytes; `next_token` yields spans into it.
pub(crate) struct RawLexer<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> RawLexer<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            src: source.as_bytes(),
            pos: 0,
        }
    }

    /// Next token with its byte span, or `None` at end of input.
    /// `Err(())` mirrors the logos error token (unknown/unclosed constructs).
    pub(crate) fn next_token(&mut self) -> Option<(Result<Token, ()>, Range<usize>)> {
        let src = self.src;
        let len = src.len();
        let mut pos = self.pos;

        // Skip [ \t]+ separators.
        while pos < len && matches!(src[pos], b' ' | b'\t') {
            pos += 1;
        }
        if pos >= len {
            self.pos = pos;
            return None;
        }

        let start = pos;
        let (tok, end) = match src[pos] {
            b'\n' => (Ok(Token::Newline), pos + 1),
            b'\r' => {
                if pos + 1 < len && src[pos + 1] == b'\n' {
                    (Ok(Token::Newline), pos + 2)
                } else {
                    // Bare CR matches no token.
                    (Err(()), pos + 1)
                }
            }
            b'(' => (Ok(Token::LParen), pos + 1),
            b')' => (Ok(Token::RParen), pos + 1),
            b'"' => scan_quoted(src, pos),
            b'#' => scan_hash(src, pos),
            b'[' => scan_bracket_argument(src, pos),
            _ => scan_unquoted(src, pos),
        };
        self.pos = end;
        Some((tok, start..end))
    }
}

/// Scan a quoted argument starting at the opening `"`.
fn scan_quoted(src: &[u8], pos: usize) -> (Result<Token, ()>, usize) {
    let mut i = pos + 1;
    while let Some(offset) = memchr::memchr2(b'"', b'\\', &src[i..]) {
        let at = i + offset;
        match src[at] {
            b'"' => return (Ok(Token::QuotedArgument), at + 1),
            _ if at + 1 < src.len() => i = at + 2,
            // Backslash at end of input — no char to escape.
            _ => break,
        }
    }
    (Err(()), pos + 1)
}

/// Scan a token starting at `#`: bracket comment, else line comment.
fn scan_hash(src: &[u8], pos: usize) -> (Result<Token, ()>, usize) {
    if let Some(open_len) = bracket_opener_len(src, pos + 1) {
        // `#[=*[` — bracket comment; find the matching `]=*]`.
        match find_bracket_close(src, pos + 1, open_len) {
            Some(end) => return (Ok(Token::BracketComment), end),
            // Unclosed bracket comment: the line-comment fallback also rejects
            // `#` followed by a well-formed opener, so the error spans `#` plus
            // the opener (mirroring the logos DFA's failed-match span).
            None => return (Err(()), pos + 1 + open_len),
        }
    }
    // Line comment: consume to (excluding) the next newline, or to EOF.
    let end = match memchr::memchr(b'\n', &src[pos + 1..]) {
        Some(nl) => pos + 1 + nl,
        None => src.len(),
    };
    (Ok(Token::LineComment), end)
}

/// Scan a bracket argument starting at `[`.
///
/// A `[` that does not begin a well-formed `[=*[` opener is an error token
/// spanning the attempted-opener prefix (`[` plus any run of `=`), matching
/// the failed-match span of the previous DFA.
fn scan_bracket_argument(src: &[u8], pos: usize) -> (Result<Token, ()>, usize) {
    let mut i = pos + 1;
    while i < src.len() && src[i] == b'=' {
        i += 1;
    }
    if i < src.len() && src[i] == b'[' {
        let open_len = i + 1 - pos;
        match find_bracket_close(src, pos, open_len) {
            Some(end) => (Ok(Token::BracketArgument), end),
            // Unclosed: the error spans the matched opener.
            None => (Err(()), pos + open_len),
        }
    } else {
        (Err(()), i)
    }
}

/// Length of a `[=*[` opener at `pos`, or `None` if not an opener.
fn bracket_opener_len(src: &[u8], pos: usize) -> Option<usize> {
    if pos >= src.len() || src[pos] != b'[' {
        return None;
    }
    let mut i = pos + 1;
    while i < src.len() && src[i] == b'=' {
        i += 1;
    }
    if i < src.len() && src[i] == b'[' {
        Some(i + 1 - pos)
    } else {
        None
    }
}

/// Find the end (exclusive) of a bracket construct whose opener `[=*[` starts
/// at `open_start` with length `open_len`; the closer is `]=*]` with the same
/// `=` count.
fn find_bracket_close(src: &[u8], open_start: usize, open_len: usize) -> Option<usize> {
    let eq_count = open_len - 2;
    let content_start = open_start + open_len;
    let mut i = content_start;
    while let Some(offset) = memchr::memchr(b']', &src[i..]) {
        let at = i + offset;
        let close_end = at + 1 + eq_count + 1;
        if close_end <= src.len()
            && src[at + 1..at + 1 + eq_count].iter().all(|&b| b == b'=')
            && src[at + 1 + eq_count] == b']'
        {
            return Some(close_end);
        }
        i = at + 1;
    }
    None
}

/// Scan an UnquotedText run starting at a non-stop byte (or `\`).
///
/// Grammar (from the previous DFA): first char is any byte outside
/// ` \t\r\n()#"\[` or an escape `\<any-but-newline>`; continuation chars are
/// the same but with `[` allowed.
fn scan_unquoted(src: &[u8], pos: usize) -> (Result<Token, ()>, usize) {
    let len = src.len();
    let mut i = pos;

    // First char: dispatch guarantees it's not a stop byte other than `\`,
    // and not `[`.
    if src[i] == b'\\' {
        if i + 1 < len && src[i + 1] != b'\n' {
            i += 2;
        } else {
            // Escape with nothing (or a newline) after it matches no token.
            return (Err(()), i + 1);
        }
    } else {
        i += 1;
    }

    loop {
        i = find_stop(src, i);
        if i < len && src[i] == b'\\' && i + 1 < len && src[i + 1] != b'\n' {
            i += 2;
            continue;
        }
        break;
    }
    (Ok(Token::UnquotedText), i)
}

/// Index of the first stop byte at or after `i`, or `src.len()`.
#[inline]
fn find_stop(src: &[u8], i: usize) -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        if std::arch::is_x86_feature_detected!("avx2") {
            // SAFETY: AVX2 support was just verified.
            return unsafe { simd::find_stop_avx2(src, i) };
        }
    }
    find_stop_scalar(src, i)
}

#[inline]
fn find_stop_scalar(src: &[u8], i: usize) -> usize {
    src[i..]
        .iter()
        .position(|&b| UNQUOTED_STOP[b as usize])
        .map_or(src.len(), |off| i + off)
}

/// AVX2 classification of the 9-byte stop set via the nibble-table trick:
/// a byte `b` is a stop byte iff `LO_TABLE[b & 0xF] & HI_TABLE[b >> 4] != 0`.
/// `vpshufb` zeroes lanes whose index has the high bit set, so bytes >= 0x80
/// classify as non-stop for free (the stop set is pure ASCII).
#[cfg(target_arch = "x86_64")]
mod simd {
    use std::arch::x86_64::*;

    /// Per-low-nibble bitmasks of the hi-nibble groups (bit0: hi 0x0,
    /// bit1: hi 0x2, bit2: hi 0x5) that form stop bytes:
    /// `\t`(09) `\n`(0A) `\r`(0D) ` `(20) `"`(22) `#`(23) `(`(28) `)`(29) `\`(5C).
    const LO: [i8; 16] = [2, 0, 2, 2, 0, 0, 0, 0, 2, 3, 1, 0, 4, 1, 0, 0];
    const HI: [i8; 16] = [1, 0, 2, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    #[target_feature(enable = "avx2")]
    pub(super) fn find_stop_avx2(src: &[u8], start: usize) -> usize {
        // SAFETY throughout: every 32-byte load is bounded by the
        // `i + 32 <= len` guard; the remaining tail is scanned scalar.

        unsafe {
            let lo_table = _mm256_broadcastsi128_si256(_mm_loadu_si128(LO.as_ptr().cast()));
            let hi_table = _mm256_broadcastsi128_si256(_mm_loadu_si128(HI.as_ptr().cast()));
            let low_mask = _mm256_set1_epi8(0x0F);
            let zero = _mm256_setzero_si256();

            let len = src.len();
            let ptr = src.as_ptr();
            let mut i = start;

            while i + 32 <= len {
                let chunk = _mm256_loadu_si256(ptr.add(i).cast());
                let lo_bits = _mm256_shuffle_epi8(lo_table, _mm256_and_si256(chunk, low_mask));
                let hi_nib = _mm256_and_si256(_mm256_srli_epi16(chunk, 4), low_mask);
                let hi_bits = _mm256_shuffle_epi8(hi_table, hi_nib);
                let classified = _mm256_and_si256(lo_bits, hi_bits);
                let mask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(classified, zero)) as u32;
                // `mask` has 0 bits where a stop byte sits.
                if mask != u32::MAX {
                    return i + (!mask).trailing_zeros() as usize;
                }
                i += 32;
            }

            // Scalar tail (< 32 bytes).
            while i < len && !super::UNQUOTED_STOP[*ptr.add(i) as usize] {
                i += 1;
            }
            i
        }
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    type TokenStream = Vec<(Result<Token, ()>, Range<usize>)>;

    fn lex_logos(input: &str) -> TokenStream {
        let mut lex = Token::lexer(input);
        let mut out = Vec::new();
        while let Some(result) = lex.next() {
            out.push((result, lex.span()));
        }
        out
    }

    fn lex_raw(input: &str) -> TokenStream {
        let mut lex = RawLexer::new(input);
        let mut out = Vec::new();
        while let Some((tok, span)) = lex.next_token() {
            out.push((tok, span));
        }
        out
    }

    #[track_caller]
    fn assert_parity(input: &str) {
        let expected = lex_logos(input);
        let actual = lex_raw(input);
        assert_eq!(
            expected, actual,
            "token stream mismatch for input: {input:?}\nlogos: {expected:?}\nraw:   {actual:?}"
        );
    }

    #[test]
    fn parity_edge_cases() {
        let cases: &[&str] = &[
            "",
            " ",
            " \t ",
            "a",
            "if(A AND B)",
            "a  \t  b",
            "a\nb",
            "a\r\nb",
            "\r",
            "a\rb",
            "\r\r\n",
            "[[content]]",
            "[=[content]=]",
            "[===[nested ]] content]===]",
            "[=[unclosed",
            "[[unclosed",
            "[",
            "[abc",
            "[=x",
            "[]",
            "a[b]c",
            "a]b",
            "#[[comment]]",
            "#[=[comment]=]",
            "#[[unclosed",
            "#[=[unclosed",
            "# plain comment\nnext",
            "# comment at eof",
            "#",
            "#[",
            "#[=",
            "#[=x rest\n",
            "\"hello world\"",
            "\"hello \\\"world\\\"\"",
            "\"unclosed",
            "\"hello\\",
            "\"multi\nline\"",
            "\"\"",
            "-Da=\"b c\"",
            "a\\ b",
            "a\\;b",
            "\\",
            "\\\n",
            "\\x",
            "a\\",
            "a\\\nb",
            "a\\\r\nb",
            "\\\r",
            "esc\\\rx",
            "set(VAR \"value\")",
            "$<$<BOOL:x>:(y)>",
            "()",
            "(())",
            "cmd (a)\n",
            "über(ø \"π\")",
            "\\é",
            "a\u{00e9}b",
            "#comment\r\ncmd()",
            "\"a\\\\\"b",
            "x=[[not bracket",
            "x[[y]]",
            "]",
            "]]",
            "]=]",
        ];
        for case in cases {
            assert_parity(case);
        }
    }

    #[test]
    fn parity_full_fixture_corpus() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/formatter");
        let mut stack = vec![root];
        let mut checked = 0usize;
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("read_dir") {
                let path = entry.expect("entry").path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "cmake") {
                    let text = std::fs::read_to_string(&path).expect("read fixture");
                    let expected = lex_logos(&text);
                    let actual = lex_raw(&text);
                    assert_eq!(expected, actual, "token stream mismatch in {path:?}");
                    checked += 1;
                }
            }
        }
        assert!(
            checked > 500,
            "expected to check many fixtures, got {checked}"
        );
    }

    #[test]
    fn parity_randomized_snippets() {
        // Deterministic LCG over a CMake-ish alphabet, heavy on delimiters and
        // escapes, to shake out span/error mismatches beyond the corpus.
        let alphabet: &[u8] = b"ab$<>{}();#\"\\[]= \t\r\nxyz_-/.:0123456789";
        let mut state: u64 = 0x243F_6A88_85A3_08D3;
        let mut next = move || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 33) as usize
        };
        for _ in 0..4000 {
            let len = next() % 40;
            let bytes: Vec<u8> = (0..len)
                .map(|_| alphabet[next() % alphabet.len()])
                .collect();
            let Ok(input) = std::str::from_utf8(&bytes) else {
                continue;
            };
            assert_parity(input);
        }
    }
}
