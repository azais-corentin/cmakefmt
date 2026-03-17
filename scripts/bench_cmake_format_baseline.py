#!/usr/bin/env python3
"""pytest-benchmark test for cmake_format (cmakelang).

Measures only the in-memory formatting call — no file I/O, no argument parsing,
no process startup.  This is the fairest comparison to cmakefmt's format_text()
which is what Criterion benchmarks.

Usage:
    pytest scripts/bench_cmake_format_baseline.py \
        --benchmark-json="cmake-format-bench.json"
"""

from __future__ import annotations

from pathlib import Path

from cmakelang import configuration
from cmakelang.format.__main__ import process_file

FIXTURE = Path(
    "crates/cmakefmt/tests/formatter/respositories/XNNPACK/CMakeLists.in.cmake"
)


def test_cmake_format(benchmark):
    cfg = configuration.Configuration()
    source = FIXTURE.read_text(encoding="utf-8")
    benchmark(process_file, cfg, source)
