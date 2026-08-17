# Tasks: Mirror Replication

**Feature**: `005-mirror-replication`

Every requirement below is covered by a failing-test task placed before its implementation task.
The tasks gate refuses to pass while any requirement has no covering task.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_mirror_replication.py::test_creates_job_per_artifact_mirror_pair`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_mirror_replication.py::test_replicates_signed_version_to_trusted_mirror`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_mirror_replication.py::test_skips_replication_for_unauthenticated_publisher`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_mirror_replication.py::test_skips_untrusted_mirror`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_mirror_replication.py::test_skips_mirror_with_unverified_certificate`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_mirror_replication.py::test_skips_manifest_with_invalid_signature`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_mirror_replication.py::test_skips_mirror_over_storage_quota`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_mirror_replication.py::test_freezes_replication_during_incident`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_mirror_replication.py::test_retries_failed_replication_with_backoff`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_mirror_replication.py::test_reports_mirror_degraded_after_retry_budget`.
- [ ] T011 [REQ-011] Write a failing test `tests/test_mirror_replication.py::test_mirror_stores_verified_copy`.
- [ ] T012 [REQ-012] Write a failing test `tests/test_mirror_replication.py::test_mirror_discards_digest_mismatch`.
- [ ] T013 [REQ-013] Write a failing test `tests/test_mirror_replication.py::test_rejects_md5_manifest_for_replication`.
- [ ] T014 [REQ-014] Write a failing test `tests/test_mirror_replication.py::test_accepts_sha1_manifest_for_legacy_mirror`.
- [ ] T015 [REQ-015] Write a failing test `tests/test_mirror_replication.py::test_marks_sha1_manifest_deprecated_in_catalog`.
- [ ] T016 [REQ-016] Write a failing test `tests/test_mirror_replication.py::test_catalog_records_digest_algorithm`.
- [ ] T017 [REQ-017] Write a failing test `tests/test_mirror_replication.py::test_fresh_mirror_serves_artifact`.
- [ ] T018 [REQ-018] Write a failing test `tests/test_mirror_replication.py::test_lagging_mirror_left_out_of_redirect_pool`.
- [ ] T019 [REQ-019] Write a failing test `tests/test_mirror_replication.py::test_status_endpoint_reports_lag`.
- [ ] T020 [REQ-020] Write a failing test `tests/test_mirror_replication.py::test_alert_raised_above_lag_threshold`.
- [ ] T021 [REQ-021] Write a failing test `tests/test_mirror_replication.py::test_reports_pending_replication_backlog`.
- [ ] T022 [REQ-022] Write a failing test `tests/test_mirror_replication.py::test_status_endpoint_flags_stale_index`.
- [ ] T023 [REQ-023] Write a failing test `tests/test_mirror_replication.py::test_allows_trusted_mirror_feed_pull`.
- [ ] T024 [REQ-024] Write a failing test `tests/test_mirror_replication.py::test_denies_expired_mirror_token`.
- [ ] T025 [REQ-025] Write a failing test `tests/test_mirror_replication.py::test_mirror_retains_artifact_under_hold`.
- [ ] T026 [REQ-026] Write a failing test `tests/test_mirror_replication.py::test_mirror_deletes_cold_copy`.
- [ ] T027 [REQ-027] Write a failing test `tests/test_mirror_replication.py::test_origin_retains_all_versions`.
- [ ] T028 [REQ-028] Write a failing test `tests/test_mirror_replication.py::test_yank_marker_propagates_to_mirrors`.
- [ ] T029 [REQ-029] Write a failing test `tests/test_mirror_replication.py::test_omits_mirror_urls_for_unsupported_client`.
- [ ] T030 [REQ-030] Write a failing test `tests/test_mirror_replication.py::test_defers_catalog_rebuild_in_maintenance_window`.
- [ ] T031 [REQ-031] Write a failing test `tests/test_mirror_replication.py::test_audit_entry_per_replication_attempt`.
- [ ] T032 [REQ-032] Write a failing test `tests/test_mirror_replication.py::test_buffers_audit_entries_when_sink_unavailable`.

## Phase 2: Implementation

- [ ] T033 [REQ-001] Implement until `tests/test_mirror_replication.py::test_creates_job_per_artifact_mirror_pair` passes.
- [ ] T034 [REQ-002] Implement until `tests/test_mirror_replication.py::test_replicates_signed_version_to_trusted_mirror` passes.
- [ ] T035 [REQ-003] Implement until `tests/test_mirror_replication.py::test_skips_replication_for_unauthenticated_publisher` passes.
- [ ] T036 [REQ-004] Implement until `tests/test_mirror_replication.py::test_skips_untrusted_mirror` passes.
- [ ] T037 [REQ-005] Implement until `tests/test_mirror_replication.py::test_skips_mirror_with_unverified_certificate` passes.
- [ ] T038 [REQ-006] Implement until `tests/test_mirror_replication.py::test_skips_manifest_with_invalid_signature` passes.
- [ ] T039 [REQ-007] Implement until `tests/test_mirror_replication.py::test_skips_mirror_over_storage_quota` passes.
- [ ] T040 [REQ-008] Implement until `tests/test_mirror_replication.py::test_freezes_replication_during_incident` passes.
- [ ] T041 [REQ-009] Implement until `tests/test_mirror_replication.py::test_retries_failed_replication_with_backoff` passes.
- [ ] T042 [REQ-010] Implement until `tests/test_mirror_replication.py::test_reports_mirror_degraded_after_retry_budget` passes.
- [ ] T043 [REQ-011] Implement until `tests/test_mirror_replication.py::test_mirror_stores_verified_copy` passes.
- [ ] T044 [REQ-012] Implement until `tests/test_mirror_replication.py::test_mirror_discards_digest_mismatch` passes.
- [ ] T045 [REQ-013] Implement until `tests/test_mirror_replication.py::test_rejects_md5_manifest_for_replication` passes.
- [ ] T046 [REQ-014] Implement until `tests/test_mirror_replication.py::test_accepts_sha1_manifest_for_legacy_mirror` passes.
- [ ] T047 [REQ-015] Implement until `tests/test_mirror_replication.py::test_marks_sha1_manifest_deprecated_in_catalog` passes.
- [ ] T048 [REQ-016] Implement until `tests/test_mirror_replication.py::test_catalog_records_digest_algorithm` passes.
- [ ] T049 [REQ-017] Implement until `tests/test_mirror_replication.py::test_fresh_mirror_serves_artifact` passes.
- [ ] T050 [REQ-018] Implement until `tests/test_mirror_replication.py::test_lagging_mirror_left_out_of_redirect_pool` passes.
- [ ] T051 [REQ-019] Implement until `tests/test_mirror_replication.py::test_status_endpoint_reports_lag` passes.
- [ ] T052 [REQ-020] Implement until `tests/test_mirror_replication.py::test_alert_raised_above_lag_threshold` passes.
- [ ] T053 [REQ-021] Implement until `tests/test_mirror_replication.py::test_reports_pending_replication_backlog` passes.
- [ ] T054 [REQ-022] Implement until `tests/test_mirror_replication.py::test_status_endpoint_flags_stale_index` passes.
- [ ] T055 [REQ-023] Implement until `tests/test_mirror_replication.py::test_allows_trusted_mirror_feed_pull` passes.
- [ ] T056 [REQ-024] Implement until `tests/test_mirror_replication.py::test_denies_expired_mirror_token` passes.
- [ ] T057 [REQ-025] Implement until `tests/test_mirror_replication.py::test_mirror_retains_artifact_under_hold` passes.
- [ ] T058 [REQ-026] Implement until `tests/test_mirror_replication.py::test_mirror_deletes_cold_copy` passes.
- [ ] T059 [REQ-027] Implement until `tests/test_mirror_replication.py::test_origin_retains_all_versions` passes.
- [ ] T060 [REQ-028] Implement until `tests/test_mirror_replication.py::test_yank_marker_propagates_to_mirrors` passes.
- [ ] T061 [REQ-029] Implement until `tests/test_mirror_replication.py::test_omits_mirror_urls_for_unsupported_client` passes.
- [ ] T062 [REQ-030] Implement until `tests/test_mirror_replication.py::test_defers_catalog_rebuild_in_maintenance_window` passes.
- [ ] T063 [REQ-031] Implement until `tests/test_mirror_replication.py::test_audit_entry_per_replication_attempt` passes.
- [ ] T064 [REQ-032] Implement until `tests/test_mirror_replication.py::test_buffers_audit_entries_when_sink_unavailable` passes.
