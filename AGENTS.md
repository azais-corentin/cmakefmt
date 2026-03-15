# Repository Guidelines

## Project Overview

`cmakefmt` is a Rust (edition 2024) formatter for CMake source files.

Workspace crates:

- `crates/cmakefmt` — core formatter library (package: `cmakefmt-rs`, lib: `cmakefmt`)
- `crates/cmakefmt-cli` — CLI binary (`cmakefmt`)
- `crates/cmakefmt-dprint` — dprint WASM plugin (`cdylib`, `wasm32-unknown-unknown`, not published to crates.io)

Primary API contract:

- `format_text(path, input, config) -> anyhow::Result<Option<String>>`
- `Ok(None)` means "already formatted" (no write needed)
- `Ok(Some(text))` means the input was reformatted
- Neither the CLI nor the dprint plugin contain formatting logic; both delegate to `format_text`.

## Architecture & Data Flow

The formatter is a one-pass pipeline. Layout decisions are made before printing (no printer backtracking).

```text
input
  ├─ bypass gates (disableFormatting / ignorePatterns / ignoreCommands)
  ├─ normalize (BOM strip, bare-CR → \n, line-ending detection)
  ▼
lexer (logos)           crates/cmakefmt/src/parser/token.rs
  ▼
parser                  crates/cmakefmt/src/parser/parse.rs
  ▼
AST                     crates/cmakefmt/src/parser/ast.rs (span-based)
  ▼
config resolution       pragma push/pop from file header lines
  ▼
IR generation           crates/cmakefmt/src/generation/gen_file.rs
                         + gen_command.rs + signatures.rs
  ▼
printer                 crates/cmakefmt/src/printer.rs
  ▼
post-process            crates/cmakefmt/src/post_process.rs
  ▼
finalize                crates/cmakefmt/src/format_text.rs
  (whitespace, final newline, bare-CR restore)
```

Key data flow types:

```text
&str (source) → File (AST with byte Spans) → PrintItems (linear IR) → String (rendered) → Cow<str> (post-processed) → String (final)
```

Design constraints you must preserve:

- AST carries byte `Span`s; source text is sliced on demand. Spans remain valid only for the original source string.
- Printer is a custom linear renderer (`PrintItems`/`Signal`). It does not perform wrapping strategy — wrapping decisions happen during IR generation.
- Cross-command alignment and comment reflow happen in text post-processing (on the rendered `String`, not the AST).
- Inline pragma behavior (`# cmakefmt: off/on/skip/push/pop`) is implemented across multiple pipeline stages (format_text, gen_file, post_process) and must stay consistent.

## Key Directories

```text
crates/
  cmakefmt/
    src/
      format_text.rs          Pipeline entry/orchestrator
      parser/                 Lexer (logos) + recursive-descent parser + AST types
      generation/             File/command IR generation + command signature database
      printer.rs              Custom IR renderer (replaces dprint_core)
      post_process.rs         Alignment passes + comment reflow (string-level)
      configuration/          Config schema (types.rs) + loading/merge (load.rs)
      instrumentation.rs      Tracing span name constants
      util.rs                 Line-ending detection helper
      lib.rs                  Public re-exports (narrow API surface)
    tests/
      formatter_tests.rs      Fixture harness (rstest)
      formatter/              *.in.cmake / *.out.cmake fixture corpus
    benches/
      formatter_fixtures_bench.rs
  cmakefmt-cli/
    src/main.rs               CLI workflow + modes + config resolution
    src/trace_summary.rs      Chrome trace → structured summary
  cmakefmt-dprint/
    src/lib.rs                dprint plugin bridge (config bridge + WASM handler)

docs/
  specs/                      Normative behavior specs (source of truth)
  website/                    VitePress user-facing documentation site

scripts/
  collect_benchmark_results.py   CI: reads Criterion JSON → per-commit entry
  append_benchmark_history.py    CI: appends entry to benchmark history file
```

## Development Commands

```bash
# Build
mise run build                         # all crates (debug, includes WASM)
mise run build:release                 # optimized WASM plugin (release-wasm profile)

# Test
mise run test                          # all tests
cargo test --test formatter_tests      # fixture tests only
mise run test:filter -- 04_casing      # fixture tests matching substring

# Lint/format
mise run check                         # clippy -D warnings + cargo fmt --check

# Bench
mise run bench                         # Criterion benchmark (formatter_fixtures)

# WASM size
mise run size                          # build release-wasm + report binary size

# Pre-commit
mise run pre-commit                    # hk hooks: fmt, clippy, dprint, etc.
```

## Code Conventions & Common Patterns

### Error handling and API behavior

- Use `anyhow::Result` in core/CLI flows.
- Parse failures report truthful 1-based `line:col` position context.
- Do not use `unwrap()` outside tests.
- Preserve `format_text` idempotency signal semantics: `Ok(None)` = no change, `Ok(Some(_))` = changed.

### Module boundaries

- Keep modules private by default; expose public API via `crates/cmakefmt/src/lib.rs`.
- Prefer `pub(crate)` for internal cross-module access.
- AST types have `pub` fields — they are read by the generation layer, not mutated.

### Configuration patterns

- Canonical config type: `configuration/types.rs` — `Configuration` struct (~40 fields) + `CommandConfiguration` (mirrors all fields as `Option<T>` for per-command overrides).
- `effective_config_for_command()` applies per-command overrides field-by-field. New fields added to `Configuration` must also be handled in this method.
- Config loading/merge: `configuration/load.rs` — `extends` chain support (recursive, cycle detection, depth limit 32), `CanonicalKey` enum for key normalization.
- Both `camelCase` and `snake_case` keys are accepted everywhere (TOML, JSON, inline pragmas).
- Config discovery: walks parent directories from target file for `.cmakefmt.toml` (preferred) or `cmakefmt.toml`.
- Config layering precedence: default → TOML file (with extends chain) → per-command overrides → inline pragma push/pop.
- Inline config header parsing supports three syntaxes (tried in order): JSON, relaxed TOML, gersemi key=value.

### Formatting engine patterns

- `generation/gen_command.rs` is the complexity hotspot (~5500 lines): wrapping, casing, sorting, argument layout, genex handling.
- `generation/signatures.rs` encodes built-in command signatures for ~50 CMake commands via `spec!` macro. Lookup is `lookup_command(name) -> Option<CommandKind>`.
- Preserve verbatim behavior for bracket args/comments and multiline quoted strings.
- Printer uses `u8` for indent level (saturating arithmetic — deeply nested CMake silently stops indenting at 255).

### Post-processing patterns

- `post_process_alignments()` returns `Cow<str>` (borrowed when no changes needed).
- Three passes: (1) comment reflow, (2) align consecutive set() values, (3) align trailing comments.
- Operates on rendered text, not AST — unaware of CMake semantics.
- Pragma push/pop is tracked per-line independently from the generation stage.

### Instrumentation

- Tracing span names follow `cmakefmt.<subsystem>.<stage>` pattern (constants in `instrumentation.rs`).
- CLI supports `--trace-output` for Chrome JSON traces and `--trace-summary-output` for structured summaries.

### CLI/plugin integration

- CLI (`crates/cmakefmt-cli/src/main.rs`) is a thin orchestration layer. Config is cached per parent directory via `HashMap<PathBuf, ConfigLoadResult>`.
- Exit codes: `0` = success, `1` = `--check` mode found unformatted files, `2` = usage/parse/IO error.
- Format errors on individual files do not abort the run — processing continues, errors accumulate, exit 2 at end.
- dprint plugin (`crates/cmakefmt-dprint/src/lib.rs`): config bridge is always compiled (testable on native), WASM handler is `#[cfg(target_arch = "wasm32")]` gated. Uses `load_from_json_map` instead of `load_from_toml_path`.

## Important Files

| File                                            | Why it matters                                                      |
| ----------------------------------------------- | ------------------------------------------------------------------- |
| `crates/cmakefmt/src/format_text.rs`            | End-to-end pipeline contract and bypass/finalization semantics      |
| `crates/cmakefmt/src/generation/gen_file.rs`    | File-level generation: indentation, block stack, pragma handling    |
| `crates/cmakefmt/src/generation/gen_command.rs` | Command-level formatting rules (largest logic surface, ~5500 lines) |
| `crates/cmakefmt/src/generation/signatures.rs`  | Built-in command signature model (~50 commands, `spec!` macro)      |
| `crates/cmakefmt/src/configuration/types.rs`    | `Configuration` + `CommandConfiguration` + all enum types           |
| `crates/cmakefmt/src/configuration/load.rs`     | Config discovery, parsing, merge, extends, key normalization        |
| `crates/cmakefmt/src/printer.rs`                | Custom IR renderer (PrintItems/Signal), replaces dprint_core        |
| `crates/cmakefmt/src/post_process.rs`           | Alignment and comment reflow (string-level passes)                  |
| `crates/cmakefmt/tests/formatter_tests.rs`      | Fixture-driven correctness + idempotency checks                     |
| `crates/cmakefmt-cli/src/main.rs`               | CLI modes (`--check`, `--diff`, `--write`, trace flags)             |
| `docs/specs/README.md`                          | Spec index + summary table of all ~40 config options                |
| `docs/specs/appendix-c-cascade-algorithm.md`    | Cascade wrapping algorithm (core algorithmic spec)                  |
| `docs/specs/appendix-e-interactions.md`         | Global option interaction and precedence ordering                   |

## Runtime & Tooling Preferences

- Rust stable, edition 2024
- Cargo workspace (`resolver = 3`)
- WASM target required: `wasm32-unknown-unknown` (must be installed in toolchain)
- Task runner: `mise` (`mise.toml` defines canonical aliases)
- Dev environment: Nix (`devenv.nix`, includes Rust + Python/uv + wasm32 target)
- Build profiles: `bench` (debug=2), `release` (lto=true), `release-wasm` (inherits release, `opt-level = "s"`)
- Code formatting: `dprint` orchestrates all file types — `rustfmt --edition 2024` for `.rs` via exec plugin
- No custom `rustfmt.toml` / `clippy.toml`; rely on defaults + `clippy -D warnings`
- Conventional commits enforced by cocogitto (`cog verify`), changelog via git-cliff
- Pre-commit hooks: `hk` (Pkl config) — cargo-fmt, cargo-clippy, dprint, actionlint, etc.
- JS runtime/tooling: use `bun` / `bun x` — never `node`, `npm`, or `npx`

## Testing & QA

### Test model

- Fixture-based integration tests using `rstest`:
  - Input: `tests/formatter/**/*.in.cmake`
  - Expected: sibling `*.out.cmake`
- `formatter_tests.rs` validates:
  1. expected output match (with colored unified diff on failure, including invisible-char highlighting)
  2. idempotency (second format must return `Ok(None)`)
  3. no orphan `.out.cmake` files
- Fixture config: `load_from_toml_path(in_path)` walks parent dirs. Place a `.cmakefmt.toml` in a fixture subdirectory for non-default config. Some fixtures use inline pragmas instead.
- rstest encodes fixture paths in test names (slashes → `__`): `cargo test -- 04_casing` filters by substring.

### Fixture categories

```text
01_wrapping/    02_indentation/  03_blank_lines/  04_casing/
05_parens_spacing/  06_comments/  07_line_endings/  08_whitespace/
09_alignment/   10_genex/        11_per_command/  12_sorting/
13_pragmas/     14_flow_control/ 15_config_meta/  16_suppression/
17_interactions/ 99_compliance/  issues/          respositories/
```

Note: `respositories/` is an intentional typo that is hardcoded in the benchmark path — do not rename without updating `benches/formatter_fixtures_bench.rs`.

### Benchmark model

- Criterion benchmark in `crates/cmakefmt/benches/formatter_fixtures_bench.rs`.
- Uses `respositories/XNNPACK/CMakeLists.in.cmake` (large real-world file), reports throughput in bytes/sec.
- Benchmark history tracked on a git orphan branch via CI scripts in `scripts/`.

## Specs and Documentation Authority

When behavior is unclear, follow this order:

1. `docs/specs/` (normative source of truth)
2. Fixtures under `crates/cmakefmt/tests/formatter/`
3. Current implementation

High-value spec docs:

- `docs/specs/README.md` — full index + config option summary table
- `docs/specs/appendix-c-cascade-algorithm.md` — cascade wrapping algorithm (Steps 0-3)
- `docs/specs/appendix-d-cli.md` — CLI contract + exit codes
- `docs/specs/appendix-e-interactions.md` — global pipeline order (17 steps) + pairwise interaction rules
- `docs/specs/13-inline-pragmas.md` — pragma stack semantics (cross-cutting)
- `docs/specs/15-config-meta.md` — config `extends` behavior and merge strategy

## Practical Assistant Workflow

Before non-trivial changes:

1. Read relevant spec section(s).
2. Locate existing implementation pattern in same subsystem.
3. Update all affected call sites/types in one cutover.
4. Run targeted test command(s) for touched behavior.
5. For performance-sensitive changes, run fixture benchmark.
6. For changes to the documentation website (`docs/website/`), use Puppeteer to visually verify the rendered result in a browser.

Avoid:

- Editing fixture expected files unless explicitly requested.
- Introducing alternate parallel conventions when existing patterns already solve the problem.
- Treating passing compile as sufficient without fixture/idempotency validation.
