# Feature Specification: Selective Adoption

**Feature**: `007-selective-adoption`
**Status**: Implemented

## Summary

A project chooses which classes of analysis run, so it can adopt one layer without adopting all of
them. Disabling is never silent: a run states what it did not check, in both its human-readable and
its machine-readable result.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - A brownfield project gates what it can today (Priority: P1)

A codebase with hundreds of requirements and no test mapping wants requirement form checked now and
traceability wired later. Today the choice is a gate that fails on every requirement or no gate,
and in practice that means no gate.

**Acceptance**: With traceability disabled, a run reports requirement-form findings and no
traceability findings, and states that traceability was not checked.

### User Story 2 - A disabled check cannot be mistaken for a passing one (Priority: P1)

Six months later nobody remembers which layers are on. Someone reads a green result and concludes
the requirements are fully checked.

**Acceptance**: Every run naming a disabled check does so whether it passes or fails, and the
machine-readable result carries the same list.

### User Story 3 - A mistyped switch is refused (Priority: P2)

An author writes `tracability = false` and believes the layer is off. It is on, and nothing said so.

**Acceptance**: An unrecognised key in the check configuration is reported rather than ignored.

### Edge Cases

- A project that declares no check configuration runs every check, so an existing project is
  unaffected by the feature existing.
- Disabling the constraint check omits constraint analysis for every specification. A partially
  merged constraint system would report contradictions among the specifications that opted in and
  say nothing about the rest, and silence reads as agreement.
- Disabling a check does not suppress findings from a different check that happens to share a code
  prefix.

## Requirements *(mandatory)*

### Functional Requirements

- REQ-001: The validator shall read a project configuration declaring which checks run.
- REQ-002: Where a project declares no check configuration, the validator shall run the requirement, verification, task and separation checks.
- REQ-003: When a check is disabled, the validator shall omit that check's findings from the result.
- REQ-004: When a check is disabled, the validator shall name every disabled check in its human-readable result.
- REQ-005: When a check is disabled, the validator shall record every disabled check in its machine-readable result.
- REQ-006: The validator shall name disabled checks whether the run passes or fails.
- REQ-007: If a check configuration names an unrecognised check, then the validator shall report a configuration finding.
- REQ-008: Where the constraint check is disabled, the validator shall omit constraint analysis for every specification in scope.
- REQ-009: The validator shall report disabled checks in a stable order irrespective of the order in which they were declared.
- REQ-010: Where a check is disabled, the validator shall leave every other check unaffected.
- REQ-011: The validator shall leave the vocabulary and constraint checks disabled unless a project enables them.
- REQ-012: If the constraint check is enabled while the vocabulary check is disabled, then the validator shall report a configuration finding.
- REQ-013: Where a second-phase check is enabled and its inputs are absent, the installation report shall state that the check can report nothing.
- REQ-014: The installation report shall state which adoption phase a project is in.

### Key Entities

- **Check**: one class of analysis running inside a gate, which a project may switch off.
- **Provenance**: the record of what a run read and what it did not.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- SC-001: A project can enable any subset of the five optional checks, including none and all.
- SC-002: 100% of runs with a disabled check state that fact in both output forms.
- SC-003: A project that enables a layer is told within one run whether that layer can operate.
- SC-004: A project adopting the first phase needs no file it does not already have.

## Assumptions

- Adoption has two phases. Requirement form, verification mapping, task coverage and separation
  check artifacts a Spec Kit project already has, so they run by default. The vocabulary and
  constraint layers need files that do not exist yet, so they are enabled deliberately once there is
  something for them to read.
- Requirement-form checking is not optional. It is the floor the other layers build on, and a
  project that does not want it does not want this tool.
- The constraint layer depends on the vocabulary layer rather than merely pairing well with it. A
  guard is written over declared terms and needs their domains to be type-checked, so the two are
  enabled together or not at all.
- Partial adoption is in tension with the project's central claim that a passing result means
  everything was checked. That tension is resolved by disclosure rather than by refusing the
  feature: a narrowed claim is acceptable, a narrowed claim that looks broad is not.
