## Appendix C — Cascading Wrap Algorithm Detail

The cascading algorithm is the heart of the formatter. For the vertical variant (`wrapStyle = "vertical"`), see the Vertical Wrapping Variant subsection below. Given a command invocation:

```
command_name(arg1 KEYWORD arg2 arg3 KEYWORD2 arg4)
```

**Step 0 — Pre-checks.** Before attempting single-line layout, two conditions force expansion:

- If `wrapArgThreshold > 0` and the command has more than `wrapArgThreshold` arguments, skip directly to Step 3 (one-per-line).
- If `magicTrailingNewline = true` (§1.5) and the closing `)` was on its own line in the input, skip Step 1 (single-line) — do not collapse to a single line.

**Step 1 — Single line.** Compute the rendered width of the entire line, including block-nesting indentation (the leading whitespace from enclosing `if`/`foreach`/`function`/etc. blocks). If ≤ `lineWidth`, emit on one line.

**Step 2 — Keyword breaks.** Place each keyword on a new line, indented by `indentWidth`
relative to the command's own indentation column. Pack that keyword's value arguments onto the same line. If any keyword
line still exceeds `lineWidth`, escalate to Step 3 for that keyword group.

**Pre-keyword positional arguments** (arguments before the first keyword, e.g., the target name) form an implicit group. With `firstArgSameLine = true`, the first positional argument stays on the opening line and remaining pre-keyword positional arguments pack onto the same line. If the packed line exceeds `lineWidth`, pre-keyword positional arguments escalate to one-per-line (same escalation rule as keyword groups). With `firstArgSameLine = false`, all pre-keyword positional arguments start on the line after the opening `(` and follow the same packing/escalation rules.

```cmake
target_link_libraries(MyTarget
  PRIVATE Boost::filesystem Threads::Threads
  PUBLIC some_other_lib
)
```

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

The algorithm recurses into generator expressions, treating `$<` as an opening bracket
and `>` as a closing bracket, with `genexIndentWidth` controlling nested indentation.

### Vertical Wrapping Variant

When `wrapStyle = "vertical"` (§1.2), the algorithm simplifies:

**Step 0 — Pre-checks.** Identical to cascade Step 0: `wrapArgThreshold` and `magicTrailingNewline` pre-checks apply. If `wrapArgThreshold` triggers, skip directly to Step 3. If `magicTrailingNewline` triggers, skip Step 1.

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
