# Tasks: Access Control

**Feature**: `003-access-control`

Every requirement below is covered by a failing-test task placed before its implementation task.
The tasks gate refuses to pass while any requirement has no covering task.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_access_control.py::test_every_request_resolves_to_exactly_one_scope`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_transport.py::test_unverified_transport_is_denied`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_transport.py::test_token_on_unverified_transport_is_recorded_as_exposed`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_token_lifecycle.py::test_exposed_token_is_treated_as_expired`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_token_lifecycle.py::test_expired_token_is_denied_not_downgraded`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_token_lifecycle.py::test_expiry_is_evaluated_against_the_server_clock`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_token_lifecycle.py::test_issued_token_expiry_is_bounded`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_token_lifecycle.py::test_near_expiry_lifetime_is_reported`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_audit_access.py::test_token_issue_is_recorded_with_scope_and_expiry`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_publication_access.py::test_write_scope_publishes_over_verified_transport`.
- [ ] T011 [REQ-011] Write a failing test `tests/test_publication_access.py::test_read_scope_cannot_publish`.
- [ ] T012 [REQ-012] Write a failing test `tests/test_publication_access.py::test_unauthenticated_publish_is_rejected`.
- [ ] T013 [REQ-013] Write a failing test `tests/test_access_control.py::test_incident_freezes_the_write_scope`.
- [ ] T014 [REQ-014] Write a failing test `tests/test_access_control.py::test_maintenance_window_freezes_the_write_scope`.
- [ ] T015 [REQ-015] Write a failing test `tests/test_token_scope.py::test_anonymous_scope_downloads_published_artifacts`.
- [ ] T016 [REQ-016] Write a failing test `tests/test_access_control.py::test_incident_withholds_anonymous_downloads`.
- [ ] T017 [REQ-017] Write a failing test `tests/test_token_scope.py::test_read_scope_allows_download_and_metadata`.
- [ ] T018 [REQ-018] Write a failing test `tests/test_token_scope.py::test_admin_scope_allows_administrative_requests`.
- [ ] T019 [REQ-019] Write a failing test `tests/test_audit_access.py::test_admin_request_is_denied_without_an_audit_sink`.
- [ ] T020 [REQ-020] Write a failing test `tests/test_token_scope.py::test_admin_scope_deletes_an_artifact`.
- [ ] T021 [REQ-021] Write a failing test `tests/test_token_scope.py::test_non_admin_scope_cannot_delete`.
- [ ] T022 [REQ-022] Write a failing test `tests/test_audit_access.py::test_admin_requests_are_audited`.
- [ ] T023 [REQ-023] Write a failing test `tests/test_mirror_access.py::test_trusted_mirror_receives_replication`.
- [ ] T024 [REQ-024] Write a failing test `tests/test_mirror_access.py::test_untrusted_or_unverified_mirror_is_skipped`.
- [ ] T025 [REQ-025] Write a failing test `tests/test_mirror_access.py::test_mirror_credential_is_read_scoped`.
- [ ] T026 [REQ-026] Write a failing test `tests/test_token_scope.py::test_scope_escalation_is_denied`.
- [ ] T027 [REQ-027] Write a failing test `tests/test_access_control.py::test_legacy_mirror_setting_requires_the_admin_scope`.
- [ ] T028 [REQ-028] Write a failing test `tests/test_token_lifecycle.py::test_revocation_takes_effect_within_sixty_seconds`.
- [ ] T029 [REQ-029] Write a failing test `tests/test_token_lifecycle.py::test_tokens_are_stored_as_salted_digests`.
- [ ] T030 [REQ-030] Write a failing test `tests/test_access_control.py::test_operator_token_report_lists_scope_and_last_use`.
- [ ] T031 [REQ-031] Write a failing test `tests/test_transport.py::test_obsolete_tls_versions_are_refused`.
- [ ] T032 [REQ-032] Write a failing test `tests/test_token_scope.py::test_anonymous_search_omits_private_namespaces`.

## Phase 2: Implementation

- [ ] T033 [REQ-001] Implement until `tests/test_access_control.py::test_every_request_resolves_to_exactly_one_scope` passes.
- [ ] T034 [REQ-002] Implement until `tests/test_transport.py::test_unverified_transport_is_denied` passes.
- [ ] T035 [REQ-003] Implement until `tests/test_transport.py::test_token_on_unverified_transport_is_recorded_as_exposed` passes.
- [ ] T036 [REQ-004] Implement until `tests/test_token_lifecycle.py::test_exposed_token_is_treated_as_expired` passes.
- [ ] T037 [REQ-005] Implement until `tests/test_token_lifecycle.py::test_expired_token_is_denied_not_downgraded` passes.
- [ ] T038 [REQ-006] Implement until `tests/test_token_lifecycle.py::test_expiry_is_evaluated_against_the_server_clock` passes.
- [ ] T039 [REQ-007] Implement until `tests/test_token_lifecycle.py::test_issued_token_expiry_is_bounded` passes.
- [ ] T040 [REQ-008] Implement until `tests/test_token_lifecycle.py::test_near_expiry_lifetime_is_reported` passes.
- [ ] T041 [REQ-009] Implement until `tests/test_audit_access.py::test_token_issue_is_recorded_with_scope_and_expiry` passes.
- [ ] T042 [REQ-010] Implement until `tests/test_publication_access.py::test_write_scope_publishes_over_verified_transport` passes.
- [ ] T043 [REQ-011] Implement until `tests/test_publication_access.py::test_read_scope_cannot_publish` passes.
- [ ] T044 [REQ-012] Implement until `tests/test_publication_access.py::test_unauthenticated_publish_is_rejected` passes.
- [ ] T045 [REQ-013] Implement until `tests/test_access_control.py::test_incident_freezes_the_write_scope` passes.
- [ ] T046 [REQ-014] Implement until `tests/test_access_control.py::test_maintenance_window_freezes_the_write_scope` passes.
- [ ] T047 [REQ-015] Implement until `tests/test_token_scope.py::test_anonymous_scope_downloads_published_artifacts` passes.
- [ ] T048 [REQ-016] Implement until `tests/test_access_control.py::test_incident_withholds_anonymous_downloads` passes.
- [ ] T049 [REQ-017] Implement until `tests/test_token_scope.py::test_read_scope_allows_download_and_metadata` passes.
- [ ] T050 [REQ-018] Implement until `tests/test_token_scope.py::test_admin_scope_allows_administrative_requests` passes.
- [ ] T051 [REQ-019] Implement until `tests/test_audit_access.py::test_admin_request_is_denied_without_an_audit_sink` passes.
- [ ] T052 [REQ-020] Implement until `tests/test_token_scope.py::test_admin_scope_deletes_an_artifact` passes.
- [ ] T053 [REQ-021] Implement until `tests/test_token_scope.py::test_non_admin_scope_cannot_delete` passes.
- [ ] T054 [REQ-022] Implement until `tests/test_audit_access.py::test_admin_requests_are_audited` passes.
- [ ] T055 [REQ-023] Implement until `tests/test_mirror_access.py::test_trusted_mirror_receives_replication` passes.
- [ ] T056 [REQ-024] Implement until `tests/test_mirror_access.py::test_untrusted_or_unverified_mirror_is_skipped` passes.
- [ ] T057 [REQ-025] Implement until `tests/test_mirror_access.py::test_mirror_credential_is_read_scoped` passes.
- [ ] T058 [REQ-026] Implement until `tests/test_token_scope.py::test_scope_escalation_is_denied` passes.
- [ ] T059 [REQ-027] Implement until `tests/test_access_control.py::test_legacy_mirror_setting_requires_the_admin_scope` passes.
- [ ] T060 [REQ-028] Implement until `tests/test_token_lifecycle.py::test_revocation_takes_effect_within_sixty_seconds` passes.
- [ ] T061 [REQ-029] Implement until `tests/test_token_lifecycle.py::test_tokens_are_stored_as_salted_digests` passes.
- [ ] T062 [REQ-030] Implement until `tests/test_access_control.py::test_operator_token_report_lists_scope_and_last_use` passes.
- [ ] T063 [REQ-031] Implement until `tests/test_transport.py::test_obsolete_tls_versions_are_refused` passes.
- [ ] T064 [REQ-032] Implement until `tests/test_token_scope.py::test_anonymous_search_omits_private_namespaces` passes.
