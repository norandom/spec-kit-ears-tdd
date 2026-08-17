# Feature Specification: Cross-Specification Constraint Merge

**Feature Branch**: `003-cross-spec-constraint-merge`

**Created**: 2026-08-17

**Status**: Draft

**Input**: Individually satisfiable specifications can be jointly unsatisfiable. A project that
accumulates features over time accumulates cross-feature contradictions that no per-feature gate can
see. This feature merges the constraint models of every specification into one graph and verifies the
whole, reporting which specifications contribute to each contradiction.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Contradictions across specifications are found (Priority: P1)

Feature A constrains a setting one way; feature B, written months later by someone else, constrains
the same setting another way. Each specification passes its own gate. Together they describe a system
that cannot exist. Merging the constraint models and checking the union exposes this.

**Why this priority**: This is the entire purpose of the feature. A per-specification gate is
structurally incapable of finding it, so there is no partial substitute.

**Independent Test**: Write two specifications that are individually satisfiable and jointly
unsatisfiable, and assert the merge gate fails and names both specifications.

**Acceptance Scenarios**:

1. **Given** two specifications that are individually satisfiable and jointly unsatisfiable, **When**
   the merge gate runs, **Then** a merge-conflict finding names both specifications.
2. **Given** a merge conflict involving five requirements of which three suffice to cause it,
   **When** the conflict is reported, **Then** the reported requirement set is minimal.
3. **Given** a merge conflict, **When** it is reported, **Then** the shared terms responsible are
   named.

---

### User Story 2 - The merge decomposes rather than globalizing (Priority: P1)

Requirements that share no term cannot interact. Rather than checking one large combined problem, the
merge partitions the union into independent components and checks each. The report attributes every
finding to a component, so a reader sees a small local problem instead of a large opaque one.

**Why this priority**: Decomposition is what makes the check tractable and the output legible. It
also bounds the cost of whatever decision procedure is chosen, which is why that choice can stay
open.

**Independent Test**: Merge a project whose specifications share no terms and assert the number of
reported components equals the number of specifications.

**Acceptance Scenarios**:

1. **Given** a merged model whose requirements form several disjoint groups, **When** the merge gate
   runs, **Then** each group is evaluated independently.
2. **Given** any completed merge run, **When** the result is produced, **Then** the number of
   evaluated components is reported.

---

### User Story 3 - Conflicts are resolved by declared intent, not by silence (Priority: P2)

Some cross-specification conflicts are deliberate: a later feature is meant to override an earlier
one. That is a decision about competing intentions, and it belongs in the record. A conflict covered
by a declared precedence between intentions is reported as resolved; one that is not is an error.

**Why this priority**: Without this, teams suppress merge findings by weakening the model, and the
gate stops meaning anything. With it, every deliberate override becomes a reviewable artifact.

**Independent Test**: Create a conflict between requirements whose intentions declare a precedence
and assert it is reported as resolved; remove the precedence and assert it becomes an error.

**Acceptance Scenarios**:

1. **Given** conflicting requirements whose intentions declare a precedence, **When** the merge gate
   runs, **Then** the conflict is reported as resolved.
2. **Given** conflicting requirements whose intentions declare no precedence, **When** the merge gate
   runs, **Then** the conflict is reported as unresolved.
3. **Given** a requirement in one specification that subsumes a requirement in another on a shared
   term, **When** the merge gate runs, **Then** a shadowing finding is produced.

### Edge Cases

- A specification declares no constraint model at all.
- Two specifications declare a precedence between the same pair of intentions in opposite directions.
- A component contains requirements from a single specification only.
- Every specification is individually unsatisfiable, so the merge adds no information.
- A term is declared in the vocabulary but referenced by no specification.

## Requirements *(mandatory)*

### Functional Requirements

- REQ-001: Where the merge gate is requested, the validator shall combine the constraint models of every discovered specification into one constraint graph.
- REQ-002: The constraint graph shall represent each vocabulary term as a node and each modelled requirement as an edge over the terms it references.
- REQ-003: When the constraint graph is built, the validator shall partition it into connected components.
- REQ-004: The validator shall evaluate each connected component independently of every other component.
- REQ-005: If requirements drawn from two or more specifications are jointly unsatisfiable, then the validator shall report a merge-conflict finding naming every contributing specification.
- REQ-006: When a merge conflict is reported, the validator shall reduce the contributing requirements to a minimal unsatisfiable subset.
- REQ-007: When a merge conflict is reported, the validator shall name the vocabulary terms shared by the conflicting requirements.
- REQ-008: If a requirement in one specification subsumes a requirement in another specification over a shared term, then the validator shall report a shadowing finding.
- REQ-009: When the merge gate completes, the validator shall report the number of connected components it evaluated.
- REQ-010: If a specification declares no constraint model, then the validator shall exclude that specification from the merge and report it as unmerged.
- REQ-011: While the merge gate is running, the validator shall leave every specification and constraint model unmodified.
- REQ-012: Where the intentions of conflicting requirements declare a precedence, the validator shall report that conflict as resolved.
- REQ-013: If a merge conflict involves requirements whose intentions declare no precedence, then the validator shall report the conflict as unresolved.
- REQ-014: If two intentions declare precedence over each other, then the validator shall report a precedence-cycle finding.
- REQ-015: The validator shall attribute every merge finding to the connected component in which it arose.

### Key Entities

- **Constraint graph**: The union of every specification's constraint model. Terms are nodes;
  modelled requirements are edges over the terms they reference.
- **Component**: A maximal set of requirements connected through shared terms. Components are
  independent by construction and are the unit of evaluation and attribution.
- **Merge conflict**: A jointly unsatisfiable set of requirements spanning two or more
  specifications, reduced to a minimal subset.
- **Precedence**: A declared ordering between intentions, used to mark a conflict as a deliberate
  override rather than a defect.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- SC-001: Two individually passing specifications that jointly contradict cause the merge gate to fail.
- SC-002: Every merge conflict report names the contributing specifications, the minimal requirement
  set, and the shared terms.
- SC-003: Adding a specification that shares no terms with any existing specification does not change
  the evaluation of any existing component.
- SC-004: Every deliberate cross-specification override is represented by a declared precedence.

## Assumptions

- Complete requirement discovery across all features is available; this feature depends on it and
  does not reimplement scoping.
- Every specification participating in a merge grounds its terms in the shared project vocabulary.
- Requirements without a constraint model are excluded from the merge rather than assumed consistent.
- The decision procedure used per component is an implementation choice; decomposition is expected to
  keep components small enough that the choice is not load-bearing.
- Conflict resolution by precedence records a decision; it does not verify that the decision is correct.
