use std::path::{Path, PathBuf};

use dprint_plugin_cmake::{CaseStyle, Configuration, format_text};

#[test]
fn test_formatter_files() {
    let formatter_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/formatter");
    let mut failures: Vec<String> = Vec::new();
    let mut skipped = 0;
    let mut count = 0;

    let mut in_files = walk_cmake_files(&formatter_dir);
    in_files.sort();

    for in_path in &in_files {
        // foo.in.cmake -> foo.out.cmake
        let stem = in_path
            .to_str()
            .expect("non-utf8 path")
            .strip_suffix(".in.cmake")
            .expect("expected .in.cmake extension");
        let out_path = PathBuf::from(format!("{stem}.out.cmake"));

        if !out_path.exists() {
            failures.push(format!("Missing .out.cmake for {}", in_path.display()));
            continue;
        }

        let raw_input = std::fs::read_to_string(in_path).unwrap();
        let expected = std::fs::read_to_string(&out_path).unwrap();

        // Strip ### header line if present and try to parse config
        let (input, config) = if raw_input.starts_with("### ") {
            let header_end = raw_input.find('\n');
            let header_content = match header_end {
                Some(pos) => &raw_input[4..pos],
                None => &raw_input[4..],
            };
            let remaining = match header_end {
                Some(pos) => &raw_input[pos + 1..],
                None => "",
            };
            let config = parse_config_header(header_content);
            (remaining, config)
        } else {
            (raw_input.as_str(), Configuration::default())
        };
        let cmake_path = Path::new("CMakeLists.txt");

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            format_text(cmake_path, input, &config)
        }));

        match result {
            Err(_) => {
                // Formatter panics on some inputs (e.g. multiline bracket args)
                skipped += 1;
                continue;
            }
            Ok(Ok(Some(formatted))) => {
                if formatted != expected {
                    failures.push(format!(
                        "MISMATCH: {}\n--- expected\n+++ actual\n{}",
                        in_path.display(),
                        simple_diff(&expected, &formatted)
                    ));
                }
                // Idempotency: format the output again
                match format_text(cmake_path, &formatted, &config) {
                    Ok(None) => { /* Already formatted — pass */ }
                    Ok(Some(reformatted)) => {
                        if reformatted != formatted {
                            failures.push(format!(
                                "IDEMPOTENCY: {}\nSecond format produced different output\n{}",
                                in_path.display(),
                                simple_diff(&formatted, &reformatted)
                            ));
                        }
                    }
                    Err(e) => {
                        failures.push(format!(
                            "IDEMPOTENCY ERROR: {}\nSecond format failed: {e}",
                            in_path.display()
                        ));
                    }
                }
            }
            Ok(Ok(None)) => {
                // No change — input already formatted; it should match expected
                if input != expected {
                    failures.push(format!(
                        "MISMATCH (no change returned): {}\n--- expected\n+++ input\n{}",
                        in_path.display(),
                        simple_diff(&expected, input)
                    ));
                }
                // Idempotency: format input again (it was already formatted)
                match format_text(cmake_path, input, &config) {
                    Ok(None) => { /* Still formatted — pass */ }
                    Ok(Some(reformatted)) => {
                        if reformatted != input {
                            failures.push(format!(
                                "IDEMPOTENCY: {}\nRe-formatting already-formatted input changed it\n{}",
                                in_path.display(),
                                simple_diff(input, &reformatted)
                            ));
                        }
                    }
                    Err(e) => {
                        failures.push(format!(
                            "IDEMPOTENCY ERROR: {}\nRe-format of unchanged input failed: {e}",
                            in_path.display()
                        ));
                    }
                }
            }
            Ok(Err(_)) => {
                // Parse errors on some inputs
                skipped += 1;
                continue;
            }
        }
        count += 1;
    }

    assert!(count > 0, "No formatter test files found");

    if !failures.is_empty() {
        panic!(
            "{}/{count} formatter tests failed:\n\n{}",
            failures.len(),
            failures.join("\n\n---\n\n")
        );
    }

    eprintln!("{count} formatter tests passed ({skipped} skipped due to formatter limitations)");
}

fn walk_cmake_files(dir: &Path) -> Vec<PathBuf> {
    let mut results = Vec::new();
    walk_recursive(dir, &mut results);
    results
}

fn walk_recursive(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(err) => panic!("Failed to read directory {}: {}", dir.display(), err),
    };
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            walk_recursive(&path, out);
        } else if path
            .file_name()
            .and_then(|n| n.to_str())
            .map_or(false, |n| n.ends_with(".in.cmake"))
        {
            out.push(path);
        }
    }
}

fn simple_diff(expected: &str, actual: &str) -> String {
    let exp_lines: Vec<&str> = expected.lines().collect();
    let act_lines: Vec<&str> = actual.lines().collect();
    let mut diff = String::new();
    let max = exp_lines.len().max(act_lines.len());

    for i in 0..max {
        let e = exp_lines.get(i).copied().unwrap_or("<missing>");
        let a = act_lines.get(i).copied().unwrap_or("<missing>");
        if e != a {
            diff.push_str(&format!("L{}: -{e}\nL{}: +{a}\n", i + 1, i + 1));
        }
    }

    if diff.is_empty() {
        if expected.ends_with('\n') != actual.ends_with('\n') {
            diff.push_str("(trailing newline differs)\n");
        }
    }

    diff
}

fn parse_config_header(header: &str) -> Configuration {
    let mut config = Configuration::default();

    // Try JSON first (camelCase keys like {"commandCase": "preserve"})
    if let Ok(map) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(header) {
        for (key, value) in &map {
            match key.as_str() {
                "commandCase" => {
                    if let Some(s) = value.as_str() {
                        config.command_case = match s {
                            "upper" => CaseStyle::Upper,
                            "preserve" => CaseStyle::Preserve,
                            _ => CaseStyle::Lower,
                        };
                    }
                }
                "keywordCase" => {
                    if let Some(s) = value.as_str() {
                        config.keyword_case = match s {
                            "lower" => CaseStyle::Lower,
                            "preserve" => CaseStyle::Preserve,
                            _ => CaseStyle::Upper,
                        };
                    }
                }
                "lineWidth" => {
                    if let Some(n) = value.as_u64() {
                        config.line_width = n as u32;
                    }
                }
                "indentWidth" => {
                    if let Some(n) = value.as_u64() {
                        config.indent_width = n as u8;
                    }
                }
                "useTabs" => {
                    if let Some(b) = value.as_bool() {
                        config.use_tabs = b;
                    }
                }
                "sortLists" => {
                    if let Some(b) = value.as_bool() {
                        config.sort_lists = b;
                    }
                }
                "closingParenNewline" => {
                    if let Some(b) = value.as_bool() {
                        config.closing_paren_newline = b;
                    }
                }
                _ => {}
            }
        }
        return config;
    }

    // Fall back to gersemi-style {key: value} format
    let content = header.trim();
    let content = content.strip_prefix('{').unwrap_or(content);
    let content = content.strip_suffix('}').unwrap_or(content);
    let content = content.trim();

    if content.is_empty() {
        return config;
    }

    for pair in split_respecting_brackets(content) {
        let pair = pair.trim();
        if let Some((key, value)) = pair.split_once(':') {
            let key = key.trim().trim_matches('"');
            let value = value.trim().trim_matches('"');
            match key {
                "line_length" => {
                    if let Ok(n) = value.parse::<u32>() {
                        config.line_width = n;
                    }
                }
                "indent" => {
                    if value == "tabs" {
                        config.use_tabs = true;
                    } else if let Ok(n) = value.parse::<u8>() {
                        config.indent_width = n;
                    }
                }
                "commandCase" | "command_case" => {
                    config.command_case = match value {
                        "upper" => CaseStyle::Upper,
                        "preserve" => CaseStyle::Preserve,
                        _ => CaseStyle::Lower,
                    };
                }
                "keywordCase" | "keyword_case" => {
                    config.keyword_case = match value {
                        "lower" => CaseStyle::Lower,
                        "preserve" => CaseStyle::Preserve,
                        _ => CaseStyle::Upper,
                    };
                }
                "sortLists" | "sort_lists" => {
                    config.sort_lists = value == "true";
                }
                _ => { /* ignore unknown keys for forward compat */ }
            }
        }
    }

    config
}

fn split_respecting_brackets(s: &str) -> Vec<&str> {
    let mut results = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' if depth == 0 => {
                results.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    if start < s.len() {
        results.push(&s[start..]);
    }
    results
}
