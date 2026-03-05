## 15 · Configuration Meta

### 15.1 `$schema`

|             |          |
| ----------- | -------- |
| **Type**    | `string` |
| **Default** | *(none)* |

Optional JSON Schema URL for editor validation and autocomplete. Has no effect on the
formatter itself.

```toml
"$schema" = "https://raw.githubusercontent.com/yourorg/cmakefmt/main/schema.json"
```

### 15.2 `extends`

|             |          |
| ----------- | -------- |
| **Type**    | `string` |
| **Default** | *(none)* |

Path to another `.cmakefmt.toml` file to use as a base. Options from the current file
override the base. Allows sharing a common config across a monorepo while permitting
per-directory tweaks. Direct or transitive circular references in the `extends` chain are detected and produce an error.
Implementations should impose a reasonable maximum depth for `extends` chains (e.g., 32 levels) to guard against pathological configurations. Exceeding this limit produces an error.

The path is resolved relative to the directory containing the config file that declares it.
Absolute paths are used as-is.

**Merge strategy:** Scalars in the child override the base. Any option with an array value in the child
replaces the base array entirely (this applies to all array-typed option values, including `customKeywords`,
`ignorePatterns`, `ignoreCommands`, `sortArguments` when array-valued, and `spaceBeforeParen` when
array-valued). Tables (`perCommandConfig`) are shallow-merged: top-level command keys in the child override the
base entry for that command *entirely* (not field-by-field). Base command entries not present
in the child are preserved.

```toml
extends = "../../.cmakefmt.toml"

[perCommandConfig.set]
wrapStyle = "vertical"
```

### 15.3 Unknown Keys

|              |         |
| ------------ | ------- |
| **Behavior** | Warning |

Unknown keys in `.cmakefmt.toml` produce a diagnostic warning (including the key name
and file path) and are ignored. This allows forward-compatibility: a config file written
for a newer formatter version can be used with an older version without errors.

Unknown keys inside `perCommandConfig` tables follow the same policy.
