## Appendix D — CLI Reference

These are CLI-only flags and are not configuration file options. They control the
formatter's runtime behavior, not the formatting output itself.

### Exit Codes

| Code | Meaning                                                         |
| ---- | --------------------------------------------------------------- |
| `0`  | Success — all files formatted (or already formatted).           |
| `1`  | Formatting changes found (`--check` mode) — files need changes. |
| `2`  | Error — parse failure, config error, or I/O error.              |

### `--check`

Exit with a non-zero status code if any file would be changed, without writing changes.
Useful for CI enforcement.

### `--diff`

Print a unified diff of formatting changes to stdout instead of the formatted output. No files are modified.

### `--stdin`

Read CMake source from stdin and write formatted output to stdout. Used for editor
integration (pipe through formatter). When `--stdin` is passed alongside file arguments,
it is an error — the formatter exits with code 2 and a diagnostic message.

### `--write` / `--inplace`

These flags are aliases for the same behavior: write formatted output back to the input
file(s) in-place. Without this flag, formatted output is written to stdout.

### `--config <path>`

Explicit path to a `.cmakefmt.toml` configuration file. Overrides the normal config
discovery walk.

### `--assume-filename <path>`

Pretend that stdin input comes from this file path. Used for config discovery (walk
upward from this file's directory) and file-type detection. Only meaningful with `--stdin`.

### `--color` / `--no-color`

Control colored output in diff and diagnostic output. When `--color` is set, ANSI color
codes are always emitted. When `--no-color` is set, color output is suppressed. Without
either flag, color is auto-detected based on whether stdout is a terminal.

### `--verbose`

Increase diagnostic output. Useful for debugging configuration resolution and
formatting decisions.

### `--quiet`

Suppress non-error output.

### `--trace-output <path>`

Enable tracing and write Chrome trace JSON (`traceEvents`) to the provided path.
One trace file is emitted per CLI invocation.

### `--trace-summary-output <path>`

Write a normalized summary JSON (stage aggregates, hotspots, per-file timing) to the
provided path. Requires `--trace-output`.

> Privacy note: the summary intentionally excludes CMake source snippets and argument text.

### `--trace-filter <directive>`

Optional `tracing_subscriber::EnvFilter` directive string used when trace capture is
enabled. If omitted, the default filter is `cmakefmt=info,cmakefmt_cli=info`.

### `--print-config`

Print the resolved configuration (after merging defaults, config file, and CLI overrides)
as TOML to stdout. Useful for debugging.
### Flag Interactions

| Combination                                       | Behavior                                                                                                                                             |
| ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--check` + `--diff`                              | Both can be used together. `--diff` prints the unified diff to stdout; `--check` controls the exit code. No changes needed → exit 0, no diff output. |
| `--check` + `--write`                             | Mutually exclusive. The formatter exits with code 2 and a diagnostic message.                                                                        |
| `--diff` + `--write`                              | Mutually exclusive. The formatter exits with code 2 and a diagnostic message.                                                                        |
| `--stdin` + file arguments                        | Error, exit code 2 (see `--stdin` above).                                                                                                            |
| `--quiet` + `--verbose`                           | `--quiet` wins; non-error output is suppressed.                                                                                                      |
| `--assume-filename` without `--stdin`             | Warning, flag is ignored.                                                                                                                            |
| `--trace-summary-output` without `--trace-output` | Error, exit code 2.                                                                                                                                  |
