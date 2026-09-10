# Tasks: Kiro Project Assessment

**Input**: Design documents from `/specs/008-kiro-assessment/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: Mandatory. Conformance fixtures under `conformance/cases/kiro-*/` are the contract and
must fail until the matching implementation lands. Parser unit tests in `ccsdd.rs` lock G1–G4.

**Organization**: Setup and types first, then one phase per user story (P1 discover, P1 baseline,
P1 supersession, P2 disclosure). Each story: failing contract → implementation → fixture passes.

## Format: `[ID] [P?] [Story?] [REQ-NNN] Description with file path`

- **[P]**: Different files, no dependency on incomplete tasks
- **[USn]**: User story from spec.md
- Every behaviour task names `REQ-NNN` and a test selector

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Module and report shape. No Kiro behaviour yet.

- [X] T001 Create `crates/ears-sdd/src/kiro.rs` with `KiroSpecification`, `Phase`, `ActiveBaseline`, and `SupersessionGraph` from `specs/008-kiro-assessment/data-model.md`, and declare `pub mod kiro` in `crates/ears-sdd/src/lib.rs`
- [X] T002 [P] Add optional `baseline_included` / `baseline_excluded` on `Summary` and `baseline` / `exclusion_reason` on `FeatureResult` in `crates/ears-sdd/src/report.rs` per `specs/008-kiro-assessment/contracts/report.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Parser locked, discovery record can name a layout family. Blocks every user story.

**⚠️ CRITICAL**: No user story work until this phase is complete

- [X] T003 [REQ-005] [REQ-007] [REQ-008] [REQ-009] [REQ-019] [REQ-020] Write unit tests for `ccsdd::Parser` in `crates/ears-sdd/src/ccsdd.rs` covering G1, G2, G3 amendments with `extends`, G4 empty documents, a baseline heading with no criteria, and fenced examples
- [X] T004 Add `family` (`speckit` | `kiro`) to `SpecLocation` in `crates/ears-sdd/src/discovery.rs`

**Checkpoint**: Parser tests green against current `ccsdd.rs`. `SpecLocation` can distinguish layouts.

---

## Phase 3: User Story 1 - A Kiro tree is visible without becoming Spec Kit (Priority: P1) 🎯 MVP

**Goal**: `ears-sdd validate --project <kiro-tree> --phase spec` discovers and parses `.kiro/specs`, does not require Spec Kit files, does not change Spec Kit-only projects, and writes nothing.

**Independent Test**: `conformance/cases/kiro-discovers-tree/expected.json` matches; `all-features-scope` still matches; `crates/ears-sdd/tests/read_only.rs` still passes on a Kiro fixture copy.

### Tests for User Story 1

> Write / run these FIRST and confirm they FAIL (except the Spec Kit regression, which must stay green).

- [X] T005 [US1] [REQ-001] [REQ-002] [REQ-004] [REQ-005] [REQ-006] [REQ-007] [REQ-008] [REQ-009] [REQ-022] Run `cargo test -p ears-sdd --test conformance` and confirm `conformance/cases/kiro-discovers-tree/expected.json`, `conformance/cases/kiro-missing-requirements-md/expected.json`, `conformance/cases/kiro-empty-headings/expected.json`, `conformance/cases/kiro-missing-criteria/expected.json`, and `conformance/cases/kiro-fenced-example/expected.json` fail against current `ears-sdd`
- [X] T006 [P] [US1] [REQ-003] Confirm `conformance/cases/all-features-scope/expected.json` still matches (Spec Kit discovery unchanged)

### Implementation for User Story 1

- [X] T007 [US1] [REQ-001] [REQ-002] Walk `.kiro/specs/*/` in `crates/ears-sdd/src/kiro.rs` and union those locations into `discovery::discover` in `crates/ears-sdd/src/discovery.rs`; Kiro-only whole-tree scope value is `.kiro/specs/*/requirements.md` per `specs/008-kiro-assessment/contracts/report.md`
- [X] T008 [US1] [REQ-004] Emit `KIRO_NO_REQUIREMENTS` and continue when a feature directory has no `requirements.md` in `crates/ears-sdd/src/kiro.rs`
- [X] T009 [US1] [REQ-005] [REQ-006] Parse each Kiro `requirements.md` with `ccsdd::Parser` in `crates/ears-sdd/src/lib.rs` and qualify identifiers with the directory basename
- [X] T010 [US1] [REQ-007] [REQ-008] [REQ-009] Treat empty-heading documents as zero criteria, promote missing-criteria notes to `KIRO_NO_CRITERIA`, and keep fenced blocks out of discovery in `crates/ears-sdd/src/lib.rs`
- [X] T011 [US1] [REQ-022] Suppress `CONFIG_MISSING` when a Kiro tree is present in `crates/ears-sdd/src/lib.rs`
- [X] T012 [US1] [REQ-003] Leave the Spec Kit glob path in `crates/ears-sdd/src/discovery.rs` unchanged when `.kiro/specs` is absent
- [X] T013 [US1] [REQ-021] Snapshot `conformance/cases/kiro-discovers-tree/project` in `crates/ears-sdd/tests/read_only.rs` so a Kiro assessment cannot write
- [X] T014 [US1] Resolve `--feature` for a Kiro basename or `requirements.md` path in `crates/ears-sdd/src/discovery.rs` per `specs/008-kiro-assessment/contracts/cli.md`
- [X] T015 [US1] [REQ-001] [REQ-002] [REQ-004] [REQ-006] [REQ-007] [REQ-008] [REQ-009] [REQ-022] Make `conformance/cases/kiro-discovers-tree/expected.json`, `kiro-missing-requirements-md/expected.json`, `kiro-empty-headings/expected.json`, `kiro-missing-criteria/expected.json`, and `kiro-fenced-example/expected.json` pass

**Checkpoint**: Pointing `validate` at a Kiro fixture parses criteria and does not touch the tree. Spec Kit projects still match `all-features-scope`.

---

## Phase 4: User Story 2 - Only current specifications are in force (Priority: P1)

**Goal**: Phase metadata decides the active baseline. Superseded, reserved, and initialized are out; missing phase is in; unreadable metadata is an error and out.

**Independent Test**: `conformance/cases/kiro-active-baseline/expected.json` and `kiro-unreadable-metadata/expected.json` match.

### Tests for User Story 2

- [X] T016 [US2] [REQ-010] [REQ-011] [REQ-012] [REQ-013] Confirm `conformance/cases/kiro-active-baseline/expected.json` fails before baseline code exists
- [X] T017 [P] [US2] [REQ-025] [REQ-026] Confirm `conformance/cases/kiro-unreadable-metadata/expected.json` fails before metadata handling exists

### Implementation for User Story 2

- [X] T018 [US2] [REQ-010] [REQ-011] [REQ-013] Read `spec.json` phase in `crates/ears-sdd/src/kiro.rs` and exclude `superseded`, `reserved`, and `initialized`; include absent phase
- [X] T019 [US2] [REQ-012] Fill `summary.baseline_included` / `baseline_excluded` and print the human `Baseline:` line in `crates/ears-sdd/src/lib.rs` and `crates/ears-sdd/src/report.rs`
- [X] T020 [US2] [REQ-025] [REQ-026] Emit `KIRO_METADATA` and exclude the specification when `spec.json` cannot be read in `crates/ears-sdd/src/kiro.rs`
- [X] T021 [US2] [REQ-010] [REQ-011] [REQ-012] [REQ-013] [REQ-025] [REQ-026] Make `conformance/cases/kiro-active-baseline/expected.json` and `kiro-unreadable-metadata/expected.json` pass

**Checkpoint**: A mixed live/superseded/reserved/initialized tree reports two included and three excluded, with reasons.

---

## Phase 5: User Story 3 - A supersession is a checked link, not a comment (Priority: P1)

**Goal**: Replacement is a graph over feature names. Dangling and unreciprocated are errors; recuts and amendments are not.

**Independent Test**: recut passes; dangling and unreciprocated fail the gate; amendment-extends and operational-supersede pass with no supersession finding.

### Tests for User Story 3

- [X] T022 [US3] [REQ-014] [REQ-018] Confirm `conformance/cases/kiro-supersession-recut/expected.json` fails before link construction exists
- [X] T023 [P] [US3] [REQ-016] Confirm `conformance/cases/kiro-supersession-dangling/expected.json` fails before dangling detection exists
- [X] T024 [P] [US3] [REQ-015] [REQ-017] Confirm `conformance/cases/kiro-supersession-unreciprocated/expected.json` fails before claim detection exists
- [X] T025 [P] [US3] [REQ-019] [REQ-020] Confirm `conformance/cases/kiro-amendment-extends/expected.json` fails or mismatches until amendments stay on the parent feature
- [X] T026 [P] [US3] [REQ-015] Confirm `conformance/cases/kiro-operational-supersede/expected.json` fails or mismatches until AC-level "supersede" is ignored

### Implementation for User Story 3

- [X] T027 [US3] [REQ-014] Collect successor names as longest-first discovered feature names in superseded `spec.json` notes in `crates/ears-sdd/src/kiro.rs`
- [X] T028 [US3] [REQ-015] Collect spec-level claims from requirements prose outside acceptance criteria (supersede lemma + feature name) in `crates/ears-sdd/src/kiro.rs`
- [X] T029 [US3] [REQ-016] [REQ-017] [REQ-018] Emit `KIRO_SUPERSESSION_DANGLING` and `KIRO_SUPERSESSION_UNRECIPROCATED`; accept a recut in `crates/ears-sdd/src/kiro.rs`
- [X] T030 [US3] [REQ-019] [REQ-020] Do not treat amendment `extends` as a supersession; assign amendment criteria to the parent feature in `crates/ears-sdd/src/kiro.rs`
- [X] T031 [US3] [REQ-014] [REQ-015] [REQ-016] [REQ-017] [REQ-018] [REQ-019] [REQ-020] Make `conformance/cases/kiro-supersession-recut/expected.json`, `kiro-supersession-dangling/expected.json`, `kiro-supersession-unreciprocated/expected.json`, `kiro-amendment-extends/expected.json`, and `kiro-operational-supersede/expected.json` pass

**Checkpoint**: Notes and relationship prose create links; acceptance-criteria "supersede" and `extends` do not.

---

## Phase 6: User Story 4 - An assessment that did not look for contradictions says so (Priority: P2)

**Goal**: Kiro documents do not receive Spec Kit identifier, traceability, tasks, or separation checks. The omission is named. Constraint analysis still appears in `disabled_checks`.

**Independent Test**: `kiro-discovers-tree` stays PASS with `KIRO_CHECKS_OMITTED` advisory and no `TRACE_*` / `EARS_*` / `CONFIG_MISSING` findings; human output still lists `constraints` as not checked.

### Tests for User Story 4

- [X] T032 [US4] [REQ-023] [REQ-024] Confirm `conformance/cases/kiro-discovers-tree/expected.json` requires `KIRO_CHECKS_OMITTED` and that `TRACE_MISSING` must not appear in that result

### Implementation for User Story 4

- [X] T033 [US4] [REQ-023] Skip identifier, traceability, tasks, and separation checks on `family = kiro` and emit `KIRO_CHECKS_OMITTED` in `crates/ears-sdd/src/lib.rs`
- [X] T034 [US4] [REQ-024] Keep default `provenance.disabled_checks` listing `constraints` for a Kiro-only project with no config in `crates/ears-sdd/src/lib.rs`
- [X] T035 [US4] [REQ-023] [REQ-024] Make `conformance/cases/kiro-discovers-tree/expected.json` pass with the advisory present and Spec Kit check codes absent

**Checkpoint**: A green Kiro run cannot be read as “traceability and contradictions passed.”

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Docs and corpus honesty after the stories pass.

- [X] T036 [P] Document `KIRO_*` codes in `docs/reference/findings.md` from `specs/008-kiro-assessment/contracts/findings.md`
- [X] T037 [P] Document Kiro tree discovery and `--feature` rules in `docs/reference/configuration.md` and `docs/reference/gates.md`
- [X] T038 Run the commands in `specs/008-kiro-assessment/quickstart.md` against the conformance fixtures
- [X] T039 [REQ-021] Re-run `crates/ears-sdd/tests/read_only.rs` including the Kiro fixture snapshot
- [X] T040 [REQ-003] Run the full conformance corpus and confirm every non-`kiro-*` case still matches its `expected.json`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational — MVP
- **User Story 2 (Phase 4)**: Depends on User Story 1 (needs discovered Kiro specs)
- **User Story 3 (Phase 5)**: Depends on User Story 2 (needs the active baseline)
- **User Story 4 (Phase 6)**: Depends on User Story 1 (disclosure on the same run)
- **Polish (Phase 7)**: Depends on stories 1–4

### User Story Dependencies

- **US1**: After Phase 2. No other story.
- **US2**: After US1.
- **US3**: After US2.
- **US4**: After US1; can overlap US2/US3 if `lib.rs` edits are sequenced.

### Within Each User Story

- Confirm the conformance fixture fails
- Implement in `kiro.rs` / `discovery.rs` / `lib.rs`
- Make the named `expected.json` pass
- Do not copy `REQ-NNN` into production code

### Parallel Opportunities

- T001 and T002
- T006 (regression) beside T005
- T017 beside T016
- T023, T024, T025, T026 after T022 starts
- T036 and T037

---

## Parallel Example: User Story 1

```bash
# After T005 confirms the Kiro fixtures fail:
Task: "Walk .kiro/specs in crates/ears-sdd/src/kiro.rs"
Task: "Suppress CONFIG_MISSING in crates/ears-sdd/src/lib.rs"  # after T007 lands locations
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 Setup
2. Phase 2 Foundational
3. Phase 3 User Story 1
4. **STOP**: `kiro-discovers-tree` passes; `all-features-scope` still passes; read-only holds

### Incremental Delivery

1. US1 → a Kiro tree is visible
2. US2 → historical specs leave the baseline
3. US3 → replacement is a checked link
4. US4 → a green run names what it did not check
5. Polish → docs and full corpus

---

## Notes

- Conformance fixtures were authored at plan time; they are the failing tests for US1–US4
- `ccsdd.rs` already parses G1–G4; T003 locks it, T009 wires it
- Production code must not contain `REQ-NNN` or copied EARS sentences
- Do not open `../facdrone` in CI; optional smoke only after the corpus is green
