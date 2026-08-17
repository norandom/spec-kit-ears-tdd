# Tasks: Maintenance Operations

**Feature**: `012-maintenance-operations`

Every requirement below is covered by a failing-test task placed before its implementation task.
The tasks gate refuses to pass while any requirement has no covering task.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_maintenance_reads.py::test_serves_published_releases_during_window`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_maintenance_reads.py::test_withholds_yanked_artifact_during_window`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_maintenance_reads.py::test_withholds_artifact_whose_stored_digest_drifted`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_maintenance_reads.py::test_download_response_carries_window_header`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_maintenance_reads.py::test_unsupported_client_receives_upgrade_notice`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_maintenance_access.py::test_allows_read_traffic_during_window`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_maintenance_access.py::test_allows_admin_break_glass_during_incident`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_maintenance_access.py::test_denies_write_scoped_requests_during_window`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_maintenance_access.py::test_denies_expired_token_during_window`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_maintenance_access.py::test_denies_rate_limited_client_during_window`.
- [ ] T011 [REQ-011] Write a failing test `tests/test_maintenance_access.py::test_denied_write_carries_retry_after_window_end`.
- [ ] T012 [REQ-012] Write a failing test `tests/test_maintenance_audit.py::test_denied_request_is_recorded_with_scope_and_window`.
- [ ] T013 [REQ-013] Write a failing test `tests/test_maintenance_publish.py::test_rejects_ordinary_publish_during_window`.
- [ ] T014 [REQ-014] Write a failing test `tests/test_maintenance_publish.py::test_rejects_unsigned_upload_during_window`.
- [ ] T015 [REQ-015] Write a failing test `tests/test_maintenance_publish.py::test_rejects_weak_digest_algorithm_during_window`.
- [ ] T016 [REQ-016] Write a failing test `tests/test_maintenance_publish.py::test_accepts_admin_hotfix_during_incident`.
- [ ] T017 [REQ-017] Write a failing test `tests/test_maintenance_publish.py::test_discards_upload_on_digest_mismatch`.
- [ ] T018 [REQ-018] Write a failing test `tests/test_maintenance_audit.py::test_spools_audit_entries_while_sink_is_down`.
- [ ] T019 [REQ-019] Write a failing test `tests/test_maintenance_audit.py::test_hotfix_publish_is_attributed_to_operator_and_incident`.
- [ ] T020 [REQ-020] Write a failing test `tests/test_maintenance_reclamation.py::test_deletes_eligible_yanked_artifacts`.
- [ ] T021 [REQ-021] Write a failing test `tests/test_maintenance_reclamation.py::test_retains_artifact_under_retention_hold`.
- [ ] T022 [REQ-022] Write a failing test `tests/test_maintenance_reclamation.py::test_publishes_reclamation_dry_run_before_window`.
- [ ] T023 [REQ-023] Write a failing test `tests/test_maintenance_audit.py::test_deletion_entry_carries_digest_and_age`.
- [ ] T024 [REQ-024] Write a failing test `tests/test_maintenance_replication.py::test_catches_up_lagging_trusted_mirror`.
- [ ] T025 [REQ-025] Write a failing test `tests/test_maintenance_replication.py::test_skips_untrusted_mirror_during_window`.
- [ ] T026 [REQ-026] Write a failing test `tests/test_maintenance_replication.py::test_window_report_lists_skipped_legacy_mirrors`.
- [ ] T027 [REQ-027] Write a failing test `tests/test_maintenance_replication.py::test_alerts_when_mirror_lag_exceeds_an_hour`.
- [ ] T028 [REQ-028] Write a failing test `tests/test_maintenance_index.py::test_defers_index_updates_during_window`.
- [ ] T029 [REQ-029] Write a failing test `tests/test_maintenance_index.py::test_stale_search_response_reports_snapshot_age`.
- [ ] T030 [REQ-030] Write a failing test `tests/test_maintenance_audit.py::test_admin_action_emits_audit_entry`.
- [ ] T031 [REQ-031] Write a failing test `tests/test_maintenance_schedule.py::test_window_schedule_published_in_advance`.
- [ ] T032 [REQ-032] Write a failing test `tests/test_maintenance_schedule.py::test_window_close_report_counts_each_action`.

## Phase 2: Implementation

- [ ] T033 [REQ-001] Implement until `tests/test_maintenance_reads.py::test_serves_published_releases_during_window` passes.
- [ ] T034 [REQ-002] Implement until `tests/test_maintenance_reads.py::test_withholds_yanked_artifact_during_window` passes.
- [ ] T035 [REQ-003] Implement until `tests/test_maintenance_reads.py::test_withholds_artifact_whose_stored_digest_drifted` passes.
- [ ] T036 [REQ-004] Implement until `tests/test_maintenance_reads.py::test_download_response_carries_window_header` passes.
- [ ] T037 [REQ-005] Implement until `tests/test_maintenance_reads.py::test_unsupported_client_receives_upgrade_notice` passes.
- [ ] T038 [REQ-006] Implement until `tests/test_maintenance_access.py::test_allows_read_traffic_during_window` passes.
- [ ] T039 [REQ-007] Implement until `tests/test_maintenance_access.py::test_allows_admin_break_glass_during_incident` passes.
- [ ] T040 [REQ-008] Implement until `tests/test_maintenance_access.py::test_denies_write_scoped_requests_during_window` passes.
- [ ] T041 [REQ-009] Implement until `tests/test_maintenance_access.py::test_denies_expired_token_during_window` passes.
- [ ] T042 [REQ-010] Implement until `tests/test_maintenance_access.py::test_denies_rate_limited_client_during_window` passes.
- [ ] T043 [REQ-011] Implement until `tests/test_maintenance_access.py::test_denied_write_carries_retry_after_window_end` passes.
- [ ] T044 [REQ-012] Implement until `tests/test_maintenance_audit.py::test_denied_request_is_recorded_with_scope_and_window` passes.
- [ ] T045 [REQ-013] Implement until `tests/test_maintenance_publish.py::test_rejects_ordinary_publish_during_window` passes.
- [ ] T046 [REQ-014] Implement until `tests/test_maintenance_publish.py::test_rejects_unsigned_upload_during_window` passes.
- [ ] T047 [REQ-015] Implement until `tests/test_maintenance_publish.py::test_rejects_weak_digest_algorithm_during_window` passes.
- [ ] T048 [REQ-016] Implement until `tests/test_maintenance_publish.py::test_accepts_admin_hotfix_during_incident` passes.
- [ ] T049 [REQ-017] Implement until `tests/test_maintenance_publish.py::test_discards_upload_on_digest_mismatch` passes.
- [ ] T050 [REQ-018] Implement until `tests/test_maintenance_audit.py::test_spools_audit_entries_while_sink_is_down` passes.
- [ ] T051 [REQ-019] Implement until `tests/test_maintenance_audit.py::test_hotfix_publish_is_attributed_to_operator_and_incident` passes.
- [ ] T052 [REQ-020] Implement until `tests/test_maintenance_reclamation.py::test_deletes_eligible_yanked_artifacts` passes.
- [ ] T053 [REQ-021] Implement until `tests/test_maintenance_reclamation.py::test_retains_artifact_under_retention_hold` passes.
- [ ] T054 [REQ-022] Implement until `tests/test_maintenance_reclamation.py::test_publishes_reclamation_dry_run_before_window` passes.
- [ ] T055 [REQ-023] Implement until `tests/test_maintenance_audit.py::test_deletion_entry_carries_digest_and_age` passes.
- [ ] T056 [REQ-024] Implement until `tests/test_maintenance_replication.py::test_catches_up_lagging_trusted_mirror` passes.
- [ ] T057 [REQ-025] Implement until `tests/test_maintenance_replication.py::test_skips_untrusted_mirror_during_window` passes.
- [ ] T058 [REQ-026] Implement until `tests/test_maintenance_replication.py::test_window_report_lists_skipped_legacy_mirrors` passes.
- [ ] T059 [REQ-027] Implement until `tests/test_maintenance_replication.py::test_alerts_when_mirror_lag_exceeds_an_hour` passes.
- [ ] T060 [REQ-028] Implement until `tests/test_maintenance_index.py::test_defers_index_updates_during_window` passes.
- [ ] T061 [REQ-029] Implement until `tests/test_maintenance_index.py::test_stale_search_response_reports_snapshot_age` passes.
- [ ] T062 [REQ-030] Implement until `tests/test_maintenance_audit.py::test_admin_action_emits_audit_entry` passes.
- [ ] T063 [REQ-031] Implement until `tests/test_maintenance_schedule.py::test_window_schedule_published_in_advance` passes.
- [ ] T064 [REQ-032] Implement until `tests/test_maintenance_schedule.py::test_window_close_report_counts_each_action` passes.
