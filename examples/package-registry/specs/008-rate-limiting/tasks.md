# Tasks: Rate Limiting

**Feature**: `008-rate-limiting`

Every requirement below is covered by a failing-test task placed before its implementation task.
The tasks gate refuses to pass while any requirement has no covering task.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_rate_limiting.py::test_identity_falls_back_to_network_prefix`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_rate_limiting.py::test_budget_is_enforced_over_rolling_window`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_rate_limiting.py::test_allowance_increases_with_token_scope`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_rate_limiting.py::test_unsupported_client_gets_anonymous_allowance`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_rate_limiting.py::test_expired_token_charged_to_anonymous_budget`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_rate_limiting.py::test_over_budget_client_is_denied_429`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_rate_limiting.py::test_read_within_budget_is_admitted_unqueued`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_rate_limiting.py::test_admin_token_bypasses_exhausted_budget`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_rate_limiting.py::test_unverified_tls_denied_before_accounting`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_rate_limiting.py::test_denied_response_carries_retry_after`.
- [ ] T011 [REQ-011] Write a failing test `tests/test_rate_limiting.py::test_budget_headers_present_on_every_response`.
- [ ] T012 [REQ-012] Write a failing test `tests/test_rate_limiting.py::test_denied_request_does_not_consume_allowance`.
- [ ] T013 [REQ-013] Write a failing test `tests/test_rate_limiting.py::test_throttle_decision_is_audited`.
- [ ] T014 [REQ-014] Write a failing test `tests/test_rate_limiting.py::test_admin_bypass_is_audited`.
- [ ] T015 [REQ-015] Write a failing test `tests/test_rate_limiting.py::test_throttle_entries_buffered_when_sink_down`.
- [ ] T016 [REQ-016] Write a failing test `tests/test_rate_limiting.py::test_writes_shed_during_incident`.
- [ ] T017 [REQ-017] Write a failing test `tests/test_rate_limiting.py::test_writes_denied_503_during_maintenance`.
- [ ] T018 [REQ-018] Write a failing test `tests/test_rate_limiting.py::test_downloads_continue_during_incident`.
- [ ] T019 [REQ-019] Write a failing test `tests/test_rate_limiting.py::test_over_budget_download_is_withheld_whole`.
- [ ] T020 [REQ-020] Write a failing test `tests/test_rate_limiting.py::test_over_budget_upload_rejected_before_storage`.
- [ ] T021 [REQ-021] Write a failing test `tests/test_rate_limiting.py::test_upload_within_budget_accepted_for_validation`.
- [ ] T022 [REQ-022] Write a failing test `tests/test_rate_limiting.py::test_quota_exhausted_upload_rejected_before_budget`.
- [ ] T023 [REQ-023] Write a failing test `tests/test_rate_limiting.py::test_lagging_mirror_stops_accepting_batches`.
- [ ] T024 [REQ-024] Write a failing test `tests/test_rate_limiting.py::test_replication_paused_during_incident`.
- [ ] T025 [REQ-025] Write a failing test `tests/test_rate_limiting.py::test_healthy_mirror_receives_replication`.
- [ ] T026 [REQ-026] Write a failing test `tests/test_rate_limiting.py::test_indexing_deferred_during_incident`.
- [ ] T027 [REQ-027] Write a failing test `tests/test_rate_limiting.py::test_deferred_artifacts_indexed_after_incident`.
- [ ] T028 [REQ-028] Write a failing test `tests/test_rate_limiting.py::test_window_uses_monotonic_clock`.
- [ ] T029 [REQ-029] Write a failing test `tests/test_rate_limiting.py::test_window_resets_are_staggered`.
- [ ] T030 [REQ-030] Write a failing test `tests/test_rate_limiting.py::test_counter_store_loss_falls_back_to_local_budgets`.
- [ ] T031 [REQ-031] Write a failing test `tests/test_rate_limiting.py::test_concurrent_upload_connections_capped`.
- [ ] T032 [REQ-032] Write a failing test `tests/test_rate_limiting.py::test_per_scope_counters_are_exported`.

## Phase 2: Implementation

- [ ] T033 [REQ-001] Implement until `tests/test_rate_limiting.py::test_identity_falls_back_to_network_prefix` passes.
- [ ] T034 [REQ-002] Implement until `tests/test_rate_limiting.py::test_budget_is_enforced_over_rolling_window` passes.
- [ ] T035 [REQ-003] Implement until `tests/test_rate_limiting.py::test_allowance_increases_with_token_scope` passes.
- [ ] T036 [REQ-004] Implement until `tests/test_rate_limiting.py::test_unsupported_client_gets_anonymous_allowance` passes.
- [ ] T037 [REQ-005] Implement until `tests/test_rate_limiting.py::test_expired_token_charged_to_anonymous_budget` passes.
- [ ] T038 [REQ-006] Implement until `tests/test_rate_limiting.py::test_over_budget_client_is_denied_429` passes.
- [ ] T039 [REQ-007] Implement until `tests/test_rate_limiting.py::test_read_within_budget_is_admitted_unqueued` passes.
- [ ] T040 [REQ-008] Implement until `tests/test_rate_limiting.py::test_admin_token_bypasses_exhausted_budget` passes.
- [ ] T041 [REQ-009] Implement until `tests/test_rate_limiting.py::test_unverified_tls_denied_before_accounting` passes.
- [ ] T042 [REQ-010] Implement until `tests/test_rate_limiting.py::test_denied_response_carries_retry_after` passes.
- [ ] T043 [REQ-011] Implement until `tests/test_rate_limiting.py::test_budget_headers_present_on_every_response` passes.
- [ ] T044 [REQ-012] Implement until `tests/test_rate_limiting.py::test_denied_request_does_not_consume_allowance` passes.
- [ ] T045 [REQ-013] Implement until `tests/test_rate_limiting.py::test_throttle_decision_is_audited` passes.
- [ ] T046 [REQ-014] Implement until `tests/test_rate_limiting.py::test_admin_bypass_is_audited` passes.
- [ ] T047 [REQ-015] Implement until `tests/test_rate_limiting.py::test_throttle_entries_buffered_when_sink_down` passes.
- [ ] T048 [REQ-016] Implement until `tests/test_rate_limiting.py::test_writes_shed_during_incident` passes.
- [ ] T049 [REQ-017] Implement until `tests/test_rate_limiting.py::test_writes_denied_503_during_maintenance` passes.
- [ ] T050 [REQ-018] Implement until `tests/test_rate_limiting.py::test_downloads_continue_during_incident` passes.
- [ ] T051 [REQ-019] Implement until `tests/test_rate_limiting.py::test_over_budget_download_is_withheld_whole` passes.
- [ ] T052 [REQ-020] Implement until `tests/test_rate_limiting.py::test_over_budget_upload_rejected_before_storage` passes.
- [ ] T053 [REQ-021] Implement until `tests/test_rate_limiting.py::test_upload_within_budget_accepted_for_validation` passes.
- [ ] T054 [REQ-022] Implement until `tests/test_rate_limiting.py::test_quota_exhausted_upload_rejected_before_budget` passes.
- [ ] T055 [REQ-023] Implement until `tests/test_rate_limiting.py::test_lagging_mirror_stops_accepting_batches` passes.
- [ ] T056 [REQ-024] Implement until `tests/test_rate_limiting.py::test_replication_paused_during_incident` passes.
- [ ] T057 [REQ-025] Implement until `tests/test_rate_limiting.py::test_healthy_mirror_receives_replication` passes.
- [ ] T058 [REQ-026] Implement until `tests/test_rate_limiting.py::test_indexing_deferred_during_incident` passes.
- [ ] T059 [REQ-027] Implement until `tests/test_rate_limiting.py::test_deferred_artifacts_indexed_after_incident` passes.
- [ ] T060 [REQ-028] Implement until `tests/test_rate_limiting.py::test_window_uses_monotonic_clock` passes.
- [ ] T061 [REQ-029] Implement until `tests/test_rate_limiting.py::test_window_resets_are_staggered` passes.
- [ ] T062 [REQ-030] Implement until `tests/test_rate_limiting.py::test_counter_store_loss_falls_back_to_local_budgets` passes.
- [ ] T063 [REQ-031] Implement until `tests/test_rate_limiting.py::test_concurrent_upload_connections_capped` passes.
- [ ] T064 [REQ-032] Implement until `tests/test_rate_limiting.py::test_per_scope_counters_are_exported` passes.
