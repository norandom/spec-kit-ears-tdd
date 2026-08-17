# Tasks: Search Indexing

**Feature**: `009-search-indexing`

Every requirement below is covered by a failing-test task placed before its implementation task.
The tasks gate refuses to pass while any requirement has no covering task.

## Phase 1: Verification

- [ ] T001 [REQ-001] Write a failing test `tests/test_search_indexing.py::test_one_document_per_published_version`.
- [ ] T002 [REQ-002] Write a failing test `tests/test_search_indexing.py::test_verified_version_is_indexed_on_commit`.
- [ ] T003 [REQ-003] Write a failing test `tests/test_search_indexing.py::test_digest_mismatch_defers_indexing`.
- [ ] T004 [REQ-004] Write a failing test `tests/test_search_indexing.py::test_invalid_signature_defers_indexing`.
- [ ] T005 [REQ-005] Write a failing test `tests/test_search_indexing.py::test_weak_digest_algorithm_defers_indexing`.
- [ ] T006 [REQ-006] Write a failing test `tests/test_search_indexing.py::test_unsigned_version_lands_in_unverified_partition`.
- [ ] T007 [REQ-007] Write a failing test `tests/test_search_indexing.py::test_maintenance_window_queues_index_writes`.
- [ ] T008 [REQ-008] Write a failing test `tests/test_search_indexing.py::test_quota_exceeded_defers_index_writes`.
- [ ] T009 [REQ-009] Write a failing test `tests/test_search_indexing.py::test_audit_sink_down_defers_indexing`.
- [ ] T010 [REQ-010] Write a failing test `tests/test_search_indexing.py::test_index_entry_names_version_and_generation`.
- [ ] T011 [REQ-011] Write a failing test `tests/test_search_indexing.py::test_generation_commit_is_audited`.
- [ ] T012 [REQ-012] Write a failing test `tests/test_search_indexing.py::test_backing_store_lag_marks_index_stale`.
- [ ] T013 [REQ-013] Write a failing test `tests/test_search_indexing.py::test_stale_index_serves_last_committed_generation`.
- [ ] T014 [REQ-014] Write a failing test `tests/test_search_indexing.py::test_stale_response_carries_generation_and_age`.
- [ ] T015 [REQ-015] Write a failing test `tests/test_search_indexing.py::test_reindex_serves_previous_generation`.
- [ ] T016 [REQ-016] Write a failing test `tests/test_search_indexing.py::test_reindex_progress_on_status_endpoint`.
- [ ] T017 [REQ-017] Write a failing test `tests/test_search_indexing.py::test_generation_switch_is_atomic`.
- [ ] T018 [REQ-018] Write a failing test `tests/test_search_indexing.py::test_partial_generation_discarded_on_quota_exhaustion`.
- [ ] T019 [REQ-019] Write a failing test `tests/test_search_indexing.py::test_yanked_version_hidden_from_default_results`.
- [ ] T020 [REQ-020] Write a failing test `tests/test_search_indexing.py::test_yanked_version_document_retained`.
- [ ] T021 [REQ-021] Write a failing test `tests/test_search_indexing.py::test_exact_version_query_returns_yanked_marker`.
- [ ] T022 [REQ-022] Write a failing test `tests/test_search_indexing.py::test_cold_old_version_document_deleted`.
- [ ] T023 [REQ-023] Write a failing test `tests/test_search_indexing.py::test_retention_hold_blocks_document_deletion`.
- [ ] T024 [REQ-024] Write a failing test `tests/test_search_indexing.py::test_trusted_mirror_receives_generation`.
- [ ] T025 [REQ-025] Write a failing test `tests/test_search_indexing.py::test_untrusted_mirror_replication_skipped`.
- [ ] T026 [REQ-026] Write a failing test `tests/test_search_indexing.py::test_anonymous_scope_sees_public_versions_only`.
- [ ] T027 [REQ-027] Write a failing test `tests/test_search_indexing.py::test_rate_limited_query_denied_with_retry_after`.
- [ ] T028 [REQ-028] Write a failing test `tests/test_search_indexing.py::test_reindex_requires_admin_scope`.
- [ ] T029 [REQ-029] Write a failing test `tests/test_search_indexing.py::test_untrusted_transport_denied`.
- [ ] T030 [REQ-030] Write a failing test `tests/test_search_indexing.py::test_incident_denies_wildcard_queries`.
- [ ] T031 [REQ-031] Write a failing test `tests/test_search_indexing.py::test_healthy_client_query_allowed`.
- [ ] T032 [REQ-032] Write a failing test `tests/test_search_indexing.py::test_service_description_publishes_refresh_interval`.

## Phase 2: Implementation

- [ ] T033 [REQ-001] Implement until `tests/test_search_indexing.py::test_one_document_per_published_version` passes.
- [ ] T034 [REQ-002] Implement until `tests/test_search_indexing.py::test_verified_version_is_indexed_on_commit` passes.
- [ ] T035 [REQ-003] Implement until `tests/test_search_indexing.py::test_digest_mismatch_defers_indexing` passes.
- [ ] T036 [REQ-004] Implement until `tests/test_search_indexing.py::test_invalid_signature_defers_indexing` passes.
- [ ] T037 [REQ-005] Implement until `tests/test_search_indexing.py::test_weak_digest_algorithm_defers_indexing` passes.
- [ ] T038 [REQ-006] Implement until `tests/test_search_indexing.py::test_unsigned_version_lands_in_unverified_partition` passes.
- [ ] T039 [REQ-007] Implement until `tests/test_search_indexing.py::test_maintenance_window_queues_index_writes` passes.
- [ ] T040 [REQ-008] Implement until `tests/test_search_indexing.py::test_quota_exceeded_defers_index_writes` passes.
- [ ] T041 [REQ-009] Implement until `tests/test_search_indexing.py::test_audit_sink_down_defers_indexing` passes.
- [ ] T042 [REQ-010] Implement until `tests/test_search_indexing.py::test_index_entry_names_version_and_generation` passes.
- [ ] T043 [REQ-011] Implement until `tests/test_search_indexing.py::test_generation_commit_is_audited` passes.
- [ ] T044 [REQ-012] Implement until `tests/test_search_indexing.py::test_backing_store_lag_marks_index_stale` passes.
- [ ] T045 [REQ-013] Implement until `tests/test_search_indexing.py::test_stale_index_serves_last_committed_generation` passes.
- [ ] T046 [REQ-014] Implement until `tests/test_search_indexing.py::test_stale_response_carries_generation_and_age` passes.
- [ ] T047 [REQ-015] Implement until `tests/test_search_indexing.py::test_reindex_serves_previous_generation` passes.
- [ ] T048 [REQ-016] Implement until `tests/test_search_indexing.py::test_reindex_progress_on_status_endpoint` passes.
- [ ] T049 [REQ-017] Implement until `tests/test_search_indexing.py::test_generation_switch_is_atomic` passes.
- [ ] T050 [REQ-018] Implement until `tests/test_search_indexing.py::test_partial_generation_discarded_on_quota_exhaustion` passes.
- [ ] T051 [REQ-019] Implement until `tests/test_search_indexing.py::test_yanked_version_hidden_from_default_results` passes.
- [ ] T052 [REQ-020] Implement until `tests/test_search_indexing.py::test_yanked_version_document_retained` passes.
- [ ] T053 [REQ-021] Implement until `tests/test_search_indexing.py::test_exact_version_query_returns_yanked_marker` passes.
- [ ] T054 [REQ-022] Implement until `tests/test_search_indexing.py::test_cold_old_version_document_deleted` passes.
- [ ] T055 [REQ-023] Implement until `tests/test_search_indexing.py::test_retention_hold_blocks_document_deletion` passes.
- [ ] T056 [REQ-024] Implement until `tests/test_search_indexing.py::test_trusted_mirror_receives_generation` passes.
- [ ] T057 [REQ-025] Implement until `tests/test_search_indexing.py::test_untrusted_mirror_replication_skipped` passes.
- [ ] T058 [REQ-026] Implement until `tests/test_search_indexing.py::test_anonymous_scope_sees_public_versions_only` passes.
- [ ] T059 [REQ-027] Implement until `tests/test_search_indexing.py::test_rate_limited_query_denied_with_retry_after` passes.
- [ ] T060 [REQ-028] Implement until `tests/test_search_indexing.py::test_reindex_requires_admin_scope` passes.
- [ ] T061 [REQ-029] Implement until `tests/test_search_indexing.py::test_untrusted_transport_denied` passes.
- [ ] T062 [REQ-030] Implement until `tests/test_search_indexing.py::test_incident_denies_wildcard_queries` passes.
- [ ] T063 [REQ-031] Implement until `tests/test_search_indexing.py::test_healthy_client_query_allowed` passes.
- [ ] T064 [REQ-032] Implement until `tests/test_search_indexing.py::test_service_description_publishes_refresh_interval` passes.
