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
- `true`: Sort arguments only in sections marked as sortable in Appendix F (§F.2), plus
  sections introduced via `customKeywords` (§4.3) when those sections contain simple values.
  Nested section structures are never value-sorted (Appendix F §F.3). Concretely, this means
  sections such as `PRIVATE`/`PUBLIC`/`INTERFACE` (for supported target commands), `DEPENDS`,
  `BYPRODUCTS`, `COMPONENTS`, and `OPTIONAL_COMPONENTS` are sortable, while structures such as
  `FILE_SET ... BASE_DIRS ... FILES ...` and `PROPERTIES <key> <value> ...` are not.
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
