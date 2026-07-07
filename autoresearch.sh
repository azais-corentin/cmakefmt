#!/usr/bin/env bash
# Autoresearch benchmark entrypoint: formats the full fixture corpus
# (weighted toward respositories/XNNPACK) and prints METRIC lines.
set -euo pipefail
cd "$(dirname "$0")"

cargo bench --bench autoresearch_bench --package cmakefmt-rs
