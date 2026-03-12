---
name: cmakefmt-tracing
description: Generate and interpret cmakefmt trace artifacts.
---

# CMakefmt Tracing Skill

Quickstart for generating cmakefmt trace artifacts and getting useful signal from them fast.

## Quickstart

Trace a single file:

```bash
cmakefmt \
  --trace-output trace.json \
  --trace-summary-output trace-summary.json \
  CMakeLists.txt
```

Trace multiple files in one invocation:

```bash
cmakefmt \
  --trace-output trace.json \
  --trace-summary-output trace-summary.json \
  --write \
  path/to/A.cmake path/to/B.cmake
```

Optional: narrow emitted events with a custom filter:

```bash
cmakefmt \
  --trace-output trace.json \
  --trace-summary-output trace-summary.json \
  --trace-filter 'cmakefmt=info,cmakefmt_cli=info' \
  path/to/CMakeLists.txt
```

## Output Artifacts

- `--trace-output <PATH>`: Chrome trace JSON (timeline) at the provided path, e.g. `trace.json`.
- `--trace-summary-output <PATH>`: normalized aggregate summary JSON at the provided path, e.g. `trace-summary.json`.

Both are per-invocation artifacts.

## How to Read the Summary Fast

Start in this order:

1. `timing.stages` — which pipeline stage dominates wall time.
2. `timing.hotspots` — which spans/functions are most expensive.
3. `files` — whether cost is broad or concentrated in a few files.

If a hotspot is unclear, correlate it with the Chrome trace timeline for ordering and overlap.

## Common Pitfalls

- `--trace-summary-output` requires `--trace-output`; using summary alone exits with an error.
- If tracing flags are omitted, no trace artifacts are produced.
- Summary data is timing/structure oriented; it is not source-content analysis.
