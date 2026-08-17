# Tasks: Client Compatibility

**Feature**: `011-client-compatibility`

Every requirement below is covered by a failing-test task placed before its implementation task.
The tasks gate refuses to pass while any requirement has no covering task.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_version_negotiation.py::test_discovery_lists_supported_versions`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_version_negotiation.py::test_negotiates_highest_common_version`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_version_negotiation.py::test_missing_version_header_defaults_to_oldest_supported`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_version_negotiation.py::test_future_version_answered_under_newest_implemented`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_version_negotiation.py::test_unknown_version_identifier_returns_negotiation_error`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_deprecation.py::test_deprecated_version_responses_carry_sunset_date`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_deprecation.py::test_sunset_date_published_180_days_ahead`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_client_compatibility.py::test_audit_entry_records_negotiated_version`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_deprecation.py::test_metrics_break_down_requests_by_protocol_version`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_client_compatibility.py::test_supported_client_request_allowed`.
- [ ] T011 [REQ-011] Write a failing test `tests/test_client_compatibility.py::test_unsupported_client_read_still_allowed`.
- [ ] T012 [REQ-012] Write a failing test `tests/test_client_compatibility.py::test_unverified_transport_denied`.
- [ ] T013 [REQ-013] Write a failing test `tests/test_client_compatibility.py::test_unsupported_client_publish_denied`.
- [ ] T014 [REQ-014] Write a failing test `tests/test_client_compatibility.py::test_unsupported_client_admin_denied`.
- [ ] T015 [REQ-015] Write a failing test `tests/test_client_compatibility.py::test_unsupported_client_over_rate_limit_denied`.
- [ ] T016 [REQ-016] Write a failing test `tests/test_client_compatibility.py::test_unsupported_client_denied_during_incident`.
- [ ] T017 [REQ-017] Write a failing test `tests/test_client_compatibility.py::test_denial_uses_legacy_error_format`.
- [ ] T018 [REQ-018] Write a failing test `tests/test_publication.py::test_rejects_md5_digest_from_unsupported_client`.
- [ ] T019 [REQ-019] Write a failing test `tests/test_publication.py::test_accepts_sha256_publish_from_supported_client`.
- [ ] T020 [REQ-020] Write a failing test `tests/test_publication.py::test_discards_upload_body_from_unsupported_client`.
- [ ] T021 [REQ-021] Write a failing test `tests/test_client_compatibility.py::test_unsupported_client_download_served`.
- [ ] T022 [REQ-022] Write a failing test `tests/test_client_compatibility.py::test_yanked_artifact_withheld_from_unsupported_client`.
- [ ] T023 [REQ-023] Write a failing test `tests/test_client_compatibility.py::test_artifacts_withheld_from_unsupported_clients_during_incident`.
- [ ] T024 [REQ-024] Write a failing test `tests/test_client_compatibility.py::test_legacy_manifest_carries_sha256_alongside_blake3`.
- [ ] T025 [REQ-025] Write a failing test `tests/test_client_compatibility.py::test_manifest_omits_fields_newer_than_client_version`.
- [ ] T026 [REQ-026] Write a failing test `tests/test_legacy_mirror.py::test_replicates_legacy_manifest_to_trusted_mirror`.
- [ ] T027 [REQ-027] Write a failing test `tests/test_legacy_mirror.py::test_skips_replication_to_untrusted_mirror`.
- [ ] T028 [REQ-028] Write a failing test `tests/test_legacy_mirror.py::test_skips_replication_when_lag_exceeds_bound`.
- [ ] T029 [REQ-029] Write a failing test `tests/test_legacy_mirror.py::test_untrusted_legacy_mirror_recorded_in_audit_log`.
- [ ] T030 [REQ-030] Write a failing test `tests/test_client_compatibility.py::test_unsupported_client_request_audited`.
- [ ] T031 [REQ-031] Write a failing test `tests/test_legacy_mirror.py::test_legacy_format_copy_retained_while_enabled`.
- [ ] T032 [REQ-032] Write a failing test `tests/test_legacy_mirror.py::test_deletes_unused_legacy_copy_after_retirement`.

## Phase 2: Implementation

- [ ] T033 [REQ-001] Implement until `tests/test_version_negotiation.py::test_discovery_lists_supported_versions` passes.
- [ ] T034 [REQ-002] Implement until `tests/test_version_negotiation.py::test_negotiates_highest_common_version` passes.
- [ ] T035 [REQ-003] Implement until `tests/test_version_negotiation.py::test_missing_version_header_defaults_to_oldest_supported` passes.
- [ ] T036 [REQ-004] Implement until `tests/test_version_negotiation.py::test_future_version_answered_under_newest_implemented` passes.
- [ ] T037 [REQ-005] Implement until `tests/test_version_negotiation.py::test_unknown_version_identifier_returns_negotiation_error` passes.
- [ ] T038 [REQ-006] Implement until `tests/test_deprecation.py::test_deprecated_version_responses_carry_sunset_date` passes.
- [ ] T039 [REQ-007] Implement until `tests/test_deprecation.py::test_sunset_date_published_180_days_ahead` passes.
- [ ] T040 [REQ-008] Implement until `tests/test_client_compatibility.py::test_audit_entry_records_negotiated_version` passes.
- [ ] T041 [REQ-009] Implement until `tests/test_deprecation.py::test_metrics_break_down_requests_by_protocol_version` passes.
- [ ] T042 [REQ-010] Implement until `tests/test_client_compatibility.py::test_supported_client_request_allowed` passes.
- [ ] T043 [REQ-011] Implement until `tests/test_client_compatibility.py::test_unsupported_client_read_still_allowed` passes.
- [ ] T044 [REQ-012] Implement until `tests/test_client_compatibility.py::test_unverified_transport_denied` passes.
- [ ] T045 [REQ-013] Implement until `tests/test_client_compatibility.py::test_unsupported_client_publish_denied` passes.
- [ ] T046 [REQ-014] Implement until `tests/test_client_compatibility.py::test_unsupported_client_admin_denied` passes.
- [ ] T047 [REQ-015] Implement until `tests/test_client_compatibility.py::test_unsupported_client_over_rate_limit_denied` passes.
- [ ] T048 [REQ-016] Implement until `tests/test_client_compatibility.py::test_unsupported_client_denied_during_incident` passes.
- [ ] T049 [REQ-017] Implement until `tests/test_client_compatibility.py::test_denial_uses_legacy_error_format` passes.
- [ ] T050 [REQ-018] Implement until `tests/test_publication.py::test_rejects_md5_digest_from_unsupported_client` passes.
- [ ] T051 [REQ-019] Implement until `tests/test_publication.py::test_accepts_sha256_publish_from_supported_client` passes.
- [ ] T052 [REQ-020] Implement until `tests/test_publication.py::test_discards_upload_body_from_unsupported_client` passes.
- [ ] T053 [REQ-021] Implement until `tests/test_client_compatibility.py::test_unsupported_client_download_served` passes.
- [ ] T054 [REQ-022] Implement until `tests/test_client_compatibility.py::test_yanked_artifact_withheld_from_unsupported_client` passes.
- [ ] T055 [REQ-023] Implement until `tests/test_client_compatibility.py::test_artifacts_withheld_from_unsupported_clients_during_incident` passes.
- [ ] T056 [REQ-024] Implement until `tests/test_client_compatibility.py::test_legacy_manifest_carries_sha256_alongside_blake3` passes.
- [ ] T057 [REQ-025] Implement until `tests/test_client_compatibility.py::test_manifest_omits_fields_newer_than_client_version` passes.
- [ ] T058 [REQ-026] Implement until `tests/test_legacy_mirror.py::test_replicates_legacy_manifest_to_trusted_mirror` passes.
- [ ] T059 [REQ-027] Implement until `tests/test_legacy_mirror.py::test_skips_replication_to_untrusted_mirror` passes.
- [ ] T060 [REQ-028] Implement until `tests/test_legacy_mirror.py::test_skips_replication_when_lag_exceeds_bound` passes.
- [ ] T061 [REQ-029] Implement until `tests/test_legacy_mirror.py::test_untrusted_legacy_mirror_recorded_in_audit_log` passes.
- [ ] T062 [REQ-030] Implement until `tests/test_client_compatibility.py::test_unsupported_client_request_audited` passes.
- [ ] T063 [REQ-031] Implement until `tests/test_legacy_mirror.py::test_legacy_format_copy_retained_while_enabled` passes.
- [ ] T064 [REQ-032] Implement until `tests/test_legacy_mirror.py::test_deletes_unused_legacy_copy_after_retirement` passes.
