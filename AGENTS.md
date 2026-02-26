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
| `generation::signatures` | Static command spec database (~66 commands): `CommandSpec`, `CommandKind`, `lookup_command()` |
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
	formatter_tests.rs      Golden file (in/out) test runner (sole test suite)
	formatter/              Golden file pairs (*.in.cmake / *.out.cmake)
		builtin/              Built-in CMake command tests (~100 commands)
		basic/                Basic formatting tests
		casing/               commandCase / keywordCase variations
		comments/             Comment handling (line, bracket, inline)
		custom_commands/      Custom command formatting tests
		disabled_formatting/  gersemi:on/off directive tests
		edge_cases/           Boundary and unusual inputs
		indentation/          indentWidth / useTabs tests
		issues/               Numbered regression tests (NNNN_description.*)
		sorting/              sortLists=true cases
		wrapping/             lineWidth / line breaking tests
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

- **Rust** stable, edition 2024
- **WASM target**: `wasm32-unknown-unknown` (install via `rustup target add wasm32-unknown-unknown`)
- **Nix dev environment**: `devenv.nix` + `devenv.yaml` + `.envrc` for automatic shell activation via direnv
- **mise** for task runner (optional; commands work directly via cargo)
- **dprint** for `mise run fmt` (optional)

## Key Dependencies

| Crate | Role |
|---|---|
| `dprint-core` 0.67 | Formatting IR, printer, WASM bridge |
| `logos` 0.16 | Lexer generation |
| `clap` 4.5 (derive) | CLI argument parsing |
| `anyhow` | Error handling (`Result<T>` = `anyhow::Result<T>`) |
| `serde` / `serde_json` | Configuration serialization |
| `glob` | File glob expansion in CLI |

## Prefer CLI Commands Over Manual Edits

When a CLI command achieves the same result as a manual file edit, you **MUST** use the CLI command. It is safer, handles edge cases, and keeps lock files / metadata in sync.

| Instead of manually editing | Use |
|---|---|
| `Cargo.toml` `[dependencies]` | `cargo add <crate>`, `cargo add <crate> --dev`, `cargo add <crate> --features <f>` |
| `Cargo.toml` removing deps | `cargo remove <crate>` |
| `Cargo.toml` version bumps | `cargo add <crate>@<version>` |
| Rust toolchain / target setup | `rustup target add <target>`, `rustup component add <component>` |

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
`gen_command.rs` dispatches via `lookup_command()`:
- **Known commands**: Builds `FormattedArg` list, optionally sorts, tries `try_single_line()` first (checks width, group size ≤4), falls back to `gen_known_multi_line()` or `gen_condition_multi_line()`.
- **Unknown commands**: `gen_unknown_command()` preserves content verbatim between parens, only normalizing the command name case.
- **Condition syntax** (`if`/`while`/`elseif`): Special-cased with `ConditionSyntax` variant, uses flow-wrap layout.

### Command Spec System
`signatures.rs` defines `CommandSpec` structs for ~66 standard CMake commands:
- `front_positional` / `back_positional`: positional arg counts
- `keywords`: typed keyword list (`Option`, `OneValue`, `MultiValue`)
- `sections`: sub-keyword groups within a keyword
- `pair_keywords`: emit as `KEY VALUE` pairs (e.g., `PROPERTIES`)
- `flow_keywords`: flow-wrap layout (e.g., `COMMAND`)
- A `spec!{}` macro reduces boilerplate for static spec definitions.

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

### Golden File Tests (`tests/formatter_tests.rs`)

The sole test suite. A custom file-pair walker that:
1. Recursively walks `tests/formatter/**/*.in.cmake` (sorted for determinism)
2. Pairs each `.in.cmake` with the corresponding `.out.cmake`
3. Parses optional `### <config>` header from line 1 of `.in.cmake`
4. Calls `format_text()`, asserts output matches `.out.cmake`
5. Verifies idempotency (formatting the output again returns `Ok(None)`)
6. Panics and parse errors are silently skipped (not failures)

### Config Header Format

First line of `.in.cmake` can override config. Two syntaxes, auto-detected:

**JSON (preferred):** `### {"sortLists": true, "lineWidth": 120, "commandCase": "preserve"}`
- Keys: `commandCase`, `keywordCase`, `lineWidth`, `indentWidth`, `useTabs`, `sortLists`, `closingParenNewline`
- Values: strings for case styles (`"upper"`, `"lower"`, `"preserve"`), numbers, booleans

**Gersemi-style fallback:** `### {indent: tabs, line_length: 80}`
- Keys: `line_length`, `indent` (`tabs` or a number), `commandCase`/`command_case`, `keywordCase`/`keyword_case`, `sortLists`/`sort_lists`

No header → `Configuration::default()` is used.

### Running Tests

```sh
cargo test                    # all tests
cargo test test_formatter     # golden file tests only
```

### Adding Tests

- **Feature test**: Create `tests/formatter/<subdir>/name.in.cmake` (input) and `name.out.cmake` (expected output).
- **Regression**: Create `tests/formatter/issues/NNNN_description.in.cmake` + `.out.cmake` (zero-padded issue number).
- **With config**: Add `### {"key": value}` as line 1 of the `.in.cmake` file.
- To update a golden file, manually edit the `.out.cmake`.

## Linting

- **Clippy**: `cargo clippy -- -D warnings` (all warnings are errors).
- **rustfmt**: `cargo fmt --check`. No `.rustfmt.toml` — default rustfmt config applies.
- No CI pipeline is configured; checks are run locally via `mise run check`.
