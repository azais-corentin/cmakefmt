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
**Responsibility:** Define which tokens are recognized as keywords for each command, canonical section orders, and condition-syntax commands.

Data:
- Per-command keyword lists (which args are structural keywords vs values)
- Canonical section orders for `sortKeywordSections` (PUBLIC→INTERFACE→PRIVATE, etc.) — 4 commands with explicit canonical orders (Appendix F)
- Condition-syntax commands (if, elseif, else, endif, while, endwhile — args are expressions, not keyword-value pairs)
- Literal constants list for `literalCase` (ON, OFF, TRUE, FALSE, AND, OR, NOT, COMMAND, POLICY, TARGET, TEST, DEFINED, EXISTS, IS_NEWER_THAN, etc.)
- Block-opening/closing command pairs (if↔endif, foreach↔endforeach, while↔endwhile, function↔endfunction, macro↔endmacro, block↔endblock)
- Simple commands (no keywords) — ~15 commands listed in Appendix F
- Context-sensitive tokens (TARGET, COMMAND, POLICY, TEST) that appear in both keyword dict and literal constants list

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

#### Pipeline Order (validated against all 23 Appendix E interaction rules):
1. **Suppression check:** `disableFormatting` → byte-for-byte passthrough (E3: absolute precedence)
2. **BOM handling:** Strip BOM (unless `disableFormatting`)
3. **Line ending detection:** Count LF vs CRLF for `auto` mode; LF wins ties; no-line-endings file → LF; bare CR not counted
4. **Pragma parsing:** Identify off/on regions, skip targets, push/pop stack (E15: pragmas + ignoreCommands don't conflict)
5. **Per-command config resolution:** Build effective config per command (E13: push > perCommandConfig; E14: ignoreCommands > perCommandConfig)
6. **Sorting:** `sortArguments` then `sortKeywordSections` (E1: sort before align; E23: sortSections before blankBetweenSections; E22: sort group boundaries override maxBlankLines)
7. **Casing:** Apply `commandCase`, `keywordCase`, `literalCase`, `customKeywords`
8. **Whitespace normalization:** `collapseSpaces` (E20: before alignment padding)
9. **Wrapping/layout:** The cascade/vertical algorithm (§1.2, Appendix C)
   - Step 0 pre-checks (E4: threshold wins over magic; E5: vertical+threshold → Step 3)
   - Step 1: try single line (E9: firstArgSameLine no-op; E10: closingParenNewline no-op; E11: blankBetweenSections no-op)
   - Step 2: keyword breaks (cascade only; E6-7: genexWrap=never → inline)
   - Step 3: one per line
   - Genex wrapping (§10) applied recursively within arguments
10. **Indentation:** Apply `indentWidth`, `indentStyle`, `continuationIndentWidth`, `genexIndentWidth` (E2: tabs for indent, spaces for alignment)
11. **Block body indentation:** `indentBlockBody` (§14.1)
12. **End command args:** `endCommandArgs` remove/preserve/match (§14.2)
13. **Parentheses spacing:** `closingParenNewline`, `spaceBeforeParen`, `spaceInsideParen` (§5)
14. **Blank line management:** `maxBlankLines`, `minBlankLinesBetweenBlocks`, `blankLineBetweenSections` (E18: blankBetweenSections > maxBlankLines in arg lists; E16: maxBlankLines enforced at EOF regardless of finalNewline; E21: blankBetweenSections acts as alignArgGroups group boundary)
15. **Alignment:** `alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`, `alignTrailingComments` (E1: after sorting; E12: alignPropertyValues no-op on single-line; E19: alignTrailingComments uses commentGap as minimum gap; E20: padding exempt from collapseSpaces)
16. **Comment formatting:** `commentPreservation`/reflow, `commentWidth` (E8: commentWidth only with reflow), `commentGap` for non-alignment gap
17. **Trailing whitespace:** `trimTrailingWhitespace`
18. **Line endings:** Apply chosen line ending
19. **Final newline:** `finalNewline` (E15: maxBlankLines still enforced)

**Note on `commentGap`:** When `alignTrailingComments = true`, `commentGap` is used as the minimum gap during alignment (step 15, rule E18). When `alignTrailingComments = false`, `commentGap` is applied as simple spacing (step 16). The option participates in both steps depending on context.

**Note on dprint-core:** Steps 9–16 are deeply interleaved in practice. The dprint-core IR (print items) may handle many of these declaratively rather than as sequential passes. The exact architecture depends on how dprint-core's `PrintItems` API is used.

**Spec:** All sections (§1–§16), Appendix C (cascade algorithm), Appendix E (interactions)
**Tests:** All `tests/formatter/` directories (326 fixture pairs total)

### 8. Suppression (`suppression`)
**Responsibility:** Handle `disableFormatting`, `ignorePatterns`, `ignoreCommands`, and pragma off/on/skip regions.
- `disableFormatting=true` → output = input (byte-for-byte, BOM preserved)
- `ignorePatterns` → skip files matching gitignore-style globs
- `ignoreCommands` → preserve matched commands verbatim (case-insensitive)
- Pragma off/on → byte-preserve regions within a file
- Pragma skip → verbatim next command
- `ignoreCommands` takes precedence over `perCommandConfig` (E13)
- `ignoreCommands` suppresses sorting/alignment for that command (E16)

**Spec:** §16 (suppression), §13 (pragmas)
**Tests:** `tests/formatter/16_suppression/` (14 pairs, including `ignorePatterns`), `tests/formatter/13_pragmas/01_off_on/`, `tests/formatter/13_pragmas/02_skip/`

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
- Per-command keyword tables (~50 recognized commands from Appendix F)
- Canonical section orders (4 commands: target_link_libraries, target_sources, install, export)
- Literal constants list (ON, OFF, TRUE, FALSE, etc.)
- Block command pairs (6 pairs: if/endif, foreach/endforeach, while/endwhile, function/endfunction, macro/endmacro, block/endblock)
- Condition-syntax command set (if, elseif, else, endif, while, endwhile)
- Simple commands (~15 commands with no keywords)
- Context-sensitive classification for tokens appearing in both keyword and literal lists (TARGET, COMMAND, POLICY, TEST)

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
- `customKeywords` classification (affects section detection and sorting beyond just casing — §4.3)
- `spaceBeforeParen` (bool for all commands, or string[] for selective), `spaceInsideParen`
- `endCommandArgs` (remove/preserve/match)
  - match for `else()` copies enclosing `if()` condition (requires block tracking)
  - match for `block()/endblock()` always produces empty `endblock()`
  - `elseif()` unaffected by endCommandArgs
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
- `lineEnding` (auto detection by frequency, LF wins ties, bare CR is ordinary character)
- `finalNewline` (add/preserve/empty-file edge cases; false only suppresses adding, does NOT strip existing; empty+false → zero bytes)
- BOM stripping (byte 0 only; inside bracket arg: not stripped)
- Line endings inside quoted strings/bracket args NOT normalized

**Spec:** §3 (blank lines), §7 (line endings), §8 (whitespace)
**Tests:** `tests/formatter/03_blank_lines/` (21 pairs), `tests/formatter/07_line_endings/` (16 pairs), `tests/formatter/08_whitespace/` (8 pairs)
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
- `sortArguments`: case-insensitive, stable, per-keyword-section
- Attached comments travel with arguments
- Group boundaries (blank lines, unattached comments)
- Selective keyword sorting (string[] variant)
- Generator expressions and variable references sorted by literal text
- Commands without recognized keyword sections unaffected
- `sortKeywordSections`: canonical order reordering (4 commands with explicit orders in Appendix F)
- Comments attached to a section travel with it; positional args before first keyword untouched
- Sorting runs before formatting (Appendix E)

**Spec:** §12 (sorting), Appendix F (canonical orders)
**Tests:** `tests/formatter/12_sorting/` (2 subdirs, 26 pairs)
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
- `disableFormatting` passthrough (BOM preserved)
- `ignorePatterns` glob matching (gitignore-style)
- `ignoreCommands` per-command verbatim preservation (case-insensitive)
- Integration with pragma off/on and skip
- `ignoreCommands` precedence over `perCommandConfig` (E13)
- `ignoreCommands` suppresses sorting/alignment (E16)

**Spec:** §16 (suppression)
**Tests:** `tests/formatter/16_suppression/` (14 pairs)
**Depends on:** Milestone 12 (pragma regions), Milestone 13 (per-command resolution)

### Phase 8: Integration (depends on all above)

#### Milestone 15: Cross-Feature Interactions
- All 23 interaction rules from Appendix E (see pipeline validation below)
- Pipeline ordering verification
- Edge cases where features intersect

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

All 23 Appendix E interaction rules mapped to pipeline steps:

| # | Rule (Options) | Pipeline Step(s) | Satisfied |
|---|---|---|---|
| 1 | `sortArguments` + `alignArgGroups` | sort=6, align=15 | Yes: 6 < 15 |
| 2 | `indentStyle="tab"` + alignment | indent=10, align=15 | Yes: architectural (tabs indent, spaces align) |
| 3 | `disableFormatting` + all | suppression=1 | Yes: step 1 short-circuits |
| 4 | `wrapArgThreshold` + `magicTrailingNewline` | both in wrap=9 (Step 0) | Yes: threshold wins per spec |
| 5 | `wrapStyle="vertical"` + `wrapArgThreshold` | both in wrap=9 | Yes: both → Step 3 |
| 6 | `genexWrap="never"` + `genexClosingAngleNewline` | wrap=9 | Yes: never → inline, angle inert |
| 7 | `genexWrap="never"` + `genexIndentWidth` | wrap=9 | Yes: never → inline, indent inert |
| 8 | `commentPreservation="preserve"` + `commentWidth` | comment=16 | Yes: width only with reflow |
| 9 | `firstArgSameLine` + single-line | wrap=9 | Yes: no-op on single-line |
| 10 | `closingParenNewline` + single-line | wrap=9/parens=13 | Yes: no-op on single-line |
| 11 | `blankLineBetweenSections` + single-section | blank=14 | Yes: no-op on single section |
| 12 | `alignPropertyValues` + single-line | align=15 | Yes: no-op on single-line |
| 13 | `perCommandConfig` + `push` pragma | resolve=5 | Yes: push > perCommandConfig |
| 14 | `ignoreCommands` + `perCommandConfig` | resolve=5 | Yes: ignore > perCommandConfig |
| 15 | `ignoreCommands` + pragmas | suppress=1, pragma=4 | Yes: either sufficient |
| 16 | `finalNewline` + `maxBlankLines` | blank=14, final=19 | Yes: maxBlankLines at EOF regardless |
| 17 | `ignoreCommands` + sorting/alignment | suppress=1, sort=6, align=15 | Yes: ignored → verbatim |
| 18 | `blankLineBetweenSections` + `maxBlankLines` | blank=14 | Yes: blankBetweenSections wins in arg lists |
| 19 | `alignTrailingComments` + `commentGap` | align=15 | Yes: commentGap as min gap |
| 20 | `collapseSpaces` + alignment | collapse=8, align=15 | Yes: 8 < 15, padding exempt |
| 21 | `alignArgGroups` + `blankLineBetweenSections` | blank=14, align=15 | Yes: 14 < 15, blank = group boundary |
| 22 | `maxBlankLines` + `sortArguments` | sort=6, blank=14 | Yes: sort boundaries override blank discard |
| 23 | `sortKeywordSections` + `blankLineBetweenSections` | sort=6, blank=14 | Yes: 6 < 14, sort first then insert |

**Result:** All 23 rules satisfied by the proposed 19-step pipeline order.

---

## Test Fixture Inventory

Total: **326 test pairs** across 17 directories.

| Directory | Spec Sections | Pairs | Description |
|---|---|---|---|
| `01_wrapping/` | §1, Appendix C | 41 | line_width, cascade steps 1-3, vertical, firstArgSameLine, wrapArgThreshold, magicTrailingNewline |
| `02_indentation/` | §2 | 21 | indent_width, indent_style (space/tab/mixed), continuationIndent, genexIndent |
| `03_blank_lines/` | §3 | 21 | maxBlankLines, minBlankLinesBetweenBlocks, blankLineBetweenSections |
| `04_casing/` | §4 | 20 | commandCase, keywordCase, customKeywords, literalCase |
| `05_parens_spacing/` | §5 | 14 | closingParenNewline, spaceBeforeParen, spaceInsideParen |
| `06_comments/` | §6 | 34 | preservation, reflow, trailing alignment, commentGap, verbatim |
| `07_line_endings/` | §7 | 16 | auto/lf/crlf detection, finalNewline, BOM |
| `08_whitespace/` | §8 | 8 | trimTrailingWhitespace, collapseSpaces |
| `09_alignment/` | §9 | 24 | propertyValues, consecutiveSet, argGroups |
| `10_genex/` | §10 | 10 | genex wrap cascade/never, closingAngle, nesting |
| `11_per_command/` | §11 | 9 | override wrap/space/sorting/alignment/width, push merge, case-insensitive match |
| `12_sorting/` | §12 | 26 | sortArguments (selective, groups, comments), sortKeywordSections |
| `13_pragmas/` | §13 | 23 | off/on, skip, push/pop |
| `14_flow_control/` | §14 | 19 | indentBlockBody, endCommandArgs (remove/preserve/match), empty commands |
| `15_config_meta/` | §15 | 6 | `$schema` ignored, `extends` merge semantics (scalar override, array replace, shallow per-command merge), relative-path resolution |
| `16_suppression/` | §16 | 14 | disableFormatting, ignoreCommands, ignorePatterns (including inherited relative-path behavior through `extends`) |
| `17_interactions/` | Appendix E | 20 | Cross-feature interaction tests |

### Config Override Mechanisms in Tests
- **`.cmakefmt.toml` files:** Present in multiple test subdirectories
- **Inline pragmas:** `# cmakefmt: push { ... }` / `# cmakefmt: pop` used heavily in most tests

---

## Ambiguities and Gaps in the Spec

### Confirmed Gaps
1. **`endCommandArgs = "match"` for `else()` requires stateful block tracking.** The formatter must track the enclosing `if()` condition through nested blocks to copy it into `else()`. The spec doesn't describe how deeply this should nest or what happens with `else()` inside `function()`/`macro()` bodies. Additionally, `block()/endblock()` always produces empty `endblock()` under match — edge case that's specified but easy to miss.
2. **Keyword dictionary authority split.** Appendix F says the authoritative source is `src/generation/signatures.rs`, but since we're rewriting src/, we need the spec to define keywords independently. The spec gives ~50 recognized commands but not exhaustive per-command keyword lists.
3. **`sortArguments = true` scope.** "All recognized keyword sections" — but what counts as recognized depends on the keyword dictionary, which is only partially specified. Need to define which commands have sortable sections.
4. **Pipeline ordering.** The spec describes individual features but doesn't specify the exact ordering of all formatting passes. Appendix E gives 23 pairwise interaction rules but not a complete total order. The pipeline order in this plan has been validated against all 23 rules (see table above).
5. **`collapseSpaces` timing.** §8.2 says it runs "during input normalization before alignment" — but it also says alignment padding is "exempt." Resolution: collapse runs first (step 8), alignment runs after (step 15) and inserts its own padding which is never re-collapsed.
6. **`perCommandConfig` excluded options.** §11 defines exclusion by category (file-level concerns) but not with a definitive option list. The pragma `push` has broader scope per §13.5. The plan defines: perCommandConfig covers 27 options (§1, §2, §4, §5, §6, §9, §10, §12); push covers all except `disableFormatting`, `extends`, `$schema`, `ignorePatterns`.
7. **`spaceInsideParen = "preserve"` on collapse.** §5.3 says preserve falls back to remove on collapse-to-single-line. What counts as "collapse" should be explicit: fallback applies when the formatter actively collapses a multi-line command to single-line, not when input was already single-line.

### Minor Ambiguities
8. **BOM stripping.** README says UTF-8 BOM is stripped. BOM is only at byte 0 of file — not inside bracket args.
9. **`maxBlankLines` at EOF with `finalNewline = false`.** §7.2 says maxBlankLines still enforced at EOF. Interpretation: trailing blanks reduced to min(existing, maxBlankLines), then no newline added.
10. **Pragma comment whitespace variants.** §13 shows `# cmakefmt:` with a space after `#`. Test `no_space_variant` confirms `#cmakefmt:` (no space) is also recognized. The spec should explicitly list accepted variants.
11. **`alignArgGroups` and `blankLineBetweenSections` interaction.** Appendix E rule 21 says blank lines from blankLineBetweenSections act as alignArgGroups group boundaries. This means alignment is re-computed per section.

### Additional Risks (from spec close-reading)
12. **Dual-type config parsing.** `sortArguments` (bool|string[]) and `spaceBeforeParen` (bool|string[]) require custom serde deserialization. Type mismatch errors must be clear.
13. **Cascade Step 2 escalation is per-keyword-group.** Appendix C says "escalate to Step 3 for that keyword group" — partial escalation within one command is possible. Some groups at Step 2, others at Step 3.
14. **`continuationIndentWidth` is relative to the KEYWORD position, not the command name.** Common misread that would produce wrong indentation.
15. **`genexIndentWidth` is relative to the `$<` column, not line start.** Requires column tracking through the formatting pipeline.
16. **`wrapArgThreshold` counts ALL tokens in parens.** Including keywords and the first positional arg. Generator expression = 1 token. Not just value args.
17. **Leading blank lines at file start are ALWAYS stripped.** Regardless of `maxBlankLines` value, even at `maxBlankLines=100`.
18. **`minBlankLinesBetweenBlocks` requires backward scan.** To find topmost attached comment for correct insertion point.
19. **Tokens in both keyword dict and literal list.** TARGET, COMMAND, POLICY, TEST need context-sensitive classification based on which command they appear in.
20. **Tab + fractional continuation.** When `indentStyle=tab` and continuation differs from indentWidth: tabs for whole multiples, spaces for remainder.
21. **Sorting runs before wrapping.** Pipeline step 6 before 9. Alignment after wrapping at step 15.
22. **Bare CR (`\r` not followed by `\n`) is ordinary character.** Not a line ending, not counted, not normalized.

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
