# Tasks: Separation Exemptions

**Input**: `spec.md` and `traceability.toml` in `specs/006-separation-exemptions/`

**Tests**: Mandatory. Eight conformance cases cover the behaviour, including one asserting that an
unmarked identifier still fails — the mechanism has to keep the gate biting.

**Organization**: Grouped by the two properties that keep an escape hatch from becoming a hole: a
marker without a reason does not count, and every exemption is reported.

---

## Phase 1: A mention can be declared intentional

- [X] T001 Add conformance cases for a marked line, a marker without a reason, a marker on the
      preceding line and an unmarked identifier covering REQ-001, REQ-002, REQ-003 and REQ-004,
      using `conformance/cases/exempt-marked-line`, `exempt-marker-without-reason`,
      `exempt-preceding-line` and `exempt-absent-still-fails`
- [X] T002 Recognize the marker as text rather than by parsing the language, so it works in any
      comment syntax, for REQ-001 in `crates/ears-sdd/src/exemptions.rs`
- [X] T003 Require a reason of at least ten characters and reject the marker without one, for
      REQ-002 and REQ-003 in `crates/ears-sdd/src/exemptions.rs`
- [X] T004 Apply a marker to its own line and the line after it for REQ-004 in
      `crates/ears-sdd/src/exemptions.rs`

## Phase 2: Exemptions are visible, never silent

- [X] T005 Add conformance cases for a redundant marker and for the exemption count covering
      REQ-005, REQ-006 and REQ-007, using `conformance/cases/exempt-redundant-marker`
- [X] T006 Report an advisory naming the file, line and reason for every applied exemption, for
      REQ-006 in `crates/ears-sdd/src/exemptions.rs`
- [X] T007 Report a marker that suppresses nothing for REQ-005 in
      `crates/ears-sdd/src/separation.rs`
- [X] T008 Report the suppression count on the phase that scans production code, present even when
      zero, for REQ-007 in `crates/ears-sdd/src/report.rs`

## Phase 3: Reach and coverage

- [X] T009 Add conformance cases for a configured path pattern, a stale pattern and exempted prose
      covering REQ-008, REQ-009 and REQ-010, using `conformance/cases/exempt-configured-path`,
      `exempt-stale-pattern` and `exempt-prose`
- [X] T010 Add the `separation_exempt` glob list and suppress matching files for REQ-008 in
      `crates/ears-sdd/src/config.rs` and `crates/ears-sdd/src/exemptions.rs`
- [X] T011 Report a configured pattern that matches no file for REQ-009 in
      `crates/ears-sdd/src/separation.rs`
- [X] T012 Attach a line to copied-prose findings so a marker can reach them, matching both a
      whole sentence on one line and a fragment of a reflowed one, for REQ-010 in
      `crates/ears-sdd/src/separation.rs`

## Phase 4: Prove it on this repository

- [X] T013 Restore the natural phrasing of the two doc comments previously reworded to evade the
      gate, declaring each with a marker, for REQ-001 in `crates/ears-sdd/src/tasks.rs`
- [X] T014 Assemble the marker constant from two pieces so the module defining it does not itself
      carry a reasonless marker, in `crates/ears-sdd/src/exemptions.rs`
