use std::hint::black_box;
use std::path::PathBuf;

use cmakefmt::{format_text, load_from_toml_path};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

fn formatter_xnnpack_in_bench(c: &mut Criterion) {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/formatter/respositories/XNNPACK/CMakeLists.in.cmake");
    let input = std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|err| panic!("failed reading {}: {err}", fixture_path.display()));
    let config = load_from_toml_path(&fixture_path).config;

    let mut group = c.benchmark_group("formatter_fixtures");
    group.throughput(Throughput::Bytes(input.len() as u64));
    group.bench_function(
        BenchmarkId::new("fixture", "respositories__xnnpack__cmakelists__in"),
        move |b| {
            b.iter(|| {
                format_text(
                    fixture_path.as_path(),
                    black_box(&input),
                    black_box(&config),
                )
                .expect("benchmark formatter invocation must succeed");
            });
        },
    );
    group.finish();
}

fn configured_criterion() -> Criterion {
    Criterion::default().without_plots()
}

criterion_group! {
    name = benches;
    config = configured_criterion();
    targets = formatter_xnnpack_in_bench
}
criterion_main!(benches);
