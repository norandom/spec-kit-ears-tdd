# Tasks: Audit Logging

**Feature**: `004-audit-logging`

Every requirement below is covered by a failing-test task placed before its implementation task.
The tasks gate refuses to pass while any requirement has no covering task.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_audit_entry.py::test_every_state_change_writes_an_entry`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_audit_entry.py::test_entry_names_actor_token_target_action_and_outcome`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_audit_entry.py::test_entry_records_token_id_not_token_value`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_audit_entry.py::test_sequence_numbers_increase_by_one`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_audit_entry.py::test_entry_chains_to_predecessor_digest`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_audit_entry.py::test_timestamps_are_utc_with_millisecond_precision`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_audit_entry.py::test_rewrite_of_a_written_entry_is_rejected`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_audit_entry.py::test_sealed_segment_digest_is_published`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_audit_durability.py::test_publication_entry_commits_before_accept`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_audit_events.py::test_digest_mismatch_rejection_records_both_digests`.
- [ ] T011 [REQ-011] Write a failing test `tests/test_audit_events.py::test_signature_presence_and_validity_are_recorded`.
- [ ] T012 [REQ-012] Write a failing test `tests/test_audit_events.py::test_yank_records_actor_and_reason`.
- [ ] T013 [REQ-013] Write a failing test `tests/test_audit_events.py::test_token_lifecycle_change_records_scope`.
- [ ] T014 [REQ-014] Write a failing test `tests/test_audit_events.py::test_legacy_mirror_admission_records_origin`.
- [ ] T015 [REQ-015] Write a failing test `tests/test_audit_durability.py::test_entry_is_durable_before_acknowledgement`.
- [ ] T016 [REQ-016] Write a failing test `tests/test_audit_outage.py::test_entries_spool_locally_while_sink_is_down`.
- [ ] T017 [REQ-017] Write a failing test `tests/test_audit_outage.py::test_publication_is_rejected_when_spool_quota_is_exhausted`.
- [ ] T018 [REQ-018] Write a failing test `tests/test_audit_outage.py::test_admin_requests_are_denied_when_spool_quota_is_exhausted`.
- [ ] T019 [REQ-019] Write a failing test `tests/test_audit_outage.py::test_downloads_continue_while_sink_is_down`.
- [ ] T020 [REQ-020] Write a failing test `tests/test_audit_outage.py::test_indexing_is_deferred_while_sink_is_down`.
- [ ] T021 [REQ-021] Write a failing test `tests/test_audit_recovery.py::test_spool_drains_in_sequence_order`.
- [ ] T022 [REQ-022] Write a failing test `tests/test_audit_recovery.py::test_drain_does_not_duplicate_recorded_entries`.
- [ ] T023 [REQ-023] Write a failing test `tests/test_audit_recovery.py::test_recovered_entries_are_indexed_before_log_reports_current`.
- [ ] T024 [REQ-024] Write a failing test `tests/test_audit_outage.py::test_health_endpoint_reports_spool_depth_and_oldest_age`.
- [ ] T025 [REQ-025] Write a failing test `tests/test_audit_access.py::test_admin_scope_query_is_allowed`.
- [ ] T026 [REQ-026] Write a failing test `tests/test_audit_access.py::test_read_and_anonymous_queries_are_denied`.
- [ ] T027 [REQ-027] Write a failing test `tests/test_audit_retention.py::test_hold_retains_entries_naming_the_artifact`.
- [ ] T028 [REQ-028] Write a failing test `tests/test_audit_retention.py::test_active_incident_retains_all_entries`.
- [ ] T029 [REQ-029] Write a failing test `tests/test_audit_retention.py::test_expired_segment_is_deleted_when_no_hold_applies`.
- [ ] T030 [REQ-030] Write a failing test `tests/test_audit_replication.py::test_sealed_segments_replicate_to_trusted_mirror`.
- [ ] T031 [REQ-031] Write a failing test `tests/test_audit_replication.py::test_untrusted_mirror_receives_no_segments`.
- [ ] T032 [REQ-032] Write a failing test `tests/test_audit_replication.py::test_lagging_mirror_is_skipped_until_reseeded`.

## Phase 2: Implementation

- [ ] T033 [REQ-001] Implement until `tests/test_audit_entry.py::test_every_state_change_writes_an_entry` passes.
- [ ] T034 [REQ-002] Implement until `tests/test_audit_entry.py::test_entry_names_actor_token_target_action_and_outcome` passes.
- [ ] T035 [REQ-003] Implement until `tests/test_audit_entry.py::test_entry_records_token_id_not_token_value` passes.
- [ ] T036 [REQ-004] Implement until `tests/test_audit_entry.py::test_sequence_numbers_increase_by_one` passes.
- [ ] T037 [REQ-005] Implement until `tests/test_audit_entry.py::test_entry_chains_to_predecessor_digest` passes.
- [ ] T038 [REQ-006] Implement until `tests/test_audit_entry.py::test_timestamps_are_utc_with_millisecond_precision` passes.
- [ ] T039 [REQ-007] Implement until `tests/test_audit_entry.py::test_rewrite_of_a_written_entry_is_rejected` passes.
- [ ] T040 [REQ-008] Implement until `tests/test_audit_entry.py::test_sealed_segment_digest_is_published` passes.
- [ ] T041 [REQ-009] Implement until `tests/test_audit_durability.py::test_publication_entry_commits_before_accept` passes.
- [ ] T042 [REQ-010] Implement until `tests/test_audit_events.py::test_digest_mismatch_rejection_records_both_digests` passes.
- [ ] T043 [REQ-011] Implement until `tests/test_audit_events.py::test_signature_presence_and_validity_are_recorded` passes.
- [ ] T044 [REQ-012] Implement until `tests/test_audit_events.py::test_yank_records_actor_and_reason` passes.
- [ ] T045 [REQ-013] Implement until `tests/test_audit_events.py::test_token_lifecycle_change_records_scope` passes.
- [ ] T046 [REQ-014] Implement until `tests/test_audit_events.py::test_legacy_mirror_admission_records_origin` passes.
- [ ] T047 [REQ-015] Implement until `tests/test_audit_durability.py::test_entry_is_durable_before_acknowledgement` passes.
- [ ] T048 [REQ-016] Implement until `tests/test_audit_outage.py::test_entries_spool_locally_while_sink_is_down` passes.
- [ ] T049 [REQ-017] Implement until `tests/test_audit_outage.py::test_publication_is_rejected_when_spool_quota_is_exhausted` passes.
- [ ] T050 [REQ-018] Implement until `tests/test_audit_outage.py::test_admin_requests_are_denied_when_spool_quota_is_exhausted` passes.
- [ ] T051 [REQ-019] Implement until `tests/test_audit_outage.py::test_downloads_continue_while_sink_is_down` passes.
- [ ] T052 [REQ-020] Implement until `tests/test_audit_outage.py::test_indexing_is_deferred_while_sink_is_down` passes.
- [ ] T053 [REQ-021] Implement until `tests/test_audit_recovery.py::test_spool_drains_in_sequence_order` passes.
- [ ] T054 [REQ-022] Implement until `tests/test_audit_recovery.py::test_drain_does_not_duplicate_recorded_entries` passes.
- [ ] T055 [REQ-023] Implement until `tests/test_audit_recovery.py::test_recovered_entries_are_indexed_before_log_reports_current` passes.
- [ ] T056 [REQ-024] Implement until `tests/test_audit_outage.py::test_health_endpoint_reports_spool_depth_and_oldest_age` passes.
- [ ] T057 [REQ-025] Implement until `tests/test_audit_access.py::test_admin_scope_query_is_allowed` passes.
- [ ] T058 [REQ-026] Implement until `tests/test_audit_access.py::test_read_and_anonymous_queries_are_denied` passes.
- [ ] T059 [REQ-027] Implement until `tests/test_audit_retention.py::test_hold_retains_entries_naming_the_artifact` passes.
- [ ] T060 [REQ-028] Implement until `tests/test_audit_retention.py::test_active_incident_retains_all_entries` passes.
- [ ] T061 [REQ-029] Implement until `tests/test_audit_retention.py::test_expired_segment_is_deleted_when_no_hold_applies` passes.
- [ ] T062 [REQ-030] Implement until `tests/test_audit_replication.py::test_sealed_segments_replicate_to_trusted_mirror` passes.
- [ ] T063 [REQ-031] Implement until `tests/test_audit_replication.py::test_untrusted_mirror_receives_no_segments` passes.
- [ ] T064 [REQ-032] Implement until `tests/test_audit_replication.py::test_lagging_mirror_is_skipped_until_reseeded` passes.
