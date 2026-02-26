# Repository Guidelines

## Project Overview

`cmakefmt` (crate name: `dprint-plugin-cmake`) is a CMake code formatter written in Rust. It ships as:

1. **`cmakefmt` binary** — standalone CLI formatter (`src/bin/cmakefmt.rs`)
2. **dprint WASM plugin** — `cdylib` loaded by the [dprint](https://dprint.dev/) formatting engine at runtime (`src/wasm_plugin.rs`, compiled only on `wasm32`)

It parses CMake files (`.cmake`, `CMakeLists.txt`) into an AST, generates a dprint IR (intermediate representation), and lets dprint-core handle line-wrapping and printing.

## Architecture & Data Flow

```
Input text
  → strip BOM
  → Lexer (logos)          src/parser/token.rs     Token enum
  → Parser                 src/parser/parse.rs     recursive descent → AST
  → IR Generation          src/generation/         AST → dprint PrintItems
  → dprint-core formatter                          PrintItems → formatted text
  → Idempotency check      src/format_text.rs      return None if unchanged
```

### Key Modules

| Module | Purpose |
|---|---|
| `parser::token` | Logos-derived `Token` enum (whitespace, parens, comments, quoted/unquoted args, bracket args) |
| `parser::ast` | AST types: `File`, `FileElement`, `CommandInvocation`, `Argument`, `Span` |
| `parser::parse` | Recursive descent parser; `parse(input) -> File` |
| `generation::gen_file` | File-level IR: handles indentation via block openers/closers (`if`/`endif`, `foreach`/`endforeach`, etc.), blank line collapsing |
| `generation::gen_command` | Command-level IR: keyword casing, single-line vs multi-line layout, optional list sorting |
| `configuration` | `Configuration` struct, `CaseStyle`/`NewLineKind` enums, `resolve_config` for dprint plugin config resolution |
| `format_text` | Top-level `format_text(path, input, config) -> Result<Option<String>>` — the public API |
| `wasm_plugin` | dprint WASM ABI bridge (conditionally compiled on `wasm32`) |
| `bin/cmakefmt` | Clap-derived CLI: stdin, `--check`, `--write`, glob expansion |

## Key Directories

```
src/
  bin/cmakefmt.rs         CLI binary entry point
  lib.rs                  Library root, public re-exports
  format_text.rs          Formatting pipeline
  parser/                 Lexer (logos), AST, recursive descent parser
  generation/             IR generation (file-level + command-level)
  configuration/          Config types, dprint config resolution
  wasm_plugin.rs          WASM plugin bridge (wasm32 only)
tests/
  integration.rs          dprint-development spec runner
  formatter_tests.rs      Golden file (in/out) test runner
  specs/                  dprint-style spec files (*.txt)
  formatter/              Golden file pairs (*.in.cmake / *.out.cmake)
    builtin/              Built-in CMake command tests
    custom_commands/      Custom command formatting tests
    disabled_formatting/  cmake-format:on/off directive tests
    issues/               Numbered regression tests (NNNN_description.*)
```

## Development Commands

Task runner: **[mise](https://mise.jdx.dev/)** (defined in `mise.toml`).

| Command | Action |
|---|---|
| `mise run build` | Debug build for WASM target |
| `mise run release` | Optimized WASM build (LTO, `opt-level=s`) |
| `mise run test` | `cargo test` (runs on host, not WASM) |
| `mise run check` | `cargo clippy -- -D warnings` + `cargo fmt --check` |
| `mise run fmt` | Format sample files with the built WASM plugin via dprint |
| `mise run size` | Report WASM binary size |

Direct equivalents: `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`.

## Toolchain Requirements

- **Rust** stable, edition 2021
- **WASM target**: `wasm32-unknown-unknown` (install via `rustup target add wasm32-unknown-unknown`)
- **mise** for task runner (optional; commands work directly via cargo)
- **dprint** for `mise run fmt` (optional)

## Key Dependencies

| Crate | Role |
|---|---|
| `dprint-core` 0.67 | Formatting IR, printer, WASM bridge |
| `logos` 0.14 | Lexer generation |
| `clap` 4.5 (derive) | CLI argument parsing |
| `anyhow` | Error handling (`Result<T>` = `anyhow::Result<T>`) |
| `serde` / `serde_json` | Configuration serialization |
| `glob` | File glob expansion in CLI |
| `dprint-development` 0.10 (dev) | Spec-based test runner |

## Code Conventions & Patterns

### Error Handling
- `anyhow::Result<T>` throughout the library. No custom error types.
- The parser silently skips unrecognized tokens rather than erroring.

### Naming
- Module files use snake_case (`gen_command.rs`, `format_text.rs`).
- Constants are `UPPER_SNAKE_CASE` (`BLOCK_OPENERS`, `KNOWN_KEYWORDS`, `SORTABLE_COMMANDS`).
- AST types are simple structs/enums with public fields — no builder patterns.

### Formatting Pipeline Pattern
The core pattern is: **parse → generate IR → let dprint-core format**. The IR is built using `dprint_core::formatting::PrintItems` with helpers from `ir_helpers`. Key IR constructs:
- `Signal::NewLine`, `Signal::SpaceOrNewLine` for layout decisions
- `ir_helpers::new_line_group(items)` for grouping
- `ir_helpers::with_indent(items)` for indentation

### Block Indentation
`gen_file.rs` tracks indent level by matching command names (case-insensitive) against:
- `BLOCK_OPENERS`: `if`, `foreach`, `while`, `function`, `macro`, `block`
- `BLOCK_MIDDLES`: `elseif`, `else`
- `BLOCK_CLOSERS`: `endif`, `endforeach`, `endwhile`, `endfunction`, `endmacro`, `endblock`

### Command Formatting
`gen_command.rs` attempts single-line layout first (`try_single_line`), falling back to multi-line with dprint indentation. Keyword casing is driven by the `KNOWN_KEYWORDS` list.

### Configuration
All config fields have sensible defaults (see `src/configuration/types.rs`):
- `line_width`: 80, `indent_width`: 4, `use_tabs`: false
- `command_case`: Lower, `keyword_case`: Upper
- `closing_paren_newline`: true, `sort_lists`: false
- `max_blank_lines`: 1, `space_before_paren`: false

Config keys use **camelCase** in the dprint plugin interface (`lineWidth`, `indentWidth`, `commandCase`) — see `src/configuration/resolve.rs`.

### Public API
The library exposes exactly:
```rust
pub use configuration::{resolve_config, CaseStyle, Configuration, NewLineKind};
pub use format_text::format_text;
```

`format_text` returns `Ok(None)` when input is already formatted (idempotency guarantee).

## Testing

### Two Test Suites

1. **Spec tests** (`tests/integration.rs`) — uses `dprint-development` framework.
   - Spec files in `tests/specs/*.txt`.
   - Format: `== test name ==` header, input text, `[expect]` marker, expected output.
   - Per-spec config overrides via JSON in spec headers.
   - Verifies idempotency (`format_twice: true`).
   - Auto-fix failing specs: `FIX=1 cargo test`.

2. **Golden file tests** (`tests/formatter_tests.rs`) — custom file-pair walker.
   - Pairs: `tests/formatter/**/*.in.cmake` → `*.out.cmake`.
   - Optional `### {json}` header on line 1 of `.in.cmake` for per-test config.
   - Panics/parse errors are caught and skipped (not failures).
   - Regressions go in `tests/formatter/issues/NNNN_description.{in,out}.cmake`.

### Running Tests

```sh
cargo test                  # all tests
cargo test test_specs       # spec tests only
cargo test test_formatter   # golden file tests only
FIX=1 cargo test test_specs # auto-fix spec expectations
```

### Adding Tests

- **New spec**: Add a `== name ==` block to the appropriate `tests/specs/*.txt` file.
- **New golden file**: Create `tests/formatter/name.in.cmake` (input) and `name.out.cmake` (expected output). For regressions, use `tests/formatter/issues/NNNN_description.{in,out}.cmake`.
- Config overrides for golden files: add `### {"key": "value"}` as the first line of the `.in.cmake` file.

## Linting

- **Clippy**: `cargo clippy -- -D warnings` (all warnings are errors).
- **rustfmt**: `cargo fmt --check`. No `.rustfmt.toml` — default rustfmt config applies.
- No CI pipeline is configured; checks are run locally via `mise run check`.
