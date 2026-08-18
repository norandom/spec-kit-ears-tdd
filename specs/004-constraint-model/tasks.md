# Tasks: Constraint Model

**Input**: `spec.md` and `traceability.toml` in `specs/004-constraint-model/`

**Tests**: Mandatory. Twelve conformance cases cover the gate behaviour; the properties that are
about the procedures rather than the verdict — region coverage, budget refusal, witness verification,
and agreement between the two procedures — are Rust tests.

**Organization**: Grouped so that everything shared by both decision procedures lands before either
of them, and the differential harness lands after both.

---

## Phase 1: The shared foundation

- [X] T001 Define the model schema with effects carrying a symmetric conflict relation for REQ-001
      in `config/model.toml.sample`
- [X] T002 Add the guard grammar, its parser, and term and comparison extraction for REQ-002 and
      REQ-003 in `crates/ears-sdd/src/guard.rs`, refusing term-to-term comparison because it needs
      arithmetic the fragment lacks and would invalidate the region abstraction
- [X] T003 Finitize domains, cutting integers at the constants their guards compare against, for
      REQ-017 and REQ-020 in `crates/ears-sdd/src/model.rs`
- [X] T004 Assert that an integer domain's regions cover it exactly once for REQ-025 in
      `crates/ears-sdd/src/model.rs`
- [X] T005 Decompose requirements by shared terms, with unconditional ones joining every component,
      and assert the partition for REQ-026 in `crates/ears-sdd/src/model.rs` and
      `crates/ears-sdd/src/analysis.rs`

## Phase 2: The reference procedure

- [X] T006 Add exhaustive enumeration over finitized domains, with assignments decoded on demand so
      the order depends only on the variable order, in `crates/ears-sdd/src/enumerate.rs`
- [X] T007 Re-evaluate every witness against the guards attributed to it before reporting, for
      REQ-024 in `crates/ears-sdd/src/enumerate.rs`
- [X] T008 Choose the lowest satisfying index so the same model always yields the same
      counterexample, for REQ-016 in `crates/ears-sdd/src/enumerate.rs`

## Phase 3: Findings and safeguards

- [X] T009 Add conformance cases for a dead guard, an impossible conflict, a real conflict and
      subsumption covering REQ-004, REQ-005 and REQ-015, using `conformance/cases/model-dead-guard`,
      `model-conflict-impossible`, `model-conflict-unclassified` and `model-subsumed`
- [X] T010 Report dead guards, conflicts and subsumption, each carrying a witness, for REQ-004,
      REQ-005, REQ-012 and REQ-015 in `crates/ears-sdd/src/analysis.rs`
- [X] T011 Refuse an over-budget component before evaluating any state of it, naming the terms
      driving the product, for REQ-021, REQ-022 and REQ-023 in `crates/ears-sdd/src/analysis.rs`
      and `crates/ears-sdd/src/config.rs`
- [X] T012 Type-check guards against declared domains for REQ-002, and add
      `conformance/cases/model-type-mismatch` — without it a boolean test on an enumeration reports
      as a dead requirement rather than a wrong guard
- [X] T013 Record unmodelled requirements as advisory for REQ-014, with
      `conformance/cases/model-partially-modelled`

## Phase 4: Classifying conflicts by intention

- [X] T014 Add conformance cases for a defect, an unadjudicated trade-off and an adjudicated one
      covering REQ-008, REQ-009 and REQ-010, using `conformance/cases/model-conflict-defect`,
      `model-conflict-unadjudicated` and `model-conflict-adjudicated`
- [X] T015 Close the precedence relation transitively and test for a unique maximum rather than a
      maximal element, for REQ-010 in `crates/ears-sdd/src/adjudicate.rs`
- [X] T016 Classify a conflict whose requirements share one intention as a defect no precedence can
      adjudicate, for REQ-009 in `crates/ears-sdd/src/adjudicate.rs`
- [X] T017 Name the specific comparisons that would resolve an unadjudicated conflict, for REQ-008
      in `crates/ears-sdd/src/adjudicate.rs`
- [X] T018 Reduce a conflict to its minimal form for REQ-006 in `crates/ears-sdd/src/analysis.rs`.
      Conflicts here are pairs, so the pair is already minimal; larger minimal sets would need
      declared invariants, which this version does not have, and that is recorded in the code

## Phase 5: The second procedure and the check on it

- [X] T019 Add the decision-diagram procedure over the same finitized variables, one-hot per term
      with contiguous blocks and no dynamic reordering, for REQ-019 in
      `crates/ears-sdd/src/bdd.rs`
- [X] T020 Constrain every query by the exactly-one domain, so no state where a term holds two
      values is ever considered, for REQ-020 in `crates/ears-sdd/src/bdd.rs`
- [X] T021 Assert the two procedures agree on satisfiability, overlap and implication across
      booleans, enumerations, integer regions and mixed domains for REQ-019 in
      `crates/ears-sdd/tests/differential.rs`
- [X] T022 Add an exhaustive agreement test over every two-variable guard for REQ-019 in
      `crates/ears-sdd/tests/differential.rs`, so a systematic encoding error cannot hide in a case
      nobody wrote by hand

## Phase 6: Wiring

- [X] T023 Run the model layer wherever traceability runs, loading terms, intentions and precedence
      once per run, for REQ-001 and REQ-007 in `crates/ears-sdd/src/lib.rs`
- [X] T024 Report the modelled and component counts for REQ-018 in `crates/ears-sdd/src/report.rs`
- [X] T025 Add `conformance/cases/model-absent-is-silent` and `model-effect-undeclared` for REQ-001
      and REQ-011
- [X] T026 Fail rather than pass when a procedure returns no verdict, for REQ-011 and REQ-013 in
      `crates/ears-sdd/src/analysis.rs`
- [X] T027 Scope each satisfiability question to the terms its requirements name, for REQ-027 and
      REQ-021 in `crates/ears-sdd/src/model.rs` and `crates/ears-sdd/src/analysis.rs`
- [X] T028 Group requirements with conflicting effects together regardless of shared guard terms,
      for REQ-028 in `crates/ears-sdd/src/model.rs`
- [X] T029 Encode each distinct guard once per component, for REQ-029 in
      `crates/ears-sdd/src/analysis.rs`
- [X] T030 Reject a partition separating a conflicting pair, for REQ-030 in
      `crates/ears-sdd/src/analysis.rs`
