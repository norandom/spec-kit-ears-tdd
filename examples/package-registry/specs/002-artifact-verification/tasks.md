# Tasks: Artifact Verification

**Feature**: `002-artifact-verification`

Every requirement below is covered by a failing-test task placed before its implementation task.
The tasks gate refuses to pass while any requirement has no covering task.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_digest.py::test_digest_is_computed_before_signature_evaluation`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_digest.py::test_rejects_artifact_on_digest_mismatch`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_digest.py::test_discards_bytes_on_digest_mismatch`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_digest.py::test_rejects_md5_even_when_digest_matches`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_digest.py::test_rejects_sha1_when_legacy_support_is_off`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_digest.py::test_accepts_strong_digest_with_valid_signature`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_digest.py::test_accepts_sha1_under_legacy_mirror_support`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_digest.py::test_digest_comparison_is_constant_time`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_reporting.py::test_verification_record_names_algorithm_and_value`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_reporting.py::test_algorithm_policy_is_documented`.
- [ ] T011 [REQ-011] Write a failing test `tests/test_signature.py::test_rejects_artifact_with_invalid_signature`.
- [ ] T012 [REQ-012] Write a failing test `tests/test_signature.py::test_rejects_unsigned_artifact_outside_legacy_support`.
- [ ] T013 [REQ-013] Write a failing test `tests/test_signature.py::test_rejects_signature_from_unbound_key`.
- [ ] T014 [REQ-014] Write a failing test `tests/test_reporting.py::test_signature_failure_names_key_algorithm_and_reason`.
- [ ] T015 [REQ-015] Write a failing test `tests/test_reporting.py::test_unsigned_legacy_artifact_is_marked_unverified`.
- [ ] T016 [REQ-016] Write a failing test `tests/test_mirror_trust.py::test_denies_fetch_over_unverified_transport`.
- [ ] T017 [REQ-017] Write a failing test `tests/test_mirror_trust.py::test_withholds_artifacts_from_untrusted_mirror`.
- [ ] T018 [REQ-018] Write a failing test `tests/test_mirror_trust.py::test_replicates_to_current_trusted_mirror`.
- [ ] T019 [REQ-019] Write a failing test `tests/test_mirror_trust.py::test_skips_replication_to_lagging_mirror`.
- [ ] T020 [REQ-020] Write a failing test `tests/test_reporting.py::test_status_endpoint_publishes_trusted_mirror_list`.
- [ ] T021 [REQ-021] Write a failing test `tests/test_reporting.py::test_audit_log_names_supplying_mirror`.
- [ ] T022 [REQ-022] Write a failing test `tests/test_serving.py::test_serves_verified_artifact_from_trusted_mirror`.
- [ ] T023 [REQ-023] Write a failing test `tests/test_serving.py::test_withholds_yanked_artifact_from_unpinned_request`.
- [ ] T024 [REQ-024] Write a failing test `tests/test_reporting.py::test_withheld_yanked_artifact_carries_reason`.
- [ ] T025 [REQ-025] Write a failing test `tests/test_cache.py::test_stores_verified_artifact_within_quota`.
- [ ] T026 [REQ-026] Write a failing test `tests/test_cache.py::test_retains_artifact_under_retention_hold`.
- [ ] T027 [REQ-027] Write a failing test `tests/test_cache.py::test_deletes_old_unused_artifact_over_quota`.
- [ ] T028 [REQ-028] Write a failing test `tests/test_access.py::test_denies_request_with_expired_token`.
- [ ] T029 [REQ-029] Write a failing test `tests/test_access.py::test_allows_read_scoped_verification_request`.
- [ ] T030 [REQ-030] Write a failing test `tests/test_reporting.py::test_every_verification_outcome_is_audited`.
- [ ] T031 [REQ-031] Write a failing test `tests/test_reporting.py::test_verification_failure_uses_stable_reason_code`.
- [ ] T032 [REQ-032] Write a failing test `tests/test_reporting.py::test_index_listing_names_verification_algorithm`.

## Phase 2: Implementation

- [ ] T033 [REQ-001] Implement until `tests/test_digest.py::test_digest_is_computed_before_signature_evaluation` passes.
- [ ] T034 [REQ-002] Implement until `tests/test_digest.py::test_rejects_artifact_on_digest_mismatch` passes.
- [ ] T035 [REQ-003] Implement until `tests/test_digest.py::test_discards_bytes_on_digest_mismatch` passes.
- [ ] T036 [REQ-004] Implement until `tests/test_digest.py::test_rejects_md5_even_when_digest_matches` passes.
- [ ] T037 [REQ-005] Implement until `tests/test_digest.py::test_rejects_sha1_when_legacy_support_is_off` passes.
- [ ] T038 [REQ-006] Implement until `tests/test_digest.py::test_accepts_strong_digest_with_valid_signature` passes.
- [ ] T039 [REQ-007] Implement until `tests/test_digest.py::test_accepts_sha1_under_legacy_mirror_support` passes.
- [ ] T040 [REQ-008] Implement until `tests/test_digest.py::test_digest_comparison_is_constant_time` passes.
- [ ] T041 [REQ-009] Implement until `tests/test_reporting.py::test_verification_record_names_algorithm_and_value` passes.
- [ ] T042 [REQ-010] Implement until `tests/test_reporting.py::test_algorithm_policy_is_documented` passes.
- [ ] T043 [REQ-011] Implement until `tests/test_signature.py::test_rejects_artifact_with_invalid_signature` passes.
- [ ] T044 [REQ-012] Implement until `tests/test_signature.py::test_rejects_unsigned_artifact_outside_legacy_support` passes.
- [ ] T045 [REQ-013] Implement until `tests/test_signature.py::test_rejects_signature_from_unbound_key` passes.
- [ ] T046 [REQ-014] Implement until `tests/test_reporting.py::test_signature_failure_names_key_algorithm_and_reason` passes.
- [ ] T047 [REQ-015] Implement until `tests/test_reporting.py::test_unsigned_legacy_artifact_is_marked_unverified` passes.
- [ ] T048 [REQ-016] Implement until `tests/test_mirror_trust.py::test_denies_fetch_over_unverified_transport` passes.
- [ ] T049 [REQ-017] Implement until `tests/test_mirror_trust.py::test_withholds_artifacts_from_untrusted_mirror` passes.
- [ ] T050 [REQ-018] Implement until `tests/test_mirror_trust.py::test_replicates_to_current_trusted_mirror` passes.
- [ ] T051 [REQ-019] Implement until `tests/test_mirror_trust.py::test_skips_replication_to_lagging_mirror` passes.
- [ ] T052 [REQ-020] Implement until `tests/test_reporting.py::test_status_endpoint_publishes_trusted_mirror_list` passes.
- [ ] T053 [REQ-021] Implement until `tests/test_reporting.py::test_audit_log_names_supplying_mirror` passes.
- [ ] T054 [REQ-022] Implement until `tests/test_serving.py::test_serves_verified_artifact_from_trusted_mirror` passes.
- [ ] T055 [REQ-023] Implement until `tests/test_serving.py::test_withholds_yanked_artifact_from_unpinned_request` passes.
- [ ] T056 [REQ-024] Implement until `tests/test_reporting.py::test_withheld_yanked_artifact_carries_reason` passes.
- [ ] T057 [REQ-025] Implement until `tests/test_cache.py::test_stores_verified_artifact_within_quota` passes.
- [ ] T058 [REQ-026] Implement until `tests/test_cache.py::test_retains_artifact_under_retention_hold` passes.
- [ ] T059 [REQ-027] Implement until `tests/test_cache.py::test_deletes_old_unused_artifact_over_quota` passes.
- [ ] T060 [REQ-028] Implement until `tests/test_access.py::test_denies_request_with_expired_token` passes.
- [ ] T061 [REQ-029] Implement until `tests/test_access.py::test_allows_read_scoped_verification_request` passes.
- [ ] T062 [REQ-030] Implement until `tests/test_reporting.py::test_every_verification_outcome_is_audited` passes.
- [ ] T063 [REQ-031] Implement until `tests/test_reporting.py::test_verification_failure_uses_stable_reason_code` passes.
- [ ] T064 [REQ-032] Implement until `tests/test_reporting.py::test_index_listing_names_verification_algorithm` passes.
