# Configuration Reference

cmakefmt is configured through a TOML file. All keys use `camelCase`.

## Config File Discovery

Configuration is read from a file named `.cmakefmt.toml` (or `cmakefmt.toml`) discovered by walking from the formatted file's directory upward to the filesystem root. The first file found wins. If both `.cmakefmt.toml` and `cmakefmt.toml` exist in the same directory, the dotfile (`.cmakefmt.toml`) takes precedence.

When reading from stdin, config discovery follows these rules:

1. If `--config <path>` is passed, use that config file.
2. If `--assume-filename <path>` is passed, discover config relative to that file's directory.
3. Fallback: discover config from the current working directory.

See the [CLI reference](/guide/cli) for details on these flags.

## Options Summary

| #    | Option                       | Type                                 | Default       |
| ---- | ---------------------------- | ------------------------------------ | ------------- |
| 1.1  | `lineWidth`                  | `integer`                            | `80`          |
| 1.2  | `wrapStyle`                  | `"cascade" \| "vertical"`            | `"cascade"`   |
| 1.3  | `firstArgSameLine`           | `boolean`                            | `true`        |
| 1.4  | `wrapArgThreshold`           | `integer`                            | `0`           |
| 2.1  | `indentWidth`                | `integer`                            | `2`           |
| 2.2  | `indentStyle`                | `"space" \| "tab"`                   | `"space"`     |
| 2.3  | `continuationIndentWidth`    | `integer \| null`                    | `null`        |
| 2.4  | `genexIndentWidth`           | `integer \| null`                    | `null`        |
| 3.1  | `maxBlankLines`              | `integer`                            | `1`           |
| 3.2  | `minBlankLinesBetweenBlocks` | `integer`                            | `0`           |
| 3.3  | `blankLineBetweenSections`   | `boolean`                            | `false`       |
| 4.1  | `commandCase`                | `"lower" \| "upper" \| "unchanged"`  | `"lower"`     |
| 4.2  | `keywordCase`                | `"lower" \| "upper" \| "unchanged"`  | `"upper"`     |
| 4.3  | `customKeywords`             | `string[]`                           | `[]`          |
| 4.4  | `literalCase`                | `"upper" \| "lower" \| "unchanged"`  | `"unchanged"` |
| 5.1  | `closingParenNewline`        | `boolean`                            | `true`        |
| 5.2  | `spaceBeforeParen`           | `boolean \| string[]`                | `false`       |
| 5.3  | `spaceInsideParen`           | `"insert" \| "remove" \| "preserve"` | `"remove"`    |
| 6.1  | `commentPreservation`        | `"preserve" \| "reflow"`             | `"preserve"`  |
| 6.2  | `commentWidth`               | `integer \| null`                    | `null`        |
| 6.3  | `alignTrailingComments`      | `boolean`                            | `false`       |
| 6.4  | `commentGap`                 | `integer`                            | `1`           |
| 7.1  | `lineEnding`                 | `"lf" \| "crlf" \| "auto"`           | `"auto"`      |
| 7.2  | `finalNewline`               | `boolean`                            | `true`        |
| 8.1  | `trimTrailingWhitespace`     | `boolean`                            | `true`        |
| 8.2  | `collapseSpaces`             | `boolean`                            | `true`        |
| 9.1  | `alignPropertyValues`        | `boolean`                            | `false`       |
| 9.2  | `alignConsecutiveSet`        | `boolean`                            | `false`       |
| 9.3  | `alignArgGroups`             | `boolean`                            | `false`       |
| 10.1 | `genexWrap`                  | `"cascade" \| "never"`               | `"cascade"`   |
| 10.2 | `genexClosingAngleNewline`   | `boolean`                            | `true`        |
| 11.1 | `perCommandConfig`           | `table`                              | `{}`          |
| 12.1 | `sortArguments`              | `boolean \| string[]`                | `false`       |
| 12.2 | `sortKeywordSections`        | `boolean`                            | `false`       |
| 14.1 | `indentBlockBody`            | `boolean`                            | `true`        |
| 14.2 | `endCommandArgs`             | `"remove" \| "preserve" \| "match"`  | `"remove"`    |
| 15.1 | `$schema`                    | `string`                             | —             |
| 15.2 | `extends`                    | `string`                             | —             |
| 16.1 | `disableFormatting`          | `boolean`                            | `false`       |
| 16.2 | `ignorePatterns`             | `string[]`                           | `[]`          |
| 16.3 | `ignoreCommands`             | `string[]`                           | `[]`          |

## Option Groups

### Line Width & Wrapping

Controls the maximum line length and how commands are broken across multiple lines when they exceed it.

- **`lineWidth`** (`integer`, default: `80`) — Maximum columns per line. The formatter never breaks within a single token. Range: 40–320.
- **`wrapStyle`** (`"cascade" | "vertical"`, default: `"cascade"`) — Wrapping strategy. `"cascade"` uses a three-step algorithm: fit on one line, then keywords on new lines with arguments packed inline, then one argument per line. `"vertical"` skips the packing step and goes directly to one-per-line.
- **`firstArgSameLine`** (`boolean`, default: `true`) — Whether the first positional argument (typically the target name) stays on the same line as the command name when wrapping.
- **`wrapArgThreshold`** (`integer`, default: `0`) — When > 0, forces one-argument-per-line wrapping whenever a command has more than this many arguments, regardless of line width. `0` disables the threshold. Keywords count toward the threshold — `target_link_libraries(MyTarget PRIVATE foo bar)` has 4 arguments, not 2.

```toml
lineWidth = 120
wrapStyle = "vertical"
firstArgSameLine = false
```

### Indentation

Controls indent character, width, and specialized indent overrides for continuation lines and generator expressions.

- **`indentWidth`** (`integer`, default: `2`) — Spaces (or tab stops) per indentation level. Each nesting level increases indentation by this amount. Range: 1–8.
- **`indentStyle`** (`"space" | "tab"`, default: `"space"`) — Whether to indent with spaces or hard tab characters.
- **`continuationIndentWidth`** (`integer | null`, default: `null`) — Indentation for value lines under a keyword within a wrapped command, measured relative to the keyword. When `null`, inherits `indentWidth`. Range: 1–8. For commands without recognized keywords, `indentWidth` is used instead.
- **`genexIndentWidth`** (`integer | null`, default: `null`) — Override indentation inside generator expressions (`$<...>`), relative to the column where `$<` starts. When `null`, inherits `indentWidth`. Range: 1–8.

```toml
indentWidth = 4
indentStyle = "tab"
continuationIndentWidth = 2
```

### Blank Lines

Controls how blank lines between statements and within commands are handled.

- **`maxBlankLines`** (`integer`, default: `1`) — Maximum consecutive blank lines preserved between top-level statements. Runs exceeding this count are collapsed. Leading blank lines at the start of a file are always removed. Range: 0–100. Blank lines inside command argument lists are always discarded during reformatting. Trailing blank lines at end-of-file are also subject to this limit.
- **`minBlankLinesBetweenBlocks`** (`integer`, default: `0`) — Minimum blank lines inserted before block-opening commands (`if`, `foreach`, `while`, `function`, `macro`, `block`). Takes precedence over `maxBlankLines` at block boundaries. Range: 0–10.
- **`blankLineBetweenSections`** (`boolean`, default: `false`) — When `true`, insert a blank line between keyword sections (e.g., between `PUBLIC` and `PRIVATE` argument groups) within a command.

```toml
maxBlankLines = 2
minBlankLinesBetweenBlocks = 1
blankLineBetweenSections = true
```

### Casing

Controls case normalization for command names, keywords, and boolean/comparison literals.

- **`commandCase`** (`"lower" | "upper" | "unchanged"`, default: `"lower"`) — Casing applied to command names (e.g., `set`, `add_library`, `if`).
- **`keywordCase`** (`"lower" | "upper" | "unchanged"`, default: `"upper"`) — Casing applied to recognized keywords (e.g., `PRIVATE`, `PUBLIC`, `VERSION`, `PROPERTIES`). Uses the built-in keyword dictionary plus any `customKeywords`.
- **`customKeywords`** (`string[]`, default: `[]`) — Additional strings treated as keywords, subject to `keywordCase` normalization and section detection. Useful for project-specific or third-party module keywords.
- **`literalCase`** (`"upper" | "lower" | "unchanged"`, default: `"unchanged"`) — Casing for well-known boolean and comparison literals (`ON`, `OFF`, `TRUE`, `FALSE`, `AND`, `OR`, `NOT`, `MATCHES`, etc.). Applies to unquoted arguments only.

```toml
commandCase = "lower"
keywordCase = "upper"
customKeywords = ["CONAN_PKG", "VCPKG_DEPS"]
literalCase = "upper"
```

### Parentheses & Spacing

Controls placement of the closing parenthesis and spacing around parentheses.

- **`closingParenNewline`** (`boolean`, default: `true`) — When a command spans multiple lines, place the closing `)` on its own line at the block's base indentation. When `false`, the `)` stays on the last argument's line. If the last argument has a trailing comment, `)` is placed before the `#` marker.
- **`spaceBeforeParen`** (`boolean | string[]`, default: `false`) — Insert a space between the command name and `(`. `false` = no space, `true` = space for all commands, or pass an array of command names to apply selectively (e.g., `["if", "elseif", "while"]`). Case-insensitive matching.
- **`spaceInsideParen`** (`"insert" | "remove" | "preserve"`, default: `"remove"`) — Controls whitespace after `(` and before `)` on single-line invocations. Does not apply to multi-line commands or empty argument lists. When a multi-line command collapses to a single line, `"preserve"` mode treats it as `"remove"`.

```toml
closingParenNewline = false
spaceBeforeParen = ["if", "elseif", "while", "foreach"]
spaceInsideParen = "insert"
```

### Comments

Controls comment formatting, alignment, and spacing.

- **`commentPreservation`** (`"preserve" | "reflow"`, default: `"preserve"`) — `"preserve"` keeps comments in-place (re-indenting as needed; standalone comments inside argument lists prevent single-line collapse). `"reflow"` reflows comment text to fit within `commentWidth`, treating consecutive `#` lines as paragraphs. Paragraph breaks occur at blank comment lines or whitespace-pattern changes. Indented blocks (4+ spaces) and fenced blocks (triple backticks) are preserved verbatim. List items (starting with `-`, `*`, `+`, or digits) are not reflowed.
- **`commentWidth`** (`integer | null`, default: `null`) — Maximum line width for comments. Only effective when `commentPreservation` is `"reflow"`. When `null`, inherits `lineWidth`. Range: 40–320.
- **`alignTrailingComments`** (`boolean`, default: `false`) — When `true`, align trailing `#` comments on consecutive lines to start at the same column. Groups are broken by blank lines or lines without trailing comments.
- **`commentGap`** (`integer`, default: `1`) — Minimum spaces between the last code token and a trailing `#` comment. Range: 0–10.

```toml
commentPreservation = "reflow"
commentWidth = 100
alignTrailingComments = true
commentGap = 2
```

### Line Endings

Controls line-ending style and whether the file ends with a newline.

- **`lineEnding`** (`"lf" | "crlf" | "auto"`, default: `"auto"`) — Line-ending sequence for output. `"auto"` detects and preserves the dominant line ending in the input (LF wins ties).
- **`finalNewline`** (`boolean`, default: `true`) — When `true`, ensure the file ends with exactly one trailing newline. When `false`, a missing trailing newline is not added (but `maxBlankLines` still limits excess trailing blank lines).

```toml
lineEnding = "lf"
finalNewline = true
```

### Whitespace

Controls trailing whitespace removal and space collapsing between arguments.

- **`trimTrailingWhitespace`** (`boolean`, default: `true`) — Remove trailing spaces and tabs at the end of every line.
- **`collapseSpaces`** (`boolean`, default: `true`) — Collapse runs of multiple spaces between arguments on the same line to a single space. Applies during input normalization, before alignment. Does not affect indentation or spaces inside quoted strings.

```toml
trimTrailingWhitespace = true
collapseSpaces = false
```

### Alignment

Controls column-alignment of property values, consecutive `set()` calls, and repeating argument patterns.

- **`alignPropertyValues`** (`boolean`, default: `false`) — Column-align values in `PROPERTIES` key-value lists (e.g., in `set_target_properties`). Only takes effect when properties are rendered one-per-line.
- **`alignConsecutiveSet`** (`boolean`, default: `false`) — Align the first value argument of consecutive `set()` commands that form a logical group (not separated by blank lines, comments, or non-`set` commands).
- **`alignArgGroups`** (`boolean`, default: `false`) — When arguments are laid out one-per-line, detect repeating structural patterns (lines with the same token count) and column-align them. Keyword-as-first-token lines are aligned by keyword column (padded to the width of the longest keyword with values, plus one gap space). Valueless keywords are excluded from width calculation and not padded. Flow keywords preserve their line-width-aware value wrapping under alignment. Groups are broken by blank lines or comment lines.

```toml
alignPropertyValues = true
alignConsecutiveSet = true
alignArgGroups = true
```

### Generator Expressions

Controls formatting of generator expressions (`$<...>`).

- **`genexWrap`** (`"cascade" | "never"`, default: `"cascade"`) — `"cascade"` applies the cascading wrapping algorithm to generator expressions, splitting at `:` delimiters and wrapping `;`-separated list items. `"never"` keeps generator expressions on a single line regardless of length.
- **`genexClosingAngleNewline`** (`boolean`, default: `true`) — Analogous to `closingParenNewline` but for the closing `>` of generator expressions. When `true`, the closing `>` is placed on its own line aligned with `$<`. Applies recursively at every nesting level.

```toml
genexWrap = "never"
genexClosingAngleNewline = false
```

### Per-Command Overrides

Allows overriding formatting options on a per-command basis.

- **`perCommandConfig`** (`table`, default: `{}`) — A TOML table keyed by command name (case-insensitive). Each entry can override options from groups 1 (wrapping), 2 (indentation), 4 (casing), 5 (parentheses & spacing), 6 (comments), 9 (alignment), 10 (generator expressions), and 12 (sorting). File-level concerns (blank lines, line endings, whitespace normalization, suppression) cannot be overridden per-command. The overridable options are: `lineWidth`, `wrapStyle`, `firstArgSameLine`, `wrapArgThreshold`, `indentWidth`, `indentStyle`, `continuationIndentWidth`, `genexIndentWidth`, `commandCase`, `keywordCase`, `customKeywords`, `literalCase`, `closingParenNewline`, `spaceBeforeParen`, `spaceInsideParen`, `commentPreservation`, `commentWidth`, `alignTrailingComments`, `commentGap`, `alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`, `genexWrap`, `genexClosingAngleNewline`, `sortArguments`, and `sortKeywordSections`.

```toml
[perCommandConfig.set]
wrapStyle = "vertical"

[perCommandConfig.target_link_libraries]
wrapStyle = "vertical"
lineWidth = 120

[perCommandConfig.if]
spaceBeforeParen = true
```

For temporary overrides within a file, see [Inline Pragmas](/guide/inline-pragmas).

### Sorting

Controls alphabetical sorting of arguments and canonical reordering of keyword sections.

- **`sortArguments`** (`boolean | string[]`, default: `false`) — When `true`, alphabetically sort arguments within keyword sections marked as sortable (e.g., `PRIVATE`, `PUBLIC`, `DEPENDS`, `SOURCES`). Pass an array of section names to sort only specific sections. Sorting is case-insensitive and stable. Attached comments travel with their argument; unattached comments act as group boundaries. Unattached comments and blank lines act as group boundaries — sorting does not cross them. Generator expressions and variable references are sorted by their literal text. For `add_library` and `add_executable`, source file sections are implicitly sortable. Commands without recognized keyword sections are unaffected.
- **`sortKeywordSections`** (`boolean`, default: `false`) — When `true`, reorder keyword sections to a canonical order (e.g., `PUBLIC` before `INTERFACE` before `PRIVATE`). The canonical order is defined per-command in the keyword dictionary.

```toml
sortArguments = true
sortKeywordSections = true
```

### Flow Control

Controls indentation of flow-control block bodies and handling of arguments in closing commands.

- **`indentBlockBody`** (`boolean`, default: `true`) — When `true`, indent the body of flow-control blocks (`if`/`foreach`/`while`/`function`/`macro`/`block`) by `indentWidth`. When `false`, block bodies are not indented.
- **`endCommandArgs`** (`"remove" | "preserve" | "match"`, default: `"remove"`) — Controls arguments inside block-closing commands (`endif()`, `endfunction()`, etc.) and intermediate commands (`else()`, `elseif()`). `"remove"` strips them, `"preserve"` keeps the original, `"match"` repeats the opening command's arguments. `else()` in `"match"` mode copies the enclosing `if()` condition. `elseif()` is never affected by this option. `block()`/`endblock()` always produces empty `endblock()`.

```toml
indentBlockBody = true
endCommandArgs = "preserve"
```

### Suppression & Ignoring

Controls disabling formatting entirely, skipping files by pattern, and skipping specific commands.

- **`disableFormatting`** (`boolean`, default: `false`) — When `true`, output is byte-for-byte identical to input. No transformations or normalizations are applied, including BOM stripping. Takes precedence over all other options.
- **`ignorePatterns`** (`string[]`, default: `[]`) — Glob patterns for files to skip entirely. Patterns are resolved relative to the config file's directory. Uses gitignore-style syntax (`*`, `**`, `?`, `[...]`).
- **`ignoreCommands`** (`string[]`, default: `[]`) — Command names whose invocations are preserved verbatim (case-insensitive). Useful for complex macro invocations or DSL-like commands where the formatter's heuristics may produce undesirable results.

```toml
ignorePatterns = ["third_party/**", "generated/*.cmake"]
ignoreCommands = ["ExternalProject_Add", "FetchContent_Declare"]
```

## Config Meta

### `$schema`

Optional JSON Schema URL for editor validation and autocomplete. Has no effect on the formatter itself.

```toml
"$schema" = "https://raw.githubusercontent.com/azais-corentin/cmakefmt/main/schema.json"
```

### `extends`

Path to another `.cmakefmt.toml` file to use as a base. Options in the current file override the base. The path is resolved relative to the directory containing the config file that declares it; absolute paths are used as-is. Circular references are detected and produce an error.

**Merge strategy:** Scalars in the child override the base. Array values in the child replace the base array entirely (applies to `customKeywords`, `ignorePatterns`, `ignoreCommands`, `sortArguments` when array-valued, and `spaceBeforeParen` when array-valued). The `perCommandConfig` table is shallow-merged: child command keys override the base entry for that command entirely, while base entries not present in the child are preserved.

```toml
extends = "../../.cmakefmt.toml"

[perCommandConfig.set]
wrapStyle = "vertical"
```

### Unknown Keys

Unknown keys in `.cmakefmt.toml` produce a diagnostic warning (including the key name and file path) and are ignored. This allows forward-compatibility: a config file written for a newer formatter version can be used with an older version without errors. Unknown keys inside `perCommandConfig` tables follow the same policy.

## Default Configuration

The following `.cmakefmt.toml` shows all options at their default values:

```toml
lineWidth = 80
wrapStyle = "cascade"
firstArgSameLine = true
wrapArgThreshold = 0
indentWidth = 2
indentStyle = "space"
# continuationIndentWidth — inherits indentWidth
# genexIndentWidth — inherits indentWidth
maxBlankLines = 1
minBlankLinesBetweenBlocks = 0
blankLineBetweenSections = false
commandCase = "lower"
keywordCase = "upper"
customKeywords = []
literalCase = "unchanged"
closingParenNewline = true
spaceBeforeParen = false
spaceInsideParen = "remove"
commentPreservation = "preserve"
# commentWidth — inherits lineWidth
alignTrailingComments = false
commentGap = 1
lineEnding = "auto"
finalNewline = true
trimTrailingWhitespace = true
collapseSpaces = true
alignPropertyValues = false
alignConsecutiveSet = false
alignArgGroups = false
genexWrap = "cascade"
genexClosingAngleNewline = true
# perCommandConfig — empty table by default
sortArguments = false
sortKeywordSections = false
disableFormatting = false
ignorePatterns = []
ignoreCommands = []
indentBlockBody = true
endCommandArgs = "remove"
```

## See Also

- [Inline Pragmas](/guide/inline-pragmas) — control formatting locally within a file using comment directives
- [CLI Reference](/guide/cli) — command-line flags and usage
