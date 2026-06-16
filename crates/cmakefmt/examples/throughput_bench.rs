//! Single-threaded throughput harness for `cmakefmt::format_text`.
//!
//! This is the autoresearch benchmark entrypoint. It is intentionally *not* a
//! Criterion bench: it runs a tight `std::time::Instant` loop so the measure is
//! fast and the keep/discard decision is trustworthy.
//!
//! Build/run via `./autoresearch.sh` (RELEASE profile, matches shipping lib).
//!
//! Correctness gate: every formatted output MUST stay byte-identical to the
//! committed `*.out.cmake` baseline. Any divergence exits non-zero.

use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::time::Instant;

use cmakefmt::{format_text, load_from_toml_path};

/// Untimed verification iterations: run `format_text` and diff every output
/// against the baseline so a formatting regression fails the harness loudly.
const VERIFY_ITERS: usize = 64;
/// Number of independently timed batches; the reported throughput is the median.
const BATCHES: usize = 15;
/// Timed iterations per batch. `BATCHES * ITERS_PER_BATCH` is the measured count.
const ITERS_PER_BATCH: usize = 32;

/// Render `format_text` once, normalizing `Ok(None)` (already-formatted) to the
/// input so the harness has a single concrete baseline string.
fn render(path: &Path, input: &str, config: &cmakefmt::Configuration) -> String {
    format_text(path, input, config)
        .unwrap_or_else(|err| panic!("format_text failed for {}: {err}", path.display()))
        .unwrap_or_else(|| input.to_string())
}

/// Benchmark a single fixture. Returns the median throughput in MB/s.
fn bench_fixture(in_path: &Path) -> f64 {
    let expected_path = PathBuf::from(
        in_path
            .to_str()
            .expect("fixture path is valid UTF-8")
            .replace(".in.cmake", ".out.cmake"),
    );

    let input = std::fs::read_to_string(in_path)
        .unwrap_or_else(|err| panic!("failed reading {}: {err}", in_path.display()));
    let expected = std::fs::read_to_string(&expected_path)
        .unwrap_or_else(|err| panic!("failed reading {}: {err}", expected_path.display()));
    let config = load_from_toml_path(in_path).config;

    // Baseline captured once; must match the committed canonical output.
    let baseline = render(in_path, &input, &config);
    if baseline != expected {
        eprintln!(
            "OUTPUT PARITY FAILURE: {} output does not match committed {}",
            in_path.display(),
            expected_path.display()
        );
        std::process::exit(1);
    }

    // Untimed: diff every iteration against the baseline.
    for _ in 0..VERIFY_ITERS {
        let out = render(in_path, black_box(&input), black_box(&config));
        if out != baseline {
            eprintln!(
                "OUTPUT PARITY FAILURE: {} produced non-deterministic / changed output",
                in_path.display()
            );
            std::process::exit(1);
        }
    }

    let input_bytes = input.len() as u64;
    let mut throughputs: Vec<f64> = Vec::with_capacity(BATCHES);
    for _ in 0..BATCHES {
        let start = Instant::now();
        for _ in 0..ITERS_PER_BATCH {
            let out = format_text(in_path, black_box(&input), black_box(&config))
                .expect("format_text must succeed");
            black_box(out);
        }
        let elapsed = start.elapsed().as_secs_f64();
        let bytes = input_bytes * ITERS_PER_BATCH as u64;
        throughputs.push(bytes as f64 / elapsed / 1e6);
    }

    throughputs.sort_by(|a, b| a.partial_cmp(b).expect("throughput is finite"));
    throughputs[throughputs.len() / 2]
}

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Primary: the large real-world XNNPACK fixture (matches the Criterion bench).
    let xnnpack = manifest.join("tests/formatter/respositories/XNNPACK/CMakeLists.in.cmake");
    let primary = bench_fixture(&xnnpack);

    // Secondary: synthetic fixture (non-default config) for extra signal.
    let synthetic = manifest.join("tests/formatter/respositories/synthetic/CMakeLists.in.cmake");
    let secondary = bench_fixture(&synthetic);

    println!("METRIC throughput_mb_s={primary:.4}");
    println!("METRIC throughput_synthetic_mb_s={secondary:.4}");
}
