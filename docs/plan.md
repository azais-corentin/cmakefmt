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

**Spec:** §15 (config meta), README.md (discovery rules), Appendix A (defaults), Appendix B (example)
**Tests:** No dedicated config-parsing fixture directory; config is tested implicitly through all fixtures with `.cmakefmt.toml` files

### 2. Lexer/Tokenizer (`lexer`)
**Responsibility:** Tokenize CMake source into a stream of typed tokens using `logos`.

Token types needed:
- Command names (identifiers before `(`)
- Parentheses `(` / `)`
- Arguments (unquoted, quoted `"..."`, bracket `[==[...]==]`)
- Comments: line comments `# ...`, bracket comments `#[==[...]==]`
- Whitespace: spaces, tabs, newlines (CR, LF, CRLF)
- Generator expressions: `$<`, `>`, `:`, `;`, `,` (within genex context)
- Pragma comments: `# cmakefmt:` prefix detection
- BOM: UTF-8 BOM (`\xEF\xBB\xBF`) detection

**Spec:** Implicit from all sections; §6.5 (bracket args/comments verbatim), §7.1 (line ending detection), §13 (pragma comment syntax)
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
- Canonical section orders for `sortKeywordSections` (PUBLIC→INTERFACE→PRIVATE, etc.)
- Condition-syntax commands (if, elseif, while — args are expressions, not keyword-value pairs)
- Literal constants list for `literalCase` (ON, OFF, TRUE, FALSE, etc.)
- Block-opening/closing command pairs (if↔endif, foreach↔endforeach, etc.)

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
- off/on: opaque region (no nested pragma parsing), unmatched off → suppress to EOF
- skip: blank/comment lines between skip and target preserved; cancelled by off before target
- push/pop: arbitrary nesting, inherits current frame, TOML inline table syntax
- Resolution order: push stack → perCommandConfig → config file → built-in default

**Spec:** §13 (inline pragmas)
**Tests:** `tests/formatter/13_pragmas/` (3 subdirs, 22 pairs)

### 6. Per-Command Config Resolution (`per_command`)
**Responsibility:** Resolve effective config for each command invocation.

Resolution chain: push stack → perCommandConfig[command_name] → config file → built-in default
- Command name matching is case-insensitive
- 27 overridable options (excludes file-level concerns)
- Push-stack can set additional options beyond perCommandConfig scope

**Spec:** §11 (per-command config), §13.4 (push/pop interaction)
**Tests:** `tests/formatter/11_per_command/` (4 pairs), `tests/formatter/13_pragmas/03_push_pop/`

### 7. Formatting Pipeline (`formatter`)
**Responsibility:** Core formatting engine. Takes CST + resolved config → formatted output.

This is the largest module, orchestrating all formatting passes in the correct order.

#### Pipeline Order (critical — derived from Appendix E interactions):
1. **Suppression check:** `disableFormatting` → byte-for-byte passthrough
2. **BOM handling:** Strip BOM (unless `disableFormatting`)
3. **Line ending detection:** Count LF vs CRLF for `auto` mode
4. **Pragma parsing:** Identify off/on regions, skip targets, push/pop stack
5. **Per-command config resolution:** Build effective config per command
6. **Sorting:** `sortArguments` then `sortKeywordSections` (sorting before formatting — Appendix E)
7. **Casing:** Apply `commandCase`, `keywordCase`, `literalCase`, `customKeywords`
8. **Whitespace normalization:** `collapseSpaces` (before alignment — §8.2)
9. **Wrapping/layout:** The cascade/vertical algorithm (§1.2, Appendix C)
   - Step 0 pre-checks (threshold, magic trailing newline)
   - Step 1: try single line
   - Step 2: keyword breaks (cascade only)
   - Step 3: one per line
   - Genex wrapping (§10) applied recursively within arguments
10. **Indentation:** Apply `indentWidth`, `indentStyle`, `continuationIndentWidth`, `genexIndentWidth`
11. **Block body indentation:** `indentBlockBody` (§14.1)
12. **End command args:** `endCommandArgs` remove/preserve/match (§14.2)
13. **Parentheses spacing:** `closingParenNewline`, `spaceBeforeParen`, `spaceInsideParen` (§5)
14. **Blank line management:** `maxBlankLines`, `minBlankLinesBetweenBlocks`, `blankLineBetweenSections`
15. **Alignment:** `alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`, `alignTrailingComments` (after wrapping — needs final line layout)
16. **Comment formatting:** `commentPreservation`/reflow, `commentGap`
17. **Trailing whitespace:** `trimTrailingWhitespace`
18. **Line endings:** Apply chosen line ending
19. **Final newline:** `finalNewline`

**Note:** Steps 9–16 are deeply interleaved in practice. The dprint-core IR (print items) may handle many of these declaratively rather than as sequential passes. The exact architecture depends on how dprint-core's `PrintItems` API is used.

**Spec:** All sections (§1–§16), Appendix C (cascade algorithm), Appendix E (interactions)
**Tests:** All `tests/formatter/` directories

### 8. Suppression (`suppression`)
**Responsibility:** Handle `disableFormatting`, `ignorePatterns`, `ignoreCommands`, and pragma off/on/skip regions.
- `disableFormatting=true` → output = input (byte-for-byte)
- `ignorePatterns` → skip files matching gitignore-style globs
- `ignoreCommands` → preserve matched commands verbatim (case-insensitive)
- Pragma off/on → byte-preserve regions within a file
- Pragma skip → verbatim next command
- `ignoreCommands` takes precedence over `perCommandConfig`

**Spec:** §16 (suppression), §13 (pragmas)
**Tests:** `tests/formatter/16_suppression/`, `tests/formatter/13_pragmas/01_off_on/`, `tests/formatter/13_pragmas/02_skip/`

### 9. CLI (`cli`)
**Responsibility:** Command-line interface using `clap`.

Modes: format (default), `--check`, `--diff`, `--stdin`, `--print-config`
Flags: `--write`/`--inplace`, `--config`, `--assume-filename`, `--color`/`--no-color`, `--verbose`, `--quiet`
Exit codes: 0 (success), 1 (changes found in --check), 2 (error)
Mutual exclusions: --check+--write, --diff+--write, --stdin+file args

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
  - Load `.cmakefmt.toml` from same directory (if present)
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
- TOML deserialization with serde
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
- Line ending detection (LF vs CRLF counting, bare CR handling)
- Pragma comment prefix detection

**Spec:** Implicit from all sections; §7.1, §6.5, §13
**Tests:** Unit tests for token streams

#### Milestone 3: Keyword Dictionary
- Per-command keyword tables
- Canonical section orders
- Literal constants list
- Block command pairs
- Condition-syntax command set

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
- `wrapArgThreshold` forcing
- `magicTrailingNewline` detection and behavior
- `indentWidth`, `indentStyle` (space/tab), `continuationIndentWidth`
- Block body indentation (`indentBlockBody`)
- `closingParenNewline`

**Spec:** §1 (wrapping), §2 (indentation), §5.1 (closingParenNewline), §14.1 (indentBlockBody), Appendix C (cascade detail)
**Tests:** `tests/formatter/01_wrapping/`, `tests/formatter/02_indentation/`, `tests/formatter/05_parens_spacing/01_closing_paren_newline/`, `tests/formatter/14_flow_control/01_indent_block_body/`
**Depends on:** Milestone 4 (parser)

#### Milestone 6: Casing & Simple Transforms
- `commandCase`, `keywordCase`, `literalCase`
- `customKeywords` classification
- `spaceBeforeParen`, `spaceInsideParen`
- `endCommandArgs` (remove/preserve/match)
- `commentGap`

**Spec:** §4 (casing), §5.2–5.3 (paren spacing), §14.2 (endCommandArgs), §6.4 (commentGap)
**Tests:** `tests/formatter/04_casing/`, `tests/formatter/05_parens_spacing/02-03/`, `tests/formatter/14_flow_control/02_end_command_args/`, `tests/formatter/14_flow_control/03_empty_commands/`, `tests/formatter/06_comments/04_comment_gap/`
**Depends on:** Milestone 4 (parser), Milestone 3 (keywords)

### Phase 4: File-Level Formatting (depends on Phase 3)

#### Milestone 7: Blank Lines, Whitespace, Line Endings
- `maxBlankLines` (collapse, leading strip, trailing limit, args discard)
- `minBlankLinesBetweenBlocks` (insert, attached comments, precedence)
- `blankLineBetweenSections`
- `trimTrailingWhitespace`
- `collapseSpaces` (before alignment, exempt alignment padding)
- `lineEnding` (auto detection, forced LF/CRLF)
- `finalNewline` (add/preserve/empty-file edge cases)
- BOM stripping

**Spec:** §3 (blank lines), §7 (line endings), §8 (whitespace)
**Tests:** `tests/formatter/03_blank_lines/`, `tests/formatter/07_line_endings/`, `tests/formatter/08_whitespace/`
**Depends on:** Milestone 5 (wrapping determines line layout)

### Phase 5: Advanced Features (depends on Phases 3–4)

These milestones can be built in parallel since they don't depend on each other (except M10 which also depends on M7 from Phase 4).

#### Milestone 8: Generator Expressions
- Genex wrapping: cascade (colon split, semicolons, commas) vs never
- `genexClosingAngleNewline`
- `genexIndentWidth`
- Recursive nesting

**Spec:** §10 (generator expressions), §2.4 (genexIndentWidth)
**Tests:** `tests/formatter/10_genex/`, `tests/formatter/02_indentation/04_genex_indent/`
**Depends on:** Milestone 5 (integrates with wrapping algorithm)

#### Milestone 9: Comment Formatting
- Comment preservation: re-indentation, standalone handling, collapse prevention
- Comment reflow: paragraph detection, code block preservation, fenced blocks, list items, unclosed fences
- `commentWidth`
- `alignTrailingComments` (consecutive lines, same nesting depth, group breaking)
- Verbatim content: bracket comments, bracket args, multi-line strings

**Spec:** §6 (comments)
**Tests:** `tests/formatter/06_comments/` (5 subdirs, 20 pairs)
**Depends on:** Milestone 5 (needs line layout for reflow width calculations)

#### Milestone 10: Alignment
- `alignPropertyValues`: column-align PROPERTIES values
- `alignConsecutiveSet`: column-align consecutive `set()` calls
- `alignArgGroups`: detect repeating token-count patterns, column-align
- Alignment must run after wrapping (needs final layout)
- `collapseSpaces` exemption for alignment padding

**Spec:** §9 (alignment), §8.2 (collapseSpaces exemption)
**Tests:** `tests/formatter/09_alignment/` (3 subdirs, 14 pairs)
**Depends on:** Milestone 5 (wrapping), Milestone 7 (collapseSpaces interaction)

#### Milestone 11: Sorting
- `sortArguments`: case-insensitive, stable, per-keyword-section
- Attached comments travel with arguments
- Group boundaries (blank lines, unattached comments)
- Selective keyword sorting (string[] variant)
- `sortKeywordSections`: canonical order reordering
- Sorting runs before formatting (Appendix E)

**Spec:** §12 (sorting), Appendix F (canonical orders)
**Tests:** `tests/formatter/12_sorting/` (2 subdirs, 13 pairs)
**Depends on:** Milestone 3 (keyword dictionary), Milestone 4 (parser)

### Phase 6: Pragma & Per-Command (depends on Phase 1; sequenced here for integration clarity)

#### Milestone 12: Pragma Engine
- Parse `# cmakefmt:` directives from comment tokens
- off/on region tracking (opaque, no nested parsing)
- skip directive (next-command targeting, blank/comment gap handling)
- push/pop config stack (TOML inline table parsing, nesting, merge semantics)
- Warning diagnostics (unmatched on, skip at EOF, pop without push, unknown push keys)

**Spec:** §13 (inline pragmas)
**Tests:** `tests/formatter/13_pragmas/` (3 subdirs, 22 pairs)
**Depends on:** Milestone 1 (config struct for push/pop)

#### Milestone 13: Per-Command Config Resolution
- Resolve effective config per command invocation
- Resolution chain: push stack → perCommandConfig → config file → default
- Case-insensitive command name matching
- Overridable vs excluded option sets
- Push stack shallow-merge with perCommandConfig

**Spec:** §11 (per-command config), §13.4 (push/pop interaction)
**Tests:** `tests/formatter/11_per_command/`, `tests/formatter/13_pragmas/03_push_pop/push_per_command_merge/`
**Depends on:** Milestone 1 (config), Milestone 12 (pragma engine)

### Phase 7: Suppression (depends on Phase 6)

#### Milestone 14: Suppression
- `disableFormatting` passthrough
- `ignorePatterns` glob matching
- `ignoreCommands` per-command verbatim preservation
- Integration with pragma off/on and skip
- `ignoreCommands` precedence over `perCommandConfig`

**Spec:** §16 (suppression)
**Tests:** `tests/formatter/16_suppression/` (2 pairs)
**Depends on:** Milestone 12 (pragma regions), Milestone 13 (per-command resolution)

### Phase 8: Integration (depends on all above)

#### Milestone 15: Cross-Feature Interactions
- All interaction rules from Appendix E
- Pipeline ordering verification
- Edge cases where features intersect

**Spec:** Appendix E (interactions)
**Tests:** `tests/formatter/17_interactions/` (~20 pairs)
**Depends on:** All previous milestones

#### Milestone 16: Test Harness
- Rewrite `tests/formatter_test.rs`
- Recursive fixture discovery
- Config loading (`.cmakefmt.toml` + pragma inline)
- Byte-exact comparison with diff reporting
- Idempotency verification

**Depends on:** Milestones 5–14 (needs a substantially working formatter)

#### Milestone 17: CLI
- clap argument parsing
- File discovery and glob expansion
- --check, --diff, --stdin, --write modes
- --config, --assume-filename
- --print-config
- Exit codes
- Color and verbosity control

**Spec:** Appendix D (CLI)
**Depends on:** Milestone 1 (config), Milestone 16 (test harness for integration tests)

#### Milestone 18: dprint Plugin Interface
- WASM plugin protocol via dprint-core
- `format_text` implementation
- Config schema exposure
- Plugin metadata

**Depends on:** Milestones 5–15 (all formatting milestones)

---

## Ambiguities and Gaps in the Spec

### Confirmed Gaps
1. **No §15 test fixtures.** There is no `tests/formatter/15_*` directory. Config meta (extends, $schema, unknown keys) has no fixture-based tests. These need to be written.
2. **`endCommandArgs = "match"` for `else()` requires stateful block tracking.** The formatter must track the enclosing `if()` condition through nested blocks to copy it into `else()`. The spec doesn't describe how deeply this should nest or what happens with `else()` inside `function()`/`macro()` bodies.
3. **Keyword dictionary authority split.** Appendix F says the authoritative source is `src/generation/signatures.rs`, but since we're rewriting src/, we need the spec to define keywords independently. The spec gives illustrative examples but not exhaustive lists for every command.
4. **`sortArguments = true` scope.** "All recognized keyword sections" — but what counts as recognized depends on the keyword dictionary, which is only partially specified. Need to define which commands have sortable sections.
5. **Pipeline ordering.** The spec describes individual features but doesn't specify the exact ordering of all formatting passes. Appendix E gives pairwise interaction rules (e.g., "sorting before alignment") but not a complete total order. The implementation plan above proposes an order; it should be validated.
6. **`collapseSpaces` timing.** §8.2 says it runs "during input normalization before alignment" — but it also says alignment padding is "exempt." If collapse runs before alignment, how is the exemption implemented? Likely: collapse runs first, alignment runs after and inserts its own padding which is never re-collapsed. This should be made explicit.
7. **`perCommandConfig` excluded options.** §11 says "file-level concerns" are excluded but defines exclusion by example (blank lines, line endings, whitespace normalization, suppression, config meta). The pragma `push` has a broader scope. Need a definitive excluded-option list for perCommandConfig vs push.
8. **`spaceInsideParen = "preserve"` on collapse.** §5.3 says preserve falls back to remove on collapse-to-single-line. But what counts as "collapse"? Only when the formatter actively collapses a multi-line command to single line, or also when input was already single-line? (Likely: fallback only on active collapse.)

### Minor Ambiguities
9. **BOM stripping.** README says UTF-8 BOM is stripped. §7.1 says line endings inside bracket args not normalized. Is BOM inside a bracket arg stripped? (Likely: BOM is only at byte 0 of file.)
10. **`maxBlankLines` at EOF with `finalNewline = false`.** §7.2 says maxBlankLines still enforced at EOF. What does "enforced" mean when finalNewline=false and there are trailing blanks? (Likely: trailing blanks reduced to min(existing, maxBlankLines) then no newline added.)
11. **Pragma comment whitespace variants.** §13 shows `# cmakefmt:` with a space after `#`. The test `no_space_variant` suggests `#cmakefmt:` (no space) is also recognized. The spec should explicitly list accepted variants.
12. **`alignArgGroups` and `blankLineBetweenSections` interaction.** Appendix E says blankLineBetweenSections blank lines act as alignArgGroups group boundaries. Does this mean alignment is re-computed per section?

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
