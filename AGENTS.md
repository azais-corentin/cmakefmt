# AGENTS.md — AI Agent Workflow for cmakefmt

This file defines the workflow for AI coding agents (Claude Code / Oh My Pi) working on this project. It codifies a 4-phase development cycle that ensures every change is specified, planned, tracked, and verified before it lands.

A fresh agent session starts here. Read this file first, then follow the phases in order.

---

## Project Documentation Map

| File            | Purpose                                                                                                      | Phase   |
| --------------- | ------------------------------------------------------------------------------------------------------------ | ------- |
| `docs/specs/`   | Authoritative formatting specification (split into per-section files). See `docs/specs/README.md` for index. | Phase 1 |
| `docs/plan.md`  | Implementation plan. Ordered list of work items derived from the spec.                                       | Phase 2 |
| `docs/tasks.md` | Task breakdown. Granular, individually-committable units of work.                                            | Phase 3 |
| `Cargo.toml`    | Rust project manifest. Uses `dprint-core` for formatting infrastructure.                                     | —       |

If `docs/plan.md` or `docs/tasks.md` do not exist yet, create them during the appropriate phase.

---

## Phase 1: Spec Validation

**Goal:** Ensure `docs/specs/` is complete and accurate before any implementation begins.

### Steps

1. Read the relevant section files under `docs/specs/` (see `docs/specs/README.md` for index).
2. Identify ambiguities, contradictions, or missing edge cases.
3. If the user's request introduces new behavior, update the spec **first** — implementation follows the spec, never the reverse.
4. When a spec section grows beyond ~150 lines, split it into subsections with clear cross-references.

### Rules

- The spec is the single source of truth. If code disagrees with the spec, the code is wrong (unless the spec has an acknowledged bug).
- Never implement behavior that is not specified. If you need it, spec it first.
- Mark open questions with `> **OPEN:**` blockquotes in the spec so they are visible.

### Example: Adding a new config option

```markdown
### X.Y `newOption`

|             |           |
| ----------- | --------- |
| **Type**    | `boolean` |
| **Default** | `false`   |

Description of what this option does, when it triggers, and how it interacts
with existing options (especially `lineWidth`, `wrapStyle`).

\`\`\`cmake
# Before (newOption = false):
some_command(ARG1 ARG2)

# After (newOption = true):
some_command(
  ARG1
  ARG2
)
\`\`\`
```

---

## Phase 2: Implementation Plan

**Goal:** Produce `docs/plan.md` — an ordered sequence of implementation milestones derived from the spec.

### Steps

1. Read the current spec (`docs/specs/`) and existing codebase (`src/`).
2. Use Plan Mode (Oh My Pi) or structured reasoning to decompose the spec into implementation milestones.
3. Write `docs/plan.md` with this structure:

```markdown
# Implementation Plan

## Milestone 1: <Name>
- **Spec sections:** §X.Y, §X.Z
- **Dependencies:** None | Milestone N
- **Summary:** What this milestone delivers.
- **Key decisions:** Any architectural choices made here.

## Milestone 2: <Name>
...
```

### Rules

- Each milestone maps to one or more spec sections.
- Dependencies between milestones are explicit. No circular dependencies.
- Milestones that touch shared state (parser, config struct, formatting pipeline) are sequential.
- Milestones that touch independent modules (e.g., different formatting rules with no interaction) can be parallelized.

---

## Phase 3: Task Breakdown

**Goal:** Produce `docs/tasks.md` — granular, individually-committable tasks derived from the plan.

### Steps

1. Read `docs/plan.md`.
2. For each milestone, break it into tasks sized for a single atomic commit.
3. Write `docs/tasks.md` with this structure:

```markdown
# Tasks

## Milestone 1: <Name>

- [ ] `task-1.1` — <Description> (spec §X.Y)
- [ ] `task-1.2` — <Description> (spec §X.Z)
- [ ] `task-1.3` — Write tests for task-1.1 and task-1.2

## Milestone 2: <Name>

- [ ] `task-2.1` — <Description> (spec §A.B)
...
```

### Task Sizing Rules

- **One logical change per task.** A task either implements a behavior or writes tests for it — never both.
- **Implementation and tests are separate tasks.** This forces test-writing to be a deliberate, first-class activity rather than an afterthought.
- A task should touch at most 3-5 files. If it touches more, split it.
- Each task must be completable in a single agent session without requiring external input.
- If a task requires a design decision, that decision belongs in the plan, not discovered mid-task.

### Marking Progress

Update `docs/tasks.md` as tasks complete:

```markdown
- [x] `task-1.1` — Implement lineWidth config parsing (spec §1.1) ← done
- [ ] `task-1.2` — Implement cascade wrapping step 1 (spec §1.2)
```

---

## Phase 4: Execution

**Goal:** Implement tasks one at a time, commit atomically, and keep documentation current.

### The Execution Loop

```
1. Pick the next uncompleted task from docs/tasks.md
2. Read the relevant spec section(s)
3. Read the relevant source files
4. Implement the change
5. Write or update tests (if this is a test task)
6. Run tests: `cargo test`
7. Update docs if affected (spec, plan, or tasks)
8. Commit with a conventional commit message
9. Mark the task complete in docs/tasks.md
10. Repeat
```

### Subagent Usage (Oh My Pi Task Tool)

Use subagents for parallel work when tasks are independent:

- **Safe to parallelize:** Tasks in different modules that don't share types or state. Example: implementing two unrelated config options that don't interact.
- **Must be sequential:** Tasks that modify shared structures (parser, `Config` struct, the formatting pipeline). Example: adding a new config field must land before any task that reads it.
- **Discovery tasks:** When entering unfamiliar code, dispatch an `explore` subagent to map the relevant files before planning edits.

Example subagent dispatch for two independent formatting rules:

```
Task tool:
  context: "Implementing formatting rules from docs/specs/. Config struct is in src/config.rs. Tests go in tests/."
  tasks:
    - IndentWidth: "Implement indentWidth (spec §2.1) in src/indent.rs"
    - IndentStyle: "Implement indentStyle (spec §2.2) in src/indent.rs"
```

Do **not** parallelize tasks that touch the same file unless you use `isolated: true`.
- **Never delegate file reading:** Do not use tasks/subagents to read full files verbatim. Use the `read` tool directly instead — it is faster, cheaper, and avoids unnecessary context duplication.

### Commit Discipline

Every commit corresponds to exactly one task. Use conventional commit format:

| Prefix      | When                                       |
| ----------- | ------------------------------------------ |
| `feat:`     | New user-facing behavior                   |
| `fix:`      | Bug fix                                    |
| `test:`     | Adding or updating tests only              |
| `refactor:` | Code restructuring with no behavior change |
| `docs:`     | Documentation-only changes                 |
| `chore:`    | Build config, CI, tooling                  |

Examples:

```
feat: implement lineWidth config parsing (spec §1.1)
test: add lineWidth boundary tests for 40 and 320
fix: handle single-token overflow in cascade wrapping
refactor: extract keyword grouping into dedicated module
docs: update spec §1.2 with vertical wrap clarification
```

Rules:
- Subject line: imperative mood, lowercase, no period, max 72 chars.
- Body (optional): explain *why*, not *what*. Reference spec sections.
- One logical change per commit. If you need "and" in the subject, split it.

---

## Preflight Checklist

Run this at the start of every coding session. No exceptions.

```
1. [ ] Read this file (AGENTS.md)
2. [ ] Read docs/specs/ — or the sections relevant to today's work (see docs/specs/README.md)
3. [ ] Read docs/plan.md — know where you are in the plan
4. [ ] Read docs/tasks.md — identify the next task(s)
5. [ ] Run `cargo test` — confirm the tree is green before you touch anything
6. [ ] Run `cargo build` — confirm it compiles
7. [ ] Check git status — no uncommitted changes from a previous session
```

If the tree is red or dirty, fix that **before** starting new work.

---

## Testing Policy

### Tests Are Separate Tasks

Every implementation task has a corresponding test task. They are distinct items in `docs/tasks.md`. This separation ensures:

- Tests are written deliberately, not as an afterthought.
- Test tasks can be reviewed independently.
- Coverage gaps are visible in the task list.

### What "Tested" Means

A behavior is tested when:

1. **Happy path:** The common case works as specified.
2. **Boundary values:** Min/max of ranges (e.g., `lineWidth` at 40 and 320).
3. **Edge cases:** Empty input, single-token overflow, deeply nested structures.
4. **Interaction:** If the option interacts with others (documented in the spec), those interactions are covered.
5. **Idempotency:** Formatting the output again produces identical output.

### Test Location

- Unit tests: `#[cfg(test)] mod tests` in the source file, or `tests/` directory.
- Integration/snapshot tests: `tests/` directory with `.cmake` fixture files.
- Run with: `cargo test`

### Test Naming

```rust
#[test]
fn line_width_default_80_columns() { ... }

#[test]
fn line_width_single_token_exceeds_limit_emits_as_is() { ... }

#[test]
fn cascade_wrap_step2_keyword_on_new_line() { ... }
```

Names describe the scenario and expected outcome. No `test_1`, `test_2`.

---

## Documentation Maintenance

Every task that changes behavior must update the affected documentation **before** the task is marked complete.

| What changed               | Update                                               |
| -------------------------- | ---------------------------------------------------- |
| New config option          | `docs/specs/<section>.md` (add/update section)       |
| Behavior change            | `docs/specs/<section>.md` (update section, examples) |
| Bug fix to spec'd behavior | `docs/specs/<section>.md` (clarify if ambiguous)     |
| Task completed             | `docs/tasks.md` (check the box)                      |
| Milestone completed        | `docs/plan.md` (mark done), `docs/tasks.md`          |
| Architectural change       | `docs/plan.md` (update affected milestones)          |

Documentation is not optional. A task with code changes and no doc updates is incomplete.
