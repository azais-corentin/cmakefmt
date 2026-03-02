**Indentation & Whitespace**
- Configurable indent width (spaces or tabs, default 2 spaces)
- Proper indentation of block bodies
- Trailing whitespace removal
- Consistent end-of-file newline
- Blank line normalization (e.g., collapse multiple blank lines into a single line)

**Command Formatting**
- Configurable casing for command names (default is lowercase)
- Consistent spacing inside parentheses — e.g., `command(args)` vs `command( args )` (default is no spaces)
- Alignment/wrapping of long argument lists with configurable line length limit
- Smart line-breaking strategies: keyword-based (e.g., `PUBLIC`, `PRIVATE` on separate lines), then fit as many arguments as possible per line
- Configurable dangling parenthesis style (closing paren on its own line or not)

**Keyword & Argument Handling**
- Configurable casing for keywords (default is uppercase: `PUBLIC`, `PRIVATE`, `INTERFACE`, `TARGETS`, `DESTINATION`, etc.)
- Keyword-aware line breaking — start a new line before recognized keywords (e.g., `target_link_libraries` with `PUBLIC`/`PRIVATE` on separate lines)
- Configurable sorting of arguments where order doesn't matter (e.g., source file lists, dependencies)
- Grouping related arguments together

**Comment Handling**
- Preserve inline comments (`# ...`) and keep them aligned
- Preserve block/bracket comments
- Option to reflow long comments to fit within line length
- Don't format inside comments

**String & Expression Handling**
- Preserve content inside quoted strings and bracket arguments (`[=[ ... ]=]`)
- Don't reformat generator expressions (`$<...>`)
- Handle nested parentheses and variable references (`${...}`) correctly

**Configurability**
- Config file support (`.cmake-format.yaml`, `.cmake-format.json`, or similar)
- Per-command formatting overrides (e.g., `set()` stays on one line, `install()` uses keyword grouping)
- Disable/enable formatting regions via magic comments (`# cmake-format: off` / `# cmake-format: on`)
- Configurable line length (e.g., 80, 100, 120)

**Robustness & Integration**
- Idempotency — running the formatter twice produces the same output
- Stdin/stdout mode for editor and CI integration
- In-place file editing with optional backup
- Diff/check mode (exit non-zero if file would change, for CI)
- Glob/recursive directory formatting
- Graceful handling of syntax errors — don't destroy malformed files
- Preserve semantic correctness (never change meaning of the CMake code)

**Nice-to-Haves**
- EditorConfig support
- Pre-commit hook integration
- LSP or editor plugin support (VS Code, Neovim, etc.)
- `cmake_minimum_required` version-aware formatting (style differences across CMake eras)
- Canonical spacing around `AND`, `OR`, `NOT` in conditionals
- Auto-insertion of missing `endfunction()`, `endif()` labels (or removal of legacy labels)

A good reference point is `cmake-format` (from `cmakelang`) — studying its config options and known limitations can help you decide where to innovate.