# Feature Specification: Complete Requirement Discovery

**Feature Branch**: `001-complete-requirement-discovery`

**Created**: 2026-08-17

**Status**: Draft

**Input**: A validation gate is only trustworthy if it examined everything it claims to have
examined. Two independent defects currently break that property: the requirement parser recognizes
only two of the Markdown forms authors actually use, and feature scoping silently narrows a
multi-feature project to a single feature. Both fail *open* — the gate reports PASS over content it
never read.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The gate sees every requirement in a specification (Priority: P1)

An author writes requirements in whatever Markdown form reads best: a bullet list, a numbered list,
a section heading per requirement, or a table with one row per requirement. Today only bullet form
is recognized. A specification written entirely as headings reports "no requirements found"; a
specification that mixes forms reports PASS while silently ignoring most of its own content.

**Why this priority**: Every other gate in the system — traceability, separation, and any future
constraint checking — operates on the set of requirements this step produces. A miss here is not a
degraded result, it is a wrong result that looks identical to a correct one.

**Independent Test**: Author one specification containing the same five requirements expressed in
five different Markdown forms, and assert the validator reports five requirements.

**Acceptance Scenarios**:

1. **Given** a specification where requirements appear as headings, **When** the spec gate runs,
   **Then** every requirement is discovered and validated.
2. **Given** a specification containing an illustrative bad requirement inside a fenced code block,
   **When** the spec gate runs, **Then** the fenced example is not treated as a requirement.
3. **Given** a specification where the same identifier appears twice, **When** the spec gate runs,
   **Then** the duplicate is reported.

---

### User Story 2 - The gate sees every feature in the project (Priority: P1)

A project accumulates many feature directories over time. The active-feature pointer written by the
specification workflow names whichever feature is currently being worked on. Every gate silently
inherits that pointer, so a project with eleven features reports on one, and cross-feature problems
are structurally invisible.

**Why this priority**: This is the hard prerequisite for any cross-specification analysis. Merging
constraints across specifications cannot be built on a discovery step that only ever returns one
specification.

**Independent Test**: Create a project with three feature directories and an active-feature pointer
naming one of them; assert the default run reports one feature and the all-features run reports
three.

**Acceptance Scenarios**:

1. **Given** a project with several features and an active-feature pointer, **When** the validator
   runs in all-features mode, **Then** every specification matched by the configured glob is
   evaluated.
2. **Given** an active-feature pointer naming a directory that no longer exists, **When** the
   validator runs, **Then** the run reports a discovery error rather than an empty success.
3. **Given** any run, **When** the result is produced, **Then** the result states how many
   specification files were examined.

---

### User Story 3 - The result is a stable contract (Priority: P2)

Agents, editors, and continuous integration consume the machine-readable result. Today it carries no
version, so any change to the shape of a finding is an undetectable breaking change for every
consumer.

**Why this priority**: The contract has to be pinned before a second implementation exists, not
after. It is also the only mechanism that lets a future implementation prove it agrees with the
current one.

**Independent Test**: Run the validator against a fixture project and compare the machine-readable
result to a stored expected result, field for field.

**Acceptance Scenarios**:

1. **Given** any machine-readable result, **When** it is parsed, **Then** it carries a schema
   version identifier.
2. **Given** a corpus of fixture projects with stored expected results, **When** the validator runs
   against each, **Then** every produced result matches its stored result.

### Edge Cases

- Two features both declare `REQ-001`, and a single production file leaks that identifier. Every
  feature in the reference project restarts numbering at `REQ-001`, so an unqualified identifier
  produces one indistinguishable finding per feature for the same line.
- A requirement identifier appears in a fenced code block used to illustrate a policy violation.
- A requirement identifier appears inside an indented code block rather than a fenced one.
- A specification file is not valid UTF-8, or begins with a byte order mark.
- A feature directory contains a specification but no traceability file.
- A test selector uses a relative path that escapes the project root.

## Requirements *(mandatory)*

Requirements use `REQ-NNN` identifiers and EARS form. The upstream template's `FR-NNN` / `MUST`
examples do not apply to this project.

### Functional Requirements

- REQ-001: The validator shall discover a requirement written as a bullet list item, a numbered list item, a heading, a table row, or a block quote.
- REQ-002: When a requirement identifier appears inside a fenced code block, the validator shall exclude that line from requirement discovery.
- REQ-003: When a requirement identifier appears more than once in one specification, the validator shall report a duplicate-identifier finding.
- REQ-004: The validator shall accept a requirement identifier whose numeric part contains three or more digits.
- REQ-005: Where the all-features option is supplied, the validator shall evaluate every specification matched by the configured specification glob.
- REQ-006: If the active-feature pointer names a specification that does not exist, then the validator shall report a discovery finding.
- REQ-007: The validator shall state the number of specification files examined in every result it produces.
- REQ-008: The machine-readable result shall carry a schema version identifier.
- REQ-009: When a test selector resolves to a location outside the configured test roots, the validator shall report a traceability finding.
- REQ-010: When a specification file cannot be decoded as UTF-8, the validator shall report an unreadable-specification finding.
- REQ-011: While a validation run is in progress, the validator shall leave every file in the project unmodified.
- REQ-012: When a competing modal verb appears inside a quoted literal within a requirement sentence, the validator shall treat that occurrence as non-normative.
- REQ-013: Where more than one specification is evaluated in one run, the validator shall qualify each requirement identifier with the specification that declares it.
- REQ-014: The validator shall report a production-code separation finding at most once for each combination of file, line, and qualified requirement identifier.
- REQ-015: The conformance corpus shall store each case as a project fixture together with its expected machine-readable result.
- REQ-016: When a conformance case is executed, the harness shall invoke the validator through its documented command-line interface.

### Key Entities

- **Specification**: A Markdown file containing zero or more normative requirements, identified by
  the configured glob or by the active-feature pointer.
- **Requirement**: A uniquely identified normative sentence in EARS form, with a source file and
  line number.
- **Finding**: A coded, located, machine-readable statement that some rule was violated.
- **Result**: The complete outcome of one validation run, versioned and serializable.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- SC-001: A specification whose requirements are expressed in any of the five supported Markdown
  forms yields the same requirement count as the equivalent bullet-form specification.
- SC-002: Running against the existing eleven-feature reference project in all-features mode reports
  eleven features rather than one.
- SC-003: Every finding code the validator can emit is exercised by at least one fixture in the
  conformance corpus.
- SC-004: The conformance corpus detects any change to the machine-readable result shape.

## Assumptions

- Specifications are authored in Markdown and stored one feature per directory.
- The existing configuration file format and its defaults remain unchanged by this feature.
- Changing the default single-feature scoping behaviour is out of scope; all-features scope is opt-in
  so that existing workflows keep working.
- Requirement identifiers keep the `REQ-` prefix; renaming the prefix is out of scope.
