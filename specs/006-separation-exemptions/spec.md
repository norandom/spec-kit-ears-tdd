# Feature Specification: Separation Exemptions

**Feature Branch**: `006-separation-exemptions`

**Created**: 2026-08-17

**Status**: Draft

**Input**: The separation gate forbids requirement identifiers and copied requirement prose in
production code, and it is right to. But it has no way to express its own policy's exception —
"tests may reference requirement IDs; production code may not" — in a language that keeps unit tests
inside the file they test. It has caught this repository's own source four times in two days, every
one a legitimate mention: doc comments explaining the identifier format, and a test fixture inside a
production module.

Each time the fix was to reword until the checker stopped noticing. That is the wrong habit to
teach. A gate with no way to say "this one is intentional" trains people to hide from it, and the
next person to hit it will reach for the same reflex on a mention that *is* a real leak.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - A legitimate mention can be declared (Priority: P1)

An author writes a doc comment, a test fixture, or an error message that necessarily contains a
requirement identifier. They mark that line as intentional, with a reason, and the gate accepts it
while leaving the mark visible to every future reader and reviewer.

**Why this priority**: Without it the only escape is rewording, which is indistinguishable from
concealment and leaves no record of the judgement.

**Independent Test**: Put a requirement identifier in a production file with a marker and a reason;
assert the gate passes and reports the exemption.

**Acceptance Scenarios**:

1. **Given** a marked line containing a requirement identifier, **When** the final gate runs,
   **Then** no separation error is reported for that line.
2. **Given** a marker with no reason, **When** the final gate runs, **Then** the marker is rejected
   and the separation error stands.
3. **Given** a marker on a line containing no requirement identifier, **When** the final gate runs,
   **Then** the redundant marker is reported.

---

### User Story 2 - Exemptions are counted, never invisible (Priority: P1)

Every exemption appears in the report and in the count of what was suppressed. A reviewer can see at
a glance how many there are and whether the number is growing.

**Why this priority**: An exemption mechanism that hides its own use converts a loud gate into a
quiet one, which is worse than having no exemption at all. The point is to move a judgement into the
open, not to remove it from the report.

**Independent Test**: Exempt two lines and assert the result reports two exemptions.

**Acceptance Scenarios**:

1. **Given** any run with exemptions in effect, **When** the report is produced, **Then** it states
   how many separation findings were exempted.
2. **Given** an exempted line, **When** the report is produced, **Then** an advisory names the file,
   the line, and the declared reason.

### Edge Cases

- A marker appears inside a fenced block in a Markdown file that happens to be a production source.
- A configured exemption pattern matches no file at all.
- A marker sits on the line above the identifier rather than on the same line.
- An exemption is declared for copied requirement prose rather than an identifier.
- Every production file is exempted, which should be visible as a number rather than as silence.

## Requirements *(mandatory)*

### Functional Requirements

- REQ-001: Where a production line carries an exemption marker, the validator shall suppress the separation finding for that line.
- REQ-002: The exemption marker shall carry a reason of at least ten characters.
- REQ-003: If an exemption marker carries no reason, then the validator shall reject the marker and report the separation finding.
- REQ-004: When an exemption marker appears on the line preceding an identifier, the validator shall apply it to that identifier.
- REQ-005: If an exemption marker suppresses nothing, then the validator shall report a redundant-exemption finding.
- REQ-006: The validator shall report an advisory naming the file, the line, and the reason for every exemption it applies.
- REQ-007: The validator shall report the number of separation findings that exemptions suppressed.
- REQ-008: Where the configuration declares an exempt path pattern, the validator shall suppress separation findings for files matching it.
- REQ-009: If a configured exempt path pattern matches no file, then the validator shall report a stale-pattern finding.
- REQ-010: The validator shall apply an exemption to copied requirement prose on the same terms as a requirement identifier.

### Key Entities

- **Exemption marker**: An inline comment declaring that a requirement identifier on this line or
  the next is intentional, together with the reason it is.
- **Exempt path pattern**: A configured glob naming files whose separation findings are suppressed
  wholesale, for cases a per-line marker cannot reach.
- **Suppression count**: How many findings the exemptions removed, reported so that the mechanism
  cannot quietly grow.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- SC-001: This repository's own source passes the final gate without rewording any comment whose
  natural phrasing contains a requirement identifier.
- SC-002: Every applied exemption is visible in the report; none is silent.
- SC-003: An exemption without a reason never suppresses anything.
- SC-004: A reviewer can determine the total number of exemptions from the report alone.

## Assumptions

- The marker is a comment in whatever language the file is written in; the validator matches its text
  rather than parsing the language.
- Exemptions apply to the separation gate only. They do not suppress EARS, traceability, task
  coverage, or vocabulary findings, none of which have a comparable legitimate exception.
- Language-aware exclusion of test regions is out of scope. Detecting them means parsing every
  supported language, and a marker states the intent explicitly rather than inferring it.
