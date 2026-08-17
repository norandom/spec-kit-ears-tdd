# Tasks: Shipment Archive

**Feature**: `002-shipment-archive`

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_archive.py::test_dispatch_records_address_as_it_stood`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_archive.py::test_completed_shipment_retained_seven_years`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_archive.py::test_recorded_address_is_immutable`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_retention.py::test_expired_shipment_address_is_deleted`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_retention.py::test_open_dispute_holds_address`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_archive.py::test_dispute_produces_dispatch_address`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_archive.py::test_address_is_copied_not_referenced`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_archive.py::test_dispatch_time_is_recorded`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_archive.py::test_address_read_requires_fulfilment_scope`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_archive.py::test_shipment_without_outcome_is_in_flight`.

## Phase 2: Implementation

- [ ] T011 [REQ-001] Implement until `test_dispatch_records_address_as_it_stood` passes.
- [ ] T012 [REQ-002] Implement until `test_completed_shipment_retained_seven_years` passes.
- [ ] T013 [REQ-003] Implement until `test_recorded_address_is_immutable` passes.
- [ ] T014 [REQ-004] Implement until `test_expired_shipment_address_is_deleted` passes.
- [ ] T015 [REQ-005] Implement until `test_open_dispute_holds_address` passes.
- [ ] T016 [REQ-006] Implement until `test_dispute_produces_dispatch_address` passes.
- [ ] T017 [REQ-007] Implement until `test_address_is_copied_not_referenced` passes.
- [ ] T018 [REQ-008] Implement until `test_dispatch_time_is_recorded` passes.
- [ ] T019 [REQ-009] Implement until `test_address_read_requires_fulfilment_scope` passes.
- [ ] T020 [REQ-010] Implement until `test_shipment_without_outcome_is_in_flight` passes.
