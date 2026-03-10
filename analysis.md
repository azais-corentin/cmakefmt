# CMakefmt Project Analysis (Specs, Fixtures, Source, Gaps, Dependencies)

## Changelog

### 2026-03-10
- Re-read all 23 documents under `docs/specs/` and revalidated each per-file summary.
- Recounted `.in.cmake` fixtures under every `nn_topic/nn_subtopic` directory; pair counts remain unchanged.
- Re-read `tests/formatter_tests.rs` and all files in `src/`; implementation snapshot remains accurate.
- Added explicit delta notes for specs/fixtures/src: no new or removed spec files, no new or removed fixture directories, and no `src/` module-surface changes.

## 0) Scope and method

This analysis is based on:
- All files under `docs/specs/`
- `tests/formatter_tests.rs`
- Fixture inventory under `tests/formatter/`
- All Rust source files under `src/`

No code was modified.

---

## 1) Specification document summaries (`docs/specs/`)

### Update delta vs previous analysis
- **New spec files:** none.
- **Removed spec files:** none.
- **Spec summaries corrected:** none; existing summaries remain accurate after full reread.

## `README.md`
- **Feature area:** Overall contract and option index.
- **Distinct configurable options defined:** **42** (full summary table across sections 1–16).
- **Key behavioral rules:**
  - Input must be UTF-8; non-UTF-8 is rejected.
  - BOM is stripped when formatting is enabled; **not** stripped when formatting is disabled.
  - Config discovery should walk up directories (`.cmakefmt.toml` preferred over `cmakefmt.toml` in same dir).
  - Parse failure behavior should preserve input and report diagnostics.

## `01-line-width-wrapping.md`
- **Feature area:** Wrapping algorithm.
- **Options:** **5** (`lineWidth`, `wrapStyle`, `firstArgSameLine`, `wrapArgThreshold`, `magicTrailingNewline`).
- **Rules:** 3-step cascade vs vertical mode, token non-splitting, threshold-forced wrapping, magic-trailing-newline skip semantics.

## `02-indentation.md`
- **Feature area:** Indentation policy.
- **Options:** **4** (`indentWidth`, `indentStyle`, `continuationIndentWidth`, `genexIndentWidth`).
- **Rules:** nesting-relative indentation, tabs-vs-spaces behavior, mixed tab+space remainder strategy, genex indentation relative to `$<` column.

## `03-blank-lines.md`
- **Feature area:** Blank-line normalization.
- **Options:** **3** (`maxBlankLines`, `minBlankLinesBetweenBlocks`, `blankLineBetweenSections`).
- **Rules:** leading blank lines always removed; block-minimum blank lines can override max; comment attachment shifts insertion point; section separators inserted between recognized keyword sections.

## `04-casing.md`
- **Feature area:** Casing normalization.
- **Options:** **4** (`commandCase`, `keywordCase`, `customKeywords`, `literalCase`).
- **Rules:** keyword dictionary drives structural keyword casing; custom keywords affect both casing and layout semantics; keyword classification has precedence over literal casing in overlaps.

## `05-parens-spacing.md`
- **Feature area:** Parenthesis placement/spacing.
- **Options:** **3** (`closingParenNewline`, `spaceBeforeParen`, `spaceInsideParen`).
- **Rules:** multiline close-paren placement, command-selective or global pre-paren spacing, single-line-only inside-paren spacing modes.

## `06-comments.md`
- **Feature area:** Comment formatting.
- **Options:** **4** (`commentPreservation`, `commentWidth`, `alignTrailingComments`, `commentGap`).
- **Rules:** preserve vs reflow, paragraph/list/code-fence heuristics, alignment grouping boundaries, comment-gap minimum spacing.
- **Also defines fixed behavior:** verbatim preservation for bracket args/comments and multiline quoted strings.

## `07-line-endings.md`
- **Feature area:** Output line endings and EOF newline.
- **Options:** **2** (`lineEnding`, `finalNewline`).
- **Rules:** dominant-ending auto detection with LF tie-break, bare CR handling, exact final-newline policy semantics, BOM-at-byte-0 scope.

## `08-whitespace.md`
- **Feature area:** Whitespace normalization.
- **Options:** **2** (`trimTrailingWhitespace`, `collapseSpaces`).
- **Rules:** trailing whitespace trimming and pre-alignment space collapse rules.

## `09-alignment.md`
- **Feature area:** Alignment modes.
- **Options:** **3** (`alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`).
- **Rules:** per-mode grouping heuristics, alignment boundaries (blank/comment/keyword), line-width overflow exclusion.

## `10-generator-expressions.md`
- **Feature area:** Genex wrapping/closure formatting.
- **Options:** **2** (`genexWrap`, `genexClosingAngleNewline`).
- **Rules:** delimiter-aware genex cascade (`:`, `;`, `,`), recursive nesting behavior, closing-angle newline control.

## `11-per-command-config.md`
- **Feature area:** Per-command overrides.
- **Options:** **1** (`perCommandConfig`).
- **Rules:** command-name case-insensitive matching, allowed-overrides subset, file-level options excluded, push-stack precedence over per-command overrides.

## `12-sorting.md`
- **Feature area:** Sorting.
- **Options:** **2** (`sortArguments`, `sortKeywordSections`).
- **Rules:** section-scoped stable case-insensitive sort, attached-comment mobility, unattached-comment boundaries, canonical per-command section ordering.

## `13-inline-pragmas.md`
- **Feature area:** Inline pragma controls.
- **Options:** **0 config-file options** (pragma directives instead).
- **Rules:** syntax for `off/on/skip/push/pop`, stack semantics, resolution order, settable/non-settable pragma keys, warning-only diagnostics behavior.

## `14-flow-control.md`
- **Feature area:** Flow-control block shaping.
- **Options:** **2** (`indentBlockBody`, `endCommandArgs`).
- **Rules:** block-body indentation toggle, close-command argument remove/preserve/match behavior, nearest-unmatched `if` resolution for `else(...)` in match mode.

## `15-config-meta.md`
- **Feature area:** Config metadata and inheritance.
- **Options:** **2** (`$schema`, `extends`).
- **Rules:** `$schema` informational only; `extends` merge semantics (scalar override, array replace, table shallow merge), cycle/depth errors, unknown-key warning policy.

## `16-suppression.md`
- **Feature area:** Global suppression and ignore routing.
- **Options:** **3** (`disableFormatting`, `ignorePatterns`, `ignoreCommands`).
- **Rules:** `disableFormatting` absolute precedence and byte-for-byte passthrough, gitignore-like glob semantics, per-command suppression.

## `appendix-a-defaults.md`
- **Feature area:** Canonical defaults snapshot.
- **Options defined:** **0 new** (lists defaults for all options).
- **Rules:** documents baseline effective values and inherited-null defaults.

## `appendix-b-example-config.md`
- **Feature area:** Example real-world config.
- **Options defined:** **0 new**.
- **Rules:** demonstrates opinionated config and per-command table examples.

## `appendix-c-cascade-algorithm.md`
- **Feature area:** Normative wrap algorithm mechanics.
- **Options defined:** **0 new** (details behavior of section 1 options).
- **Rules:** explicit Step 0/1/2/3 flow, pre-keyword positional handling, vertical-mode simplification.

## `appendix-d-cli.md`
- **Feature area:** CLI-only behavior.
- **Config-file options defined:** **0**.
- **CLI flags/controls specified:** **10 major flags/areas** (`--check`, `--diff`, `--stdin`, `--write/--inplace`, `--config`, `--assume-filename`, `--color/--no-color`, `--verbose`, `--quiet`, `--print-config`) + interaction matrix.

## `appendix-e-interactions.md`
- **Feature area:** Pipeline ordering and pairwise option interactions.
- **Options defined:** **0 new**.
- **Rules:** normative 17-step global pipeline and interaction consequences (sorting/alignment precedence, suppression precedence, genex option interactions, etc.).

## `appendix-f-keyword-dictionary.md`
- **Feature area:** Normative keyword dictionary and command classes.
- **Options defined:** **0 new**.
- **Rules:** command-class taxonomy, sortable sections, canonical section order, nested non-sortable structures, keyword-vs-literal precedence.

---

## 2) Test harness and fixture inventory

### Update delta vs previous analysis
- **New fixture directories (`nn_topic/nn_subtopic`):** none.
- **Removed fixture directories (`nn_topic/nn_subtopic`):** none.
- **Pair-count deltas:** none (all previously listed counts revalidated from current `.in.cmake` inventory).
- **Harness behavior changes:** none; fixtures are still driven by defaults or `###` headers, not per-directory `.cmakefmt.toml` files.

## 2.1 `tests/formatter_tests.rs` harness behavior

### Discovery
- Recursively walks `tests/formatter/` for `*.in.cmake` and `*.out.cmake`.
- Builds stem sets and reports missing counterpart files on either side.
- Sorts input files before execution.

### Invocation
- For each `.in.cmake`:
  - Reads input and expected output.
  - If input begins with `### `, parses that header using `load_from_header` and strips the header line from formatted content.
  - Otherwise uses `Configuration::default()`.
  - Calls `format_text(Path::new("CMakeLists.txt"), input, &config)` inside `catch_unwind`.

### Comparison and validation
- Cases handled:
  - **panic:** reported as PANIC failure.
  - **`Ok(Some(formatted))`:** compared against expected (`simple_diff`), then second-pass idempotency check (`format_text(formatted)` must not change it).
  - **`Ok(None)`:** input treated as already formatted; input must equal expected; second-pass idempotency rechecked.
  - **`Err(e)`:** reported as PARSE ERROR.
- Any failures panic at end with aggregated report.
- `simple_diff` includes special rendering for invisible-only differences.

### Important harness constraints observed
- The harness **does not** load per-fixture `.cmakefmt.toml` files.
- Effective config comes only from:
  1. default config, or
  2. a leading `### ...` header line in the `.in.cmake` file.

This is crucial because many fixture directories include `.cmakefmt.toml`, but the harness path/config flow never reads those files.

---

## 2.2 Fixture directories by `nn_topic/nn_subtopic`

Pair counts below are counted from `.in.cmake` files; harness enforces matching `.out.cmake` counterparts.

### 01_wrapping
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `01_wrapping/01_line_width` | line-width fit/overflow boundaries | 8 | `lineWidth` | `indentation_exceeds_width`, `token_exceeds_width` |
| `01_wrapping/02_cascade` | cascade step behavior | 8 | `wrapStyle` (cascade), `lineWidth` | `step2_pre_keyword_positional` (`customKeywords` + cascade) |
| `01_wrapping/03_vertical` | vertical wrapping mode | 4 | `wrapStyle` | none significant |
| `01_wrapping/04_first_arg_same_line` | opener-line first arg policy | 7 | `firstArgSameLine`, `lineWidth` | `overflow_tolerated` |
| `01_wrapping/05_wrap_arg_threshold` | arg-count forced wrapping | 7 | `wrapArgThreshold` | `genex_counts_as_one`, `counting_includes_keywords` |
| `01_wrapping/06_magic_trailing_newline` | no-collapse signal | 7 | `magicTrailingNewline` | `vertical_skips_to_step3` (`wrapStyle` + magic) |

### 02_indentation
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `02_indentation/01_indent_width` | base indent width | 5 | `indentWidth` | `applies_to_block_body` (indent with flow blocks) |
| `02_indentation/02_indent_style` | tabs vs spaces | 6 | `indentStyle`, `indentWidth` | `tab_mixed_continuation` (`continuationIndentWidth`), `tab_visual_width_for_line_calc` (`lineWidth`) |
| `02_indentation/03_continuation_indent` | keyword-value continuation indentation | 5 | `continuationIndentWidth`, `indentWidth` | `explicit_4`, `relative_to_keyword` |
| `02_indentation/04_genex_indent` | genex-specific indentation | 5 | `genexIndentWidth` | `tab_genex_remainder_tabs` (`indentStyle` + `indentWidth` + `lineWidth`) |

### 03_blank_lines
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `03_blank_lines/01_max_blank_lines` | blank-line collapse limit | 6 | `maxBlankLines` | `blank_lines_in_args_discarded` |
| `03_blank_lines/02_min_blank_lines_between_blocks` | block separator minimums | 10 | `minBlankLinesBetweenBlocks` | `precedence_over_max` (`maxBlankLines` + min override) |
| `03_blank_lines/03_blank_line_between_sections` | section separators within commands | 5 | `blankLineBetweenSections` | mild `lineWidth` interplay in some fixtures |

### 04_casing
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `04_casing/01_command_case` | command casing | 3 | `commandCase` | none |
| `04_casing/02_keyword_case` | keyword casing | 4 | `keywordCase` | none |
| `04_casing/03_custom_keywords` | custom keyword classification | 5 | `customKeywords`, `keywordCase` | `custom_affects_sorting`, `keyword_precedence_over_literal` |
| `04_casing/04_literal_case` | literal casing | 8 | `literalCase` | `keyword_precedence_in_context` |

### 05_parens_spacing
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `05_parens_spacing/01_closing_paren_newline` | multiline close-paren placement | 4 | `closingParenNewline` | `false_with_trailing_comment` (`commentGap` behavior) |
| `05_parens_spacing/02_space_before_paren` | space before `(` | 4 | `spaceBeforeParen` | `case_insensitive` |
| `05_parens_spacing/03_space_inside_paren` | single-line inside-paren spacing | 6 | `spaceInsideParen` | `collapse_to_single_preserve_removes` (`magicTrailingNewline=false`) |

### 06_comments
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `06_comments/01_preservation` | preserve mode and comment positioning | 5 | `commentPreservation` (preserve) | `standalone_prevents_collapse` |
| `06_comments/02_reflow` | comment reflow heuristics | 12 | `commentPreservation`, `commentWidth`, `lineWidth` | `pragma_comments_never_reflowed` (pragmas + reflow) |
| `06_comments/03_trailing_comments` | trailing comment alignment groups | 6 | `alignTrailingComments` | depth/group break fixtures |
| `06_comments/04_comment_gap` | code-to-`#` spacing | 3 | `commentGap` | none |
| `06_comments/05_verbatim` | verbatim multiline/bracket handling | 8 | fixed behavior (§6.5), plus `lineEnding` in dedicated sub-fixture configs | `*_line_endings_preserved` |

### 07_line_endings
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `07_line_endings/01_line_ending` | LF/CRLF/auto behavior | 6 | `lineEnding` | `auto_dominant`, `auto_equal_lf_wins` |
| `07_line_endings/02_final_newline` | EOF newline behavior | 8 | `finalNewline` | `false_max_blank_lines` (`maxBlankLines` + final newline) |
| `07_line_endings/03_bom` | BOM handling | 3 | BOM behavior + `disableFormatting` interaction | `bom_preserved_when_disabled` |

### 08_whitespace
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `08_whitespace/01_trim_trailing` | trailing whitespace trim | 4 | `trimTrailingWhitespace` | none |
| `08_whitespace/02_collapse_spaces` | intra-line space collapsing | 4 | `collapseSpaces` | `preserves_strings`, `not_indentation` |

### 09_alignment
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `09_alignment/01_property_values` | property value column alignment | 4 | `alignPropertyValues` | `multi_value_wraps` |
| `09_alignment/02_consecutive_set` | consecutive `set()` alignment groups | 9 | `alignConsecutiveSet` | blank/comment/non-set breakers |
| `09_alignment/03_arg_groups` | tabular arg-group alignment | 11 | `alignArgGroups`, `lineWidth` | `overflow_excluded`, `cross_keyword_no_align` |

### 10_genex
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `10_genex/01_genex_wrap` | genex wrapping strategy | 7 | `genexWrap`, `lineWidth` | delimiter-specific fixtures (`colon`, `semicolon`, `comma`) |
| `10_genex/02_closing_angle` | closing `>` newline policy | 3 | `genexClosingAngleNewline` | `recursive_nesting` |

### 11_per_command
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `11_per_command/01_override_wrap_style` | per-command wrap style override | 1 | `perCommandConfig.wrapStyle` | none |
| `11_per_command/02_override_space_before_paren` | per-command paren spacing | 1 | `perCommandConfig.spaceBeforeParen` | none |
| `11_per_command/03_push_overrides_per_command` | precedence with pragma stack | 1 | `perCommandConfig` + `push` | yes (resolution order) |
| `11_per_command/04_case_insensitive_match` | case-insensitive command key matching | 1 | `perCommandConfig` key matching | yes |
| `11_per_command/05_override_line_width` | per-command line width | 1 | `perCommandConfig.lineWidth` | none |
| `11_per_command/06_multiple_overrides` | multi-field command override | 1 | multiple per-command fields | yes |
| `11_per_command/07_excluded_options` | excluded file-level options | 1 | per-command exclusion rules | yes |
| `11_per_command/08_override_sorting` | per-command sorting override | 1 | `perCommandConfig.sortArguments` | yes |
| `11_per_command/09_override_alignment` | per-command alignment override | 1 | `perCommandConfig.alignPropertyValues` | yes |

### 12_sorting
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `12_sorting/01_sort_arguments` | section-scoped value sorting | 17 | `sortArguments` | comments/group boundaries/custom keywords/genex/variable text |
| `12_sorting/02_sort_keyword_sections` | section reordering | 10 | `sortKeywordSections` | attached comments and positional retention |

### 13_pragmas
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `13_pragmas/01_off_on` | region suppression | 7 | `off/on` pragmas | nested pragma opacity, config preservation |
| `13_pragmas/02_skip` | single-command suppression | 5 | `skip` pragma | skip + push/pop, skip + off cancellation |
| `13_pragmas/03_push_pop` | scoped override stack | 11 | `push/pop` pragmas | per-command merge, ignoreCommands, indentBlockBody, resolution order |

### 14_flow_control
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `14_flow_control/01_indent_block_body` | block-body indentation | 5 | `indentBlockBody` | nested/branch cases |
| `14_flow_control/02_end_command_args` | end/else arg policy | 14 | `endCommandArgs` | match mode nesting, wrap behavior, all block types |
| `14_flow_control/03_empty_commands` | empty command invariant | 1 | fixed behavior (§14.3) | none |

### 15_config_meta
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `15_config_meta/01_schema` | schema metadata | 1 | `$schema` | none |
| `15_config_meta/02_extends` | inheritance/merge semantics | 5 | `extends` | scalar override, array replace, per-command shallow merge, relative path |

### 16_suppression
| Directory | Feature area | Pairs | Spec options exercised | Interaction-looking fixtures |
|---|---:|---:|---|---|
| `16_suppression/01_disable_formatting` | full formatting bypass | 6 | `disableFormatting` | overrides all options/pragmas, preserves BOM/line endings/whitespace |
| `16_suppression/02_ignore_commands` | command-level bypass | 4 | `ignoreCommands` | case-insensitive matching, surrounding formatting |
| `16_suppression/03_ignore_patterns` | file-level bypass via globs | 4 | `ignorePatterns`, `extends`-relative resolution | inherited-relative path behavior |

### 17_interactions
Subdirectories matching `topic/subtopic` form:

| Directory | Feature area | Pairs | Spec options exercised |
|---|---:|---:|---|
| `17_interactions/ignore_commands_plus_alignment` | suppression vs alignment | 1 | `ignoreCommands` + `alignArgGroups` |
| `17_interactions/ignore_commands_plus_per_command` | suppression vs per-command overrides | 1 | `ignoreCommands` + `perCommandConfig` |
| `17_interactions/ignore_commands_plus_pragmas` | suppression vs pragma skip | 1 | `ignoreCommands` + `skip` |
| `17_interactions/ignore_commands_plus_sorting` | suppression vs sorting | 1 | `ignoreCommands` + `sortArguments` |

Additionally, `17_interactions/` root has **16** top-level pair fixtures that directly map to Appendix E pairwise interactions, including:
- `align_trailing_comments_plus_gap`
- `arg_groups_plus_blank_sections`
- `blank_between_sections_plus_max_blank_lines`
- `collapse_spaces_exempt_alignment`
- `final_newline_plus_max_blank_lines`
- `first_arg_single_line_no_effect`
- `genex_never_ignores_closing_angle`
- `genex_never_ignores_indent_width`
- `indent_style_tab_plus_alignment`
- `max_blank_lines_sort_group_boundaries`
- `preserve_comment_width_no_effect`
- `sort_args_then_align_arg_groups`
- `sort_sections_then_blank_lines`
- `vertical_plus_threshold`
- `wrap_threshold_plus_magic`
- `wrap_threshold_wins_over_magic`

---

## 3) Current `src/` implementation state

### Update delta vs previous analysis
- **New `src/` modules/files:** none.
- **Removed `src/` modules/files:** none.
- **Implementation status changes:** no material delta from previous analysis.
- **Placeholder/TODO delta:** still no `TODO`/`FIXME`; existing placeholders remain (`wasm_plugin` empty metadata/update hooks, `parse_one_expression` panic guard).

## 3.1 Module structure

- `src/lib.rs`
  - re-exports config types/loaders and `format_text`
- `src/configuration/`
  - `types.rs` (runtime config model)
  - `load.rs` (config parsing from dprint/header/CLI wrapper)
  - `resolve.rs` (dprint plugin config resolution)
- `src/parser/`
  - tokenizer (`logos`) + AST + parser
- `src/generation/`
  - `gen_file.rs` (file-level layout / block indentation / blank lines)
  - `gen_command.rs` (command-level formatting engine)
  - `signatures.rs` (command keyword dictionaries/specs)
- `src/format_text.rs`
  - parse + generate + print options bridge
- `src/wasm_plugin.rs`
  - dprint plugin adapter
- `src/bin/cmakefmt.rs`
  - CLI binary

## 3.2 Public API surface (`lib.rs` exports)

- `format_text`
- `Configuration`, `CaseStyle`, `NewLineKind`
- `ConfigLoadResult`, `ConfigDiagnostic`, `ConfigDiagnosticSeverity`
- `load_from_cli`, `load_from_dprint`, `load_from_header`
- `resolve_config`

## 3.3 What is already implemented

### Implemented configuration fields (actual runtime `Configuration`)
`line_width`, `indent_width`, `use_tabs`, `new_line_kind`, `command_case`, `keyword_case`, `closing_paren_newline`, `sort_lists`, `max_blank_lines`, `space_before_paren`.

### Implemented formatter behavior (high-level)
- Parsing:
  - commands, nested paren groups, quoted/unquoted/bracket args, line and bracket comments.
- Generation:
  - command case and keyword case transformations.
  - space-before-paren list matching.
  - single-line attempt then multiline fallback.
  - command-spec-based keyword grouping using large signature tables.
  - some section/property/pair/flow formatting patterns.
  - conditional-expression formatting helpers (`if`/`while`/`elseif`).
  - genex parsing/formatting helpers embedded in command formatting.
- File-level:
  - block-indent tracking via opener/middle/closer command names.
  - blank-line compression via `max_blank_lines`.
  - always emits trailing newline.
- Output controls:
  - line ending selection (`lf`, `crlf`, simple `auto` detection based on first newline encountered).
  - BOM strip at beginning via `strip_bom`.
- Sorting:
  - `sort_lists` bool with a hardcoded set of sortable commands and keywords.

## 3.4 TODOs, stubs, placeholders

- No explicit `TODO/FIXME` markers were found in `src/`.
- Practical placeholders/stub-like behavior observed:
  - `wasm_plugin.rs`:
    - `help_url` and `config_schema_url` are empty strings.
    - `check_config_updates` always returns empty changes.
  - `gen_command.rs`:
    - `panic!` path in `parse_one_expression` for empty input path assumption.
- Architectural incompleteness vs spec is substantial (covered below), but not marked with in-code TODOs.

---

## 4) Gaps: specs vs tests vs implementation

## 4.1 Spec’d features with fixtures but no (or effectively no) implementation in `src/`

The following are clearly represented in fixture directories but absent from the runtime config/model and formatter pipeline implementation:

- **Wrapping controls beyond `lineWidth`:**
  - `wrapStyle`, `firstArgSameLine`, `wrapArgThreshold`, `magicTrailingNewline`
- **Indentation variants:**
  - `indentStyle` as spec enum (`space|tab`) via config keys, `continuationIndentWidth`, `genexIndentWidth`
- **Blank-line controls:**
  - `minBlankLinesBetweenBlocks`, `blankLineBetweenSections`
- **Casing extensions:**
  - `customKeywords`, `literalCase` option semantics
- **Paren/spacing:**
  - `spaceInsideParen`
- **Comment options:**
  - `commentPreservation` modes, `commentWidth`, `alignTrailingComments`, `commentGap`
- **Line ending/final newline nuances:**
  - `finalNewline` option semantics
  - dominant-count `lineEnding=auto` behavior (current implementation is first-newline heuristic)
- **Whitespace options:**
  - explicit `trimTrailingWhitespace` and `collapseSpaces` controls
- **Alignment options:**
  - `alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`
- **Genex options:**
  - `genexWrap`, `genexClosingAngleNewline` as configurable behavior
- **Per-command overrides:**
  - `perCommandConfig`
- **Sorting semantics:**
  - spec-level `sortArguments` semantics (bool/array scope, comment attachment boundaries, etc.)
  - `sortKeywordSections`
- **Pragmas:**
  - `off/on/skip/push/pop` behavior and precedence
- **Flow control options:**
  - `indentBlockBody` toggle
  - `endCommandArgs` remove/preserve/match behavior
- **Config meta:**
  - `$schema` handling in config files
  - `extends` chain loading/merge behavior in runtime formatter
- **Suppression/ignore:**
  - `disableFormatting`, `ignorePatterns`, `ignoreCommands`

### Key root cause observed
`tests/formatter_tests.rs` only feeds default config or a `###` header-derived config into `format_text`; it does not load fixture `.cmakefmt.toml` files. Meanwhile, formatter runtime also lacks config-file discovery/loading pipeline per spec.

## 4.2 Implemented features with little/no fixture-level coverage

- **Large command signature catalog** (`generation/signatures.rs` ~80+ command specs) extends far beyond explicit fixture coverage concentration.
- **dprint plugin adapter paths** (`resolve_config`, plugin metadata/update hooks) are not exercised by formatter fixtures.
- **CLI behavior** in `src/bin/cmakefmt.rs` (glob expansion, check/write/stdin modes) is outside formatter fixture harness scope.
- **`load_from_header` parser** has unit tests in `src/configuration/load.rs`, but that is separate from fixture `.cmakefmt.toml` semantics.

## 4.3 Spec items with neither tests nor implementation (in this repo state)

Most notably from **Appendix D / README runtime behavior**:
- `--diff`
- `--config <path>`
- `--assume-filename <path>`
- `--color` / `--no-color`
- `--verbose`
- `--quiet`
- `--print-config`
- full CLI interaction matrix semantics

Also not implemented and not directly fixture-tested:
- Config discovery walk-up semantics (`.cmakefmt.toml` vs `cmakefmt.toml`, stdin resolution rules).

---

## 5) Feature dependency analysis (foundational vs leaf)

Using Appendix E pipeline + current architecture, the implementation dependency order should be:

## 5.1 Foundational (must work first)

1. **Configuration resolution and suppression gates**
   - `disableFormatting`, `ignorePatterns`, `ignoreCommands`
   - config-file loading/discovery + `extends`
   - pragma stack (`off/on/skip/push/pop`)

2. **Canonical keyword/section classification layer**
   - Appendix F dictionary behavior
   - `customKeywords`
   - section and group boundaries used by sorting/alignment/blank-lines

3. **Core wrapping and indentation engine**
   - `lineWidth`, `wrapStyle`, `firstArgSameLine`, `wrapArgThreshold`, `magicTrailingNewline`
   - `indentWidth`, `indentStyle`, `continuationIndentWidth`, `genexIndentWidth`

4. **Flow-control structure semantics**
   - `indentBlockBody`
   - `endCommandArgs` matching logic

These four layers are prerequisites for trustworthy downstream formatting.

## 5.2 Mid-layer dependents

- **Sorting** (`sortArguments`, `sortKeywordSections`) depends on correct section/keyword parsing and comment attachment.
- **Blank-line policies** (`minBlankLinesBetweenBlocks`, `blankLineBetweenSections`, `maxBlankLines`) depend on stable block/section detection and post-wrap layout.
- **Alignment** options depend on post-wrap line layout and spacing normalization.
- **Comment reflow/alignment/gap** depends on line-width calculations and final indentation model.

## 5.3 Leaf-ish / relatively independent

- `commandCase`, `keywordCase`, `literalCase` (given correct token classification)
- `spaceBeforeParen`, `spaceInsideParen`, `closingParenNewline`
- output finalization (`lineEnding`, `finalNewline`, BOM behavior)
- metadata (`$schema` warning behavior)

These are easier to implement once foundational parsing/layout/config routing is correct.

---

## 6) Bottom-line status

- Spec corpus is comprehensive and option-rich (42 config options + pragma and CLI contracts).
- Fixture corpus is extensive and deliberately interaction-heavy.
- Current source implementation is a **subset formatter** with a much smaller runtime configuration model.
- The largest systemic gap is **config routing/control plane** (config-file discovery, per-command overrides, pragmas, suppression), which blocks many fixture-intended behaviors even before line-formatting details.
- Feature implementation order should prioritize the foundational control/config layers before polishing leaf formatting options.