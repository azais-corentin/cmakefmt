# Repository Guidelines

## Project Overview

`cmakefmt` is a Rust (edition 2024) formatter for CMake source files.

Workspace crates:
- `crates/cmakefmt` — core formatter library
- `crates/cmakefmt-cli` — CLI binary (`cmakefmt`)
- `crates/cmakefmt-dprint` — dprint WASM plugin (`cdylib`, `wasm32-unknown-unknown`)

Primary API contract:
- `format_text(path, input, config) -> anyhow::Result<Option<String>>`
- `Ok(None)` means “already formatted” (no write needed)

## Architecture & Data Flow

The formatter is a one-pass pipeline. Layout decisions are made before printing (no printer backtracking).

```text
input
  ├─ bypass gates (disableFormatting / ignorePatterns / ignoreCommands)
  ├─ normalize (BOM, line endings, CR handling)
  ▼
lexer (logos)           crates/cmakefmt/src/parser/token.rs
  ▼
parser                  crates/cmakefmt/src/parser/parse.rs
  ▼
AST                     crates/cmakefmt/src/parser/ast.rs (span-based)
  ▼
IR generation           crates/cmakefmt/src/generation/gen_file.rs
                         + gen_command.rs + signatures.rs
  ▼
printer                 crates/cmakefmt/src/printer.rs
  ▼
post-process            crates/cmakefmt/src/post_process.rs
  ▼
finalize                crates/cmakefmt/src/format_text.rs
```

Design constraints you must preserve:
- AST carries byte `Span`s; source text is sliced on demand.
- Printer is custom linear rendering (`PrintItems`/`Signal`) and does not perform wrapping strategy.
- Cross-command alignment/comment reflow happen in text post-processing.
- Inline pragma behavior (`# cmakefmt: off/on/skip/push/pop`) is implemented across pipeline stages and must stay consistent.

## Key Directories

```text
crates/
  cmakefmt/
    src/
      format_text.rs          Pipeline entry/orchestrator
      parser/                Lexer + parser + AST
      generation/            Command/file generation and command signatures
      printer.rs             IR renderer
      post_process.rs        Alignment/comment reflow
      configuration/         Config schema and loading/merge
      lib.rs                 Public re-exports
    tests/
      formatter_tests.rs     Fixture harness (rstest)
      formatter/             *.in.cmake / *.out.cmake fixture corpus
    benches/
      formatter_fixtures_bench.rs
  cmakefmt-cli/src/main.rs    CLI workflow + modes + config resolution
  cmakefmt-cli/src/trace_summary.rs
  cmakefmt-dprint/src/lib.rs  dprint plugin bridge

docs/
  specs/                      Normative behavior specs (source of truth)
  features.json               Feature status registry
  analysis.md                 Spec/fixture/implementation gap analysis
  plan.md                     Implementation roadmap

scripts/
  collect_benchmark_results.py
  append_benchmark_history.py
```

## Development Commands

```bash
# Build
mise run build
cargo build --target wasm32-unknown-unknown
mise run build:release

# Test
mise run test
cargo test --test formatter_tests
mise run test:filter -- 04_casing

# Lint/format
mise run check

# Bench
mise run bench
```

## Code Conventions & Common Patterns

### Error handling and API behavior
- Use `anyhow::Result` in core/CLI flows.
- Parse failures should report truthful position (`line:col`) context.
- Do not use `unwrap()` outside tests.
- Preserve `format_text` idempotency signal semantics (`Option<String>`).

### Module boundaries
- Keep modules private by default; expose public API via `crates/cmakefmt/src/lib.rs`.
- Prefer `pub(crate)` for internal cross-module access.

### Configuration patterns
- Canonical config type lives in `configuration/types.rs`.
- Config loading/merge logic lives in `configuration/load.rs` (including `extends`, aliases, precedence).
- Both `camelCase` and `snake_case` keys are accepted.
- Config discovery walks upward from target file path.

### Formatting engine patterns
- `generation/gen_command.rs` is the complexity hotspot for wrapping, casing, sorting, and argument layout.
- `generation/signatures.rs` encodes built-in command signatures (`spec!` macro, keyword typing).
- Preserve verbatim behavior for bracket args/comments and multiline quoted strings.

### CLI/plugin integration
- CLI (`crates/cmakefmt-cli/src/main.rs`) should remain a thin orchestration layer around core formatting.
- dprint plugin (`crates/cmakefmt-dprint/src/lib.rs`) maps dprint config/settings to core `Configuration` and delegates to `format_text`.

## Important Files

| File                                            | Why it matters                                                 |
| ----------------------------------------------- | -------------------------------------------------------------- |
| `crates/cmakefmt/src/format_text.rs`            | End-to-end pipeline contract and bypass/finalization semantics |
| `crates/cmakefmt/src/generation/gen_file.rs`    | File-level generation and pragma/block handling                |
| `crates/cmakefmt/src/generation/gen_command.rs` | Command-level formatting rules (largest logic surface)         |
| `crates/cmakefmt/src/generation/signatures.rs`  | Built-in command signature model                               |
| `crates/cmakefmt/src/configuration/load.rs`     | Config discovery, parsing, merge precedence                    |
| `crates/cmakefmt/src/post_process.rs`           | Alignment and comment reflow                                   |
| `crates/cmakefmt/tests/formatter_tests.rs`      | Fixture-driven correctness + idempotency checks                |
| `crates/cmakefmt-cli/src/main.rs`               | CLI modes (`--check`, `--diff`, `--write`, trace flags)        |
| `docs/specs/README.md`                          | Spec index; first stop before behavior changes                 |
| `docs/specs/appendix-e-interactions.md`         | Global option interaction and precedence ordering              |

## Runtime & Tooling Preferences

- Rust stable, edition 2024
- Cargo workspace (`resolver = 3`)
- WASM target required: `wasm32-unknown-unknown`
- Task runner: `mise` (`mise.toml` defines canonical aliases)
- Dev environment: Nix (`devenv.nix`, includes Rust + Python/uv)
- Release tuning includes `release-wasm` profile (`opt-level = "s"`)
- No custom `rustfmt.toml` / `clippy.toml`; rely on defaults + `clippy -D warnings`

## Testing & QA

### Test model
- Fixture-based integration tests using `rstest`:
  - Input: `tests/formatter/**/*.in.cmake`
  - Expected: sibling `*.out.cmake`
- `formatter_tests.rs` validates:
  1. expected output match
  2. idempotency (second format must be no-op)
  3. no orphan `.out.cmake` files
- Failure output includes readable unified diffs (including invisible-char highlighting).

### Benchmark model
- Criterion benchmark in `crates/cmakefmt/benches/formatter_fixtures_bench.rs`.
- Benchmark history helpers in `scripts/` are used by `.github/workflows/benchmark-fixtures.yml`.

## Specs and Documentation Authority

When behavior is unclear, follow this order:
1. `docs/specs/` (normative source of truth)
2. Fixtures under `crates/cmakefmt/tests/formatter/`
3. Current implementation

High-value spec docs:
- `docs/specs/README.md` — full index
- `docs/specs/appendix-d-cli.md` — CLI contract + exit codes
- `docs/specs/appendix-e-interactions.md` — global ordering/precedence
- `docs/specs/13-inline-pragmas.md` — pragma stack semantics
- `docs/specs/15-config-meta.md` — config `extends` behavior and constraints

## Practical Assistant Workflow

Before non-trivial changes:
1. Read relevant spec section(s).
2. Locate existing implementation pattern in same subsystem.
3. Update all affected call sites/types in one cutover.
4. Run targeted test command(s) for touched behavior.
5. For performance-sensitive changes, run fixture benchmark.

Avoid:
- Editing fixture expected files unless explicitly requested.
- Introducing alternate parallel conventions when existing patterns already solve the problem.
- Treating passing compile as sufficient without fixture/idempotency validation.