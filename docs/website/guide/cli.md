# CLI Reference

## Usage

```bash
cmakefmt [OPTIONS] [FILE...]
```

`cmakefmt` formats CMake files (`CMakeLists.txt` and `*.cmake`). By default, it reads the given files and writes formatted output to stdout.

If no files are given and `--stdin` is not passed, the formatter reads from stdin.

## Modes

### Default (stdout)

Formatted output is written to stdout. No files are modified.

### `--write` / `--inplace`

Write formatted output back to the input file(s) in-place. `-w` is the short form. `--write` and `--inplace` are aliases for the same behavior.

### `--check`

Exit with a non-zero status code if any file would be changed, without writing changes. Useful for CI enforcement.

### `--diff`

Print a unified diff of formatting changes to stdout instead of the formatted output. No files are modified.

### `--stdin`

Read CMake source from stdin and write formatted output to stdout. Used for editor integration (pipe through formatter). When `--stdin` is passed alongside file arguments, it is an error — the formatter exits with code 2 and a diagnostic message.

## Exit Codes

| Code | Meaning                                                         |
| ---- | --------------------------------------------------------------- |
| `0`  | Success — all files formatted (or already formatted).           |
| `1`  | Formatting changes found (`--check` mode) — files need changes. |
| `2`  | Error — parse failure, config error, or I/O error.              |

## Flags

### Output Mode

| Flag                         | Description                                                        |
| ---------------------------- | ------------------------------------------------------------------ |
| `--check`                    | Exit non-zero if changes needed. No files are modified.            |
| `--diff`                     | Print a unified diff of formatting changes. No files are modified. |
| `-w`, `--write`, `--inplace` | Write formatted output back to files in-place.                     |
| `--stdin`                    | Read source from stdin, write formatted output to stdout.          |

### Configuration

| Flag                       | Description                                                                                                                       |
| -------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `--config <PATH>`          | Explicit path to a `.cmakefmt.toml` configuration file. Overrides the normal config discovery walk.                               |
| `--assume-filename <PATH>` | Pretend stdin input comes from this file path. Used for config discovery and file-type detection. Only meaningful with `--stdin`. |
| `--print-config`           | Print the resolved configuration (after merging defaults, config file, and CLI overrides) as TOML to stdout.                      |

### Diagnostics

| Flag         | Description                                                                                         |
| ------------ | --------------------------------------------------------------------------------------------------- |
| `--color`    | Always emit ANSI color in diagnostics and diff output.                                              |
| `--no-color` | Suppress ANSI color in diagnostics and diff output.                                                 |
| `--verbose`  | Increase diagnostic output. Useful for debugging configuration resolution and formatting decisions. |
| `--quiet`    | Suppress non-error output.                                                                          |

Without `--color` or `--no-color`, color is auto-detected based on whether stdout is a terminal.

### Tracing

| Flag                            | Description                                                                                                                    |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `--trace-output <PATH>`         | Enable tracing and write Chrome trace JSON (`traceEvents`) to the provided path. One trace file is emitted per invocation.     |
| `--trace-summary-output <PATH>` | Write a normalized summary JSON (stage aggregates, hotspots, per-file timing) to the provided path. Requires `--trace-output`. |
| `--trace-filter <DIRECTIVE>`    | `tracing_subscriber::EnvFilter` directive string for trace capture. Default: `cmakefmt=info,cmakefmt_cli=info`.                |

## Format Override Flags

These CLI flags override values from the [configuration file](/guide/configuration):

| Flag                                       | Description                                                                            |
| ------------------------------------------ | -------------------------------------------------------------------------------------- |
| `--line-width <N>`                         | Maximum line width.                                                                    |
| `--indent-width <N>`                       | Number of spaces per indentation level.                                                |
| `--use-tabs`                               | Use tabs instead of spaces for indentation.                                            |
| `--new-line-kind <lf\|crlf\|auto>`         | Newline style.                                                                         |
| `--command-case <lower\|upper\|unchanged>` | Case style for commands.                                                               |
| `--keyword-case <lower\|upper\|unchanged>` | Case style for keywords.                                                               |
| `--closing-paren-newline <true\|false>`    | Place closing paren on a new line in multi-line commands.                              |
| `--sort-lists`                             | Sort argument lists alphabetically.                                                    |
| `--max-blank-lines <N>`                    | Maximum consecutive blank lines to preserve.                                           |
| `--space-before-paren <cmd1,cmd2,...>`     | Insert space before `(` for these commands (comma-separated, e.g. `if,while,foreach`). |

## Flag Interactions

| Combination                                       | Behavior                                                                                                                                             |
| ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--check` + `--diff`                              | Both can be used together. `--diff` prints the unified diff to stdout; `--check` controls the exit code. No changes needed → exit 0, no diff output. |
| `--check` + `--write`                             | Mutually exclusive. The formatter exits with code 2 and a diagnostic message.                                                                        |
| `--diff` + `--write`                              | Mutually exclusive. The formatter exits with code 2 and a diagnostic message.                                                                        |
| `--stdin` + file arguments                        | Error, exit code 2.                                                                                                                                  |
| `--quiet` + `--verbose`                           | `--quiet` wins; non-error output is suppressed.                                                                                                      |
| `--assume-filename` without `--stdin`             | Warning, flag is ignored.                                                                                                                            |
| `--trace-summary-output` without `--trace-output` | Error, exit code 2.                                                                                                                                  |
