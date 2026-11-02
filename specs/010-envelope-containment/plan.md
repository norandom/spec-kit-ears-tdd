# Implementation Plan: Envelope Containment

**Feature**: `010-envelope-containment` | **Date**: 2026-09-22 | **Spec**: [spec.md](spec.md)

**Repository branch at planning time**: `main`. Nothing is implemented. This feature starts after
the 009 engine's first usable slice (US5 + US1 + US2 of 009) is green.

## Summary

Read an agent workflow definition into an envelope model (one reserved phase enumeration, tool and
transition effects, derived phase identifiers), through a named adapter from a fixed registry, with
guard terms resolved against an envelope-specific vocabulary. Feed both sides to the 009 containment
engine under the `revision` orientation. Name every construct not modelled, state that reachability
was not computed, and record the adapter per source.

## Technical context

| Concern | Choice |
|---|---|
| Language/platform | Rust 1.82; existing static binary |
| Dependencies | Existing `serde_json`, `toml`; no new crate |
| Storage | Read-only definitions, sidecar derogation models, envelope vocabularies |
| Public surface | Manifest `kind = "envelope"`, source fields `definition`, `vocabulary`, `format`, optional `model` |
| Tests | Corpus cases under `conformance/pending/contain-envelope-*`, unit tests in `envelope.rs` and `adapters.rs` |
| Limit | 009 state budget per effect question |
| Scope | 23 requirements, three stories; US3's native adapter blocked on a sample file |

## Constitution check

As 009: one binary, no subprocesses, read-only inputs, visible omissions, deterministic evidence,
no requirement IDs in production code. The adapter registry is a `match`, not a plugin mechanism,
which keeps the single-binary property.

## Documents

- [research.md](research.md): why an own vocabulary, why adapter-first, where set logic suffices.
- [data-model.md](data-model.md): additions to the 009 model.
- [contracts/envelope.md](contracts/envelope.md): source shape, interchange format, adapters,
  findings.
- [quickstart.md](quickstart.md): validation now and after implementation.
- [tasks.md](tasks.md): ordered red/green work with requirement and test mappings.

## Code structure and sequence

1. `crates/ears-sdd/src/envelope.rs` (new): interchange reader producing `EnvelopeModel`
   (phase enum, contributions per phase, unmodelled JSON-pointer paths). Structural refusals here.
2. `crates/ears-sdd/src/adapters.rs` (new): `Format` parsing (`<name>/<version>`), a fixed
   `match` from format to reader, `AdapterId { id, version }`. The interchange reader is the only
   entry until a native sample is checked in.
3. `contain.rs` (009): accept envelope sources; load the envelope vocabulary through the existing
   vocabulary parser without running the unused-term pass; build questions from `EnvelopeModel`
   contributions the same way as from a `ModelFile` snapshot; apply the `revision` orientation.
4. `report.rs` / `lib.rs`: `adapter` per source, `reachability_computed`, `unmodelled` on envelope
   records; `CONTAIN_REACHABILITY_OMITTED` on every completed envelope comparison.
5. Corpus: upgrade `contain-envelope-widened`, write the remaining `contain-envelope-*` cases.

## Requirement-to-design translation

| Requirements | Design | Verification |
|---|---|---|
| REQ-001, REQ-002, REQ-003, REQ-004 | Interchange reader, phase enum, stable phase IDs | `contain-envelope-widened` |
| REQ-005, REQ-006, REQ-015 | Terms resolved against the envelope vocabulary; affected transition effects withheld | `contain-envelope-undeclared-term`, `contain-envelope-vocabulary` |
| REQ-007 | Existing parser/type checks; no host expressions | `contain-envelope-guard-invalid` |
| REQ-008, REQ-009 | Sorted unmodelled paths on either side, including tool-less phases | `contain-envelope-widened` plus unknown-field variant |
| REQ-010, REQ-011 | Phase-local comparison, reachability disclosed absent | `contain-envelope-widened` plus unreachable-phase variant |
| REQ-012, REQ-013 | Structural/schema read failure withholds definition | `contain-envelope-unreadable` |
| REQ-014 | Source-phase AND guard per transition effect | `contain-envelope-transition-widened` |
| REQ-016, REQ-017 | Vocabulary required; no unused-term pass over it | `contain-envelope-vocabulary`, `envelope.rs::tests::envelope_vocabulary_is_not_checked_for_use` |
| REQ-018 | `revision` orientation for the envelope kind | `contain-envelope-narrowed` |
| REQ-019, REQ-020, REQ-021, REQ-022 | Fixed adapter registry, default format, adapter provenance | `contain-envelope-format-unknown`, `adapters.rs::tests::registry_is_fixed` |
| REQ-023 | Native adapter returns dropped constructs | blocked on a checked-in sample; see T009 |

## Implementer context

Read the 009 plan's "Implementer context" first; everything there applies. This section adds what
is specific to envelopes.

### What an envelope becomes in code

Given the interchange example in the contract, the reader produces, per side:

| Contribution | Effect | Guard | Contributor ID |
|---|---|---|---|
| `planning` permits `Read` | `tool:Read` | `envelope-phase == 'planning'` | `PHASE-planning` |
| `planning` permits `Grep` | `tool:Grep` | `envelope-phase == 'planning'` | `PHASE-planning` |
| `planning` → `implement` when `test-result == 'fail'` | `transition:planning:implement` | `envelope-phase == 'planning' and test-result == 'fail'` | `PHASE-planning` |
| `implement` permits `Read` | `tool:Read` | `envelope-phase == 'implement'` | `PHASE-implement` |

`envelope-phase` is an enum term whose members are the phase names of that side. The two sides can
have different member sets; 009's shared-encoding rule (union of domains, per-source masks) handles
that. `test-result` comes from the envelope vocabulary. The `EnvelopeModel` is a list of these
contributions plus the unmodelled paths; it is not a `ModelFile`, because a `ModelFile` entry
carries one effect and a phase carries many.

### Where the engine is needed and where it is not

A tool widening is `tool:X` asserted under phase `p` on the candidate and not on the reference.
That is set membership. A transition widening is a real guard difference and needs the encoding.
Both must produce the same finding shape (`CONTAIN_WIDENED`, witness, contributor, derogation
check). If the tool path bypasses the encoding, its witness is `{ "envelope-phase": "<p>" }` and
nothing else, and the independent evaluator still confirms it.

### Rules specific to this feature

- The definition file is never written. The derogation sidecar is a separate file.
- The envelope vocabulary goes through `vocabulary::parse` and the structural checks; it does not
  go through the pass that raises `TERM_UNUSED`. Add a unit test that fails if it ever does.
- The adapter registry is a `match` on the format string. Adding an adapter is a code change with
  a version bump and a fixture. There is no path from configuration to a reader.
- Unknown fields are recorded, never interpreted. Unknown schema versions and unknown formats are
  refused, never partially read. The two rules are different and both are tested.
- Every completed envelope comparison emits `CONTAIN_REACHABILITY_OMITTED`, including a comparison
  with no findings. A green envelope result without that advisory is a bug.

### What is blocked and why

T009, the native adapter, needs a real workflow file of the target runtime's format checked in
under `conformance/pending/contain-envelope-native/`. Do not write the adapter from documentation
or from memory of the format. When the file arrives: write the mapping table (native construct →
interchange field or unmodelled), the fixture, and the adapter, in that order. Until then the task
stays open and the plan gate keeps reporting its selector as missing, which is the intended signal.
