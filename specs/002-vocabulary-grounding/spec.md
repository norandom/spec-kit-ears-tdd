# Feature Specification: Vocabulary Grounding

**Feature Branch**: `002-vocabulary-grounding`

**Created**: 2026-08-17

**Status**: Draft

**Input**: Requirements name things, and nothing today checks that two requirements naming the same
thing agree on what it is. This feature adds a project vocabulary of grounded terms and a registry of
intentions, with one rule: a term or intention is declared before it is used, and an undeclared one
fails the gate. It ships standalone, with no solver and no new dependency, because that rule is most
of what grounding actually buys and it de-risks everything downstream.

Research finding that shaped this split: of 49 candidate checks across the whole grounding programme,
37 need nothing but graph code and none need a description-logic reasoner. Subsumption over a term
hierarchy is transitive closure; equivalence is an alternate label; disjointness is an axiom for the
constraint layer. This specification covers the graph-only half.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - A term means one thing across the project (Priority: P1)

Two specifications describe the same underlying setting under different names, or the same name with
different value domains. Nothing notices, because there is no shared identity for the things
requirements talk about. A project vocabulary declares each term once, and every reference resolves
against it or fails.

**Why this priority**: Term identity is the join key for every later analysis. Retrofitting identity
onto references that were never required to be unique is not soundly possible, so this cannot be
deferred.

**Independent Test**: Declare a vocabulary, reference an undeclared term, assert the run fails naming
the term and the file.

**Acceptance Scenarios**:

1. **Given** a requirement referencing a declared term, **When** the gate runs, **Then** the
   reference resolves and no finding is produced.
2. **Given** a requirement referencing an undeclared term, **When** the gate runs, **Then** an
   unresolved-term finding names the term.
3. **Given** two specifications declaring one term identifier with different value domains, **When**
   the gate runs, **Then** a term-collision finding is produced.

---

### User Story 2 - Requirements record why they exist (Priority: P2)

A requirement without a recorded reason cannot be argued about later. An intention registry declares
each reason once, with a rationale, and requirements reference intentions by identifier. Precedence
between intentions is declarable, which is what makes a deliberate override reviewable rather than
silent.

**Why this priority**: The reasons are cheap to record while the decision is fresh and impossible to
reconstruct afterwards. The precedence relation is also the input the later conflict layer needs, so
recording it early costs nothing and unblocks that feature.

**Independent Test**: Reference an undeclared intention and assert the run fails; declare a
precedence cycle and assert the run fails.

**Acceptance Scenarios**:

1. **Given** a requirement referencing an undeclared intention, **When** the gate runs, **Then** an
   unresolved-intention finding is produced.
2. **Given** a precedence relation containing a cycle, **When** the gate runs, **Then** a
   precedence-cycle finding is produced.
3. **Given** an intention no requirement serves, **When** the gate runs, **Then** an advisory finding
   is produced.

---

### User Story 3 - The vocabulary does not rot (Priority: P2)

A vocabulary that nobody prunes degrades into hundreds of singleton terms that ground nothing, and
the feature becomes ceremony. The tool pays the authoring cost by scaffolding terms from existing
prose, and reports terms that are unused, referenced once, or near-duplicates of each other.

**Why this priority**: A blank vocabulary file is the single most likely way this feature dies in
adoption. Scaffolding and reporting are what keep it alive past the first quarter.

**Independent Test**: Run the scaffold against an existing feature with no vocabulary and assert it
proposes term stubs drawn from that feature's prose.

**Acceptance Scenarios**:

1. **Given** a project with specifications but no vocabulary, **When** the scaffold runs, **Then**
   term stubs derived from the existing prose are proposed.
2. **Given** a vocabulary containing an unreferenced term, **When** the report runs, **Then** that
   term is listed.

### Edge Cases

- A term declares itself, directly or transitively, as its own broader term.
- Two terms differ only by capitalization or by hyphen versus underscore.
- A term is deprecated while requirements still reference it.
- The vocabulary declares a term whose identifier collides with a reserved symbol of the later
  constraint layer.
- Every requirement in a feature carries the same single intention.

## Requirements *(mandatory)*

### Functional Requirements

- REQ-001: The validator shall read a project vocabulary declaring the terms that requirements reference.
- REQ-002: The vocabulary shall declare each term with a stable identifier, a human-readable label, a definition, and a value domain.
- REQ-003: If a term declaration omits its definition, then the validator shall report a vocabulary finding.
- REQ-004: If a requirement references a term that the vocabulary does not declare, then the validator shall report an unresolved-term finding.
- REQ-005: Where a term declares a broader term, the validator shall compute the broader relation as a transitive closure over a directed acyclic graph.
- REQ-006: If the broader relation contains a cycle, then the validator shall report a vocabulary-cycle finding.
- REQ-007: If a broader relation names an undeclared term, then the validator shall report an unresolved-term finding.
- REQ-008: When two terms declare the same normalized label, the validator shall report a duplicate-label finding.
- REQ-009: When two specifications declare one term identifier with differing value domains, the validator shall report a term-collision finding.
- REQ-010: Where a declared term is referenced by no requirement, the validator shall report an advisory finding.
- REQ-011: The validator shall read a project registry declaring the intentions that requirements serve.
- REQ-012: If a requirement references an intention that the registry does not declare, then the validator shall report an unresolved-intention finding.
- REQ-013: If a declared intention is served by no requirement, then the validator shall report an advisory finding.
- REQ-014: Where intentions declare a precedence, the validator shall interpret the precedence relation as a strict partial order.
- REQ-015: If the precedence relation contains a cycle, then the validator shall report a precedence-cycle finding.
- REQ-016: Where a vocabulary term is marked deprecated, the validator shall report a warning naming its replacement.
- REQ-017: When the vocabulary scaffold runs, it shall propose term stubs derived from existing specification prose.
- REQ-018: The validator shall evaluate the vocabulary and the intention registry without requiring any constraint model.
- REQ-019: When proposing vocabulary, the validator shall derive candidates from the condition clause of a requirement as well as from its subject.
- REQ-020: When proposing vocabulary, the validator shall exclude every term the project already declares, including its alternative labels.
- REQ-021: When proposing vocabulary, the validator shall order candidates by the number of requirements that reference each one.
- REQ-022: When proposing vocabulary, the validator shall record how many requirements reference each candidate.
- REQ-023: Where two distinct phrases reduce to one identifier, the validator shall report both rather than choosing between them.
- REQ-024: The validator shall convert a SKOS concept scheme into a project vocabulary.
- REQ-025: The validator shall express the project vocabulary as a SKOS concept scheme.
- REQ-026: Where an imported concept declares no value domain, the validator shall record that term as an entity.
- REQ-027: Where an imported concept declares no definition, the validator shall record an empty definition rather than composing one.

### Key Entities

- **Term**: The globally identified thing a requirement constrains, with an identifier, label,
  definition, value domain, and optional broader terms. Terms are the nodes any later merge joins on.
- **Vocabulary**: The project-level registry of terms.
- **Intention**: A declared reason a requirement exists, carrying a rationale.
- **Precedence**: A declared pairwise ordering between intentions, recording that one deliberately
  overrides another.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- SC-001: A reference to an undeclared term or intention fails the gate in every case.
- SC-002: The feature adds no runtime dependency and no per-feature file.
- SC-003: A project can adopt the vocabulary for one feature without any other feature changing.
- SC-004: Every check in this feature is answerable by graph traversal over declared data.

## Assumptions

- The vocabulary and intention registry are project-level TOML, authored by engineers rather than by
  ontologists.
- Interchange formats are out of scope. If an adopter later needs one, it is an export, never the
  internal representation.
- No description-logic reasoner is used or required; the broader relation is treated as transitive by
  the tool, which is the conformant reading since the corresponding standard property is explicitly
  non-transitive.
- Guards, effects, and any solver-backed analysis belong to `004-constraint-model` and are out of
  scope here.
