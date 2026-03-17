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

For properties with multiple values (e.g., `LINK_LIBRARIES lib1 lib2`), all values remain on the same line after the key at the alignment column. Standard cascade wrapping applies within the remaining line width.

```cmake
# alignPropertyValues = true, multi-value property
set_target_properties(MyTarget PROPERTIES
  CXX_STANDARD              17
  LINK_LIBRARIES            lib1 lib2 lib3
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

More precisely, two consecutive argument lines are candidates for columnar alignment when they contain the same number of CMake arguments (where each argument is a quoted string, unquoted word, or generator expression — each counted as a single argument regardless of internal content). Alignment pads each column with spaces to the maximum width of that column across all lines in the group. When multiple values are packed onto a single line by the wrapping stage, they use single-space separation — column alignment padding is not applied within a packed line, only across corresponding lines.

A "token" in this context is a single CMake argument — a quoted string, unquoted argument,
or generator expression (including its entire nested content as one token).

**Detection heuristics:**

- A group requires at least 2 consecutive lines with the same token count.
- Lines where alignment would cause any column to extend beyond `lineWidth` are excluded
  from alignment and rendered normally.
- A blank line or a comment line breaks the current group; alignment restarts after it.
- Arguments under different section keywords are never aligned across keyword boundaries.
- Additionally, keyword-as-first-token lines are aligned by their keyword column regardless of the total token count on each line. This is separate from the same-token-count alignment of value lines.
- Valueless keywords (keywords with no value arguments, e.g., `VERBATIM`) are excluded from keyword-column width calculation and are not padded. They appear at their natural width on their own line.
- Keyword-specific value wrapping is preserved under alignment. Flow keywords (e.g., `COMMAND`) wrap their values at `lineWidth` with continuation lines indented to the keyword value column (keyword column width + base indent).

> **Known limitation:** The heuristic matches on token count alone and does not verify
> structural similarity. Two consecutive lines with the same number of tokens but
> semantically different structure (e.g., a path and a flag) will be aligned. In practice,
> this rarely produces undesirable results because CMake argument lists are typically
> homogeneous within a keyword section.

**Example 1 — `install(TARGETS ...)`:**

```cmake
# alignArgGroups = true
install(TARGETS
  MyLib      RUNTIME DESTINATION bin
  MyOtherLib RUNTIME DESTINATION lib
  MyPlugin   LIBRARY DESTINATION plugins
)
```

**Example 2 — `add_custom_command` with keyword alignment and flow wrapping:**

```cmake
# alignArgGroups = true, lineWidth = 60
add_custom_command(
  OUTPUT  ${CMAKE_CURRENT_BINARY_DIR}/generated.cpp
  COMMAND generator --input schema.json --output
          generated.cpp
  DEPENDS schema.json
  COMMENT "Generating code"
  VERBATIM
)
```

In this example, keyword-as-first-token lines (`OUTPUT`, `COMMAND`, `DEPENDS`, `COMMENT`) are aligned by their keyword column (width 8 = max keyword width 7 + 1 gap). `COMMAND` values flow-wrap at `lineWidth = 60` with continuation at the value column. `VERBATIM` (valueless) is not padded and does not participate in keyword-column width calculation.

**Example 3 — `set()` with tabular data:**

```cmake
# alignArgGroups = true
set(MY_TABLE
  "name"  "type"   "default"
  "width" "int"    "80"
  "style" "string" "cascade"
)
```

**Example 4 — `target_sources` with `FILE_SET`:**

```cmake
# alignArgGroups = true
target_sources(MyLib
  FILE_SET HEADERS
    BASE_DIRS include
    FILES
      include/mylib/core.h include/mylib/utils.h
  FILE_SET CXX_MODULES
    BASE_DIRS src
    FILES src/core.cppm src/utils.cppm
)
```
