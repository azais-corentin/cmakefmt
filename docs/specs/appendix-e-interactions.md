## Appendix E — Option Interaction Rules

This appendix documents interactions between options where the combined behavior is not
obvious from reading each option's description in isolation.

### E.1 Normative global formatting pipeline order

Unless explicitly suppressed by `disableFormatting` (§16.1) or pragma regions (§13.1), the formatter
applies transformations in this total order:

1. Evaluate suppression gates (`disableFormatting`, ignored files/regions/commands).
2. Strip UTF-8 BOM only when present at byte 0 (see §7.1).
3. Detect dominant line ending for `lineEnding = "auto"`.
4. Resolve active config for each command (`push` stack → `perCommandConfig` → file config → defaults).
5. Apply section/value sorting (`sortArguments`, then `sortKeywordSections`).
6. Apply casing normalization (`commandCase`, `keywordCase`, `literalCase`, `customKeywords`).
7. Normalize intra-token spacing (`collapseSpaces`, excluding verbatim regions).
8. Apply wrapping/layout decisions (Appendix C; includes genex layout rules).
9. Apply indentation (`indentWidth`, `indentStyle`, continuation/genex indentation).
10. Apply flow-control shaping (`indentBlockBody`, then `endCommandArgs`).
11. Apply parenthesis spacing/newline options (§5).
12. Apply blank-line policies (`minBlankLinesBetweenBlocks`, `blankLineBetweenSections`, `maxBlankLines`).
13. Apply alignment options (`alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`, `alignTrailingComments`).
14. Apply comment formatting/preservation behavior (`commentPreservation`, `commentWidth`, `commentGap`).
15. Trim trailing whitespace (`trimTrailingWhitespace`).
16. Emit configured line endings (`lineEnding`).
17. Enforce final newline policy (`finalNewline`).

Rows in §E.2 describe pairwise consequences of this order.

### E.2 Pairwise interaction rules

| Options                                               | Interaction                                                                                                                                                                                                                            |
| ----------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `sortArguments` + `alignArgGroups`                    | Sorting is applied first, then alignment. Arguments are reordered within their keyword section, and the resulting layout is then column-aligned if `alignArgGroups` is enabled.                                                        |
| `indentStyle = "tab"` + alignment options             | When `indentStyle = "tab"`, alignment padding (the spaces after the leading tab indentation that align columns) always uses space characters, never tabs. Leading indentation uses tabs; alignment columns use spaces.                 |
| `disableFormatting = true` + all other options        | `disableFormatting` takes absolute precedence. Output is byte-for-byte identical to input. No other option has any effect.                                                                                                             |
| `wrapStyle = "vertical"` + `wrapArgThreshold`         | `wrapArgThreshold` forces expansion regardless of `wrapStyle`. Under `"vertical"`, the result is always one-per-line (Step 3). Under `"cascade"`, `wrapArgThreshold` skips directly to Step 3. In practice, the outcome is identical.  |
| `genexWrap = "never"` + `genexClosingAngleNewline`    | `genexClosingAngleNewline` has no effect when `genexWrap = "never"` because generator expressions are always single-line.                                                                                                              |
| `genexWrap = "never"` + `genexIndentWidth`            | `genexIndentWidth` has no effect when `genexWrap = "never"` because generator expressions are never expanded to multiple lines.                                                                                                        |
| `commentPreservation = "preserve"` + `commentWidth`   | `commentWidth` has no effect unless `commentPreservation = "reflow"`.                                                                                                                                                                  |
| `firstArgSameLine` + single-line commands             | `firstArgSameLine` only affects commands that wrap. Single-line commands always have the first argument on the same line.                                                                                                              |
| `closingParenNewline` + single-line commands          | `closingParenNewline` only affects commands that wrap. Single-line commands always have `)` on the same line.                                                                                                                          |
| `blankLineBetweenSections` + single-section commands  | No blank lines inserted when a command has only one section.                                                                                                                                                                           |
| `alignPropertyValues` + single-line property commands | `alignPropertyValues` only takes effect when properties are rendered one-per-line.                                                                                                                                                     |
| `perCommandConfig` + `push` pragma                    | Per §13.4.4: `push` stack overrides always take priority over `perCommandConfig`.                                                                                                                                                      |
| `ignoreCommands` + `perCommandConfig`                 | If a command is in `ignoreCommands`, it is preserved verbatim. `perCommandConfig` entries for that command are not applied.                                                                                                            |
| `ignoreCommands` + pragmas (`off`/`skip`)             | Both suppress formatting. `ignoreCommands` applies globally to all invocations; pragmas apply to specific locations. They do not conflict — either one is sufficient to suppress formatting.                                           |
| `finalNewline` + `maxBlankLines`                      | `maxBlankLines` enforces its limit on consecutive blank lines at EOF regardless of `finalNewline`. `finalNewline = false` only controls whether a missing trailing newline is added.                                                   |
| `ignoreCommands` + sorting/alignment                  | Ignored commands are preserved verbatim; `sortArguments`, `sortKeywordSections`, and alignment options do not apply to them.                                                                                                           |
| `blankLineBetweenSections` + `maxBlankLines`          | When `blankLineBetweenSections = true`, the inserted blank lines take precedence over `maxBlankLines` within argument lists, analogous to `minBlankLinesBetweenBlocks` (§3.2).                                                         |
| `alignTrailingComments` + `commentGap`                | When `alignTrailingComments = true`, the alignment column uses `commentGap` as the minimum gap between the longest code segment and the `#` marker.                                                                                    |
| `collapseSpaces` + alignment options                  | `collapseSpaces` applies during input normalization (before layout). Alignment-generated padding (`alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`, `alignTrailingComments`) is inserted after collapsing and is exempt. |
| `alignArgGroups` + `blankLineBetweenSections`         | When both are enabled, `blankLineBetweenSections` inserts blank lines between sections first, then `alignArgGroups` detects column patterns within each section independently. The blank line acts as an alignment group boundary.     |
| `maxBlankLines` + `sortArguments`                     | Blank lines inside argument lists that serve as sorting group boundaries (§12.1) are preserved when `sortArguments` is enabled, overriding the normal §3.1 rule that discards blank lines inside argument lists.                       |
| `sortKeywordSections` + `blankLineBetweenSections`    | When both are enabled, section reordering (`sortKeywordSections`) is applied first, then blank-line insertion (`blankLineBetweenSections`).                                                                                            |
