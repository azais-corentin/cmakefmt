use std::path::Path;

fn main() {
    // Ensure the test binary recompiles when fixture files change.
    // rstest's #[files] globs at compile time; Cargo doesn't track those
    // paths automatically.
    let fixture_dir = Path::new("tests/formatter");
    if fixture_dir.is_dir() {
        rerun_if_changed_recursive(fixture_dir);
    }
}

fn rerun_if_changed_recursive(dir: &Path) {
    println!("cargo:rerun-if-changed={}", dir.display());
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rerun_if_changed_recursive(&path);
        } else if path.extension().is_some_and(|ext| ext == "cmake") {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}
