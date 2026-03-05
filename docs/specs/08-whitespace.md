## 8 · Whitespace Normalization

### 8.1 `trimTrailingWhitespace`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, remove any trailing whitespace (spaces, tabs) at the end of every line.
This is standard hygiene for version-controlled files and almost universally desired.

```cmake
# Input (· represents trailing spaces):
set(FOO "bar")···
message(STATUS "hello")·

# Output (trimTrailingWhitespace = true):
set(FOO "bar")
message(STATUS "hello")
```

### 8.2 `collapseSpaces`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, collapse runs of multiple spaces between arguments on the same line
to a single space. `collapseSpaces` applies during input normalization, before alignment.
Alignment-generated padding (from `alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`,
or `alignTrailingComments`) is inserted after collapsing and is therefore exempt. Does not affect
indentation (which is controlled by `indentWidth`) or spaces inside quoted strings.

```cmake
# Input:
set(FOO    "bar"    "baz")

# Output (collapseSpaces = true):
set(FOO "bar" "baz")
```
