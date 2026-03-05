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

---

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

---

## 2 · Indentation

### 2.1 `indentWidth`

|             |           |
| ----------- | --------- |
| **Type**    | `integer` |
| **Default** | `2`       |
| **Range**   | `1 – 8`   |

Number of spaces (or tab stops) per indentation level. Each nesting level — keywords under
a command, values under a keyword, nested generator-expression arguments, and body blocks of
`if`/`foreach`/`function`/`macro` — increases indentation by this amount.

```cmake
# indentWidth = 2 (default)
target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem
)

# indentWidth = 4
target_link_libraries(MyTarget
    PRIVATE
        Boost::filesystem
)
```

### 2.2 `indentStyle`

|             |                    |
| ----------- | ------------------ |
| **Type**    | `"space" \| "tab"` |
| **Default** | `"space"`          |

Whether to indent with spaces or hard tab characters. When set to `"tab"`, a single `\t`
is emitted per indent level; `indentWidth` then represents the *visual width* of one tab
for the purpose of line-width calculation.

When set to `"space"` (default), any tab characters in the input are converted to
`indentWidth` spaces. When set to `"tab"`, indentation uses tab characters; tab characters
within quoted strings are always preserved regardless of this setting.

```cmake
# indentStyle = "space", indentWidth = 2
target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem
)

# indentStyle = "tab" (→ represents a tab character)
target_link_libraries(MyTarget
→PRIVATE
→→Boost::filesystem
)
```

(In the tab example above, `→` represents a tab character.)

When `indentStyle = "tab"` and `continuationIndentWidth` or `genexIndentWidth` differs from `indentWidth`, the formatter uses tabs for whole-`indentWidth` multiples and spaces for the remainder. For example, with `indentWidth = 4` and `continuationIndentWidth = 6`: a keyword at depth 1 uses 1 tab (4 columns), and a value line at the keyword's depth + 6 columns = column 10 is rendered as 2 tabs + 2 spaces. This "tabs for indentation, spaces for alignment" strategy ensures consistent display regardless of tab-width settings.

### 2.3 `continuationIndentWidth`

|             |                                |
| ----------- | ------------------------------ |
| **Type**    | `integer \| null`              |
| **Default** | `null` (inherit `indentWidth`) |
| **Range**   | `1 – 8`                        |

Indentation applied to value lines under a keyword within a wrapped command. A "continuation
line" is a value argument that appears on a subsequent line beneath its keyword. The
`continuationIndentWidth` is measured **relative to the keyword**, not relative to the
command name. If `null`, inherits the value of `indentWidth`.

Set explicitly when you want value arguments indented more than the structural indent
(common in projects using 2-space indent but 4-space continuation).

```cmake
# indentWidth = 2, continuationIndentWidth = 4
target_link_libraries(MyTarget
  PRIVATE
      Boost::filesystem
      Threads::Threads
)
```

In this example, `PRIVATE` is indented by `indentWidth` (2) relative to the command,
and the library names are indented by `continuationIndentWidth` (4) relative to `PRIVATE`.

For commands without recognized keywords, arguments are indented by `indentWidth` (not `continuationIndentWidth`). `continuationIndentWidth` only applies to value arguments appearing under a recognized keyword.

### 2.4 `genexIndentWidth`

|             |                                |
| ----------- | ------------------------------ |
| **Type**    | `integer \| null`              |
| **Default** | `null` (inherit `indentWidth`) |
| **Range**   | `1 – 8`                        |

Override indentation specifically inside generator expressions (`$<...>`). Generator
expressions can be deeply nested, and some teams prefer a narrower indent inside them
to reduce rightward drift. The indent is relative to the column where `$<` starts, not
relative to the beginning of the line.

```cmake
# genexIndentWidth = 2 (with indentWidth = 4)
target_compile_definitions(MyLib
    PRIVATE
        $<$<CONFIG:Debug>:
          DEBUG_MODE=1
          VERBOSE_LOG=1
        >
)
```

---

## 3 · Blank Lines


A **blank line** is a line containing no characters, or only whitespace characters, before the line ending.

### 3.1 `maxBlankLines`

|             |           |
| ----------- | --------- |
| **Type**    | `integer` |
| **Default** | `1`       |
| **Range**   | `0 – 100` |

Maximum number of **consecutive** blank lines preserved between top-level statements.
Runs exceeding this count are collapsed. A value of `0` collapses *all* blank lines between statements.
See §3.2 for interaction with `minBlankLinesBetweenBlocks`.

Author-placed blank lines inside command argument lists are discarded during reformatting, except where they serve as sorting group boundaries (§12.1) — those blank lines are preserved when `sortArguments` is enabled. Only `blankLineBetweenSections` (§3.3) can insert blank lines within a command's argument list. `maxBlankLines` does not apply inside argument lists.

Leading blank lines at the beginning of the file are always stripped entirely, independent of `maxBlankLines`. Trailing blank lines at end-of-file are also subject to this limit.

> **Note:** Leading blank lines are removed unconditionally — even `maxBlankLines = 100` does not preserve them.

```cmake
# Input (3 consecutive blank lines):
set(FOO "bar")



set(BAZ "qux")

# Output (maxBlankLines = 1):
set(FOO "bar")

set(BAZ "qux")
```

### 3.2 `minBlankLinesBetweenBlocks`

|             |           |
| ----------- | --------- |
| **Type**    | `integer` |
| **Default** | `0`       |
| **Range**   | `0 – 10`  |

Minimum number of blank lines inserted between block-opening commands
(`if`, `foreach`, `while`, `function`, `macro`, `block`) and the preceding statement at any nesting level. This applies inside block bodies (e.g., inside `function()`, `if()`, `foreach()`) as well as at the file root. Ensures visual separation of logical sections.

A value of `1` guarantees at least one blank line before every `if()`, `foreach()`,
`while()`, `function()`, `macro()`, or `block()` block, unless the preceding line is a comment that belongs
to the block.

A comment "belongs to" the immediately following block if there is no blank line between the
comment and the block-opening command. A group of consecutive comment lines (line comments
`# ...` or bracket comments `#[[ ... ]]`) with no blank lines between them and the
block-opening command are all considered attached to the block. The blank-line insertion
point moves above the topmost attached comment.

The minimum applies only before block-opening commands. Block-closing commands (`endif`,
`endforeach`, `endwhile`, `endfunction`, `endmacro`, `endblock`) are excluded — the
minimum blank-line guarantee does not apply before them.

When a block-opening command is the first statement in the file (or the first statement inside a block body after the opener), `minBlankLinesBetweenBlocks` does not insert blank lines before it — there is no preceding statement to separate from.

When `minBlankLinesBetweenBlocks` exceeds `maxBlankLines`, `minBlankLinesBetweenBlocks`
takes precedence at block boundaries. The formatter inserts the minimum required blank
lines even if this exceeds `maxBlankLines`.

```cmake
# Input (no blank line before if):
set(FOO "bar")
if(FOO)
  message(STATUS "yes")
endif()

# Output (minBlankLinesBetweenBlocks = 1):
set(FOO "bar")

if(FOO)
  message(STATUS "yes")
endif()
```

```cmake
# Input (minBlankLinesBetweenBlocks = 1):
set(FOO "bar")
# This comment is attached to the if block
# So is this one
if(FOO)
  message(STATUS "yes")
endif()

# Output:
set(FOO "bar")

# This comment is attached to the if block
# So is this one
if(FOO)
  message(STATUS "yes")
endif()
```

### 3.3 `blankLineBetweenSections`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, insert a blank line after the last argument of a section, before the next
section keyword — serving as a visual separator between sections. This applies to section
keywords recognized by the formatter's keyword dictionary (e.g., `PUBLIC`, `PRIVATE`,
`INTERFACE`, `SOURCES`, `DEPENDS`, `PROPERTIES`, and others; see Appendix F) inside commands that contain
multiple sections.

The blank line is inserted *between* sections — not before the first section keyword in a
command. If a command has only one section, this option has no effect.

A blank line is always inserted between consecutive section keywords, even when the preceding section has zero arguments (e.g., a standalone keyword like `VERBATIM`).

When `blankLineBetweenSections = true`, the inserted blank lines take precedence over `maxBlankLines` within argument lists, analogous to `minBlankLinesBetweenBlocks` (§3.2). See Appendix E for interaction details.

```cmake
# blankLineBetweenSections = true
target_link_libraries(MyTarget
  PUBLIC
    Boost::filesystem

  PRIVATE
    internal_lib
)
```

---

## 4 · Casing

### 4.1 `commandCase`

|             |                                     |
| ----------- | ----------------------------------- |
| **Type**    | `"lower" \| "upper" \| "unchanged"` |
| **Default** | `"lower"`                           |

Casing applied to command names (`cmake_minimum_required`, `add_library`, `if`, etc.).

- `"lower"`: Lowercases all commands. `CMAKE_MINIMUM_REQUIRED(...)` → `cmake_minimum_required(...)`.
- `"upper"`: Uppercases all commands.
- `"unchanged"`: Preserves the original casing from the source file.

```cmake
# Input:
SET(FOO "bar")
CMAKE_MINIMUM_REQUIRED(VERSION 3.20)

# Output (commandCase = "lower"):
set(FOO "bar")
cmake_minimum_required(VERSION 3.20)
```

### 4.2 `keywordCase`

|             |                                     |
| ----------- | ----------------------------------- |
| **Type**    | `"lower" \| "upper" \| "unchanged"` |
| **Default** | `"upper"`                           |

Casing applied to recognized keywords. The formatter ships with a built-in keyword
dictionary covering standard CMake keywords (e.g., `VERSION`, `PRIVATE`, `PUBLIC`,
`INTERFACE`, `PROPERTIES`, `REQUIRED`, `COMPONENTS`, `CONFIG`, `TARGETS`,
`DESTINATION`, `NAMESPACE`). The full keyword dictionary is defined in the source code
(`src/generation/signatures.rs`) and is the authoritative reference.

- `"upper"` (default): All keywords uppercased.
- `"lower"`: All keywords lowercased.
- `"unchanged"`: Preserve original casing.

### 4.3 `customKeywords`

|             |            |
| ----------- | ---------- |
| **Type**    | `string[]` |
| **Default** | `[]`       |

Additional strings that should be treated as keywords (and subjected to `keywordCase`
normalization). Useful for project-specific or third-party module keywords not in the
built-in dictionary.

```toml
customKeywords = ["CONAN_PKG", "VCPKG_DEPS", "MY_OPTION"]
```

Custom keywords participate in section detection, argument sorting (§12.1), and formatting layout — not only casing normalization. Adding a keyword to this list causes the formatter to treat it as a section header in all commands.

If a token in `customKeywords` also appears in the built-in literal list (§4.4), keyword classification takes precedence — the token is treated as a keyword in all contexts and `literalCase` does not apply to it.

### 4.4 `literalCase`

|             |                                     |
| ----------- | ----------------------------------- |
| **Type**    | `"upper" \| "lower" \| "unchanged"` |
| **Default** | `"unchanged"`                       |

Unlike keywords (§4.2), which are structural tokens that affect formatting layout (e.g., PRIVATE, PUBLIC, SOURCES), literals are boolean and comparison constants used as argument values.

Casing applied to well-known boolean/constant literals: `ON`, `OFF`, `TRUE`, `FALSE`,
`YES`, `NO`, `AND`, `OR`, `NOT`, `STREQUAL`, `STRLESS`, `STRGREATER`, `STRLESS_EQUAL`,
`STRGREATER_EQUAL`, `VERSION_EQUAL`, `VERSION_LESS`, `VERSION_GREATER`,
`VERSION_LESS_EQUAL`, `VERSION_GREATER_EQUAL`, `EQUAL`, `LESS`, `GREATER`,
`LESS_EQUAL`, `GREATER_EQUAL`, `MATCHES`, `IN_LIST`, `DEFINED`, `COMMAND`,
`POLICY`, `TARGET`, `TEST`, `EXISTS`, `IS_DIRECTORY`, `IS_SYMLINK`,
`IS_ABSOLUTE`, `IS_NEWER_THAN`, `PATH_EQUAL`.

Normalizing these literals to uppercase is a common convention, but many projects prefer leaving them as-is.

These tokens are normalized *everywhere* they appear as unquoted arguments, not only in condition contexts.
Any unquoted argument matching one of the listed tokens (case-insensitive match) is subject to `literalCase` normalization.
Literal case normalization applies only to unquoted arguments. Content inside quoted strings is never modified.

When a token appears in both the keyword dictionary and the literal list (e.g., `TARGET`, `COMMAND`, `POLICY`, `TEST`), keyword classification takes precedence when the token is parsed as a keyword (not as a value argument to another keyword) according to the current command's keyword dictionary. `literalCase` only applies when the token is used as a plain argument value.

```cmake
# Input:
option(USE_FEATURE "Enable feature" on)
if(DEFINED result AND off)

# Output (literalCase = "upper"):
option(USE_FEATURE "Enable feature" ON)
if(DEFINED result AND OFF)
```

---

## 5 · Parentheses & Spacing

### 5.1 `closingParenNewline`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When a command invocation spans multiple lines, place the closing `)` on its own line at
the current block's base indentation.

```cmake
# closingParenNewline = true (default)
target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem
)

# closingParenNewline = false
target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem)
```

When `false`, the `)` stays on the last argument's line. If the last argument has a trailing comment, `)` is inserted *before* the `#` marker (between the argument and the comment), separated by `commentGap` spaces from `)` to `#`. This prevents `)` from being swallowed into the comment text.

```cmake
# closingParenNewline = false, last argument has trailing comment
target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem) # link filesystem
```

### 5.2 `spaceBeforeParen`

|             |                       |
| ----------- | --------------------- |
| **Type**    | `boolean \| string[]` |
| **Default** | `false`               |

Controls whether a space is inserted between the command name and the opening `(`.

- `false` (default): No space — `if(...)`.
- `true`: Space for all commands — `if (...)`.
- `["if", "elseif", "while", "foreach"]`: Space only for the listed commands.

Command names in the list are matched case-insensitively.

### 5.3 `spaceInsideParen`

|             |                                      |
| ----------- | ------------------------------------ |
| **Type**    | `"insert" \| "remove" \| "preserve"` |
| **Default** | `"remove"`                           |

Controls whitespace immediately after the opening `(` and before the closing `)` on
single-line invocations.

- **`"insert"`**: Insert a space after `(` and before `)` on single-line commands.

  ```cmake
  # spaceInsideParen = "insert"
  set( MY_VAR "hello" )
  ```

- **`"remove"`** (default): Remove any space after `(` and before `)` on single-line commands.

  ```cmake
  # spaceInsideParen = "remove"
  set(MY_VAR "hello")
  ```

- **`"preserve"`**: Keep whatever spacing was in the input.

  ```cmake
  # spaceInsideParen = "preserve" — input preserved as-is
  set( MY_VAR "hello" )  # stays as-is
  set(MY_VAR "hello")    # stays as-is
  ```

Does not apply to multi-line commands (spacing is controlled by indentation in that case).

When a multi-line command collapses to a single line (e.g., because its arguments now fit), `"preserve"` mode has no meaningful input spacing to preserve. In this case, the formatter treats `"preserve"` as `"remove"` — no space is inserted after `(` or before `)`.

`spaceInsideParen` has no effect on empty argument lists — `cmd()` stays `cmd()`.

---

## 6 · Comments

### 6.1 `commentPreservation`

|             |                          |
| ----------- | ------------------------ |
| **Type**    | `"preserve" \| "reflow"` |
| **Default** | `"preserve"`             |

Controls how the formatter handles comments.

- **`"preserve"`** (default): Comments are kept in-place. Inline comments within argument
  lists are re-indented to match surrounding arguments. Standalone comment lines between
  commands are preserved verbatim (content and relative position unchanged). Standalone
  comment lines inside argument lists (between arguments) are re-indented to match the
  surrounding indentation level and prevent single-line collapse of the command.

  ```cmake
  # commentPreservation = "preserve" — standalone comment prevents collapse
  # Input:
  set(FOO
    # This comment prevents single-line collapse
    "bar"
  )

  # Output (not collapsed despite fitting on one line):
  set(FOO
    # This comment prevents single-line collapse
    "bar"
  )
  ```

- **`"reflow"`**: Comment text is reflowed to fit within `lineWidth`, respecting the
  current indentation. Leading `#` markers and any initial whitespace pattern are preserved.
  Block comments (consecutive `#` lines) are treated as a single paragraph for reflow.
  Formatting pragma comments (§13) are never reflowed regardless of this setting.

  Paragraph detection follows these rules:

  1. **Paragraph breaks.** A blank comment line (a line containing only `#` with optional trailing whitespace), or a change in leading whitespace pattern after the `#` marker (e.g., indented sub-lists), starts a new paragraph.

  2. **Code blocks.** Lines indented with 4+ spaces relative to the comment block's baseline are treated as code and are not reflowed — they are preserved as-is within the paragraph flow. Additionally, lines enclosed in triple-backtick fences (` ``` `) within a comment block are treated as fenced code blocks: all lines between the opening and closing fence (inclusive) are preserved verbatim and never reflowed. If an opening fence has no matching closing fence before the end of the comment block, all remaining lines from the opening fence to the end of the block are treated as code (greedy match).

  3. **List items.** Lines that begin with a list marker (`-`, `*`, `+`, or a digit followed by `.` or `)`) after the `#` and optional whitespace are treated as individual list items. List items are not reflowed across item boundaries — each item is treated as its own paragraph. Continuation lines of a list item — lines indented deeper than the list marker — belong to the same item paragraph and are exempt from paragraph-break rule (1). Only a new list marker at the same or lesser indentation starts a new paragraph.

  > **Known limitation:** Nested list markers (sub-lists indented under a parent item) are
  > treated as continuation lines of the parent item, not as independent list items. Deeply
  > nested list structures may not reflow as expected.

A file consisting entirely of comments is handled identically — each comment is preserved or reflowed according to this setting.

### 6.2 `commentWidth`

|             |                              |
| ----------- | ---------------------------- |
| **Type**    | `integer \| null`            |
| **Default** | `null` (inherit `lineWidth`) |
| **Range**   | `40 – 320`                   |

Maximum line width for comments specifically. When `null`, inherits `lineWidth`.
Only effective when `commentPreservation` is `"reflow"`.

### 6.3 `alignTrailingComments`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, align trailing comments (`# ...`) on consecutive lines to start at the same
column. The alignment column is determined by the longest code segment in the group plus
a minimum gap of `commentGap` spaces (§6.4).

A group of consecutive lines is broken by any blank line or any line that does not have
a trailing comment. Only lines within the same group are aligned together.

Applies to consecutive lines at the same block-nesting depth (i.e., lines inside the same
`if`/`foreach`/`function`/etc. body). Lines at different nesting depths are never aligned
together. Trailing comments inside wrapped multi-line command argument lists are aligned
within their argument group (a per-keyword-section scope within a command invocation), not
across the entire file.

```cmake
# alignTrailingComments = true
set(FOO "bar")       # The foo variable
set(BAZ_LONG "qux")  # The baz variable
set(X "y")           # Short one

# alignTrailingComments = false (default)
set(FOO "bar") # The foo variable
set(BAZ_LONG "qux") # The baz variable
set(X "y") # Short one
```

### 6.4 `commentGap`

|             |           |
| ----------- | --------- |
| **Type**    | `integer` |
| **Default** | `1`       |
| **Range**   | `0 – 10`  |

Minimum number of spaces between the end of a code token and the start of a trailing `#` comment.
A value of `0` produces no space between the last code token and the `#` marker
(e.g., `set(FOO "bar")# comment`). Note that `commentGap = 0` produces visually unusual
adjacency and may reduce readability; it is primarily useful for niche formatting styles.

```cmake
# Input:
set(FOO "bar")# trailing comment

# Output (commentGap = 1, default):
set(FOO "bar") # trailing comment

# Output (commentGap = 2):
set(FOO "bar")  # trailing comment
```

### 6.5 Verbatim Content *(fixed behavior, not configurable)*

Bracket arguments (`[==[...]==]`) and bracket comments (`#[==[...]==]`) are preserved
verbatim by the formatter — their content is never reformatted, reflowed, or modified in
any way. This applies regardless of any other formatting settings. Only the indentation of
the opening line is adjusted to match the surrounding context.

Double-quoted strings spanning multiple lines are preserved verbatim — their content is
never reformatted or reflowed. Only the opening line's indentation is adjusted to match
the surrounding context.

Line-ending characters inside double-quoted strings and bracket arguments are never normalized
by `lineEnding` (§7.1). They are preserved exactly as they appear in the input.

---

## 7 · Line Endings & Final Newline

### 7.1 `lineEnding`

|             |                            |
| ----------- | -------------------------- |
| **Type**    | `"lf" \| "crlf" \| "auto"` |
| **Default** | `"auto"`                   |

Controls which line-ending sequence is written to the output.

- **`"lf"`**: Unix-style `\n`.
- **`"crlf"`**: Windows-style `\r\n`.
- **`"auto"`**: Detect the dominant line ending in the input file and preserve it.
  "Dominant" means the line-ending sequence (`\n` vs `\r\n`) that occurs most frequently.
  On equal counts, `\n` (LF) wins as tiebreaker.
  If the file has no line endings (e.g., single-line file, empty file, or zero bytes), default to `"lf"`.

Bare carriage returns (`\r` not followed by `\n`) are treated as ordinary characters, not line endings. They are not counted in dominant-line-ending detection and are not normalized by the `lineEnding` option.

### 7.2 `finalNewline`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, ensure the file ends with exactly one trailing newline. Excess trailing
newlines are removed; a missing trailing newline is added. Empty or whitespace-only
files are normalized to a single newline character.
An empty file (zero bytes) is normalized to a single newline character when `finalNewline = true`.

For example, if the input file ends with `cmake_minimum_required(VERSION 3.20)` and no trailing newline, the formatter appends one. If the input ends with two trailing newlines, excess trailing newlines are removed, leaving exactly one.

When `false`, do not add a trailing newline if one is absent. `finalNewline = false` only controls
whether a *missing* trailing newline is added. Existing trailing newlines are not stripped by this
option — however, `maxBlankLines` (§3.1) still enforces its limit on consecutive blank lines at EOF
regardless of `finalNewline`.

When `finalNewline = false` and the input is an empty file (zero bytes), the output is also zero bytes. When `finalNewline = false` and the input is a whitespace-only file, the file is still subject to `trimTrailingWhitespace` and `maxBlankLines`, which may reduce it to empty.

```cmake
# Input (finalNewline = false, maxBlankLines = 1):
set(FOO "bar")



# Output: maxBlankLines collapses 3 blank lines to 1; no trailing newline added
set(FOO "bar")

```

POSIX convention and most editors expect a trailing newline, so the default is `true`.

---

## 8 · Whitespace Normalization

### 8.1 `trimTrailingWhitespace`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, remove any trailing whitespace (spaces, tabs) at the end of every line.
This is standard hygiene for version-controlled files and almost universally desired.

```cmake
# Input (· represents trailing spaces):
set(FOO "bar")···
message(STATUS "hello")·

# Output (trimTrailingWhitespace = true):
set(FOO "bar")
message(STATUS "hello")
```

### 8.2 `collapseSpaces`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, collapse runs of multiple spaces between arguments on the same line
to a single space. `collapseSpaces` applies during input normalization, before alignment.
Alignment-generated padding (from `alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`,
or `alignTrailingComments`) is inserted after collapsing and is therefore exempt. Does not affect
indentation (which is controlled by `indentWidth`) or spaces inside quoted strings.

```cmake
# Input:
set(FOO    "bar"    "baz")

# Output (collapseSpaces = true):
set(FOO "bar" "baz")
```

---

## 9 · Alignment

### 9.1 `alignPropertyValues`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, column-align the values in property key-value pair lists, primarily in
`set_target_properties(...  PROPERTIES ...)`. Property names are left-aligned and values
start at the same column.

```cmake
# alignPropertyValues = true
set_target_properties(MyTarget PROPERTIES
  CXX_STANDARD              17
  CXX_STANDARD_REQUIRED     ON
  POSITION_INDEPENDENT_CODE ON
)

# alignPropertyValues = false (default)
set_target_properties(MyTarget PROPERTIES
  CXX_STANDARD 17
  CXX_STANDARD_REQUIRED ON
  POSITION_INDEPENDENT_CODE ON
)
```

Only takes effect when properties are rendered one-per-line.

For properties with multiple values (e.g., `LINK_LIBRARIES lib1 lib2`), alignment is based on the first value token. Additional values wrap below at the alignment column:

```cmake
# alignPropertyValues = true, multi-value property
set_target_properties(MyTarget PROPERTIES
  CXX_STANDARD              17
  LINK_LIBRARIES            lib1
                            lib2
                            lib3
  POSITION_INDEPENDENT_CODE ON
)
```

### 9.2 `alignConsecutiveSet`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, align the values of consecutive `set()` commands that form a logical group
(not separated by blank lines, standalone comment lines, or non-`set` commands). This applies only to `set()` —
`option()`, `cmake_dependent_option()`, and other variable-setting commands are not included.

```cmake
# alignConsecutiveSet = true
set(FOO    "bar")
set(BAZ    "qux")
set(LONGER "value")

# alignConsecutiveSet = false (default)
set(FOO "bar")
set(BAZ "qux")
set(LONGER "value")
```

All consecutive `set()` calls form an alignment group.
Alignment is based on the first value argument (the second token after `set(`). For example,
`set(FOO "bar")` and `set(BAZ "qux" CACHE STRING "")` align on `"bar"` and `"qux"` respectively.

A blank line, a standalone comment line, or any non-`set()` command breaks the alignment group.

Valueless `set()` calls (e.g., `set(FOO)` which unsets the variable) and keyword-only calls without a value argument (e.g., `set(FOO PARENT_SCOPE)`) are skipped — they are formatted normally (unaligned) in place but do not break the alignment group. Surrounding `set()` calls with values continue to align across the skipped call.

### 9.3 `alignArgGroups`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, and when a command's arguments are laid out one-per-line, detect repeating
structural patterns in the argument list and column-align them. The formatter looks for
groups of consecutive lines that share the same number of tokens and attempts to align
corresponding columns.

More precisely, two consecutive argument lines are candidates for columnar alignment when they contain the same number of CMake arguments (where each argument is a quoted string, unquoted word, or generator expression — each counted as a single argument regardless of internal content). Alignment pads each column with spaces to the maximum width of that column across all lines in the group.

A "token" in this context is a single CMake argument — a quoted string, unquoted argument,
or generator expression (including its entire nested content as one token).

**Detection heuristics:**

- A group requires at least 2 consecutive lines with the same token count.
- Lines where alignment would cause any column to extend beyond `lineWidth` are excluded
  from alignment and rendered normally.
- A blank line or a comment line breaks the current group; alignment restarts after it.
- Arguments under different section keywords are never aligned across keyword boundaries.
- Additionally, keyword-as-first-token lines are aligned by their keyword column regardless of the total token count on each line. This is separate from the same-token-count alignment of value lines.

> **Known limitation:** The heuristic matches on token count alone and does not verify
> structural similarity. Two consecutive lines with the same number of tokens but
> semantically different structure (e.g., a path and a flag) will be aligned. In practice,
> this rarely produces undesirable results because CMake argument lists are typically
> homogeneous within a keyword section.

**Example 1 — `install(TARGETS ...)`:**

```cmake
# alignArgGroups = true
install(TARGETS
  MyLib       RUNTIME DESTINATION bin
  MyOtherLib  RUNTIME DESTINATION lib
  MyPlugin    LIBRARY DESTINATION plugins
)
```

**Example 2 — `add_custom_command` with multiple similar keyword groups:**

```cmake
# alignArgGroups = true
add_custom_command(
  OUTPUT    ${CMAKE_CURRENT_BINARY_DIR}/generated.cpp
  COMMAND   generator --input schema.json --output generated.cpp
  DEPENDS   schema.json
  COMMENT   "Generating code"
  VERBATIM
)
```

In this example, the keyword-as-first-token lines (`OUTPUT`, `COMMAND`, `DEPENDS`, `COMMENT`) are aligned by their keyword column, regardless of each line's total token count. `VERBATIM` stands alone (no value) and does not participate in alignment.

**Example 3 — `set()` with tabular data:**

```cmake
# alignArgGroups = true
set(MY_TABLE
  "name"    "type"    "default"
  "width"   "int"     "80"
  "style"   "string"  "cascade"
)
```

**Example 4 — `target_sources` with `FILE_SET`:**

```cmake
# alignArgGroups = true
target_sources(MyLib
  FILE_SET HEADERS
    BASE_DIRS include
    FILES
      include/mylib/core.h
      include/mylib/utils.h
  FILE_SET CXX_MODULES
    BASE_DIRS src
    FILES
      src/core.cppm
      src/utils.cppm
)
```

---

## 10 · Generator Expressions

### 10.1 `genexWrap`

|             |                        |
| ----------- | ---------------------- |
| **Type**    | `"cascade" \| "never"` |
| **Default** | `"cascade"`            |

Controls whether generator expressions (`$<...>`) are eligible for multi-line formatting.

- **`"cascade"`** (default): Generator expressions follow the same cascading wrapping
  algorithm as command arguments. Each nesting level adds `genexIndentWidth` (or
  `indentWidth`) spaces. See "Generator Expression Argument Boundaries" below for
  how the internal structure of a genex maps to the cascade algorithm.

- **`"never"`**: Generator expressions are always kept on a single line, regardless
  of length. This applies recursively to all nesting levels —
  `$<$<CONFIG:Debug>:$<TARGET_FILE:foo>>` remains a single line regardless of depth.
  This can cause line-width violations for deeply nested genexes.

```cmake
# genexWrap = "never" — generator expression stays on one line
target_compile_definitions(MyLib
  PRIVATE
    $<$<CONFIG:Debug>:DEBUG_MODE=1;VERBOSE_LOG=1>
)
```

#### Generator Expression Argument Boundaries

Generator expressions use different delimiters than command invocations. The following rules define how genex internals map to the cascade algorithm:

- **Colon (`:`)** separates the condition/name part from the value part. In `$<condition:value>`, the colon acts as the primary split point. If the value part causes overflow, wrapping occurs after the colon — everything after the colon moves to the next line. For `$<IF:cond,a,b>`, all three comma-separated parameters are on the value side of the colon and wrap together.
- **Semicolons (`;`)** separate list items and are treated as argument separators for wrapping purposes. In `$<$<CONFIG:Debug>:DEBUG_MODE=1;VERBOSE_LOG=1>`, the semicolons delimit `DEBUG_MODE=1` and `VERBOSE_LOG=1` as separate arguments that can each occupy their own line.
- **Commas (`,`)** separate positional parameters in genexes like `$<IF:condition,true_value,false_value>`. Each comma-separated parameter is an independent argument for wrapping.

When `genexWrap = "cascade"`, the cascade proceeds as:

1. **Single line** — if the entire genex fits within the remaining line width, emit inline.
2. **Split at primary delimiter** — place the value part on a new line after the colon, indented by `genexIndentWidth` relative to the column where `$<` starts. List items (semicolons) and positional params (commas) pack onto the value line.
3. **One per line** — if packing still overflows, each list item / positional param gets its own line.

Nested genexes recurse: `$<` opens a new bracket scope and `>` closes it, analogous to `(`/`)` in commands.

```cmake
# genexWrap = "cascade" — single line (fits)
target_compile_definitions(MyLib PRIVATE $<$<CONFIG:Debug>:DEBUG_MODE=1>)

# genexWrap = "cascade" — split at colon, semicolons as separate args
target_compile_definitions(MyLib
  PRIVATE
    $<$<CONFIG:Debug>:
      DEBUG_MODE=1
      VERBOSE_LOG=1
    >
)

# genexWrap = "cascade" — $<IF:...> with comma-separated params
target_link_libraries(MyLib
  PRIVATE
    $<IF:
      $<CONFIG:Debug>,
      debug_lib,
      release_lib
    >
)
```

### 10.2 `genexClosingAngleNewline`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

Analogous to `closingParenNewline` but for the closing `>` of generator expressions.
This setting applies recursively to every closing `>` at every nesting level of generator expressions.

- `true` (default): Closing `>` on its own line, aligned with the `$<` opener.
- `false`: Closing `>` stays on the last content line.

```cmake
# genexClosingAngleNewline = true (default)
target_compile_definitions(MyLib
  PRIVATE
    $<$<CONFIG:Debug>:
      DEBUG_MODE=1
      VERBOSE_LOG=1
    >
)

# genexClosingAngleNewline = false
target_compile_definitions(MyLib
  PRIVATE
    $<$<CONFIG:Debug>:
      DEBUG_MODE=1
      VERBOSE_LOG=1>
)
```

---

## 11 · Command-Specific Overrides

### 11.1 `perCommandConfig`

|             |                                    |
| ----------- | ---------------------------------- |
| **Type**    | `table { [commandName]: { ... } }` |
| **Default** | `{}`                               |

Allows overriding a subset of formatting options on a per-command basis. The command name
is matched case-insensitively. Any option from groups 1 (wrapping), 2 (indentation),
4 (casing), 5 (parentheses & spacing), 6 (comments), 9 (alignment), 10 (generator
expressions), and 12 (sorting) may be overridden. Per-command `lineWidth` replaces the
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
`firstArgSameLine`, `wrapArgThreshold`, `magicTrailingNewline`, `indentWidth`,
`indentStyle`, `continuationIndentWidth`, `genexIndentWidth`, `commandCase`, `keywordCase`,
`customKeywords`, `literalCase`, `closingParenNewline`, `spaceBeforeParen`,
`spaceInsideParen`, `commentPreservation`, `commentWidth`,
`alignTrailingComments`, `commentGap`, `alignPropertyValues`, `alignConsecutiveSet`,
`alignArgGroups`, `genexWrap`, `genexClosingAngleNewline`, `sortArguments`, and
`sortKeywordSections`.

---

## 12 · Sorting

### 12.1 `sortArguments`

|             |                       |
| ----------- | --------------------- |
| **Type**    | `boolean \| string[]` |
| **Default** | `false`               |

When enabled, alphabetically sort arguments within specific keyword sections. This is
primarily useful for dependency lists and source-file lists, where a canonical order
reduces merge conflicts.

- `false` (default): No sorting.
- `true`: Sort arguments in all recognized section keyword groups. A keyword section is
  sortable if its keyword appears in the formatter's keyword dictionary (§4.2) and its
  arguments are simple values (not sub-keyword structures). For example, this covers
  keywords such as SOURCES, PRIVATE, PUBLIC, INTERFACE, FILES, DEPENDS, COMPONENTS,
  TARGETS, CONFIGURATIONS, and any keywords added via `customKeywords` (§4.3). The
  general rule (keyword in dictionary + simple values) is normative; the list above is illustrative.
- `["SOURCES", "FILES"]`: Only sort arguments under the listed keyword sections.

Sorting is case-insensitive. Arguments that compare equal after case-folding retain their
original relative order (stable sort).

Duplicate arguments are preserved. The stable sort retains duplicates in their original relative order.

A comment is considered "attached" to an argument if it immediately precedes the argument
with no blank line between them, or if it is a trailing comment on the same line as the
argument. Attached comments travel with the argument during sorting.

Unattached comments (comments separated from the following argument by one or more blank lines) act as group boundaries — sorting only occurs within sub-groups between unattached comments. The unattached comments and blank lines remain in place.

Arguments containing generator expressions (`$<...>`) and variable references (`${...}`)
are sorted by their literal text representation (the unexpanded source text). The formatter
does not evaluate or expand variables before sorting.

Multi-line arguments (e.g., bracket arguments or arguments containing embedded newlines) are compared by their original input text as-is, with internal whitespace preserved.

```cmake
# Input:
target_sources(MyApp
  PRIVATE
    zebra.cpp
    alpha.cpp
    middle.cpp
)

# Output (sortArguments = true):
target_sources(MyApp
  PRIVATE
    alpha.cpp
    middle.cpp
    zebra.cpp
)
```

Commands without recognized keyword sections are not affected by `sortArguments`. For example, `set(VAR c a b)` retains its original argument order regardless of this setting.

### 12.2 `sortKeywordSections`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, reorder keyword sections within a command to a canonical order. For
`target_link_libraries` and similar commands, the canonical order is
`PUBLIC` → `INTERFACE` → `PRIVATE`.

This is an opinionated option and off by default.

The canonical section order is defined per-command in the formatter's keyword dictionary.
All commands with PUBLIC/INTERFACE/PRIVATE sections use the same canonical order:
PUBLIC → INTERFACE → PRIVATE, unless a different order is explicitly listed in Appendix F.
For commands not in the dictionary, no reordering is performed. See Appendix F for the
full per-command canonical order.

Positional arguments preceding the first keyword section remain in place and are not subject to reordering.

Comments attached to a keyword section (comments immediately preceding the keyword with no blank line between them, or trailing comments on the keyword line) travel with the section during reordering, mirroring the attached-comment rule in §12.1.

```cmake
# Input:
target_link_libraries(MyTarget
  PRIVATE
    internal_lib
  PUBLIC
    Boost::filesystem
)

# Output (sortKeywordSections = true): PUBLIC before PRIVATE (canonical order)
target_link_libraries(MyTarget
  PUBLIC
    Boost::filesystem
  PRIVATE
    internal_lib
)
```

---

## 13 · Inline Pragmas

### 13.1 Pragma Syntax

The formatter recognizes inline comment directives (pragmas) that control formatting
behavior locally within a file. All pragmas use the `cmakefmt:` prefix.

```
pragma     := "#" ws? "cmakefmt:" ws? action
action     := "off" | "on" | "skip"
            | "push" ws toml-inline-table
            | "pop"

ws         := [ \t]+
```

Where `toml-inline-table` follows [TOML inline table syntax](https://toml.io/en/v1.1.0#inline-table) with one relaxation: trailing commas are permitted. Values inside the table are TOML scalars (integer, boolean, double-quoted string) or TOML inline arrays.

**Rules:**

- One pragma per line. No code on the same line.
- The prefix `cmakefmt:` is case-sensitive, lowercase.
- Whitespace between `#` and `cmakefmt:` is optional: `#cmakefmt: off` and `# cmakefmt: off` are both valid.
- `push` takes an inline TOML table: `push { lineWidth = 120 }`. Braces are always required; bare `push` without braces is forbidden. `push {}` creates an empty save-point. Values use TOML scalar syntax: `80` (integer), `true`/`false` (boolean), `"lower"` (string). Arrays use TOML inline-array syntax (e.g., `push { spaceBeforeParen = ["if", "elseif"] }`). TOML inline tables are supported for table-typed options (e.g., `push { perCommandConfig = { set = { wrapStyle = "vertical" } } }`).
- Trailing content after a valid pragma is a warning and is ignored.

### 13.2 `off` / `on`

Disable formatting for a region. Every byte between `# cmakefmt: off` and `# cmakefmt: on`
is emitted exactly as read.

```cmake
# cmakefmt: off
set(MY_CAREFULLY_ALIGNED_MATRIX
  1 0 0 0
  0 1 0 0
  0 0 1 0
  0 0 0 1
)
# cmakefmt: on
```

- The off-region is **opaque and byte-preserved**: the formatter does not parse or execute pragmas inside it. The first `# cmakefmt: on` encountered ends the region.
- `off` without a matching `on` before EOF suppresses formatting for the rest of the file. This is valid, not a warning.
- `on` without a preceding `off` is a warning and is ignored.
- Formatting resumes immediately after `# cmakefmt: on`. Outside off-regions, all active formatting and normalization options apply.
- `off`/`on` does not interact with the `push`/`pop` stack. The configuration state is unchanged across an off-region:

```cmake
# cmakefmt: push { lineWidth = 120 }
# cmakefmt: off
...verbatim content...
# cmakefmt: on
# lineWidth is still 120
# cmakefmt: pop
```

### 13.3 `skip`

Suppress formatting for the **next command invocation** only. The command is emitted verbatim.

```cmake
# cmakefmt: skip
ExternalProject_Add(googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG        release-1.12.1
)
```

- Blank lines and comments between `skip` and the target command are allowed and preserved.
- `skip` before EOF with no subsequent command is a warning.
- `skip` does not accept inline overrides or interact with the `push`/`pop` stack.
- If `# cmakefmt: off` appears between a `skip` pragma and the next command, the off-region starts immediately and the pending `skip` is consumed (cancelled). After `# cmakefmt: on`, the `skip` does not persist.
- `push`/`pop` pragmas between `skip` and the target command are processed normally. `skip` only suppresses formatting of the next command invocation; it does not create an inert zone for other pragmas.

### 13.4 `push` / `pop`

#### 13.4.1 `push`

Create a new configuration frame on the stack. The frame inherits all values from the
current top, then applies any inline overrides.

```cmake
# cmakefmt: push { lineWidth = 120, alignPropertyValues = true }
set_target_properties(MyTarget PROPERTIES
  CXX_STANDARD              17
  CXX_STANDARD_REQUIRED     ON
  POSITION_INDEPENDENT_CODE ON
)
# cmakefmt: pop
```

`push {}` (empty table) creates a save-point with no changes.

When `push` includes `perCommandConfig`, the pushed table shallow-merges with the current effective `perCommandConfig` at the command-key level (same semantics as `extends` §15.2): top-level command keys in the pushed table override the existing entry for that command entirely. Command entries not present in the pushed table are preserved from the current frame. Step 2 of the resolution order (§13.4.4) consults this effective merged `perCommandConfig`.

#### 13.4.2 `pop`

Discard the top frame, restoring the configuration beneath it. `pop` when the stack has
only the root frame is a warning and is ignored.

#### 13.4.3 Nesting

Frames nest arbitrarily. Each `pop` discards exactly one frame:

```cmake
# cmakefmt: push { lineWidth = 120 }         ← frame 1
  # cmakefmt: push { indentWidth = 4 }       ← frame 2
    # lineWidth = 120, indentWidth = 4
  # cmakefmt: pop                          ← discard frame 2
  # lineWidth = 120, indentWidth restored
# cmakefmt: pop                            ← discard frame 1
# All values restored to config-file state
```

#### 13.4.4 Resolution Order

When resolving an option's effective value for a given command:

1. **Push stack** — walk the stack from top to bottom. The first frame that explicitly sets the option wins.
2. **`perCommandConfig`** — if no frame in the stack sets the option, check the command-specific override table.
3. **Config file value** — the value from `.cmakefmt.toml`.
4. **Built-in default** — the hardcoded fallback.

A frame "explicitly sets" an option only if the option's key appeared in that frame's `push` pragma inline table. Options not mentioned in the pragma are transparent — the frame does not override them.

A `push` override always takes priority over `perCommandConfig`.

### 13.5 Pragma-Settable Options

The options settable via `push` include all formatting options listed in the Summary Table. Only `disableFormatting`,
`extends`, `$schema`, and `ignorePatterns` cannot be set in a pragma — these control configuration
infrastructure or file-level routing, not formatting behavior. Setting any of these produces a warning and is
ignored. Note that `push` has broader scope than `perCommandConfig` (§11.1): it can also
set file-level options such as `maxBlankLines`, `lineEnding`, `finalNewline`,
`trimTrailingWhitespace`, `collapseSpaces`, `endCommandArgs`, and `indentBlockBody`.
Changing `indentBlockBody` via `push` affects only blocks opened after the push, not the currently enclosing block.
`ignoreCommands` is settable via `push`, enabling local command suppression within a file
region. These file-level and flow-control options are excluded from `perCommandConfig`
because they apply to document structure or block boundaries, not to individual command
invocations.

### 13.6 Diagnostics

All pragma diagnostics are **warnings** (never errors) and include file path and line
number. The formatter never fails due to a malformed pragma.

| Condition                            | Behavior                                |
| ------------------------------------ | --------------------------------------- |
| Malformed syntax                     | Entire pragma ignored                   |
| Unknown option name                  | That assignment skipped, others applied |
| Type mismatch or out-of-range value  | That assignment skipped, others applied |
| Non-settable option                  | That assignment skipped, others applied |
| `on` without preceding `off`         | Ignored                                 |
| `pop` without matching `push`        | Ignored                                 |
| `skip` at EOF (no following command) | Ignored                                 |
| Unmatched `push` at EOF              | Implicitly popped                       |
| Duplicate key in one pragma          | Last value wins (warning)               |

---

## 14 · Conditional & Flow Control Formatting

### 14.1 `indentBlockBody`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, the body of flow-control blocks (`if`/`elseif`/`else`/`endif`,
`foreach`/`endforeach`, `while`/`endwhile`, `function`/`endfunction`,
`macro`/`endmacro`, `block`/`endblock`) is indented by `indentWidth`.

When `false`, no additional indentation is applied to block bodies. This produces
a flat style sometimes seen in older CMake codebases.

```cmake
# indentBlockBody = true (default)
if(ENABLE_TESTS)
  add_subdirectory(tests)
endif()

# indentBlockBody = false
if(ENABLE_TESTS)
add_subdirectory(tests)
endif()
```

### 14.2 `endCommandArgs`

|             |                                     |
| ----------- | ----------------------------------- |
| **Type**    | `"remove" \| "preserve" \| "match"` |
| **Default** | `"remove"`                          |

Controls the arguments inside block-closing commands (`endif()`, `endfunction()`,
`endforeach()`, `endmacro()`, `endwhile()`, `endblock()`) and intermediate flow-control
commands (`else()`, `elseif()`). Older CMake required repeating the
condition/name; modern CMake does not.

- **`"remove"`** (default): Strip arguments from closing commands and `else()`.
  `endif(condition)` → `endif()`; `else(condition)` → `else()`.
  `elseif()` is unaffected by `"remove"` — its condition is its own, not a repetition.
- **`"preserve"`**: Keep whatever arguments are present.
- **`"match"`**: For closing commands, copies the full argument list verbatim from the
  corresponding opening command. `endwhile(x AND y)` matches `while(x AND y)`;
  `endfunction(my_func)` matches `function(my_func)`. Complex conditions with
  AND/OR/parentheses are copied as-is. If the closing command has no argument, one is
  added; if it has a mismatched argument, it is corrected.
  For `else()`, `"match"` copies the `if()`-condition from the enclosing `if` block.
  `elseif()` is unaffected by `"match"` — it has its own condition expression.

For `block()`/`endblock()`: since `block()` has no positional name argument (only keyword clauses like `SCOPE_FOR` and `PROPAGATE`), `endCommandArgs = "match"` produces `endblock()` unconditionally.

For `else()`: under `"remove"`, legacy arguments are stripped (`else(condition)` → `else()`). Under `"match"`, the condition from the enclosing `if()` is copied into `else()`. Under `"preserve"`, existing arguments are kept as-is. `elseif()` always retains its own condition expression — `"remove"` and `"match"` do not alter it because its condition is definitional, not a repetition of the opening command.

```cmake
# endCommandArgs = "match" — else() copies enclosing if() condition
# Input:
if(WIN32)
  message("Windows")
else()
  message("Other")
endif()

# Output:
if(WIN32)
  message("Windows")
else(WIN32)
  message("Other")
endif(WIN32)
```

Block matching follows CMake's syntactic nesting — each closing command is paired with the nearest unmatched opening command of the corresponding type.

The copied arguments are subject to normal wrapping rules — if the closing command with
its matched arguments exceeds `lineWidth`, it wraps like any other command invocation.

Nested parentheses in conditions (e.g., `if((A AND B) OR C)`) are treated as grouped
sub-expressions. When wrapping occurs, the parenthesized group is indented as a unit.

```cmake
# endCommandArgs = "match"
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
  message(STATUS "Debug mode")
endif(CMAKE_BUILD_TYPE STREQUAL "Debug")

# endCommandArgs = "remove" (default)
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
  message(STATUS "Debug mode")
endif()
```

### 14.3 Empty Commands

Commands with no arguments (`endif()`, `else()`, `return()`, `endforeach()`, etc.) are always
formatted on a single line. Wrapping and indentation options do not apply to empty argument lists.

---
## 15 · Configuration Meta

### 15.1 `$schema`

|             |          |
| ----------- | -------- |
| **Type**    | `string` |
| **Default** | *(none)* |

Optional JSON Schema URL for editor validation and autocomplete. Has no effect on the
formatter itself.

```toml
"$schema" = "https://raw.githubusercontent.com/yourorg/cmakefmt/main/schema.json"
```

### 15.2 `extends`

|             |          |
| ----------- | -------- |
| **Type**    | `string` |
| **Default** | *(none)* |

Path to another `.cmakefmt.toml` file to use as a base. Options from the current file
override the base. Allows sharing a common config across a monorepo while permitting
per-directory tweaks. Direct or transitive circular references in the `extends` chain are detected and produce an error.
Implementations should impose a reasonable maximum depth for `extends` chains (e.g., 32 levels) to guard against pathological configurations. Exceeding this limit produces an error.

The path is resolved relative to the directory containing the config file that declares it.
Absolute paths are used as-is.

**Merge strategy:** Scalars in the child override the base. Any option with an array value in the child
replaces the base array entirely (this applies to all array-typed option values, including `customKeywords`,
`ignorePatterns`, `ignoreCommands`, `sortArguments` when array-valued, and `spaceBeforeParen` when
array-valued). Tables (`perCommandConfig`) are shallow-merged: top-level command keys in the child override the
base entry for that command *entirely* (not field-by-field). Base command entries not present
in the child are preserved.

```toml
extends = "../../.cmakefmt.toml"

[perCommandConfig.set]
wrapStyle = "vertical"
```

### 15.3 Unknown Keys

|              |         |
| ------------ | ------- |
| **Behavior** | Warning |

Unknown keys in `.cmakefmt.toml` produce a diagnostic warning (including the key name
and file path) and are ignored. This allows forward-compatibility: a config file written
for a newer formatter version can be used with an older version without errors.

Unknown keys inside `perCommandConfig` tables follow the same policy.

---

## 16 · Suppression & Ignore Options

### 16.1 `disableFormatting`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, formatting is fully disabled and output **MUST** be byte-for-byte identical to the input.
In this mode, the formatter applies no transformations or normalizations, including `lineEnding`, `finalNewline`, `trimTrailingWhitespace`, `collapseSpaces`, `maxBlankLines`, and UTF-8 BOM stripping.
`disableFormatting = true` takes precedence over all other options and pragmas.

```cmake
# Input (irregular formatting):
SET(  FOO   "bar"  )

# Output (disableFormatting = true): byte-for-byte identical to input
SET(  FOO   "bar"  )
```

### 16.2 `ignorePatterns`

|             |            |
| ----------- | ---------- |
| **Type**    | `string[]` |
| **Default** | `[]`       |

Glob patterns for files that should be skipped entirely. Patterns are resolved
relative to the configuration file's directory. When patterns are inherited via
`extends` (§15.2), each pattern is resolved relative to the config file in which
it appears, not relative to the extending file.

Patterns use gitignore-style glob syntax: `*` matches any sequence except `/`, `**` matches
any sequence including `/`, `?` matches a single character, `[...]` matches character classes.

```toml
ignorePatterns = [
  "third_party/**",
  "generated/*.cmake",
  "build/**"
]
```

### 16.3 `ignoreCommands`

|             |            |
| ----------- | ---------- |
| **Type**    | `string[]` |
| **Default** | `[]`       |

A list of command names whose invocations should be skipped entirely during formatting
(preserved verbatim). Case-insensitive.

```toml
ignoreCommands = ["ExternalProject_Add", "FetchContent_Declare"]
```

Useful for complex macro invocations or commands with DSL-like syntax where the
formatter's heuristics may produce undesirable results.

```cmake
# ignoreCommands = ["ExternalProject_Add"]

# Input (irregular formatting):
ExternalProject_Add(googletest
    GIT_REPOSITORY  https://github.com/google/googletest.git
    GIT_TAG         release-1.12.1
)

# Output: preserved verbatim, no formatting applied
ExternalProject_Add(googletest
    GIT_REPOSITORY  https://github.com/google/googletest.git
    GIT_TAG         release-1.12.1
)
```

---

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

---

## Appendix A — Default Configuration

The following `.cmakefmt.toml` shows all options at their default values:

```toml
lineWidth = 80
wrapStyle = "cascade"
firstArgSameLine = true
wrapArgThreshold = 0
magicTrailingNewline = true
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

---

## Appendix B — Example Config File

A typical opinionated project configuration:

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

---

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

---

## Appendix D — CLI Reference

These are CLI-only flags and are not configuration file options. They control the
formatter's runtime behavior, not the formatting output itself.

### Exit Codes

| Code | Meaning                                                         |
| ---- | --------------------------------------------------------------- |
| `0`  | Success — all files formatted (or already formatted).           |
| `1`  | Formatting changes found (`--check` mode) — files need changes. |
| `2`  | Error — parse failure, config error, or I/O error.              |

### `--check`

Exit with a non-zero status code if any file would be changed, without writing changes.
Useful for CI enforcement.

### `--diff`

Print a unified diff of formatting changes to stdout instead of the formatted output. No files are modified.

### `--stdin`

Read CMake source from stdin and write formatted output to stdout. Used for editor
integration (pipe through formatter). When `--stdin` is passed alongside file arguments,
it is an error — the formatter exits with code 2 and a diagnostic message.

### `--write` / `--inplace`

These flags are aliases for the same behavior: write formatted output back to the input
file(s) in-place. Without this flag, formatted output is written to stdout.

### `--config <path>`

Explicit path to a `.cmakefmt.toml` configuration file. Overrides the normal config
discovery walk.

### `--assume-filename <path>`

Pretend that stdin input comes from this file path. Used for config discovery (walk
upward from this file's directory) and file-type detection. Only meaningful with `--stdin`.

### `--color` / `--no-color`

Control colored output in diff and diagnostic output. When `--color` is set, ANSI color
codes are always emitted. When `--no-color` is set, color output is suppressed. Without
either flag, color is auto-detected based on whether stdout is a terminal.

### `--verbose`

Increase diagnostic output. Useful for debugging configuration resolution and
formatting decisions.

### `--quiet`

Suppress non-error output.

### `--print-config`

Print the resolved configuration (after merging defaults, config file, and CLI overrides)
as TOML to stdout. Useful for debugging.

### Flag Interactions

| Combination                           | Behavior                                                                                                                                             |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--check` + `--diff`                  | Both can be used together. `--diff` prints the unified diff to stdout; `--check` controls the exit code. No changes needed → exit 0, no diff output. |
| `--check` + `--write`                 | Mutually exclusive. The formatter exits with code 2 and a diagnostic message.                                                                        |
| `--diff` + `--write`                  | Mutually exclusive. The formatter exits with code 2 and a diagnostic message.                                                                        |
| `--stdin` + file arguments            | Error, exit code 2 (see `--stdin` above).                                                                                                            |
| `--quiet` + `--verbose`               | `--quiet` wins; non-error output is suppressed.                                                                                                      |
| `--assume-filename` without `--stdin` | Warning, flag is ignored.                                                                                                                            |
---

## Appendix E — Option Interaction Rules

This appendix documents interactions between options where the combined behavior is not
obvious from reading each option's description in isolation.

| Options                                               | Interaction                                                                                                                                                                                                                                                                                                        |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `sortArguments` + `alignArgGroups`                    | Sorting is applied first, then alignment. Arguments are reordered within their keyword section, and the resulting layout is then column-aligned if `alignArgGroups` is enabled.                                                                                                                                    |
| `indentStyle = "tab"` + alignment options             | When `indentStyle = "tab"`, alignment padding (the spaces after the leading tab indentation that align columns) always uses space characters, never tabs. Leading indentation uses tabs; alignment columns use spaces.                                                                                             |
| `disableFormatting = true` + all other options        | `disableFormatting` takes absolute precedence. Output is byte-for-byte identical to input. No other option has any effect.                                                                                                                                                                                         |
| `wrapArgThreshold` + `magicTrailingNewline`           | Both are Step 0 pre-checks (Appendix C). `wrapArgThreshold` skips directly to Step 3 (one-per-line). `magicTrailingNewline` skips Step 1 only — Step 2 (keyword breaks) is still attempted for keyword-bearing commands before escalating to Step 3. When both trigger, `wrapArgThreshold` wins (Step 3 directly). |
| `wrapStyle = "vertical"` + `wrapArgThreshold`         | `wrapArgThreshold` forces expansion regardless of `wrapStyle`. Under `"vertical"`, the result is always one-per-line (Step 3). Under `"cascade"`, `wrapArgThreshold` skips directly to Step 3. In practice, the outcome is identical.                                                                              |
| `genexWrap = "never"` + `genexClosingAngleNewline`    | `genexClosingAngleNewline` has no effect when `genexWrap = "never"` because generator expressions are always single-line.                                                                                                                                                                                          |
| `genexWrap = "never"` + `genexIndentWidth`            | `genexIndentWidth` has no effect when `genexWrap = "never"` because generator expressions are never expanded to multiple lines.                                                                                                                                                                                    |
| `commentPreservation = "preserve"` + `commentWidth`   | `commentWidth` has no effect unless `commentPreservation = "reflow"`.                                                                                                                                                                                                                                              |
| `firstArgSameLine` + single-line commands             | `firstArgSameLine` only affects commands that wrap. Single-line commands always have the first argument on the same line.                                                                                                                                                                                          |
| `closingParenNewline` + single-line commands          | `closingParenNewline` only affects commands that wrap. Single-line commands always have `)` on the same line.                                                                                                                                                                                                      |
| `blankLineBetweenSections` + single-section commands  | No blank lines inserted when a command has only one section.                                                                                                                                                                                                                                                       |
| `alignPropertyValues` + single-line property commands | `alignPropertyValues` only takes effect when properties are rendered one-per-line.                                                                                                                                                                                                                                 |
| `perCommandConfig` + `push` pragma                    | Per §13.4.4: `push` stack overrides always take priority over `perCommandConfig`.                                                                                                                                                                                                                                  |
| `ignoreCommands` + `perCommandConfig`                 | If a command is in `ignoreCommands`, it is preserved verbatim. `perCommandConfig` entries for that command are not applied.                                                                                                                                                                                        |
| `ignoreCommands` + pragmas (`off`/`skip`)             | Both suppress formatting. `ignoreCommands` applies globally to all invocations; pragmas apply to specific locations. They do not conflict — either one is sufficient to suppress formatting.                                                                                                                       |
| `finalNewline` + `maxBlankLines`                      | `maxBlankLines` enforces its limit on consecutive blank lines at EOF regardless of `finalNewline`. `finalNewline = false` only controls whether a missing trailing newline is added.                                                                                                                               |
| `ignoreCommands` + sorting/alignment                  | Ignored commands are preserved verbatim; `sortArguments`, `sortKeywordSections`, and alignment options do not apply to them.                                                                                                                                                                                       |
| `blankLineBetweenSections` + `maxBlankLines`          | When `blankLineBetweenSections = true`, the inserted blank lines take precedence over `maxBlankLines` within argument lists, analogous to `minBlankLinesBetweenBlocks` (§3.2).                                                                                                                                     |
| `alignTrailingComments` + `commentGap`                | When `alignTrailingComments = true`, the alignment column uses `commentGap` as the minimum gap between the longest code segment and the `#` marker.                                                                                                                                                                |
| `collapseSpaces` + alignment options                  | `collapseSpaces` applies during input normalization (before layout). Alignment-generated padding (`alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`, `alignTrailingComments`) is inserted after collapsing and is exempt.                                                                             |
| `alignArgGroups` + `blankLineBetweenSections`         | When both are enabled, `blankLineBetweenSections` inserts blank lines between sections first, then `alignArgGroups` detects column patterns within each section independently. The blank line acts as an alignment group boundary.                                                                                 |
| `maxBlankLines` + `sortArguments`                     | Blank lines inside argument lists that serve as sorting group boundaries (§12.1) are preserved when `sortArguments` is enabled, overriding the normal §3.1 rule that discards blank lines inside argument lists.                                                                                                   |
| `sortKeywordSections` + `blankLineBetweenSections`    | When both are enabled, section reordering (`sortKeywordSections`) is applied first, then blank-line insertion (`blankLineBetweenSections`).                                                                                                                                                                        |

---

## Appendix F — Keyword Dictionary

> **Note:** The keyword dictionary in `src/generation/signatures.rs` is the authoritative reference.
> This appendix provides a condensed overview of recognized commands and canonical section orders
> used by `sortKeywordSections` (§12.2). Full keyword tables per command are intentionally omitted
> to avoid drift from the source of truth.

### Condition-syntax commands

`if`, `elseif`, `else`, `endif`, `while`, `endwhile` — parsed as condition expressions, not
keyword-structured commands.

### Commands with canonical section orders

| Command                 | Canonical Section Order                                                                                                               |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `target_link_libraries` | `PUBLIC` → `INTERFACE` → `PRIVATE` → `LINK_PUBLIC` → `LINK_PRIVATE` → `LINK_INTERFACE_LIBRARIES`                                      |
| `target_sources`        | `PUBLIC` → `INTERFACE` → `PRIVATE`                                                                                                    |
| `install`               | `ARCHIVE` → `LIBRARY` → `RUNTIME` → `OBJECTS` → `FRAMEWORK` → `BUNDLE` → `PUBLIC_HEADER` → `PRIVATE_HEADER` → `RESOURCE` → `FILE_SET` |
| `export`                | `PACKAGE_DEPENDENCY` → `TARGET` → `VERSION`                                                                                           |

### Other recognized commands

The following commands are in the keyword dictionary with their own keyword/section definitions:

**Target commands:** `target_compile_definitions`, `target_compile_options`, `target_compile_features`,
`target_link_options`, `target_include_directories`, `add_executable`, `add_library`,
`set_target_properties`, `set_source_files_properties`, `set_tests_properties`, `set_directory_properties`

**Project & package:** `project`, `find_package`, `cmake_minimum_required`, `cmake_pkg_config`

**Custom commands:** `add_custom_command`, `add_custom_target`, `execute_process`

**Variables & properties:** `set`, `unset`, `option`, `return`, `mark_as_advanced`,
`define_property`, `get_property`, `set_property`, `get_directory_property`,
`get_filename_component`, `set_package_properties`

**Control flow:** `foreach`, `block`

**String/list/file:** `string`, `list`, `file`, `cmake_path`, `cmake_language`,
`cmake_host_system_information`, `math`, `separate_arguments`

**Build & test:** `add_test`, `gtest_discover_tests`, `build_command`, `try_compile`, `try_run`,
`message`, `source_group`, `configure_file`, `include`, `add_subdirectory`,
`enable_language`, `load_cache`, `create_test_sourcelist`,
`include_external_msproject`, and the `ctest_*` family.

**Find modules:** `find_library`, `find_file`, `find_path`, `find_program`,
`cmake_parse_arguments`

### Simple commands (no keywords)

`add_compile_definitions`, `add_compile_options`, `add_definitions`, `add_dependencies`,
`add_link_options`, `aux_source_directory`, `enable_testing`, `fltk_wrap_ui`,
`get_source_file_property`, `get_target_property`, `get_test_property`,
`include_regular_expression`, `remove_definitions`

### Block closers

`endforeach`, `endfunction`, `endmacro`, `endblock` — accept 0 or 1 positional argument.