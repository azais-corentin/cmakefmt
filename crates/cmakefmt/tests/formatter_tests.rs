use std::path::{Path, PathBuf};

use cmakefmt::{Configuration, format_text, load_from_toml_path};
use rstest::rstest;

// ---------------------------------------------------------------------------
// Per-fixture test — rstest generates one #[test] per .in.cmake file
// ---------------------------------------------------------------------------

#[rstest]
fn test_fixture(#[files("tests/formatter/**/*.in.cmake")] in_path: PathBuf) {
    let stem = in_path
        .to_str()
        .expect("non-utf8 path")
        .strip_suffix(".in.cmake")
        .expect("expected .in.cmake suffix");
    let out_path = PathBuf::from(format!("{stem}.out.cmake"));
    assert!(
        out_path.exists(),
        "Missing .out.cmake for {}",
        in_path.display()
    );

    let input = std::fs::read_to_string(&in_path).unwrap();
    let expected = std::fs::read_to_string(&out_path).unwrap();
    let config = load_fixture_config(&in_path);

    match format_text(in_path.as_path(), &input, &config) {
        Ok(Some(formatted)) => {
            assert_eq_with_diff(&expected, &formatted, &in_path);
            check_idempotency(in_path.as_path(), &formatted, &config);
        }
        Ok(None) => {
            // No change — input should already match expected
            assert_eq_with_diff(&expected, &input, &in_path);
            check_idempotency(in_path.as_path(), &input, &config);
        }
        Err(e) => panic!("PARSE ERROR: {}\n{e}", in_path.display()),
    }
}

// ---------------------------------------------------------------------------
// Orphan detection — catches .out.cmake files without a matching .in.cmake
// ---------------------------------------------------------------------------

#[test]
fn no_orphan_out_files() {
    let formatter_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/formatter");
    let out_files = walk_cmake_files(&formatter_dir, ".out.cmake");

    let mut orphans = Vec::new();
    for out_path in &out_files {
        let stem = out_path
            .to_str()
            .unwrap()
            .strip_suffix(".out.cmake")
            .unwrap();
        let in_path = PathBuf::from(format!("{stem}.in.cmake"));
        if !in_path.exists() {
            orphans.push(out_path.display().to_string());
        }
    }

    assert!(
        orphans.is_empty(),
        "Found .out.cmake files without matching .in.cmake:\n{}",
        orphans.join("\n")
    );
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn load_fixture_config(in_path: &Path) -> Configuration {
    let result = load_from_toml_path(in_path);
    result.config
}

/// Panics with a colored diff when `expected` and `actual` differ.
fn assert_eq_with_diff(expected: &str, actual: &str, path: &Path) {
    if expected != actual {
        panic!(
            "MISMATCH: {}\n--- expected\n+++ actual\n{}",
            path.display(),
            simple_diff(expected, actual)
        );
    }
}

/// Formats `text` a second time and panics if the output changes (non-idempotent).
fn check_idempotency(path: &Path, text: &str, config: &Configuration) {
    match format_text(path, text, config) {
        Ok(None) => { /* Already formatted — pass */ }
        Ok(Some(reformatted)) => {
            if reformatted != text {
                panic!(
                    "IDEMPOTENCY: {}\nSecond format produced different output\n{}",
                    path.display(),
                    simple_diff(text, &reformatted)
                );
            }
        }
        Err(e) => {
            panic!(
                "IDEMPOTENCY ERROR: {}\nSecond format failed: {e}",
                path.display()
            );
        }
    }
}

fn walk_cmake_files(dir: &Path, suffix: &str) -> Vec<PathBuf> {
    let mut results = Vec::new();
    walk_recursive(dir, suffix, &mut results);
    results
}

fn walk_recursive(dir: &Path, suffix: &str, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(err) => panic!("Failed to read directory {}: {}", dir.display(), err),
    };
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            walk_recursive(&path, suffix, out);
        } else if path
            .file_name()
            .and_then(|n| n.to_str())
            .map_or(false, |n| n.ends_with(suffix))
        {
            out.push(path);
        }
    }
}

fn simple_diff(expected: &str, actual: &str) -> String {
    use std::cell::Cell;
    use std::fmt;

    use imara_diff::{
        Algorithm, Diff, InternedInput, Interner, Token, UnifiedDiffConfig, UnifiedDiffPrinter,
    };

    struct ColoredLineDiffPrinter<'a> {
        interner: &'a Interner<&'a str>,
        has_invisible_hunk: Cell<bool>,
    }

    impl UnifiedDiffPrinter for ColoredLineDiffPrinter<'_> {
        fn display_header(
            &self,
            mut f: impl fmt::Write,
            start_before: u32,
            start_after: u32,
            len_before: u32,
            len_after: u32,
        ) -> fmt::Result {
            writeln!(
                f,
                "\x1b[36m@@ -{},{} +{},{} @@\x1b[0m",
                start_before + 1,
                len_before,
                start_after + 1,
                len_after
            )
        }

        fn display_context_token(&self, mut f: impl fmt::Write, token: Token) -> fmt::Result {
            let line = self.interner[token];
            write!(f, " {line}")?;
            if !line.ends_with('\n') {
                writeln!(f)?;
            }
            Ok(())
        }

        fn display_hunk(
            &self,
            mut f: impl fmt::Write,
            before: &[Token],
            after: &[Token],
        ) -> fmt::Result {
            let before_text: String = before.iter().map(|&t| self.interner[t]).collect();
            let after_text: String = after.iter().map(|&t| self.interner[t]).collect();
            let invisible_only = before_text != after_text
                && strip_invisible(&before_text) == strip_invisible(&after_text);

            if invisible_only {
                self.has_invisible_hunk.set(true);
            }

            if let Some(&last) = before.last() {
                for &token in before {
                    let line = self.interner[token];
                    let display = if invisible_only {
                        make_invisible_visible(line)
                    } else {
                        line.to_owned()
                    };
                    write!(f, "\x1b[31m-{display}\x1b[0m")?;
                }
                if !self.interner[last].ends_with('\n') {
                    writeln!(f)?;
                }
            }
            if let Some(&last) = after.last() {
                for &token in after {
                    let line = self.interner[token];
                    let display = if invisible_only {
                        make_invisible_visible(line)
                    } else {
                        line.to_owned()
                    };
                    write!(f, "\x1b[32m+{display}\x1b[0m")?;
                }
                if !self.interner[last].ends_with('\n') {
                    writeln!(f)?;
                }
            }
            Ok(())
        }
    }

    let input = InternedInput::new(expected, actual);
    let mut diff = Diff::compute(Algorithm::Histogram, &input);
    diff.postprocess_lines(&input);

    let printer = ColoredLineDiffPrinter {
        interner: &input.interner,
        has_invisible_hunk: Cell::new(false),
    };
    let result = diff
        .unified_diff(&printer, UnifiedDiffConfig::default(), &input)
        .to_string();

    if printer.has_invisible_hunk.get() {
        format!(
            "(some hunks have invisible-only differences; showing: \u{00B7} = space, \u{2192} = tab, \u{240D} = CR, \u{240A} = LF, \u{237D} = NBSP, <\u{2026}> = other)\n{result}"
        )
    } else {
        result
    }
}

/// Returns true for characters considered invisible in diffs.
///
/// Note: CR/LF are intentionally excluded so pure line-ending changes are
/// treated as regular (non-invisible-only) diffs.
fn is_invisible(c: char) -> bool {
    if matches!(c, '\r' | '\n') {
        return false;
    }

    c.is_whitespace()
        || c.is_control()
        || matches!(
            c,
            '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}' | '\u{2060}' | '\u{00AD}'
        )
}

fn strip_invisible(s: &str) -> String {
    s.chars().filter(|c| !is_invisible(*c)).collect()
}

fn make_invisible_visible(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        match c {
            ' ' => out.push('\u{00B7}'),
            '\t' => out.push('\u{2192}'),
            '\r' => out.push('\u{240D}'),
            '\n' => {
                out.push('\u{240A}');
                out.push('\n');
            }
            '\0' => out.push('\u{2400}'),
            '\u{00A0}' => out.push('\u{237D}'),
            '\u{200B}' => out.push_str("<ZWSP>"),
            '\u{200C}' => out.push_str("<ZWNJ>"),
            '\u{200D}' => out.push_str("<ZWJ>"),
            '\u{FEFF}' => out.push_str("<BOM>"),
            '\u{2060}' => out.push_str("<WJ>"),
            '\u{00AD}' => out.push_str("<SHY>"),
            c if c.is_control() => {
                use std::fmt::Write;
                let _ = write!(out, "<0x{:02X}>", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod invisible_diff_tests {
    use super::*;

    #[test]
    fn test_strip_invisible() {
        assert_eq!(strip_invisible("hello world"), "helloworld");
        assert_eq!(strip_invisible("a\tb\nc"), "ab\nc");
        assert_eq!(strip_invisible("a\r\nb"), "a\r\nb");
        assert_eq!(strip_invisible(""), "");
        assert_eq!(strip_invisible("abc"), "abc");
        // Zero-width chars
        assert_eq!(strip_invisible("a\u{200B}b"), "ab");
        assert_eq!(strip_invisible("a\u{FEFF}b"), "ab");
    }

    #[test]
    fn test_make_invisible_visible() {
        assert_eq!(make_invisible_visible(" "), "\u{00B7}");
        assert_eq!(make_invisible_visible("\t"), "\u{2192}");
        assert_eq!(make_invisible_visible("\r"), "\u{240D}");
        assert_eq!(make_invisible_visible("\n"), "\u{240A}\n");
        assert_eq!(make_invisible_visible("\0"), "\u{2400}");
        assert_eq!(make_invisible_visible("\u{00A0}"), "\u{237D}");
        assert_eq!(make_invisible_visible("\u{200B}"), "<ZWSP>");
        assert_eq!(make_invisible_visible("\u{FEFF}"), "<BOM>");
        assert_eq!(make_invisible_visible("abc"), "abc");
        assert_eq!(make_invisible_visible("a b"), "a\u{00B7}b");
    }

    #[test]
    fn test_simple_diff_invisible_only() {
        // Tab vs spaces — invisible-only difference
        let result = simple_diff("a\tb\n", "a  b\n");
        assert!(
            result.contains("invisible-only differences"),
            "expected invisible-only legend, got: {result}"
        );
        assert!(result.contains('\u{2192}'), "expected tab symbol in diff");
        assert!(result.contains('\u{00B7}'), "expected space symbol in diff");
    }

    #[test]
    fn test_simple_diff_invisible_only_renders_crlf_symbols() {
        let result = simple_diff("a\tb\r\n", "a  b\r\n");
        assert!(
            result.contains("invisible-only differences"),
            "expected invisible-only legend, got: {result}"
        );
        assert!(result.contains('\u{240D}'), "expected CR symbol in diff");
        assert!(result.contains('\u{240A}'), "expected LF symbol in diff");
    }

    #[test]
    fn test_simple_diff_trailing_newline_is_not_invisible_only() {
        // Trailing newline is now treated as a regular visible diff trigger.
        let result = simple_diff("hello\n", "hello");
        assert!(
            !result.contains("invisible-only differences"),
            "trailing newline diff should not trigger invisible-only mode, got: {result}"
        );
    }

    #[test]
    fn test_simple_diff_visible_differences_unchanged() {
        // Visible differences should produce normal diff output
        let result = simple_diff("foo\n", "bar\n");
        assert!(!result.contains("invisible-only differences"));
        assert!(result.contains("foo"));
        assert!(result.contains("bar"));
    }

    #[test]
    fn test_simple_diff_trailing_space_shows_invisible() {
        // A trailing space is invisible — should trigger invisible-only mode
        let result = simple_diff("PUBLIC main1.cxx\n", "PUBLIC main1.cxx \n");
        assert!(
            result.contains("invisible-only differences"),
            "trailing space diff should trigger invisible-only mode, got: {result}"
        );
        assert!(
            result.contains('\u{00B7}'),
            "expected middle dot for space in diff"
        );
    }
}
