use memchr::memchr2_iter;

/// Count dominant line-ending style. CRLF is counted as one unit; bare `\r`
/// (not followed by `\n`) is not counted per §7.1. On tie or no endings, LF wins.
pub(crate) fn detect_dominant_line_ending(text: &str) -> &'static str {
    let bytes = text.as_bytes();
    let mut lf: u32 = 0;
    let mut crlf: u32 = 0;

    for pos in memchr2_iter(b'\r', b'\n', bytes) {
        if bytes[pos] == b'\r' {
            if bytes.get(pos + 1) == Some(&b'\n') {
                crlf += 1;
            }
            // Bare CR — not counted as a line ending.
        } else {
            // Standalone `\n`. But skip if preceded by `\r` (already counted as CRLF).
            if pos > 0 && bytes[pos - 1] == b'\r' {
                continue;
            }
            lf += 1;
        }
    }

    if crlf > lf { "\r\n" } else { "\n" }
}
