# CMake Formatter — Configuration & Formatting Rules Specification

This document defines every formatting behavior and its associated configuration option for the CMake formatter.
Options are organized into logical groups. Each option includes its type, default value, a description of its
effect, and concrete before/after examples where helpful.

Input is expected to be valid UTF-8. Non-UTF-8 input is rejected with an error.
When `disableFormatting = false` (default), a leading UTF-8 BOM (byte-order mark) is stripped from input and never emitted in output.
When `disableFormatting = true` (§16.1), output is byte-for-byte identical to input and no preprocessing or normalization (including BOM stripping) is applied.

Configuration is read from a TOML file named `.cmakefmt.toml` (or `cmakefmt.toml`) discovered by walking
from the formatted file's directory upward to the filesystem root. The first file found wins. If both
`.cmakefmt.toml` and `cmakefmt.toml` exist in the same directory, `.cmakefmt.toml` (the dotfile) takes
precedence. All keys use `camelCase`. When reading from stdin, config discovery follows these rules:

1. If `--config <path>` is passed, use that config file.
2. If `--assume-filename <path>` is passed, discover config relative to that file's directory.
3. Fallback: discover config from the current working directory.

If input cannot be parsed, the formatter returns the input unchanged, emits a diagnostic to stderr, and
exits with a non-zero status code. The formatter never silently corrupts unparseable input.

## Table of Contents

| #  | Section                               | File                                                                 |
| -- | ------------------------------------- | -------------------------------------------------------------------- |
| 1  | Line Width & Wrapping                 | [01-line-width-wrapping.md](01-line-width-wrapping.md)               |
| 2  | Indentation                           | [02-indentation.md](02-indentation.md)                               |
| 3  | Blank Lines                           | [03-blank-lines.md](03-blank-lines.md)                               |
| 4  | Casing                                | [04-casing.md](04-casing.md)                                         |
| 5  | Parentheses & Spacing                 | [05-parens-spacing.md](05-parens-spacing.md)                         |
| 6  | Comments                              | [06-comments.md](06-comments.md)                                     |
| 7  | Line Endings & Final Newline          | [07-line-endings.md](07-line-endings.md)                             |
| 8  | Whitespace Normalization              | [08-whitespace.md](08-whitespace.md)                                 |
| 9  | Alignment                             | [09-alignment.md](09-alignment.md)                                   |
| 10 | Generator Expressions                 | [10-generator-expressions.md](10-generator-expressions.md)           |
| 11 | Command-Specific Overrides            | [11-per-command-config.md](11-per-command-config.md)                 |
| 12 | Sorting                               | [12-sorting.md](12-sorting.md)                                       |
| 13 | Inline Pragmas                        | [13-inline-pragmas.md](13-inline-pragmas.md)                         |
| 14 | Conditional & Flow Control Formatting | [14-flow-control.md](14-flow-control.md)                             |
| 15 | Configuration Meta                    | [15-config-meta.md](15-config-meta.md)                               |
| 16 | Suppression & Ignore Options          | [16-suppression.md](16-suppression.md)                               |
| A  | Default Configuration                 | [appendix-a-defaults.md](appendix-a-defaults.md)                     |
| B  | Example Config File                   | [appendix-b-example-config.md](appendix-b-example-config.md)         |
| C  | Cascading Wrap Algorithm Detail       | [appendix-c-cascade-algorithm.md](appendix-c-cascade-algorithm.md)   |
| D  | CLI Reference                         | [appendix-d-cli.md](appendix-d-cli.md)                               |
| E  | Option Interaction Rules              | [appendix-e-interactions.md](appendix-e-interactions.md)             |
| F  | Keyword Dictionary                    | [appendix-f-keyword-dictionary.md](appendix-f-keyword-dictionary.md) |

## Summary Table

| #    | Option                       | Type                                 | Default       |
| ---- | ---------------------------- | ------------------------------------ | ------------- |
| 1.1  | `lineWidth`                  | `integer`                            | `80`          |
| 1.2  | `wrapStyle`                  | `"cascade" \| "vertical"`            | `"cascade"`   |
| 1.3  | `firstArgSameLine`           | `boolean`                            | `true`        |
| 1.4  | `wrapArgThreshold`           | `integer`                            | `0`           |
| 1.5  | `magicTrailingNewline`       | `boolean`                            | `true`        |
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
