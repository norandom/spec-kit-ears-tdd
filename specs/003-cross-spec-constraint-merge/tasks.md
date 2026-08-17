# Tasks: Cross-Specification Constraint Merge

**Input**: `spec.md` and `traceability.toml` in `specs/003-cross-spec-constraint-merge/`

**Tests**: Mandatory. Seven conformance cases, each running with all-features scope because a merge
of one specification is not a merge.

**Organization**: The union comes first, then what it can find, then the classification it inherits
from the constraint model.

---

## Phase 1: One graph from many specifications

- [X] T001 Add conformance cases for a cross-specification conflict, disjoint terms and disjoint
      guards covering REQ-001, REQ-002, REQ-003 and REQ-004, using
      `conformance/cases/merge-cross-spec-conflict`, `merge-disjoint-terms` and `merge-no-overlap`
- [X] T002 Combine every specification's constraint model into one pool and decompose it by shared
      terms for REQ-001, REQ-002 and REQ-003 in `crates/ears-sdd/src/analysis.rs`
- [X] T003 Evaluate each component independently, reusing the per-feature machinery, for REQ-004 in
      `crates/ears-sdd/src/analysis.rs`
- [X] T004 Report the merged component count for REQ-009 in `crates/ears-sdd/src/report.rs`

## Phase 2: What only the merge can see

- [X] T005 Report a conflict spanning two or more specifications, naming each contributing one, for
      REQ-005 and REQ-007 in `crates/ears-sdd/src/analysis.rs`
- [X] T006 Skip pairs from a single specification, which its own run has already reported, so the
      merge surfaces only what is new, for REQ-005 in `crates/ears-sdd/src/analysis.rs`
- [X] T007 Name the terms a conflicting component shares for REQ-007 in
      `crates/ears-sdd/src/analysis.rs`
- [X] T008 Report cross-specification subsumption as shadowing for REQ-008 in
      `crates/ears-sdd/src/analysis.rs`
- [X] T009 Attribute every merge finding to its component for REQ-015 in
      `crates/ears-sdd/src/analysis.rs`
- [X] T010 Reduce a merge conflict to its minimal form for REQ-006 in
      `crates/ears-sdd/src/analysis.rs`; conflicts are pairs here, so the pair is already minimal

## Phase 3: Honesty about coverage

- [X] T011 Exclude a specification with no constraint model and report it, but only once a merge has
      actually happened, for REQ-010 in `crates/ears-sdd/src/analysis.rs` with
      `conformance/cases/merge-unmerged-spec` — a project declaring no models at all should hear
      nothing from this layer
- [X] T012 Leave every specification and model unmodified for REQ-011 in
      `crates/ears-sdd/tests/read_only.rs`

## Phase 4: Deciding who wins

- [X] T013 Add conformance cases for a defect, an unadjudicated trade-off and an adjudicated one
      covering REQ-012 and REQ-013, using `conformance/cases/merge-conflict-defect`,
      `merge-conflict-unadjudicated` and `merge-conflict-adjudicated`
- [X] T014 Classify a merge conflict by the intentions its requirements serve, reusing the
      unique-maximum test, for REQ-012 and REQ-013 in `crates/ears-sdd/src/analysis.rs`
- [X] T015 Report a precedence cycle for REQ-014 in `crates/ears-sdd/src/vocabulary.rs` and
      `crates/ears-sdd/src/adjudicate.rs`
