# Tasks: Artifact Publication

**Feature**: `001-artifact-publication`

Every requirement below is covered by a failing-test task placed before its implementation task.
The tasks gate refuses to pass while any requirement has no covering task.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_publication_auth.py::test_scope_is_resolved_before_body_is_read`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_publication_auth.py::test_write_scoped_request_is_admitted`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_publication_auth.py::test_unknown_publisher_is_denied`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_publication_auth.py::test_read_scope_cannot_publish`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_publication_auth.py::test_expired_token_is_denied`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_publication_auth.py::test_unverified_transport_is_denied`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_publication_auth.py::test_rate_limited_publisher_is_denied`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_publication_auth.py::test_denial_names_required_and_presented_scope`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_publication_quota.py::test_accepted_upload_is_charged_to_the_namespace`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_publication_quota.py::test_rejects_over_quota`.
- [ ] T011 [REQ-011] Write a failing test `tests/test_publication_quota.py::test_over_quota_upload_leaves_no_staged_bytes`.
- [ ] T012 [REQ-012] Write a failing test `tests/test_publication_manifest.py::test_accepts_signed_sha256_upload`.
- [ ] T013 [REQ-013] Write a failing test `tests/test_publication_manifest.py::test_digest_is_recomputed_server_side`.
- [ ] T014 [REQ-014] Write a failing test `tests/test_publication_manifest.py::test_digest_mismatch_is_rejected`.
- [ ] T015 [REQ-015] Write a failing test `tests/test_publication_manifest.py::test_weak_digest_algorithm_is_rejected`.
- [ ] T016 [REQ-016] Write a failing test `tests/test_publication_manifest.py::test_unsigned_upload_is_rejected`.
- [ ] T017 [REQ-017] Write a failing test `tests/test_publication_manifest.py::test_invalid_signature_is_rejected`.
- [ ] T018 [REQ-018] Write a failing test `tests/test_publication_manifest.py::test_manifest_entry_is_written_atomically`.
- [ ] T019 [REQ-019] Write a failing test `tests/test_publication_manifest.py::test_accepted_artifact_is_stored_by_content_address`.
- [ ] T020 [REQ-020] Write a failing test `tests/test_publication_manifest.py::test_mismatched_upload_is_discarded`.
- [ ] T021 [REQ-021] Write a failing test `tests/test_publication_duplicates.py::test_manifest_holds_one_entry_per_version`.
- [ ] T022 [REQ-022] Write a failing test `tests/test_publication_duplicates.py::test_identical_republication_is_idempotent`.
- [ ] T023 [REQ-023] Write a failing test `tests/test_publication_duplicates.py::test_version_reuse_with_different_bytes_is_rejected`.
- [ ] T024 [REQ-024] Write a failing test `tests/test_publication_duplicates.py::test_yanked_version_cannot_be_republished`.
- [ ] T025 [REQ-025] Write a failing test `tests/test_publication_duplicates.py::test_retention_hold_preserves_existing_bytes`.
- [ ] T026 [REQ-026] Write a failing test `tests/test_publication_audit.py::test_every_decision_emits_an_audit_entry`.
- [ ] T027 [REQ-027] Write a failing test `tests/test_publication_audit.py::test_publication_refused_without_an_audit_sink`.
- [ ] T028 [REQ-028] Write a failing test `tests/test_publication_audit.py::test_incident_freezes_publication`.
- [ ] T029 [REQ-029] Write a failing test `tests/test_publication_index.py::test_accepted_artifact_is_indexed_before_acknowledgement`.
- [ ] T030 [REQ-030] Write a failing test `tests/test_publication_index.py::test_maintenance_window_defers_indexing`.
- [ ] T031 [REQ-031] Write a failing test `tests/test_publication_manifest.py::test_pre_policy_entries_are_left_unchanged`.

## Phase 2: Implementation

- [ ] T032 [REQ-001] Implement until `tests/test_publication_auth.py::test_scope_is_resolved_before_body_is_read` passes.
- [ ] T033 [REQ-002] Implement until `tests/test_publication_auth.py::test_write_scoped_request_is_admitted` passes.
- [ ] T034 [REQ-003] Implement until `tests/test_publication_auth.py::test_unknown_publisher_is_denied` passes.
- [ ] T035 [REQ-004] Implement until `tests/test_publication_auth.py::test_read_scope_cannot_publish` passes.
- [ ] T036 [REQ-005] Implement until `tests/test_publication_auth.py::test_expired_token_is_denied` passes.
- [ ] T037 [REQ-006] Implement until `tests/test_publication_auth.py::test_unverified_transport_is_denied` passes.
- [ ] T038 [REQ-007] Implement until `tests/test_publication_auth.py::test_rate_limited_publisher_is_denied` passes.
- [ ] T039 [REQ-008] Implement until `tests/test_publication_auth.py::test_denial_names_required_and_presented_scope` passes.
- [ ] T040 [REQ-009] Implement until `tests/test_publication_quota.py::test_accepted_upload_is_charged_to_the_namespace` passes.
- [ ] T041 [REQ-010] Implement until `tests/test_publication_quota.py::test_rejects_over_quota` passes.
- [ ] T042 [REQ-011] Implement until `tests/test_publication_quota.py::test_over_quota_upload_leaves_no_staged_bytes` passes.
- [ ] T043 [REQ-012] Implement until `tests/test_publication_manifest.py::test_accepts_signed_sha256_upload` passes.
- [ ] T044 [REQ-013] Implement until `tests/test_publication_manifest.py::test_digest_is_recomputed_server_side` passes.
- [ ] T045 [REQ-014] Implement until `tests/test_publication_manifest.py::test_digest_mismatch_is_rejected` passes.
- [ ] T046 [REQ-015] Implement until `tests/test_publication_manifest.py::test_weak_digest_algorithm_is_rejected` passes.
- [ ] T047 [REQ-016] Implement until `tests/test_publication_manifest.py::test_unsigned_upload_is_rejected` passes.
- [ ] T048 [REQ-017] Implement until `tests/test_publication_manifest.py::test_invalid_signature_is_rejected` passes.
- [ ] T049 [REQ-018] Implement until `tests/test_publication_manifest.py::test_manifest_entry_is_written_atomically` passes.
- [ ] T050 [REQ-019] Implement until `tests/test_publication_manifest.py::test_accepted_artifact_is_stored_by_content_address` passes.
- [ ] T051 [REQ-020] Implement until `tests/test_publication_manifest.py::test_mismatched_upload_is_discarded` passes.
- [ ] T052 [REQ-021] Implement until `tests/test_publication_duplicates.py::test_manifest_holds_one_entry_per_version` passes.
- [ ] T053 [REQ-022] Implement until `tests/test_publication_duplicates.py::test_identical_republication_is_idempotent` passes.
- [ ] T054 [REQ-023] Implement until `tests/test_publication_duplicates.py::test_version_reuse_with_different_bytes_is_rejected` passes.
- [ ] T055 [REQ-024] Implement until `tests/test_publication_duplicates.py::test_yanked_version_cannot_be_republished` passes.
- [ ] T056 [REQ-025] Implement until `tests/test_publication_duplicates.py::test_retention_hold_preserves_existing_bytes` passes.
- [ ] T057 [REQ-026] Implement until `tests/test_publication_audit.py::test_every_decision_emits_an_audit_entry` passes.
- [ ] T058 [REQ-027] Implement until `tests/test_publication_audit.py::test_publication_refused_without_an_audit_sink` passes.
- [ ] T059 [REQ-028] Implement until `tests/test_publication_audit.py::test_incident_freezes_publication` passes.
- [ ] T060 [REQ-029] Implement until `tests/test_publication_index.py::test_accepted_artifact_is_indexed_before_acknowledgement` passes.
- [ ] T061 [REQ-030] Implement until `tests/test_publication_index.py::test_maintenance_window_defers_indexing` passes.
- [ ] T062 [REQ-031] Implement until `tests/test_publication_manifest.py::test_pre_policy_entries_are_left_unchanged` passes.
