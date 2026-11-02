# Implementation Plan: Baseline Containment

**Feature**: `009-baseline-containment` | **Date**: 2026-09-18 | **Spec**: [spec.md](spec.md)

**Repository branch at planning time**: `main`. No feature branch or production implementation
has been created. The feature pointer now selects this directory.

## Summary

Compare effect activation regions between two explicitly sourced models. Reuse the existing guard
AST, finite domain representation, BDD encoding and independent evaluator. For each effect, union
its guards on each side under one shared encoding, retaining each source's domain limits before
combining sources; compare
both directional differences. Apply the comparison kind's orientation to decide which difference
is a finding. Return deterministic checked witnesses, source attribution and a separate completeness
status. Add input adapters for predecessor/successor unions and Kiro amendments. The workflow
envelope adapter is `010-envelope-containment` and reuses this engine unchanged.

## Technical context

| Concern | Choice |
|---|---|
| Language/platform | Rust 1.82; existing static binary and supported platform matrix |
| Dependencies | Existing `serde`, `serde_json`, `toml`, `biodivine-lib-bdd`; no new crate |
| Storage | Explicit read-only snapshots, vocabularies and assessment metadata |
| Public surface | Existing `validate` / `status`, optional `checks.containment`, comparison manifest |
| Tests | Existing corpus, differential harness, read-only test; focused containment unit and determinism tests |
| Limit | Existing `state_space_budget`, applied per effect or conflicting-effect pair before evaluation |
| Scope | 41 requirements, six stories, all P1; the envelope adapter is feature 010 |

## Constitution check

`.specify/memory/constitution.md` is an unfilled template, so its placeholder text supplies no
enforceable rules. Apply `docs/architecture.md` and `docs/design/decisions.md`: one binary, no
subprocesses during validation, read-only inputs, visible omissions, deterministic evidence, and
no requirement IDs or copied normative prose in production code. Tests and fixtures may use IDs.

**Before and after design:** Satisfied by the proposed design. No solver, runtime integration,
global configuration change, or new dependency. The artifact gates still report missing future
tests; those failures remain work rather than being reclassified as manual verification.

## Documents

- [research.md](research.md): reuse decisions and source-level traps.
- [data-model.md](data-model.md): sources, question state and coverage identities.
- [contracts/containment.md](contracts/containment.md): inputs, kind orientation, semantics,
  findings and reports.
- [quickstart.md](quickstart.md): validation now and after implementation.
- [tasks.md](tasks.md): ordered red/green work with requirement and test mappings.

## Code structure and sequence

1. Extend `crates/ears-sdd/tests/conformance.rs` to retain containment evidence and check exit status.
   Preserve normalization for old cases. Pending cases remain parked until their implementation is
   ready; explicitly run a selected pending fixture during its red/green cycle.
2. Add `crates/ears-sdd/src/contain.rs` for comparison input loading and effect queries. Reuse
   `model::{ModelFile, ModelledRequirement, Component, finitize}`, `guard::Guard`, `bdd::Encoding`,
   and `enumerate::{evaluate, verify_assignment}`. Extract only the reusable parsing/type-checking
   pieces from `analysis.rs` and `vocabulary.rs`; retain invalid-input status rather than treating
   skipped entries as empty. No generic solver or source-provider traits.
3. Add optional configuration and report records in `config.rs` and `report.rs`, orchestrate from
   `lib.rs`, and render references and coverage in the existing human renderer. No new CLI subcommand.
   Include comparison/effect identity wherever containment findings are deduplicated or sorted.
4. Extend shared finitization in `model.rs` only where needed for domain endpoints and safe integer
   arithmetic. Build a separate shared component for each containment question. Do not reuse the
   contradiction decomposition as the containment budget boundary.
5. Expose Kiro graph edges and parsed amendment criteria from `kiro.rs` instead of scraping prose
   again. Resolve live successor closure with cycle detection. Read all external model sidecars
   from the assessor; snapshot both roots in the read-only test.
6. Apply the kind orientation in `contain.rs` after both differences are computed, never before:
   a suppressed direction is still computed so its state count and `status` are truthful. Keep the
   orientation table as one `match` on the kind enum with no configuration path to it.

## Query algorithm

Validate each source independently, including model schema, declared IDs/effects, guard grammar,
domain validity and term replacements. Collect explicit unmodelled requirements and invalid effects.
For each valid effect question:

1. Canonicalize term identities using declared replacement chains. Reject missing identities,
   cycles, or incompatible kinds on affected effects. Keep compatible source-specific domains.
2. Collect dependencies and comparison discontinuities from both sides. Include domain endpoint
   discontinuities from every snapshot, including every successor. Sort variables/values and
   construct per-source domain masks; no state is evaluated yet.
3. Compute the state product with overflow detection. A question over budget is indeterminate;
   continue other affordable questions. Do not build its BDD or enumerate its assignments.
4. Construct one `Encoding`. Within each source, OR contributor guards and apply its domain mask;
   then union the masked source regions on each side. A mask over a union of source domains would
   incorrectly extend one successor's guards into states supported only by another successor.
   Compute candidate-minus-reference and reference-minus-candidate. Empty OR is false.
5. Choose the least semantic assignment for each nonempty difference, by trying values in the
   specified order with BDD satisfiability restrictions. Do not rely on library bit-order witness
   preferences. Re-evaluate guards, masks and the opposite side independently before reporting.
6. Attribute all positive contributors; determine derogation coverage across all widened regions,
   not just that witness. Under the `policy` kind only, check cross-side conflicting-effect pairs
   under their own budgets.
7. Apply the kind's orientation: turn the reported direction(s) into findings, record the
   suppressed direction as counts, and stamp `kind` and `reports` on the comparison record.

Contradictory effects remain active and the existing defect findings remain errors. A conflict
does not make the model's activation regions empty. Partial comparisons keep useful per-effect
results, while their aggregate status records the incompleteness.

## Delivery and evidence

The first usable slice is US1 + US2 + the US5 refusal properties: an ordinary revision comparison
that detects both directions and cannot report malformed or over-budget input as contained. US3
adds explicit decisions, US6 the policy and amendment orientations, then US4 lifecycle sources. Shared code changes
are sequential even where independent fixture authoring can proceed in parallel.

Pending JSON is a test design, not a passing test. The plan gate checks that named files and test
names exist; it does not run their behavior. Keep absent selectors visible until real tests are
written. Move each case into `conformance/cases/` and update traceability only after its red/green
cycle. At completion run the configured `cargo nextest run`, formatting/lint checks, the entire
corpus and the repository-wide final gate. Validation never launches these tests itself.

## Requirement-to-design translation

The following table groups contiguous requirements only where they share one design decision.
Exact selectors for every ID remain in [traceability.toml](traceability.toml).

| Requirements | Design | Verification |
|---|---|---|
| REQ-001, REQ-004 | Explicit comparison manifest and source provenance | `contain-equal`, structured report assertions |
| REQ-002, REQ-003 | Missing-reference result, withheld comparison | `contain-reference-missing` |
| REQ-005 | Union guard and domain-boundary partition | `contain-shared-partition`, differential integer cases |
| REQ-006, REQ-007 | Shared sorted variables, side masks | `contain.rs` ordering and distinction tests |
| REQ-008, REQ-009 | OR of guards per declared effect; unguarded is true within side domains | `contain-equal`, `contain-unguarded` |
| REQ-010, REQ-012, REQ-013 | Forward difference, checked witness and candidate contributors | `contain-widened` |
| REQ-011, REQ-014 | Reverse difference and reference contributors | `contain-narrowed` |
| REQ-015 | Canonical semantic witness ordering | `determinism.rs::containment_witness_is_stable` |
| REQ-016 | Independent positive and negative witness verification | `contain.rs::tests::witness_satisfies_attributed_guards` |
| REQ-017 | Declaration difference independent of activation | `contain-effect-new` |
| REQ-018, REQ-038 | Symmetric cross-side conflicts under `policy` only; per-pair budget | `contain-baseline-conflict`, `contain-policy-upper-bound`, `contain.rs::tests::conflict_questions_only_under_policy` |
| REQ-019 | Candidate contributor to same-effect reference derogation | `contain-widened-declared` and multi-contributor case |
| REQ-020, REQ-021, REQ-022 | Target/reason validation, completed-question staleness | `contain-derogation-unknown`, `contain-derogation-stale`, `contain-derogation-invalid` |
| REQ-023, REQ-024, REQ-025 | State preflight and explicit indeterminate outcome | `contain-budget-exceeded`, evaluation-entry test |
| REQ-026, REQ-027, REQ-028 | Per-source canonical vocabulary and affected-effect refusal | `contain-vocab-drift`, `contain-deprecated-term` |
| REQ-029, REQ-030 | Side/source-qualified coverage and effect counts | `contain-unmodelled`, `contain-equal` |
| REQ-031, REQ-032, REQ-033 | Predecessor versus live-successor union; partial union disclosure | `contain-supersession-gap`, covered variant, `contain-supersession-uncompared` |
| REQ-034, REQ-039 | Explicit extension links, grouped Kiro criteria; narrowing suppressed | `contain-amendment-refines` |
| REQ-035, REQ-036 | Assessor-owned sources and read-only external roots | `contain-assessed-reference`, `read_only.rs::no_gate_modifies_the_project` |
| REQ-037, REQ-040 | Policy orientation; kind and reported directions on every record | `contain-policy-upper-bound`, `contain-equal` |
| REQ-041 | Reference read from manifest path; no version-control resolution | `contain.rs::tests::comparison_inputs_are_explicit` |

## Complexity tracking

No additional dependency or abstraction. The new comparison manifest is necessary to name two
independently sourced models. Git checkout, network resolution, runtime enforcement, envelopes and
reachability stay outside this implementation.

## Implementer context

This section exists for whoever, or whatever, implements the feature without having read the rest
of the repository. It states what the code already provides, what the words above mean in code,
and the rules the repository's own gates enforce on the change.

### Where things are

| Path | What it holds | What this feature does with it |
|---|---|---|
| `crates/ears-sdd/src/guard.rs` | `Guard` AST (`Always`, `Term`, `Compare{term,op,value}`, `Not`, `And`, `Or`), `guard::parse`, `Guard::terms()` | Reuse unchanged. Activation regions are `Guard::Or` over contributors. |
| `crates/ears-sdd/src/model.rs` | `ModelFile` (TOML: `[effects]`, `[requirements.REQ-NNN] when/then`), `EffectDeclaration`, `RequirementEntry`, `ModelledRequirement`, `Component`, `finitize`, `split_points`, `regions_cover_domain` | Add optional `[derogations.<id>]` table. Add domain-endpoint cut points and checked `i64` arithmetic. |
| `crates/ears-sdd/src/bdd.rs` | `Encoding` over one sorted variable set; `encode`, `domain` | Build one `Encoding` per effect question; compute the two differences. |
| `crates/ears-sdd/src/enumerate.rs` | Brute-force `evaluate` and `verify_assignment` | Independent check of every witness before it is reported. |
| `crates/ears-sdd/src/analysis.rs` | Existing contradiction analysis; `conflict_pairs`, guard type checks, `decompose`, `validate_merged` | Extract the parsing and type-check pieces to crate visibility. Do not reuse `decompose` as the containment component boundary. |
| `crates/ears-sdd/src/vocabulary.rs` | `Term` with `deprecated` and `replaced_by` | Add a guard canonicalizer that follows replacement chains and rejects cycles. |
| `crates/ears-sdd/src/kiro.rs` | `list`, `evaluate`, supersession links | Expose successor edges and parsed amendment `extends` links instead of re-scraping prose. |
| `crates/ears-sdd/src/config.rs`, `report.rs`, `lib.rs` | `[checks]` config, `Report`/`Finding` (with `detail`), orchestration and human rendering | Add `checks.containment`, the `comparisons` array, `CONTAIN_*` findings, and human output. |
| `crates/ears-sdd/src/contain.rs` (new) | Manifest loading, snapshot validation, effect questions, orientation, derogations | Everything above is composed here. |
| `crates/ears-sdd/tests/conformance.rs` | Corpus runner over `conformance/cases/*` (`cmd.json`, `expected.json`, `project/`) | Extend `normalize` to keep containment `detail` and `comparisons`; assert exit status. |
| `conformance/pending/contain-*` | Fixtures written from the rule before the code exists | Move each into `cases/` only after its red/green cycle. |

### Vocabulary of this feature, in code terms

- **Source**: one `(spec path, model path, vocabulary path, optional external root)` tuple from the
  manifest. A comparison has one reference source and one or more candidate sources.
- **Snapshot**: a source after loading and validation: parsed requirements, declared effects,
  canonical terms, the set of requirement IDs with no model entry, and the set of effects whose
  guards failed validation. An invalid effect is not an empty region; it makes the question
  `uncompared`.
- **Activation region** for effect `e` on one side: within each source, `Or` of the guards of
  requirements whose `then` is `e`, `And` that source's domain mask; then `Or` across sources.
  No contributors means `false`.
- **Question**: one effect (or, under `policy`, one conflicting pair), the sorted union of terms its
  guards mention on both sides, the shared finitized values for those terms, and a state count
  computed by multiplication before anything is encoded. Over budget means nothing is encoded.
- **Difference**: `candidate AND NOT reference` and `reference AND NOT candidate`, each a BDD.
- **Witness**: the least satisfying assignment under the documented value order, chosen by
  restricting the BDD value by value in that order, then confirmed with `enumerate` on both the
  positive guards and the absence of the opposite side.
- **Orientation**: a `match` on `Kind` returning which differences become findings and whether
  conflict questions are asked. Fixed in code. Stamped on the result as `kind` and `reports`.

### Rules the repository enforces on the change

- Production code under `crates/` contains no `REQ-NNN` identifiers and no copied requirement
  prose; the separation gate fails the build otherwise. Tests and fixtures are exempt. If a
  comment has to quote an identifier, the marker `ears-sdd:allow-requirement-id` on the line
  before is the documented escape and `tasks.rs` shows its use.
- No new crate dependency. No subprocess, no Git, no network during `validate` or `status`.
- Nothing in the assessing or assessed project is written. `tests/read_only.rs` snapshots the tree
  and fails on any change.
- Determinism is a hard requirement: the corpus states expected results as data, so the witness for
  an unchanged pair of models is identical across runs and across declaration order.
- A budget is a state count, never a time. Exceeding it is a finding, not a retry.
- A check that did not run is reported as a gap. Never fold `uncompared` or `indeterminate` into
  `ok`.
- Every finding named in this plan already has a row in `contracts/containment.md` with its
  severity. Do not invent codes; add a row first if one is missing.

### How to work a task

1. Read the task's selectors in `traceability.toml`. Each names a test or fixture that does not yet
   exist or does not yet pass.
2. Write the fixture or test first. For a corpus case, `expected.json` is written from the rule in
   the spec, not from what the binary prints. `cmd.json` carries a one-line `note` saying what the
   case demonstrates.
3. Run the case red: point the CLI at `conformance/pending/<case>/project` with the `args` from
   `cmd.json` and confirm the divergence is the missing behavior, not a malformed fixture.
4. Implement, run green, then move the directory to `conformance/cases/` and repoint the selector.
5. Before checking a task off: `cargo fmt --all -- --check`, `cargo clippy --workspace
   --all-targets -- -D warnings`, `cargo nextest run`, and
   `cargo run -q -p ears-sdd -- validate --phase tasks --feature specs/009-baseline-containment`.

### Reading the fixtures

`conformance/pending/contain-widened/` is the smallest complete example: a vocabulary with two
boolean terms, a `model.toml` whose one guard is `maintenance-window`, a `reference.toml` whose
guard is `maintenance-window and operator-present`, and an `expected.json` with one
`CONTAIN_WIDENED` on `REQ-001`. The witness is `maintenance-window = true, operator-present =
false`. `contain-policy-upper-bound/` is the same shape with `kind = "policy"`, a policy kept under
`.specify/policy/`, one effect narrower than the policy (no finding) and one wider (a finding).
The seven older fixtures predate the manifest contract and are upgraded by their story's first
task; the policy fixture follows the contract as written.
