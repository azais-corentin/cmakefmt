## 13 · Inline Pragmas

### 13.1 Pragma Syntax

The formatter recognizes inline comment directives (pragmas) that control formatting
behavior locally within a file. All pragmas use the `cmakefmt:` prefix.

```
pragma     := "#" ws? "cmakefmt:" ws? action
action     := "off" | "on" | "skip"
            | "push" ws toml-inline-table
            | "pop"

ws         := [ \t]+
```

Where `toml-inline-table` follows [TOML inline table syntax](https://toml.io/en/v1.1.0#inline-table) with one relaxation: trailing commas are permitted. Values inside the table are TOML scalars (integer, boolean, double-quoted string) or TOML inline arrays.

**Rules:**

- One pragma per line. No code on the same line.
- The prefix `cmakefmt:` is case-sensitive, lowercase.
- Whitespace between `#` and `cmakefmt:` is optional: `#cmakefmt: off` and `# cmakefmt: off` are both valid.
- `push` takes an inline TOML table: `push { lineWidth = 120 }`. Braces are always required; bare `push` without braces is forbidden. `push {}` creates an empty save-point. Values use TOML scalar syntax: `80` (integer), `true`/`false` (boolean), `"lower"` (string). Arrays use TOML inline-array syntax (e.g., `push { spaceBeforeParen = ["if", "elseif"] }`). TOML inline tables are supported for table-typed options (e.g., `push { perCommandConfig = { set = { wrapStyle = "vertical" } } }`).
- Trailing content after a valid pragma is a warning and is ignored.

### 13.2 `off` / `on`

Disable formatting for a region. Every byte between `# cmakefmt: off` and `# cmakefmt: on`
is emitted exactly as read.

```cmake
# cmakefmt: off
set(MY_CAREFULLY_ALIGNED_MATRIX
  1 0 0 0
  0 1 0 0
  0 0 1 0
  0 0 0 1
)
# cmakefmt: on
```

- The off-region is **opaque and byte-preserved**: the formatter does not parse or execute pragmas inside it. The first `# cmakefmt: on` encountered ends the region.
- `off` without a matching `on` before EOF suppresses formatting for the rest of the file. This is valid, not a warning.
- `on` without a preceding `off` is a warning and is ignored.
- Formatting resumes immediately after `# cmakefmt: on`. Outside off-regions, all active formatting and normalization options apply.
- `off`/`on` does not interact with the `push`/`pop` stack. The configuration state is unchanged across an off-region:

```cmake
# cmakefmt: push { lineWidth = 120 }
# cmakefmt: off
...verbatim content...
# cmakefmt: on
# lineWidth is still 120
# cmakefmt: pop
```

### 13.3 `skip`

Suppress formatting for the **next command invocation** only. The command is emitted verbatim.

```cmake
# cmakefmt: skip
ExternalProject_Add(googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG        release-1.12.1
)
```

- Blank lines and comments between `skip` and the target command are allowed and preserved.
- `skip` before EOF with no subsequent command is a warning.
- `skip` does not accept inline overrides or interact with the `push`/`pop` stack.
- If `# cmakefmt: off` appears between a `skip` pragma and the next command, the off-region starts immediately and the pending `skip` is consumed (cancelled). After `# cmakefmt: on`, the `skip` does not persist.
- `push`/`pop` pragmas between `skip` and the target command are processed normally. `skip` only suppresses formatting of the next command invocation; it does not create an inert zone for other pragmas.

### 13.4 `push` / `pop`

#### 13.4.1 `push`

Create a new configuration frame on the stack. The frame inherits all values from the
current top, then applies any inline overrides.

```cmake
# cmakefmt: push { lineWidth = 120, alignPropertyValues = true }
set_target_properties(MyTarget PROPERTIES
  CXX_STANDARD              17
  CXX_STANDARD_REQUIRED     ON
  POSITION_INDEPENDENT_CODE ON
)
# cmakefmt: pop
```

`push {}` (empty table) creates a save-point with no changes.

When `push` includes `perCommandConfig`, the pushed table shallow-merges with the current effective `perCommandConfig` at the command-key level (same semantics as `extends` §15.2): top-level command keys in the pushed table override the existing entry for that command entirely. Command entries not present in the pushed table are preserved from the current frame. Step 2 of the resolution order (§13.4.4) consults this effective merged `perCommandConfig`.

#### 13.4.2 `pop`

Discard the top frame, restoring the configuration beneath it. `pop` when the stack has
only the root frame is a warning and is ignored.

#### 13.4.3 Nesting

Frames nest arbitrarily. Each `pop` discards exactly one frame:

```cmake
# cmakefmt: push { lineWidth = 120 }         ← frame 1
  # cmakefmt: push { indentWidth = 4 }       ← frame 2
    # lineWidth = 120, indentWidth = 4
  # cmakefmt: pop                          ← discard frame 2
  # lineWidth = 120, indentWidth restored
# cmakefmt: pop                            ← discard frame 1
# All values restored to config-file state
```

#### 13.4.4 Resolution Order

When resolving an option's effective value for a given command:

1. **Push stack** — walk the stack from top to bottom. The first frame that explicitly sets the option wins.
2. **`perCommandConfig`** — if no frame in the stack sets the option, check the command-specific override table.
3. **Config file value** — the value from `.cmakefmt.toml`.
4. **Built-in default** — the hardcoded fallback.

A frame "explicitly sets" an option only if the option's key appeared in that frame's `push` pragma inline table. Options not mentioned in the pragma are transparent — the frame does not override them.

A `push` override always takes priority over `perCommandConfig`.

### 13.5 Pragma-Settable Options

The options settable via `push` include all formatting options listed in the Summary Table. Only `disableFormatting`,
`extends`, and `ignorePatterns` cannot be set in a pragma — these control configuration
infrastructure or file-level routing, not formatting behavior. Setting any of these produces a warning and is
ignored. Note that `push` has broader scope than `perCommandConfig` (§11.1): it can also
set file-level options such as `maxBlankLines`, `lineEnding`, `finalNewline`,
`trimTrailingWhitespace`, `collapseSpaces`, `endCommandArgs`, and `indentBlockBody`.
Changing `indentBlockBody` via `push` affects only blocks opened after the push, not the currently enclosing block.
`ignoreCommands` is settable via `push`, enabling local command suppression within a file
region. These file-level and flow-control options are excluded from `perCommandConfig`
because they apply to document structure or block boundaries, not to individual command
invocations.

### 13.6 Diagnostics

All pragma diagnostics are **warnings** (never errors) and include file path and line
number. The formatter never fails due to a malformed pragma.

| Condition                            | Behavior                                |
| ------------------------------------ | --------------------------------------- |
| Malformed syntax                     | Entire pragma ignored                   |
| Unknown option name                  | That assignment skipped, others applied |
| Type mismatch or out-of-range value  | That assignment skipped, others applied |
| Non-settable option                  | That assignment skipped, others applied |
| `on` without preceding `off`         | Ignored                                 |
| `pop` without matching `push`        | Ignored                                 |
| `skip` at EOF (no following command) | Ignored                                 |
| Unmatched `push` at EOF              | Implicitly popped                       |
| Duplicate key in one pragma          | Last value wins (warning)               |
