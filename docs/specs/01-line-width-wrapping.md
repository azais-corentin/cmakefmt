## 1 · Line Width & Wrapping

### 1.1 `lineWidth`

|             |            |
| ----------- | ---------- |
| **Type**    | `integer`  |
| **Default** | `80`       |
| **Range**   | `40 – 320` |

Maximum number of columns per line. The formatter uses a **cascading wrapping strategy**
controlled by `wrapStyle` (§1.2) to decide how to break a command invocation that exceeds
this width. Comments are also reflowed to respect this limit unless they are inside a
`# cmakefmt: off` region.

Default 80 matches common convention across build-system files and terminal widths.

When a single token (command name, argument, string literal) exceeds `lineWidth`, the line
is emitted as-is. The formatter never breaks within a single token.

When indentation alone exceeds `lineWidth` due to deep nesting, the line is emitted at the computed indentation column regardless. The formatter never caps or reduces indentation.

### 1.2 `wrapStyle`

|             |                           |
| ----------- | ------------------------- |
| **Type**    | `"cascade" \| "vertical"` |
| **Default** | `"cascade"`               |

Controls the overall line-wrapping philosophy. This is the master switch for wrapping behavior.

- **`"cascade"`** (default) — Three-step strategy:
  1. *Fit on single line* if total width ≤ `lineWidth`.
  2. *Keywords on new lines, arguments inline* — each keyword starts a new line, and its
     arguments are packed onto that line as space allows.
  3. *One argument per line* — if step 2 still overflows for a keyword group, that group escalates to one argument per line. Each keyword group independently decides between Step 2 and Step 3 based on its own rendered width.

  This applies recursively to all nesting levels, including generator expressions.

- **`"vertical"`** — Equivalent to step 1 → step 3 directly (skip step 2). Produces a
  strictly vertical style whenever a command does not fit on a single line.

  When `wrapStyle = "vertical"`, keyword/value indentation hierarchy is identical to cascade Step 3 —
  keywords are indented by `indentWidth` relative to the command, and values by `continuationIndentWidth`
  (or `indentWidth`) relative to the keyword.

  ```cmake
  # wrapStyle = "vertical" — fits on one line, stays on one line
  set(MY_VAR "hello")

  # wrapStyle = "vertical" — does not fit, every argument on its own line
  target_link_libraries(MyTarget
    PRIVATE
      Boost::filesystem
      Threads::Threads
    PUBLIC
      some_other_lib
  )
  ```

### 1.3 `firstArgSameLine`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

Controls whether the first positional argument (typically the target name) stays on the
same line as the command name.

For commands without recognized keywords (e.g., `set(VAR val1 val2 val3)`), the first argument is
the first token after the opening `(`. When `firstArgSameLine = true`, this token stays on the
opening line regardless of wrapping.

- `true` (default): The first argument always sits on the opening line.

  ```cmake
  target_sources(MyProgram
    PRIVATE
      main.cpp
  )
  ```

- `false`: The first argument moves to the next line when the command wraps.

  ```cmake
  target_sources(
    MyProgram
    PRIVATE
      main.cpp
  )
  ```

When `firstArgSameLine = false`, the first argument is indented by `indentWidth` relative to the command (at the same level as keywords).

If the command name plus the first argument together exceed `lineWidth` when `firstArgSameLine = true`, the opening line is emitted as-is (no individual token is split). The overflow is tolerated on the opening line; remaining arguments still wrap normally per the cascade algorithm.

### 1.4 `wrapArgThreshold`

|             |                |
| ----------- | -------------- |
| **Type**    | `integer`      |
| **Default** | `0` (disabled) |
| **Range**   | `0 – 999`      |

When set to a value > 0, forces wrapping to one-arg-per-line whenever a command invocation
has **more than** this many arguments, regardless of whether it would fit within `lineWidth`.
The command name is not counted. Only tokens inside the parentheses count — this includes
keywords and value arguments, including the first positional argument (e.g., the target name). For example,
`target_link_libraries(MyTarget PRIVATE foo bar)` has 4 arguments (`MyTarget`, `PRIVATE`, `foo`, `bar`).
A value of `4` means any command with 5+ arguments always wraps.
A generator expression counts as a single argument regardless of its internal structure.

Useful for keeping commands like `set()` compact while forcing long `target_link_libraries()` invocations to expand.

```cmake
# Input:
set(MY_VAR a b c d e)

# Output (wrapArgThreshold = 4): 6 arguments > 4, forced one-per-line
set(MY_VAR
  a
  b
  c
  d
  e
)

# Output (wrapArgThreshold = 0, default): fits on one line, no forced wrapping
set(MY_VAR a b c d e)
```

### 1.5 `magicTrailingNewline`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, a "magic trailing newline" is treated as an explicit signal to prevent single-line collapse, even if the invocation would fit on a single line. A magic trailing
newline is detected in the **input** when the closing `)` appears on its own line — that is, preceded only by whitespace (or nothing) on that line. This is an input-layout signal; it does not describe the output format.

A trailing newline on a wrapped invocation serves as an author-intent signal to prevent single-line collapse.

Magic trailing newline skips Step 1 (single-line) only. The cascade still attempts Step 2 (keyword breaks) for keyword-bearing commands before escalating to Step 3. Under `wrapStyle = "vertical"`, since Step 2 does not exist, magic trailing newline skips Step 1 and proceeds directly to Step 3 (one-per-line). For example:

```cmake
# Input (magic trailing newline detected):
target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem
)

# Output (magicTrailingNewline = true): Step 2 is still attempted
target_link_libraries(MyTarget
  PRIVATE Boost::filesystem
)
```

Magic trailing newline detection requires at least one argument. Empty commands (e.g., `endif(\n)`) are not affected.

When `false`, the formatter collapses any invocation that fits onto a single line regardless
of the original layout.

```cmake
# Input (closing paren on its own line — magic trailing newline detected):
set(FOO
  "bar"
)

# Output (magicTrailingNewline = true): stays expanded even though it fits on one line
set(FOO
  "bar"
)

# Output (magicTrailingNewline = false): collapsed because it fits
set(FOO "bar")
```
