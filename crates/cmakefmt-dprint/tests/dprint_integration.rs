//! Integration tests that exercise the cmakefmt dprint WASM plugin end-to-end.
//!
//! These tests require two external prerequisites:
//! - The WASM plugin binary built via `mise run build:debug:wasm`
//! - The `dprint` CLI on `$PATH`
//!
//! When either prerequisite is missing the tests skip gracefully (pass with a
//! diagnostic message) rather than fail, so `cargo test` always succeeds.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

// ── Prerequisites ────────────────────────────────────────────────────────────

/// Absolute path to the debug WASM plugin binary.
fn wasm_plugin_path() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .join("../../target/wasm32-unknown-unknown/debug/cmakefmt_dprint.wasm")
        .canonicalize()
        .unwrap_or_else(|_| {
            // Return the non-canonical path so the skip message is still useful.
            manifest_dir.join("../../target/wasm32-unknown-unknown/debug/cmakefmt_dprint.wasm")
        })
}

fn dprint_available() -> bool {
    Command::new("dprint")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
}

/// Returns `true` when all prerequisites are met. Prints a skip reason and
/// returns `false` otherwise.
fn prerequisites_met() -> bool {
    let wasm = wasm_plugin_path();
    if !wasm.exists() {
        eprintln!(
            "SKIP: WASM plugin not found at {}\n\
             Build it first: mise run build:debug:wasm",
            wasm.display()
        );
        return false;
    }
    if !dprint_available() {
        eprintln!("SKIP: `dprint` CLI not found on PATH");
        return false;
    }
    true
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Create a temp directory containing a `dprint.json` and one or more CMake
/// files. Returns the temp dir handle (directory is deleted on drop).
fn create_dprint_project(config_json: &serde_json::Value, files: &[(&str, &str)]) -> TempDir {
    let dir = TempDir::new().expect("failed to create temp dir");
    let config_path = dir.path().join("dprint.json");
    fs::write(&config_path, config_json.to_string()).expect("failed to write dprint.json");
    for &(name, content) in files {
        let file_path = dir.path().join(name);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("failed to create parent dirs");
        }
        fs::write(&file_path, content).expect("failed to write test file");
    }
    dir
}

fn dprint_config(wasm_path: &Path, extra: serde_json::Value) -> serde_json::Value {
    let mut config = serde_json::json!({
        "cmake": {},
        "plugins": [wasm_path.to_string_lossy()],
    });
    if let (Some(base), Some(extra)) = (config.as_object_mut(), extra.as_object()) {
        for (k, v) in extra {
            base.insert(k.clone(), v.clone());
        }
    }
    config
}

fn run_dprint(args: &[&str], dir: &Path) -> std::process::Output {
    Command::new("dprint")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to execute dprint")
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn test_dprint_formats_cmake_file() {
    if !prerequisites_met() {
        return;
    }

    let wasm = wasm_plugin_path();
    let config = dprint_config(&wasm, serde_json::Value::Null);

    let input = "set(FOO    \"bar\"    \"baz\")\n";
    let expected = "set(FOO \"bar\" \"baz\")\n";

    let dir = create_dprint_project(&config, &[("CMakeLists.txt", input)]);
    let cmake_path = dir.path().join("CMakeLists.txt");

    // Format the file.
    let output = run_dprint(&["fmt", "--config", "dprint.json"], dir.path());
    assert!(
        output.status.success(),
        "dprint fmt failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let formatted = fs::read_to_string(&cmake_path).expect("failed to read formatted file");
    assert_eq!(formatted, expected, "formatted output mismatch");

    // Idempotency: `dprint check` should report no changes.
    let check = run_dprint(&["check", "--config", "dprint.json"], dir.path());
    assert!(
        check.status.success(),
        "dprint check failed after format (not idempotent):\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&check.stdout),
        String::from_utf8_lossy(&check.stderr),
    );
}

#[test]
fn test_dprint_check_already_formatted() {
    if !prerequisites_met() {
        return;
    }

    let wasm = wasm_plugin_path();
    let config = dprint_config(&wasm, serde_json::Value::Null);

    let input = "set(FOO \"bar\" \"baz\")\n";
    let dir = create_dprint_project(&config, &[("CMakeLists.txt", input)]);

    let check = run_dprint(&["check", "--config", "dprint.json"], dir.path());
    assert!(
        check.status.success(),
        "dprint check should pass for already-formatted file:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&check.stdout),
        String::from_utf8_lossy(&check.stderr),
    );
}

#[test]
fn test_dprint_config_passthrough() {
    if !prerequisites_met() {
        return;
    }

    let wasm = wasm_plugin_path();
    // Set a very narrow lineWidth so wrapping kicks in.
    let config = dprint_config(&wasm, serde_json::json!({ "lineWidth": 20 }));

    // This line is 38 chars — well over the 20-char limit — so it must wrap.
    let input = "set(MY_VAR \"hello\" \"world\" \"!\")\n";
    let dir = create_dprint_project(&config, &[("CMakeLists.txt", input)]);

    let output = run_dprint(&["fmt", "--config", "dprint.json"], dir.path());
    assert!(
        output.status.success(),
        "dprint fmt failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let formatted =
        fs::read_to_string(dir.path().join("CMakeLists.txt")).expect("failed to read file");

    // With lineWidth=20, the arguments must have been wrapped onto separate
    // lines — the output should contain newlines beyond the final one.
    let interior_newlines = formatted.trim_end().matches('\n').count();
    assert!(
        interior_newlines >= 1,
        "expected wrapping with lineWidth=20 but got single-line output: {formatted:?}",
    );

    // Idempotency check.
    let check = run_dprint(&["check", "--config", "dprint.json"], dir.path());
    assert!(
        check.status.success(),
        "dprint check failed after format (not idempotent):\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&check.stdout),
        String::from_utf8_lossy(&check.stderr),
    );
}
