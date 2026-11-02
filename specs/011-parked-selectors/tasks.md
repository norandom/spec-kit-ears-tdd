# Tasks: Parked Selectors

**Input**: Design documents from `/specs/011-parked-selectors/`

**Prerequisites**: plan.md, spec.md

**Tests**: Mandatory. Four new conformance cases are the contract and must fail until the
implementation lands; two existing cases are the regression guard and must stay byte-stable.

## Format: `[ID] [P?] [Story?] [REQ-NNN] Description with file path`

---

## Phase 1: Spec artifacts (Kiro step: requirements → design → tasks)

- [x] T001 Write `specs/011-parked-selectors/spec.md`, `plan.md`, `tasks.md`,
  `checklists/requirements.md`, `quickstart.md`, `traceability.toml`

## Phase 2: Tests first (red)

- [x] T002 [REQ-001] [REQ-002] [REQ-003] [REQ-005] Author the four fixtures and their
  `expected.json` from the rule: `conformance/cases/pending-selector-parked/`,
  `pending-selector-parked-present/`, `pending-selector-parked-name-missing/`,
  `pending-selector-parked-disabled/`; confirm `pending-selector-parked*` fail against the current
  binary with `TRACE_TEST_FILE` errors
- [x] T003 [P] [REQ-002] [REQ-006] Confirm `conformance/cases/selector-missing-test/expected.json`
  and `conformance/cases/selector-traversal/expected.json` still match unchanged

## Phase 3: Implementation (green)

- [x] T004 [REQ-004] Add `parked_selector_prefixes` (default `["conformance/pending/"]`) and a
  `parked_prefixes()` POSIX normalizer to `crates/ears-sdd/src/config.rs`
- [x] T005 [REQ-001] [REQ-002] [REQ-003] [REQ-005] [REQ-006] In `crates/ears-sdd/src/traceability.rs`,
  branch the missing-file check: parked prefix → `TRACE_TEST_PARKED` advisory, else
  `TRACE_TEST_FILE` error; existing files keep the named-test check; root/traversal checks run
  first
- [x] T006 [REQ-001] [REQ-002] [REQ-003] [REQ-005] Make the four new cases pass;
  `cargo test -p ears-sdd --test conformance`

## Phase 4: Docs and gates

- [x] T007 [P] Add `TRACE_TEST_PARKED` to `docs/reference/findings.md` and
  `parked_selector_prefixes` to `docs/reference/configuration.md`
- [x] T008 [REQ-001] Run `ears-sdd validate --project . --feature specs/011-parked-selectors
  --phase spec|plan|tasks|final` (all green) and `--phase final --all` (green; 009/010 pending
  selectors now advisories)
- [x] T009 Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test -p ears-sdd`

---

## Dependencies & Execution Order

Phase 1 → Phase 2 (red) → Phase 3 (green) → Phase 4. T003 is the regression guard and runs beside
T002. T007 is independent of T004–T006.

## Notes

- Do not copy `REQ-NNN` into production code; tests and fixtures may name them.
- The point of the feature is this repo's own `final --all` going green with 009/010 parked.
