use std::io::Write;
use std::process::Command;

use tempfile::NamedTempFile;

fn cmakefmt_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_cmakefmt"))
}

// ---------------------------------------------------------------------------
// stdin mode
// ---------------------------------------------------------------------------

#[test]
fn stdin_formats_output() {
    let mut cmd = cmakefmt_bin();
    cmd.arg("--stdin");
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn cmakefmt");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"SET(A 1)\n")
        .unwrap();

    let output = child.wait_with_output().expect("failed to wait");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.starts_with("set("),
        "expected lowercase 'set(', got: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// check mode
// ---------------------------------------------------------------------------

#[test]
fn check_already_formatted_exits_0() {
    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "set(A 1)\n").unwrap();

    let output = cmakefmt_bin()
        .arg("--check")
        .arg(tmp.path())
        .output()
        .expect("failed to run");

    assert!(
        output.status.success(),
        "expected exit 0 for formatted file, got: {}",
        output.status
    );
}

#[test]
fn check_unformatted_exits_nonzero() {
    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "SET(A 1)\n").unwrap();

    let output = cmakefmt_bin()
        .arg("--check")
        .arg(tmp.path())
        .output()
        .expect("failed to run");

    assert!(
        !output.status.success(),
        "expected non-zero exit for unformatted file"
    );
}

// ---------------------------------------------------------------------------
// diff mode
// ---------------------------------------------------------------------------

#[test]
fn diff_shows_changes() {
    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "SET(A 1)\n").unwrap();

    let output = cmakefmt_bin()
        .arg("--diff")
        .arg("--no-color")
        .arg(tmp.path())
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // diff output should contain the change from SET to set
    assert!(
        stdout.contains("set(") || stdout.contains("SET("),
        "expected diff output, got: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// write mode
// ---------------------------------------------------------------------------

#[test]
fn write_modifies_file_in_place() {
    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "SET(A 1)\n").unwrap();
    let path = tmp.path().to_path_buf();

    let output = cmakefmt_bin()
        .arg("--write")
        .arg(&path)
        .output()
        .expect("failed to run");

    assert!(output.status.success());

    let content = std::fs::read_to_string(&path).unwrap();
    assert!(
        content.starts_with("set("),
        "file should be modified in place, got: {content}"
    );
}

// ---------------------------------------------------------------------------
// config overrides
// ---------------------------------------------------------------------------

#[test]
fn line_width_override() {
    let mut cmd = cmakefmt_bin();
    cmd.arg("--stdin").arg("--line-width").arg("40");
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());

    let input = "target_link_libraries(mylib PUBLIC dep1 dep2 dep3 dep4 dep5)\n";
    let mut child = cmd.spawn().expect("failed to spawn");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().expect("failed to wait");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // With line width 40, the long command should be wrapped
    assert!(
        stdout.contains('\n'),
        "expected wrapping with --line-width 40"
    );
}

// ---------------------------------------------------------------------------
// non-existent file
// ---------------------------------------------------------------------------

#[test]
fn nonexistent_file_errors() {
    let output = cmakefmt_bin()
        .arg("/nonexistent/file.cmake")
        .output()
        .expect("failed to run");

    assert!(
        !output.status.success(),
        "expected non-zero exit for missing file"
    );
}

// ---------------------------------------------------------------------------
// print-config
// ---------------------------------------------------------------------------

#[test]
fn print_config_outputs_toml() {
    let output = cmakefmt_bin()
        .arg("--print-config")
        .arg("--stdin")
        .output()
        .expect("failed to run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("lineWidth") || stdout.contains("line_width"),
        "expected TOML config output, got: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// config file errors
// ---------------------------------------------------------------------------

#[test]
fn nonexistent_config_file_produces_stderr() {
    let mut cmd = cmakefmt_bin();
    cmd.arg("--config")
        .arg("/nonexistent/path/.cmakefmt.toml")
        .arg("--stdin");
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"set(A 1)\n")
        .unwrap();

    let output = child.wait_with_output().expect("failed to wait");
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Should either error or at least produce a warning on stderr
    assert!(
        !output.status.success() || !stderr.is_empty(),
        "expected error or warning for missing config file, got exit={} stderr={:?}",
        output.status,
        stderr
    );
}

// ---------------------------------------------------------------------------
// multiple CLI config overrides
// ---------------------------------------------------------------------------

#[test]
fn indent_width_override() {
    let mut cmd = cmakefmt_bin();
    cmd.arg("--stdin").arg("--indent-width").arg("8");
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());

    let input = "if(X)\nset(A 1)\nendif()\n";
    let mut child = cmd.spawn().expect("failed to spawn");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().expect("failed to wait");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // With indent width 8, the nested set should have 8 spaces of indentation
    assert!(
        stdout.contains("        set("),
        "expected 8-space indent, got: {stdout}"
    );
}

#[test]
fn command_case_upper_override() {
    let mut cmd = cmakefmt_bin();
    cmd.arg("--stdin").arg("--command-case").arg("upper");
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());

    let input = "set(A 1)\nmessage(STATUS \"hello\")\n";
    let mut child = cmd.spawn().expect("failed to spawn");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().expect("failed to wait");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("SET(") && stdout.contains("MESSAGE("),
        "expected uppercase commands, got: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// check mode with multiple files
// ---------------------------------------------------------------------------

#[test]
fn check_multiple_files_reports_all_unformatted() {
    let mut tmp1 = NamedTempFile::new().unwrap();
    write!(tmp1, "SET(A 1)\n").unwrap();

    let mut tmp2 = NamedTempFile::new().unwrap();
    write!(tmp2, "SET(B 2)\n").unwrap();

    let output = cmakefmt_bin()
        .arg("--check")
        .arg(tmp1.path())
        .arg(tmp2.path())
        .output()
        .expect("failed to run");

    assert!(
        !output.status.success(),
        "expected non-zero exit when multiple files need formatting"
    );
}

#[test]
fn check_multiple_files_all_formatted_exits_0() {
    let mut tmp1 = NamedTempFile::new().unwrap();
    write!(tmp1, "set(A 1)\n").unwrap();

    let mut tmp2 = NamedTempFile::new().unwrap();
    write!(tmp2, "set(B 2)\n").unwrap();

    let output = cmakefmt_bin()
        .arg("--check")
        .arg(tmp1.path())
        .arg(tmp2.path())
        .output()
        .expect("failed to run");

    assert!(
        output.status.success(),
        "expected exit 0 when all files are formatted, got: {}",
        output.status
    );
}

// ---------------------------------------------------------------------------
// stdin with empty input
// ---------------------------------------------------------------------------

#[test]
fn stdin_empty_input() {
    let mut cmd = cmakefmt_bin();
    cmd.arg("--stdin");
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn");
    child.stdin.take().unwrap().write_all(b"").unwrap();

    let output = child.wait_with_output().expect("failed to wait");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Empty input should produce empty or minimal output
    assert!(
        stdout.is_empty() || stdout == "\n",
        "expected empty or single newline for empty input, got: {:?}",
        stdout
    );
}

// ---------------------------------------------------------------------------
// diff mode on already formatted file
// ---------------------------------------------------------------------------

#[test]
fn diff_already_formatted_shows_no_diff() {
    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "set(A 1)\n").unwrap();

    let output = cmakefmt_bin()
        .arg("--diff")
        .arg("--no-color")
        .arg(tmp.path())
        .output()
        .expect("failed to run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // No diff should be empty or contain no changes
    assert!(
        !stdout.contains("---") && !stdout.contains("+++"),
        "expected no diff for already formatted file, got: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// write mode on already formatted file doesn't corrupt
// ---------------------------------------------------------------------------

#[test]
fn write_already_formatted_preserves_content() {
    let mut tmp = NamedTempFile::new().unwrap();
    let original = "set(A 1)\n";
    write!(tmp, "{original}").unwrap();
    let path = tmp.path().to_path_buf();

    let output = cmakefmt_bin()
        .arg("--write")
        .arg(&path)
        .output()
        .expect("failed to run");

    assert!(output.status.success());
    let content = std::fs::read_to_string(&path).unwrap();
    assert_eq!(
        content, original,
        "write mode should preserve already-formatted content"
    );
}
