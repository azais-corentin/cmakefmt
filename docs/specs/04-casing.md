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

```cmake
# Input:
SET(FOO "bar")
CMAKE_MINIMUM_REQUIRED(VERSION 3.20)

# Output (commandCase = "lower"):
set(FOO "bar")
cmake_minimum_required(VERSION 3.20)
```

### 4.2 `keywordCase`

|             |                                     |
| ----------- | ----------------------------------- |
| **Type**    | `"lower" \| "upper" \| "unchanged"` |
| **Default** | `"upper"`                           |

Casing applied to recognized keywords. The formatter uses the normative keyword dictionary
defined in Appendix F (including section keywords and command-specific keyword slots).
Examples include `VERSION`, `PRIVATE`, `PUBLIC`, `INTERFACE`, `PROPERTIES`, `REQUIRED`,
`COMPONENTS`, `CONFIG`, `TARGETS`, `DESTINATION`, and `NAMESPACE`.

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

Custom keywords participate in section detection, argument sorting (§12.1), and formatting layout — not only casing normalization. Adding a keyword to this list causes the formatter to treat it as a section header in all commands.

If a token in `customKeywords` also appears in the built-in literal list (§4.4), keyword classification takes precedence — the token is treated as a keyword in all contexts and `literalCase` does not apply to it.

### 4.4 `literalCase`

|             |                                     |
| ----------- | ----------------------------------- |
| **Type**    | `"upper" \| "lower" \| "unchanged"` |
| **Default** | `"unchanged"`                       |

Unlike keywords (§4.2), which are structural tokens that affect formatting layout (e.g., PRIVATE, PUBLIC, SOURCES), literals are boolean and comparison constants used as argument values.

Casing applied to well-known boolean/constant literals: `ON`, `OFF`, `TRUE`, `FALSE`,
`YES`, `NO`, `AND`, `OR`, `NOT`, `STREQUAL`, `STRLESS`, `STRGREATER`, `STRLESS_EQUAL`,
`STRGREATER_EQUAL`, `VERSION_EQUAL`, `VERSION_LESS`, `VERSION_GREATER`,
`VERSION_LESS_EQUAL`, `VERSION_GREATER_EQUAL`, `EQUAL`, `LESS`, `GREATER`,
`LESS_EQUAL`, `GREATER_EQUAL`, `MATCHES`, `IN_LIST`, `DEFINED`, `COMMAND`,
`POLICY`, `TARGET`, `TEST`, `EXISTS`, `IS_DIRECTORY`, `IS_SYMLINK`,
`IS_ABSOLUTE`, `IS_NEWER_THAN`, `PATH_EQUAL`.

Normalizing these literals to uppercase is a common convention, but many projects prefer leaving them as-is.

These tokens are normalized *everywhere* they appear as unquoted arguments, not only in condition contexts.
Any unquoted argument matching one of the listed tokens (case-insensitive match) is subject to `literalCase` normalization.
Literal case normalization applies only to unquoted arguments. Content inside quoted strings is never modified.

When a token appears in both the keyword dictionary and the literal list (e.g., `TARGET`, `COMMAND`, `POLICY`, `TEST`), keyword classification takes precedence when the token is parsed as a keyword (not as a value argument to another keyword) according to the current command's keyword dictionary. `literalCase` only applies when the token is used as a plain argument value.

```cmake
# Input:
option(USE_FEATURE "Enable feature" on)
if(DEFINED result AND off)

# Output (literalCase = "upper"):
option(USE_FEATURE "Enable feature" ON)
if(DEFINED result AND OFF)
```
