# CMake Formatter — Configuration & Formatting Rules Specification

This document defines every formatting behavior and its associated configuration option for the CMake formatter.
Options are organized into logical groups. Each option includes its type, default value, a description of its
effect, and concrete before/after examples where helpful.

Input is expected to be valid UTF-8. Non-UTF-8 input is rejected with an error.
When `disableFormatting = false` (default), a leading UTF-8 BOM (byte-order mark) is stripped from input and never emitted in output.
When `disableFormatting = true` (§13.7), output is byte-for-byte identical to input and no preprocessing or normalization (including BOM stripping) is applied.

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

### 1.2 `wrapStyle`

|             |                                           |
| ----------- | ----------------------------------------- |
| **Type**    | `"cascade" \| "vertical"` |
| **Default** | `"cascade"`                               |

Controls the overall line-wrapping philosophy. This is the master switch for wrapping behavior.

- **`"cascade"`** (default) — Three-step strategy:
  1. *Fit on single line* if total width ≤ `lineWidth`.
  2. *Keywords on new lines, arguments inline* — each keyword starts a new line, and its
     arguments are packed onto that line as space allows.
  3. *One argument per line* — if step 2 still overflows, every argument gets its own line.

  This applies recursively to all nesting levels, including generator expressions.

- **`"vertical"`** — Equivalent to step 1 → step 3 directly (skip step 2). Produces a
  strictly vertical style whenever a command does not fit on a single line.

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


### 1.4 `wrapArgThreshold`

|             |                |
| ----------- | -------------- |
| **Type**    | `integer`      |
| **Default** | `0` (disabled) |
| **Range**   | `0 – 999`      |

When set to a value > 0, forces wrapping to one-arg-per-line whenever a command invocation
has **more than** this many arguments, regardless of whether it would fit within `lineWidth`.
The command name is not counted. Only tokens inside the parentheses count — this includes
keywords and value arguments. For example,
`target_link_libraries(MyTarget PRIVATE foo bar)` has 4 arguments.
A value of `4` means any command with 5+ arguments always wraps.

Useful for keeping commands like `set()` compact while forcing long `target_link_libraries()` invocations to expand.

### 1.5 `magicTrailingNewline`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, a "magic trailing newline" is treated as an explicit signal to keep the
expanded layout, even if the invocation would fit on a single line. A magic trailing
newline is detected when the closing `)` appears on its own line (possibly with only
whitespace before it) in the input.

A trailing newline on a wrapped invocation serves as an author-intent signal to keep the expanded layout.

When `false`, the formatter collapses any invocation that fits onto a single line regardless
of the original layout.

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

When set to `"space"` (default), any tab characters in the input are converted to
`indentWidth` spaces. When set to `"tab"`, indentation uses tab characters; tab characters
within quoted strings are always preserved regardless of this setting.


### 2.3 `continuationIndentWidth`

|             |                                |
| ----------- | ------------------------------ |
| **Type**    | `integer \| null`              |
| **Default** | `null` (inherit `indentWidth`) |

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
| **Range**   | `0 – 100` |

Maximum number of **consecutive** blank lines preserved anywhere in the file — between
top-level commands and within argument lists. Runs exceeding this count are collapsed.
A value of `0` collapses *all* blank lines.

When `minBlankLinesBetweenBlocks` exceeds `maxBlankLines`, `minBlankLinesBetweenBlocks`
takes precedence at block boundaries. The formatter inserts the minimum required blank
lines even if this exceeds `maxBlankLines`.


### 3.2 `minBlankLinesBetweenBlocks`

|             |           |
| ----------- | --------- |
| **Type**    | `integer` |
| **Default** | `0`       |
| **Range**   | `0 – 10`  |

Minimum number of blank lines inserted between top-level block-opening commands
(`if`, `foreach`, `function`, `macro`) and the preceding command. Ensures visual
separation of logical sections.

A value of `1` guarantees at least one blank line before every `if()`, `foreach()`,
`function()`, or `macro()` block, unless the preceding line is a comment that belongs
to the block.

When `minBlankLinesBetweenBlocks` exceeds `maxBlankLines`, `minBlankLinesBetweenBlocks`
takes precedence at block boundaries. The formatter inserts the minimum required blank
lines even if this exceeds `maxBlankLines`.


### 3.3 `blankLineBetweenSections`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, insert a blank line after the last argument of a section, before the next
section keyword — serving as a visual separator between sections. This applies to section
keywords recognized by the formatter's keyword dictionary (e.g., `PUBLIC`, `PRIVATE`,
`INTERFACE`, `SOURCES`, `DEPENDS`, `PROPERTIES`, and others) inside commands that contain
multiple sections.

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

### 4.4 `literalCase`

|             |                                     |
| ----------- | ----------------------------------- |
| **Type**    | `"upper" \| "lower" \| "unchanged"` |
| **Default** | `"unchanged"`                       |

Casing applied to well-known boolean/constant literals: `ON`, `OFF`, `TRUE`, `FALSE`,
`YES`, `NO`, `AND`, `OR`, `NOT`, `STREQUAL`, `MATCHES`, etc.

Normalizing these literals to uppercase is a common convention, but many projects prefer leaving them as-is.

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

`spaceInsideParen` has no effect on empty argument lists — `cmd()` stays `cmd()`.

### 5.4 `trailingSpaceInParens`

|             |                          |
| ----------- | ------------------------ |
| **Type**    | `"remove" \| "preserve"` |
| **Default** | `"remove"`               |

Controls handling of whitespace between the last argument and `)` on a single-line command.

- `"remove"` (default): `set(FOO "bar" )` → `set(FOO "bar")`.
- `"preserve"`: Keep any trailing space that was present in the input.

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
  Paragraph breaks within a comment block are detected by: (a) a blank comment line (a line
  containing only `#` with optional trailing whitespace), or (b) a change in leading
  whitespace pattern after the `#` marker (e.g., indented sub-lists).
  Formatting pragma comments (§13) are never reflowed regardless of this setting.

- **`"strip"`**: Remove all comments. **Use with extreme caution.**

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
a minimum gap of two spaces.

A group of consecutive lines is broken by any blank line or any line that does not have
a trailing comment. Only lines within the same group are aligned together.

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


### 6.4 `commentGap`

|             |           |
| ----------- | --------- |
| **Type**    | `integer` |
| **Default** | `1`       |
| **Range**   | `0 – 10`  |

Minimum number of spaces between the end of a code token and the start of a trailing `#` comment.
A value of `0` produces no space between the last code token and the `#` marker
(e.g., `set(FOO "bar")# comment`).

### 6.5 Verbatim Content *(fixed behavior, not configurable)*

Bracket arguments (`[==[...]==]`) and bracket comments (`#[==[...]==]`) are preserved
verbatim by the formatter — their content is never reformatted, reflowed, or modified in
any way. This applies regardless of any other formatting settings. Only the indentation of
the opening line is adjusted to match the surrounding context.

Double-quoted strings spanning multiple lines are preserved verbatim — their content is
never reformatted or reflowed. Only the opening line's indentation is adjusted to match
the surrounding context.

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

### 8.2 `collapseSpaces`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, collapse runs of multiple spaces between arguments on the same line
to a single space. Does not affect indentation (which is controlled by `indentWidth`)
or spaces inside quoted strings.

### 8.3 Backslash Line Continuations *(fixed behavior, not configurable)*

Backslash line continuations (`\` at end-of-line) are joined before formatting.
The formatter never emits backslash continuations in output.

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

### 9.2 `alignConsecutiveSet`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, align the values of consecutive `set()` commands that form a logical group
(not separated by blank lines or non-`set` commands). This applies only to `set()` —
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


### 9.3 `alignArgGroups`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, and when a command's arguments are laid out one-per-line, detect repeating
structural patterns in the argument list and column-align them. The formatter looks for
groups of consecutive lines that share the same number of tokens and attempts to align
corresponding columns.

**Detection heuristics:**

- A group requires at least 2 consecutive lines with the same token count.
- Lines where alignment would cause any column to extend beyond `lineWidth` are excluded
  from alignment and rendered normally.
- A blank line or a comment line breaks the current group; alignment restarts after it.
- Arguments under different section keywords are never aligned across keyword boundaries.

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
  `indentWidth`) spaces. Condition/value separation and closing `>` placement follow
  consistent rules.

- **`"never"`**: Generator expressions are always kept on a single line, regardless
  of length. This applies recursively to all nesting levels —
  `$<$<CONFIG:Debug>:$<TARGET_FILE:foo>>` remains a single line regardless of depth.
  This can cause line-width violations for deeply nested genexes.

### 10.2 `genexClosingAngleNewline`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

Analogous to `closingParenNewline` but for the closing `>` of generator expressions.

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
`spaceInsideParen`, `trailingSpaceInParens`, `commentPreservation`, `commentWidth`,
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
  arguments are simple values (not sub-keyword structures). In practice this covers:
  SOURCES, PRIVATE, PUBLIC, INTERFACE, FILES, DEPENDS, COMPONENTS, TARGETS,
  CONFIGURATIONS, and any keywords added via `customKeywords` (§4.3).
- `["SOURCES", "FILES"]`: Only sort arguments under the listed keyword sections.

Sorting is case-insensitive. Arguments that compare equal after case-folding retain their
original relative order (stable sort).

A comment is considered "attached" to an argument if it immediately precedes the argument
with no blank line between them, or if it is a trailing comment on the same line as the
argument. Attached comments travel with the argument during sorting.

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

### 13.1 Pragma Syntax

The formatter recognizes inline comment directives (pragmas) that control formatting
behavior locally within a file. All pragmas use the `cmakefmt:` prefix.

```
pragma     := "#" ws? "cmakefmt:" ws? action
action     := "off" | "on" | "skip"
            | "push" ( ws assignment_list )?
            | "pop"

assignment_list := assignment ( "," ws? assignment )*
assignment      := key ws? "=" ws? value

key        := [a-zA-Z][a-zA-Z0-9]*
value      := integer | "true" | "false" | '"' [^"]* '"'
ws         := [ \t]+
```

**Rules:**

- One pragma per line. No code on the same line.
- The prefix `cmakefmt:` is case-sensitive, lowercase.
- Whitespace between `#` and `cmakefmt:` is optional: `#cmakefmt: off` and `# cmakefmt: off` are both valid.
- Values use TOML scalar syntax: `80` (integer), `true`/`false` (boolean), `"lower"` (string). Arrays and tables are not supported.
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
# cmakefmt: push lineWidth = 120
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

### 13.4 `push` / `pop`

#### 13.4.1 `push`

Create a new configuration frame on the stack. The frame inherits all values from the
current top, then applies any inline overrides.

```cmake
# cmakefmt: push lineWidth = 120, alignPropertyValues = true
set_target_properties(MyTarget PROPERTIES
  CXX_STANDARD              17
  CXX_STANDARD_REQUIRED     ON
  POSITION_INDEPENDENT_CODE ON
)
# cmakefmt: pop
```

A bare `push` (no assignments) creates a save-point with no changes.

#### 13.4.2 `pop`

Discard the top frame, restoring the configuration beneath it. `pop` when the stack has
only the root frame is a warning and is ignored.

#### 13.4.3 Nesting

Frames nest arbitrarily. Each `pop` discards exactly one frame:

```cmake
# cmakefmt: push lineWidth = 120           ← frame 1
  # cmakefmt: push indentWidth = 4         ← frame 2
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

A `push` override always takes priority over `perCommandConfig`.

### 13.5 Pragma-Settable Options

The options settable via `push` include all formatting options. Only `disableFormatting`,
`extends`, and `$schema` cannot be set in a pragma — these control configuration
infrastructure, not formatting behavior. Setting any of these produces a warning and is
ignored. Note that `push` has broader scope than `perCommandConfig` (§11.1): it can also
set file-level options such as `maxBlankLines`, `lineEnding`, `finalNewline`,
`trimTrailingWhitespace`, and `collapseSpaces`.

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

### 13.7 `disableFormatting`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, formatting is fully disabled and output **MUST** be byte-for-byte identical to the input.
In this mode, the formatter applies no transformations or normalizations, including `lineEnding`, `finalNewline`, `trimTrailingWhitespace`, `collapseSpaces`, `maxBlankLines`, and UTF-8 BOM stripping.
`disableFormatting = true` takes precedence over all other options and pragmas.

### 13.8 `ignorePatterns`

|             |            |
| ----------- | ---------- |
| **Type**    | `string[]` |
| **Default** | `[]`       |

Glob patterns for files that should be skipped entirely. Patterns are resolved
relative to the configuration file's directory. When patterns are inherited via
`extends` (§16.2), each pattern is resolved relative to the config file in which
it appears, not relative to the extending file.

```toml
ignorePatterns = [
  "third_party/**",
  "generated/*.cmake",
  "build/**"
]
```


### 13.9 `ignoreCommands`

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

### 14.2 `elseOnNewline`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

Whether `else()` / `elseif()` always starts on its own line. This is almost universally
`true` for CMake, but the option exists for completeness.

### 14.3 `endCommandArgs`

|             |                                     |
| ----------- | ----------------------------------- |
| **Type**    | `"remove" \| "preserve" \| "match"` |
| **Default** | `"remove"`                          |

Controls the arguments inside block-closing commands (`endif()`, `endfunction()`,
`endforeach()`, `endmacro()`, `endwhile()`, `endblock()`). Older CMake required repeating the
condition/name; modern CMake does not.

- **`"remove"`** (default): Strip arguments from closing commands.
  `endif(condition)` → `endif()`.
- **`"preserve"`**: Keep whatever arguments are present.
- **`"match"`**: Copies the full argument list verbatim from the corresponding opening
  command. `endwhile(x AND y)` matches `while(x AND y)`; `endfunction(my_func)` matches
  `function(my_func)`. Complex conditions with AND/OR/parentheses are copied as-is. If
  the closing command has no argument, one is added; if it has a mismatched argument, it
  is corrected.

The copied arguments are subject to normal wrapping rules — if the closing command with
its matched arguments exceeds `lineWidth`, it wraps like any other command invocation.

Nested parentheses in conditions (e.g., `if((A AND B) OR C)`) are treated as grouped
sub-expressions. When wrapping occurs, the parenthesized group is indented as a unit.

### 14.4 Empty Commands

Commands with no arguments (`endif()`, `else()`, `return()`, `endforeach()`, etc.) are always
formatted on a single line. Wrapping and indentation options do not apply to empty argument lists.

---

## 15 · Quote Style

### 15.1 `quoteUnquotedPaths`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, wrap unquoted arguments that look like file paths (containing `/`, `\`,
or `.`) in double quotes. This prevents subtle bugs with paths containing spaces on
some platforms.

---

## 16 · Configuration Meta

### 16.1 `$schema`

|             |          |
| ----------- | -------- |
| **Type**    | `string` |
| **Default** | *(none)* |

Optional JSON Schema URL for editor validation and autocomplete. Has no effect on the
formatter itself.

```toml
"$schema" = "https://raw.githubusercontent.com/yourorg/cmakefmt/main/schema.json"
```

### 16.2 `extends`

|             |          |
| ----------- | -------- |
| **Type**    | `string` |
| **Default** | *(none)* |

Path to another `.cmakefmt.toml` file to use as a base. Options from the current file
override the base. Allows sharing a common config across a monorepo while permitting
per-directory tweaks. Circular references are detected and produce an error.

**Merge strategy:** Scalars in the child override the base. Arrays (`customKeywords`,
`ignorePatterns`, `ignoreCommands`) in the child replace the base array entirely. Tables
(`perCommandConfig`) are shallow-merged: child keys override base keys, but base keys not
present in the child are preserved.

```toml
extends = "../../.cmakefmt.toml"

[perCommandConfig.set]
wrapStyle = "vertical"
```

---

## Summary Table

| #    | Option                       | Type                                      | Default       |
| ---- | ---------------------------- | ----------------------------------------- | ------------- |
| 1.1  | `lineWidth`                  | `integer`                                 | `80`          |
| 1.2  | `wrapStyle`                  | `"cascade" \| "vertical"`               | `"cascade"`   |
| 1.3  | `firstArgSameLine`           | `boolean`                                 | `true`        |
| 1.4  | `wrapArgThreshold`           | `integer`                                 | `0`           |
| 1.5  | `magicTrailingNewline`       | `boolean`                                 | `true`        |
| 2.1  | `indentWidth`                | `integer`                                 | `2`           |
| 2.2  | `indentStyle`                | `"space" \| "tab"`                        | `"space"`     |
| 2.3  | `continuationIndentWidth`    | `integer \| null`                         | `null`        |
| 2.4  | `genexIndentWidth`           | `integer \| null`                         | `null`        |
| 3.1  | `maxBlankLines`              | `integer`                                 | `1`           |
| 3.2  | `minBlankLinesBetweenBlocks` | `integer`                                 | `0`           |
| 3.3  | `blankLineBetweenSections`   | `boolean`                                 | `false`       |
| 4.1  | `commandCase`                | `"lower" \| "upper" \| "unchanged"`       | `"lower"`     |
| 4.2  | `keywordCase`                | `"lower" \| "upper" \| "unchanged"`       | `"upper"`     |
| 4.3  | `customKeywords`             | `string[]`                                | `[]`          |
| 4.4  | `literalCase`                | `"upper" \| "lower" \| "unchanged"`       | `"unchanged"` |
| 5.1  | `closingParenNewline`        | `boolean`                                 | `true`        |
| 5.2  | `spaceBeforeParen`           | `boolean \| string[]`                     | `false`       |
| 5.3  | `spaceInsideParen`           | `boolean`                                 | `false`       |
| 5.4  | `trailingSpaceInParens`      | `"remove" \| "preserve"`                  | `"remove"`    |
| 6.1  | `commentPreservation`        | `"preserve" \| "reflow" \| "strip"`       | `"preserve"`  |
| 6.2  | `commentWidth`               | `integer \| null`                         | `null`        |
| 6.3  | `alignTrailingComments`      | `boolean`                                 | `false`       |
| 6.4  | `commentGap`                 | `integer`                                 | `1`           |
| 7.1  | `lineEnding`                 | `"lf" \| "crlf" \| "auto"`                | `"auto"`      |
| 7.2  | `finalNewline`               | `boolean`                                 | `true`        |
| 8.1  | `trimTrailingWhitespace`     | `boolean`                                 | `true`        |
| 8.2  | `collapseSpaces`             | `boolean`                                 | `true`        |
| 9.1  | `alignPropertyValues`        | `boolean`                                 | `false`       |
| 9.2  | `alignConsecutiveSet`        | `boolean`                                 | `false`       |
| 9.3  | `alignArgGroups`             | `boolean`                                 | `false`       |
| 10.1 | `genexWrap`                  | `"cascade" \| "never"`                    | `"cascade"`   |
| 10.2 | `genexClosingAngleNewline`   | `boolean`                                 | `true`        |
| 11.1 | `perCommandConfig`           | `table`                                   | `{}`          |
| 12.1 | `sortArguments`              | `boolean \| string[]`                     | `false`       |
| 12.2 | `sortKeywordSections`        | `boolean`                                 | `false`       |
| 13.7 | `disableFormatting`          | `boolean`                                 | `false`       |
| 13.8 | `ignorePatterns`             | `string[]`                                | `[]`          |
| 13.9 | `ignoreCommands`             | `string[]`                                | `[]`          |
| 14.1 | `indentBlockBody`            | `boolean`                                 | `true`        |
| 14.2 | `elseOnNewline`              | `boolean`                                 | `true`        |
| 14.3 | `endCommandArgs`             | `"remove" \| "preserve" \| "match"`       | `"remove"`    |
| 15.1 | `quoteUnquotedPaths`         | `boolean`                                 | `false`       |
| 16.1 | `$schema`                    | `string`                                  | —             |
| 16.2 | `extends`                    | `string`                                  | —             |

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
spaceInsideParen = false
trailingSpaceInParens = "remove"
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
sortArguments = false
sortKeywordSections = false
disableFormatting = false
ignorePatterns = []
ignoreCommands = []
indentBlockBody = true
elseOnNewline = true
endCommandArgs = "remove"
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

**Step 0 — Pre-checks.** Before attempting single-line layout, two conditions force expansion:

- If `wrapArgThreshold > 0` and the command has more than `wrapArgThreshold` arguments, skip directly to Step 3 (one-per-line).
- If `magicTrailingNewline = true` (§1.5) and the closing `)` was on its own line in the input, preserve the expanded layout — do not collapse to a single line.

**Step 1 — Single line.** Compute the rendered width. If ≤ `lineWidth`, emit on one line.

**Step 2 — Keyword breaks.** Place each keyword on a new line, indented by `indentWidth`
relative to the command's own indentation column. Pack that keyword's value arguments onto the same line. If any keyword
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

Print a unified diff of formatting changes to stdout instead of writing files in-place.

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

---

## Appendix E — Option Interaction Rules

This appendix documents interactions between options where the combined behavior is not
obvious from reading each option's description in isolation.

| Options                                                   | Interaction                                                                                                                                                                                                                                                       |
| --------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `spaceInsideParen` + `trailingSpaceInParens`              | `trailingSpaceInParens = "remove"` takes precedence: even when `spaceInsideParen = true` adds a space after `(`, `trailingSpaceInParens = "remove"` strips the space before `)`. The two options control opposite ends of the parenthesized region independently. |
| `sortArguments` + `alignArgGroups`                        | Sorting is applied first, then alignment. Arguments are reordered within their keyword section, and the resulting layout is then column-aligned if `alignArgGroups` is enabled.                                                                                   |
| `commentPreservation = "strip"` + `alignTrailingComments` | When comments are stripped, there are no trailing comments to align. `alignTrailingComments` is effectively a no-op.                                                                                                                                              |

| `indentStyle = "tab"` + alignment options | When `indentStyle = "tab"`, alignment padding (the spaces after the leading tab indentation that align columns) always uses space characters, never tabs. Leading indentation uses tabs; alignment columns use spaces. |