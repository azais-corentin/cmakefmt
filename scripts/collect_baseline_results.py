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


def now_iso() -> str:
    return (
        datetime.now(timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def extract_baseline(
    bench_json: dict[str, Any],
    tool_version: str,
    runner: str,
    rust_version: str,
) -> dict[str, Any]:
    """Extract baseline stats from pytest-benchmark JSON with per-fixture detail."""
    benchmarks = bench_json.get("benchmarks")
    if not benchmarks or not isinstance(benchmarks, list):
        raise ValueError("pytest-benchmark JSON missing 'benchmarks' array")

    timestamp = now_iso()
    fixtures: list[dict[str, Any]] = []
    total_input_bytes = 0
    total_mean = 0.0
    total_median = 0.0

    for bench in benchmarks:
        stats = bench.get("stats")
        extra = bench.get("extra_info", {})
        if not stats:
            raise ValueError("pytest-benchmark JSON missing 'stats' in a benchmark")

        fixture_name = extra.get("fixture_name", bench.get("name", "unknown"))
        input_bytes = int(extra.get("input_bytes", 0))
        mean_s = float(stats["mean"])
        median_s = float(stats["median"])
        min_s = float(stats["min"])
        max_s = float(stats["max"])
        stddev_s = float(stats["stddev"])
        iterations = int(stats["iterations"])

        fixtures.append(
            {
                "fixture_name": fixture_name,
                "input_bytes": input_bytes,
                "mean_seconds": mean_s,
                "median_seconds": median_s,
                "min_seconds": min_s,
                "max_seconds": max_s,
                "stddev_seconds": stddev_s,
                "iterations": iterations,
                "throughput_gb_per_s": throughput_gb_per_s(input_bytes, mean_s),
            }
        )

        total_input_bytes += input_bytes
        total_mean += mean_s
        total_median += median_s

    return {
        "tool_version": tool_version,
        "runner": runner,
        "rust_version": rust_version,
        "timestamp": timestamp,
        "input_bytes": total_input_bytes,
        "mean_seconds": total_mean,
        "median_seconds": total_median,
        "throughput_gb_per_s": throughput_gb_per_s(total_input_bytes, total_mean),
        "fixtures": fixtures,
    }


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
        "cmake_format": extract_baseline(
            cmake_format_json,
            args.cmake_format_version,
            args.runner,
            args.rust_version,
        ),
        "gersemi": extract_baseline(
            gersemi_json,
            args.gersemi_version,
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
