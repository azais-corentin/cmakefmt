# CMake Formatter — Configuration & Formatting Rules Specification

This document defines every formatting behavior and its associated configuration option for the CMake formatter.
Options are organized into logical groups. Each option includes its type, default value, a description of its
effect, and concrete before/after examples where helpful.

The design draws inspiration from the cascading-strategy formatters of the Rust (rustfmt), Python (ruff),
and JavaScript (oxfmt) ecosystems: a small set of opinionated defaults that cover the common case,
with enough knobs to satisfy teams with strong existing conventions.

Configuration is read from a TOML file named `.cmakefmt.toml` (or `cmakefmt.toml`) discovered by walking
from the formatted file's directory upward to the filesystem root. The first file found wins. All keys use
`camelCase`.

---

## 1 · Line Width & Wrapping

### 1.1 `lineWidth`

|             |            |
| ----------- | ---------- |
| **Type**    | `integer`  |
| **Default** | `80`       |
| **Range**   | `40 – 320` |

Maximum number of columns per line. The formatter uses a **cascading wrapping strategy**
controlled by `wrapStyle` (§1.3) to decide how to break a command invocation that exceeds
this width. Comments are also reflowed to respect this limit unless they are inside a
`# cmake-format: off` region.

Inspired by rustfmt's `max_width` and ruff's `line-length`. The default of 80 matches
the most common convention across build-system files and terminal widths.

### 1.2 `forceOneArgPerLine`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, skip step 2 of the cascading algorithm (packing multiple arguments per line)
and go directly to step 3 (one argument per line) whenever a command does not fit on a
single line. Produces a strictly vertical style.

### 1.3 `wrapStyle`

|             |                                           |
| ----------- | ----------------------------------------- |
| **Type**    | `"cascade" \| "vertical" \| "keepBreaks"` |
| **Default** | `"cascade"`                               |

Controls the overall line-wrapping philosophy. This is the master switch for wrapping behavior;
`forceOneArgPerLine` (§1.2) is a convenience alias that sets `wrapStyle = "vertical"`.

- **`"cascade"`** (default) — Three-step strategy:
  1. *Fit on single line* if total width ≤ `lineWidth`.
  2. *Keywords on new lines, arguments inline* — each keyword starts a new line, and its
     arguments are packed onto that line as space allows.
  3. *One argument per line* — if step 2 still overflows, every argument gets its own line.

  This applies recursively to all nesting levels, including generator expressions.

- **`"vertical"`** — Equivalent to step 1 → step 3 directly (skip step 2). This is the
  same as setting `forceOneArgPerLine = true`.

- **`"keepBreaks"`** — Preserve the author's existing line breaks: if a line break exists
  in the input between two arguments, keep it; if arguments are on the same line and they
  fit within `lineWidth`, keep them together. Never add new line breaks unless a single
  argument token itself exceeds `lineWidth`. Useful for projects migrating gradually.

  Inspired by rustfmt's `"Preserve"` indent style.

### 1.4 `firstArgSameLine`

|             |                       |
| ----------- | --------------------- |
| **Type**    | `boolean \| string[]` |
| **Default** | `true`                |

Controls whether the first positional argument (typically the target name) stays on the
same line as the command name.

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

- `["set", "option", ...]`: A list of command names for which the first argument stays on
  the opening line; all other commands move it to the next line. Case-insensitive.

### 1.5 `wrapArgThreshold`

|             |                |
| ----------- | -------------- |
| **Type**    | `integer`      |
| **Default** | `0` (disabled) |

When set to a value > 0, forces wrapping to one-arg-per-line whenever a command invocation
has **more than** this many arguments, regardless of whether it would fit within `lineWidth`.
A value of `4` means any command with 5+ arguments always wraps.

Inspired by rustfmt's heuristic width thresholds (`fn_call_width`, `array_width`). Useful for
keeping commands like `set()` compact while forcing long `target_link_libraries()` to expand.

### 1.6 `commandWidthOverrides`

|             |                                    |
| ----------- | ---------------------------------- |
| **Type**    | `table { [commandName]: integer }` |
| **Default** | `{}`                               |

Per-command override for `lineWidth`. Allows specific commands to have a tighter or looser
line-width threshold. Command names are case-insensitive.

```toml
[commandWidthOverrides]
"if" = 120
"set" = 60
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

### 2.2 `indentStyle`

|             |                    |
| ----------- | ------------------ |
| **Type**    | `"space" \| "tab"` |
| **Default** | `"space"`          |

Whether to indent with spaces or hard tab characters. When set to `"tab"`, a single `\t`
is emitted per indent level; `indentWidth` then represents the *visual width* of one tab
for the purpose of line-width calculation.

Inspired by ruff's `indent-style` and oxfmt's `useTabs`. Tabs in the input are always
normalized according to this setting.

### 2.3 `continuationIndentWidth`

|             |                                |
| ----------- | ------------------------------ |
| **Type**    | `integer \| null`              |
| **Default** | `null` (inherit `indentWidth`) |

Indentation applied to continuation lines of a wrapped command. If `null`, inherits the
value of `indentWidth`. Set explicitly when you want arguments indented more than the
structural indent (common in projects using 2-space indent but 4-space continuation).

```cmake
# indentWidth = 2, continuationIndentWidth = 4
target_link_libraries(MyTarget
    PRIVATE
        Boost::filesystem
        Threads::Threads
)
```

### 2.4 `genexIndentWidth`

|             |                                |
| ----------- | ------------------------------ |
| **Type**    | `integer \| null`              |
| **Default** | `null` (inherit `indentWidth`) |

Override indentation specifically inside generator expressions (`$<...>`). Generator
expressions can be deeply nested, and some teams prefer a narrower indent inside them
to reduce rightward drift.

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

### 3.1 `maxBlankLines`

|             |           |
| ----------- | --------- |
| **Type**    | `integer` |
| **Default** | `1`       |

Maximum number of **consecutive** blank lines preserved anywhere in the file — between
top-level commands and within argument lists. Runs exceeding this count are collapsed.
A value of `0` collapses *all* blank lines.

Inspired by rustfmt's `blank_lines_upper_bound`.

### 3.2 `minBlankLinesBetweenBlocks`

|             |           |
| ----------- | --------- |
| **Type**    | `integer` |
| **Default** | `0`       |

Minimum number of blank lines inserted between top-level block-opening commands
(`if`, `foreach`, `function`, `macro`) and the preceding command. Ensures visual
separation of logical sections.

A value of `1` guarantees at least one blank line before every `if()`, `foreach()`,
`function()`, or `macro()` block, unless the preceding line is a comment that belongs
to the block.

Inspired by rustfmt's `blank_lines_lower_bound`.

### 3.3 `blankLineAfterSectionKeyword`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, insert a blank line after each section keyword (`PUBLIC`, `PRIVATE`,
`INTERFACE`) inside commands that contain multiple sections, improving visual separation
between dependency groups.

```cmake
# blankLineAfterSectionKeyword = true
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

### 4.2 `keywordCase`

|             |                                     |
| ----------- | ----------------------------------- |
| **Type**    | `"lower" \| "upper" \| "unchanged"` |
| **Default** | `"upper"`                           |

Casing applied to recognized keywords. The formatter ships with a built-in keyword
dictionary (VERSION, PRIVATE, PUBLIC, INTERFACE, PROPERTIES, FILE_SET, TYPE, HEADERS,
BASE_DIRS, FILES, LANGUAGES, REQUIRED, COMPONENTS, CONFIG, TARGETS, DESTINATION,
NAMESPACE, EXPORT, IMPORTED, GLOBAL, ALIAS, STATIC, SHARED, MODULE, OBJECT,
EXCLUDE_FROM_ALL, SOURCES, DEPENDS, COMMAND, WORKING_DIRECTORY, COMMENT, etc.).

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

### 4.4 `literalCase`

|             |                                     |
| ----------- | ----------------------------------- |
| **Type**    | `"upper" \| "lower" \| "unchanged"` |
| **Default** | `"unchanged"`                       |

Casing applied to well-known boolean/constant literals: `ON`, `OFF`, `TRUE`, `FALSE`,
`YES`, `NO`, `AND`, `OR`, `NOT`, `STREQUAL`, `MATCHES`, etc.

Inspired by rustfmt's `hex_literal_case`. Normalizing these to uppercase is a common
convention, but many projects prefer leaving them as-is.

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

When `false`, the `)` stays on the last argument's line.

### 5.2 `spaceBeforeParen`

|             |                       |
| ----------- | --------------------- |
| **Type**    | `boolean \| string[]` |
| **Default** | `false`               |

Controls whether a space is inserted between the command name and the opening `(`.

- `false` (default): No space — `if(...)`.
- `true`: Space for all commands — `if (...)`.
- `["if", "elseif", "while", "foreach"]`: Space only for the listed commands.

Command names in the list are matched case-insensitively, after `commandCase` normalization.

### 5.3 `spaceInsideParen`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, insert a space after the opening `(` and before the closing `)` on
single-line invocations.

```cmake
# spaceInsideParen = true
set( MY_VAR "hello" )

# spaceInsideParen = false (default)
set(MY_VAR "hello")
```

Does not apply to multi-line commands (spacing is controlled by indentation in that case).

---

## 6 · Comments

### 6.1 `commentPreservation`

|             |                                     |
| ----------- | ----------------------------------- |
| **Type**    | `"preserve" \| "reflow" \| "strip"` |
| **Default** | `"preserve"`                        |

Controls how the formatter handles comments.

- **`"preserve"`** (default): Comments are kept in-place. Inline comments within argument
  lists are re-indented to match surrounding arguments. Standalone comment lines between
  commands are preserved verbatim (content and relative position unchanged).

- **`"reflow"`**: Comment text is reflowed to fit within `lineWidth`, respecting the
  current indentation. Leading `#` markers and any initial whitespace pattern are preserved.
  Block comments (consecutive `#` lines) are treated as a single paragraph for reflow.

- **`"strip"`**: Remove all comments. **Use with extreme caution.**

### 6.2 `commentWidth`

|             |                              |
| ----------- | ---------------------------- |
| **Type**    | `integer \| null`            |
| **Default** | `null` (inherit `lineWidth`) |

Maximum line width for comments specifically. When `null`, inherits `lineWidth`.
Only effective when `commentPreservation` is `"reflow"`.

Inspired by rustfmt's `comment_width`, which defaults to 80 independently of `max_width`.

### 6.3 `alignTrailingComments`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, align trailing comments (`# ...`) on consecutive lines to start at the same
column. The alignment column is determined by the longest code segment in the group plus
a minimum gap of two spaces.

```cmake
# alignTrailingComments = true
set(FOO "bar")         # The foo variable
set(BAZ_LONG "qux")   # The baz variable
set(X "y")             # Short one

# alignTrailingComments = false (default)
set(FOO "bar") # The foo variable
set(BAZ_LONG "qux") # The baz variable
set(X "y") # Short one
```

Inspired by clang-format's `AlignTrailingComments` and the feature requests tracked
for rustfmt.

### 6.4 `commentGap`

|             |           |
| ----------- | --------- |
| **Type**    | `integer` |
| **Default** | `1`       |

Minimum number of spaces between the end of a code token and the start of a trailing `#` comment.

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
  If the file has no line endings (single-line file), default to `"lf"`.

Inspired by ruff's `line-ending` and rustfmt's `newline_style`.

### 7.2 `finalNewline`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, ensure the file ends with exactly one trailing newline. Excess trailing
newlines are removed; a missing trailing newline is added. Empty or whitespace-only
files are normalized to a single newline character.

When `false`, do not add a trailing newline if one is absent, and do not strip trailing
newlines if present (aside from `maxBlankLines` enforcement).

Inspired by oxfmt's `insertFinalNewline`. POSIX convention and most editors expect a
trailing newline, so the default is `true`.

---

## 8 · Whitespace Normalization

### 8.1 `tabToSpaces`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, all tab characters in the input (including within argument values) are
converted to `indentWidth` spaces (or the configured tab width) before formatting.
When `false`, tab characters within quoted strings are preserved as-is, but
indentation tabs are still normalized according to `indentStyle`.

### 8.2 `trimTrailingWhitespace`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, remove any trailing whitespace (spaces, tabs) at the end of every line.
This is standard hygiene for version-controlled files and almost universally desired.

### 8.3 `collapseSpaces`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, collapse runs of multiple spaces between arguments on the same line
to a single space. Does not affect indentation (which is controlled by `indentWidth`)
or spaces inside quoted strings.

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
  CXX_STANDARD          17
  CXX_STANDARD_REQUIRED ON
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

### 9.2 `alignConsecutiveSet`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, align the values of consecutive `set()` commands that form a logical group
(not separated by blank lines or non-`set` commands).

```cmake
# alignConsecutiveSet = true
set(FOO     "bar")
set(BAZ     "qux")
set(LONGER  "value")

# alignConsecutiveSet = false (default)
set(FOO "bar")
set(BAZ "qux")
set(LONGER "value")
```

Inspired by rustfmt's `enum_discrim_align_threshold` and clang-format's alignment family.

### 9.3 `alignArgGroups`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, and when a command's arguments are laid out one-per-line, attempt to
column-align groups of arguments that have parallel structure. This primarily affects
commands like `install()` or `configure_file()` where pairs of related values appear.

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
  `indentWidth`) spaces. Condition/value separation and closing `>` placement follow
  consistent rules.

- **`"never"`**: Generator expressions are always kept on a single line, regardless
  of length. This can cause line-width violations for deeply nested genexes.

### 10.2 `genexClosingAngleSameLine`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

Analogous to `closingParenNewline` but for the closing `>` of generator expressions.

- `false` (default): Closing `>` on its own line, aligned with the `$<` opener.
- `true`: Closing `>` stays on the last content line.

---

## 11 · Command-Specific Overrides

### 11.1 `perCommandConfig`

|             |                                    |
| ----------- | ---------------------------------- |
| **Type**    | `table { [commandName]: { ... } }` |
| **Default** | `{}`                               |

Allows overriding a subset of formatting options on a per-command basis. The command name
is matched case-insensitively. Any option from groups 1, 2, 5, and 9 may be overridden.

```toml
[perCommandConfig.set]
forceOneArgPerLine = false

[perCommandConfig.target_link_libraries]
forceOneArgPerLine = true
closingParenNewline = true

[perCommandConfig.if]
spaceBeforeParen = true
```

Inspired by rustfmt's `fn_call_width` / `struct_lit_width` per-construct heuristics and
ruff's per-rule configuration.

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
- `true`: Sort arguments in all recognized keyword sections (SOURCES, PRIVATE, PUBLIC,
  INTERFACE, FILES, DEPENDS).
- `["SOURCES", "FILES"]`: Only sort arguments under the listed keyword sections.

Sorting is stable: arguments that compare equal retain their original relative order.
Comments attached to an argument (immediately preceding comment lines or trailing
comments) travel with the argument during sorting.

Inspired by ruff/isort's import sorting and rustfmt's `reorder_imports`.

### 12.2 `sortKeywordSections`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, reorder keyword sections within a command to a canonical order. For
`target_link_libraries` and similar commands, the canonical order is
`PUBLIC` → `PRIVATE` → `INTERFACE`.

This is an opinionated option and off by default.

---

## 13 · Formatting Suppression

### 13.1 Inline Pragma Comments

The formatter recognizes special comment directives to locally disable formatting:

```cmake
# cmake-format: off
set(MY_CAREFULLY_ALIGNED_MATRIX
  1 0 0 0
  0 1 0 0
  0 0 1 0
  0 0 0 1
)
# cmake-format: on
```

Everything between `# cmake-format: off` and `# cmake-format: on` is preserved
verbatim — no indentation changes, no wrapping, no casing normalization.

A single-command skip is also supported:

```cmake
# cmake-format: skip
set(INTENTIONALLY_UGLY_BUT_FUNCTIONAL "value"   "value2"     "value3")
```

This skips formatting for the immediately following command invocation only.

Inspired by ruff's `# fmt: off` / `# fmt: on` / `# fmt: skip` pragmas and
clang-format's `// clang-format off` / `// clang-format on`.

### 13.2 `disableFormatting`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

A master switch that disables all formatting transformations. The file is read and
written back with no changes. Useful for testing, CI dry-runs, or temporarily
disabling formatting in a subtree via a nested config file.

Inspired by rustfmt's `disable_all_formatting`.

### 13.3 `ignorePatterns`

|             |            |
| ----------- | ---------- |
| **Type**    | `string[]` |
| **Default** | `[]`       |

Glob patterns for files that should be skipped entirely. Patterns are resolved
relative to the configuration file's directory.

```toml
ignorePatterns = [
  "third_party/**",
  "generated/*.cmake",
  "build/**"
]
```

Inspired by oxfmt's `ignorePatterns` and ruff's `exclude`.

### 13.4 `ignoreCommands`

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

---

## 14 · Trailing Tokens

### 14.1 `magicTrailingNewline`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, a trailing newline within an argument list (i.e., the author placed the last
argument on its own line, with nothing after it before the `)`) is treated as an explicit
signal to keep the expanded layout, even if the invocation would fit on a single line.

This mirrors the behavior of ruff's "magic trailing comma"
(`skip-magic-trailing-comma = false`), adapted to CMake where commas don't exist
but trailing newlines serve a similar role as an author-intent signal.

When `false`, the formatter collapses any invocation that fits onto a single line regardless
of the original layout.

### 14.2 `trailingSpaceInParens`

|             |                          |
| ----------- | ------------------------ |
| **Type**    | `"remove" \| "preserve"` |
| **Default** | `"remove"`               |

Controls handling of whitespace between the last argument and `)` on a single-line command.

- `"remove"` (default): `set( FOO "bar" )` → `set(FOO "bar")` (combined with `spaceInsideParen`).
- `"preserve"`: Keep any trailing space that was present in the input.

---

## 15 · Conditional & Flow Control Formatting

### 15.1 `indentBlockBody`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, the body of flow-control blocks (`if`/`elseif`/`else`/`endif`,
`foreach`/`endforeach`, `while`/`endwhile`, `function`/`endfunction`,
`macro`/`endmacro`) is indented by `indentWidth`.

When `false`, no additional indentation is applied to block bodies. This produces
a flat style sometimes seen in older CMake codebases.

### 15.2 `indentBlockGuards`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, the block-opening and block-closing commands themselves are indented
relative to an enclosing block. This produces a Python-like visual nesting:

```cmake
# indentBlockGuards = true
function(my_func)
  if(condition)
    if(nested)
      message("deep")
    endif()
  endif()
endfunction()

# indentBlockGuards = false (default) — only body is indented
function(my_func)
  if(condition)
    if(nested)
      message("deep")
    endif()
  endif()
endfunction()
```

Note: with the default (`false`), `endif()`, `endforeach()`, etc. sit at the same indent
as their opening counterpart. With `true`, they are indented like the body.

### 15.3 `elseOnNewline`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

Whether `else()` / `elseif()` always starts on its own line. This is almost universally
`true` for CMake, but the option exists for completeness.

### 15.4 `endCommandArgs`

|             |                                     |
| ----------- | ----------------------------------- |
| **Type**    | `"remove" \| "preserve" \| "match"` |
| **Default** | `"remove"`                          |

Controls the arguments inside block-closing commands (`endif()`, `endfunction()`,
`endforeach()`, `endmacro()`, `endwhile()`). Older CMake required repeating the
condition/name; modern CMake does not.

- **`"remove"`** (default): Strip arguments from closing commands.
  `endif(condition)` → `endif()`.
- **`"preserve"`**: Keep whatever arguments are present.
- **`"match"`**: If arguments are present, ensure they match the opening command's
  condition/name. Report a warning if they don't match, but preserve them.

---

## 16 · Quote Style

### 16.1 `quoteStyle`

|             |                           |
| ----------- | ------------------------- |
| **Type**    | `"double" \| "unchanged"` |
| **Default** | `"unchanged"`             |

Controls normalization of string quoting.

- **`"double"`**: Ensure all explicitly quoted strings use double quotes. Bare/unquoted
  arguments are not affected.
- **`"unchanged"`** (default): Preserve the original quoting style.

CMake only supports double-quoted strings, so this option is primarily about whether
to *add* quotes to arguments that might benefit from them. The formatter never removes
quotes that are semantically necessary.

### 16.2 `quoteUnquotedPaths`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, wrap unquoted arguments that look like file paths (containing `/`, `\`,
or `.`) in double quotes. This prevents subtle bugs with paths containing spaces on
some platforms.

---

## 17 · Diff & Output Modes

These options control the CLI behavior of the formatter, not the formatting output itself.
They are included here for completeness and to document expected behavior for tooling
integration.

### 17.1 `--check` (CLI flag)

Exit with a non-zero status code if any file would be changed, without writing changes.
Useful for CI enforcement. Inspired by `ruff format --check` and `oxfmt --check`.

### 17.2 `--diff` (CLI flag)

Print a unified diff of formatting changes to stdout instead of writing files in-place.

### 17.3 `--stdin` (CLI flag)

Read CMake source from stdin and write formatted output to stdout. Used for editor
integration (pipe through formatter).

### 17.4 `--print-config` (CLI flag)

Print the resolved configuration (after merging defaults, config file, and CLI overrides)
as TOML to stdout. Useful for debugging. Inspired by `rustfmt --print-config default`.

---

## 18 · Configuration Meta

### 18.1 `$schema`

|             |          |
| ----------- | -------- |
| **Type**    | `string` |
| **Default** | *(none)* |

Optional JSON Schema URL for editor validation and autocomplete. Has no effect on the
formatter itself.

```toml
"$schema" = "https://raw.githubusercontent.com/yourorg/cmakefmt/main/schema.json"
```

### 18.2 `extends`

|             |          |
| ----------- | -------- |
| **Type**    | `string` |
| **Default** | *(none)* |

Path to another `.cmakefmt.toml` file to use as a base. Options from the current file
override the base. Allows sharing a common config across a monorepo while permitting
per-directory tweaks.

```toml
extends = "../../.cmakefmt.toml"

[perCommandConfig.set]
forceOneArgPerLine = false
```

---

## Summary Table

| #    | Option                         | Type                  | Default        |
| ---- | ------------------------------ | --------------------- | -------------- |
| 1.1  | `lineWidth`                    | `integer`             | `80`           |
| 1.2  | `forceOneArgPerLine`           | `boolean`             | `false`        |
| 1.3  | `wrapStyle`                    | `enum`                | `"cascade"`    |
| 1.4  | `firstArgSameLine`             | `boolean \| string[]` | `true`         |
| 1.5  | `wrapArgThreshold`             | `integer`             | `0`            |
| 1.6  | `commandWidthOverrides`        | `table`               | `{}`           |
| 2.1  | `indentWidth`                  | `integer`             | `2`            |
| 2.2  | `indentStyle`                  | `enum`                | `"space"`      |
| 2.3  | `continuationIndentWidth`      | `integer \| null`     | `null`         |
| 2.4  | `genexIndentWidth`             | `integer \| null`     | `null`         |
| 3.1  | `maxBlankLines`                | `integer`             | `1`            |
| 3.2  | `minBlankLinesBetweenBlocks`   | `integer`             | `0`            |
| 3.3  | `blankLineAfterSectionKeyword` | `boolean`             | `false`        |
| 4.1  | `commandCase`                  | `enum`                | `"lower"`      |
| 4.2  | `keywordCase`                  | `enum`                | `"upper"`      |
| 4.3  | `customKeywords`               | `string[]`            | `[]`           |
| 4.4  | `literalCase`                  | `enum`                | `"unchanged"`  |
| 5.1  | `closingParenNewline`          | `boolean`             | `true`         |
| 5.2  | `spaceBeforeParen`             | `boolean \| string[]` | `false`        |
| 5.3  | `spaceInsideParen`             | `boolean`             | `false`        |
| 6.1  | `commentPreservation`          | `enum`                | `"preserve"`   |
| 6.2  | `commentWidth`                 | `integer \| null`     | `null`         |
| 6.3  | `alignTrailingComments`        | `boolean`             | `false`        |
| 6.4  | `commentGap`                   | `integer`             | `1`            |
| 7.1  | `lineEnding`                   | `enum`                | `"auto"`       |
| 7.2  | `finalNewline`                 | `boolean`             | `true`         |
| 8.1  | `tabToSpaces`                  | `boolean`             | `true`         |
| 8.2  | `trimTrailingWhitespace`       | `boolean`             | `true`         |
| 8.3  | `collapseSpaces`               | `boolean`             | `true`         |
| 9.1  | `alignPropertyValues`          | `boolean`             | `false`        |
| 9.2  | `alignConsecutiveSet`          | `boolean`             | `false`        |
| 9.3  | `alignArgGroups`               | `boolean`             | `false`        |
| 10.1 | `genexWrap`                    | `enum`                | `"cascade"`    |
| 10.2 | `genexClosingAngleSameLine`    | `boolean`             | `false`        |
| 11.1 | `perCommandConfig`             | `table`               | `{}`           |
| 12.1 | `sortArguments`                | `boolean \| string[]` | `false`        |
| 12.2 | `sortKeywordSections`          | `boolean`             | `false`        |
| 13.1 | *(pragma comments)*            | —                     | —              |
| 13.2 | `disableFormatting`            | `boolean`             | `false`        |
| 13.3 | `ignorePatterns`               | `string[]`            | `[]`           |
| 13.4 | `ignoreCommands`               | `string[]`            | `[]`           |
| 14.1 | `magicTrailingNewline`         | `boolean`             | `true`         |
| 14.2 | `trailingSpaceInParens`        | `enum`                | `"remove"`     |
| 15.1 | `indentBlockBody`              | `boolean`             | `true`         |
| 15.2 | `indentBlockGuards`            | `boolean`             | `false`        |
| 15.3 | `elseOnNewline`                | `boolean`             | `true`         |
| 15.4 | `endCommandArgs`               | `enum`                | `"remove"`     |
| 16.1 | `quoteStyle`                   | `enum`                | `"unchanged"`  |
| 16.2 | `quoteUnquotedPaths`           | `boolean`             | `false`        |
| 17.x | *(CLI flags)*                  | —                     | —              |
| 18.1 | `$schema`                      | `string`              | —              |
| 18.2 | `extends`                      | `string`              | —              |

---

## Appendix A — Default Configuration

The following `.cmakefmt.toml` shows all options at their default values:

```toml
lineWidth = 80
forceOneArgPerLine = false
wrapStyle = "cascade"
firstArgSameLine = true
wrapArgThreshold = 0
indentWidth = 2
indentStyle = "space"
# continuationIndentWidth — inherits indentWidth
# genexIndentWidth — inherits indentWidth
maxBlankLines = 1
minBlankLinesBetweenBlocks = 0
blankLineAfterSectionKeyword = false
commandCase = "lower"
keywordCase = "upper"
customKeywords = []
literalCase = "unchanged"
closingParenNewline = true
spaceBeforeParen = false
spaceInsideParen = false
commentPreservation = "preserve"
# commentWidth — inherits lineWidth
alignTrailingComments = false
commentGap = 1
lineEnding = "auto"
finalNewline = true
tabToSpaces = true
trimTrailingWhitespace = true
collapseSpaces = true
alignPropertyValues = false
alignConsecutiveSet = false
alignArgGroups = false
genexWrap = "cascade"
genexClosingAngleSameLine = false
sortArguments = false
sortKeywordSections = false
disableFormatting = false
ignorePatterns = []
ignoreCommands = []
magicTrailingNewline = true
trailingSpaceInParens = "remove"
indentBlockBody = true
indentBlockGuards = false
elseOnNewline = true
endCommandArgs = "remove"
quoteStyle = "unchanged"
quoteUnquotedPaths = false
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

The cascading algorithm is the heart of the formatter. Given a command invocation:

```
command_name(arg1 KEYWORD arg2 arg3 KEYWORD2 arg4)
```

**Step 1 — Single line.** Compute the rendered width. If ≤ `lineWidth`, emit on one line.

**Step 2 — Keyword breaks.** Place each keyword on a new line, indented by `indentWidth`
from the command. Pack that keyword's value arguments onto the same line. If any keyword
line still exceeds `lineWidth`, escalate to step 3 for that keyword group.

```cmake
target_link_libraries(MyTarget
  PRIVATE Boost::filesystem Threads::Threads
  PUBLIC some_other_lib
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

When `firstArgSameLine` is `true`, `MyTarget` stays on the opening line in all steps.

The algorithm recurses into generator expressions, treating `$<` as an opening bracket
and `>` as a closing bracket, with `genexIndentWidth` controlling nested indentation.