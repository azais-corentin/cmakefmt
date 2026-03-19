# Configuration Reference (JSON)

When using cmakefmt as a dprint plugin, configuration is provided via JSON in your `dprint.json` file. All keys use `camelCase` (note: `snake_case` is also accepted but `camelCase` is recommended for JSON configuration).

## Basic Setup

First, add the cmakefmt plugin to your `dprint.json`:

```json
{
  "plugins": [
    "https://github.com/azais-corentin/cmakefmt/releases/latest/download/cmakefmt-dprint.wasm"
  ]
}
```

Then add your configuration under the `"cmake"` key:

```json
{
  "cmake": {
    "lineWidth": 120,
    "indentWidth": 4,
    "commandCase": "lower",
    "keywordCase": "upper"
  },
  "plugins": [
    "https://github.com/azais-corentin/cmakefmt/releases/latest/download/cmakefmt-dprint.wasm"
  ]
}
```

## Global dprint Configuration

cmakefmt respects dprint's global configuration options when provided:

- **`lineWidth`** — Overrides cmakefmt's `lineWidth` if not explicitly set in the `cmake` section
- **`indentWidth`** — Overrides cmakefmt's `indentWidth` if not explicitly set in the `cmake` section
- **`useTabs`** — Maps to cmakefmt's `indentStyle` (when `true`, sets `indentStyle` to `"tab"`)
- **`newLineKind`** — Maps to cmakefmt's `lineEnding` (`"lf"`, `"crlf"`, or `"auto"`)

Example with global configuration:

```json
{
  "lineWidth": 100,
  "indentWidth": 2,
  "useTabs": false,
  "newLineKind": "lf",
  "cmake": {
    "commandCase": "lower",
    "keywordCase": "upper"
  },
  "plugins": [
    "https://github.com/azais-corentin/cmakefmt/releases/latest/download/cmakefmt-dprint.wasm"
  ]
}
```

## Options Summary

All cmakefmt configuration options are available when using the dprint plugin. Below is a complete reference of all available options.

| #    | Option                       | Type                                 | Default       |
| ---- | ---------------------------- | ------------------------------------ | ------------- |
| 1.1  | `lineWidth`                  | `number`                             | `80`          |
| 1.2  | `wrapStyle`                  | `"cascade" \| "vertical"`            | `"cascade"`   |
| 1.3  | `firstArgSameLine`           | `boolean`                            | `true`        |
| 1.4  | `wrapArgThreshold`           | `number`                             | `0`           |
| 2.1  | `indentWidth`                | `number`                             | `2`           |
| 2.2  | `indentStyle`                | `"space" \| "tab"`                   | `"space"`     |
| 2.3  | `continuationIndentWidth`    | `number \| null`                     | `null`        |
| 3.1  | `maxBlankLines`              | `number`                             | `1`           |
| 3.2  | `minBlankLinesBetweenBlocks` | `number`                             | `0`           |
| 3.3  | `blankLineBetweenSections`   | `boolean`                            | `false`       |
| 4.1  | `commandCase`                | `"lower" \| "upper" \| "unchanged"`  | `"lower"`     |
| 4.2  | `keywordCase`                | `"lower" \| "upper" \| "unchanged"`  | `"upper"`     |
| 4.3  | `customKeywords`             | `string[]`                           | `[]`          |
| 4.4  | `literalCase`                | `"upper" \| "lower" \| "unchanged"`  | `"unchanged"` |
| 5.1  | `closingParenNewline`        | `boolean`                            | `true`        |
| 5.2  | `spaceBeforeParen`           | `boolean \| string[]`                | `false`       |
| 5.3  | `spaceInsideParen`           | `"insert" \| "remove" \| "preserve"` | `"remove"`    |
| 6.1  | `commentPreservation`        | `"preserve" \| "reflow"`             | `"preserve"`  |
| 6.2  | `commentWidth`               | `number \| null`                     | `null`        |
| 6.3  | `alignTrailingComments`      | `boolean`                            | `false`       |
| 6.4  | `commentGap`                 | `number`                             | `1`           |
| 7.1  | `lineEnding`                 | `"lf" \| "crlf" \| "auto"`           | `"auto"`      |
| 7.2  | `finalNewline`               | `boolean`                            | `true`        |
| 8.1  | `trimTrailingWhitespace`     | `boolean`                            | `true`        |
| 8.2  | `collapseSpaces`             | `boolean`                            | `true`        |
| 9.1  | `alignPropertyValues`        | `boolean`                            | `false`       |
| 9.2  | `alignConsecutiveSet`        | `boolean`                            | `false`       |
| 9.3  | `alignArgGroups`             | `boolean`                            | `false`       |
| 11.1 | `perCommandConfig`           | `object`                             | `{}`          |
| 12.1 | `sortArguments`              | `boolean \| string[]`                | `false`       |
| 12.2 | `sortKeywordSections`        | `boolean`                            | `false`       |
| 14.1 | `indentBlockBody`            | `boolean`                            | `true`        |
| 14.2 | `endCommandArgs`             | `"remove" \| "preserve" \| "match"`  | `"remove"`    |
| 16.1 | `disableFormatting`          | `boolean`                            | `false`       |
| 16.2 | `ignorePatterns`             | `string[]`                           | `[]`          |
| 16.3 | `ignoreCommands`             | `string[]`                           | `[]`          |

**Note:** The `extends` option (15.1) is not available when using the dprint plugin, as configuration inheritance is not supported in the JSON configuration format.

## Option Groups

### Line Width & Wrapping

Controls the maximum line length and how commands are broken across multiple lines when they exceed it.

- **`lineWidth`** (`number`, default: `80`) — Maximum columns per line. The formatter never breaks within a single token; if a command exceeds lineWidth and contains an oversized token, the command still wraps to multi-line layout and the oversized token occupies its own line. Range: 40–320.
- **`wrapStyle`** (`"cascade" | "vertical"`, default: `"cascade"`) — Wrapping strategy. `"cascade"` uses a three-step algorithm: fit on one line, then keywords on new lines with arguments packed inline, then one argument per line. `"vertical"` skips the packing step and goes directly to one-per-line; keywords are indented by `indentWidth` and values by `continuationIndentWidth`, following the same hierarchy as cascade Step 3.
- **`firstArgSameLine`** (`boolean`, default: `true`) — Whether the first positional argument (typically the target name) stays on the same line as the command name when wrapping. When `true` and the command name plus first argument exceed `lineWidth`, the overflow is tolerated (the line is not wrapped at the first argument). When `false`, the first argument is indented by `indentWidth` relative to the block's indentation level.
- **`wrapArgThreshold`** (`number`, default: `0`) — When > 0, forces a command to multi-line layout (skips cascade Step 1) whenever it has more than this many arguments, regardless of line width. Within the multi-line layout, keyword groups follow normal cascade Step 2/3 rules. Range: 0–999. `0` disables the threshold. Keywords count toward the threshold — `target_link_libraries(MyTarget PRIVATE foo bar)` has 4 arguments, not 2.

```json
{
  "cmake": {
    "lineWidth": 120,
    "wrapStyle": "vertical",
    "firstArgSameLine": false
  }
}
```

### Indentation

Controls indent character, width, and specialized indent overrides for continuation lines.

- **`indentWidth`** (`number`, default: `2`) — Spaces (or tab stops) per indentation level. Each nesting level increases indentation by this amount. Range: 1–8.
- **`indentStyle`** (`"space" | "tab"`, default: `"space"`) — Whether to indent with spaces or hard tab characters. When set to `"space"`, input tabs are converted to `indentWidth` spaces. When set to `"tab"`, the formatter uses tabs for indentation and spaces for alignment when `continuationIndentWidth` produces a non-tab-stop offset.
- **`continuationIndentWidth`** (`number | null`, default: `null`) — Indentation for value lines under a keyword within a wrapped command, measured relative to the keyword. Applies when arguments are laid out one-per-line (Step 3 of the cascade algorithm); when keyword groups pack inline (Step 2), `continuationIndentWidth` is not used. When `null`, inherits `indentWidth`. Range: 1–8. For commands without recognized keywords, `indentWidth` is used instead.

```json
{
  "cmake": {
    "indentWidth": 4,
    "indentStyle": "tab",
    "continuationIndentWidth": 2
  }
}
```

### Blank Lines

Controls how blank lines between statements and within commands are handled.

- **`maxBlankLines`** (`number`, default: `1`) — Maximum consecutive blank lines preserved between top-level statements. Runs exceeding this count are collapsed. Leading blank lines at the start of a file are always removed. Range: 0–100. Blank lines inside command argument lists are discarded during reformatting, except where they serve as sorting group boundaries (preserved when `sortArguments` is enabled). Trailing blank lines at end-of-file are also subject to this limit.
- **`minBlankLinesBetweenBlocks`** (`number`, default: `0`) — Minimum blank lines inserted before block-opening commands (`if`, `foreach`, `while`, `function`, `macro`, `block`). Takes precedence over `maxBlankLines` at block boundaries. Range: 0–10. Applies at any nesting level, including inside block bodies. Comments attached above a block move the insertion point above them. Does not insert blank lines before closing commands (`endif`, `endforeach`, etc.). The first statement inside a block body is also exempt.
- **`blankLineBetweenSections`** (`boolean`, default: `false`) — When `true`, insert a blank line between keyword sections (e.g., between `PUBLIC` and `PRIVATE` argument groups) within a command. The blank line is inserted even between consecutive keywords with zero arguments. This option takes precedence over `maxBlankLines` for blank lines within argument lists.

```json
{
  "cmake": {
    "maxBlankLines": 2,
    "minBlankLinesBetweenBlocks": 1,
    "blankLineBetweenSections": true
  }
}
```

### Casing

Controls case normalization for command names, keywords, and boolean/comparison literals.

- **`commandCase`** (`"lower" | "upper" | "unchanged"`, default: `"lower"`) — Casing applied to command names (e.g., `set`, `add_library`, `if`).
- **`keywordCase`** (`"lower" | "upper" | "unchanged"`, default: `"upper"`) — Casing applied to recognized keywords (e.g., `PRIVATE`, `PUBLIC`, `VERSION`, `PROPERTIES`). Uses the built-in keyword dictionary plus any `customKeywords`.
- **`customKeywords`** (`string[]`, default: `[]`) — Additional strings treated as keywords, subject to `keywordCase` normalization and section detection. Useful for project-specific or third-party module keywords.
- **`literalCase`** (`"upper" | "lower" | "unchanged"`, default: `"unchanged"`) — Casing for well-known boolean and comparison literals (`ON`, `OFF`, `TRUE`, `FALSE`, `AND`, `OR`, `NOT`, `MATCHES`, etc.). Applies to unquoted arguments only. When a token appears in both the keyword dictionary and the literal list, keyword classification takes precedence when the token is parsed as a keyword.

```json
{
  "cmake": {
    "commandCase": "lower",
    "keywordCase": "upper",
    "customKeywords": ["CONAN_PKG", "VCPKG_DEPS"],
    "literalCase": "upper"
  }
}
```

### Parentheses & Spacing

Controls placement of the closing parenthesis and spacing around parentheses.

- **`closingParenNewline`** (`boolean`, default: `true`) — When a command spans multiple lines, place the closing `)` on its own line at the block's base indentation. When `false`, the `)` stays on the last argument's line. The inline `)` is accounted for during wrapping, which can produce more compact layouts than `closingParenNewline = true`. If the last argument has a trailing comment, `)` is placed before the `#` marker with `commentGap` spaces between `)` and `#`.
- **`spaceBeforeParen`** (`boolean | string[]`, default: `false`) — Insert a space between the command name and `(`. `false` = no space, `true` = space for all commands, or pass an array of command names to apply selectively (e.g., `["if", "elseif", "while"]`). Case-insensitive matching.
- **`spaceInsideParen`** (`"insert" | "remove" | "preserve"`, default: `"remove"`) — Controls whitespace after `(` and before `)` on single-line invocations. Does not apply to multi-line commands or empty argument lists. When a multi-line command collapses to a single line, `"preserve"` mode treats it as `"remove"`.

```json
{
  "cmake": {
    "closingParenNewline": false,
    "spaceBeforeParen": ["if", "elseif", "while", "foreach"],
    "spaceInsideParen": "insert"
  }
}
```

### Comments

Controls comment formatting, alignment, and spacing.

- **`commentPreservation`** (`"preserve" | "reflow"`, default: `"preserve"`) — `"preserve"` keeps comments in-place (re-indenting as needed; standalone comments inside argument lists prevent single-line collapse). `"reflow"` reflows comment text to fit within `commentWidth`, treating consecutive `#` lines as paragraphs. Paragraph breaks occur at blank comment lines or whitespace-pattern changes. Indented blocks (4+ spaces) and fenced blocks (triple backticks) are preserved verbatim. List items (starting with `-`, `*`, `+`, or digits) are not reflowed.
- **`commentWidth`** (`number | null`, default: `null`) — Maximum line width for comments. Only effective when `commentPreservation` is `"reflow"`. When `null`, inherits `lineWidth`. Range: 40–320.
- **`alignTrailingComments`** (`boolean`, default: `false`) — When `true`, align trailing `#` comments on consecutive lines to start at the same column. Groups are broken by blank lines or lines without trailing comments. Only lines at the same block-nesting depth are aligned together. Within multi-line commands, trailing comments are aligned within per-keyword-section scope.
- **`commentGap`** (`number`, default: `1`) — Minimum spaces between the last code token and a trailing `#` comment. Range: 0–10.

```json
{
  "cmake": {
    "commentPreservation": "reflow",
    "commentWidth": 100,
    "alignTrailingComments": true,
    "commentGap": 2
  }
}
```

### Line Endings

Controls line-ending style and whether the file ends with a newline.

- **`lineEnding`** (`"lf" | "crlf" | "auto"`, default: `"auto"`) — Line-ending sequence for output. `"auto"` detects and preserves the dominant line ending in the input (LF wins ties).
- **`finalNewline`** (`boolean`, default: `true`) — When `true`, ensure the file ends with exactly one trailing newline. When `false`, a missing trailing newline is not added (but `maxBlankLines` still limits excess trailing blank lines).

```json
{
  "cmake": {
    "lineEnding": "lf",
    "finalNewline": true
  }
}
```

### Whitespace

Controls trailing whitespace removal and space collapsing between arguments.

- **`trimTrailingWhitespace`** (`boolean`, default: `true`) — Remove trailing spaces and tabs at the end of every line.
- **`collapseSpaces`** (`boolean`, default: `true`) — Collapse runs of multiple spaces between arguments on the same line to a single space. Applies during input normalization, before alignment. Does not affect indentation or spaces inside quoted strings.

```json
{
  "cmake": {
    "trimTrailingWhitespace": true,
    "collapseSpaces": false
  }
}
```

### Alignment

Controls column-alignment of property values, consecutive `set()` calls, and repeating argument patterns.

- **`alignPropertyValues`** (`boolean`, default: `false`) — Column-align values in `PROPERTIES` key-value lists (e.g., in `set_target_properties`). Only takes effect when properties are rendered one-per-line. When a property has multiple values, all values remain on the same line after the key at the alignment column.
- **`alignConsecutiveSet`** (`boolean`, default: `false`) — Align the first value argument of consecutive `set()` commands that form a logical group (not separated by blank lines, comments, or non-`set` commands). Valueless `set()` calls (e.g., `set(FOO)`) and those with only scope keywords (e.g., `set(FOO PARENT_SCOPE)`) are skipped but do not break the alignment group.
- **`alignArgGroups`** (`boolean`, default: `false`) — When arguments are laid out one-per-line, detect repeating structural patterns (lines with the same token count) and column-align them. Keyword-as-first-token lines are aligned by keyword column (padded to the width of the longest keyword with values, plus one gap space). Valueless keywords are excluded from width calculation and not padded. Flow keywords preserve their line-width-aware value wrapping under alignment. Groups are broken by blank lines or comment lines. When multiple values are packed onto a single line, they use single-space separation — column alignment applies across corresponding lines, not within a packed line.

```json
{
  "cmake": {
    "alignPropertyValues": true,
    "alignConsecutiveSet": true,
    "alignArgGroups": true
  }
}
```

### Generator Expressions

Generator expressions (`$<...>`) are treated as atomic tokens and are never split across lines. If a generator expression exceeds `lineWidth`, it stays on one line. There are no configuration options for generator expression formatting.

### Per-Command Overrides

Allows overriding formatting options on a per-command basis.

- **`perCommandConfig`** (`object`, default: `{}`) — A JSON object keyed by command name (case-insensitive). Each entry can override options from groups 1 (wrapping), 2 (indentation), 4 (casing), 5 (parentheses & spacing), 6 (comments), 9 (alignment), and 12 (sorting). File-level concerns (blank lines, line endings, whitespace normalization, suppression) cannot be overridden per-command. The overridable options are: `lineWidth`, `wrapStyle`, `firstArgSameLine`, `wrapArgThreshold`, `indentWidth`, `indentStyle`, `continuationIndentWidth`, `commandCase`, `keywordCase`, `customKeywords`, `literalCase`, `closingParenNewline`, `spaceBeforeParen`, `spaceInsideParen`, `commentPreservation`, `commentWidth`, `alignTrailingComments`, `commentGap`, `alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`, `sortArguments`, and `sortKeywordSections`.

```json
{
  "cmake": {
    "perCommandConfig": {
      "set": {
        "wrapStyle": "vertical"
      },
      "target_link_libraries": {
        "wrapStyle": "vertical",
        "lineWidth": 120
      },
      "if": {
        "spaceBeforeParen": true
      }
    }
  }
}
```

For temporary overrides within a file, see [Inline Pragmas](/guide/inline-pragmas).

### Sorting

Controls alphabetical sorting of arguments and canonical reordering of keyword sections.

- **`sortArguments`** (`boolean | string[]`, default: `false`) — When `true`, alphabetically sort arguments within keyword sections marked as sortable (e.g., `PRIVATE`, `PUBLIC`, `DEPENDS`, `SOURCES`). Pass an array of section names to sort only specific sections. Sorting is case-insensitive and stable. Attached comments travel with their argument; unattached comments act as group boundaries. Unattached comments and blank lines act as group boundaries — sorting does not cross them. Generator expressions and variable references are sorted by their literal text. For `add_library` and `add_executable`, source file sections are implicitly sortable. Commands without recognized keyword sections are unaffected. Duplicates are preserved in their original order. Multi-line arguments (quoted strings spanning multiple lines) are compared by their original text.
- **`sortKeywordSections`** (`boolean`, default: `false`) — When `true`, reorder keyword sections to a canonical order (e.g., `PUBLIC` before `INTERFACE` before `PRIVATE`). The canonical order is defined per-command in the keyword dictionary. Positional arguments before the first keyword remain in place. Comments attached to a keyword section travel with it during reordering.

```json
{
  "cmake": {
    "sortArguments": true,
    "sortKeywordSections": true
  }
}
```

### Flow Control

Controls indentation of flow-control block bodies and handling of arguments in closing commands.

- **`indentBlockBody`** (`boolean`, default: `true`) — When `true`, indent the body of flow-control blocks (`if`/`foreach`/`while`/`function`/`macro`/`block`) by `indentWidth`. When `false`, block bodies are not indented.
- **`endCommandArgs`** (`"remove" | "preserve" | "match"`, default: `"remove"`) — Controls arguments inside block-closing commands (`endif()`, `endfunction()`, etc.) and intermediate commands (`else()`, `elseif()`). `"remove"` strips them, `"preserve"` keeps the original, `"match"` repeats the opening command's arguments. `else()` in `"match"` mode copies the enclosing `if()` condition. `elseif()` is never affected by this option. `block()`/`endblock()` always produces empty `endblock()`.

```json
{
  "cmake": {
    "indentBlockBody": true,
    "endCommandArgs": "preserve"
  }
}
```

### Suppression & Ignoring

Controls disabling formatting entirely, skipping files by pattern, and skipping specific commands.

- **`disableFormatting`** (`boolean`, default: `false`) — When `true`, output is byte-for-byte identical to input. No transformations or normalizations are applied, including BOM stripping. Takes precedence over all other options.
- **`ignorePatterns`** (`string[]`, default: `[]`) — Glob patterns for files to skip entirely. Patterns are resolved relative to the config file's directory. Uses gitignore-style syntax (`*`, `**`, `?`, `[...]`).
- **`ignoreCommands`** (`string[]`, default: `[]`) — Command names whose invocations are preserved verbatim (case-insensitive). Useful for complex macro invocations or DSL-like commands where the formatter's heuristics may produce undesirable results.

```json
{
  "cmake": {
    "ignorePatterns": ["third_party/**", "generated/*.cmake"],
    "ignoreCommands": ["ExternalProject_Add", "FetchContent_Declare"]
  }
}
```

## Complete Example

Here's a complete example `dprint.json` with commonly used options:

```json
{
  "lineWidth": 100,
  "indentWidth": 2,
  "useTabs": false,
  "newLineKind": "lf",
  "cmake": {
    "commandCase": "lower",
    "keywordCase": "upper",
    "closingParenNewline": true,
    "trimTrailingWhitespace": true,
    "endCommandArgs": "remove",
    "sortArguments": ["SOURCES", "FILES"],
    "alignPropertyValues": true,
    "ignorePatterns": ["build/**", "third_party/**"],
    "ignoreCommands": ["ExternalProject_Add"],
    "perCommandConfig": {
      "if": {
        "spaceBeforeParen": true
      },
      "elseif": {
        "spaceBeforeParen": true
      }
    }
  },
  "plugins": [
    "https://github.com/azais-corentin/cmakefmt/releases/latest/download/cmakefmt-dprint.wasm"
  ]
}
```

## Default Configuration

The following shows all options at their default values (JSON format):

```json
{
  "cmake": {
    "lineWidth": 80,
    "wrapStyle": "cascade",
    "firstArgSameLine": true,
    "wrapArgThreshold": 0,
    "indentWidth": 2,
    "indentStyle": "space",
    "continuationIndentWidth": null,
    "maxBlankLines": 1,
    "minBlankLinesBetweenBlocks": 0,
    "blankLineBetweenSections": false,
    "commandCase": "lower",
    "keywordCase": "upper",
    "customKeywords": [],
    "literalCase": "unchanged",
    "closingParenNewline": true,
    "spaceBeforeParen": false,
    "spaceInsideParen": "remove",
    "commentPreservation": "preserve",
    "commentWidth": null,
    "alignTrailingComments": false,
    "commentGap": 1,
    "lineEnding": "auto",
    "finalNewline": true,
    "trimTrailingWhitespace": true,
    "collapseSpaces": true,
    "alignPropertyValues": false,
    "alignConsecutiveSet": false,
    "alignArgGroups": false,
    "perCommandConfig": {},
    "sortArguments": false,
    "sortKeywordSections": false,
    "disableFormatting": false,
    "ignorePatterns": [],
    "ignoreCommands": [],
    "indentBlockBody": true,
    "endCommandArgs": "remove"
  }
}
```

## See Also

- [Inline Pragmas](/guide/inline-pragmas) — control formatting locally within a file using comment directives
- [Configuration (TOML)](/guide/configuration-toml) — TOML configuration for CLI users
- [dprint documentation](https://dprint.dev/) — learn more about dprint
