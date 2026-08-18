# Tasks: Vocabulary Grounding

**Input**: `spec.md` and `traceability.toml` in `specs/002-vocabulary-grounding/`

**Tests**: Mandatory. Fourteen conformance cases under `conformance/cases/` cover the behaviour;
the scaffold is a unit test because it produces text rather than a gate verdict.

**Organization**: Grouped by the layer being added. The feature is opt-in throughout: a project that
declares no vocabulary and tags nothing is unaffected by any of it.

---

## Phase 1: Terms resolve or the gate fails

- [X] T001 Add conformance cases for a resolving tag, an undeclared tag and a definition-less term
      covering REQ-001, REQ-002, REQ-003 and REQ-004, using `conformance/cases/vocab-resolves`,
      `vocab-undeclared-term` and `vocab-missing-definition`
- [X] T002 Define the term schema with a stable identifier, label, definition and value domain for
      REQ-002 in `crates/ears-sdd/src/vocabulary.rs` and `config/vocabulary.toml.sample`
- [X] T003 Add `tags` and a singular `intent` to the traceability entry, both optional, for REQ-001
      in `crates/ears-sdd/src/traceability.rs`
- [X] T004 Report an undeclared tag and an empty definition for REQ-003 and REQ-004 in
      `crates/ears-sdd/src/vocabulary.rs`

## Phase 2: The hierarchy is a directed acyclic graph

- [X] T005 Add conformance cases for a cycle and an undeclared parent covering REQ-006 and REQ-007,
      using `conformance/cases/vocab-cycle` and `vocab-broader-undeclared`
- [X] T006 Compute the broader relation as a transitive closure and use it to decide which terms are
      in use for REQ-005 in `crates/ears-sdd/src/vocabulary.rs`
- [X] T007 Detect cycles and undeclared parents for REQ-006 and REQ-007 in
      `crates/ears-sdd/src/vocabulary.rs`

## Phase 3: A term means one thing

- [X] T008 Add conformance cases for a shared label and a domain collision covering REQ-008 and
      REQ-009, using `conformance/cases/vocab-duplicate-label` and `vocab-term-collision`
- [X] T009 Report two identifiers sharing a normalized label for REQ-008 in
      `crates/ears-sdd/src/vocabulary.rs`
- [X] T010 Merge project and feature-local vocabularies and report a redeclaration whose domain
      differs for REQ-009 in `crates/ears-sdd/src/vocabulary.rs`

## Phase 4: Intentions and precedence

- [X] T011 Add conformance cases for an undeclared intention, an unserved one and a precedence cycle
      covering REQ-011, REQ-012, REQ-013, REQ-014 and REQ-015, using
      `conformance/cases/intent-undeclared`, `intent-orphan` and `intent-precedence-cycle`
- [X] T012 Read the intention registry and report an undeclared reference for REQ-011 and REQ-012 in
      `crates/ears-sdd/src/vocabulary.rs` and `config/intentions.toml.sample`
- [X] T013 Interpret precedence as a strict partial order and report a cycle for REQ-014 and
      REQ-015 in `crates/ears-sdd/src/vocabulary.rs`

## Phase 5: The vocabulary does not rot

- [X] T014 Add conformance cases for an unreferenced term, a deprecated term and a project with no
      constraint model covering REQ-010, REQ-016 and REQ-018, using
      `conformance/cases/vocab-orphan-term`, `vocab-deprecated-term` and `vocab-without-model`
- [X] T015 Report unreferenced terms and unserved intentions as advisory, not error, for REQ-010 and
      REQ-013 in `crates/ears-sdd/src/vocabulary.rs`
- [X] T016 Warn on a deprecated tag and name its replacement for REQ-016 in
      `crates/ears-sdd/src/vocabulary.rs`
- [X] T017 Add the `vocab-init` scaffold deriving stubs from requirement subjects and backticked
      spans, emitting empty definitions so it cannot be committed unread, for REQ-017 in
      `crates/ears-sdd/src/vocabulary.rs` and `crates/ears-sdd/src/main.rs`
- [X] T018 Keep the layer independent of any constraint model for REQ-018 in
      `crates/ears-sdd/src/lib.rs`
- [X] T019 Derive candidates from condition clauses as well as subjects, for REQ-019 in
      `crates/ears-sdd/src/vocabulary.rs`
- [X] T020 Exclude declared terms and their alternative labels from proposals, for REQ-020 in
      `crates/ears-sdd/src/vocabulary.rs` and `crates/ears-sdd/src/lib.rs`
- [X] T021 Rank candidates by reference count and record it, for REQ-021 and REQ-022 in
      `crates/ears-sdd/src/vocabulary.rs`
- [X] T022 Report phrases that reduce to one identifier, for REQ-023 in
      `crates/ears-sdd/src/vocabulary.rs`
- [X] T023 Convert a SKOS concept scheme into a vocabulary, for REQ-024, REQ-026 and REQ-027 in
      `crates/ears-sdd/src/skos.rs`
- [X] T024 Express the vocabulary as a SKOS concept scheme, for REQ-025 in
      `crates/ears-sdd/src/skos.rs`
