//! Autoresearch throughput harness.
//!
//! Formats the full fixture corpus under `tests/formatter/` once per pass,
//! with the XNNPACK fixture repeated `XNNPACK_TOTAL_REPEATS` times so it
//! dominates the byte weight. Reports the median per-pass throughput.
//!
//! Run via `cargo bench --bench autoresearch_bench` (bench profile inherits
//! release codegen). Output contract: `METRIC <name>=<value>` lines on stdout.

use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::time::Instant;

use cmakefmt::{Configuration, format_text, load_from_toml_path};

const XNNPACK_TOTAL_REPEATS: u64 = 5;
const WARMUP_PASSES: usize = 5;
const TIMED_PASSES: usize = 50;

struct Fixture {
    path: PathBuf,
    input: String,
    config: Configuration,
}

fn collect_inputs(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("read_dir failed") {
        let path = entry.expect("dir entry failed").path();
        if path.is_dir() {
            collect_inputs(&path, out);
        } else if path.to_str().is_some_and(|p| p.ends_with(".in.cmake")) {
            out.push(path);
        }
    }
}

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(|a, b| a.partial_cmp(b).expect("non-finite timing"));
    let n = values.len();
    assert!(n > 0, "no timed passes recorded");
    if n % 2 == 1 {
        values[n / 2]
    } else {
        (values[n / 2 - 1] + values[n / 2]) / 2.0
    }
}

fn run_fixture(fixture: &Fixture) {
    let result = format_text(
        fixture.path.as_path(),
        black_box(&fixture.input),
        black_box(&fixture.config),
    )
    .unwrap_or_else(|e| panic!("format failed for {}: {e}", fixture.path.display()));
    black_box(result);
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/formatter");
    let mut paths = Vec::new();
    collect_inputs(&root, &mut paths);
    paths.sort();
    assert!(
        !paths.is_empty(),
        "no fixtures found under {}",
        root.display()
    );

    // Load inputs and per-fixture configs up front; only format_text is timed.
    let fixtures: Vec<Fixture> = paths
        .into_iter()
        .map(|path| {
            let input = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("failed reading {}: {e}", path.display()));
            let config = load_from_toml_path(&path).config;
            Fixture {
                path,
                input,
                config,
            }
        })
        .collect();

    let xnnpack_idx = fixtures
        .iter()
        .position(|f| {
            f.path
                .ends_with("respositories/XNNPACK/CMakeLists.in.cmake")
        })
        .expect("XNNPACK fixture missing");

    let xnnpack_bytes = fixtures[xnnpack_idx].input.len() as u64;
    let rest_bytes: u64 = fixtures
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != xnnpack_idx)
        .map(|(_, f)| f.input.len() as u64)
        .sum();
    let xnnpack_pass_bytes = XNNPACK_TOTAL_REPEATS * xnnpack_bytes;
    let pass_bytes = rest_bytes + xnnpack_pass_bytes;

    let mut rest_secs = Vec::with_capacity(TIMED_PASSES);
    let mut xnnpack_secs = Vec::with_capacity(TIMED_PASSES);

    for pass in 0..(WARMUP_PASSES + TIMED_PASSES) {
        let t = Instant::now();
        for (i, fixture) in fixtures.iter().enumerate() {
            if i != xnnpack_idx {
                run_fixture(fixture);
            }
        }
        let rest = t.elapsed().as_secs_f64();

        let t = Instant::now();
        for _ in 0..XNNPACK_TOTAL_REPEATS {
            run_fixture(&fixtures[xnnpack_idx]);
        }
        let xnn = t.elapsed().as_secs_f64();

        if pass >= WARMUP_PASSES {
            rest_secs.push(rest);
            xnnpack_secs.push(xnn);
        }
    }

    let pass_secs: Vec<f64> = rest_secs
        .iter()
        .zip(&xnnpack_secs)
        .map(|(a, b)| a + b)
        .collect();

    let throughput = pass_bytes as f64 / median(pass_secs);
    let xnnpack_throughput = xnnpack_pass_bytes as f64 / median(xnnpack_secs);
    let rest_throughput = rest_bytes as f64 / median(rest_secs);

    println!(
        "fixtures={} pass_bytes={} xnnpack_weight={:.1}% timed_passes={}",
        fixtures.len(),
        pass_bytes,
        100.0 * xnnpack_pass_bytes as f64 / pass_bytes as f64,
        TIMED_PASSES,
    );
    println!("METRIC throughput_bytes_per_sec={throughput:.0}");
    println!("METRIC xnnpack_throughput_bytes_per_sec={xnnpack_throughput:.0}");
    println!("METRIC small_fixture_throughput_bytes_per_sec={rest_throughput:.0}");
}
