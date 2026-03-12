# Repository Guidelines

## Project Overview

cmakefmt is a CMake source code formatter written in Rust (edition 2024). It ships as three crates in a Cargo workspace:

- **`cmakefmt`** — core formatter library (lexer, parser, IR generation, printer, post-processing, configuration)
- **`cmakefmt-cli`** — standalone CLI binary (`cmakefmt`)
- **`cmakefmt-dprint`** — dprint WASM plugin (`cdylib` targeting `wasm32-unknown-unknown`)

## Architecture & Data Flow

The formatter is a linear pipeline with no backtracking. Entry point: `format_text(path, input, config) -> Result<Option<String>>`. Returns `None` when input is already correctly formatted (idempotency signal — callers skip writes without string comparison).

```
input
  │
  ├─ suppression gates (disableFormatting, ignorePatterns, ignoreCommands)
  ├─ BOM strip, bare-CR normalization, line-ending detection
  │
  ▼
Lex (logos)           crates/cmakefmt/src/parser/token.rs
  │                   Token enum: Space, Newline, LParen, RParen,
  ▼                   BracketArgument, BracketComment, LineComment,
Parse                 QuotedArgument, UnquotedText
  │                   crates/cmakefmt/src/parser/parse.rs
  ▼                   Recursive descent → ast::File
Generate IR           crates/cmakefmt/src/generation/gen_file.rs
  │                   + gen_command.rs + signatures.rs
  ▼                   All layout decisions made here → PrintItems
Print                 crates/cmakefmt/src/printer.rs
  │                   Linear stream renderer, no backtracking → String
  ▼
Post-process          crates/cmakefmt/src/post_process.rs
  │                   Text-level alignment (consecutive set, trailing
  ▼                   comments, comment reflow) — requires multi-line context
Finalize              Back in format_text.rs
  │                   Whitespace trim, final newline, bare-CR restoration
  ▼
output
```

Key design decisions:
- AST nodes store `Span` (byte offsets), not owned strings — source text is sliced on demand throughout the pipeline.
- The printer is a custom replacement for dprint_core's formatter. It has no backtracking or CJK width measurement — all layout decisions are made in the generation layer.
- Post-processing is text-level (not IR-level) because cross-command alignment cannot be expressed per-command.
- Inline `# cmakefmt: push {...}/pop` pragmas allow scoped config overrides within a file. Pragma parsing exists in three places: `format_text.rs`, `gen_file.rs`, and `post_process.rs`.

## Key Directories

```
crates/
  cmakefmt/
    src/
      configuration/    Config types (types.rs) and loading (load.rs)
      generation/       IR generation: gen_file.rs, gen_command.rs, signatures.rs
      parser/           Lexer (token.rs), AST (ast.rs), parser (parse.rs)
      format_text.rs    Pipeline orchestrator — the single public entry point
      post_process.rs   Text-level alignment and comment reflow passes
      printer.rs        PrintItems stream renderer
      lib.rs            Public re-export surface
    tests/
      formatter_tests.rs   Sole integration test (fixture-based harness)
      formatter/            Fixture pairs organized by feature area (19 categories)
    benches/
      formatter_fixtures_bench.rs   Criterion benchmark
  cmakefmt-cli/
    src/main.rs         CLI binary (clap derive)
  cmakefmt-dprint/
    src/lib.rs          dprint WASM plugin bridge
docs/
  specs/                Authoritative behavioral specifications (23 files)
  features.json         Machine-readable feature status registry
  plan.md               Implementation plan (12 phases)
  analysis.md           Gap analysis between spec, fixtures, and implementation
scripts/                Benchmark collection/history Python scripts
```

## Development Commands

```bash
# Build (native)
cargo build

# Build WASM plugin (debug / release)
cargo build --target wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown

# Run all tests
cargo test

# Run the formatter integration test only
cargo test --test formatter_tests

# Filter to specific fixture categories
CMAKEFMT_TEST_FILTER=01_wrapping cargo test --test formatter_tests
CMAKEFMT_TEST_FILTER=06_comments/03_align cargo test --test formatter_tests

# Lint
cargo clippy -- -D warnings

# Format check
cargo fmt -- --check

# Run both lint + format check (via mise)
mise run check

# Benchmarks
cargo bench --bench formatter_fixtures_bench

# Interactive fixture diff viewer (requires fzf)
mise run test-diff
```

## Code Conventions & Patterns

### Error Handling
- `anyhow::Result` throughout the core library and CLI. Parse errors use `anyhow::bail!` with 1-based `line:col` messages.
- `thiserror` for structured error types where needed.
- No `unwrap()` outside of tests.

### Naming & Visibility
- Modules are private by default; public API is re-exported through `lib.rs`.
- Internal types use `pub(crate)` or are private to their module.
- All public functions must have doc comments.

### Configuration
- 42 configuration options defined in `configuration/types.rs` (`Configuration` struct, 30+ fields).
- Per-command overrides via `perCommandConfig` table and `effective_config_for_command()`.
- Loading supports TOML, JSON, CLI flags, and `extends` chains (depth limit 32, cycle detection).
- Both `camelCase` and `snake_case` config key aliases are accepted.
- Config discovery walks up from the formatted file's directory to filesystem root (`.cmakefmt.toml` preferred over `cmakefmt.toml`).

### Command Signatures
- `generation/signatures.rs` contains a static database of CMake built-in command specs using a `spec!` macro.
- `CommandSpec`, `KwType` (Option/OneValue/MultiValue/Group), and `CommandKind` (Known/ConditionSyntax) classify arguments as positional vs keyword.

### Control Flow
- Prefer exhaustive `match` statements over `if let` chains when branching on enums.
- `gen_command.rs` (5341 lines) is the largest and most complex module — handles all argument layout, casing, wrapping, alignment, and sorting logic.

### Patterns to Note
- The `PRAGMA_PREFIX` constant (`"cmakefmt:"`) is defined independently in `format_text.rs`, `gen_file.rs`, and `post_process.rs`.
- `format_text` returns `None` (not unchanged input) when no formatting change occurred.
- Bracket arguments/comments and multiline quoted strings are always preserved verbatim.

## Important Files

| File | Role |
|------|------|
| `crates/cmakefmt/src/format_text.rs` | Pipeline orchestrator — start reading here |
| `crates/cmakefmt/src/generation/gen_command.rs` | Command-level formatting engine (5341 lines) |
| `crates/cmakefmt/src/generation/signatures.rs` | CMake command signature database (2242 lines) |
| `crates/cmakefmt/src/configuration/types.rs` | All config type definitions |
| `crates/cmakefmt/src/configuration/load.rs` | Config loading from all sources (2340 lines) |
| `crates/cmakefmt/src/post_process.rs` | Alignment and comment reflow passes |
| `crates/cmakefmt/src/printer.rs` | Print IR stream renderer |
| `crates/cmakefmt/src/lib.rs` | Public API surface |
| `crates/cmakefmt-cli/src/main.rs` | CLI binary entry point |
| `docs/specs/README.md` | Spec index — summary table of all 42 options |

## Runtime & Tooling

- **Rust edition**: 2024 (stable channel)
- **Build**: Cargo (workspace, resolver v3)
- **Task runner**: `mise` (see `mise.toml` for available tasks)
- **Dev environment**: Nix via `devenv.nix` (provides Rust stable + Python)
- **WASM target**: `wasm32-unknown-unknown` (for the dprint plugin)
- **Release profile**: LTO enabled, `opt-level = "s"` (size-optimized for WASM)
- **CI**: GitHub Actions (benchmark tracking only; no automated test/lint CI)
- **No `rustfmt.toml` or `clippy.toml`** — default Rust formatting, lint enforcement via `cargo clippy -- -D warnings`

### Dependencies (core library)

| Crate | Purpose |
|-------|---------|
| `logos` | Lexer codegen via derive macro |
| `anyhow` | Error handling |
| `serde` + `serde_json` | Config serialization, JSON bridge for dprint |
| `toml` | Config file parsing |
| `glob` | File pattern matching (ignore patterns) |
| `clap` (optional, `cli` feature) | CLI argument parsing |

## Testing & QA

### Test Structure
- **Single integration test**: `crates/cmakefmt/tests/formatter_tests.rs` runs all formatter fixtures in one `#[test]` function.
- **Fixture pairs**: `*.in.cmake` (input) / `*.out.cmake` (expected output) under `crates/cmakefmt/tests/formatter/`.
- **19 fixture categories** numbered `01_wrapping` through `99_compliance`, plus `respositories/` (real-world CMake files).
- **Per-fixture config**: `.cmakefmt.toml` placed alongside fixtures; discovered by `load_from_toml_path` walking upward.

### How It Works
1. Test harness walks `tests/formatter/` for all `.in.cmake` files
2. Pairs each with its `.out.cmake` counterpart
3. Loads config via upward `.cmakefmt.toml` discovery (or defaults)
4. Calls `format_text()`, asserts byte-for-byte equality with expected output
5. Re-formats the output and asserts no change (**idempotency check**)
6. All failures collected and reported in aggregate (not fail-fast)
7. Each `format_text()` call is wrapped in `catch_unwind` — a panic in one fixture doesn't abort the suite

### Filtering
```bash
# Substring match on fixture file path
CMAKEFMT_TEST_FILTER=04_casing cargo test --test formatter_tests
```

`cargo test -- <name>` cannot target individual fixtures — the only narrowing mechanism is `CMAKEFMT_TEST_FILTER`.

### Benchmarks
- Criterion-based benchmark in `crates/cmakefmt/benches/formatter_fixtures_bench.rs`
- Uses the `respositories/XNNPACK/CMakeLists.in.cmake` fixture
- Run: `cargo bench --bench formatter_fixtures_bench`

## Specification

`docs/specs/` is the **single source of truth** for behavioral requirements. When inferred behavior conflicts with the specification, follow the spec.

Spec files cover 16 feature areas plus 6 appendices:
- `01`–`16`: Line width/wrapping, indentation, blank lines, casing, parens/spacing, comments, line endings, whitespace, alignment, generator expressions, per-command config, sorting, inline pragmas, flow control, config meta, suppression
- Appendices: defaults snapshot, example config, cascade algorithm detail, CLI reference, interaction rules (17-step pipeline ordering), keyword dictionary

## Boundaries

### Always Do
- Read the relevant spec section before implementing a feature
- Run the relevant test subset after every implementation change
- Commit after each feature passes its tests

### Ask First
- Changing public API signatures
- Adding new dependencies to `Cargo.toml`
- Architectural changes that affect multiple modules

### Never Do
- Modify any file under `tests/`
- Skip running tests before committing
- Implement multiple unrelated features in a single commit
- Use `unsafe` code without explicit approval
- If a test fixture appears to conflict with the spec, stop and ask the user before continuing
