## 11 · Command-Specific Overrides

### 11.1 `perCommandConfig`

|             |                                    |
| ----------- | ---------------------------------- |
| **Type**    | `table { [commandName]: { ... } }` |
| **Default** | `{}`                               |

Allows overriding a subset of formatting options on a per-command basis. The command name
Any option from groups 1 (wrapping), 2 (indentation),
4 (casing), 5 (parentheses & spacing), 6 (comments), 9 (alignment), and 12 (sorting) may be overridden. Per-command `lineWidth` replaces the
former `commandWidthOverrides` option.

File-level concerns (blank lines, line endings, whitespace normalization, formatting
suppression, configuration meta) are excluded from per-command overrides because they
apply to the document as a whole, not to individual command invocations.

```toml
[perCommandConfig.set]
wrapStyle = "vertical"

[perCommandConfig.target_link_libraries]
wrapStyle = "vertical"
closingParenNewline = true
lineWidth = 120

[perCommandConfig.if]
spaceBeforeParen = true
```

When pragma `push` overrides are active (§13.4), the current stack frame takes priority
over `perCommandConfig`. See §13.4.4 for the full resolution order.

The exact options overridable via `perCommandConfig` are: `lineWidth`, `wrapStyle`,
`firstArgSameLine`, `wrapArgThreshold`, `indentWidth`,
`indentStyle`, `continuationIndentWidth`, `commandCase`, `keywordCase`,
`customKeywords`, `literalCase`, `closingParenNewline`, `spaceBeforeParen`,
`spaceInsideParen`, `commentPreservation`, `commentWidth`,
`alignTrailingComments`, `commentGap`, `alignPropertyValues`, `alignConsecutiveSet`,
`alignArgGroups`, `sortArguments`, and `sortKeywordSections`.
