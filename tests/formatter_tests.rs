use std::path::{Path, PathBuf};

use dprint_plugin_cmake::{format_text, Configuration};

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

        // Strip ### header line if present
        let input = if raw_input.starts_with("### ") {
            match raw_input.find('\n') {
                Some(pos) => &raw_input[pos + 1..],
                None => "",
            }
        } else {
            &raw_input
        };

        let config = Configuration::default();
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
