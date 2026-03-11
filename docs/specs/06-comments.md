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
set(FOO "bar")      # The foo variable
set(BAZ_LONG "qux") # The baz variable
set(X "y")          # Short one

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
