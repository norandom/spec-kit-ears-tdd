# Feature Specification: Parked Selectors

**Feature Branch**: `011-parked-selectors`

**Created**: 2026-11-02

**Status**: Implemented

**Input**: A repository with in-progress features keeps conformance cases parked under
`conformance/pending/` until their implementation lands, and names those parked paths from
`traceability.toml`. Today every such selector is a `TRACE_TEST_FILE` error, so a spec that is
correctly staged holds the plan/tasks/final gates red for the whole duration of its feature.
Parking must be visible — a parked selector is declared intent, not a pass — but it must not fail
the run. This feature makes a missing file under a configured parked prefix an advisory
(`TRACE_TEST_PARKED`) while leaving every other selector check unchanged.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - A parked selector is declared, not an error (Priority: P1)

An operator runs the gate over a project whose feature 009/010-style work is in progress: specs are
written, conformance cases are parked under `conformance/pending/`, and traceability names the
parked paths. The run should be green with one advisory per parked selector, so CI stays meaningful
for everything that *is* implemented.

**Why this priority**: without it, every in-progress feature blocks the shared main branch, which
teaches people to either not stage tests early or to mark unimplemented work as done.

**Independent Test**: A fixture whose `traceability.toml` names a missing
`conformance/pending/…/expected.json` produces `ok: true` with a `TRACE_TEST_PARKED` advisory and
no `TRACE_TEST_FILE`.

**Acceptance Scenarios**:

1. **Given** a selector under a parked prefix whose file is absent, **When** the traceability gate
   runs, **Then** the finding is `TRACE_TEST_PARKED` with advisory severity and the run is `ok`.
2. **Given** a selector outside every parked prefix whose file is absent, **When** the gate runs,
   **Then** the finding is `TRACE_TEST_FILE` with error severity (unchanged behaviour).
3. **Given** `parked_selector_prefixes = []` in configuration, **When** a
   `conformance/pending/…` selector file is absent, **Then** it is a `TRACE_TEST_FILE` error.

---

### User Story 2 - Parking never weakens the checks that do run (Priority: P1)

A parked prefix changes only the severity of *missing* files. A parked file that exists is checked
exactly like any other selector, and traversal/test-root rules apply before parking is considered.

**Why this priority**: "parked" must not become a hole through which a renamed or deleted test
keeps its mapping green, or a selector escapes the project.

**Independent Test**: A parked selector naming `::a_test` inside a file that exists but does not
contain `a_test` produces `TRACE_TEST_NAME` as an error.

**Acceptance Scenarios**:

1. **Given** a selector with a named test under a parked prefix whose file exists but lacks the
   name, **When** the gate runs, **Then** `TRACE_TEST_NAME` is an error.
2. **Given** a parked selector that traverses out of the project or outside the configured test
   roots, **When** the gate runs, **Then** `TRACE_TEST_ROOT` is an error, not an advisory.

### Edge Cases

- A parked prefix that is a prefix of another parked prefix (longest-match not required; any match
  parks).
- A selector file that exists under a parked prefix and contains the named test: no finding.
- `require_test_files = false`: no file existence check at all, parked or not.
- Configuration with Windows-style separators in prefixes is normalized before matching.
- A parked selector in a Kiro-family specification is treated the same as in Spec Kit features.

## Requirements *(mandatory)*

Requirements use `REQ-NNN` identifiers and EARS form.

### Functional Requirements

- REQ-001: Where a test selector resolves under a parked prefix, the validator shall report a
  missing selector file with a TRACE_TEST_PARKED advisory.
- REQ-002: When a missing test selector resolves under no parked prefix, the validator shall report
  TRACE_TEST_FILE as an error.
- REQ-003: When a selector file exists under a parked prefix, the validator shall apply the same
  named-test check as for any other selector.
- REQ-004: The validator shall expose parked_selector_prefixes in configuration, defaulting to
  conformance/pending/.
- REQ-005: Where parked_selector_prefixes is empty, the validator shall report missing files under
  conformance/pending as TRACE_TEST_FILE errors.
- REQ-006: The validator shall apply test-root and traversal checks to parked selectors before and
  unchanged by the parking rule.

### Key Entities

- **Parked prefix**: A configured path prefix (normalized to POSIX form, trailing slash) marking
  selectors whose backing files are staged but not yet written. Default `conformance/pending/`.
- **Parked selector**: A test selector resolving under a parked prefix. Missing file ⇒
  `TRACE_TEST_PARKED` advisory; existing file ⇒ ordinary checks.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- SC-001: A fixture naming a missing parked selector passes with exactly one advisory and no
  errors.
- SC-002: The existing `selector-missing-test` and `selector-traversal` conformance cases are
  byte-unchanged, proving non-parked behaviour did not move.
- SC-003: With `parked_selector_prefixes = []`, the parked-missing fixture fails with
  `TRACE_TEST_FILE`.
- SC-004: This repository's own `validate --phase final --all` is green while specs 009/010 remain
  parked, with `TRACE_TEST_PARKED` advisories naming each pending selector.

## Assumptions

- Parking is a corpus-staging convention (`conformance/pending/`), not a claim that a test passes;
  the advisory keeps it visible in both human and machine-readable output.
- The default prefix matches this project's pending-corpus layout; other projects can override or
  empty it.
- A parked selector whose file exists is indistinguishable from an ordinary selector.
