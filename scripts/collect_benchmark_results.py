#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import os
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


NANOSECONDS_PER_SECOND = 1_000_000_000.0
BYTES_PER_DECIMAL_GIGABYTE = 1_000_000_000.0
TARGET_GROUP_ID = "formatter_fixtures"


@dataclass(frozen=True)
class CriterionBenchmark:
    benchmark_id: str
    group_id: str
    function_id: str | None
    value_str: str | None
    directory_name: str
    throughput_bytes: int


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Collect Criterion benchmark outputs into a single per-commit JSON entry.",
    )
    parser.add_argument(
        "--criterion-dir",
        default="target/criterion",
        help="Criterion output directory (default: target/criterion)",
    )
    parser.add_argument(
        "--output",
        required=True,
        help="Output path for the normalized benchmark entry JSON.",
    )
    parser.add_argument(
        "--commit-sha",
        default=os.getenv("GITHUB_SHA", "unknown"),
        help="Commit SHA for this benchmark run.",
    )
    parser.add_argument(
        "--commit-timestamp",
        default=None,
        help="Commit timestamp in ISO-8601. Defaults to current UTC time.",
    )
    parser.add_argument(
        "--runner",
        default=os.getenv("RUNNER_NAME") or os.getenv("RUNNER_OS") or "unknown",
        help="Runner identifier.",
    )
    parser.add_argument(
        "--rust-version",
        default=os.getenv("RUST_VERSION", "unknown"),
        help="Rust toolchain version used for the benchmark run.",
    )
    return parser.parse_args()


def parse_throughput_bytes(raw_throughput: Any, source_path: Path) -> int:
    if raw_throughput is None:
        raise ValueError(
            f"benchmark throughput is missing in {source_path}; bench harness must set Throughput::Bytes(...)"
        )

    # Criterion serializes Throughput::Bytes in benchmark.json. Accept both known serde forms
    # to keep this collector robust across criterion/cargo-criterion output differences.
    if isinstance(raw_throughput, dict):
        if "Bytes" in raw_throughput:
            return int(raw_throughput["Bytes"])
        if "bytes" in raw_throughput:
            return int(raw_throughput["bytes"])

    if (
        isinstance(raw_throughput, list)
        and len(raw_throughput) == 2
        and str(raw_throughput[0]).lower() == "bytes"
    ):
        return int(raw_throughput[1])

    raise ValueError(
        f"unsupported Criterion throughput payload in {source_path}: {raw_throughput!r}"
    )


def parse_criterion_benchmark(
    benchmark_meta: dict[str, Any], source_path: Path
) -> CriterionBenchmark:
    benchmark_id = benchmark_meta.get("full_id") or benchmark_meta.get("title")
    if not isinstance(benchmark_id, str) or not benchmark_id:
        raise ValueError(f"missing benchmark full_id/title in {source_path}")

    group_id = benchmark_meta.get("group_id")
    if not isinstance(group_id, str) or not group_id:
        raise ValueError(f"missing benchmark group_id in {source_path}")

    function_id = benchmark_meta.get("function_id")
    if function_id is not None and not isinstance(function_id, str):
        raise ValueError(f"invalid function_id in {source_path}")

    value_str = benchmark_meta.get("value_str")
    if value_str is not None and not isinstance(value_str, str):
        raise ValueError(f"invalid value_str in {source_path}")

    directory_name = benchmark_meta.get("directory_name")
    if not isinstance(directory_name, str) or not directory_name:
        raise ValueError(f"missing benchmark directory_name in {source_path}")

    throughput_bytes = parse_throughput_bytes(
        benchmark_meta.get("throughput"), source_path
    )

    return CriterionBenchmark(
        benchmark_id=benchmark_id,
        group_id=group_id,
        function_id=function_id,
        value_str=value_str,
        directory_name=directory_name,
        throughput_bytes=throughput_bytes,
    )


def load_criterion_results(
    criterion_dir: Path,
) -> dict[str, tuple[CriterionBenchmark, dict[str, Any]]]:
    results_by_benchmark_id: dict[str, tuple[CriterionBenchmark, dict[str, Any]]] = {}

    for benchmark_json in sorted(criterion_dir.glob("**/new/benchmark.json")):
        estimates_path = benchmark_json.with_name("estimates.json")
        if not estimates_path.exists():
            continue

        with benchmark_json.open("r", encoding="utf-8") as handle:
            benchmark_meta = json.load(handle)

        if benchmark_meta.get("group_id") != TARGET_GROUP_ID:
            # Ignore stale benchmark runs from old harness IDs under the same criterion directory.
            continue

        benchmark = parse_criterion_benchmark(benchmark_meta, benchmark_json)
        with estimates_path.open("r", encoding="utf-8") as handle:
            estimates = json.load(handle)

        if benchmark.benchmark_id in results_by_benchmark_id:
            raise ValueError(
                f"duplicate benchmark id in criterion output: {benchmark.benchmark_id}"
            )

        results_by_benchmark_id[benchmark.benchmark_id] = (benchmark, estimates)

    if not results_by_benchmark_id:
        raise ValueError(
            f"no {TARGET_GROUP_ID} benchmark estimate files found under {criterion_dir}"
        )

    return results_by_benchmark_id


def ns_to_seconds(value_ns: Any) -> float:
    return float(value_ns) / NANOSECONDS_PER_SECOND


def parse_fixture_identity(benchmark: CriterionBenchmark) -> tuple[str, str, str]:
    source = benchmark.value_str or benchmark.directory_name or benchmark.benchmark_id
    lowered = source.lower()

    parts = lowered.split("__")
    if len(parts) >= 2:
        fixture_stem = "__".join(parts[:-1])
        variant = parts[-1]
        return fixture_stem, variant, source

    return lowered, "default", source


def build_fixture_entry(
    benchmark: CriterionBenchmark, estimates: dict[str, Any]
) -> dict[str, Any]:
    mean = estimates["mean"]
    median = estimates["median"]

    mean_seconds = ns_to_seconds(mean["point_estimate"])
    median_seconds = ns_to_seconds(median["point_estimate"])
    mean_ci = mean["confidence_interval"]
    median_ci = median["confidence_interval"]

    fixture_stem, variant, path = parse_fixture_identity(benchmark)
    throughput_gb_per_s = (
        float(benchmark.throughput_bytes) / mean_seconds / BYTES_PER_DECIMAL_GIGABYTE
        if mean_seconds > 0.0
        else 0.0
    )

    return {
        "benchmark_id": benchmark.benchmark_id,
        "group_id": benchmark.group_id,
        "function_id": benchmark.function_id,
        "value_str": benchmark.value_str,
        "directory_name": benchmark.directory_name,
        "fixture_stem": fixture_stem,
        "variant": variant,
        "path": path,
        "input_bytes": benchmark.throughput_bytes,
        "throughput_bytes_per_iteration": benchmark.throughput_bytes,
        "mean_seconds": mean_seconds,
        "median_seconds": median_seconds,
        "mean_confidence_interval_lower_seconds": ns_to_seconds(mean_ci["lower_bound"]),
        "mean_confidence_interval_upper_seconds": ns_to_seconds(mean_ci["upper_bound"]),
        "median_confidence_interval_lower_seconds": ns_to_seconds(
            median_ci["lower_bound"]
        ),
        "median_confidence_interval_upper_seconds": ns_to_seconds(
            median_ci["upper_bound"]
        ),
        "throughput_gb_per_s": throughput_gb_per_s,
    }


def normalize_commit_timestamp(raw_timestamp: str | None) -> str:
    if raw_timestamp:
        return raw_timestamp
    return (
        datetime.now(timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def main() -> int:
    args = parse_args()

    criterion_dir = Path(args.criterion_dir)
    output_file = Path(args.output)

    results_by_id = load_criterion_results(criterion_dir)

    fixtures: list[dict[str, Any]] = []
    for benchmark_id in sorted(results_by_id):
        benchmark, estimates = results_by_id[benchmark_id]
        fixtures.append(build_fixture_entry(benchmark, estimates))

    total_input_bytes = sum(int(item["input_bytes"]) for item in fixtures)
    weighted_total_seconds = sum(float(item["mean_seconds"]) for item in fixtures)

    aggregate_throughput = (
        float(total_input_bytes) / weighted_total_seconds / BYTES_PER_DECIMAL_GIGABYTE
        if weighted_total_seconds > 0.0
        else 0.0
    )

    result = {
        "commit_sha": args.commit_sha,
        "commit_timestamp": normalize_commit_timestamp(args.commit_timestamp),
        "runner": args.runner,
        "rust_version": args.rust_version,
        "fixtures": fixtures,
        "aggregate": {
            "total_input_bytes": total_input_bytes,
            "weighted_total_seconds": weighted_total_seconds,
            "aggregate_throughput_gb_per_s": aggregate_throughput,
        },
    }

    output_file.parent.mkdir(parents=True, exist_ok=True)
    with output_file.open("w", encoding="utf-8") as handle:
        json.dump(result, handle, indent=2)
        handle.write("\n")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
