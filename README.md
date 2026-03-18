<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/azais-corentin/cmakefmt/HEAD/docs/logos/cmakefmt-logo-dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/azais-corentin/cmakefmt/HEAD/docs/logos/cmakefmt-logo-light.svg">
  <img src="https://raw.githubusercontent.com/azais-corentin/cmakefmt/HEAD/docs/logos/cmakefmt-logo-light.svg" alt="cmakefmt" height="160">
</picture>

<h1 style="color: #C5BCF7;">cmakefmt</h1>

<p>opinionated cmake formatting</p>

[![CI](https://github.com/azais-corentin/cmakefmt/actions/workflows/ci.yml/badge.svg)](https://github.com/azais-corentin/cmakefmt/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/cmakefmt-rs)](https://crates.io/crates/cmakefmt-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Documentation](https://azais-corentin.github.io/cmakefmt/)
| [Getting Started](https://azais-corentin.github.io/cmakefmt/guide/getting-started)
| [Configuration](https://azais-corentin.github.io/cmakefmt/guide/configuration)

</div>

cmakefmt takes your CMake source files (`CMakeLists.txt` and `*.cmake`), applies a
consistent set of formatting rules, and outputs the result. It normalizes command
casing, keyword casing, indentation, argument wrapping, alignment, and more -- with
nearly 40 configuration options. If a file cannot be parsed, the
formatter returns it unchanged -- it never silently corrupts your code.

## Before / After

<img src="https://raw.githubusercontent.com/azais-corentin/cmakefmt/HEAD/docs/screenshots/input.svg" alt="Before formatting" style="max-width: 100%; height: auto;">
<img src="https://raw.githubusercontent.com/azais-corentin/cmakefmt/HEAD/docs/screenshots/output.svg" alt="After formatting" style="max-width: 100%; height: auto;">

## Features

- **Command casing** -- normalizes `PROJECT`, `Set`, `IF` to lowercase (or uppercase)
- **Keyword & literal casing** -- normalizes `public`, `Private` to `PUBLIC`, `PRIVATE`; `on`/`Off` to `ON`/`OFF`
- **Argument wrapping** -- multiple strategies (cascade, vertical) with `firstArgSameLine`, `wrapArgThreshold`, and per-command overrides
- **Alignment** -- aligns property values, condition arguments, and keyword groups
- **Comment formatting** -- reflowing, indentation, and trailing comment alignment
- **Sorting** -- alphabetical sorting of argument lists under configurable keywords
- **Blank line control** -- `maxBlankLines`, `minBlankLinesBetweenBlocks`, `blankLineBetweenSections` for structural spacing
- **Generator expressions** -- `$<...>` expressions are recognized and treated atomically, never broken across lines
- **Line endings & final newline** -- LF/CRLF/auto normalization, trailing whitespace removal
- **Per-command overrides** -- fine-grained control over individual commands (e.g. `spaceBeforeParen` for `if`)
- **Inline pragmas** -- `# cmakefmt: off/on/skip` to suppress formatting, `push { ... }`/`pop` for scoped config overrides
- **Config inheritance** -- `extends` key for sharing base configs across projects
- **Safe by default** -- unparseable input is returned unchanged; never silently corrupts

## Installation

### cargo install

```bash
cargo install cmakefmt-cli
```

### Prebuilt Binaries

Download prebuilt binaries for your platform from
[GitHub Releases](https://github.com/azais-corentin/cmakefmt/releases):

- linux-x64, linux-arm64
- macos-x64, macos-arm64
- windows-x64, windows-arm64

### dprint Plugin

Add cmakefmt as a WASM plugin in your `.dprintrc.json`:

```json
{
  "plugins": [
    "https://github.com/azais-corentin/cmakefmt/releases/latest/download/cmakefmt-dprint.wasm"
  ]
}
```

## Quick Start

Format a file and print to stdout:

```bash
cmakefmt CMakeLists.txt
```

Format in place:

```bash
cmakefmt --write CMakeLists.txt  # also: --inplace
```

Check formatting without modifying files (exits with code 1 if changes are needed):

```bash
cmakefmt --check CMakeLists.txt
```

Show a unified diff of what would change:

```bash
cmakefmt --diff CMakeLists.txt
```

Format from stdin:

```bash
cat CMakeLists.txt | cmakefmt --stdin
```

## Configuration

Create a `.cmakefmt.toml` (or `cmakefmt.toml`) in your project root. The formatter
discovers config by walking from the formatted file's directory upward to the
filesystem root. All keys use `camelCase` (`snake_case` is also accepted).

```toml
lineWidth = 120
indentWidth = 4
indentStyle = "space"
commandCase = "lower"
keywordCase = "upper"
closingParenNewline = true
lineEnding = "lf"
finalNewline = true
trimTrailingWhitespace = true
endCommandArgs = "remove"

sortArguments = ["SOURCES", "FILES"]
alignPropertyValues = true

ignorePatterns = ["build/**", "third_party/**"]
ignoreCommands = ["ExternalProject_Add"]

[perCommandConfig.if]
spaceBeforeParen = true

[perCommandConfig.elseif]
spaceBeforeParen = true
```

For the full list of options and their defaults, see the
[Configuration Reference](https://azais-corentin.github.io/cmakefmt/guide/configuration).

## Editor Integration

### External Formatter (stdin pipe)

Most editors can invoke an external command to format the current buffer. Configure
your editor to pipe through:

```bash
cmakefmt --stdin --assume-filename <path>
```

The `--assume-filename` flag tells cmakefmt which file path to use for config
discovery (walking upward to find `.cmakefmt.toml`).

### dprint

If you use [dprint](https://dprint.dev/), add the cmakefmt WASM plugin to your
`.dprintrc.json` (see the Installation section above), then use dprint's editor
extensions for VS Code, Neovim, or other supported editors.

## Documentation

Full documentation is available at
[azais-corentin.github.io/cmakefmt](https://azais-corentin.github.io/cmakefmt/).

## License

MIT -- see [LICENSE](LICENSE) for details.
