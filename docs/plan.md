# Implementation Plan

## Major Modules/Components

### 1. Configuration (`config`)
**Responsibility:** Define, parse, validate, merge, and resolve all 44 config options.

Sub-components:
- **`Config` struct:** All 44 fields with types and defaults matching Appendix A
- **TOML parsing:** Deserialize `.cmakefmt.toml` / `cmakefmt.toml` via serde
- **Config discovery:** Walk-up from file directory, dotfile wins over non-dot, stdin uses `--assume-filename`
- **`extends` resolution:** Relative path resolution, circular detection, max depth 32, merge strategy (scalars override, arrays replace, `perCommandConfig` shallow-merges at command-key level)
- **Validation:** Range checks (lineWidth 40–320, indentWidth 1–8, etc.), unknown key warnings
- **`$schema` handling:** Stored but ignored
- **Resolved config:** `continuationIndentWidth`/`genexIndentWidth`/`commentWidth` null → inherit from parent option
- **Dual-type parsing:** `sortArguments` (bool|string[]) and `spaceBeforeParen` (bool|string[]) require custom serde deserialization

**Spec:** §15 (config meta), README.md (discovery rules), Appendix A (defaults), Appendix B (example)
**Tests:** `tests/formatter/15_config_meta/` (schema + extends), plus implicit coverage through fixture-local `.cmakefmt.toml` files across formatter tests

### 2. Lexer/Tokenizer (`lexer`)
**Responsibility:** Tokenize CMake source into a stream of typed tokens using `logos`.

Token types needed:
- Command names (identifiers before `(`)
- Parentheses `(` / `)`
- Arguments (unquoted, quoted `"..."`, bracket `[==[...]==]`)
- Comments: line comments `# ...`, bracket comments `#[==[...]==]`
- Whitespace: spaces, tabs, newlines (CR, LF, CRLF)
- Generator expressions: `$<`, `>`, `:`, `;`, `,` (within genex context)
- Pragma comments: `# cmakefmt:` prefix detection (with and without space after `#`)
- BOM: UTF-8 BOM (`\xEF\xBB\xBF`) detection

**Spec:** Implicit from all sections; §6.5 (bracket args/comments verbatim), §7.1 (line ending detection, bare CR handling), §13 (pragma comment syntax)
**Tests:** Implicitly tested through all formatter fixtures

### 3. Parser / CST Builder (`parser`)
**Responsibility:** Build a Concrete Syntax Tree from the token stream. Must preserve all whitespace, comments, and formatting for round-trip fidelity.

CST node types:
- **File:** root node containing top-level statements
- **Command invocation:** command name + `(` + argument list + `)`
- **Argument list:** sequence of arguments, keywords, comments, whitespace
- **Argument:** unquoted, quoted, bracket, generator expression
- **Generator expression:** `$<` + content (colon-separated, nested genexes) + `>`
- **Comment:** line comment or bracket comment, standalone or trailing
- **Blank line:** whitespace-only line between statements
- **Block structure:** if/else/elseif/endif, foreach/endforeach, while/endwhile, function/endfunction, macro/endmacro, block/endblock — nesting tracked

**Spec:** §14 (flow control blocks), §6.5 (verbatim content), §10 (generator expressions)
**Tests:** Implicitly tested through all formatter fixtures

### 4. Keyword Dictionary (`keywords`)
**Responsibility:** Encode the normative Appendix F model: command classes, per-command keyword sections, sortable vs non-sortable structures, canonical section orders, block closers, and keyword-vs-literal precedence.

Data:
- Command classes from Appendix F §F.1: condition-syntax commands, simple commands, and the exhaustive keyword-structured command families
- Per-command section metadata from §F.2: recognized section keywords, which sections are simple-value sortable under `sortArguments`, and canonical section orders for `sortKeywordSections`
- Nested non-sortable structures from §F.3, such as `FILE_SET ... BASE_DIRS ... FILES ...`, `install(...)` destination blocks, and alternating `PROPERTIES <key> <value>` pairs
- Condition-syntax commands from §F.1.1 plus the explicit block-closing commands in §F.4, together with the pairing metadata needed by `endCommandArgs`
- Literal constants used by `literalCase`, together with Appendix F §F.5 overlap precedence for tokens like `TARGET`, `COMMAND`, `POLICY`, and `TEST`
**Spec:** §4 (casing), §12 (sorting), §14 (flow control), Appendix F (keyword dictionary)
**Tests:** `tests/formatter/04_casing/`, `tests/formatter/12_sorting/`, `tests/formatter/14_flow_control/`

### 5. Pragma Engine (`pragmas`)
**Responsibility:** Parse and execute inline pragma directives.

Directives:
- `# cmakefmt: off` / `# cmakefmt: on` — byte-preserve regions
- `# cmakefmt: skip` — verbatim next command
- `# cmakefmt: push { ... }` / `# cmakefmt: pop` — config stack

Rules:
- One pragma per line, case-sensitive prefix `# cmakefmt:`
- Also recognized: `#cmakefmt:` (no space after `#`)
- off/on: opaque region (no nested pragma parsing), unmatched off → suppress to EOF
- skip: blank/comment lines between skip and target preserved; cancelled by off before target
- push/pop: arbitrary nesting, inherits current frame, TOML inline table syntax (trailing commas allowed, bare push without braces forbidden)
- Unmatched push at EOF implicitly popped (warning)
- Resolution order: push stack → perCommandConfig → config file → built-in default
- Push can set file-level options beyond perCommandConfig scope (maxBlankLines, lineEnding, finalNewline, trimTrailingWhitespace, collapseSpaces, endCommandArgs, indentBlockBody, ignoreCommands)
- Only `disableFormatting`, `extends`, `$schema`, `ignorePatterns` are NOT settable via push
- `indentBlockBody` via push affects only blocks opened after push

**Spec:** §13 (inline pragmas)
**Tests:** `tests/formatter/13_pragmas/` (3 subdirs, 23 pairs)

### 6. Per-Command Config Resolution (`per_command`)
**Responsibility:** Resolve effective config for each command invocation.

Resolution chain: push stack → perCommandConfig[command_name] → config file → built-in default
- Command name matching is case-insensitive
- 27 overridable options via perCommandConfig (wrapping §1, indentation §2, casing §4, parens/spacing §5, comments §6, alignment §9, genex §10, sorting §12)
- Excluded from perCommandConfig: file-level concerns (blank lines §3, line endings §7, whitespace §8, flow control §14, suppression §16, config meta §15)
- Push-stack can set additional options beyond perCommandConfig scope

**Spec:** §11 (per-command config), §13.4 (push/pop interaction)
**Tests:** `tests/formatter/11_per_command/` (9 pairs), `tests/formatter/13_pragmas/03_push_pop/`

### 7. Formatting Pipeline (`formatter`)
**Responsibility:** Core formatting engine. Takes CST + resolved config → formatted output.

This is the largest module, orchestrating all formatting passes in the correct order.

#### Pipeline Order (refines normative Appendix E.1 and is cross-checked against Appendix E.2):
1. **Suppression gates:** `disableFormatting`, ignored files, ignored commands, and pragma-defined verbatim regions short-circuit formatting where applicable
2. **BOM handling:** Strip a UTF-8 BOM only when present at byte offset 0 and formatting is active
3. **Line ending detection:** Count LF vs CRLF for `lineEnding = "auto"`; LF wins ties; files with no line endings default to LF; bare CR is not counted
4. **Active-config resolution:** Scan pragmas (`off`/`on`, `skip`, `push`/`pop`) and resolve the effective config for each command (`push` stack → `perCommandConfig` → file config → defaults)
5. **Sorting:** Apply `sortArguments`, then `sortKeywordSections`
6. **Casing:** Apply `commandCase`, `keywordCase`, `literalCase`, and `customKeywords`
7. **Whitespace normalization:** Apply `collapseSpaces` to non-verbatim content before any alignment padding is inserted
8. **Wrapping/layout:** Apply Appendix C cascade/vertical layout, including genex wrapping rules
   - Step 0 pre-checks (`wrapArgThreshold`, `magicTrailingNewline`)
   - Step 1 single-line attempt
   - Step 2 keyword-group breaking for cascade mode
   - Step 3 one-per-line expansion
9. **Indentation:** Apply `indentWidth`, `indentStyle`, `continuationIndentWidth`, and `genexIndentWidth`
10. **Flow-control shaping:** Apply `indentBlockBody`, then `endCommandArgs`
11. **Parenthesis spacing/newlines:** Apply `closingParenNewline`, `spaceBeforeParen`, `spaceInsideParen`
12. **Blank-line policies:** Apply `minBlankLinesBetweenBlocks`, `blankLineBetweenSections`, and `maxBlankLines`
13. **Alignment:** Apply `alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`, and `alignTrailingComments`
14. **Comment formatting:** Apply `commentPreservation`, `commentWidth`, and `commentGap`
15. **Trailing whitespace:** Apply `trimTrailingWhitespace`
16. **Line endings:** Emit the configured line-ending sequence
17. **Final newline:** Enforce `finalNewline`

**Note on Appendix E:** Appendix E.1 is the authoritative user-visible order. The pragma scan in step 4 is an implementation refinement of Appendix E.1's suppression/config-resolution stages, not an alternative pipeline.

**Note on `commentGap`:** When `alignTrailingComments = true`, `commentGap` supplies the minimum gap during step 13. When trailing-comment alignment is off, `commentGap` applies as ordinary comment spacing in step 14.

**Note on dprint-core:** Steps 8–14 are likely to be expressed through dprint-core print items rather than as rigid, isolated passes. The implementation still must preserve the Appendix E.1 ordering semantics above.

**Spec:** All sections (§1–§16), Appendix C (cascade algorithm), Appendix E (interactions)
**Tests:** All `tests/formatter/` directories (329 fixture pairs total)

### 8. Suppression (`suppression`)
**Responsibility:** Enforce byte-preserving escape hatches at file, region, and command granularity.
- `disableFormatting=true` → output = input byte-for-byte; overrides pragmas and every normalization/formatting pass, including BOM stripping
- `ignorePatterns` → skip files via gitignore-style globs, resolved relative to the config file that defined each pattern (including inherited patterns via `extends`)
- `ignoreCommands` → preserve matched invocations verbatim (case-insensitive) while surrounding commands still format normally
- Pragma off/on → byte-preserve explicit regions within a file
- Pragma skip → preserve the next command only
- `ignoreCommands` takes precedence over `perCommandConfig`
- Ignored commands are excluded from sorting/alignment and other command-local formatting

**Spec:** §16 (suppression), §13 (pragmas)
**Tests:** `tests/formatter/16_suppression/` (14 pairs, including inherited `ignorePatterns` cases), `tests/formatter/13_pragmas/01_off_on/`, `tests/formatter/13_pragmas/02_skip/`

### 9. CLI (`cli`)
**Responsibility:** Command-line interface using `clap`.

Modes: format (default), `--check`, `--diff`, `--stdin`, `--print-config`
Flags: `--write`/`--inplace` (aliases), `--config`, `--assume-filename`, `--color`/`--no-color`, `--verbose`, `--quiet`
Exit codes: 0 (success), 1 (changes found in --check), 2 (error)
Mutual exclusions: --check+--write (error), --diff+--write (error), --stdin+file args (error)
Allowed combinations: --check+--diff
Priority: --quiet wins over --verbose
Warning: --assume-filename without --stdin is warning (ignored)

**Spec:** Appendix D (CLI)
**Tests:** No fixture tests; needs separate CLI integration tests

### 10. dprint Plugin Interface (`plugin`)
**Responsibility:** Implement the dprint WASM plugin protocol via `dprint-core`.
- Expose `format_text(file_path, input, config) → Option<String>`
- Config resolution from dprint's JSON config format
- Plugin info (name, version, config schema)

**Spec:** Not explicitly specified (implementation detail)
**Tests:** No dedicated fixtures; the formatter fixtures test the core logic

### 11. Test Harness (`test_harness`)
**Responsibility:** Discover and run all fixture pairs, compare output, report diffs.
- Walk `tests/formatter/` recursively
- For each `.in.cmake` / `.out.cmake` pair:
  - Load `.cmakefmt.toml` from same directory (if present), walking up to find nearest
  - Format `.in.cmake` with config
  - Compare result to `.out.cmake` byte-for-byte
  - Report diff on mismatch (using `imara-diff`)
- Idempotency check: format the output again, verify identical

**Tests:** `tests/formatter_test.rs` (to be rewritten)

---

## Implementation Order

### Phase 1: Foundation (no dependencies)
These modules can be built in parallel.

#### Milestone 1: Configuration
- Define `Config` struct with all 44 fields, types, defaults
- TOML deserialization with serde (including dual-type fields: `sortArguments: bool|string[]`, `spaceBeforeParen: bool|string[]`)
- Validation (ranges, enum variants)
- `null` → inherited value resolution
- `extends` chain loading and merging
- Config discovery (walk-up algorithm)
- Unknown key warnings

**Spec:** §15, README.md, Appendix A, Appendix B
**Tests:** Unit tests for parsing, validation, merge, discovery

#### Milestone 2: Lexer
- Define token types for all CMake constructs
- Implement logos-based tokenizer
- BOM detection and stripping
- Line ending detection (LF vs CRLF counting, bare CR as ordinary character)
- Pragma comment prefix detection (both `# cmakefmt:` and `#cmakefmt:` variants)

**Spec:** Implicit from all sections; §7.1, §6.5, §13
**Tests:** Unit tests for token streams

#### Milestone 3: Keyword Dictionary
- Command classes from Appendix F §F.1: condition-syntax commands, simple commands, and the exhaustive keyword-structured families
- Per-command keyword-section tables from Appendix F §F.2, including sortable simple-value sections and canonical section orders where defined
- Nested non-sortable structures from Appendix F §F.3 (`FILE_SET ... FILES`, `install(...)` destination blocks, `PROPERTIES <key> <value>` pairs)
- Condition-syntax commands plus the explicit block-closing commands from Appendix F §F.4, with pairing metadata for `endCommandArgs`
- Literal constants list (ON, OFF, TRUE, FALSE, etc.)
- Context-sensitive keyword-vs-literal overlap precedence from Appendix F §F.5 (TARGET, COMMAND, POLICY, TEST, plus `customKeywords`)

**Spec:** §4, §12, §14, Appendix F
**Tests:** Unit tests for keyword lookup

### Phase 2: Parsing (depends on Phase 1)

#### Milestone 4: Parser / CST
- Build CST from token stream
- Track all whitespace, comments, blank lines
- Nesting structure for blocks (if/endif, etc.)
- Generator expression parsing (nested `$<...>`)
- Round-trip fidelity: CST → text should reproduce input exactly

**Spec:** §14, §6.5, §10
**Tests:** Round-trip tests (format with all-defaults should still parse correctly)
**Depends on:** Milestone 2 (lexer), Milestone 3 (keywords for block detection)

### Phase 3: Core Formatting (depends on Phase 2)

#### Milestone 5: Basic Formatting — Wrapping & Indentation
The heart of the formatter. Implement the cascade/vertical wrapping algorithm and indentation.
- Cascade algorithm: Step 0 pre-checks, Step 1 (single line), Step 2 (keyword breaks), Step 3 (one per line)
- Vertical variant (skip Step 2)
- `firstArgSameLine` behavior
- `wrapArgThreshold` forcing (counts ALL tokens in parens including keywords; genex = 1 token)
- `magicTrailingNewline` detection and behavior (cascade: skip Step 1 only, Step 2 still tried; vertical: skip to Step 3)
- `indentWidth`, `indentStyle` (space/tab), `continuationIndentWidth` (relative to KEYWORD, not command name)
- Tab + fractional continuation: tabs for whole multiples of indentWidth, spaces for remainder
- Block body indentation (`indentBlockBody`)
- `closingParenNewline`
- Single tokens exceeding lineWidth are never broken
- Deep nesting exceeding lineWidth via indentation alone is emitted as-is

**Spec:** §1 (wrapping), §2 (indentation), §5.1 (closingParenNewline), §14.1 (indentBlockBody), Appendix C (cascade detail)
**Tests:** `tests/formatter/01_wrapping/` (41 pairs), `tests/formatter/02_indentation/` (21 pairs), `tests/formatter/05_parens_spacing/01_closing_paren_newline/`, `tests/formatter/14_flow_control/01_indent_block_body/`
**Depends on:** Milestone 4 (parser)

#### Milestone 6: Casing & Simple Transforms
- `commandCase`, `keywordCase`, `literalCase`
- `customKeywords` classification: affects section detection, layout, and sorting globally, not only casing
- `customKeywords` take precedence over `literalCase` when a token could be either
- `spaceBeforeParen` (bool for all commands, or string[] for selective), `spaceInsideParen`
- `endCommandArgs` (remove/preserve/match)
  - match for `else()` copies the nearest unmatched enclosing `if()` condition in syntactic nesting order
  - match for `block()/endblock()` always produces empty `endblock()`
  - `elseif()` is unaffected because its condition is definitional, not repeated
- `commentGap`
- Empty commands always single-line (§14.3)

**Spec:** §4 (casing), §5.2–5.3 (paren spacing), §14.2 (endCommandArgs), §14.3 (empty commands), §6.4 (commentGap)
**Tests:** `tests/formatter/04_casing/` (20 pairs), `tests/formatter/05_parens_spacing/` (14 pairs), `tests/formatter/14_flow_control/02_end_command_args/`, `tests/formatter/14_flow_control/03_empty_commands/`, `tests/formatter/06_comments/04_comment_gap/`
**Depends on:** Milestone 4 (parser), Milestone 3 (keywords)

### Phase 4: File-Level Formatting (depends on Phase 3)

#### Milestone 7: Blank Lines, Whitespace, Line Endings
- `maxBlankLines` (collapse, leading strip unconditional, trailing limit, args discard)
- `minBlankLinesBetweenBlocks` (insert before block-openers at any nesting level, attached comments via backward scan, precedence over maxBlankLines at block boundaries, not before closers, not when first statement)
- `blankLineBetweenSections` (insert even between zero-arg sections, precedence over maxBlankLines in arg lists)
- `trimTrailingWhitespace`
- `collapseSpaces` (before alignment, exempt alignment padding)
- `lineEnding` (auto detection by frequency, LF wins ties, files with no line endings default to LF; bare CR is ordinary content)
- BOM stripping (only a UTF-8 BOM at byte offset 0; interior U+FEFF is preserved as content)
- `finalNewline` (true => exactly one trailing newline; false => do not add a missing newline, do not strip existing ones, and still honor EOF blank-line limits)
- Empty-file and whitespace-only EOF behavior under `finalNewline` true/false
- Line endings inside quoted strings/bracket args NOT normalized

**Spec:** §3 (blank lines), §7 (line endings), §8 (whitespace)
**Tests:** `tests/formatter/03_blank_lines/` (21 pairs), `tests/formatter/07_line_endings/` (17 pairs), `tests/formatter/08_whitespace/` (8 pairs)
**Depends on:** Milestone 5 (wrapping determines line layout)

### Phase 5: Advanced Features (depends on Phases 3–4)

These milestones can be built in parallel since they don't depend on each other (except M10 which also depends on M7 from Phase 4).

#### Milestone 8: Generator Expressions
- Genex wrapping: cascade (colon split, semicolons, commas) vs never
- `genexClosingAngleNewline`
- `genexIndentWidth` (relative to `$<` column, not line start)
- Recursive nesting
- `genexWrap = "never"` makes `genexClosingAngleNewline` and `genexIndentWidth` inert (E6, E7)

**Spec:** §10 (generator expressions), §2.4 (genexIndentWidth)
**Tests:** `tests/formatter/10_genex/` (10 pairs), `tests/formatter/02_indentation/04_genex_indent/`
**Depends on:** Milestone 5 (integrates with wrapping algorithm)

#### Milestone 9: Comment Formatting
- Comment preservation: re-indentation, standalone handling, collapse prevention
- Comment reflow: paragraph detection, code block preservation (4+ space indent), fenced blocks (triple-backtick), list items (-, *, +, digit.), unclosed fences
- `commentWidth` (null → lineWidth)
- `alignTrailingComments` (consecutive lines, group broken by blank line)
- `commentGap` (minimum gap for alignment when alignTrailingComments=true, simple spacing otherwise)
- Verbatim content: bracket comments, bracket args, multi-line strings
- Pragma comments never reflowed
- Known limitation: nested list markers treated as continuation lines of parent item

**Spec:** §6 (comments)
**Tests:** `tests/formatter/06_comments/` (5 subdirs, 34 pairs)
**Depends on:** Milestone 5 (needs line layout for reflow width calculations)

#### Milestone 10: Alignment
- `alignPropertyValues`: column-align PROPERTIES values in `set_target_properties`
- `alignConsecutiveSet`: column-align consecutive `set()` calls (valueless/keyword-only set() skipped but don't break group)
- `alignArgGroups`: detect repeating token-count patterns, column-align (min 2 lines, keyword-as-first-token aligned by keyword column, no cross-keyword-boundary alignment)
- `alignTrailingComments`: column-align trailing comments across consecutive lines (group broken by blank line; uses `commentGap` as minimum gap)
- Alignment must run after wrapping (needs final layout)
- `collapseSpaces` exemption for alignment padding
- Lines that would exceed lineWidth excluded from alignment
- Blank/comment line breaks alignment group

**Spec:** §9 (alignment), §8.2 (collapseSpaces exemption)
**Tests:** `tests/formatter/09_alignment/` (3 subdirs, 24 pairs)
**Depends on:** Milestone 5 (wrapping), Milestone 7 (collapseSpaces interaction)

#### Milestone 11: Sorting
- `sortArguments`: case-insensitive, stable, and limited to Appendix F sortable sections plus custom-keyword sections that contain simple values
- Attached comments travel with arguments
- Unattached comments and blank lines create sorting group boundaries
- Selective keyword sorting (string[] variant)
- Generator expressions and variable references sorted by literal source text
- Nested recognized structures are never value-sorted
- Commands without recognized keyword sections unaffected
- `sortKeywordSections`: reorder only commands with canonical Appendix F section orders
- Comments attached to a section travel with it; positional args before the first keyword remain fixed
- Sorting runs before later layout/alignment passes per Appendix E.1

**Spec:** §12 (sorting), Appendix F (canonical orders)
**Tests:** `tests/formatter/12_sorting/` (2 subdirs, 27 pairs)
**Depends on:** Milestone 3 (keyword dictionary), Milestone 4 (parser)

### Phase 6: Pragma & Per-Command (depends on Phase 1; sequenced here for integration clarity)

#### Milestone 12: Pragma Engine
- Parse `# cmakefmt:` directives from comment tokens (both space and no-space variants)
- off/on region tracking (opaque, no nested parsing)
- skip directive (next-command targeting, blank/comment gap handling)
- push/pop config stack (TOML inline table parsing, trailing commas allowed, nesting, merge semantics)
- Warning diagnostics (unmatched on, skip at EOF, pop without push, unknown push keys, non-settable push keys)
- Unmatched push at EOF: implicitly popped with warning

**Spec:** §13 (inline pragmas)
**Tests:** `tests/formatter/13_pragmas/` (3 subdirs, 23 pairs)
**Depends on:** Milestone 1 (config struct for push/pop)

#### Milestone 13: Per-Command Config Resolution
- Resolve effective config per command invocation
- Resolution chain: push stack → perCommandConfig → config file → default
- Case-insensitive command name matching
- Overridable vs excluded option sets (27 overridable via perCommandConfig, broader set via push)
- Push stack shallow-merge with perCommandConfig at command-key level

**Spec:** §11 (per-command config), §13.4 (push/pop interaction)
**Tests:** `tests/formatter/11_per_command/` (9 pairs), `tests/formatter/13_pragmas/03_push_pop/push_per_command_merge/`
**Depends on:** Milestone 1 (config), Milestone 12 (pragma engine)

### Phase 7: Suppression (depends on Phase 6)

#### Milestone 14: Suppression
- `disableFormatting` passthrough (byte-for-byte; BOM preserved; overrides pragmas and all normalization)
- `ignorePatterns` glob matching (gitignore-style, resolved relative to the config file that declared each pattern, including inherited patterns via `extends`)
- `ignoreCommands` per-command verbatim preservation (case-insensitive)
- Ignored commands preserve only matching invocations; surrounding commands still format normally
- Integration with pragma off/on and skip
- `ignoreCommands` precedence over `perCommandConfig` (Appendix E.2)
- `ignoreCommands` suppresses sorting/alignment and other command-local formatting

**Spec:** §16 (suppression)
**Tests:** `tests/formatter/16_suppression/` (14 pairs)
**Depends on:** Milestone 12 (pragma regions), Milestone 13 (per-command resolution)

### Phase 8: Integration (depends on all above)

#### Milestone 15: Cross-Feature Interactions
- Validate Appendix E.1 as the authoritative global pipeline order
- Cross-check Appendix E.2 pairwise consequences against the refined formatter pipeline described below
- Cover edge cases where suppression, sorting, wrapping, blank lines, alignment, comments, and config overrides intersect

**Spec:** Appendix E (interactions)
**Tests:** `tests/formatter/17_interactions/` (20 pairs)
**Depends on:** All previous milestones

#### Milestone 16: Test Harness
- Rewrite `tests/formatter_test.rs`
- Recursive fixture discovery
- Config loading (`.cmakefmt.toml` walk-up + pragma inline)
- Byte-exact comparison with diff reporting
- Idempotency verification (format output again, verify identical)

**Depends on:** Milestones 5–14 (needs a substantially working formatter)

#### Milestone 17: CLI
- clap argument parsing
- File discovery and glob expansion
- --check, --diff, --stdin, --write modes
- --config, --assume-filename
- --print-config
- Exit codes (0/1/2)
- Color and verbosity control (--quiet wins over --verbose)

**Spec:** Appendix D (CLI)
**Depends on:** Milestone 1 (config), Milestone 16 (test harness for integration tests)

#### Milestone 18: dprint Plugin Interface
- WASM plugin protocol via dprint-core
- `format_text` implementation
- Config schema exposure
- Plugin metadata

**Depends on:** Milestones 5–15 (all formatting milestones)

---

## Pipeline Order Validation Against Appendix E

Appendix E.1 defines the normative total order. The table below maps the §E.2 pairwise rules onto the refined 17-step pipeline in the formatter section:

| # | Rule (Options) | Pipeline Step(s) | Satisfied |
|---|---|---|---|
| 1 | `sortArguments` + `alignArgGroups` | sort=5, align=13 | Yes: 5 < 13 |
| 2 | `indentStyle="tab"` + alignment | indent=9, align=13 | Yes: tabs for indentation, spaces for alignment padding |
| 3 | `disableFormatting` + all | suppression=1 | Yes: step 1 short-circuits |
| 4 | `wrapArgThreshold` + `magicTrailingNewline` | wrap=8 (Step 0) | Yes: threshold wins per Appendix C |
| 5 | `wrapStyle="vertical"` + `wrapArgThreshold` | wrap=8 | Yes: both force Step 3 output |
| 6 | `genexWrap="never"` + `genexClosingAngleNewline` | wrap=8 | Yes: inline genex leaves closing-angle rule inert |
| 7 | `genexWrap="never"` + `genexIndentWidth` | wrap=8 | Yes: inline genex leaves indent rule inert |
| 8 | `commentPreservation="preserve"` + `commentWidth` | comment=14 | Yes: width matters only during reflow |
| 9 | `firstArgSameLine` + single-line | wrap=8 | Yes: no-op on single-line output |
| 10 | `closingParenNewline` + single-line | wrap=8, parens=11 | Yes: no-op on single-line output |
| 11 | `blankLineBetweenSections` + single-section | blank=12 | Yes: no extra blank line inserted |
| 12 | `alignPropertyValues` + single-line | align=13 | Yes: no-op on single-line property commands |
| 13 | `perCommandConfig` + `push` pragma | resolve=4 | Yes: push overrides per-command config |
| 14 | `ignoreCommands` + `perCommandConfig` | suppression=1, resolve=4 | Yes: ignored commands bypass command-local config effects |
| 15 | `ignoreCommands` + pragmas | suppression=1, resolve=4 | Yes: either suppression mechanism is sufficient; no conflict |
| 16 | `finalNewline` + `maxBlankLines` | blank=12, final=17 | Yes: EOF blank-line limits apply before final-newline enforcement |
| 17 | `ignoreCommands` + sorting/alignment | suppression=1, sort=5, align=13 | Yes: ignored commands remain verbatim |
| 18 | `blankLineBetweenSections` + `maxBlankLines` | blank=12 | Yes: inserted section separators win inside argument lists |
| 19 | `alignTrailingComments` + `commentGap` | align=13 | Yes: `commentGap` supplies the minimum alignment gap |
| 20 | `collapseSpaces` + alignment | collapse=7, align=13 | Yes: collapse happens first; alignment padding is exempt |
| 21 | `alignArgGroups` + `blankLineBetweenSections` | blank=12, align=13 | Yes: inserted blank lines become alignment group boundaries |
| 22 | `maxBlankLines` + `sortArguments` | sort=5, blank=12 | Yes: sort-group boundaries are preserved through blank-line handling |
| 23 | `sortKeywordSections` + `blankLineBetweenSections` | sort=5, blank=12 | Yes: reorder sections before inserting separators |

**Result:** All 23 pairwise rules are satisfied by a refinement of the normative Appendix E.1 pipeline, not by a competing inferred order.
---

## Test Fixture Inventory

Total: **329 test pairs** across 17 directories.

| Directory | Spec Sections | Pairs | Description |
|---|---|---|---|
| `01_wrapping/` | §1, Appendix C | 41 | line_width, cascade steps 1-3, vertical, firstArgSameLine, wrapArgThreshold, magicTrailingNewline |
| `02_indentation/` | §2 | 21 | indent_width, indent_style (space/tab/mixed), continuationIndent, genexIndent |
| `03_blank_lines/` | §3 | 21 | maxBlankLines, minBlankLinesBetweenBlocks, blankLineBetweenSections |
| `04_casing/` | §4 | 20 | commandCase, keywordCase, customKeywords driving section detection/layout/sorting, literalCase, keyword precedence over literalCase |
| `05_parens_spacing/` | §5 | 14 | closingParenNewline, spaceBeforeParen, spaceInsideParen |
| `06_comments/` | §6 | 34 | preservation, reflow, trailing alignment, commentGap, verbatim |
| `07_line_endings/` | §7 | 17 | lf/crlf/auto detection, bare CR ignored for auto-detect, byte-0 BOM stripping only, interior U+FEFF preserved, finalNewline true/false edge cases |
| `08_whitespace/` | §8 | 8 | trimTrailingWhitespace, collapseSpaces |
| `09_alignment/` | §9 | 24 | propertyValues, consecutiveSet, argGroups |
| `10_genex/` | §10 | 10 | genex wrap cascade/never, closingAngle, nesting |
| `11_per_command/` | §11 | 9 | override wrap/space/sorting/alignment/width, push merge, case-insensitive match |
| `12_sorting/` | §12 | 27 | sortArguments (stable ordering, comment attachment/boundaries, literal-text genex/vars, custom keyword sections, nested non-sortable structures), sortKeywordSections |
| `13_pragmas/` | §13 | 23 | off/on, skip, push/pop |
| `14_flow_control/` | §14 | 20 | indentBlockBody; endCommandArgs remove/preserve/match with nearest-unmatched `else()` behavior, nested/mixed blocks, `block()/endblock()`, empty commands |
| `15_config_meta/` | §15 | 6 | `$schema` ignored, `extends` scalar/array/perCommandConfig merge semantics, relative-path resolution |
| `16_suppression/` | §16 | 14 | disableFormatting overrides pragmas and normalization, ignoreCommands case-insensitive with surrounding formatting preserved, ignorePatterns including inherited relative-path behavior |
| `17_interactions/` | Appendix E | 20 | Appendix E.1 global-order checks plus Appendix E.2 pairwise interaction fixtures |

### Config Override Mechanisms in Tests
- **`.cmakefmt.toml` files:** Present in multiple test subdirectories
- **Inline pragmas:** `# cmakefmt: push { ... }` / `# cmakefmt: pop` used heavily in most tests

---

## Ambiguities and Gaps in the Spec

The refreshed spec suite resolves the older plan-level uncertainties around Appendix E ordering, Appendix F authority, `endCommandArgs = "match"` nesting, pragma spacing variants, and `finalNewline = false`. Remaining work is mostly implementation-risk management rather than spec gap-filling.

### Remaining Implementation Risks
1. **`collapseSpaces` timing still needs a concrete normalization boundary.** Appendix E.1/E.2 now specifies that collapsing happens before alignment and that alignment padding is exempt; the implementation still needs a clean place to enforce that.
2. **`perCommandConfig` excluded options should become an explicit allowlist.** The spec describes the category boundary and the broader `push` scope, but the code should materialize an obvious option set rather than re-derive it ad hoc.
3. **`spaceInsideParen = "preserve"` on collapse-to-single-line needs a precise trigger.** The fallback-to-remove rule applies when the formatter actively collapses a multi-line command, not when the input was already single-line.
4. **Dual-type config parsing needs careful diagnostics.** `sortArguments` (bool|string[]) and `spaceBeforeParen` (bool|string[]) require custom serde paths with clear error messages.
5. **Cascade Step 2 escalation is per keyword group.** Appendix C allows one group to remain at Step 2 while another escalates to Step 3 inside the same command.
6. **Indentation widths are column-relative, not line-relative.** `continuationIndentWidth` is relative to the keyword column and `genexIndentWidth` is relative to the `$<` column.
7. **`wrapArgThreshold` counts every token inside the parentheses.** Keywords and the first positional argument count; a full generator expression counts as one token.
8. **Blank-line insertion rules need context-aware scanning.** Leading file blank lines are always stripped, and `minBlankLinesBetweenBlocks` requires a backward scan to keep attached comments with the following block opener.
9. **Keyword/literal overlap remains context-sensitive in implementation.** Tokens like `TARGET`, `COMMAND`, `POLICY`, and `TEST` must follow Appendix F slot classification before falling back to `literalCase`.
10. **Tabs with fractional continuation widths need mixed indentation output.** Under `indentStyle = "tab"`, whole multiples use tabs and the remainder uses spaces.
---

## Dependency Graph (simplified)

```
Phase 1 (parallel):   Config ─┐    Lexer ─┐    Keywords ─┐
                               │           │              │
Phase 2:                       └───> Parser/CST <─────────┘
                                        │
Phase 3 (parallel):        Wrapping+Indent ←─┤    Casing+Transforms
                                │             │         │
Phase 4:              BlankLines+Whitespace+LineEndings  │
                                │                        │
Phase 5 (parallel):   Genex  Comments  Alignment  Sorting
                        │       │         │          │
Phase 6:                    Pragmas ──> PerCommandConfig
                                            │
Phase 7:                              Suppression
                                            │
Phase 8:                Interactions + TestHarness + CLI + dprint Plugin
```

### Parallelization Notes
- **Phase 1:** M1, M2, M3 are fully independent.
- **Phase 3:** M5 and M6 are independent (wrapping doesn't need casing; casing doesn't need wrapping). Both need parser (M4).
- **Phase 5:** M8 (genex), M9 (comments), M11 (sorting) can run in parallel. M10 (alignment) depends on M7 from Phase 4.
- **Phase 6:** M12 (pragmas) depends only on M1. Can start as early as Phase 2 in practice. M13 depends on M12.
- **Phase 8:** M16 (test harness) is an integration milestone. M17 (CLI) and M18 (plugin) depend on having a working formatter.

### Critical Path
Config → Lexer → Parser → Wrapping → BlankLines/Whitespace → Alignment → Integration
(Longest sequential chain: M1 → M2 → M4 → M5 → M7 → M10 → M15)

---

## Config Option Coverage Matrix

All 44 config options mapped to their implementing milestone:

| Option | Type | Default | Spec | Milestone |
|---|---|---|---|---|
| lineWidth | int (40–320) | 80 | §1.1 | M5 |
| wrapStyle | cascade\|vertical | cascade | §1.2 | M5 |
| firstArgSameLine | bool | true | §1.3 | M5 |
| wrapArgThreshold | int (0–999) | 0 | §1.4 | M5 |
| magicTrailingNewline | bool | true | §1.5 | M5 |
| indentWidth | int (1–8) | 2 | §2.1 | M5 |
| indentStyle | space\|tab | space | §2.2 | M5 |
| continuationIndentWidth | int\|null | null→indentWidth | §2.3 | M5 |
| genexIndentWidth | int\|null | null→indentWidth | §2.4 | M8 |
| maxBlankLines | int (0–100) | 1 | §3.1 | M7 |
| minBlankLinesBetweenBlocks | int (0–10) | 0 | §3.2 | M7 |
| blankLineBetweenSections | bool | false | §3.3 | M7 |
| commandCase | lower\|upper\|unchanged | lower | §4.1 | M6 |
| keywordCase | lower\|upper\|unchanged | upper | §4.2 | M6 |
| customKeywords | string[] | [] | §4.3 | M6 |
| literalCase | upper\|lower\|unchanged | unchanged | §4.4 | M6 |
| closingParenNewline | bool | true | §5.1 | M5 |
| spaceBeforeParen | bool\|string[] | false | §5.2 | M6 |
| spaceInsideParen | insert\|remove\|preserve | remove | §5.3 | M6 |
| commentPreservation | preserve\|reflow | preserve | §6.1 | M9 |
| commentWidth | int\|null | null→lineWidth | §6.2 | M9 |
| alignTrailingComments | bool | false | §6.3 | M9/M10 |
| commentGap | int (0–10) | 1 | §6.4 | M6/M9 |
| lineEnding | lf\|crlf\|auto | auto | §7.1 | M7 |
| finalNewline | bool | true | §7.2 | M7 |
| trimTrailingWhitespace | bool | true | §8.1 | M7 |
| collapseSpaces | bool | true | §8.2 | M7 |
| alignPropertyValues | bool | false | §9.1 | M10 |
| alignConsecutiveSet | bool | false | §9.2 | M10 |
| alignArgGroups | bool | false | §9.3 | M10 |
| genexWrap | cascade\|never | cascade | §10.1 | M8 |
| genexClosingAngleNewline | bool | true | §10.2 | M8 |
| perCommandConfig | table | {} | §11 | M13 |
| sortArguments | bool\|string[] | false | §12.1 | M11 |
| sortKeywordSections | bool | false | §12.2 | M11 |
| indentBlockBody | bool | true | §14.1 | M5 |
| endCommandArgs | remove\|preserve\|match | remove | §14.2 | M6 |
| $schema | string | (none) | §15.1 | M1 |
| extends | string | (none) | §15.2 | M1 |
| disableFormatting | bool | false | §16.1 | M14 |
| ignorePatterns | string[] | [] | §16.2 | M14 |
| ignoreCommands | string[] | [] | §16.3 | M14 |

Plus 2 implicit behaviors:
- **Config discovery rules** (README.md) → M1
- **BOM handling** (README.md) → M2/M7
