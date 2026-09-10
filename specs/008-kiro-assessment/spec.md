# Feature Specification: Kiro Project Assessment

**Feature Branch**: `008-kiro-assessment`

**Created**: 2026-09-10

**Status**: Draft

**Input**: A validator that only reads Spec Kit `spec.md` files cannot assess a Kiro project.
Kiro stores one feature per directory, writes EARS into `requirements.md`, and records lifecycle
in phase metadata. Some of those features are live, some are reserved slots, and some have been
superseded by later work. Treating every file on disk as current invents contradictions that were
already resolved by replacement. Ignoring superseded files without checking that a live successor
exists hides the opposite defect: a retirement with no replacement.

This feature discovers a Kiro specification tree, parses its requirement grammars, computes the
active baseline, and checks that every supersession has a live counterpart. It does not write into
the assessed project. It does not search Kiro prose for logical contradictions; that remains the
existing constraint-model gate, applied later to the active baseline once models exist.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - A Kiro tree is visible without becoming Spec Kit (Priority: P1)

An operator points the validator at a Kiro project such as facdrone. Today the run looks for Spec
Kit artifacts, finds none, and either errors or reports an empty success. The operator needs every
feature directory under the Kiro tree to be discovered and every acceptance criterion parsed, using
the grammars those documents actually use, without installing Spec Kit files into that project and
without changing any file there.

**Why this priority**: Nothing else in this feature can run on a set the gate did not see. A miss
here is not a degraded result; it is a wrong result that looks identical to a correct one.

**Independent Test**: Point the validator at a fixture Kiro tree with several feature directories
and no Spec Kit configuration; assert every requirements document is parsed, the result names how
many directories were examined, and the fixture files are byte-identical afterwards.

**Acceptance Scenarios**:

1. **Given** a project whose specifications live in a Kiro tree and which contains no Spec Kit
   configuration, **When** the validator runs, **Then** every feature directory that holds a
   requirements document is discovered and parsed.
2. **Given** a Spec Kit project with no Kiro tree, **When** the validator runs, **Then** Spec Kit
   discovery is unchanged.
3. **Given** any assessment run, **When** it completes, **Then** no file in the assessed project
   has been created, deleted, or modified.

---

### User Story 2 - Only current specifications are in force (Priority: P1)

A Kiro project accumulates features. Some are reserved or only initialized. Some were recut or
replaced and kept on disk for history, with phase metadata marking them superseded. An assessment
that folds those historical requirements into the current set will report conflicts the project
already decided. An assessment that drops them silently will not notice a retirement with no
successor.

**Why this priority**: The difference between a contradiction and a replacement is baseline
membership. Without an active baseline, later contradiction finding cannot be honest.

**Independent Test**: Build a tree with one live specification, one superseded specification that
names the live one as successor, one reserved slot, and one initialized slot; assert the active
baseline contains only the live specification and the report names each exclusion and its reason.

**Acceptance Scenarios**:

1. **Given** specifications marked superseded, reserved, and initialized beside live ones, **When**
   the active baseline is computed, **Then** only the live specifications are included.
2. **Given** a specification with no phase metadata, **When** the active baseline is computed,
   **Then** that specification is included.
3. **Given** a completed baseline computation, **When** the result is produced, **Then** it reports
   how many specifications were included, how many were excluded, and why each exclusion happened.

---

### User Story 3 - A supersession is a checked link, not a comment (Priority: P1)

When requirements change, the old specification is kept and marked superseded, and a later
specification takes over the obligation. That is a replacement. Two live specifications that both
claim the same obligation is not. The gate must distinguish them using the metadata and
relationship prose the Kiro project already writes: phase notes that name a successor, and live
specifications that claim to supersede a named predecessor. Amendments that extend a frozen
baseline are refinements, not replacements.

**Why this priority**: This is the requirement the feature exists to make testable. Without it,
"requirements changed" and "requirements contradict" remain the same finding.

**Independent Test**: (a) a superseded specification whose notes name a live successor passes the
supersession check; (b) a superseded specification that names no live successor is reported as
dangling; (c) a live specification that claims to supersede another live specification is reported
as unreciprocated; (d) an amendment that extends a baseline requirement is not treated as a
supersession.

**Acceptance Scenarios**:

1. **Given** a superseded specification whose phase notes name a specification in the active
   baseline, **When** the supersession check runs, **Then** the link is recorded and no
   dangling-supersession finding is produced.
2. **Given** a superseded specification with no link to any specification in the active baseline,
   **When** the supersession check runs, **Then** a dangling-supersession finding names it.
3. **Given** a specification in the active baseline that claims to supersede a discovered
   specification which is not marked superseded, **When** the supersession check runs, **Then** an
   unreciprocated-supersession finding names both.
4. **Given** a superseded specification whose notes name two live successors, **When** the
   supersession check runs, **Then** the recut is accepted.
5. **Given** an amendment heading that extends a baseline requirement in the same specification,
   **When** the supersession check runs, **Then** no supersession is inferred from that extension.

---

### User Story 4 - An assessment that did not look for contradictions says so (Priority: P2)

The operator asked to assess contradicting requirements. This feature does not do that: Kiro trees
have no constraint models, and this project refuses to invent contradictions from English. A green
result that omitted the search will be read as "no contradictions." The result must name the
omission. Spec Kit identifier, verification-mapping, task, and separation checks do not apply to
Kiro documents and must not fail the run for lacking artifacts those documents never had.

**Why this priority**: A narrowed claim is acceptable. A narrowed claim that looks broad is the
failure mode this tool exists to prevent.

**Independent Test**: Assess a Kiro fixture with no constraint models; assert the result states
that constraint analysis was not performed, and that it contains no Spec Kit identifier or
traceability findings against those Kiro documents.

**Acceptance Scenarios**:

1. **Given** a Kiro project with no constraint models, **When** the validator runs, **Then** the
   result states that constraint analysis was not performed.
2. **Given** Kiro specifications that do not use Spec Kit identifiers or verification mappings,
   **When** the validator runs, **Then** those specifications produce no Spec Kit identifier,
   verification-mapping, task-coverage, or separation findings.

### Edge Cases

- A feature directory exists under the Kiro tree with no requirements document.
- A requirements document holds only a placeholder comment or an introduction, with no requirement
  headings.
- A baseline requirement heading has a title and no acceptance criteria.
- Requirement-shaped text appears inside a fenced example block.
- `spec.json` is missing (native Kiro); the specification is in force.
- `spec.json` is present but not readable; the specification cannot be trusted into the baseline.
- Phase notes name a feature directory that is not in the discovered tree.
- A live specification's acceptance criteria use the word "supersede" for a product behaviour
  (for example replacing a stored run), not for another specification.
- A live specification mentions a superseded predecessor in passing without claiming to replace it.
- One predecessor is recut into several live successors.
- Two live specifications both name the same superseded predecessor; the predecessor is not
  dangling.
- An amendment extends a requirement in a specification that is itself superseded; the amendment
  leaves the baseline with its parent.
- The assessed project also contains a Spec Kit tree; each family is discovered under its own
  rules.
- Constraint analysis is out of scope for this feature and must not be silently reported as a pass.

## Requirements *(mandatory)*

Requirements use `REQ-NNN` identifiers and EARS form. The upstream template's `FR-NNN` / `MUST`
examples do not apply to this project.

### Functional Requirements

- REQ-001: When a project contains a Kiro specification tree, the validator shall discover every feature directory in that tree that holds a requirements document.
- REQ-002: When the validator discovers Kiro specifications, the validator shall report how many specification directories it examined.
- REQ-003: Where a project contains no Kiro specification tree, the validator shall leave Spec Kit specification discovery unchanged.
- REQ-004: If a Kiro feature directory contains no requirements document, then the validator shall report that specification as having no requirements and continue the run.
- REQ-005: The validator shall parse every discovered Kiro requirements document into individual acceptance criteria.
- REQ-006: The validator shall qualify each parsed criterion identifier with the feature that declares it.
- REQ-007: Where a requirements document contains no requirement headings, the validator shall treat that document as containing no criteria.
- REQ-008: If a baseline requirement heading produces no acceptance criteria, then the validator shall report a missing-criteria finding.
- REQ-009: When requirement text appears inside a fenced code block, the validator shall exclude that text from criterion discovery.
- REQ-010: The validator shall compute an active baseline from the discovered Kiro specifications.
- REQ-011: When a specification's recorded phase is superseded, reserved, or initialized, the validator shall exclude that specification from the active baseline.
- REQ-012: When the active baseline is computed, the validator shall report the number of included specifications, the number of excluded specifications, and the reason for each exclusion.
- REQ-013: Where a specification records no phase, the validator shall include that specification in the active baseline.
- REQ-014: When a specification is excluded as superseded, the validator shall collect successor names as discovered feature names that appear in that specification's phase notes.
- REQ-015: If a specification in the active baseline claims to supersede a discovered specification, then the validator shall record a supersession linking the claiming specification to the named specification.
- REQ-016: If a superseded specification has no supersession linking it to a specification in the active baseline, then the validator shall report a dangling-supersession finding.
- REQ-017: If a specification in the active baseline claims to supersede a discovered specification that is not excluded as superseded, then the validator shall report an unreciprocated-supersession finding.
- REQ-018: Where a superseded specification links to more than one specification in the active baseline, the validator shall accept the recut.
- REQ-019: The validator shall treat an amendment that extends a baseline requirement as a refinement of that requirement.
- REQ-020: The validator shall assign amendment criteria to the same specification as the baseline they extend.
- REQ-021: While assessing a project, the validator shall leave every file in that project unmodified.
- REQ-022: Where the assessed project contains no Spec Kit configuration, the validator shall still discover and assess its Kiro specifications.
- REQ-023: When assessing Kiro specifications, the validator shall omit Spec Kit identifier, verification-mapping, task-coverage, and separation checks on those specifications.
- REQ-024: When constraint analysis is not performed, the validator shall name that omission in the result.
- REQ-025: If a specification's phase metadata cannot be read, then the validator shall report a metadata finding.
- REQ-026: If a specification's phase metadata cannot be read, then the validator shall exclude that specification from the active baseline.

### Key Entities

- **Kiro specification tree**: A project's collection of feature directories, each holding a
  requirements document and optional phase metadata. The tree is discovered; it is not converted
  into Spec Kit files.
- **Feature directory**: One named specification. Its name is the identifier used in supersession
  links.
- **Phase metadata**: The recorded lifecycle of a specification (including superseded, reserved,
  and initialized). When present, it is the authority for baseline membership. When absent, the
  specification is in force.
- **Active baseline**: The set of discovered specifications that are in force. Superseded,
  reserved, and initialized specifications are out of it. Unreadable phase metadata is out of it
  until it can be read.
- **Supersession**: A directed link from a live successor to a superseded predecessor. Links are
  collected from phase notes that mention a discovered feature name, and from spec-level claims in
  a live specification's requirements prose outside acceptance criteria. The word "supersede"
  inside acceptance criteria that describe product behaviour is not a spec-level claim.
- **Recut**: One superseded predecessor with more than one live successor. Valid.
- **Amendment**: Additional acceptance criteria recorded after a baseline was approved, extending
  a named requirement in the same specification. Refinement, not replacement.
- **Dangling supersession**: A superseded specification with no link to any specification in the
  active baseline.
- **Unreciprocated supersession**: A live specification claiming to replace another specification
  that is still in the active baseline.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- SC-001: Pointing the validator at a Kiro project reports every feature directory that holds a
  requirements document, and leaves every file in that project unchanged.
- SC-002: A superseded specification is absent from the active baseline, and is reported as
  dangling if it has no live successor.
- SC-003: A live specification that claims to replace a specification still in the active baseline
  produces an unreciprocated-supersession finding naming both.
- SC-004: Every assessment that does not analyse contradictions states that fact in the result.
- SC-005: A Spec Kit project with no Kiro tree produces the same discovery result this feature
  being present as it did before.

## Assumptions

- The assessed Kiro project is not modified. No configuration, vocabulary, model, or ledger is
  written into it.
- Contradiction finding stays the existing constraint-model merge. This feature only makes the
  active baseline available for that merge later. A Kiro tree without models does not get a
  prose-level contradiction search.
- EARS-form linting of Kiro acceptance criteria is out of scope. So are steering files, task
  coverage of Kiro `tasks.md`, and phase-drift (a delivered feature still marked tasks-generated).
- Successor names are the feature directory names already in the tree. The checker does not guess
  identity from titles or prose that does not contain those names.
- Spec-level supersession claims live in phase notes or in requirements prose outside acceptance
  criteria (introduction, relationship, appendix). Operational uses of "supersede" inside
  acceptance criteria are product behaviour.
- A native Kiro specification with no phase metadata is in force.
- Amendments belong to their parent specification. If the parent is superseded, they leave the
  baseline with it.
- Spec Kit projects remain the default layout. Kiro discovery is additional, triggered by the
  presence of a Kiro specification tree.
- Selective-adoption disclosure (a disabled or omitted check is named) applies: omitting
  constraint analysis is a stated gap, not a pass.
