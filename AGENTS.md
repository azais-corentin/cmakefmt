# AGENTS.md

## Commands

Use Cargo for all routine development tasks:

- Build: `cargo build`
- Run all tests: `cargo test`
- Run a single test file: `cargo test --test {test_name}`
- Run a specific test: `cargo test {test_function_name}`
- Lint: `cargo clippy -- -D warnings`
- Format check: `cargo fmt -- --check`

## Project Structure

This project is a Rust codebase targeting edition 2024 and built with Cargo.

- `src/` — implementation source code
- `docs/specs/` — markdown specification files, organized as one file per feature area
- `tests/formatter/` — input/expected-output fixture pairs used by formatter tests

Behavioral requirements live in `docs/specs/` and are authoritative. Treat those spec files as the source of truth for expected behavior.

The files under `tests/formatter/` are pre-written acceptance fixtures and must not be modified.

## Code Style & Conventions

- All public functions must have doc comments.
- Use `thiserror` for error types; do not use `anyhow` in library code.
- Prefer exhaustive `match` statements over `if let` chains when branching on enums or other structured state.
- Do not use `unwrap()` outside of tests.
- Follow the existing naming conventions already established in the codebase.

## Testing Philosophy

- Tests are pre-written and are immutable acceptance criteria.
- Never modify test files or test fixture data.
- The implementation is correct when all relevant tests pass.
- If a test appears incorrect, flag the issue in a comment or review note, but do not change the test.

## Specification

- `docs/specs/` is the single source of truth.
- When inferred behavior conflicts with the specification, follow the spec.
- Each spec file covers one feature area and includes examples that clarify expected behavior.

## Boundaries

### Always Do

- Run the relevant test subset after every implementation change.
- Commit after each feature passes its tests.
- Read the relevant spec section before implementing a feature.

### Ask First

- Changing public API signatures
- Adding new dependencies to `Cargo.toml`
- Architectural changes that affect multiple modules

### Never Do

- Modify any file in `tests/`
- Skip running tests before committing
- Implement multiple unrelated features in a single commit
- Use `unsafe` code without explicit approval
