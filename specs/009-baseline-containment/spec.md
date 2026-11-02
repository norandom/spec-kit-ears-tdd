# Feature Specification: Baseline Containment

**Feature Branch**: `009-baseline-containment`

**Created**: 2026-09-17

**Status**: Draft — design and test-first work breakdown prepared; implementation pending.
Amended 2026-09-22: the agent operating envelope (formerly User Story 6, REQ-037 to REQ-050) moved
to `010-envelope-containment`; the policy and amendment kinds gained their own story, orientation
rules and fixture.

**Input**: The existing gates ask whether a set of requirements can hold together. They cannot ask
whether one set of requirements stays inside another. Those are different questions, and the second
is the one a reviewer faces on every edit: this revision reads fine, and nothing in it contradicts
anything, but does it now permit something the approved version forbade?

A diff cannot answer that. Two requirements can change no shared word and still move the boundary,
because the boundary is a property of the guards taken together rather than of any sentence. The
merge gate cannot answer it either: a widened specification is usually perfectly satisfiable, so
there is no contradiction to find.

This feature adds one query over the models `004-constraint-model` already builds. For each declared
effect, a model determines the set of states in which it asserts that effect. Containment asks
whether the candidate's set is inside the reference's. Where it is not, the answer is a state: the
concrete assignment under which the candidate asserts something the reference did not, named
alongside the requirements that put it there.

The same query serves four references without changing: the previous revision of a specification, a
project-level maximum policy, the union of the live successors of a superseded specification, and
the baseline requirement an amendment extends. What differs between them is not the query but which
direction of difference is a finding, and that is declared per comparison kind rather than guessed.

A fifth reference, an earlier revision of the operating envelope the agent runs under, uses the same
engine and is specified separately in `010-envelope-containment`, because it needs an input format
and an adapter this feature does not.

**Depends on**: `002-vocabulary-grounding` for term identity, `004-constraint-model` for guards,
effects, finitization, and the budget, `003-cross-spec-constraint-merge` for the union of models
over a set of specifications, and `008-kiro-assessment` for the active baseline and the supersession
links this feature checks the content of.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - An edit that widens the boundary is named as one (Priority: P1)

An author revises a specification. Every requirement still parses, nothing contradicts, and the gate
passes. One guard lost a conjunct, so a state that used to fall outside the specification now falls
inside it, and the specification now obliges a behaviour there. Nobody reading the diff sees this,
because the sentence that changed reads like a clarification.

Comparing the revision against the approved one reports the state, the effect, and the requirements
that assert it there.

**Why this priority**: This is the feature. Every other story is the same query pointed at a
different reference.

**Independent Test**: Take a model, weaken one guard so that it holds in a state where it previously
did not, and assert a widening finding naming the effect, the witness state, and the requirement.

**Acceptance Scenarios**:

1. **Given** a candidate that asserts an effect in a state where the reference does not, **When** the
   containment check runs, **Then** a widening finding names that effect and a witness state.
2. **Given** a widening finding, **When** it is reported, **Then** it names the candidate
   requirements that assert the effect in the witness state.
3. **Given** a candidate and a reference that assert every effect over identical states, **When** the
   containment check runs, **Then** no widening and no narrowing finding is produced.
4. **Given** an unchanged pair of models, **When** the containment check runs twice, **Then** both
   runs report the same witness.

---

### User Story 2 - A dropped obligation is not silence (Priority: P1)

The opposite edit is as easy to miss and reads even better. A guard gains a conjunct, and a
requirement that used to apply in some state no longer applies there. Nothing contradicts; the
specification simply obliges less than the approved one did. Read as a diff it looks like added
precision.

Narrowing is not automatically a defect, because narrowing is what a deliberate retreat looks like.
It is reported because the reviewer is the one who decides which it was.

**Why this priority**: A check that reports only widening teaches people that shrinking a
specification is free, and a specification shrinks far more quietly than it grows.

**Independent Test**: Strengthen one guard so that it fails in a state where it previously held, and
assert a narrowing finding naming the reference requirement that no longer has a counterpart.

**Acceptance Scenarios**:

1. **Given** a reference that asserts an effect in a state where the candidate does not, **When** the
   containment check runs, **Then** a narrowing finding names that effect and a witness state.
2. **Given** a narrowing finding, **When** it is reported, **Then** it names the reference
   requirements that assert the effect in the witness state.
3. **Given** a candidate that declares an effect absent from the reference, **When** the containment
   check runs, **Then** a new-effect finding names that effect.

---

### User Story 3 - A deliberate widening is a declared decision (Priority: P1)

Some widenings are the point of the change. A feature is meant to permit what the previous revision
did not, and somebody decided that. Reported as an error it becomes noise, and a check that produces
noise is switched off.

A widening covered by a derogation naming the reference requirement and carrying a reason is
reported as declared rather than as an error, and stays visible for exactly as long as it applies. A
derogation that no longer covers any widening is reported, so the list does not rot.

**Why this priority**: Without it the check is unusable on a project that ever intends to change
anything. With it every intentional boundary move becomes a reviewable artifact with a name on it.

**Independent Test**: Produce a widening, declare a derogation naming the reference requirement, and
assert the finding becomes advisory; remove the widening and assert the derogation is reported as
stale.

**Acceptance Scenarios**:

1. **Given** a widening covered by a declared derogation, **When** the containment check runs,
   **Then** that widening is reported as declared.
2. **Given** a derogation naming a requirement the reference does not declare, **When** the
   containment check runs, **Then** an unknown-derogation finding names it.
3. **Given** a derogation that covers no reported widening, **When** the containment check runs,
   **Then** a stale-derogation finding names it.

---

### User Story 4 - A supersession is checked for content, not only for a name (Priority: P1)

`008-kiro-assessment` checks that a superseded specification has a live successor. That is a link
check: it establishes that somebody took over, not that anything was taken over. A recut that drops
an obligation on the way past passes it, and the obligation is gone with a green result over it.

Where the predecessor and its successors carry models, the predecessor is compared against the union
of its live successors. A state and an effect the predecessor asserted and no successor asserts is
the gap, reported with both.

**Why this priority**: This is the difference between a retirement that was completed and one that
was announced, and it is the question the supersession machinery was built to make askable.

**Independent Test**: Build a predecessor recut into two successors that between them assert every
effect the predecessor did, and assert no gap; remove one successor requirement and assert a
supersession-gap finding naming the effect and the state.

**Acceptance Scenarios**:

1. **Given** a superseded specification whose live successors together assert every effect it
   asserts in every state, **When** the containment check runs, **Then** no supersession-gap finding
   is produced.
2. **Given** a superseded specification asserting an effect in a state no live successor asserts it,
   **When** the containment check runs, **Then** a supersession-gap finding names the effect, the
   state, and the predecessor requirement.
3. **Given** a superseded specification carrying no constraint model, **When** the containment check
   runs, **Then** that supersession is reported as uncompared.

---

### User Story 5 - A comparison that could not be made is not a pass (Priority: P1)

Two models are comparable only over a vocabulary that means the same thing on both sides and a
finitization fine enough to distinguish every state either side can distinguish. Either can fail. A
term can be renamed between revisions, and an integer term can be compared against different
constants on each side, so that regions which look alike are not.

Each failure has the same consequence: the query was not answered. None of them may be reported as
containment.

**Why this priority**: A containment check is worth having only because people will rely on a green
result to approve a change they did not read closely. Every unearned green here is spent directly on
a decision.

**Independent Test**: Compare two models whose integer guards name different constants, and assert
the partition used is the union of both; rename a term on one side only and assert the comparison is
withheld rather than answered.

**Acceptance Scenarios**:

1. **Given** two models whose guards compare an integer term against different constants, **When**
   the comparison is encoded, **Then** that term's domain is partitioned at the constants of both
   models.
2. **Given** a term referenced by one model that resolves to no term on the other side, **When** the
   containment check runs, **Then** a vocabulary-drift finding is produced and the affected effect
   is reported as uncompared.
3. **Given** a containment question larger than the configured budget, **When** the containment check
   runs, **Then** the comparison is reported as indeterminate and no state of that question is
   evaluated.
4. **Given** a requirement carrying no model entry on either side, **When** the containment check
   runs, **Then** that requirement is reported as not compared.

### User Story 6 - A policy is an upper bound and an amendment is a refinement (Priority: P1)

Not every reference is an earlier version of the same thing. A project can keep one model that says
what any specification is ever allowed to assert: writes only inside a maintenance window, deletion
only with an approval. That is a maximum, not a baseline. A candidate that asserts less than the
maximum is exactly what the maximum is for, and reporting that as narrowing would produce a warning
on every feature in the project.

An amendment is the mirror image. It extends one named baseline requirement, so it is expected to
assert its effect in a subset of the states the baseline asserts it in. Narrowing is the point;
widening is an amendment that quietly rewrote what it claimed to refine.

The engine does not change for either. What changes is which direction of difference is reported,
and the comparison names its kind so that a reader of the report knows which question was asked.

**Why this priority**: The policy reference is the reason a containment check exists at the project
level rather than only per specification. Without an orientation rule per kind, the policy
comparison is unusable and the amendment comparison has no pass criterion.

**Independent Test**: Declare a policy comparison whose candidate is narrower on one effect and
wider on another, and assert one widening finding and no narrowing finding; declare an amendment
comparison whose amendment is narrower than the extended requirement, and assert no finding.

**Acceptance Scenarios**:

1. **Given** a policy comparison whose candidate asserts an effect in a strict subset of the states
   the policy asserts it in, **When** the containment check runs, **Then** no narrowing finding is
   produced for that effect.
2. **Given** a policy comparison whose candidate asserts an effect in a state the policy does not,
   **When** the containment check runs, **Then** a widening finding names that effect, the witness
   state, and the candidate requirement.
3. **Given** a policy comparison whose candidate asserts an effect that conflicts with an effect the
   policy asserts in the same state, **When** the containment check runs, **Then** a
   baseline-conflict finding names both effects and the state.
4. **Given** a revision comparison, **When** the containment check runs, **Then** no
   conflicting-effect question is asked, because a revision replaces its reference rather than
   answering to it.
5. **Given** an amendment comparison whose amendment asserts its effect in a subset of the states
   the extended requirement asserts it in, **When** the containment check runs, **Then** no finding
   is produced.
6. **Given** an amendment comparison whose amendment asserts its effect in a state the extended
   requirement does not, **When** the containment check runs, **Then** a widening finding is
   produced.
7. **Given** any completed comparison, **When** the result is produced, **Then** it names the
   comparison kind and the directions that kind reports.

### Edge Cases

- The reference names a revision that does not exist, or the project has no approved revision yet.
- The candidate and the reference are the same model.
- An effect is declared by both sides and asserted by neither.
- A requirement carries no guard, so it asserts its effect in every state, and the other side guards
  the same effect.
- A term is deprecated on one side with a replacement declared on both.
- A term is deprecated on one side and the replacement is absent from the other.
- An integer term's declared bounds differ between the two revisions.
- An enumeration gains a member between revisions, so the candidate has states the reference cannot
  express.
- An enumeration loses a member, so the reference has states the candidate cannot express.
- A superseded specification has exactly one successor, and that successor is itself superseded.
- Every live successor of a superseded specification is unmodelled.
- A derogation names a requirement that exists in the reference and produces no widening.
- Two derogations cover the same widening.
- Every guard asserting one effect is unsatisfiable, so that effect's activation region is empty.
- A model asserts conflicting effects in one state; that defect does not erase its activation regions
  or turn the comparison into a vacuous pass.
- The reference is empty, so every effect the candidate asserts is a widening.
- A policy declares an effect the candidate never declares; under the policy kind that is not a
  finding, because a maximum is allowed to permit what nobody uses.
- A candidate declares an effect the policy does not; every state in which the candidate asserts it
  is a widening, and no derogation can cover it because there is no same-effect policy requirement.
- A policy model lives inside the specification glob, so the merge gate would also read it as a
  feature and report its wide guards as cross-specification conflicts.
- An amendment extends a requirement that carries no model entry, so the amendment comparison is
  uncompared rather than passed.
- A comparison manifest names a reference path that exists in version control history but not in
  the working tree.

## Requirements *(mandatory)*

Requirements use `REQ-NNN` identifiers and EARS form.

### Functional Requirements

- REQ-001: When a containment check is requested, the validator shall resolve one reference constraint model and one candidate constraint model.
- REQ-002: If a requested reference cannot be resolved, then the validator shall report a missing-reference finding.
- REQ-003: If a requested reference cannot be resolved, then the validator shall withhold the containment verdict for that comparison.
- REQ-004: When a containment check completes, the validator shall name the reference it compared against and the source of that reference.
- REQ-005: When encoding a comparison, the validator shall partition each integer term's domain at the union of the literal constants named by both models.
- REQ-006: The validator shall encode the reference model and the candidate model under one shared variable ordering.
- REQ-007: The validator shall confirm that two states distinguished by either model remain distinct in the shared encoding.
- REQ-008: The validator shall compute, for each declared effect, the set of states in which a model asserts that effect.
- REQ-009: Where a modelled requirement declares no guard, the validator shall treat that requirement as asserting its effect in every state.
- REQ-010: If the candidate asserts an effect in a state where the reference does not assert it, then the validator shall report a widening finding.
- REQ-011: If the reference asserts an effect in a state where the candidate does not assert it, then the validator shall report a narrowing finding.
- REQ-012: When a widening or a narrowing finding is reported, the validator shall accompany it with a witness state over the terms the compared effect depends on.
- REQ-013: When a widening finding is reported, the validator shall name the candidate requirements that assert the effect in the witness state.
- REQ-014: When a narrowing finding is reported, the validator shall name the reference requirements that assert the effect in the witness state.
- REQ-015: The validator shall produce the same witness for one pair of models on every run.
- REQ-016: Before reporting a witness, the validator shall confirm that the witness satisfies the guards the finding attributes to it.
- REQ-017: Where the candidate declares an effect the reference does not declare, the validator shall report a new-effect finding.
- REQ-018: Where a comparison declares the policy kind, the validator shall report a baseline-conflict finding for each conflicting-effect pair the candidate and the policy assert in one state.
- REQ-019: Where a widening is covered by a declared derogation naming a reference requirement, the validator shall report that widening as declared.
- REQ-020: If a derogation names a requirement the reference model does not declare, then the validator shall report an unknown-derogation finding.
- REQ-021: If a declared derogation covers no reported widening, then the validator shall report a stale-derogation finding.
- REQ-022: The validator shall require a reason of at least ten characters on every declared derogation.
- REQ-023: Before answering a containment question, the validator shall compute the size of the space that question ranges over.
- REQ-024: If a containment question exceeds the configured budget, then the validator shall report an over-budget finding without evaluating any state of that question.
- REQ-025: If a containment question exceeds the configured budget, then the validator shall report that comparison as indeterminate rather than as contained.
- REQ-026: If a term referenced by one model resolves to no term on the other side, then the validator shall report a vocabulary-drift finding.
- REQ-027: If a vocabulary drift affects an effect, then the validator shall report that effect as uncompared.
- REQ-028: Where a term is declared deprecated and its replacement is declared on both sides, the validator shall resolve that term to its replacement before comparing.
- REQ-029: Where a requirement on either side carries no model entry, the validator shall report that requirement as not compared.
- REQ-030: When a containment check completes, the validator shall report the number of effects compared and the number of requirements that did not participate.
- REQ-031: When a superseded specification and its live successors carry constraint models, the validator shall compare that specification against the union of those successors.
- REQ-032: If a superseded specification asserts an effect in a state no live successor asserts it, then the validator shall report a supersession-gap finding.
- REQ-033: Where a superseded specification or every one of its live successors carries no constraint model, the validator shall report that supersession as uncompared.
- REQ-034: When an amendment extends a baseline requirement, the validator shall compare the amended requirement against the baseline requirement it extends.
- REQ-035: Where an assessed project is read without being adopted, the validator shall read the reference model from the assessing project.
- REQ-036: While a containment check is running, the validator shall leave every file in the compared projects unmodified.
- REQ-037: Where a comparison declares the policy kind, the validator shall report no narrowing finding for that comparison.
- REQ-038: Where a comparison declares a kind other than policy, the validator shall ask no conflicting-effect question for that comparison.
- REQ-039: Where a comparison declares the amendment kind, the validator shall report no narrowing finding for that comparison.
- REQ-040: When a comparison completes, the validator shall record the comparison kind and the directions that kind reports in the result.
- REQ-041: Where a comparison manifest names a reference model, the validator shall read that model from the named path without resolving any revision from version control.

### Comparison semantics

These definitions resolve the direction and scope of the requirements above. Concrete file formats
and report fields are specified in [the contracts](contracts/containment.md).

- For effect `e`, each source's activation region is the union of the guards of its requirements
  asserting `e`, restricted to that source's declared domains. A side with several successor sources
  is the union of those already-restricted regions. Conflicting effects do not cancel each
  other. Existing model defects remain findings; a malformed or unresolved model is not an empty
  model. A declared effect with no asserting requirements has an empty region.
- Widening is `candidate(e) \\ reference(e)`; narrowing is `reference(e) \\ candidate(e)`.
  A new-effect finding records a new declaration independently of whether its region is empty.
  A baseline conflict uses the symmetric union of both sides' declared conflict relations, and is
  asked only under the policy kind: a revision, a successor union or an amendment replaces or
  refines its reference, so opposing effects across those sides are already visible as one
  widening plus one narrowing and carry no further meaning.
- Each effect's comparison uses the union of the terms mentioned by its guards on both sides.
  Each conflicting-effect pair uses the union of the terms of that pair. Unrelated vocabulary terms
  do not enlarge a question. Both models use the same term and value order for that question.
- Enumeration additions and removals are comparable when the term's identity and kind are unchanged.
  Shared integer partitions include domain boundaries as well as guard constants. A state outside
  one side's domain cannot activate that side's effects, including through an unconditional guard.
  A missing term or incompatible domain kind withholds the affected comparison. Labels alone are
  not term identity, and a rename without a declared replacement is not inferred.
- A derogation belongs to a candidate requirement and names a reference requirement asserting the
  same effect. It covers only the widening contributed by that candidate requirement. A declared
  widening requires coverage of every contributing candidate requirement over the whole difference,
  not merely at the displayed witness. Baseline conflicts are not waived. A derogation is not judged
  stale when its comparison was withheld or indeterminate.
- For supersession, the predecessor is the reference and the union of its live successors is the
  candidate. A gap is narrowing in this orientation. Successor chains terminate at the active
  baseline; cycles and unresolved links prevent a complete claim. Missing successor models remain
  visible. An uncovered region is a proven gap only when every relevant successor was compared.
  A Spec Kit amendment comparison is limited to its explicitly named requirement pair. A Kiro
  amendment comparison is limited to one amendment heading's criteria and the baseline headings
  named by that amendment's parsed extension links; unrelated amendments do not cover its gaps.
- Each kind fixes which direction is reported. The engine computes both differences for every
  kind; the kind decides which of them become findings and which are recorded as counts only.

  | Kind | Reference | Candidate | Widening | Narrowing | Conflict question |
  |---|---|---|---|---|---|
  | `revision` | earlier snapshot of the same specification | current model | error (or declared) | warning | not asked |
  | `policy` | project maximum policy | current model | error (or declared) | not a finding | asked, error |
  | `supersession` | superseded predecessor | union of live successors | not a finding | `CONTAIN_SUPERSESSION_GAP`, error | not asked |
  | `amendment` | the extended baseline requirement | the amendment | error (or declared) | not a finding | not asked |

  Under `supersession`, widening is the successors doing more than the predecessor, which is what a
  successor is for. Under `policy` and `amendment`, narrowing is the candidate staying inside its
  bound, which is what the bound is for. The suppressed direction is still computed and its state
  count is reported, so a reader can see that it was asked and not merely skipped.
- A policy is an ordinary model file with an ordinary specification document beside it, kept at an
  explicit path outside the project's specification glob. It is named only by the manifest. Kept
  inside the glob it would be discovered as a feature, gated as one, and merged with the others,
  where its deliberately wide guards would read as cross-specification conflicts.
- A reference snapshot is a pair of files, the model and the specification document it was read
  against, at paths the manifest names. Producing that pair is a step the project performs when it
  approves a revision, and it is documented in the quickstart; the validator reads the pair and
  never fetches, checks out, or otherwise resolves a revision itself.
- A comparison records its result separately from the process exit status. No missing, invalid,
  drifted, over-budget, or partially modelled input earns an unqualified containment claim. Narrowing
  remains a reported change for review; it is not automatically classified as a defect.

### Key Entities

- **Reference model**: The constraint model a candidate is measured against. Its source is one of an
  earlier revision of the same specification, a project-level maximum policy, a superseded
  predecessor specification, or the baseline requirement an amendment extends. The query does not
  change with the source, and the source is named in the result because the verdict means nothing
  without it.
- **Candidate model**: The constraint model under review, or the union of live successor models in
  a supersession comparison.
- **Activation region**: For one model and one effect, the set of states in which that model asserts
  that effect. Containment is a relation between activation regions, computed for each effect
  separately, which is what keeps each question small.
- **Widening**: A state in an activation region of the candidate and outside the corresponding region
  of the reference. The candidate asserts something the reference did not.
- **Narrowing**: The same in the other direction. The candidate no longer asserts something the
  reference did.
- **Derogation**: A declared, reasoned exception naming a reference requirement, which converts a
  widening from an error into a recorded decision. It is a record of a choice, not a proof that the
  choice was right.
- **Supersession gap**: An effect and a state the superseded predecessor asserts and no live
  successor asserts. Distinct from a dangling supersession, which is the absence of a successor at
  all.
- **Shared encoding**: The finitization under which both models are read. Built from the union of
  both models' declared domains and both models' literal constants, because two partitions built
  separately are not comparable.
- **Comparison kind**: One of revision, policy, supersession, or amendment. It selects nothing about
  how the query is evaluated and everything about which answer is a finding. It is recorded in the
  result because a widening under one kind is a pass under another.
- **Policy model**: A constraint model that states the most any specification in the project is
  permitted to assert, kept outside the specification glob and named explicitly by the manifest.
  Its requirements are the targets a derogation names when a feature deliberately exceeds it.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- SC-001: An edit that permits a state the approved revision forbade fails the gate, and the report
  names that state without the reader rerunning the tool.
- SC-002: An edit that drops an obligation is reported, and is distinguishable in the report from an
  edit that adds one.
- SC-003: Every widening that survives review is represented by a declared derogation carrying a
  reason and a named reference requirement.
- SC-004: A recut that loses an obligation fails, where a recut that redistributes the same
  obligations passes.
- SC-005: Repeated runs over an unchanged pair of models produce identical findings and identical
  witnesses.
- SC-006: No comparison that was withheld, drifted, or exceeded its budget is reported as contained.
- SC-007: A project adopts containment for one specification without any other specification
  changing.
- SC-008: A candidate that obliges less than the project policy passes the policy comparison with no
  finding; one that permits a state the policy excludes fails it, and the report names that state.
- SC-009: Every completed comparison record names its kind, and a reader can tell from the record
  alone which direction of difference would have failed it.

## Assumptions

- Containment is computed over constraint models, never over prose. A project whose specifications
  carry no models gets the uncompared report of REQ-029 and REQ-033 rather than a guess, which is
  the same refusal `008-kiro-assessment` makes about contradictions in Kiro trees.
- The fragment stays the one `004-constraint-model` declares: booleans, small enumerations, and
  integers compared against literal constants. Containment adds a query over that fragment and does
  not extend it. A requirement for genuine arithmetic remains the point at which the decision
  procedure is revisited, and is no nearer because of this feature.
- The decision procedure is an implementation choice under the same obligation as every other
  question in this project: two procedures agree, the witness is canonical, and the bound is a state
  count rather than elapsed time.
- Each effect is compared independently, so a containment question ranges over the terms one effect
  depends on rather than over the whole model. The budget is applied per question on that basis.
- Reference selection is a command line concern and is out of scope here. What is in scope is that
  whatever was selected is named in the result, because a containment verdict without its reference
  is not a claim.
- An assessed project is not written to. A reference model for a Kiro tree lives in the assessing
  project, which is what makes containment available to a tree that cannot carry models of its own.
- Derogations follow the separation-exemption precedent: a marker, a named target, a reason of
  substance, and a stale entry reported rather than ignored.
- Selective-adoption disclosure applies. Containment being off, or a comparison being withheld, is a
  stated gap and never folded into a pass.
- Narrowing is reported rather than ranked. Whether a dropped obligation is a retreat or a correction
  is a judgement this tool does not make.
- The policy kind is the project-level form of the same query and needs no second engine. What it
  needs is an orientation rule, which is why the kind is declared rather than inferred from the
  paths involved.
- A reference snapshot is produced by the project, not by the validator. Copying the approved
  model and specification to the reference paths is a two-file operation performed at approval
  time and committed with the approval. A command that performs that copy is a candidate follow-up
  and is not part of this feature, because it writes and every command here except `init` reads.
- Comparing a requirements model against an agent operating envelope, and comparing two envelopes,
  are `010-envelope-containment`. That feature reuses this engine and adds the input format, the
  adapter, and an envelope-specific vocabulary; nothing in it changes a rule stated here.
