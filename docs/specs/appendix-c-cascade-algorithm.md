## Appendix C — Cascading Wrap Algorithm Detail

The cascading algorithm is the heart of the formatter. For the vertical variant (`wrapStyle = "vertical"`), see the Vertical Wrapping Variant subsection below. Given a command invocation:

```
command_name(arg1 KEYWORD arg2 arg3 KEYWORD2 arg4)
```

**Step 0 — Pre-checks.** Before attempting single-line layout, one condition forces expansion:

- If `wrapArgThreshold > 0` and the command has more than `wrapArgThreshold` arguments, skip Step 1 (single-line). The command proceeds to Step 2, where keyword groups independently decide between inline and expanded layout per normal cascade rules.

**Step 1 — Single line.** Compute the rendered width of the entire line, including block-nesting indentation (the leading whitespace from enclosing `if`/`foreach`/`function`/etc. blocks). If ≤ `lineWidth`, emit on one line.

**Step 2 — Keyword breaks.** Place each keyword on a new line, indented by `indentWidth`
relative to the command's own indentation column. Pack that keyword's value arguments onto the same line (inline layout). If the keyword plus its packed values still exceeds `lineWidth`, place the keyword on its own line with values packed on the next line, indented by `continuationIndentWidth`. If the packed values line width ≥ `lineWidth`, escalate to Step 3 for that keyword group (one value per line).

**Pre-keyword positional arguments** (arguments before the first keyword, e.g., the target name) form an implicit group. With `firstArgSameLine = true`, the first positional argument stays on the opening line and remaining pre-keyword positional arguments pack onto the same line. If the packed line exceeds `lineWidth`, pre-keyword positional arguments escalate to one-per-line (same escalation rule as keyword groups). With `firstArgSameLine = false`, all pre-keyword positional arguments start on the line after the opening `(` and follow the same packing/escalation rules.

```cmake
target_link_libraries(MyTarget
  PRIVATE Boost::filesystem Threads::Threads
  PUBLIC some_other_lib
)
```

When `firstArgSameLine = false`, if any keyword group requires expanded layout (keyword on its own line, values on next line), subsequent keyword groups also expand for visual consistency.

Step 2 with multiple pre-keyword positional arguments:

```cmake
# firstArgSameLine = true — pre-keyword args pack on the opening line
some_command(arg1 arg2 arg3
  KEYWORD val1 val2
)

# firstArgSameLine = false — pre-keyword args start on next line, pack together
some_command(
  arg1 arg2 arg3
  KEYWORD val1 val2
)
```

**Step 3 — One per line.** Each argument occupies its own line:

```cmake
target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem
    Threads::Threads
  PUBLIC
    some_other_lib
)
```

**Commands without keywords.** For commands like `set()` that have no recognized keywords, Step 2 is not applicable (there are no keywords to break on). The cascade goes directly from Step 1 to Step 3:

```cmake
# Step 1 — fits on one line
set(MY_VAR a b c)

# Step 3 — does not fit, one argument per line
set(MY_VAR
  a_long_value
  another_long_value
  yet_another_value
)
```

When `firstArgSameLine` is `true`, `MyTarget` stays on the opening line in all steps.

**`firstArgSameLine = false` variant.** When `firstArgSameLine` is `false`, the cascade
proceeds identically but the first argument moves to the next line:

Step 1 — Single line (unchanged, argument stays inline when it fits):

```cmake
target_link_libraries(MyTarget PRIVATE Boost::filesystem)
```

Step 2 — Keyword breaks with first argument on next line:

```cmake
target_link_libraries(
  MyTarget
  PRIVATE Boost::filesystem Threads::Threads
  PUBLIC some_other_lib
)
```

Step 3 — One per line with first argument on next line:

```cmake
target_link_libraries(
  MyTarget
  PRIVATE
    Boost::filesystem
    Threads::Threads
  PUBLIC
    some_other_lib
)
```

Generator expressions are treated as atomic tokens and are not recursed into by the cascade algorithm.

**`closingParenNewline = false` interaction.** When `closingParenNewline` is `false`, the inline `)` occupies space on the last argument's line. The cascade algorithm includes this extra width — 1 character for `)`, plus the trailing-comment width when the command has a deferred trailing comment — in Steps 2 and 3 line-width calculations. This can allow more arguments to pack onto a single line compared to `closingParenNewline = true`, where `)` occupies its own line and does not contribute to argument-line width.

### Vertical Wrapping Variant

When `wrapStyle = "vertical"` (§1.2), the algorithm simplifies:

**Step 0 — Pre-checks.** Identical to cascade Step 0: the `wrapArgThreshold` pre-check applies. If `wrapArgThreshold` triggers, skip Step 1 and proceed to Step 3 (one-per-line).

**Step 1 — Single line.** Identical to cascade Step 1: if the entire invocation fits within `lineWidth`, emit on one line.

**Step 3 — One per line.** If Step 1 fails (or is skipped), go directly to one-argument-per-line layout. Step 2 (keyword breaks with packed arguments) is never attempted.

Indentation rules are identical to cascade Step 3: keywords indent by `indentWidth` relative to the command, values indent by `continuationIndentWidth` (or `indentWidth`) relative to the keyword.

```cmake
# wrapStyle = "vertical" — fits on one line
set(MY_VAR "hello")

# wrapStyle = "vertical" — does not fit, one-per-line
target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem
    Threads::Threads
  PUBLIC
    some_other_lib
)
```
