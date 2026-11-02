# Tasks: Envelope Containment

**Input:** [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md),
[data-model.md](data-model.md), [contracts/envelope.md](contracts/envelope.md).

**Status:** Work breakdown only. No task is complete. Tests are mandatory. Every task names its
requirement coverage and selectors. Paths are repository-relative. Prerequisite: the 009 engine's
first usable slice (009 tasks T001 to T015) is green.

## Phase 1: US1 — Envelope widening with a phase witness (P1)

**Independent check:** Adding a tool or weakening a transition guard produces its own effect
witness, even for an unreachable phase. Unmodelled constructs are named and reachability is
disclosed absent on every completed comparison.

- [x] T001 [US1] Upgrade `conformance/pending/contain-envelope-widened/` to the contract: `schema_version` on both definitions, a manifest with `kind = "envelope"` and explicit `definition`/`vocabulary` sources, a `comparisons` record with `reachability_computed`, `unmodelled` and `adapter`, and `detail` on the finding. Write `contain-envelope-transition-widened/expected.json` with a guard that lost a conjunct. Cover new tools, added/removed phases, case sensitivity, parallel transitions, no transitions, unreachable phases, unknown fields on both sides and tool-less phases. Coverage: REQ-001, REQ-002, REQ-003, REQ-004, REQ-008, REQ-009, REQ-010, REQ-011, REQ-014. Selectors: the two `expected.json` files and `crates/ears-sdd/src/envelope.rs::tests::local_permissions_preserve_phase_identity`.
- [x] T002 [US1] Implement `crates/ears-sdd/src/envelope.rs`: read versioned JSON, derive phase enum, tool and transition contributions and `PHASE-<name>` identifiers, record unmodelled JSON-pointer paths. Route through `contain.rs`, emit `CONTAIN_REACHABILITY_OMITTED`, and keep multiple findings for one phase distinct. Run T001 red then green. Coverage: REQ-001, REQ-002, REQ-003, REQ-004, REQ-008, REQ-009, REQ-010, REQ-011, REQ-014. Selectors: T001.
- [x] T003 [US1] Write `conformance/pending/contain-envelope-narrowed/expected.json` (a tool removed from a phase produces `CONTAIN_NARROWED` as a warning and the gate passes) and `contain-envelope-guard-invalid/` and `contain-envelope-unreadable/` (unsupported versions, malformed/duplicate JSON fields, invalid endpoints/types, enum values outside domains, reserved-term collisions, unsupported guard constructs). Coverage: REQ-007, REQ-012, REQ-013, REQ-018. Selectors: each directory's `expected.json` and `crates/ears-sdd/src/envelope.rs::tests::invalid_definitions_are_withheld`.
- [x] T004 [US1] Enforce schema/structural checks in `envelope.rs`, reuse guard parsing and type validation, withhold affected transition effects for invalid guards and the whole definition for unreadable structure, and apply the `revision` orientation to the envelope kind in `contain.rs`. Run T003 red then green. Coverage: REQ-007, REQ-012, REQ-013, REQ-018. Selectors: T003.

## Phase 2: US2 — Envelope vocabulary (P1)

**Independent check:** A term declared only in the envelope vocabulary resolves in a guard and is
never reported unused; a source without a vocabulary is withheld.

- [x] T005 [US2] Write `conformance/pending/contain-envelope-undeclared-term/` (guard names a term absent from the envelope vocabulary: `CONTAIN_UNDECLARED_TERM`, affected transition uncompared, unrelated tools still compared) and `contain-envelope-vocabulary/` (term declared only in the envelope vocabulary; product vocabulary gate enabled; no `TERM_UNUSED`; a second manifest entry without `vocabulary` produces `CONTAIN_INPUT_INVALID`). Add `crates/ears-sdd/src/envelope.rs::tests::envelope_vocabulary_is_not_checked_for_use`. Coverage: REQ-005, REQ-006, REQ-015, REQ-016, REQ-017. Selectors: the two `expected.json` files and the named test.
- [x] T006 [US2] Load the envelope vocabulary in `contain.rs` through the existing vocabulary parser and structural checks only; resolve guard terms against it alone; refuse a source without one. Run T005 red then green. Coverage: REQ-005, REQ-006, REQ-015, REQ-016, REQ-017. Selectors: T005.

## Phase 3: US3 — Adapters (P2)

**Independent check:** An unknown format is refused by name; the default and explicit interchange
format both record the adapter identity and version in provenance.

- [x] T007 [US3] Write `conformance/pending/contain-envelope-format-unknown/expected.json` (format with no adapter: `CONTAIN_DEFINITION_UNREADABLE` naming the format, comparison withheld) and extend `contain-envelope-widened` with one source naming `format = "ears-sdd-envelope/1.0"` explicitly and one omitting it, both recording the same adapter. Add `crates/ears-sdd/src/adapters.rs::tests::registry_is_fixed`, asserting the registry contents and that no configuration input reaches it. Coverage: REQ-019, REQ-020, REQ-021, REQ-022. Selectors: the expectation file, the extended `contain-envelope-widened/expected.json`, and the named test.
- [x] T008 [US3] Implement `crates/ears-sdd/src/adapters.rs`: format parsing, fixed registry, adapter identity and version on each envelope source record. Run T007 red then green. Coverage: REQ-019, REQ-020, REQ-021, REQ-022. Selectors: T007.
- [ ] T009 [US3] Blocked on input: a real workflow file of the target state-machine guardrail runtime, checked in under `conformance/pending/contain-envelope-native/`. When present: write the mapping table in `contracts/envelope.md`, the fixture's `expected.json` with every dropped native construct named, then the adapter in `adapters.rs` with its own version. Do not write the adapter from documentation. Coverage: REQ-023. Selectors: `conformance/pending/contain-envelope-native/expected.json` and `crates/ears-sdd/src/adapters.rs::tests::native_adapter_names_dropped_constructs`.

## Phase 4: Integration

- [x] T010 Move implemented cases from `conformance/pending/` to `conformance/cases/`, repoint selectors in `specs/010-envelope-containment/traceability.toml`, update `conformance/pending/README.md`, and document the envelope source, vocabulary, adapter table and findings in `docs/reference/configuration.md` and `docs/reference/findings.md`. Coverage: REQ-001 through REQ-023. Selectors: all mappings in `traceability.toml`.
- [x] T011 Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run`, and `cargo run -q -p ears-sdd -- validate --phase final --all`. Record results in `quickstart.md`. T009 remaining open is recorded as such, not as done. Coverage: REQ-001 through REQ-023. Selectors: full corpus and the unit tests named above.

## Notes

- M3 slice (2026-11-02): envelope reader (interchange format 1.0), reserved `envelope-phase`
  term, tool/transition effects, unmodelled-construct naming, reachability omission advisory,
  fixed adapter registry. 8 `contain-envelope-*` cases live in `conformance/cases/`.
  T009 (native adapter) stays blocked on a checked-in sample of the runtime's real file.

## Dependencies and parallel opportunities

Execution: US1 → US2 → US3 → integration. US1 needs 009's engine; US2 needs the reader; US3 needs
both. Fixture authoring within a phase can proceed in parallel; edits to `envelope.rs`,
`adapters.rs` and `contain.rs` are sequential. T009 waits for its input and blocks nothing else.

There are 11 tasks: US1 4, US2 2, US3 3, integration 2. The first usable milestone is US1 + US2.
