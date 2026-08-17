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

A decision procedure can exceed its budget or fail to reach a verdict. Either must be
distinguishable from success, and neither may be reported as a pass.

**Why this priority**: A gate that reports success because it misread a non-answer is worse than no
gate at all.

**Independent Test**: Configure a budget smaller than the model requires and assert the run reports
indeterminate rather than passing.

**Acceptance Scenarios**:

1. **Given** an analysis that exceeds its budget, **When** the gate runs, **Then** the outcome is
   indeterminate rather than passing.
2. **Given** a model evaluated by two different decision procedures, **When** both complete, **Then**
   they report the same findings.
3. **Given** a term whose domain is an enumeration, **When** the model is evaluated, **Then** no
   assignment outside that enumeration is considered.

### Edge Cases

- A component's state space is large enough that evaluating it would not finish. Its size is a
  product of declared domain sizes and so is known before any search begins; there is no reason to
  start one.
- The same analysis is run on a fast machine and a loaded one. A bound measured in elapsed time
  would give different verdicts, which would make the corpus stop being a contract.
- A reported witness does not actually satisfy the guards attributed to it, because the encoding is
  wrong rather than the model.
- An integer domain's regions leave a gap, so some values are never considered and a conflict hides
  in the gap.
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
- REQ-011: If the decision procedure does not return a verdict, then the validator shall treat the run as failed rather than as passing.
- REQ-019: The validator shall produce the same findings for one model under every decision procedure it supports.
- REQ-020: Where a term's domain is an enumeration or a bounded integer, the validator shall encode it so that no assignment outside that domain is considered.
- REQ-021: Before evaluating a component, the validator shall compute the size of its state space and report an over-budget component without evaluating it.
- REQ-022: The validator shall bound every analysis by a count rather than by elapsed time.
- REQ-023: When a component exceeds its budget, the validator shall name the component, its variable count, its state count, and the terms contributing most to that count.
- REQ-024: Before reporting a witness, the validator shall confirm that the witness satisfies the guards the finding attributes to it.
- REQ-025: The validator shall confirm that the regions of an integer domain cover that domain exactly once.
- REQ-026: The validator shall confirm that every modelled requirement belongs to exactly one component.
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
- SC-004: No outcome other than a computed verdict is ever reported as a pass.
- SC-005: An over-budget component is reported without any state of it being evaluated.
- SC-006: The same model and budget produce the same verdict on any machine.
- SC-007: An over-budget report names a specific lever, so the reader's next action is narrowing a
  named term rather than raising the budget.

## Assumptions

- Guards range over booleans, small enumerations, and integers compared against literal constants.
  Real-valued and string-valued terms are out of scope for this version.
- Two decision procedures are implemented rather than one: exhaustive enumeration over the finitized
  domains, which is obviously correct and trivially deterministic, and a reduced ordered binary
  decision diagram, which scales past enumeration when a component is large but structured. The
  enumerator exists as much to check the diagram's encoding as to answer queries — one-hot
  constraints, interval boundaries, and variable order are where a decision-diagram encoding goes
  quietly wrong, and disagreement between two independent procedures is the cheapest way to catch it.
- An integer domain is partitioned at the constants its guards compare against, so a bound of ten
  thousand costs a handful of regions rather than ten thousand values. This is sound and complete
  only while comparisons are against literals, which is what the fragment above allows.
- Temporal and ordering properties are out of scope, and the model file reserves room for them so
  that adding them later is not a breaking change.
- The decision procedure is an implementation choice. Whichever is chosen, the constraint graph is
  decomposed into independent components first, so each individual problem is expected to stay small.
- The intention layer is consumed as data and is never given to the decision procedure; assertions
  are named after requirements, and the join to intentions happens in the tool.
- Vocabulary grounding is available from `002-vocabulary-grounding`; this feature does not
  reimplement term resolution.
