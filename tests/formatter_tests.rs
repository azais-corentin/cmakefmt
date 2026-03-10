use std::collections::HashSet;
use std::path::{Path, PathBuf};

use cmakefmt::{CaseStyle, Configuration, format_text, load_from_header};

#[test]
fn test_formatter_files() {
    let formatter_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/formatter");
    let mut failures: Vec<String> = Vec::new();
    let mut count = 0;

    let mut in_files = walk_cmake_files(&formatter_dir, ".in.cmake");
    in_files.sort();
    let out_files = walk_cmake_files(&formatter_dir, ".out.cmake");

    let in_stems: HashSet<String> = in_files
        .iter()
        .map(|p| {
            p.to_str()
                .unwrap()
                .strip_suffix(".in.cmake")
                .unwrap()
                .to_owned()
        })
        .collect();
    let out_stems: HashSet<String> = out_files
        .iter()
        .map(|p| {
            p.to_str()
                .unwrap()
                .strip_suffix(".out.cmake")
                .unwrap()
                .to_owned()
        })
        .collect();

    for stem in in_stems.difference(&out_stems) {
        failures.push(format!("Missing .out.cmake for {stem}.in.cmake"));
    }
    for stem in out_stems.difference(&in_stems) {
        failures.push(format!("Missing .in.cmake for {stem}.out.cmake"));
    }

    for in_path in &in_files {
        // foo.in.cmake -> foo.out.cmake
        let stem = in_path
            .to_str()
            .expect("non-utf8 path")
            .strip_suffix(".in.cmake")
            .expect("expected .in.cmake extension");
        let out_path = PathBuf::from(format!("{stem}.out.cmake"));

        if !out_stems.contains(stem) {
            continue;
        }

        let raw_input = std::fs::read_to_string(in_path).unwrap();
        let expected = std::fs::read_to_string(&out_path).unwrap();

        // Strip ### header line if present and try to parse config
        let (input, config, header_overrides, header_diagnostics, has_custom_header) =
            if raw_input.starts_with("### ") {
                let header_end = raw_input.find('\n');
                let header_content = match header_end {
                    Some(pos) => &raw_input[4..pos],
                    None => &raw_input[4..],
                };
                let remaining = match header_end {
                    Some(pos) => &raw_input[pos + 1..],
                    None => "",
                };
                let parsed_header = parse_config_header(header_content);
                (
                    remaining,
                    parsed_header.config,
                    parsed_header.overrides,
                    parsed_header.diagnostics,
                    true,
                )
            } else {
                (
                    raw_input.as_str(),
                    Configuration::default(),
                    Vec::new(),
                    Vec::new(),
                    false,
                )
            };
        let cmake_path = Path::new("CMakeLists.txt");

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            format_text(cmake_path, input, &config)
        }));

        match result {
            Err(panic_info) => {
                let msg = panic_info
                    .downcast_ref::<String>()
                    .map(|s| s.as_str())
                    .or_else(|| panic_info.downcast_ref::<&str>().copied())
                    .unwrap_or("(no message)");
                failures.push(format!("PANIC: {}\n{msg}", in_path.display()));
                count += 1;
            }
            Ok(Ok(Some(formatted))) => {
                if formatted != expected {
                    failures.push(with_config_context(
                        format!(
                            "MISMATCH: {}\n--- expected\n+++ actual\n{}",
                            in_path.display(),
                            simple_diff(&expected, &formatted)
                        ),
                        has_custom_header,
                        &header_overrides,
                        &config,
                        &header_diagnostics,
                    ));
                }
                // Idempotency: format the output again
                match format_text(cmake_path, &formatted, &config) {
                    Ok(None) => { /* Already formatted — pass */ }
                    Ok(Some(reformatted)) => {
                        if reformatted != formatted {
                            failures.push(with_config_context(
                                format!(
                                    "IDEMPOTENCY: {}\nSecond format produced different output\n{}",
                                    in_path.display(),
                                    simple_diff(&formatted, &reformatted)
                                ),
                                has_custom_header,
                                &header_overrides,
                                &config,
                                &header_diagnostics,
                            ));
                        }
                    }
                    Err(e) => {
                        failures.push(with_config_context(
                            format!(
                                "IDEMPOTENCY ERROR: {}\nSecond format failed: {e}",
                                in_path.display()
                            ),
                            has_custom_header,
                            &header_overrides,
                            &config,
                            &header_diagnostics,
                        ));
                    }
                }
            }
            Ok(Ok(None)) => {
                // No change — input already formatted; it should match expected
                if input != expected {
                    failures.push(with_config_context(
                        format!(
                            "MISMATCH (no change returned): {}\n--- expected\n+++ input\n{}",
                            in_path.display(),
                            simple_diff(&expected, input)
                        ),
                        has_custom_header,
                        &header_overrides,
                        &config,
                        &header_diagnostics,
                    ));
                }
                // Idempotency: format input again (it was already formatted)
                match format_text(cmake_path, input, &config) {
                    Ok(None) => { /* Still formatted — pass */ }
                    Ok(Some(reformatted)) => {
                        if reformatted != input {
                            failures.push(with_config_context(
                                format!(
                                    "IDEMPOTENCY: {}\nRe-formatting already-formatted input changed it\n{}",
                                    in_path.display(),
                                    simple_diff(input, &reformatted)
                                ),
                                has_custom_header,
                                &header_overrides,
                                &config,
                                &header_diagnostics,
                            ));
                        }
                    }
                    Err(e) => {
                        failures.push(with_config_context(
                            format!(
                                "IDEMPOTENCY ERROR: {}\nRe-format of unchanged input failed: {e}",
                                in_path.display()
                            ),
                            has_custom_header,
                            &header_overrides,
                            &config,
                            &header_diagnostics,
                        ));
                    }
                }
            }
            Ok(Err(e)) => {
                failures.push(format!("PARSE ERROR: {}\n{e}", in_path.display()));
                count += 1;
            }
        }
        count += 1;
    }

    if !failures.is_empty() {
        panic!(
            "{}/{count} formatter tests failed:

{}",
            failures.len(),
            failures.join("\n\n---\n\n")
        );
    }

    assert!(count > 0, "No formatter test files found");

    eprintln!("{count} formatter tests passed");
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

/// Returns true for any character with no visible glyph.
fn is_invisible(c: char) -> bool {
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

fn with_config_context(
    base_message: String,
    has_custom_header: bool,
    header_overrides: &[(String, String)],
    config: &Configuration,
    header_diagnostics: &[String],
) -> String {
    if !has_custom_header {
        return base_message;
    }

    let overrides = if header_overrides.is_empty() {
        "<none parsed>".to_string()
    } else {
        header_overrides
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let diagnostics_section = if header_diagnostics.is_empty() {
        String::new()
    } else {
        format!("\nHeader diagnostics: {}", header_diagnostics.join(" | "))
    };
    let config_section =
        format!("Header overrides: {overrides}{diagnostics_section}\nResolved config: {config:#?}");

    if let Some((head, tail)) = base_message.split_once('\n') {
        format!("{head}\n{config_section}\n{tail}")
    } else {
        format!("{base_message}\n{config_section}")
    }
}
struct ParsedConfigHeader {
    config: Configuration,
    overrides: Vec<(String, String)>,
    diagnostics: Vec<String>,
}

fn parse_config_header(header: &str) -> ParsedConfigHeader {
    let result = load_from_header(header);
    ParsedConfigHeader {
        config: result.config,
        overrides: result.overrides,
        diagnostics: result
            .diagnostics
            .into_iter()
            .map(|diagnostic| format!("{} ({})", diagnostic.message, diagnostic.key))
            .collect(),
    }
}

#[cfg(test)]
mod invisible_diff_tests {
    use super::*;

    #[test]
    fn test_strip_invisible() {
        assert_eq!(strip_invisible("hello world"), "helloworld");
        assert_eq!(strip_invisible("a\tb\nc"), "abc");
        assert_eq!(strip_invisible("a\r\nb"), "ab");
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
    fn test_simple_diff_trailing_newline() {
        // Trailing newline difference — subsumed by invisible-only detection
        let result = simple_diff("hello\n", "hello");
        assert!(
            result.contains("invisible-only differences"),
            "expected invisible-only legend for trailing newline diff, got: {result}"
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

#[cfg(test)]
mod config_header_tests {
    use super::*;

    #[test]
    fn test_parse_config_header_json_collects_overrides_and_config() {
        let parsed =
            parse_config_header(r#"{"commandCase":"upper","lineWidth":100,"sortLists":true}"#);

        assert_eq!(
            parsed.overrides,
            vec![
                ("commandCase".to_string(), "Upper".to_string()),
                ("lineWidth".to_string(), "100".to_string()),
                ("sortLists".to_string(), "true".to_string()),
            ]
        );
        assert_eq!(parsed.config.command_case, CaseStyle::Upper);
        assert_eq!(parsed.config.line_width, 100);
        assert!(parsed.config.sort_lists);
    }

    #[test]
    fn test_parse_config_header_gersemi_collects_overrides_and_config() {
        let parsed = parse_config_header(
            "{indent: tabs, line_width: 120, new_line_kind: lf, keyword_case: preserve, sort_lists: true}",
        );

        assert_eq!(
            parsed.overrides,
            vec![
                ("indent".to_string(), "tabs".to_string()),
                ("keyword_case".to_string(), "Preserve".to_string()),
                ("line_width".to_string(), "120".to_string()),
                ("new_line_kind".to_string(), "Lf".to_string()),
                ("sort_lists".to_string(), "true".to_string()),
            ]
        );
        assert!(parsed.config.use_tabs);
        assert_eq!(parsed.config.line_width, 120);
        assert_eq!(parsed.config.new_line_kind, cmakefmt::NewLineKind::Lf);
        assert_eq!(parsed.config.keyword_case, CaseStyle::Preserve);
        assert!(parsed.config.sort_lists);
    }

    #[test]
    fn test_parse_config_header_gersemi_removed_aliases_emit_diagnostics() {
        let parsed = parse_config_header("{line_length: 120, newline: lf}");

        assert_eq!(
            parsed.config.line_width,
            Configuration::default().line_width
        );
        assert_eq!(
            parsed.config.new_line_kind,
            Configuration::default().new_line_kind
        );
        assert!(parsed.overrides.is_empty());
        assert_eq!(parsed.diagnostics.len(), 2);
        assert_eq!(
            parsed.diagnostics,
            vec![
                "Unknown property in configuration (line_length)".to_string(),
                "Unknown property in configuration (newline)".to_string(),
            ]
        );
    }

    #[test]
    fn test_with_config_context_applies_only_for_custom_header() {
        let config = Configuration::default();
        let base =
            "MISMATCH: tests/formatter/sample.in.cmake\n--- expected\n+++ actual\n@@ -1 +1 @@"
                .to_string();
        let without_header = with_config_context(base.clone(), false, &[], &config, &[]);
        assert_eq!(without_header, base);

        let with_header = with_config_context(
            base.clone(),
            true,
            &[("lineWidth".to_string(), "100".to_string())],
            &config,
            &[],
        );
        assert!(with_header.contains("Header overrides: lineWidth=100"));
        assert!(with_header.contains("Resolved config:"));
        assert!(!with_header.contains("Header diagnostics:"));

        let with_diagnostics = with_config_context(
            base.clone(),
            true,
            &[("lineWidth".to_string(), "100".to_string())],
            &config,
            &["Unknown property in configuration (line_length)".to_string()],
        );
        assert!(
            with_diagnostics
                .contains("Header diagnostics: Unknown property in configuration (line_length)")
        );
        let overrides_index = with_header.find("Header overrides:").unwrap();
        let diff_index = with_header.find("--- expected").unwrap();
        assert!(overrides_index < diff_index);
        assert!(with_header.starts_with("MISMATCH: tests/formatter/sample.in.cmake\n"));
    }
}
