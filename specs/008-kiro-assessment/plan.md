# Implementation Plan: Kiro Project Assessment

**Branch**: `008-kiro-assessment` | **Date**: 2026-09-10 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/008-kiro-assessment/spec.md`

## Summary

`ears-sdd validate` discovers a Kiro specification tree (`.kiro/specs/<feature>/requirements.md`), parses the four requirement grammars already sketched in `ccsdd.rs`, computes an active baseline from phase metadata, and checks that every superseded specification has a live successor. The assessed project is not modified. Spec Kit discovery is unchanged when no Kiro tree is present. Contradiction search is not added; when it does not run, the result says so.

## Technical Context

**Language/Version**: Rust 1.82 (workspace `rust-version`)

**Primary Dependencies**: Existing `ears-sdd` crate (`serde`, `serde_json`, `globset`, `ignore`, `regex`). No new crates.

**Storage**: N/A — read-only files in the assessed project

**Testing**: `cargo test -p ears-sdd` (conformance corpus + unit tests + read-only test)

**Target Platform**: Existing `ears-sdd` matrix (static binary, no interpreter)

**Project Type**: Library + CLI (`crates/ears-sdd`)

**Performance Goals**: A 24-feature tree with a few hundred criteria (facdrone scale) assessed in well under one second

**Constraints**: Validation writes nothing. No sidecar config in the assessed tree. No prose-level contradiction search. No EARS-form lint of Kiro criteria in this feature.

**Scale/Scope**: One crate, one CLI entry. ~12 conformance fixtures plus parser unit tests. Does not open `../facdrone` in CI.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

`.specify/memory/constitution.md` is still the Spec Kit placeholder and is not an enforceable gate. The project’s real constraints are in `docs/architecture.md` and `docs/design/decisions.md`:

- Validation is read-only and never runs project tests.
- Scope is part of the claim; a check that did not run is named.
- Behavioural contract lives in `conformance/cases/` as data.
- Production code must not contain requirement IDs or copied EARS sentences.
- Single static binary; no new interpreter or solver.

This design satisfies those. No complexity-tracking row.

**Post-design re-check**: Contracts add optional report fields without a schema bump; Kiro checks are omitted with an advisory rather than silence; `CONFIG_MISSING` is suppressed only when a Kiro tree is present. Still no constitution violation.

## Project Structure

### Documentation (this feature)

```text
specs/008-kiro-assessment/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── cli.md
│   ├── findings.md
│   └── report.md
├── checklists/requirements.md
├── traceability.toml
└── tasks.md              # $speckit-tasks — not created here
```

### Source Code (repository root)

```text
crates/ears-sdd/src/
├── ccsdd.rs              # parser (already present; add tests, keep public IR)
├── kiro.rs               # new: tree walk, spec.json, baseline, supersession graph
├── discovery.rs          # union Kiro locations with Spec Kit glob; --feature for Kiro
├── lib.rs                # skip Spec Kit checks on kiro family; filter CONFIG_MISSING
├── report.rs             # optional baseline fields on Summary / FeatureResult
├── config.rs             # unchanged load; caller decides CONFIG_MISSING
└── main.rs               # unchanged CLI surface

crates/ears-sdd/tests/
├── conformance.rs        # picks up new cases automatically
└── read_only.rs          # also snapshot a Kiro fixture

conformance/cases/
├── kiro-discovers-tree/
├── kiro-missing-requirements-md/
├── kiro-empty-headings/
├── kiro-missing-criteria/
├── kiro-fenced-example/
├── kiro-active-baseline/
├── kiro-supersession-recut/
├── kiro-supersession-dangling/
├── kiro-supersession-unreciprocated/
├── kiro-amendment-extends/
├── kiro-operational-supersede/
└── kiro-unreadable-metadata/

docs/reference/findings.md   # document KIRO_* codes (implementation, not this phase)
```

**Structure Decision**: Stay in the single `ears-sdd` crate. Parser remains `ccsdd`; layout/baseline/supersession is a new `kiro` module so discovery stays the Spec Kit scoping story plus a union.

## Complexity Tracking

None.

## Requirement-to-design translation

| REQ | Design | Verification |
|---|---|---|
| REQ-001 | Walk `.kiro/specs/*/` when that directory exists; each child with `requirements.md` is a spec | `kiro-discovers-tree` |
| REQ-002 | `provenance.specs_examined` / `summary.specs_examined` count those directories | `kiro-discovers-tree` |
| REQ-003 | Spec Kit glob path unchanged when `.kiro/specs` is absent | `all-features-scope` |
| REQ-004 | Child without `requirements.md` → `KIRO_NO_REQUIREMENTS`, continue | `kiro-missing-requirements-md` |
| REQ-005 | `ccsdd::Parser` over each `requirements.md` | `kiro-discovers-tree`, `kiro-empty-headings` |
| REQ-006 | `feature` = directory basename; qualified id `{feature}:{id}` | `kiro-discovers-tree` |
| REQ-007 | No requirement headings → zero criteria, no error | `kiro-empty-headings` |
| REQ-008 | Parser note `REQ_NO_CRITERIA` promoted to `KIRO_NO_CRITERIA` | `kiro-missing-criteria` |
| REQ-009 | Existing fence skip in `ccsdd` | `kiro-fenced-example` |
| REQ-010 | `kiro::baseline` over discovered Kiro specs | `kiro-active-baseline` |
| REQ-011 | Exclude `superseded` / `reserved` / `initialized` | `kiro-active-baseline` |
| REQ-012 | `summary.baseline_*` + human `Baseline:` line | `kiro-active-baseline` |
| REQ-013 | Missing `spec.json` / missing `phase` → included | `kiro-active-baseline` |
| REQ-014 | Feature-name occurrences in `notes` of superseded specs | `kiro-supersession-recut` |
| REQ-015 | Supersede lemma + feature name in non-criteria prose | `kiro-supersession-unreciprocated`, `kiro-operational-supersede` |
| REQ-016 | No link to an included spec → `KIRO_SUPERSESSION_DANGLING` | `kiro-supersession-dangling` |
| REQ-017 | Claim targeting a still-included spec → `KIRO_SUPERSESSION_UNRECIPROCATED` | `kiro-supersession-unreciprocated` |
| REQ-018 | Two included successors, no finding | `kiro-supersession-recut` |
| REQ-019 | `extends` does not create a supersession link | `kiro-amendment-extends` |
| REQ-020 | Amendment criteria share the parent feature | `kiro-amendment-extends` |
| REQ-021 | Existing read-only invariant; add a Kiro fixture snapshot | `tests/read_only.rs`, `kiro-discovers-tree` |
| REQ-022 | No `.specify/` in Kiro fixtures; `CONFIG_MISSING` filtered | `kiro-discovers-tree` |
| REQ-023 | `family = kiro` skips identifier/traceability/tasks/separation; `KIRO_CHECKS_OMITTED` | `kiro-discovers-tree` |
| REQ-024 | Default config leaves constraints off; `disabled_checks` lists it | `kiro-discovers-tree` |
| REQ-025 | Unreadable `spec.json` → `KIRO_METADATA` | `kiro-unreadable-metadata` |
| REQ-026 | That specification is excluded (`unreadable-metadata`) | `kiro-unreadable-metadata` |

Do not copy these identifiers into production code. Tests and conformance fixtures may name them.
