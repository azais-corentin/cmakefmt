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
  For `else()`, `"match"` copies the `if()` condition from the nearest unmatched enclosing
  `if()` in syntactic nesting order. `elseif()` is unaffected by `"match"` — it has its
  own condition expression.

For `else()`: under `"remove"`, legacy arguments are stripped (`else(condition)` → `else()`).
Under `"match"`, the nearest unmatched enclosing `if()` condition is copied into `else()`.
Under `"preserve"`, existing arguments are kept as-is. `elseif()` always retains its own
condition expression — `"remove"` and `"match"` do not alter it because its condition is
definitional, not a repetition of the opening command.

For `block()`/`endblock()`: since `block()` has no positional name argument (only keyword clauses like `SCOPE_FOR` and `PROPAGATE`), `endCommandArgs = "match"` produces `endblock()` unconditionally.

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

Block matching follows CMake's syntactic nesting — each closing command is paired with the nearest unmatched opening command of the corresponding type, and each `else()`/`elseif()` is associated with the nearest unmatched enclosing `if()`.

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
