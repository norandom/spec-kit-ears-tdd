# Tasks: Complete Requirement Discovery

**Input**: `spec.md` and `traceability.toml` in `specs/001-complete-requirement-discovery/`

**Tests**: Mandatory. Every behavioural task is verified by a conformance case under
`conformance/cases/`, which is data rather than test code, so the case survives a reimplementation
in another language. Cases were written before the behaviour existed; several passed against the old
implementation, and that is what the defect looked like from outside.

**Organization**: Grouped by the property being restored. Selectors are the case directories named in
`traceability.toml`.

---

## Phase 1: Discovery sees every requirement

- [X] T001 Add conformance cases for the five Markdown forms, fenced-block exclusion, duplicate
      identifiers and wide identifiers covering REQ-001 through REQ-004, using
      `conformance/cases/markdown-forms`, `fenced-code-block`, `duplicate-identifier` and
      `wide-identifier`
- [X] T002 Implement bullet, numbered, heading, table-row and block-quote parsing with fenced-block
      tracking for REQ-001 and REQ-002 in `crates/ears-sdd/src/requirements.rs`
- [X] T003 Accept identifiers of three or more digits and report duplicates for REQ-003 and REQ-004
      in `crates/ears-sdd/src/requirements.rs`

## Phase 2: Scope is explicit and honest

- [X] T004 Add conformance cases for all-features mode, a stale feature pointer and a feature
      outside the project covering REQ-005 and REQ-006, using
      `conformance/cases/all-features-scope`, `stale-feature-pointer`, `pinned-feature-scope` and
      `feature-outside-project`
- [X] T005 Implement scope resolution as flag, then environment, then feature pointer, then glob,
      with `--all` overriding, for REQ-005 in `crates/ears-sdd/src/discovery.rs`
- [X] T006 Report a stale pointer and refuse a feature resolving outside the project for REQ-006 in
      `crates/ears-sdd/src/discovery.rs`

## Phase 3: The report is a contract

- [X] T007 Add conformance cases asserting the schema version and the examined-file count for
      REQ-007 and REQ-008 in `conformance/cases/all-features-scope`
- [X] T008 Add the schema version, the provenance block and the examined counts for REQ-007 and
      REQ-008 in `crates/ears-sdd/src/report.rs`
- [X] T009 Run every case through the command-line interface for REQ-015 and REQ-016 in
      `crates/ears-sdd/tests/conformance.rs`

## Phase 4: Traceability means something

- [X] T010 Add conformance cases for traversal selectors, undecodable specifications and quoted
      modals covering REQ-009, REQ-010 and REQ-012, using `conformance/cases/selector-traversal`,
      `undecodable-spec` and `modal-in-quoted-literal`
- [X] T011 Reject a traversal selector rather than normalizing it into the test roots for REQ-009 in
      `crates/ears-sdd/src/traceability.rs`
- [X] T012 Report an undecodable specification and accept a byte order mark for REQ-010 in
      `crates/ears-sdd/src/requirements.rs`
- [X] T013 Mask balanced quoted literals and ignore hyphenated compounds for REQ-012 in
      `crates/ears-sdd/src/ears.rs`
- [X] T014 Add conformance cases for a missing, present, nested, anchored and bare test selector
      covering REQ-017 and REQ-018, using `conformance/cases/selector-missing-test`,
      `selector-present-test`, `selector-nested-test`, `selector-anchor-style` and
      `selector-bare-file`
- [X] T015 Verify the selector's final segment appears in its file for REQ-017 and REQ-018 in
      `crates/ears-sdd/src/traceability.rs`

## Phase 5: Findings survive multiple features

- [X] T016 Add a conformance case for two features declaring the same identifier with a shared leak
      for REQ-013 and REQ-014 in `conformance/cases/feature-qualified-ids`
- [X] T017 Qualify findings by their owning feature and deduplicate on that qualification for
      REQ-013 and REQ-014 in `crates/ears-sdd/src/separation.rs`

## Phase 6: Validation is read-only

- [X] T018 Assert every gate leaves the project byte-identical in both scope modes for REQ-011 in
      `crates/ears-sdd/tests/read_only.rs`
