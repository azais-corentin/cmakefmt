# Tracing Instrumentation Guide

This document describes runtime tracing for `cmakefmt` using `tracing` + `tracing-chrome`.

## Scope and Activation

Tracing is **CLI-only** and fully opt-in.

- No `.cmakefmt.toml` keys are used for tracing.
- One invocation of `cmakefmt` produces one combined trace artifact pair.

## CLI Flags

- `--trace-output <PATH>`
  - Enables tracing and writes Chrome trace JSON (`traceEvents`) to `PATH`.
- `--trace-summary-output <PATH>`
  - Writes a normalized summary JSON to `PATH`.
  - Requires `--trace-output`.
- `--trace-filter <DIRECTIVE>`
  - Optional `tracing_subscriber::EnvFilter` directive string.
  - Default filter: `cmakefmt=info,cmakefmt_cli=info`.

## Output Artifacts

### 1) Chrome trace JSON

Raw timeline output compatible with Chrome Trace Viewer / Perfetto.

- Field: `traceEvents`
- Includes span timing events emitted by cmakefmt core + CLI runtime.

### 2) Normalized summary JSON

Stable, AI-friendly aggregate view built from the Chrome trace.

Top-level schema:

- `schemaVersion`: currently `cmakefmt.trace.summary.v1`
- `toolVersion`: CLI package version
- `generatedAt`: generation timestamp (string, unix-ms encoded)
- `generatedAtUnixMs`: generation timestamp (numeric unix ms)
- `invocation`
  - `mode`: `{ check, diff, write, stdin }`
  - `fileCount`
  - `inputBytesTotal`
  - `changedFiles`
  - `errorCount`
- `timing`
  - `totalWallMs`
  - `stages[]`: `{ name, totalMs, pctTotal, calls, avgMs, p95Ms }`
  - `hotspots[]`: `{ name, totalMs, selfMs, calls, avgMs, p95Ms }`
- `files[]`
  - `path`
  - `inputBytes`
  - `changed`
  - `stageDurationsMs` (map of stage name -> ms)
  - `status` (`ok` | `error`)
- `notes[]`

## Event Taxonomy

Representative span names (stable contract):

- Pipeline-level
  - `cmakefmt.format.invocation`
  - `cmakefmt.format.pipeline`
  - `cmakefmt.format.parse`
  - `cmakefmt.format.generate_ir`
  - `cmakefmt.format.print`
  - `cmakefmt.format.post_process`
- Parser
  - `cmakefmt.parser.file`
  - `cmakefmt.parser.command`
- Generation
  - `cmakefmt.gen_file`
  - `cmakefmt.gen_file.command`
  - `cmakefmt.gen_command`
- Printer
  - `cmakefmt.printer.format`
- Post-process
  - `cmakefmt.post_process`
  - `cmakefmt.post_process.align_block`
  - `cmakefmt.post_process.reflow_comment`

## Privacy and Data Policy

Tracing metadata is intentionally structural/timing-oriented.

- Includes timings, counts, command names, and file paths.
- Excludes CMake source snippets and argument text in the normalized summary.

## Usage Examples

Trace one file:

```bash
cmakefmt --trace-output trace.json --trace-summary-output trace-summary.json CMakeLists.txt
```

Trace with custom filter:

```bash
cmakefmt \
  --trace-output trace.json \
  --trace-summary-output trace-summary.json \
  --trace-filter 'cmakefmt=info,cmakefmt_cli=info' \
  --write \
  path/to/A.cmake path/to/B.cmake
```

## Notes for AI Performance Analysis

For optimization triage:

1. Start from `timing.stages` to identify dominant phases.
2. Use `timing.hotspots` to rank expensive functions/spans.
3. Use `files[]` to detect skew (single-file outliers vs broad regressions).
4. Correlate suspicious spans in summary with raw Chrome timeline for sequencing and overlap.
