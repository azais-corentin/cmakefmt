#!/usr/bin/env python3
"""pytest-benchmark tests for cmake_format (cmakelang).

Measures only the in-memory formatting call — no file I/O, no argument parsing,
no process startup.  This is the fairest comparison to cmakefmt's format_text()
which is what Criterion benchmarks.

Usage:
    pytest scripts/bench_cmake_format_baseline.py \
        --benchmark-json="cmake-format-bench.json"
"""

from __future__ import annotations

from pathlib import Path

import pytest
from cmakelang import configuration
from cmakelang.format.__main__ import process_file

FIXTURES = {
    "xnnpack": Path(
        "crates/cmakefmt/tests/formatter/respositories/XNNPACK/CMakeLists.in.cmake"
    ),
    "synthetic": Path(
        "crates/cmakefmt/tests/formatter/respositories/synthetic/CMakeLists.in.cmake"
    ),
}


@pytest.mark.parametrize("fixture_name", FIXTURES.keys())
def test_cmake_format(benchmark, fixture_name):
    fixture_path = FIXTURES[fixture_name]
    cfg = configuration.Configuration()
    source = fixture_path.read_text(encoding="utf-8")
    benchmark.extra_info["fixture_name"] = fixture_name
    benchmark.extra_info["input_bytes"] = len(source.encode("utf-8"))
    benchmark(process_file, cfg, source)
