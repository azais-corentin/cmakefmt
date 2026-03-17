#!/usr/bin/env python3
"""Collect cmake_format and gersemi baseline benchmark results into the history JSON.

Reads pytest-benchmark JSON output for both cmake_format and gersemi, then
writes a ``baselines`` key into the history file alongside ``schema_version``
and ``entries``.

Usage:
    python3 scripts/collect_baseline_results.py \
        --cmake-format-json cmake-format-bench.json \
        --gersemi-json gersemi-bench.json \
        --cmake-format-version "0.6.13" \
        --gersemi-version "0.17.1" \
        --input-bytes 351753 \
        --runner Linux \
        --rust-version "rustc 1.82.0" \
        --history-file benchmarks/cmakefmt-fixtures-history.json
"""

from __future__ import annotations

import argparse
import json
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

BYTES_PER_DECIMAL_GIGABYTE = 1_000_000_000.0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Collect baseline benchmark results for cmake_format and gersemi.",
    )
    parser.add_argument(
        "--cmake-format-json",
        required=True,
        help="Path to pytest-benchmark JSON output for cmake_format.",
    )
    parser.add_argument(
        "--gersemi-json",
        required=True,
        help="Path to pytest-benchmark JSON output for gersemi.",
    )
    parser.add_argument(
        "--cmake-format-version",
        required=True,
        help="cmake_format (cmakelang) version string.",
    )
    parser.add_argument(
        "--gersemi-version",
        required=True,
        help="gersemi version string.",
    )
    parser.add_argument(
        "--input-bytes",
        required=True,
        type=int,
        help="Size of the benchmark fixture in bytes.",
    )
    parser.add_argument(
        "--runner",
        required=True,
        help="Runner identifier (e.g. Linux).",
    )
    parser.add_argument(
        "--rust-version",
        required=True,
        help="Rust toolchain version used for the benchmark run.",
    )
    parser.add_argument(
        "--history-file",
        required=True,
        help="Path to persistent benchmark history JSON.",
    )
    return parser.parse_args()


def throughput_gb_per_s(input_bytes: int, mean_seconds: float) -> float:
    if mean_seconds <= 0.0:
        return 0.0
    return float(input_bytes) / mean_seconds / BYTES_PER_DECIMAL_GIGABYTE


def extract_pytest_benchmark_baseline(
    bench_json: dict[str, Any],
    tool_version: str,
    input_bytes: int,
    runner: str,
    rust_version: str,
) -> dict[str, Any]:
    """Extract baseline stats from pytest-benchmark JSON output."""
    benchmarks = bench_json.get("benchmarks")
    if not benchmarks or not isinstance(benchmarks, list):
        raise ValueError("pytest-benchmark JSON missing 'benchmarks' array")

    stats = benchmarks[0].get("stats")
    if not stats:
        raise ValueError("pytest-benchmark JSON missing 'stats' in first benchmark")

    mean_s = float(stats["mean"])
    median_s = float(stats["median"])
    min_s = float(stats["min"])
    max_s = float(stats["max"])
    stddev_s = float(stats["stddev"])
    iterations = int(stats["iterations"])

    return {
        "tool_version": tool_version,
        "runner": runner,
        "rust_version": rust_version,
        "timestamp": now_iso(),
        "input_bytes": input_bytes,
        "mean_seconds": mean_s,
        "median_seconds": median_s,
        "min_seconds": min_s,
        "max_seconds": max_s,
        "stddev_seconds": stddev_s,
        "iterations": iterations,
        "throughput_gb_per_s": throughput_gb_per_s(input_bytes, mean_s),
    }


def now_iso() -> str:
    return (
        datetime.now(timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def load_history(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {"schema_version": 1, "entries": []}
    with path.open("r", encoding="utf-8") as f:
        data = json.load(f)
    if not isinstance(data, dict):
        raise ValueError(f"history file must contain an object: {path}")
    return data


def main() -> int:
    args = parse_args()

    cmake_format_json = json.loads(
        Path(args.cmake_format_json).read_text(encoding="utf-8")
    )
    gersemi_json = json.loads(Path(args.gersemi_json).read_text(encoding="utf-8"))

    history = load_history(Path(args.history_file))

    history["baselines"] = {
        "cmake_format": extract_pytest_benchmark_baseline(
            cmake_format_json,
            args.cmake_format_version,
            args.input_bytes,
            args.runner,
            args.rust_version,
        ),
        "gersemi": extract_pytest_benchmark_baseline(
            gersemi_json,
            args.gersemi_version,
            args.input_bytes,
            args.runner,
            args.rust_version,
        ),
    }

    history_file = Path(args.history_file)
    history_file.parent.mkdir(parents=True, exist_ok=True)
    with history_file.open("w", encoding="utf-8") as f:
        json.dump(history, f, indent=2)
        f.write("\n")

    print(f"Baselines written to {history_file}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
