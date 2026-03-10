# Implementation Plan

## Changelog

### 2026-03-10 (incremental refresh)
- Revalidated against the latest `analysis.md`: no new/removed spec files, fixture directories, or `src/` module-surface changes.
- Preserved the existing feature inventory (no newly discovered feature areas and no removals from spec scope).
- Re-layered phase assignments to strict dependency levels so each phase depends only on earlier phases (no intra-phase dependency chains).
- Reordered features within each phase from simpler to more complex.

### 2026-03-10
- Incrementally refreshed from `analysis.md` (no new/removed spec files or fixture directories).
- Preserved feature scope and aligned planning assumptions to current analysis (42 spec options; fixtures run via default config or leading `###` header config).
- Marked fixture-harness rewrite for per-directory `.cmakefmt.toml` as obsolete (fixtures and harness behavior are acceptance constraints).

## Planning Assumptions
- `docs/specs/` is authoritative for behavior.
- `tests/formatter_tests.rs` is the acceptance harness for fixture coverage.
- Files under `tests/formatter/` are immutable; plan items target implementation in `src/` and runtime behavior.
- Some spec areas (notably CLI matrix behavior) currently require explicit command-matrix validation in addition to fixture runs.

## Obsolete From Previous Plan

| Item | Status | Rationale |
|---|---|---|
| Rewrite fixture harness to load nearest `.cmakefmt.toml` | Obsolete | Latest analysis confirms harness intentionally uses default config or `###` header config only; changing harness/fixtures is out of scope. |

## Phase 1 — Configuration Foundation

| Feature | One-line description | Spec file(s) | Test file(s) | Complexity | Dependencies |
|---|---|---|---|---|---|
| Configuration Model Parity | Expand runtime configuration to the full spec surface (42 options), including dual-type fields and inherited-null semantics. | `docs/specs/README.md`, `docs/specs/appendix-a-defaults.md`, `docs/specs/appendix-b-example-config.md` | `src/configuration/load.rs`, `tests/formatter_tests.rs` (header-config fixture coverage) | Large | — |

## Phase 2 — Config Resolution and Structural Classification

| Feature | One-line description | Spec file(s) | Test file(s) | Complexity | Dependencies |
|---|---|---|---|---|---|
| Config Discovery & `extends` Resolution | Implement config discovery walk-up and `extends` merge rules (scalar override, array replace, shallow table merge, cycle/depth diagnostics). | `docs/specs/15-config-meta.md`, `docs/specs/README.md` | `src/configuration/load.rs`, `tests/formatter_tests.rs` (fixtures under `tests/formatter/15_config_meta/**`) | Large | Configuration Model Parity |
| Keyword/Section Classification Layer | Align command classes, section boundaries, sortable scopes, and keyword-vs-literal precedence with Appendix F plus `customKeywords`. | `docs/specs/04-casing.md`, `docs/specs/12-sorting.md`, `docs/specs/14-flow-control.md`, `docs/specs/appendix-f-keyword-dictionary.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/04_casing/**`, `tests/formatter/12_sorting/**`, `tests/formatter/14_flow_control/**`) | Large | Configuration Model Parity |

## Phase 3 — Early Pipeline Control and Base Layout

| Feature | One-line description | Spec file(s) | Test file(s) | Complexity | Dependencies |
|---|---|---|---|---|---|
| Suppression Gate Routing | Enforce early short-circuit behavior for `disableFormatting`, `ignorePatterns`, and `ignoreCommands` with correct precedence. | `docs/specs/16-suppression.md`, `docs/specs/appendix-e-interactions.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/16_suppression/**`, `tests/formatter/17_interactions/ignore_commands_plus_*`) | Medium | Config Discovery & `extends` Resolution |
| Wrapping Engine Parity | Implement full cascade/vertical wrapping behavior (`wrapStyle`, `firstArgSameLine`, `wrapArgThreshold`, `magicTrailingNewline`) on top of `lineWidth`. | `docs/specs/01-line-width-wrapping.md`, `docs/specs/appendix-c-cascade-algorithm.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/01_wrapping/**`) | Large | Configuration Model Parity, Keyword/Section Classification Layer |

## Phase 4 — Indentation and Pragma Control

| Feature | One-line description | Spec file(s) | Test file(s) | Complexity | Dependencies |
|---|---|---|---|---|---|
| Indentation Engine Parity | Implement indentation semantics for spaces/tabs, continuation alignment, and generator-expression relative indentation columns. | `docs/specs/02-indentation.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/02_indentation/**`) | Medium | Wrapping Engine Parity |
| Inline Pragma Engine | Implement `off/on/skip/push/pop` parsing, stack semantics, and warning-only diagnostics without violating suppression guarantees. | `docs/specs/13-inline-pragmas.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/13_pragmas/**`) | Large | Configuration Model Parity, Suppression Gate Routing |

## Phase 5 — Command-Scoped Structural Semantics

| Feature | One-line description | Spec file(s) | Test file(s) | Complexity | Dependencies |
|---|---|---|---|---|---|
| Flow-Control Block Shaping | Implement `indentBlockBody` and `endCommandArgs` behavior, including nearest-unmatched `if` matching for `else(...)`. | `docs/specs/14-flow-control.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/14_flow_control/**`) | Medium | Indentation Engine Parity, Keyword/Section Classification Layer |
| Generator Expression Formatting Controls | Implement `genexWrap` and `genexClosingAngleNewline` semantics with recursive nesting support. | `docs/specs/10-generator-expressions.md`, `docs/specs/02-indentation.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/10_genex/**`, `tests/formatter/02_indentation/04_genex_indent/**`) | Medium | Wrapping Engine Parity, Indentation Engine Parity |
| Per-Command Override Resolution | Apply command-level effective config with case-insensitive matching and precedence: `push` stack > `perCommandConfig` > file/default config. | `docs/specs/11-per-command-config.md`, `docs/specs/13-inline-pragmas.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/11_per_command/**`, `tests/formatter/13_pragmas/03_push_pop/**`) | Medium | Configuration Model Parity, Inline Pragma Engine, Keyword/Section Classification Layer |

## Phase 6 — Casing/Paren Policies and Sorting

| Feature | One-line description | Spec file(s) | Test file(s) | Complexity | Dependencies |
|---|---|---|---|---|---|
| Casing & Parenthesis Spacing | Implement `commandCase`, `keywordCase`, `literalCase`, `spaceBeforeParen`, `spaceInsideParen`, and `closingParenNewline` interactions. | `docs/specs/04-casing.md`, `docs/specs/05-parens-spacing.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/04_casing/**`, `tests/formatter/05_parens_spacing/**`) | Medium | Keyword/Section Classification Layer, Per-Command Override Resolution |
| Sorting Semantics | Implement stable, case-insensitive `sortArguments` and canonical `sortKeywordSections` with comment-attachment boundaries preserved. | `docs/specs/12-sorting.md`, `docs/specs/appendix-f-keyword-dictionary.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/12_sorting/**`) | Large | Keyword/Section Classification Layer, Per-Command Override Resolution |

## Phase 7 — Vertical Rhythm Policies

| Feature | One-line description | Spec file(s) | Test file(s) | Complexity | Dependencies |
|---|---|---|---|---|---|
| Blank-Line Policies | Implement `maxBlankLines`, `minBlankLinesBetweenBlocks`, and `blankLineBetweenSections` with correct precedence and attachment scanning. | `docs/specs/03-blank-lines.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/03_blank_lines/**`) | Medium | Flow-Control Block Shaping, Sorting Semantics |

## Phase 8 — Alignment

| Feature | One-line description | Spec file(s) | Test file(s) | Complexity | Dependencies |
|---|---|---|---|---|---|
| Alignment Modes | Implement `alignPropertyValues`, `alignConsecutiveSet`, `alignArgGroups`, and trailing comment alignment with overflow exclusion rules. | `docs/specs/09-alignment.md`, `docs/specs/06-comments.md`, `docs/specs/08-whitespace.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/09_alignment/**`, `tests/formatter/06_comments/03_trailing_comments/**`) | Large | Wrapping Engine Parity, Sorting Semantics, Blank-Line Policies |

## Phase 9 — Comment Formatting

| Feature | One-line description | Spec file(s) | Test file(s) | Complexity | Dependencies |
|---|---|---|---|---|---|
| Comment Formatting Engine | Implement preserve/reflow modes, `commentWidth`, and `commentGap`, while honoring pragma and verbatim exclusions. | `docs/specs/06-comments.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/06_comments/**`) | Large | Wrapping Engine Parity, Indentation Engine Parity, Alignment Modes |

## Phase 10 — Whitespace and Output Finalization

| Feature | One-line description | Spec file(s) | Test file(s) | Complexity | Dependencies |
|---|---|---|---|---|---|
| Whitespace, Line Endings, and EOF Finalization | Enforce `trimTrailingWhitespace`, `collapseSpaces`, dominant `lineEnding=auto`, BOM behavior, and `finalNewline` semantics. | `docs/specs/07-line-endings.md`, `docs/specs/08-whitespace.md`, `docs/specs/README.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/07_line_endings/**`, `tests/formatter/08_whitespace/**`) | Medium | Blank-Line Policies, Alignment Modes, Comment Formatting Engine |

## Phase 11 — Cross-Feature Interaction Conformance

| Feature | One-line description | Spec file(s) | Test file(s) | Complexity | Dependencies |
|---|---|---|---|---|---|
| Appendix E Interaction Conformance | Validate and fix ordering/precedence interactions across the full formatter pipeline. | `docs/specs/appendix-e-interactions.md` | `tests/formatter_tests.rs` (fixtures under `tests/formatter/17_interactions/**`) | Large | Wrapping Engine Parity, Indentation Engine Parity, Flow-Control Block Shaping, Per-Command Override Resolution, Generator Expression Formatting Controls, Sorting Semantics, Casing & Parenthesis Spacing, Blank-Line Policies, Alignment Modes, Comment Formatting Engine, Whitespace, Line Endings, and EOF Finalization, Suppression Gate Routing, Inline Pragma Engine |

## Phase 12 — CLI Contract Parity

| Feature | One-line description | Spec file(s) | Test file(s) | Complexity | Dependencies |
|---|---|---|---|---|---|
| CLI Contract Parity | Implement Appendix D command behavior matrix (`--check`, `--diff`, `--stdin`, `--write`, config flags, verbosity/color, exit codes) and README runtime behavior expectations at process boundaries. | `docs/specs/appendix-d-cli.md`, `docs/specs/README.md` | `src/bin/cmakefmt.rs` (manual scenario validation against Appendix D matrix; no dedicated acceptance fixture file today) | Large | Config Discovery & `extends` Resolution, Suppression Gate Routing, Appendix E Interaction Conformance |

## Delivery Notes
- Implement one feature at a time in phase order; each row is intended to be independently verifiable.
- For fixture-backed features, validation runs through `tests/formatter_tests.rs` scoped to the relevant fixture directory.
- For CLI behavior, maintain a reproducible command matrix tied to Appendix D until dedicated automated CLI tests are added.