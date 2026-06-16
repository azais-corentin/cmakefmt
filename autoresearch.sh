#!/usr/bin/env bash
# Autoresearch benchmark entrypoint for cmakefmt::format_text throughput.
#
# Builds the throughput harness against the core lib in RELEASE (matches the
# shipping profile: [profile.release] lto=true, opt-level=2) and runs a tight
# std::time::Instant loop. Emits:
#   METRIC throughput_mb_s=<value>            (primary, XNNPACK fixture)
#   METRIC throughput_synthetic_mb_s=<value>  (secondary, synthetic fixture)
#
# Output parity is the hard constraint: the harness diffs every formatted output
# against the committed *.out.cmake baseline and exits non-zero on any change.
set -euo pipefail

cd "$(dirname "$0")"

# Build only the harness example for the core crate, in release.
cargo build --release --quiet --example throughput_bench -p cmakefmt-rs

BIN="target/release/examples/throughput_bench"
if [[ ! -x "$BIN" ]]; then
  echo "harness binary not found: $BIN" >&2
  exit 1
fi

exec "$BIN"
