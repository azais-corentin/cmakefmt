# Repository Guidelines

## Project Overview

`cmakefmt` is a CMake file formatter written in Rust (edition 2024). It produces two artifacts:
1. A **native CLI binary** (`cmakefmt`) for standalone use.
2. A **WASM plugin** (`cdylib`, `wasm32-unknown-unknown`) for integration with [dprint](https://dprint.dev/).

It formats `CMakeLists.txt` and `*.cmake` files with configurable indentation, line width, command/keyword casing, list sorting, and comment preservation.

## Architecture & Data Flow

The formatting pipeline is a three-stage process:

```
Input text
  -> strip BOM
  -> parser::parse()          # Logos lexer -> recursive descent parser -> AST
  -> gen_file() / gen_command()  # AST -> dprint_core PrintItems (IR)
  -> dprint_core::formatting::format()  # IR -> formatted text
  -> compare to input (None if unchanged, Some(result) if changed)
```

Key design decisions:
- The lexer uses [Logos](https://docs.rs/logos) for zero-copy tokenization.
- The parser is a hand-written recursive descent parser producing a simple AST (`File -> FileElement -> CommandInvocation -> Argument`).
- Code generation emits `dprint_core::formatting::PrintItems`, which handles line wrapping, indentation, and layout decisions.
- Command formatting is signature-aware: `src/generation/signatures.rs` contains static specs for known CMake commands (keyword types, positional args, sections). Unknown commands are formatted with content preservation.
- Block structure (indentation for `if`/`foreach`/`while`/`macro`/`function`) is tracked via `BLOCK_OPENERS`, `BLOCK_MIDDLES`, `BLOCK_CLOSERS` sets in `gen_file.rs`.

## Key Directories

```
src/
  bin/cmakefmt.rs          # CLI entry point (clap derive API)
  lib.rs                   # Library root, public API surface
  format_text.rs           # Top-level formatting pipeline
  wasm_plugin.rs           # dprint WASM plugin handler (compiled only on wasm32)
  parser/
    token.rs               # Logos lexer token definitions
    ast.rs                 # AST types (File, FileElement, CommandInvocation, Argument, Span)
    parse.rs               # Recursive descent parser
  generation/
    gen_file.rs            # File-level IR generation (block indentation, blank line handling)
    gen_command.rs          # Command-level IR generation (~3200 lines, largest file)
    signatures.rs          # Static command signatures (CommandSpec, KwType, lookup_command)
  configuration/
    types.rs               # Configuration struct and enums (CaseStyle, NewLineKind)
    resolve.rs             # dprint plugin config resolution from JSON
tests/
  formatter_tests.rs       # Integration test harness
  formatter/               # Test fixtures (*.in.cmake / *.out.cmake pairs)
    empty/                 # Edge case fixtures
    repos/                 # Real-world repo samples
```

## Development Commands

The project uses [mise](https://mise.jdx.dev/) as task runner. All tasks are defined in `mise.toml`.

| Command | Purpose |
|---|---|
| `cargo test` | Run all tests (host target) |
| `cargo clippy -- -D warnings` | Lint with all warnings as errors |
| `cargo fmt --check` | Check Rust formatting |
| `mise run check` | Clippy + rustfmt check (combined) |
| `mise run build` | Build WASM plugin (debug) |
| `mise run release` | Build optimized WASM plugin |
| `mise run test` | Run all tests |
| `mise run fmt` | Format sample files using the built WASM plugin via dprint |

The dev environment is Nix-based (`devenv.nix` + `devenv.yaml` + `.envrc` for direnv). It provides a stable Rust toolchain via `rust-overlay`.

## Code Conventions & Patterns

### Error Handling
- `anyhow::Result` for fallible operations throughout the codebase.
- `anyhow::bail!` for parser errors (not custom error types).
- The CLI catches panics with `std::panic::catch_unwind` in tests for robustness.

### Naming
- Rust standard: `snake_case` for functions/variables, `PascalCase` for types/enums.
- Module files match their purpose directly (`gen_file.rs`, `gen_command.rs`, `signatures.rs`).
- Test fixtures: `<name>.in.cmake` (input) / `<name>.out.cmake` (expected output).

### Architecture Patterns
- **Span-based AST**: The AST stores `Span` (byte range) references into the original source `&str` rather than owned strings. This enables zero-copy formatting for passthrough content.
- **Case-insensitive command lookup**: `lookup_command()` in `signatures.rs` normalizes command names to lowercase for matching.
- **Signature-driven formatting**: Known commands are formatted based on their `CommandSpec` (positional args, keywords, sections). Unknown commands get a generic layout.
- **`KwType` enum**: `Option` (flag), `OneValue`, `MultiValue`, `Group` for keyword argument classification.
- **Block tracking**: `gen_file.rs` maintains `indent_level` by matching command names against opener/closer/middle sets.
- **Conditional compilation**: `wasm_plugin.rs` is gated behind `#[cfg(target_arch = "wasm32")]`.

### Configuration
The `Configuration` struct (`src/configuration/types.rs`) has these fields and defaults:
- `line_width`: 80
- `indent_width`: 2
- `use_tabs`: false
- `new_line_kind`: Auto (detect from file)
- `command_case`: Lower
- `keyword_case`: Upper
- `closing_paren_newline`: true
- `sort_lists`: false
- `max_blank_lines`: 1
- `space_before_paren`: false

## Public API

The library exports (`src/lib.rs`):
- `format_text(path, input, config) -> Result<Option<String>>` -- core formatting function. Returns `None` if input is already formatted.
- `Configuration`, `CaseStyle`, `NewLineKind` -- configuration types.
- `resolve_config` -- dprint plugin config resolution.

## Testing

### Framework
Rust's built-in `#[test]` via `cargo test`. No third-party test framework.

### Test Structure
All tests live in `tests/formatter_tests.rs`. There are two test categories:

1. **Fixture-based integration tests** (`test_formatter_files`): Discovers `*.in.cmake` / `*.out.cmake` pairs under `tests/formatter/`. Formats input, compares to expected output, then checks **idempotency** (formatting the output again produces no change).

2. **Inline unit tests** (`invisible_diff_tests` module): Tests for diff helper functions used in test output.

### Adding a Test
1. Create `tests/formatter/<name>.in.cmake` with the unformatted input.
2. Create `tests/formatter/<name>.out.cmake` with the expected formatted output.
3. Optionally add a `### {"commandCase": "upper", ...}` JSON header as the first line of the `.in.cmake` file to override `Configuration` for that test case. Supports camelCase keys matching `Configuration` field names.
4. Run `cargo test`.

### Test Assertions
- Output equality via string comparison.
- Idempotency: `format_text(output) == None` (already formatted).
- Failures show colored unified diffs via `imara-diff`.

## Dependencies

| Crate | Purpose |
|---|---|
| `dprint-core` | Formatting IR (`PrintItems`), WASM plugin interface |
| `logos` | Lexer/tokenizer generation |
| `clap` (derive) | CLI argument parsing |
| `anyhow` | Error handling |
| `glob` | File pattern expansion in CLI |
| `serde` + `serde_json` | Configuration serialization |
| `imara-diff` (dev) | Unified diff output in test failures |

## Important Files

- `src/generation/gen_command.rs` -- The largest and most complex file (~3200 lines). Contains all command-specific formatting logic including keyword grouping, section/property/pair/flow layouts, single-line vs multi-line decisions, and sort_lists support. Most formatting bugs and features will involve this file.
- `src/generation/signatures.rs` -- Static database of known CMake command signatures (~2200 lines). Must be updated when adding support for new commands.
- `src/parser/parse.rs` -- The parser. Handles edge cases like unquoted legacy concatenation (adjacent tokens merged into one Unquoted span).
- `src/configuration/resolve.rs` -- Maps dprint JSON config keys to `Configuration`. Declares file matching patterns (`*.cmake`, `CMakeLists.txt`).

## Build Artifacts

- **Native binary**: `target/debug/cmakefmt` or `target/release/cmakefmt`
- **WASM plugin**: `target/wasm32-unknown-unknown/release/cmakefmt.wasm`
- Release profile uses `lto = true` and `opt-level = "s"` for size optimization of the WASM artifact.
