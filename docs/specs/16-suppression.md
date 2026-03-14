## 16 · Suppression & Ignore Options

### 16.1 `disableFormatting`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

When `true`, formatting is fully disabled and output **MUST** be byte-for-byte identical to the input.
In this mode, the formatter applies no transformations or normalizations, including `lineEnding`, `finalNewline`, `trimTrailingWhitespace`, `collapseSpaces`, `maxBlankLines`, and UTF-8 BOM stripping.
`disableFormatting = true` takes precedence over all other options and pragmas.

```cmake
# Input (irregular formatting):
SET(  FOO   "bar"  )

# Output (disableFormatting = true): byte-for-byte identical to input
SET(  FOO   "bar"  )
```

### 16.2 `ignorePatterns`

|             |            |
| ----------- | ---------- |
| **Type**    | `string[]` |
| **Default** | `[]`       |

Glob patterns for files that should be skipped entirely. Patterns are resolved
relative to the configuration file's directory. When patterns are inherited via
`extends` (§15.2), each pattern is resolved relative to the config file in which
it appears, not relative to the extending file.

Patterns use gitignore-style glob syntax: `*` matches any sequence except `/`, `**` matches
any sequence including `/`, `?` matches a single character, `[...]` matches character classes.

```toml
ignorePatterns = [
  "third_party/**",
  "generated/*.cmake",
  "build/**",
]
```

### 16.3 `ignoreCommands`

|             |            |
| ----------- | ---------- |
| **Type**    | `string[]` |
| **Default** | `[]`       |

A list of command names whose invocations should be skipped entirely during formatting
(preserved verbatim). Case-insensitive.

```toml
ignoreCommands = ["ExternalProject_Add", "FetchContent_Declare"]
```

Useful for complex macro invocations or commands with DSL-like syntax where the
formatter's heuristics may produce undesirable results.

```cmake
# ignoreCommands = ["ExternalProject_Add"]

# Input (irregular formatting):
ExternalProject_Add(googletest
    GIT_REPOSITORY  https://github.com/google/googletest.git
    GIT_TAG         release-1.12.1
)

# Output: preserved verbatim, no formatting applied
ExternalProject_Add(googletest
    GIT_REPOSITORY  https://github.com/google/googletest.git
    GIT_TAG         release-1.12.1
)
```
