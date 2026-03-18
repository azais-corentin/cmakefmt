# Getting Started

cmakefmt is an opinionated formatter for CMake files (`CMakeLists.txt` and `*.cmake`). It takes CMake source, applies a consistent set of formatting rules, and outputs the result. Input must be valid UTF-8. If a file cannot be parsed, the formatter returns the input unchanged and exits with a non-zero status code -- it never silently corrupts unparseable input.

## Installation

### cargo install

```bash
cargo install cmakefmt-cli
```

### Prebuilt Binaries

Download prebuilt binaries for your platform from [GitHub Releases](https://github.com/azais-corentin/cmakefmt/releases):

- linux-x64, linux-arm64
- macos-x64, macos-arm64
- windows-x64, windows-arm64

### dprint Plugin

Add cmakefmt as a WASM plugin in your `dprint.json`:

```json
{
  "plugins": [
    "https://github.com/azais-corentin/cmakefmt/releases/latest/download/cmakefmt-dprint.wasm"
  ]
}
```

## Quick Start

Format a file and print the result to stdout:

```bash
cmakefmt CMakeLists.txt
```

Format a file in place:

```bash
cmakefmt --write CMakeLists.txt
```

Check formatting without modifying files (useful for CI -- exits with code 1 if changes are needed):

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

See the full list of flags in the [CLI Reference](/guide/cli).

## Configuration

Create a `.cmakefmt.toml` (or `cmakefmt.toml`) in your project root. The formatter discovers config by walking from the formatted file's directory upward to the filesystem root. All keys use `camelCase` (`snake_case` is also accepted).

Example configuration:

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

For a full list of options and their defaults, see the [Configuration Reference (TOML)](/guide/configuration-toml) for CLI usage or [Configuration Reference (JSON)](/guide/configuration-json) for dprint plugin usage.

## Editor Integration

### External Formatter (stdin pipe)

Most editors can invoke an external command to format the current buffer. Configure your editor to pipe through:

```bash
cmakefmt --stdin --assume-filename <path>
```

The `--assume-filename` flag tells cmakefmt which file path to use for config discovery (walking upward to find `.cmakefmt.toml`).

### dprint

If you use [dprint](https://dprint.dev/), add the cmakefmt WASM plugin to your `dprint.json` (see the Installation section above), then use dprint's editor extensions for VS Code, Neovim, or other supported editors.
