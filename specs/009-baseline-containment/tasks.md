# Tasks: Baseline Containment

**Input:** [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md),
[data-model.md](data-model.md), [contracts](contracts/containment.md).

**Status:** Work breakdown only. No implementation task is complete. Tests are mandatory.
Every behavioral task below names its requirement coverage and selectors. Paths are repository-relative.

## Phase 1: Prepare evidence

- [x] T001 Inventory all selectors in `specs/009-baseline-containment/traceability.toml` against the files and contract; record current missing evidence and fixture changes in `specs/009-baseline-containment/quickstart.md`. Author the test tasks below before production work so the tasks gate can find real test bodies and pending expectations; do not add empty placeholder files or weaken verification.
- [x] T002 Add a failing regression around the normalizer in `crates/ears-sdd/tests/conformance.rs` proving that changing a containment witness, effect, reference source, comparison ID, or exit status fails the assertion, while old cases retain their normalized shape. Coverage: REQ-004, REQ-012, REQ-030. Selector: `crates/ears-sdd/tests/conformance.rs::containment_evidence_is_not_discarded`.
- [x] T003 Extend `crates/ears-sdd/tests/conformance.rs` to preserve containment details/comparisons and check exit status; run T002 red then green. Coverage: REQ-004, REQ-012, REQ-030. Selector: `crates/ears-sdd/tests/conformance.rs::containment_evidence_is_not_discarded`.

## Phase 2: Foundations

All story test-authoring tasks may be completed in advance. Before changing production behavior,
run `cargo run -q -p ears-sdd -- validate --phase tasks --feature specs/009-baseline-containment`.
Resolve every missing selector with its real test body or fixture. A passing artifact gate does
not satisfy that test; each red run still precedes implementation.

- [x] T004 Add failing tests for opt-in/default-off disclosure, missing/empty manifest, duplicate IDs, required source snapshots, unknown fields/version, scope selection, and invalid check dependencies in `crates/ears-sdd/src/contain.rs`; author `conformance/cases/contain-reference-missing/expected.json` and its fixture. Coverage: REQ-001, REQ-002, REQ-003, REQ-004. Selectors: `crates/ears-sdd/src/contain.rs::tests::comparison_inputs_are_explicit`, `conformance/cases/contain-reference-missing/expected.json`.
- [x] T005 Load the typed comparison manifest and source provenance in `crates/ears-sdd/src/contain.rs`, add optional config/report records in `config.rs` and `report.rs`, and wire `lib.rs` without evaluating effects yet. Requested but unanswered comparisons remain uncompared. Extract only reusable parsing/type checks from `analysis.rs` and `vocabulary.rs`. Run T004 red then green. Coverage: REQ-001, REQ-002, REQ-003, REQ-004. Selectors: T004.

## Phase 3: US5 — A refused comparison is not a pass (P1 prerequisite)

**Independent check:** An integer guard comparison uses both partitions; unknown terms and
over-budget questions remain visibly unanswered while unrelated affordable effects still run.

- [x] T006 [US5] Write shared-encoding tests in `crates/ears-sdd/src/contain.rs` and cases under `conformance/pending/contain-shared-partition/`, `contain-deprecated-term/`, and `contain-vocab-drift/`. Cover added/removed enum members, changed integer bounds, disjoint domains, `i64` endpoints, declaration order, replacement cycles and incompatible kinds. Coverage: REQ-005, REQ-006, REQ-007, REQ-026, REQ-027, REQ-028. Selectors: `crates/ears-sdd/src/contain.rs::tests::one_shared_variable_ordering`, `crates/ears-sdd/src/contain.rs::tests::shared_encoding_preserves_distinct_states`, and each named directory's `expected.json`.
- [x] T007 [US5] Build per-effect shared variables, canonical identities, side-domain masks and affected-effect refusal in `crates/ears-sdd/src/contain.rs`; make endpoint arithmetic safe in `model.rs`. Reject invalid domain declarations before encoding. Run T006 red then green and preserve existing model results. Coverage: REQ-005, REQ-006, REQ-007, REQ-026, REQ-027, REQ-028. Selectors: T006.
- [x] T008 [US5] Author `conformance/cases/contain-budget-exceeded/expected.json` and `conformance/cases/contain-unmodelled/expected.json`, including overflow, budget equality, zero budget with an unconditional effect, and partially modelled variants. Add an evaluation-entry counter test in `crates/ears-sdd/src/contain.rs` proving no evaluator or encoding is entered for an over-budget question. Coverage: REQ-023, REQ-024, REQ-025, REQ-029, REQ-030. Selectors: the two `expected.json` files and `crates/ears-sdd/src/contain.rs::tests::over_budget_does_not_evaluate`.
- [x] T009 [US5] Apply checked state-size preflight, distinguish indeterminate/uncompared results, and record side-qualified excluded requirement counts in `crates/ears-sdd/src/contain.rs` and `report.rs`. Missing, malformed, unreadable and type-invalid inputs never become empty models. Run T008 red then green. Coverage: REQ-023, REQ-024, REQ-025, REQ-029, REQ-030. Selectors: T008.

## Phase 4: US1 — Detect widening with a reproducible witness (P1)

**Independent check:** Dropping one conjunct yields the exact extra state and all active candidate
contributors; identical models yield no directional finding with explicit reference provenance.

- [x] T010 [US1] Upgrade `conformance/cases/contain-equal/` and `conformance/cases/contain-widened/` to the contracts: explicit manifest/reference spec/vocabulary, all declared effect counts, comparison provenance and witness details. Write `conformance/cases/contain-unguarded/expected.json` with candidate/reference fixtures. Cover valid zero-effect empty models, empty effect regions, missing-reference declarations that cannot establish a new effect, unsatisfiable guards, multiple contributors and multiple comparisons sharing IDs. Coverage: REQ-001, REQ-004, REQ-008, REQ-009, REQ-010, REQ-012, REQ-013, REQ-030. Selectors: each directory's `expected.json`.
- [x] T011 [P] [US1] Add real assertion bodies to `crates/ears-sdd/tests/determinism.rs::containment_witness_is_stable`, `crates/ears-sdd/src/contain.rs::tests::witness_satisfies_attributed_guards`, and `crates/ears-sdd/tests/differential.rs::containment_regions_agree`. Compare repeated/reordered inputs and independent evaluation of union/difference masks, including the negative half of each witness. Coverage: REQ-005, REQ-006, REQ-007, REQ-015, REQ-016. Selectors: the three named tests.
- [x] T012 [US1] Implement effect unions and forward differences with one `bdd::Encoding` per question in `crates/ears-sdd/src/contain.rs`; choose the canonical semantic witness and independently verify it using `enumerate.rs`. Report all contributors, preserve every effect/comparison during sorting/deduplication, and retain existing model defects. Run T010 and T011 red then green. Coverage: REQ-008, REQ-009, REQ-010, REQ-012, REQ-013, REQ-015, REQ-016, REQ-030. Selectors: T010 and T011.
- [x] T013 [US1] Integrate comparison results into JSON and human output in `crates/ears-sdd/src/lib.rs` and `report.rs`. Show source, status, witnesses and coverage; preserve `status` exit behavior and disabled-layer compatibility. Re-run T002, T004 and T010. Coverage: REQ-001, REQ-003, REQ-004, REQ-012, REQ-025, REQ-030. Selectors: T002, T004 and T010.

## Phase 5: US2 — Report dropped obligations and new effects (P1)

**Independent check:** Adding a conjunct names a reference witness; a newly declared effect is
distinguished from its nonempty widening, and cross-baseline conflicts remain errors.

- [x] T014 [US2] Upgrade `conformance/cases/contain-narrowed/expected.json` and write `conformance/cases/contain-effect-new/expected.json` and `conformance/cases/contain-baseline-conflict/expected.json` with their complete fixtures. The conflict case declares `kind = "policy"`, because that is the only kind under which the question is asked. Cover declaration-only empty effects, conflict declarations on either side, disjoint guard dependencies and internal defects that cannot become vacuous passes. Coverage: REQ-011, REQ-014, REQ-017, REQ-018. Selectors: the three named expectation files.
- [x] T015 [US2] Add reverse differences, declaration differences and symmetric cross-side conflict queries in `crates/ears-sdd/src/contain.rs`; apply independent per-pair budgets and source attribution. Run T014 red then green. Coverage: REQ-011, REQ-014, REQ-017, REQ-018, REQ-023, REQ-024, REQ-025. Selectors: T014 and `conformance/cases/contain-budget-exceeded/expected.json`.

## Phase 6: US3 — Record deliberate widening (P1)

**Independent check:** A valid same-effect reference derogation declares the full widening;
short reasons, unknown targets and stale entries remain findings.

- [x] T016 [US3] Upgrade `conformance/pending/contain-widened-declared/` and `contain-derogation-stale/`; write `contain-derogation-unknown/expected.json` and `contain-derogation-invalid/expected.json`. Test wrong-effect targets, trimmed Unicode reason length, multiple contributors beyond the first witness, duplicate coverage and deferred staleness on withheld/budget-refused questions in `crates/ears-sdd/src/contain.rs`. Coverage: REQ-019, REQ-020, REQ-021, REQ-022. Selectors: each directory's `expected.json` and `crates/ears-sdd/src/contain.rs::tests::derogations_cover_every_contributor`.
- [x] T017 [US3] Add the optional derogation table in `crates/ears-sdd/src/model.rs` and implement validation/whole-region usage checks in `contain.rs`. Existing constraint analysis accepts the new field without changing conflict adjudication. Only fully covered widening becomes advisory. Run T016 red then green. Coverage: REQ-019, REQ-020, REQ-021, REQ-022. Selectors: T016.

## Phase 7: US4 — Check supersession and amendment content (P1)

**Independent check:** Two successors jointly cover a predecessor; deleting an obligation yields a
gap only when all relevant successors were compared. Kiro heading links compare grouped criteria.

- [x] T018 [US4] Write complete cases under `conformance/pending/contain-supersession-gap/`, `contain-supersession-covered/`, and `contain-supersession-uncompared/`. Include live-descendant chains, cycles, absent models, partially modelled successors equal local IDs in different successors, and an unguarded successor whose narrower domain cannot expand into another successor's states. Coverage: REQ-031, REQ-032, REQ-033. Selectors: each directory's `expected.json`.
- [x] T019 [US4] Expose existing successor edges from `crates/ears-sdd/src/kiro.rs`, resolve live descendants and compare their source-qualified union in `contain.rs`. Verify manifest mappings cover the live set, map narrowing to gaps, and withhold unproven gaps for partial unions. Run T018 red then green. Coverage: REQ-031, REQ-032, REQ-033. Selectors: T018.
- [x] T020 [P] [US4] Write `conformance/pending/contain-amendment-refines/expected.json` and its Kiro fixture with multiple criteria under an amendment and multiple explicitly extended baseline headings; include unknown targets, unrelated requirements and two separate amendments sharing an effect, selected by distinct amendment headings. Coverage: REQ-034. Selector: `conformance/pending/contain-amendment-refines/expected.json`.
- [x] T021 [US4] Retain parsed `CcsddRequirement::extends` and criteria in `crates/ears-sdd/src/kiro.rs`; build scoped unions for each amendment in `contain.rs`, using existing `ccsdd.rs` parsing. Support explicit Spec Kit requirement pairs without inferring links from prose. Run T020 red then green. Coverage: REQ-034. Selector: T020.
- [ ] T022 [P] [US4] Write `conformance/pending/contain-assessed-reference/expected.json` and an assessor plus separate Kiro tree. Extend `crates/ears-sdd/tests/read_only.rs::no_gate_modifies_the_project` to snapshot both roots and assert containment actually ran on success and failure; cover explicit external roots and refused path escape. Coverage: REQ-035, REQ-036. Selectors: the named expectation and test.
- [ ] T023 [US4] Resolve external specification paths only under the explicitly named source root and keep models/vocabularies under the assessor in `crates/ears-sdd/src/contain.rs`; retain source provenance and read-only behavior. Run T022 red then green. Coverage: REQ-035, REQ-036. Selectors: T022.

## Phase 8: US6 — Policy is an upper bound, amendment is a refinement (P1)

**Independent check:** Under `kind = "policy"` a narrower candidate produces no finding and a wider
one produces `CONTAIN_WIDENED`; under `kind = "amendment"` narrowing is silent; under
`kind = "revision"` no conflicting-effect question is constructed; every comparison record carries
`kind` and `reports`.

- [x] T024 [US6] Author `conformance/pending/contain-policy-upper-bound/` to the contract: manifest with `kind = "policy"`, a policy under `.specify/policy/` outside `spec_glob`, a candidate narrower on one effect and wider on another, and an `expected.json` with exactly one `CONTAIN_WIDENED` and no `CONTAIN_NARROWED`. Add `crates/ears-sdd/src/contain.rs::tests::conflict_questions_only_under_policy`, counting constructed questions per kind over one model pair with a declared conflicting pair. Extend `contain-amendment-refines/expected.json` (T020) with a narrower amendment that produces no finding and a wider one that produces `CONTAIN_WIDENED`. Coverage: REQ-018, REQ-037, REQ-038, REQ-039, REQ-040. Selectors: `conformance/cases/contain-policy-upper-bound/expected.json`, `conformance/pending/contain-amendment-refines/expected.json`, and the named unit test.
- [x] T025 [US6] Implement the kind orientation in `crates/ears-sdd/src/contain.rs` as one `match` on the kind enum applied after both differences are computed; gate conflicting-effect question construction on the policy kind; stamp `kind` and `reports` on every comparison record in `report.rs` and render them in the human output. Confirm in `contain.rs::tests::comparison_inputs_are_explicit` that a reference path is read literally and that no version-control lookup exists on the code path (no `git` invocation, no `.git` read). Run T024 red then green; re-run T010 for the `reports` field on `contain-equal`. Coverage: REQ-018, REQ-037, REQ-038, REQ-039, REQ-040, REQ-041. Selectors: T024 and `crates/ears-sdd/src/contain.rs::tests::comparison_inputs_are_explicit`.

## Phase 9: Integration and final verification

- [x] T026 Move implemented case directories from `conformance/pending/` to `conformance/cases/`, repoint every affected selector in `specs/009-baseline-containment/traceability.toml`, and update `conformance/pending/README.md`. Do this per passing increment; never move unimplemented cases merely to increase coverage counts. Coverage: REQ-001 through REQ-041. Selectors: all mappings in `traceability.toml`.
- [ ] T027 Update `docs/reference/configuration.md`, `docs/reference/findings.md`, `docs/reference/gates.md` and `specs/009-baseline-containment/quickstart.md` with the shipped contract and executed scenarios. Correct warning/exit-status documentation to match existing gate behavior. Document the reference-snapshot copy step and the kind orientation table. Coverage: REQ-004, REQ-025, REQ-030, REQ-040, REQ-041. Selectors: the activated equal, budget and policy corpus cases.
- [x] T028 Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, configured `cargo nextest run`, and `cargo run -q -p ears-sdd -- validate --phase final --all`. Record actual results in `specs/009-baseline-containment/quickstart.md`; mark tasks complete only after their behavior and tests pass. Coverage: REQ-001 through REQ-041. Selectors: full conformance corpus, differential, determinism and read-only tests named above.

## Dependencies and parallel opportunities

Execution: evidence preparation → foundation → US5 → US1 → US2 → US3 → US6 → US4 → integration.
US5 runs first because a witness is meaningful only after encoding and refusal rules hold. US6
needs both directions from US1/US2 and the derogation table from US3, and US4's supersession and
amendment orientations are instances of the same `match`, so US6 precedes US4. Nothing here
depends on `010-envelope-containment`; that feature depends on this one.

Author all test files/fixtures first to satisfy the artifact gate, then run each story's tests red
before its production changes and green afterward. Fixtures may be authored independently, but
production tasks touching `contain.rs`, `model.rs`, `kiro.rs`, `lib.rs` or `report.rs` remain sequential.
A task is done when its tests pass and the four commands in the plan's "How to work a task" section are clean.
The `[P]` labels mean independent test authoring after setup, not concurrent edits to the same file.

| Story | Safe parallel authoring example |
|---|---|
| US1 | T010 JSON fixtures while T011 edits the determinism/differential tests |
| US2 | The three T014 fixture directories can be authored separately |
| US3 | The four T016 fixture directories can be authored separately; one owner for unit tests |
| US4 | T020 amendment fixture and T022 external/read-only test after T018 |
| US5 | T006 partition/vocabulary fixtures and T008 budget fixtures; serialize unit-file edits |
| US6 | T024 policy fixture and amendment fixture extension can be authored separately |

There are 28 tasks: 5 setup/foundation, US1 4, US2 2, US3 2, US4 6, US5 4, US6 2,
and 3 integration tasks. The first usable milestone is US5 + US1 + US2; the full feature includes
every story. No deploy, release, merge, envelope or external runtime work is part of this task list.

## Notes

- M2 slice (2026-11-02): derogations (US3), supersession union over explicit candidate lists and
  Spec Kit amendment pairs (US4), kind orientation (US6), shared-partition/deprecated-term
  fixtures (T006), determinism + differential witness tests (T011). Still open: resolving live
  descendants from the Kiro supersession graph automatically (T019 follow-up), Kiro
  `amendment_heading` selection (T021 follow-up), and the external assessed-root fixture
  (T022/T023).
