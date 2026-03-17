# cmakefmt Release Procedure

## Context

- Workspace version: `0.1.1` (set in root `Cargo.toml` under `[workspace.package]`)
- No tags or releases exist yet
- Published crates: `cmakefmt-rs` (core lib), `cmakefmt-cli` (binary). `cmakefmt-dprint` has `publish = false`
- CI runs on push to `main` and PRs (tests + clippy + fmt check)
- Release workflow (`.github/workflows/release.yml`) triggers on `v*` tags and handles: validation, multi-platform CLI builds, WASM build, crates.io publish, and GitHub Release creation with git-cliff changelog
- Conventional commits enforced by cocogitto (`cog verify`)

## Release Commands

Run these commands in order from the repository root.

### 1. Ensure you're on a clean, up-to-date `main`

```bash
git checkout main
git pull --ff-only
```

### 2. Run the full check suite locally

```bash
mise run check        # clippy -D warnings + cargo fmt --check
mise run test         # all tests
mise run bench        # optional: verify no perf regression
```

### 3. Bump the version

Edit the workspace version in `Cargo.toml`:

```bash
# Edit Cargo.toml [workspace.package] version field
# e.g., "0.1.1" → "0.2.0"
```

No per-crate `Cargo.toml` versions to update — all three crates inherit via `version.workspace = true`.

### 4. Commit the version bump

```bash
git add Cargo.toml
git commit -m "chore(release): v0.2.0"
```

The `chore(release)` prefix is recognized by `cliff.toml` and will be **skipped** in the generated changelog.

### 5. Tag the release

```bash
git tag v0.2.0
```

The tag **must** match `v[0-9].*` (per `cliff.toml` `tag_pattern`) and the version in `Cargo.toml` (the CI workflow verifies this).

### 6. Push the commit and tag

```bash
git push origin main
git push origin v0.2.0
```

### 7. CI takes over

Pushing the `v*` tag triggers `.github/workflows/release.yml`, which automatically:

1. **Validates** — extracts version from tag, verifies it matches `Cargo.toml`, runs `cargo test`
2. **Builds CLI** — cross-compiles for 6 targets (linux x86_64/aarch64, macOS x86_64/aarch64, Windows x86_64/aarch64), packages as `.tar.gz` / `.zip`
3. **Builds WASM** — builds `cmakefmt-dprint` with `release-wasm` profile, produces `.wasm` artifact
4. **Publishes to crates.io** — publishes `cmakefmt-rs` first, waits 30s for propagation, then publishes `cmakefmt-cli` (requires `CARGO_REGISTRY_TOKEN` secret in `crates-io` environment)
5. **Creates GitHub Release** — generates changelog via `git-cliff --latest --strip header`, downloads all build artifacts, creates a GitHub release with the changelog and attached binaries

### Summary (copy-paste)

```bash
# Pre-flight
git checkout main && git pull --ff-only
mise run check
mise run test

# Version bump (replace X.Y.Z with your target version)
# Edit Cargo.toml: version = "X.Y.Z"
git add Cargo.toml
git commit -m "chore(release): vX.Y.Z"
git tag vX.Y.Z

# Ship it
git push origin main
git push origin vX.Y.Z
```

## Prerequisites

- `CARGO_REGISTRY_TOKEN` secret configured in the GitHub repo's `crates-io` environment
- Rust stable toolchain with `wasm32-unknown-unknown` target (CI handles this, but needed for local `mise run build:release`)
- `mise` for running local task aliases
