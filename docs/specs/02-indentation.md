## 2 · Indentation

### 2.1 `indentWidth`

|             |           |
| ----------- | --------- |
| **Type**    | `integer` |
| **Default** | `2`       |
| **Range**   | `1 – 8`   |

Number of spaces (or tab stops) per indentation level. Each nesting level — keywords under
a command, values under a keyword, and body blocks of
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

When `indentStyle = "tab"` and `continuationIndentWidth` differs from `indentWidth`, the formatter uses tabs for whole-`indentWidth` multiples and spaces for the remainder. For example, with `indentWidth = 4` and `continuationIndentWidth = 6`: a keyword at depth 1 uses 1 tab (4 columns), and a value line at the keyword's depth + 6 columns = column 10 is rendered as 2 tabs + 2 spaces. This "tabs for indentation, spaces for alignment" strategy ensures consistent display regardless of tab-width settings.

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

When a keyword group fits within `lineWidth` (Step 2 of the cascade algorithm), values
pack inline with their keyword — `continuationIndentWidth` does not apply in this layout:

```cmake
# indentWidth = 2, continuationIndentWidth = 4
target_link_libraries(MyTarget
  PRIVATE Boost::filesystem Threads::Threads
)
```

When the keyword group does not fit within `lineWidth` and escalates to one-per-line
layout (Step 3), values indent by `continuationIndentWidth` relative to the keyword:

```cmake
# indentWidth = 2, continuationIndentWidth = 4, lineWidth = 47
target_link_libraries(MyTarget
  PRIVATE
      Boost::filesystem
      Threads::Threads
      fmt::fmt
      spdlog::spdlog
)
```

In the Step 3 example, `PRIVATE` is indented by `indentWidth` (2) relative to the command,
and the library names are indented by `continuationIndentWidth` (4) relative to `PRIVATE`.

For commands without recognized keywords (including the universal keyword set — see Appendix F §F.1.3), arguments are indented by `indentWidth` (not `continuationIndentWidth`). `continuationIndentWidth` only applies to value arguments appearing under a recognized keyword.
