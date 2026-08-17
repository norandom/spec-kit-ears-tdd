# Feature Specification: Task Coverage Gate

**Feature Branch**: `005-task-coverage-gate`

**Created**: 2026-08-17

**Status**: Draft

**Input**: `--phase tasks` is the gate that stands between an approved plan and production-code
changes, and today it never opens `tasks.md`. It is byte-identical to `--phase plan`, so the one
gate whose job is to confirm the work was decomposed test-first confirms nothing about the work at
all. This feature gives it something to check.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - A requirement cannot reach implementation untasked (Priority: P1)

An author writes a plan, generates tasks, and starts implementing. A requirement that no task ever
mentions is a requirement nobody planned to build, and the current gate waves it through. The tasks
gate reports it before any production code is written.

**Why this priority**: This is the entire purpose of the phase. Without it there are three gates
that check specifications and traceability and none that check the decomposition.

**Independent Test**: Write a specification with three requirements and a task list mentioning two;
assert the third is reported.

**Acceptance Scenarios**:

1. **Given** a requirement no task references, **When** the tasks gate runs, **Then** an
   uncovered-requirement finding names it.
2. **Given** a feature with no task list at all, **When** the tasks gate runs, **Then** a
   missing-task-list finding is reported rather than a silent pass.
3. **Given** a task referencing an identifier the specification does not declare, **When** the tasks
   gate runs, **Then** an unknown-reference finding is reported.

---

### User Story 2 - Ranges count as coverage (Priority: P1)

Task lists written by hand reference requirements in spans — `REQ-005–REQ-010` — because listing six
identifiers inline is unreadable. A gate that only matches literal identifiers would report four
false failures for that one span, and a gate that cries wolf gets switched off.

**Why this priority**: This is not a refinement of User Story 1, it is a precondition for it. The
reference project's task lists use spans throughout; without expansion the feature is unusable there
on day one.

**Independent Test**: Write a task list referencing `REQ-002–REQ-005` and assert all four are
treated as covered.

**Acceptance Scenarios**:

1. **Given** a task referencing a span of identifiers, **When** the tasks gate runs, **Then** every
   identifier within the span counts as covered.
2. **Given** a span written with a dash, an ellipsis, or the words `through` or `to`, **When** the
   tasks gate runs, **Then** every form is recognized. Both dash and word forms occur in the
   reference project's existing task lists, and recognizing only one of them produced seven false
   failures against work that was in fact fully decomposed.
3. **Given** a span whose endpoints are in descending order, **When** the tasks gate runs, **Then**
   it is reported rather than silently expanding to nothing.

### Edge Cases

- A span crosses a gap in numbering, naming identifiers the specification never declares.
- An identifier appears only inside a fenced code block in the task list.
- The task list references a requirement belonging to a different feature.
- A span endpoint uses a different digit width from its partner, such as `REQ-005–REQ-0010`.
- The task list exists but contains no task at all.

## Requirements *(mandatory)*

### Functional Requirements

- REQ-001: Where the tasks gate is requested, the validator shall evaluate the task list beside each specification.
- REQ-002: If a specification has no task list, then the validator shall report a missing-task-list finding.
- REQ-003: When a task list references a span of requirement identifiers, the validator shall expand that span to every identifier it covers.
- REQ-004: The validator shall recognize a span joined by a hyphen, an en-dash, an em-dash, an ellipsis, the word `through`, or the word `to`.
- REQ-005: If a declared requirement is referenced by no task, then the validator shall report an uncovered-requirement finding.
- REQ-006: If a task list references an identifier that its specification does not declare, then the validator shall report an unknown-reference finding.
- REQ-007: If a span names its endpoints in descending order, then the validator shall report a malformed-span finding.
- REQ-008: When a requirement identifier appears inside a fenced code block, the validator shall exclude it from task coverage.
- REQ-009: While the tasks gate runs, the validator shall leave every file unmodified.
- REQ-010: The validator shall report the number of declared requirements that the task list covers.

### Key Entities

- **Task list**: The `tasks.md` beside a specification, decomposing its requirements into work.
- **Reference**: A requirement identifier named by a task, either literally or as an endpoint of a
  span.
- **Span**: A contiguous range of requirement identifiers written as two endpoints joined by a dash,
  standing for every identifier between them.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- SC-001: A requirement referenced only through a span is never reported as uncovered.
- SC-002: The reference project's twelve existing task lists pass the gate without edits.
- SC-003: Every uncovered requirement is named individually, so the report is a work list.
- SC-004: The tasks gate fails on at least one condition the plan gate accepts, making the phases
  meaningfully distinct.

## Assumptions

- Task lists are Markdown beside the specification, named by the same convention Spec Kit uses.
- A span is a shorthand for its endpoints and everything between them; it does not assert that every
  intermediate identifier is declared.
- Ordering between test tasks and implementation tasks is out of scope. Determining it from prose is
  a heuristic, and a gate that guesses is worse than one that checks less.
- Task completion state is out of scope; the gate checks that work was planned, not that it is done.
