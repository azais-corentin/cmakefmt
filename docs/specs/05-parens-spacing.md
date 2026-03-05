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
