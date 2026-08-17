# Tasks: Retention and Collection

**Feature**: `006-retention-and-collection`

Every requirement below is covered by a failing-test task placed before its implementation task.
The tasks gate refuses to pass while any requirement has no covering task.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_retention_policy.py::test_policy_names_every_threshold`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_collection.py::test_deletes_unused_artifact_past_retention_window`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_collection.py::test_deletes_yanked_artifact_after_grace_period`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_collection.py::test_quota_pressure_shortens_retention_window`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_retention.py::test_retains_artifact_inside_retention_window`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_retention.py::test_retains_downloaded_artifact`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_holds.py::test_hold_survives_quota_pressure`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_retention.py::test_yank_grace_period_keeps_pinned_builds_resolvable`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_collection.py::test_incident_pauses_deletion`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_collection.py::test_no_deletion_without_audit_sink`.
- [ ] T011 [REQ-011] Write a failing test `tests/test_collection.py::test_referenced_artifact_is_never_collected`.
- [ ] T012 [REQ-012] Write a failing test `tests/test_manifest.py::test_manifest_lists_referenced_digests`.
- [ ] T013 [REQ-013] Write a failing test `tests/test_manifest.py::test_broken_reference_is_reported`.
- [ ] T014 [REQ-014] Write a failing test `tests/test_serving.py::test_withholds_artifact_with_digest_mismatch`.
- [ ] T015 [REQ-015] Write a failing test `tests/test_serving.py::test_yanked_artifact_still_served_by_exact_version`.
- [ ] T016 [REQ-016] Write a failing test `tests/test_holds.py::test_admin_token_can_place_hold`.
- [ ] T017 [REQ-017] Write a failing test `tests/test_holds.py::test_write_token_cannot_place_hold`.
- [ ] T018 [REQ-018] Write a failing test `tests/test_holds.py::test_hold_record_carries_owner_reason_and_expiry`.
- [ ] T019 [REQ-019] Write a failing test `tests/test_holds.py::test_expired_hold_is_reported_not_auto_lifted`.
- [ ] T020 [REQ-020] Write a failing test `tests/test_reports.py::test_hold_listing_includes_age_and_owner`.
- [ ] T021 [REQ-021] Write a failing test `tests/test_audit.py::test_retain_decisions_are_audited`.
- [ ] T022 [REQ-022] Write a failing test `tests/test_audit.py::test_deletion_entry_names_rule_and_threshold`.
- [ ] T023 [REQ-023] Write a failing test `tests/test_audit.py::test_deletion_entry_records_digest_and_algorithm`.
- [ ] T024 [REQ-024] Write a failing test `tests/test_audit.py::test_deletion_entry_records_observed_download_count`.
- [ ] T025 [REQ-025] Write a failing test `tests/test_audit.py::test_deletion_entries_outlive_the_artifact`.
- [ ] T026 [REQ-026] Write a failing test `tests/test_replication.py::test_tombstone_reaches_trusted_mirror`.
- [ ] T027 [REQ-027] Write a failing test `tests/test_replication.py::test_legacy_mirror_receives_no_tombstone`.
- [ ] T028 [REQ-028] Write a failing test `tests/test_replication.py::test_mirror_reports_last_applied_sweep`.
- [ ] T029 [REQ-029] Write a failing test `tests/test_index.py::test_index_rebuilt_after_collection`.
- [ ] T030 [REQ-030] Write a failing test `tests/test_index.py::test_index_rebuild_deferred_during_maintenance`.
- [ ] T031 [REQ-031] Write a failing test `tests/test_reports.py::test_dry_run_deletes_nothing`.
- [ ] T032 [REQ-032] Write a failing test `tests/test_reports.py::test_sweep_report_groups_counts_by_reason`.

## Phase 2: Implementation

- [ ] T033 [REQ-001] Implement until `tests/test_retention_policy.py::test_policy_names_every_threshold` passes.
- [ ] T034 [REQ-002] Implement until `tests/test_collection.py::test_deletes_unused_artifact_past_retention_window` passes.
- [ ] T035 [REQ-003] Implement until `tests/test_collection.py::test_deletes_yanked_artifact_after_grace_period` passes.
- [ ] T036 [REQ-004] Implement until `tests/test_collection.py::test_quota_pressure_shortens_retention_window` passes.
- [ ] T037 [REQ-005] Implement until `tests/test_retention.py::test_retains_artifact_inside_retention_window` passes.
- [ ] T038 [REQ-006] Implement until `tests/test_retention.py::test_retains_downloaded_artifact` passes.
- [ ] T039 [REQ-007] Implement until `tests/test_holds.py::test_hold_survives_quota_pressure` passes.
- [ ] T040 [REQ-008] Implement until `tests/test_retention.py::test_yank_grace_period_keeps_pinned_builds_resolvable` passes.
- [ ] T041 [REQ-009] Implement until `tests/test_collection.py::test_incident_pauses_deletion` passes.
- [ ] T042 [REQ-010] Implement until `tests/test_collection.py::test_no_deletion_without_audit_sink` passes.
- [ ] T043 [REQ-011] Implement until `tests/test_collection.py::test_referenced_artifact_is_never_collected` passes.
- [ ] T044 [REQ-012] Implement until `tests/test_manifest.py::test_manifest_lists_referenced_digests` passes.
- [ ] T045 [REQ-013] Implement until `tests/test_manifest.py::test_broken_reference_is_reported` passes.
- [ ] T046 [REQ-014] Implement until `tests/test_serving.py::test_withholds_artifact_with_digest_mismatch` passes.
- [ ] T047 [REQ-015] Implement until `tests/test_serving.py::test_yanked_artifact_still_served_by_exact_version` passes.
- [ ] T048 [REQ-016] Implement until `tests/test_holds.py::test_admin_token_can_place_hold` passes.
- [ ] T049 [REQ-017] Implement until `tests/test_holds.py::test_write_token_cannot_place_hold` passes.
- [ ] T050 [REQ-018] Implement until `tests/test_holds.py::test_hold_record_carries_owner_reason_and_expiry` passes.
- [ ] T051 [REQ-019] Implement until `tests/test_holds.py::test_expired_hold_is_reported_not_auto_lifted` passes.
- [ ] T052 [REQ-020] Implement until `tests/test_reports.py::test_hold_listing_includes_age_and_owner` passes.
- [ ] T053 [REQ-021] Implement until `tests/test_audit.py::test_retain_decisions_are_audited` passes.
- [ ] T054 [REQ-022] Implement until `tests/test_audit.py::test_deletion_entry_names_rule_and_threshold` passes.
- [ ] T055 [REQ-023] Implement until `tests/test_audit.py::test_deletion_entry_records_digest_and_algorithm` passes.
- [ ] T056 [REQ-024] Implement until `tests/test_audit.py::test_deletion_entry_records_observed_download_count` passes.
- [ ] T057 [REQ-025] Implement until `tests/test_audit.py::test_deletion_entries_outlive_the_artifact` passes.
- [ ] T058 [REQ-026] Implement until `tests/test_replication.py::test_tombstone_reaches_trusted_mirror` passes.
- [ ] T059 [REQ-027] Implement until `tests/test_replication.py::test_legacy_mirror_receives_no_tombstone` passes.
- [ ] T060 [REQ-028] Implement until `tests/test_replication.py::test_mirror_reports_last_applied_sweep` passes.
- [ ] T061 [REQ-029] Implement until `tests/test_index.py::test_index_rebuilt_after_collection` passes.
- [ ] T062 [REQ-030] Implement until `tests/test_index.py::test_index_rebuild_deferred_during_maintenance` passes.
- [ ] T063 [REQ-031] Implement until `tests/test_reports.py::test_dry_run_deletes_nothing` passes.
- [ ] T064 [REQ-032] Implement until `tests/test_reports.py::test_sweep_report_groups_counts_by_reason` passes.
