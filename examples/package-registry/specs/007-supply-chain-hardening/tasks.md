# Tasks: Supply Chain Hardening

**Feature**: `007-supply-chain-hardening`

Every requirement below is covered by a failing-test task placed before its implementation task.
The tasks gate refuses to pass while any requirement has no covering task.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_digest_policy.py::test_only_sha256_and_blake3_permitted`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_publication.py::test_rejects_md5_or_sha1_digest`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_publication.py::test_rejects_digest_mismatch`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_signature.py::test_rejects_unsigned_publication`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_signature.py::test_rejects_signature_from_untrusted_key`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_publication.py::test_accepts_signed_sha256_publication`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_storage.py::test_stores_signature_and_digest_with_artifact`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_signature.py::test_key_enrolment_predates_publication`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_manifest.py::test_manifest_records_key_identifier`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_signature.py::test_rejects_publication_signed_by_revoked_key`.
- [ ] T011 [REQ-011] Write a failing test `tests/test_serving.py::test_withholds_md5_or_sha1_artifact`.
- [ ] T012 [REQ-012] Write a failing test `tests/test_serving.py::test_withholds_on_digest_mismatch`.
- [ ] T013 [REQ-013] Write a failing test `tests/test_serving.py::test_withholds_unverifiable_signature`.
- [ ] T014 [REQ-014] Write a failing test `tests/test_serving.py::test_serves_verified_artifact`.
- [ ] T015 [REQ-015] Write a failing test `tests/test_mirror.py::test_stale_mirror_withholds_downloads`.
- [ ] T016 [REQ-016] Write a failing test `tests/test_mirror.py::test_skips_untrusted_mirror`.
- [ ] T017 [REQ-017] Write a failing test `tests/test_mirror.py::test_skips_replication_of_weak_digest`.
- [ ] T018 [REQ-018] Write a failing test `tests/test_incident.py::test_incident_halts_replication`.
- [ ] T019 [REQ-019] Write a failing test `tests/test_mirror.py::test_replicates_verified_artifact`.
- [ ] T020 [REQ-020] Write a failing test `tests/test_access.py::test_denies_publication_over_unverified_tls`.
- [ ] T021 [REQ-021] Write a failing test `tests/test_access.py::test_denies_unauthenticated_publication`.
- [ ] T022 [REQ-022] Write a failing test `tests/test_access.py::test_allows_write_scoped_publication`.
- [ ] T023 [REQ-023] Write a failing test `tests/test_incident.py::test_incident_freezes_publication`.
- [ ] T024 [REQ-024] Write a failing test `tests/test_incident.py::test_incident_suspends_deletion`.
- [ ] T025 [REQ-025] Write a failing test `tests/test_incident.py::test_incident_logs_every_download`.
- [ ] T026 [REQ-026] Write a failing test `tests/test_audit.py::test_publication_fails_closed_without_audit_sink`.
- [ ] T027 [REQ-027] Write a failing test `tests/test_audit.py::test_audits_signature_verification_failure`.
- [ ] T028 [REQ-028] Write a failing test `tests/test_audit.py::test_audit_entry_records_rejected_digest`.
- [ ] T029 [REQ-029] Write a failing test `tests/test_policy_notice.py::test_sunset_date_announced_90_days_ahead`.
- [ ] T030 [REQ-030] Write a failing test `tests/test_docs.py::test_resigning_procedure_documented`.
- [ ] T031 [REQ-031] Write a failing test `tests/test_serving.py::test_withheld_download_names_failed_check`.
- [ ] T032 [REQ-032] Write a failing test `tests/test_reporting.py::test_weekly_report_counts_deprecated_digests`.

## Phase 2: Implementation

- [ ] T033 [REQ-001] Implement until `tests/test_digest_policy.py::test_only_sha256_and_blake3_permitted` passes.
- [ ] T034 [REQ-002] Implement until `tests/test_publication.py::test_rejects_md5_or_sha1_digest` passes.
- [ ] T035 [REQ-003] Implement until `tests/test_publication.py::test_rejects_digest_mismatch` passes.
- [ ] T036 [REQ-004] Implement until `tests/test_signature.py::test_rejects_unsigned_publication` passes.
- [ ] T037 [REQ-005] Implement until `tests/test_signature.py::test_rejects_signature_from_untrusted_key` passes.
- [ ] T038 [REQ-006] Implement until `tests/test_publication.py::test_accepts_signed_sha256_publication` passes.
- [ ] T039 [REQ-007] Implement until `tests/test_storage.py::test_stores_signature_and_digest_with_artifact` passes.
- [ ] T040 [REQ-008] Implement until `tests/test_signature.py::test_key_enrolment_predates_publication` passes.
- [ ] T041 [REQ-009] Implement until `tests/test_manifest.py::test_manifest_records_key_identifier` passes.
- [ ] T042 [REQ-010] Implement until `tests/test_signature.py::test_rejects_publication_signed_by_revoked_key` passes.
- [ ] T043 [REQ-011] Implement until `tests/test_serving.py::test_withholds_md5_or_sha1_artifact` passes.
- [ ] T044 [REQ-012] Implement until `tests/test_serving.py::test_withholds_on_digest_mismatch` passes.
- [ ] T045 [REQ-013] Implement until `tests/test_serving.py::test_withholds_unverifiable_signature` passes.
- [ ] T046 [REQ-014] Implement until `tests/test_serving.py::test_serves_verified_artifact` passes.
- [ ] T047 [REQ-015] Implement until `tests/test_mirror.py::test_stale_mirror_withholds_downloads` passes.
- [ ] T048 [REQ-016] Implement until `tests/test_mirror.py::test_skips_untrusted_mirror` passes.
- [ ] T049 [REQ-017] Implement until `tests/test_mirror.py::test_skips_replication_of_weak_digest` passes.
- [ ] T050 [REQ-018] Implement until `tests/test_incident.py::test_incident_halts_replication` passes.
- [ ] T051 [REQ-019] Implement until `tests/test_mirror.py::test_replicates_verified_artifact` passes.
- [ ] T052 [REQ-020] Implement until `tests/test_access.py::test_denies_publication_over_unverified_tls` passes.
- [ ] T053 [REQ-021] Implement until `tests/test_access.py::test_denies_unauthenticated_publication` passes.
- [ ] T054 [REQ-022] Implement until `tests/test_access.py::test_allows_write_scoped_publication` passes.
- [ ] T055 [REQ-023] Implement until `tests/test_incident.py::test_incident_freezes_publication` passes.
- [ ] T056 [REQ-024] Implement until `tests/test_incident.py::test_incident_suspends_deletion` passes.
- [ ] T057 [REQ-025] Implement until `tests/test_incident.py::test_incident_logs_every_download` passes.
- [ ] T058 [REQ-026] Implement until `tests/test_audit.py::test_publication_fails_closed_without_audit_sink` passes.
- [ ] T059 [REQ-027] Implement until `tests/test_audit.py::test_audits_signature_verification_failure` passes.
- [ ] T060 [REQ-028] Implement until `tests/test_audit.py::test_audit_entry_records_rejected_digest` passes.
- [ ] T061 [REQ-029] Implement until `tests/test_policy_notice.py::test_sunset_date_announced_90_days_ahead` passes.
- [ ] T062 [REQ-030] Implement until `tests/test_docs.py::test_resigning_procedure_documented` passes.
- [ ] T063 [REQ-031] Implement until `tests/test_serving.py::test_withheld_download_names_failed_check` passes.
- [ ] T064 [REQ-032] Implement until `tests/test_reporting.py::test_weekly_report_counts_deprecated_digests` passes.
