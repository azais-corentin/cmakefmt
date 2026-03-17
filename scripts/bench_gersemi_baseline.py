#!/usr/bin/env python3
"""pytest-benchmark tests for gersemi.

Measures only the in-memory formatting call — no file I/O, no argument parsing,
no process startup.  This is the fairest comparison to cmakefmt's format_text()
which is what Criterion benchmarks.

Usage:
    pytest scripts/bench_gersemi_baseline.py \
        --benchmark-json="gersemi-bench.json"
"""

from __future__ import annotations

from pathlib import Path

import pytest
from gersemi.configuration import OutcomeConfiguration
from gersemi.runner import create_formatter

FIXTURES = {
    "xnnpack": Path(
        "crates/cmakefmt/tests/formatter/respositories/XNNPACK/CMakeLists.in.cmake"
    ),
    "synthetic": Path(
        "crates/cmakefmt/tests/formatter/respositories/synthetic/CMakeLists.in.cmake"
    ),
}


@pytest.mark.parametrize("fixture_name", FIXTURES.keys())
def test_gersemi(benchmark, fixture_name):
    fixture_path = FIXTURES[fixture_name]
    cfg = OutcomeConfiguration()
    formatter = create_formatter(cfg, known_definitions={}, lines_to_format=())
    source = fixture_path.read_text(encoding="utf-8")
    benchmark.extra_info["fixture_name"] = fixture_name
    benchmark.extra_info["input_bytes"] = len(source.encode("utf-8"))
    benchmark(formatter.format, source)
