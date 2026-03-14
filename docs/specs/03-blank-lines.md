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
