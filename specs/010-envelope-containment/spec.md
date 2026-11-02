# Feature Specification: Envelope Containment

**Feature Branch**: `010-envelope-containment`

**Created**: 2026-09-22

**Status**: Draft — split out of `009-baseline-containment` on 2026-09-22 (its former User Story 6
and REQ-037 to REQ-050), with an envelope-specific vocabulary and an adapter story added.
Implementation pending; depends on the 009 engine landing first.

**Input**: An agent workflow definition names, per phase, which tools the agent may call and which
guarded transitions leave that phase. It is edited like code, reviewed like configuration, and
verified like neither. A phase gains a tool, or a transition guard loses a conjunct, and the agent
can now act in a situation the approved envelope excluded.

That is the edit `009-baseline-containment` catches in a requirements model, against a different
artifact, and it has no gate at all today. A workflow definition finitizes into the same fragment a
constraint model already uses: the phases are one enumeration, the permitted tools are effects, and
the transition guards are guard expressions. Read that way, an envelope is a candidate and an
earlier envelope is a reference, and the containment engine answers without changing.

Two things about an envelope are not like a specification, and 009 was wrong to treat them as if
they were. First, the terms an envelope's transition guards name (`test-result`, `review-state`)
describe the agent's process, not the product, so they do not belong in the product vocabulary and
the vocabulary gate would flag every one of them as unused. An envelope therefore carries its own
vocabulary. Second, nobody writes the validator's interchange format by hand. The value is in
reading the file a runtime actually executes, so this feature owns the adapter that turns a native
workflow file into the interchange model, and names what it could not carry over.

**Depends on**: `009-baseline-containment` for the containment engine, the comparison manifest, the
kind orientation and every `CONTAIN_*` finding not listed here; `004-constraint-model` for the guard
grammar and finitization; `002-vocabulary-grounding` for the vocabulary file format.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - An envelope edit that widens what the agent may do is named (Priority: P1)

A reviewer sees a workflow definition change in which the `implement` phase gains `Bash`, and a
transition into `implement` loses the conjunct that required a failing test. Read as a diff it is
two lines. Read as an envelope it is a new capability in a phase and a wider door into that phase.

Comparing the revised definition against the approved one reports the tool and the phase for the
first, and the transition, its source phase and a witness for the second.

What this story does not promise: reachability. A tool added to a phase no transition enters is
still reported, because the phase is in the definition and a later edit can connect it. A
transition that makes a dangerous phase newly reachable without changing that phase's tools is not
reported, except where the transition's own guard weakened. Every envelope result says so.

**Why this priority**: This is the feature. The other stories make it usable.

**Independent Test**: Read a workflow definition into a model, permit one tool in one additional
phase, and assert a widening finding naming the tool, the phase, and the identifier derived for that
phase. Weaken one transition guard and assert a widening naming the transition and a witness.

**Acceptance Scenarios**:

1. **Given** a candidate envelope permitting a tool in a phase where the reference does not, **When**
   the containment check runs, **Then** a widening finding names that tool and that phase.
2. **Given** a candidate envelope permitting a tool the reference permits in no phase, **When** the
   containment check runs, **Then** a new-effect finding names that tool.
3. **Given** a candidate transition whose guard lost a conjunct, **When** the containment check runs,
   **Then** a widening finding names the transition, its source phase, and a witness satisfying the
   candidate guard but not the reference guard.
4. **Given** a candidate envelope that removes a tool from a phase, **When** the containment check
   runs, **Then** a narrowing finding names that tool and that phase as a warning.
5. **Given** a workflow definition declaring a construct the reader does not model, **When** the
   comparison completes, **Then** the result names that construct.
6. **Given** any completed envelope comparison, **When** the result is produced, **Then** it states
   that reachability was not computed.

---

### User Story 2 - Envelope terms live in their own vocabulary (Priority: P1)

A transition guard says `test-result == 'fail'`. That term has a domain, and the validator refuses
to guess it, so it has to be declared somewhere. The product vocabulary is the wrong place: no
requirement tags `test-result`, so the vocabulary gate reports it unused on every run, and the
product's concept scheme fills with agent-process terms that mean nothing to the product.

An envelope source names its own vocabulary file. Guard terms resolve against that file alone. It
is a vocabulary in the same format as the product's, checked for the same structural defects, and
exempt from the one check that only makes sense for a vocabulary requirements tag.

**Why this priority**: Without it, every project that adopts envelope checking either pollutes its
vocabulary or switches off the unused-term check, and both are the kind of quiet damage this
project exists to refuse.

**Independent Test**: Declare `test-result` only in the envelope vocabulary, run the ordinary
vocabulary gate, and assert no unused-term finding; omit the envelope vocabulary from the source
and assert the comparison is withheld with an input-invalid finding.

**Acceptance Scenarios**:

1. **Given** an envelope source naming a vocabulary that declares every guard term, **When** the
   containment check runs, **Then** every transition effect is compared.
2. **Given** a transition guard naming a term the envelope vocabulary does not declare, **When** the
   containment check runs, **Then** an undeclared-term finding is produced and the effects that term
   reaches are reported as uncompared.
3. **Given** a term declared only in the envelope vocabulary, **When** the product vocabulary gate
   runs, **Then** no unused-term finding names it.
4. **Given** an envelope source that names no vocabulary, **When** the containment check runs,
   **Then** an input-invalid finding is produced and the comparison is withheld.

---

### User Story 3 - A runtime's native workflow file is read through a named adapter (Priority: P2)

The interchange format in [contracts/envelope.md](contracts/envelope.md) is the validator's own.
Nobody edits it by hand and no runtime writes it, so a comparison over it checks a file the agent
never ran under. The file that matters is the one the runtime executes.

An envelope source names a `format`. The validator reads the definition through the adapter
registered for that format, which produces the interchange model plus the list of native
constructs it did not carry over: approval gates, model routing, fork and join, whatever the
runtime has that a phase-and-tool model does not. The adapter's identity and version go into the
provenance, so a result over a native file says which reading of that file it is a result over.

The first native adapter is intended for a state-machine guardrail runtime of the StateWright kind
(JSON-defined states, per-state tool allowlists, guarded transitions). Its mapping is not written
here, because the mapping has to be written from a real file of that format, and none is checked
in yet. Until one is, this story's adapter requirements are satisfied by the interchange adapter
alone, and the native adapter is a task that is blocked on input rather than a task that is done.

**Why this priority**: Stories 1 and 2 are complete without it. It is P2 because it is the one
story whose evidence this repository does not yet hold, not because it matters less; it is the
reason the feature is worth having.

**Independent Test**: Name a format with no registered adapter and assert an unreadable-definition
finding naming the format; name the interchange format explicitly and assert the provenance records
the adapter identity and version.

**Acceptance Scenarios**:

1. **Given** an envelope source naming a format with a registered adapter, **When** the containment
   check runs, **Then** the definition is read through that adapter.
2. **Given** an envelope source naming a format with no registered adapter, **When** the containment
   check runs, **Then** an unreadable-definition finding names the format and the comparison is
   withheld.
3. **Given** an envelope source naming no format, **When** the containment check runs, **Then** the
   definition is read as the interchange format.
4. **Given** any definition read through an adapter, **When** the result is produced, **Then** the
   provenance records the adapter identity and version.
5. **Given** a native definition containing a construct the adapter does not carry into the
   interchange model, **When** the result is produced, **Then** that construct is named.

### Edge Cases

- A workflow definition declares a phase that no transition enters.
- A phase permits no tools at all.
- A phase exists in the candidate envelope and not in the reference, or the other way round.
- A transition guard compares a term against a value outside that term's declared enumeration.
- A tool is named with different capitalisation on the two sides.
- A construct the reader does not model appears only in a phase that permits no tools.
- The two envelopes are written against different schema versions of the interchange format.
- An envelope declares no transitions at all, so every phase is compared and none is reachable.
- The reference and candidate envelopes name different vocabulary files, and a term is declared
  with different domains in each.
- The envelope vocabulary declares a term named `envelope-phase`, which the reader reserves.
- The reference is read through one adapter and the candidate through another.
- An adapter's version changes between two runs over the same pair of files.
- A native file declares two states that map to the same interchange phase name.

## Requirements *(mandatory)*

Requirements use `REQ-NNN` identifiers and EARS form.

### Functional Requirements

- REQ-001: Where a containment check names a workflow definition, the validator shall read that definition into a constraint model.
- REQ-002: When reading a workflow definition, the validator shall represent that definition's phases as the members of one enumeration term.
- REQ-003: When reading a workflow definition, the validator shall represent each tool a phase permits as an effect asserted under that phase.
- REQ-004: When reading a workflow definition, the validator shall derive a stable requirement identifier for each phase it reads.
- REQ-005: If a transition guard names a term the envelope vocabulary does not declare, then the validator shall report an undeclared-term finding.
- REQ-006: If a transition guard names a term the envelope vocabulary does not declare, then the validator shall report the effects that term reaches as uncompared.
- REQ-007: The validator shall reject a transition guard containing any construct outside the declared guard grammar.
- REQ-008: Where a workflow definition declares a construct the validator does not model, the validator shall report that construct as unmodelled.
- REQ-009: When an envelope comparison completes, the validator shall name every construct of that definition it did not model.
- REQ-010: The validator shall compare workflow definitions phase by phase without computing reachability over their transitions.
- REQ-011: When an envelope comparison completes, the validator shall report that reachability was not computed.
- REQ-012: If a workflow definition cannot be read, then the validator shall report an unreadable-definition finding.
- REQ-013: If a workflow definition cannot be read, then the validator shall withhold the containment verdict for that comparison.
- REQ-014: When reading a workflow definition, the validator shall represent each transition as an effect asserted under its source phase and transition guard.
- REQ-015: Where an envelope source names a vocabulary, the validator shall resolve every transition guard term against that vocabulary alone.
- REQ-016: If an envelope source names no vocabulary, then the validator shall report an input-invalid finding for that comparison.
- REQ-017: While reading an envelope vocabulary, the validator shall apply no unused-term check to that vocabulary.
- REQ-018: Where a comparison declares the envelope kind, the validator shall report a narrowing finding as a warning.
- REQ-019: Where an envelope source names a format, the validator shall read that definition through the adapter registered for that format.
- REQ-020: If an envelope source names a format for which no adapter is registered, then the validator shall report an unreadable-definition finding naming that format.
- REQ-021: Where an envelope source names no format, the validator shall read that definition as the interchange format.
- REQ-022: When an adapter reads a definition, the validator shall record the adapter identity and version in the comparison provenance.
- REQ-023: When an adapter reads a native definition, the validator shall name every native construct it did not carry into the interchange model.

### Comparison semantics

The engine, the shared encoding, the budget, the witness rules and the derogation rules are those
of `009-baseline-containment` and are not restated. What follows is what an envelope adds.

- Tool effects describe permissions in a phase: effect `tool:<name>` under guard
  `envelope-phase == '<phase>'`. Transition effects describe a permitted step from one named phase
  to another: effect `transition:<from>:<to>` under `envelope-phase == '<from>' and <guard>`. They
  do not propagate permissions into the destination. Invalid transition guards withhold that
  transition effect; independently declared tool effects remain comparable. Parallel transitions
  between the same phases contribute a union of guards to the same effect.
- The orientation for the `envelope` kind is that of `revision`: widening is an error unless
  declared, narrowing is a warning, and no conflicting-effect question is asked. An envelope has
  no conflict declarations to ask it over.
- A derogation on an envelope names the derived phase identifier of the reference
  (`PHASE-<name>`) and lives in a sidecar model next to the definition, because the definition
  itself is never written to.
- Comparison of tool permissions within one phase is a set difference and needs no encoding; the
  implementation is free to answer it that way as long as the result is reported through the same
  finding, witness and derogation path as a transition widening, so that a reader cannot tell
  which route produced a finding.
- A comparison is withheld, never passed, when the definition is unreadable, the source names no
  vocabulary, or the named format has no adapter. Unmodelled constructs do not withhold; they mark
  the comparison incomplete while results over the represented permissions stand.

### Key Entities

- **Workflow definition**: An agent operating envelope. Named phases, the tools each phase permits,
  and guarded transitions between them. It is read in place and never written to, and it is not
  installed into anything.
- **Envelope model**: The constraint model derived from a workflow definition. Phases are the
  members of one enumeration term, permitted tools and guarded transitions are effects, and each
  phase carries a derived requirement identifier so that a finding points at a phase rather than
  at a line of a configuration file.
- **Envelope vocabulary**: A vocabulary file in the ordinary format, named by the envelope source,
  declaring the domain of every term a transition guard uses. Separate from the product vocabulary
  and exempt from the unused-term check.
- **Adapter**: A reader for one named format that produces an envelope model and the list of native
  constructs it did not represent. The interchange format has the built-in adapter; every other
  format has one written from a real file of that format. Identity and version are provenance.
- **Unmodelled construct**: A part of a workflow definition the reader or the adapter does not
  represent. It is named in the result rather than skipped, because an envelope comparison that
  quietly ignored half the envelope would read exactly like one that checked it.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- SC-001: An envelope revision that permits a tool in a phase the approved revision did not fails
  the gate, and the report names the phase and the tool.
- SC-002: An envelope revision that weakens a transition guard fails the gate, and the report names
  the transition and a state that satisfies the new guard and not the old one.
- SC-003: Every construct the reader does not model is named in the result of every comparison
  that read it, whether that comparison passed or failed.
- SC-004: A project that adopts envelope checking adds no term to its product vocabulary and
  receives no new unused-term finding.
- SC-005: A result over a native workflow file names the adapter that read it, and two runs over
  the same file with the same adapter version produce identical findings.

## Assumptions

- A workflow definition is an input, not an integration. It is read in place, in whatever project
  holds it, and nothing is written back. The validator does not run an agent, enforce an envelope,
  or depend on the runtime that does; the direction of any relationship with such a runtime is
  that the runtime calls this validator.
- A definition's phase set is closed, so the enumeration of phases is derived from the definition
  itself. Every other term a transition guard names carries a domain the definition does not hold,
  so it is resolved against the envelope vocabulary and refused when absent.
- Reachability, liveness, and deadlock over a definition's transition relation are out of scope.
  Those questions need an image computation over a transition relation that this feature does not
  add, and a partial answer to them would be worse than the stated absence of one. A transition
  contributes its source, destination, and guard to a local permission effect, without a
  reachability claim.
- Comparing a requirements model against an envelope, which is the question of whether a runtime
  guardrail enforces what a requirement obliges, is a different mapping between two artifacts and
  is not this feature.
- The interchange reader models one schema version. A definition written against a version it does
  not know is refused rather than partially interpreted, because a partial interpretation of a
  policy file is the failure this whole project exists to refuse. The same rule binds every adapter.
- The first native adapter needs a real file of its format checked into the fixtures before its
  mapping can be specified. Until then the adapter registry holds the interchange adapter only, and
  the task for the native adapter is recorded as blocked on that input, not as done and not as
  dropped.
