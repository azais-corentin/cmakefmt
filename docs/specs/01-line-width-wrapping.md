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

The formatter never breaks within a single token (command name, argument, string literal).
If a command exceeds `lineWidth` and contains a token wider than `lineWidth`, the command
still wraps to multi-line layout; the oversized token occupies its own line.

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

  This applies recursively to all nesting levels.

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

If the command name plus the first argument together exceed `lineWidth` when `firstArgSameLine = true`, the opening line is emitted as-is (no individual token is split). The overflow is tolerated on the opening line; remaining arguments wrap normally per the cascade algorithm, including keyword inline layout.

### 1.4 `wrapArgThreshold`

|             |                |
| ----------- | -------------- |
| **Type**    | `integer`      |
| **Default** | `0` (disabled) |
| **Range**   | `0 – 999`      |

When set to a value > 0, forces the command to multi-line layout (skips Step 1 of the cascade)
whenever a command invocation has **more than** this many arguments, regardless of whether it would
fit within `lineWidth`. Within the multi-line layout, keyword groups follow normal cascade Step 2/3
rules — each keyword group independently decides between inline and expanded layout.
The command name is not counted. Only tokens inside the parentheses count — this includes
keywords and value arguments, including the first positional argument (e.g., the target name). For example,
`target_link_libraries(MyTarget PRIVATE foo bar)` has 4 arguments (`MyTarget`, `PRIVATE`, `foo`, `bar`).
A value of `4` means any command with 5+ arguments always wraps.
A generator expression counts as a single argument regardless of its internal structure.

Useful for keeping commands like `set()` compact while forcing long `target_link_libraries()` invocations to expand.

```cmake
# Input:
set(MY_VAR a b c d e)

# Output (wrapArgThreshold = 4): 6 arguments > 4, forced multi-line
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
