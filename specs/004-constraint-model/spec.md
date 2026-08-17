# Feature Specification: Constraint Model

**Feature Branch**: `004-constraint-model`

**Created**: 2026-08-17

**Status**: Draft

**Input**: The solver-backed half of the grounding programme, split out of the original
`002-grounded-requirement-model` so that vocabulary grounding can ship standalone with no solver
dependency. A requirement optionally carries a formal guard over vocabulary terms and a declared
effect. The gate can then find requirements that can never fire, pairs that contradict, and pairs
where one silently makes the other redundant — and, by joining a minimal conflict back to the
intentions its requirements serve, tell a genuine trade-off apart from a specification defect.

**Depends on**: `002-vocabulary-grounding` for term identity. `003-cross-spec-constraint-merge`
composes the models this feature defines.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Contradictions and dead rules are found (Priority: P1)

An author supplements a requirement's EARS prose with a guard and an effect. The gate reports
requirements whose guard can never hold, pairs whose guards overlap while their effects conflict, and
pairs where one guard implies another with the same effect.

**Why this priority**: These are findings the prose gate cannot produce at all, and they are the unit
the merge feature composes.

**Independent Test**: Write two requirements with overlapping guards and conflicting effects; assert
a conflict finding naming both, reduced to a minimal set, with a witness assignment.

**Acceptance Scenarios**:

1. **Given** a requirement whose guard is unsatisfiable, **When** the gate runs, **Then** a
   dead-guard finding is produced.
2. **Given** two requirements whose guards overlap and whose effects conflict, **When** the gate
   runs, **Then** a conflict finding carries a minimal requirement set and a witness.
3. **Given** a requirement whose guard implies another's with an identical effect, **When** the gate
   runs, **Then** a subsumption finding is produced.

---

### User Story 2 - A conflict is classified, not just reported (Priority: P1)

Knowing that two requirements contradict is half an answer. Joining each contributing requirement to
its declared intention separates two very different situations: requirements serving *different*
goals are a trade-off somebody has to adjudicate, while requirements serving the *same* goal are
simply a defect that no precedence can or should resolve.

**Why this priority**: This classification is the point of carrying intentions at all, and it is the
part of the design with no known prior art. It is also cheap once minimal conflict sets exist.

**Independent Test**: Build a conflict whose requirements share one intention and assert a
specification-defect finding; change one intention and assert it becomes a trade-off.

**Acceptance Scenarios**:

1. **Given** a minimal conflict whose requirements all serve one intention, **When** the gate runs,
   **Then** a specification-defect finding is produced.
2. **Given** a minimal conflict spanning two or more intentions, **When** the gate runs, **Then** the
   conflict is reported as a trade-off.

---

### User Story 3 - The gate never passes on a result it did not compute (Priority: P1)

A solver can time out, exceed a budget, or reject malformed input while still emitting a
satisfiability verdict on its reply stream. Any of those must be distinguishable from success.

**Why this priority**: A gate that reports success because it misread an error stream is worse than
no gate, and this failure mode has been observed in practice with duplicate assertion names.

**Independent Test**: Feed the solver malformed input and assert the run fails rather than reporting
a verdict.

**Acceptance Scenarios**:

1. **Given** an error on the solver reply stream, **When** the gate runs, **Then** the run fails
   regardless of any verdict on that stream.
2. **Given** an analysis that exceeds its budget, **When** the gate runs, **Then** the outcome is
   indeterminate rather than passing.

### Edge Cases

- The same model is evaluated twice with requirements declared in a different order.
- A minimal conflict set changes size after a solver version change.
- An integer term is compared against a constant outside its declared bounds, which is both a static
  domain error and a cause of an unsatisfiable guard.
- Two requirements declare identical guards and identical effects.
- A requirement carries a guard but no intention.

## Requirements *(mandatory)*

### Functional Requirements

- REQ-001: The validator shall read an optional constraint model file located beside each specification.
- REQ-002: Where a constraint model declares a guard, the validator shall resolve every symbol in that guard against the project vocabulary.
- REQ-003: The validator shall reject a guard expression containing any construct outside the declared guard grammar.
- REQ-004: If a requirement guard is unsatisfiable over its declared domains, then the validator shall report a dead-guard finding.
- REQ-005: When two requirements have overlapping guards and conflicting effects, the validator shall report a conflict finding.
- REQ-006: When a conflict is reported, the validator shall reduce the contributing requirements to a minimal unsatisfiable subset.
- REQ-007: The validator shall name each solver assertion after the requirement identifier that assertion encodes.
- REQ-008: When a minimal conflict set is reported, the validator shall map each contributing requirement to the intention it serves.
- REQ-009: If every requirement in a minimal conflict set serves one intention, then the validator shall report a specification-defect finding.
- REQ-010: Where a minimal conflict set spans two or more intentions, the validator shall report the conflict as a trade-off.
- REQ-011: If the solver reply stream carries an error, then the validator shall treat the run as failed.
- REQ-012: The validator shall accompany every satisfiability finding with a witness assignment over the referenced terms.
- REQ-013: If an analysis exceeds its configured budget, then the validator shall report an indeterminate outcome distinguishable from a passing outcome.
- REQ-014: Where a requirement has no constraint model entry, the validator shall record that requirement as unmodelled.
- REQ-015: When one requirement guard implies another and both declare the same effect, the validator shall report a subsumption finding.
- REQ-016: The validator shall produce one minimal conflict set for a given model irrespective of the order in which its requirements are declared.
- REQ-017: Where an integer term is compared only against literal constants, the validator shall partition that term's domain at those constants.
- REQ-018: The validator shall record the identity and version of the decision procedure in every result that depends on it.

### Key Entities

- **Guard**: A boolean expression over vocabulary terms, restricted to a declared grammar, describing
  when a requirement applies.
- **Effect**: The declared observable consequence a requirement asserts, drawn from a per-project set
  carrying a conflict relation.
- **Minimal conflict set**: The smallest set of requirements that cannot hold together. Its stability
  across runs is a correctness property, not a cosmetic one, because the classification in User
  Story 2 is computed from it.
- **Witness**: A concrete assignment over the referenced terms demonstrating a reported finding.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- SC-001: Every satisfiability finding includes a witness a reader can act on without rerunning the tool.
- SC-002: Repeated runs over an unchanged model produce identical minimal conflict sets.
- SC-003: A project may adopt the model for one requirement without any other requirement changing.
- SC-004: No solver outcome other than a computed verdict is ever reported as a pass.

## Assumptions

- Guards range over booleans, small enumerations, and integers compared against literal constants.
  Real-valued and string-valued terms are out of scope for this version.
- Temporal and ordering properties are out of scope, and the model file reserves room for them so
  that adding them later is not a breaking change.
- The decision procedure is an implementation choice. Whichever is chosen, the constraint graph is
  decomposed into independent components first, so each individual problem is expected to stay small.
- The intention layer is consumed as data and is never given to the decision procedure; assertions
  are named after requirements, and the join to intentions happens in the tool.
- Vocabulary grounding is available from `002-vocabulary-grounding`; this feature does not
  reimplement term resolution.
