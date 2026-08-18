# Tasks: Selective Adoption

**Feature**: `007-selective-adoption`

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test for reading the check configuration.
- [ ] T002 [REQ-002] Write a failing test for every check defaulting on.
- [ ] T003 [REQ-003] Write a failing test for a disabled check omitting its findings.
- [ ] T004 [REQ-004] Write a failing test for the human-readable disabled list.
- [ ] T005 [REQ-005] Write a failing test for the machine-readable disabled list.
- [ ] T006 [REQ-006] Write a failing test for disclosure on a passing run.
- [ ] T007 [REQ-007] Write a failing test for an unrecognised check key.
- [ ] T008 [REQ-008] Write a failing test for constraints being skipped wholesale.
- [ ] T009 [REQ-009] Write a failing test for the stable order of the disabled list.
- [ ] T010 [REQ-010] Write a failing test for one switch leaving the others alone.

## Phase 2: Implementation

- [ ] T011 [REQ-001] Add the check table to the typed configuration.
- [ ] T012 [REQ-002] Default every check on.
- [ ] T013 [REQ-003] Gate each layer on its switch.
- [ ] T014 [REQ-004] Print the disabled list beneath the scope line.
- [ ] T015 [REQ-005] Record the disabled list in provenance.
- [ ] T016 [REQ-006] Print it irrespective of outcome.
- [ ] T017 [REQ-007] Refuse unknown keys in the check table.
- [ ] T018 [REQ-008] Skip the constraint layer for every specification together.
- [ ] T019 [REQ-009] Emit the disabled list in declaration-independent order.
- [ ] T020 [REQ-010] Keep the switches independent of one another.
- [ ] T021 [REQ-011] Write a failing test for the advanced layers defaulting off.
- [ ] T022 [REQ-012] Write a failing test for constraints without vocabulary.
- [ ] T023 [REQ-013] Write a failing test for an enabled vocabulary with nothing declared.
- [ ] T024 [REQ-014] Write a failing test for an enabled constraint check with no models.
- [ ] T025 [REQ-011] Default the vocabulary and constraint checks off.
- [ ] T026 [REQ-012] Refuse the incoherent combination.
- [ ] T027 [REQ-013] Report an enabled vocabulary layer with no vocabulary.
- [ ] T028 [REQ-014] Report an enabled constraint layer with no models.
