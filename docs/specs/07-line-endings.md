## 7 · Line Endings & Final Newline

### 7.1 `lineEnding`

|             |                            |
| ----------- | -------------------------- |
| **Type**    | `"lf" \| "crlf" \| "auto"` |
| **Default** | `"auto"`                   |

Controls which line-ending sequence is written to the output.

BOM handling scope: a UTF-8 BOM is stripped only when the three-byte BOM sequence appears at
byte offset 0 of the file input (unless formatting is suppressed per §16.1). U+FEFF characters
appearing anywhere else in the file are treated as ordinary content and preserved.

- **`"lf"`**: Unix-style `\n`.
- **`"crlf"`**: Windows-style `\r\n`.
- **`"auto"`**: Detect the dominant line ending in the input file and preserve it.
  "Dominant" means the line-ending sequence (`\n` vs `\r\n`) that occurs most frequently.
  On equal counts, `\n` (LF) wins as tiebreaker.
  If the file has no line endings (e.g., single-line file, empty file, or zero bytes), default to `"lf"`.

Bare carriage returns (`\r` not followed by `\n`) are treated as ordinary characters, not line endings. They are not counted in dominant-line-ending detection and are not normalized by the `lineEnding` option.

### 7.2 `finalNewline`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `true`    |

When `true`, ensure the file ends with exactly one trailing newline. Excess trailing
newlines are removed; a missing trailing newline is added. Empty or whitespace-only
files are normalized to a single newline character.
An empty file (zero bytes) is normalized to a single newline character when `finalNewline = true`.

For example, if the input file ends with `cmake_minimum_required(VERSION 3.20)` and no trailing newline, the formatter appends one. If the input ends with two trailing newlines, excess trailing newlines are removed, leaving exactly one.

When `false`, do not add a trailing newline if one is absent. `finalNewline = false` only controls
whether a *missing* trailing newline is added. Existing trailing newlines are not stripped by this
option — however, `maxBlankLines` (§3.1) still enforces its limit on consecutive blank lines at EOF
regardless of `finalNewline`.

When `finalNewline = false` and the input is an empty file (zero bytes), the output is also zero bytes. When `finalNewline = false` and the input is a whitespace-only file, the file is still subject to `trimTrailingWhitespace` and `maxBlankLines`, which may reduce it to empty.

```cmake
# Input (finalNewline = false, maxBlankLines = 1):
set(FOO "bar")



# Output: maxBlankLines collapses 3 blank lines to 1; no trailing newline added
set(FOO "bar")

```

POSIX convention and most editors expect a trailing newline, so the default is `true`.
