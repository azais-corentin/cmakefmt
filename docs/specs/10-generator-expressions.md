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
