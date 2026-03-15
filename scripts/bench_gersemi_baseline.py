#!/usr/bin/env python3
"""pytest-benchmark test for gersemi.

Measures only the in-memory formatting call — no file I/O, no argument parsing,
no process startup.  This is the fairest comparison to cmakefmt's format_text()
which is what Criterion benchmarks.

Usage:
    pytest scripts/bench_gersemi_baseline.py \
        --benchmark-json="gersemi-bench.json"
"""

from __future__ import annotations

from pathlib import Path

from gersemi.configuration import OutcomeConfiguration
from gersemi.runner import create_formatter

FIXTURE = Path("crates/cmakefmt/tests/formatter/respositories/XNNPACK/CMakeLists.in.cmake")


def test_gersemi(benchmark):
    cfg = OutcomeConfiguration()
    formatter = create_formatter(cfg, known_definitions={}, lines_to_format=())
    source = FIXTURE.read_text(encoding="utf-8")
    benchmark(formatter.format, source)
