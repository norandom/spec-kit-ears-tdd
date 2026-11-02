# Implementation Plan: Parked Selectors

**Branch**: `011-parked-selectors` | **Date**: 2026-11-02 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/011-parked-selectors/spec.md`

## Summary

The traceability gate gains a parking rule: a missing test file under a configured parked prefix
(default `conformance/pending/`) is a `TRACE_TEST_PARKED` advisory, not a `TRACE_TEST_FILE` error.
Existing files, named-test checks, test-root and traversal checks are unchanged. New config field
`parked_selector_prefixes`. Four conformance cases pin the behaviour.

## Technical Context

**Language/Version**: Rust 1.82 (workspace `rust-version`)

**Primary Dependencies**: Existing `ears-sdd` crate. No new crates.

**Storage**: N/A — read-only validation

**Testing**: `cargo test -p ears-sdd` (conformance corpus gains four cases)

**Project Type**: Library + CLI (`crates/ears-sdd`)

**Constraints**: Advisory must not affect `ok`. `deny_unknown_fields`-style strictness on config is
not used (`unknown` flatten map), so the new field is additive. No production code may contain
`REQ-NNN` identifiers.

## Constitution Check

`.specify/memory/constitution.md` is ratified v1.0.0. This design satisfies:

- Principle III (scope is part of the claim): a parked selector is *named* as advisory, never
  silently skipped — the advisory appears in human and JSON output and in `summary.advisories`.
- Principle II (read-only): no writes.
- Principle V (translation boundary): no requirement IDs in production code.
- Principle IV (contract is data): behaviour pinned by conformance cases.

## Project Structure

```text
specs/011-parked-selectors/
├── spec.md
├── plan.md
├── tasks.md
├── traceability.toml
├── checklists/requirements.md
└── quickstart.md

crates/ears-sdd/src/
├── config.rs          # + parked_selector_prefixes field, default, parked_prefixes() helper
└── traceability.rs    # parking branch in the require_test_files check

conformance/cases/
├── pending-selector-parked/            # missing parked file → advisory, ok
├── pending-selector-parked-present/    # existing parked file, no name → ok, no findings
├── pending-selector-parked-name-missing/ # existing parked file, wrong name → TRACE_TEST_NAME error
└── pending-selector-parked-disabled/   # prefixes = [] → TRACE_TEST_FILE error

docs/reference/
├── findings.md        # + TRACE_TEST_PARKED row
└── configuration.md   # + parked_selector_prefixes row
```

## Requirement-to-design translation

| REQ | Design | Verification |
|---|---|---|
| REQ-001 | `traceability.rs`: missing file under parked prefix → `TRACE_TEST_PARKED` advisory | `pending-selector-parked` |
| REQ-002 | unchanged error path | `selector-missing-test` (regression), `pending-selector-parked-disabled` |
| REQ-003 | name check runs on existing files regardless of prefix | `pending-selector-parked-name-missing`, `pending-selector-parked-present` |
| REQ-004 | `config.rs`: field + default + `parked_prefixes()` normalizer | `pending-selector-parked` |
| REQ-005 | empty prefix list parks nothing | `pending-selector-parked-disabled` |
| REQ-006 | root/traversal checks run before the parking branch | `selector-traversal` (regression) |

## Complexity Tracking

None.
