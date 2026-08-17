# Tasks: Incident Response

**Feature**: `010-incident-response`

Every requirement below is covered by a failing-test task placed before its implementation task.
The tasks gate refuses to pass while any requirement has no covering task.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_incident_posture.py::test_posture_applies_from_declaration_timestamp`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_incident_reporting.py::test_status_endpoint_names_active_incident`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_incident_evidence.py::test_every_handled_request_is_audited`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_incident_evidence.py::test_publication_rejected_when_audit_sink_unavailable`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_incident_evidence.py::test_audit_sink_loss_raises_named_alert`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_incident_posture.py::test_write_scoped_publication_is_frozen`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_incident_posture.py::test_unauthenticated_publication_rejected`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_incident_posture.py::test_admin_remediation_release_accepted`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_incident_posture.py::test_digest_mismatch_upload_discarded`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_incident_posture.py::test_read_path_stays_open_during_incident`.
- [ ] T011 [REQ-011] Write a failing test `tests/test_incident_posture.py::test_expired_token_denied_during_incident`.
- [ ] T012 [REQ-012] Write a failing test `tests/test_incident_posture.py::test_unverified_transport_denied_during_incident`.
- [ ] T013 [REQ-013] Write a failing test `tests/test_incident_posture.py::test_rate_limited_client_denied_during_incident`.
- [ ] T014 [REQ-014] Write a failing test `tests/test_incident_serving.py::test_verified_artifact_still_served_during_incident`.
- [ ] T015 [REQ-015] Write a failing test `tests/test_incident_serving.py::test_yanked_artifact_withheld_during_incident`.
- [ ] T016 [REQ-016] Write a failing test `tests/test_incident_serving.py::test_unsigned_artifact_withheld_during_incident`.
- [ ] T017 [REQ-017] Write a failing test `tests/test_incident_serving.py::test_weak_digest_artifact_withheld_during_incident`.
- [ ] T018 [REQ-018] Write a failing test `tests/test_incident_serving.py::test_lagging_mirror_artifacts_withheld`.
- [ ] T019 [REQ-019] Write a failing test `tests/test_incident_evidence.py::test_collection_suspended_during_incident`.
- [ ] T020 [REQ-020] Write a failing test `tests/test_incident_replication.py::test_untrusted_mirror_replication_skipped`.
- [ ] T021 [REQ-021] Write a failing test `tests/test_incident_replication.py::test_legacy_mirror_replication_skipped`.
- [ ] T022 [REQ-022] Write a failing test `tests/test_incident_replication.py::test_verified_artifact_reaches_trusted_mirrors`.
- [ ] T023 [REQ-023] Write a failing test `tests/test_incident_serving.py::test_unverified_artifact_indexing_deferred`.
- [ ] T024 [REQ-024] Write a failing test `tests/test_incident_serving.py::test_remediation_release_indexed_first`.
- [ ] T025 [REQ-025] Write a failing test `tests/test_incident_evidence.py::test_manifest_entries_carry_incident_identifier`.
- [ ] T026 [REQ-026] Write a failing test `tests/test_incident_reporting.py::test_closure_report_lists_withheld_artifacts`.
- [ ] T027 [REQ-027] Write a failing test `tests/test_incident_reporting.py::test_legacy_artifacts_listed_for_restoration`.
- [ ] T028 [REQ-028] Write a failing test `tests/test_incident_reporting.py::test_unsupported_client_receives_advisory_address`.
- [ ] T029 [REQ-029] Write a failing test `tests/test_incident_reporting.py::test_incident_status_outranks_maintenance_status`.
- [ ] T030 [REQ-030] Write a failing test `tests/test_incident_reporting.py::test_index_reported_stale_until_queue_drained`.
- [ ] T031 [REQ-031] Write a failing test `tests/test_incident_reporting.py::test_dashboard_lists_over_quota_namespaces`.
- [ ] T032 [REQ-032] Write a failing test `tests/test_incident_reporting.py::test_never_downloaded_withheld_artifacts_listed_with_age`.

## Phase 2: Implementation

- [ ] T033 [REQ-001] Implement until `tests/test_incident_posture.py::test_posture_applies_from_declaration_timestamp` passes.
- [ ] T034 [REQ-002] Implement until `tests/test_incident_reporting.py::test_status_endpoint_names_active_incident` passes.
- [ ] T035 [REQ-003] Implement until `tests/test_incident_evidence.py::test_every_handled_request_is_audited` passes.
- [ ] T036 [REQ-004] Implement until `tests/test_incident_evidence.py::test_publication_rejected_when_audit_sink_unavailable` passes.
- [ ] T037 [REQ-005] Implement until `tests/test_incident_evidence.py::test_audit_sink_loss_raises_named_alert` passes.
- [ ] T038 [REQ-006] Implement until `tests/test_incident_posture.py::test_write_scoped_publication_is_frozen` passes.
- [ ] T039 [REQ-007] Implement until `tests/test_incident_posture.py::test_unauthenticated_publication_rejected` passes.
- [ ] T040 [REQ-008] Implement until `tests/test_incident_posture.py::test_admin_remediation_release_accepted` passes.
- [ ] T041 [REQ-009] Implement until `tests/test_incident_posture.py::test_digest_mismatch_upload_discarded` passes.
- [ ] T042 [REQ-010] Implement until `tests/test_incident_posture.py::test_read_path_stays_open_during_incident` passes.
- [ ] T043 [REQ-011] Implement until `tests/test_incident_posture.py::test_expired_token_denied_during_incident` passes.
- [ ] T044 [REQ-012] Implement until `tests/test_incident_posture.py::test_unverified_transport_denied_during_incident` passes.
- [ ] T045 [REQ-013] Implement until `tests/test_incident_posture.py::test_rate_limited_client_denied_during_incident` passes.
- [ ] T046 [REQ-014] Implement until `tests/test_incident_serving.py::test_verified_artifact_still_served_during_incident` passes.
- [ ] T047 [REQ-015] Implement until `tests/test_incident_serving.py::test_yanked_artifact_withheld_during_incident` passes.
- [ ] T048 [REQ-016] Implement until `tests/test_incident_serving.py::test_unsigned_artifact_withheld_during_incident` passes.
- [ ] T049 [REQ-017] Implement until `tests/test_incident_serving.py::test_weak_digest_artifact_withheld_during_incident` passes.
- [ ] T050 [REQ-018] Implement until `tests/test_incident_serving.py::test_lagging_mirror_artifacts_withheld` passes.
- [ ] T051 [REQ-019] Implement until `tests/test_incident_evidence.py::test_collection_suspended_during_incident` passes.
- [ ] T052 [REQ-020] Implement until `tests/test_incident_replication.py::test_untrusted_mirror_replication_skipped` passes.
- [ ] T053 [REQ-021] Implement until `tests/test_incident_replication.py::test_legacy_mirror_replication_skipped` passes.
- [ ] T054 [REQ-022] Implement until `tests/test_incident_replication.py::test_verified_artifact_reaches_trusted_mirrors` passes.
- [ ] T055 [REQ-023] Implement until `tests/test_incident_serving.py::test_unverified_artifact_indexing_deferred` passes.
- [ ] T056 [REQ-024] Implement until `tests/test_incident_serving.py::test_remediation_release_indexed_first` passes.
- [ ] T057 [REQ-025] Implement until `tests/test_incident_evidence.py::test_manifest_entries_carry_incident_identifier` passes.
- [ ] T058 [REQ-026] Implement until `tests/test_incident_reporting.py::test_closure_report_lists_withheld_artifacts` passes.
- [ ] T059 [REQ-027] Implement until `tests/test_incident_reporting.py::test_legacy_artifacts_listed_for_restoration` passes.
- [ ] T060 [REQ-028] Implement until `tests/test_incident_reporting.py::test_unsupported_client_receives_advisory_address` passes.
- [ ] T061 [REQ-029] Implement until `tests/test_incident_reporting.py::test_incident_status_outranks_maintenance_status` passes.
- [ ] T062 [REQ-030] Implement until `tests/test_incident_reporting.py::test_index_reported_stale_until_queue_drained` passes.
- [ ] T063 [REQ-031] Implement until `tests/test_incident_reporting.py::test_dashboard_lists_over_quota_namespaces` passes.
- [ ] T064 [REQ-032] Implement until `tests/test_incident_reporting.py::test_never_downloaded_withheld_artifacts_listed_with_age` passes.
