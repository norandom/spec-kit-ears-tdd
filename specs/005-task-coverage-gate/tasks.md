# Tasks: Task Coverage Gate

**Input**: `spec.md` and `traceability.toml` in `specs/005-task-coverage-gate/`

**Tests**: Mandatory. Each behaviour is pinned by a conformance case under `conformance/cases/`.

**Organization**: Grouped by the property being added. The span work is separated because it was
corrected by measurement against real task lists rather than by design.

---

## Phase 1: The gate opens the task list

- [X] T001 Add conformance cases for full coverage, an uncovered requirement, a missing task list
      and an unknown reference covering REQ-001, REQ-002, REQ-005 and REQ-006, using
      `conformance/cases/tasks-full-coverage`, `tasks-uncovered-requirement`, `tasks-list-missing`
      and `tasks-unknown-reference`
- [X] T002 Read the task list beside each specification and report a missing one for REQ-001 and
      REQ-002 in `crates/ears-sdd/src/tasks.rs`
- [X] T003 Report declared requirements no task references, and references the specification does
      not declare, for REQ-005 and REQ-006 in `crates/ears-sdd/src/tasks.rs`
- [X] T004 Add a case proving the plan gate is unaffected by a missing task list for REQ-001 in
      `conformance/cases/tasks-plan-gate-unaffected`

## Phase 2: Ranges count as coverage

- [X] T005 Add conformance cases for a dash span, a word span and a descending span covering
      REQ-003, REQ-004 and REQ-007, using `conformance/cases/tasks-span-coverage`,
      `tasks-word-span` and `tasks-descending-span`
- [X] T006 Expand spans numerically rather than textually, so a mismatched digit width between
      endpoints is a non-issue, for REQ-003 in `crates/ears-sdd/src/tasks.rs`
- [X] T007 Accept dash, ellipsis and word range markers for REQ-004 in
      `crates/ears-sdd/src/tasks.rs`, after a dashes-only implementation produced seven false
      failures against the reference project
- [X] T008 Report a backwards span rather than expanding it to nothing for REQ-007 in
      `crates/ears-sdd/src/tasks.rs`

## Phase 3: Reporting and safety

- [X] T009 Add a case for a reference inside a fenced code block for REQ-008 in
      `conformance/cases/tasks-fenced-reference`
- [X] T010 Reuse the specification parser's fence tracking so an illustration is not coverage for
      REQ-008 in `crates/ears-sdd/src/tasks.rs`
- [X] T011 Report the covered count on the tasks phase only, rather than as a misleading zero
      elsewhere, for REQ-010 in `crates/ears-sdd/src/report.rs`
- [X] T012 Cover the tasks phase in the read-only assertion for REQ-009 in
      `crates/ears-sdd/tests/read_only.rs`
