# Repository Guidelines

## Project Overview

`cmakefmt` is a Rust (edition 2024) formatter for CMake sources (`CMakeLists.txt`, `*.cmake`). It ships as:

1. Native CLI binary: `cmakefmt`
2. WASM plugin for dprint: `cmakefmt.wasm`

Core behavior: parse input CMake, generate dprint IR, format deterministically, and return `None` when input is already formatted.

## Architecture & Data Flow

Primary pipeline (`src/format_text.rs`):

```text
input
  -> strip BOM
  -> parser::parse()                     (src/parser/parse.rs)
  -> gen_file() / gen_command()          (src/generation/*)
  -> dprint_core::formatting::format()
  -> compare with input (None if unchanged)
```

Key architectural patterns:

- **Span-based AST** (`src/parser/ast.rs`): nodes keep byte spans into original source for low-copy transformations.
- **Recursive-descent parser** over Logos tokens (`src/parser/token.rs`, `src/parser/parse.rs`) with `anyhow::bail!` on parse failures.
- **Signature-driven formatting** (`src/generation/signatures.rs` + `src/generation/gen_command.rs`): known commands use `CommandSpec`; unknown commands take preservation-oriented fallback formatting.
- **Block indentation tracking** (`src/generation/gen_file.rs`): `if`/`elseif`/`else`/`endif`, loops, functions/macros handled through opener/middle/closer sets.
- **Dual runtime entrypoints**:
  - Library API (`src/lib.rs`) for host usage
  - WASM plugin (`src/wasm_plugin.rs`) behind `#[cfg(target_arch = "wasm32")]`

## Key Directories

- `src/bin/cmakefmt.rs` — CLI entrypoint (stdin/files, check/write modes, glob expansion)
- `src/format_text.rs` — top-level format orchestrator
- `src/parser/` — lexer, AST, parser
- `src/generation/` — file-level and command-level IR generation
- `src/configuration/` — config types, header parsing/loading, dprint resolve
- `tests/formatter_tests.rs` — fixture runner + helper unit tests
- `tests/formatter/` — fixture corpus (`*.in.cmake` / `*.out.cmake`)

## Development Commands

Prefer `mise` tasks (`mise.toml`) for standard workflows:

- `mise run test` — run full test suite (`cargo test`)
- `mise run check` — clippy + rustfmt check
- `mise run build` — build WASM plugin (debug)
- `mise run release` — build WASM plugin (optimized)
- `mise run fmt` — run dprint using built WASM plugin

Direct cargo equivalents:

- `cargo test`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`
- `cargo build --target wasm32-unknown-unknown`
- `cargo build --release --target wasm32-unknown-unknown`

Targeted test examples:

- Run fixture integration test only: `cargo test --test formatter_tests test_formatter_files -- --exact`
- Run a single test binary: `cargo test --test formatter_tests`

## Code Conventions & Common Patterns

### Error handling

- Use `anyhow::Result` across fallible paths.
- Parser failures use `anyhow::bail!`; avoid ad-hoc error type proliferation unless required.

### Naming and structure

- Standard Rust naming (`snake_case`, `PascalCase`).
- Keep module names aligned to responsibility (`gen_file.rs`, `gen_command.rs`, `signatures.rs`).
- Fixture naming is strict: `<name>.in.cmake` and `<name>.out.cmake`.

### Formatting behavior patterns

- Command names are normalized via case-insensitive lookup in `lookup_command()`.
- `CommandSpec` + `KwType` drive keyword/section/pair/flow layout behavior.
- Unknown commands should preserve intent and avoid destructive rewrites.
- Respect config defaults from `src/configuration/types.rs`; per-input header overrides are supported.

### Conditional compilation

- WASM-specific functionality lives in `src/wasm_plugin.rs` and is `wasm32`-gated.

## Important Files

- `src/generation/gen_command.rs` — most formatter logic and highest-change-risk surface.
- `src/generation/signatures.rs` — static CMake command knowledge base.
- `src/parser/parse.rs` — parsing correctness and edge-case handling.
- `src/configuration/load.rs` — header/config parsing, key mapping, diagnostics.
- `src/bin/cmakefmt.rs` — CLI behavior contract (`--check`, `--write`, stdin semantics).
- `src/configuration/resolve.rs` — dprint file matching (`*.cmake`, `CMakeLists.txt`).

## Runtime/Tooling Preferences

- Language/runtime: Rust stable toolchain.
- Task runner: `mise`.
- Dev environment: `devenv` + `direnv` (`.envrc`, `devenv.nix`, `devenv.yaml`).
- Plugin workflow assumes dprint compatibility and wasm artifact output at:
  - `target/wasm32-unknown-unknown/release/cmakefmt.wasm`

## Testing & QA

Test setup (`tests/formatter_tests.rs`):

1. **Fixture integration test** (`test_formatter_files`)
   - Discovers all fixture pairs under `tests/formatter/`
   - Asserts formatted output exactly matches `.out.cmake`
   - Asserts **idempotency** (reformatting expected output yields no change)

2. **Helper unit tests**
   - `invisible_diff_tests`: whitespace/invisible-char diff rendering
   - `config_header_tests`: JSON + gersemi-style header parsing, diagnostics behavior

Fixture config overrides:

- First-line header in `.in.cmake`:
  - Example: `### {"indentWidth": 4, "lineWidth": 120}`
- Parsed through configuration loader and included in failure diagnostics.

QA expectations for formatter changes:

- Run targeted fixture test first, then full `cargo test`.
- Do not update fixtures unless behavior changes are explicitly intended.
- If output differs only in invisible characters, inspect diff helper output before changing logic.

## Scripts/Docs Notes

- No large custom script surface was found; `mise.toml` is the authoritative automation entry.
- `FEATURES.md` is roadmap/context, not operational source of truth.
- For day-to-day coding decisions, prioritize `src/*`, `tests/formatter_tests.rs`, `Cargo.toml`, and `mise.toml`.