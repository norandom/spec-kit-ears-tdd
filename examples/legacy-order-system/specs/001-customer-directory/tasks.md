# Tasks: Customer Directory

**Feature**: `001-customer-directory`

Brownfield, so most of this code exists. A task here is a characterisation test that pins current
behaviour, followed by whatever change the specification turned out to require.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_directory.py::test_one_address_per_customer`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_directory.py::test_address_is_replaced_in_place`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_directory.py::test_replacement_time_is_recorded`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_erasure.py::test_erasure_clears_every_store`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_erasure.py::test_erasure_defers_until_order_completes`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_erasure.py::test_erasure_time_is_recorded`.
- [ ] T007 [REQ-007] Review the customer-facing erasure report with the privacy team.
- [ ] T008 [REQ-008] Write a failing test `tests/test_directory.py::test_invalid_address_is_rejected`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_directory.py::test_missing_address_reports_incomplete`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_directory.py::test_address_read_requires_scope`.

## Phase 2: Implementation

- [ ] T011 [REQ-001] Implement until `test_one_address_per_customer` passes.
- [ ] T012 [REQ-002] Implement until `test_address_is_replaced_in_place` passes.
- [ ] T013 [REQ-003] Implement until `test_replacement_time_is_recorded` passes.
- [ ] T014 [REQ-004] Implement until `test_erasure_clears_every_store` passes.
- [ ] T015 [REQ-005] Implement until `test_erasure_defers_until_order_completes` passes.
- [ ] T016 [REQ-006] Implement until `test_erasure_time_is_recorded` passes.
- [ ] T017 [REQ-007] Record the privacy team's review outcome.
- [ ] T018 [REQ-008] Implement until `test_invalid_address_is_rejected` passes.
- [ ] T019 [REQ-009] Implement until `test_missing_address_reports_incomplete` passes.
- [ ] T020 [REQ-010] Implement until `test_address_read_requires_scope` passes.
