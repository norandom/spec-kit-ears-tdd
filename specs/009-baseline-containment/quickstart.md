# Validate Baseline Containment

Run commands from the repository root. Rust and Cargo are required; `cargo nextest` is the configured
project test command. `cargo test -p ears-sdd` also runs the existing test targets.

## Current planning checks

```sh
cargo run -q -p ears-sdd -- validate --phase spec --all
cargo run -q -p ears-sdd -- validate --phase plan --feature specs/009-baseline-containment
cargo run -q -p ears-sdd -- validate --phase tasks --feature specs/009-baseline-containment
cargo test -p ears-sdd
```

The spec check can pass now. The plan/tasks checks intentionally expose absent automated-test
selectors until their tasks are implemented; `SPEC_SCOPE` additionally records the selected feature.
Do not disable `require_test_files`, remove missing mappings, or substitute manual verification to
make these checks pass. The existing Cargo suite does not execute `conformance/pending/`.

Results recorded on 2026-09-18 (before the 2026-09-22 amendment that moved the envelope story to
`010-envelope-containment` and added the policy and amendment orientation; see the end of this
file for the re-run):

- Repository-wide spec gate: passed, 9 features / 200 requirements, no findings.
- Feature plan/tasks gates: 28 `TRACE_TEST_FILE` errors covering 20 absent files (18 pending
  expectation files and two Rust test files). All 50 requirements have task coverage. The narrowed
  scope warning and six unused-vocabulary advisories are also visible.
- `cargo test -p ears-sdd`: 100 passed (92 unit, 2 corpus, 5 differential, 1 read-only); no failures.
- Artifact checks: requirement IDs and traceability entries match, task IDs are sequential,
  all 30 implementation tasks remain unchecked, and local documentation links resolve.

These results validate the specification and existing behavior. Containment is not implemented;
none of the seven pending cases has been activated or counted as a passing behavior test.

## Producing a reference snapshot

The validator never resolves a revision itself. When a revision is approved, the project copies the
approved model and the specification it was read against to the reference paths the manifest names,
and commits them with the approval:

```sh
cp specs/001-alpha/model.toml specs/001-alpha/reference.toml
cp specs/001-alpha/spec.md    specs/001-alpha/reference.md
git add specs/001-alpha/reference.toml specs/001-alpha/reference.md
```

A policy is authored once, at a path outside `spec_glob` such as `.specify/policy/`, and named by a
`kind = "policy"` comparison. It is edited like any specification and its edits are reviewed like
any other; the containment check does not gate the policy against anything.

## After the comparison engine is implemented

Author the explicit manifest described in [contracts/containment.md](contracts/containment.md),
then use an activated fixture:

```sh
cargo run -q -p ears-sdd -- validate --project conformance/cases/contain-equal/project --phase plan --all --json
cargo run -q -p ears-sdd -- validate --project conformance/cases/contain-widened/project --phase plan --all --json
cargo run -q -p ears-sdd -- validate --project conformance/cases/contain-narrowed/project --phase plan --all --json
```

| Scenario | Check the report for |
|---|---|
| Equal models | Named sources, `equal`, complete coverage, no direction finding |
| Guard loses a conjunct | `CONTAIN_WIDENED`, witness and candidate contributors, failed gate |
| Guard gains a conjunct | `CONTAIN_NARROWED`, reference contributors; visible warning |
| Valid derogation | Same widening stays visible as declared; invalid or stale exceptions are named |
| Missing reference / vocabulary drift | Withheld comparison, no whole-model containment claim |
| Budget exceeded | State count and budget, `indeterminate`, no evaluation of that question |
| Complete recut / dropped obligation | Covered predecessor or `CONTAIN_SUPERSESSION_GAP` |
| External assessment | Assessor model sources, read-only comparison of both trees |
| Policy, candidate narrower | Status `narrowed`, `reports = ["widening"]`, no finding, gate passes |
| Policy, candidate wider | `CONTAIN_WIDENED` with witness and candidate requirement, gate fails |
| Amendment narrower than its baseline | No finding; wider produces `CONTAIN_WIDENED` |

To execute a red test before moving a case, point the same command at its `conformance/pending/`
directory and compare the output with `expected.json`. A failure must be the intended missing
behavior, not a malformed fixture or unrelated traceability error. The corpus runner's new evidence
assertions, differential tests, and read-only test provide the automated checks on activation.

## Completion

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run
cargo run -q -p ears-sdd -- validate --phase final --all
```

Every case has moved to the active corpus, every selector points to real executed evidence, and
every implementation task is checked only after its tests pass. The final gate checks traceability
and separation; a successful final gate alone is not a substitute for running the test suite.

## Re-run after the 2026-09-22 amendment

- Repository-wide spec gate: passed, 10 features / 214 requirements, no findings.
- Feature tasks gate for 009: 25 `TRACE_TEST_FILE` errors (absent pending cases and Rust tests
  named by the 41 requirements), no `TASK_UNCOVERED`, no `TASK_UNKNOWN_REF`. The narrowed-scope
  warning and the six unused-vocabulary advisories are unchanged.
- Feature tasks gate for 010: 18 `TRACE_TEST_FILE` errors, no task-coverage findings.
- `cargo test -p ears-sdd`: 100 passed (92 unit, 2 corpus, 5 differential, 1 read-only); no
  failures. No production code changed.
- Eight cases are parked under `conformance/pending/`: seven for 009 including the new
  `contain-policy-upper-bound`, one (`contain-envelope-widened`) for 010.

## Evidence inventory (T001, 2026-09-23)

The tasks gate reports 25 `TRACE_TEST_FILE` findings over 16 absent selectors. They split into two
kinds, and the difference decides when each can be written.

**Fixtures that can be written ahead (14).** Each is data under `conformance/pending/` and needs no
code to exist: `contain-reference-missing`, `contain-shared-partition`, `contain-deprecated-term`,
`contain-budget-exceeded`, `contain-unmodelled`, `contain-unguarded`, `contain-effect-new`,
`contain-baseline-conflict`, `contain-derogation-unknown`, `contain-derogation-invalid`,
`contain-supersession-gap`, `contain-supersession-uncompared`, `contain-amendment-refines`,
`contain-assessed-reference`. Each story's first task writes its own.

**Rust test modules that cannot (2).** `crates/ears-sdd/src/contain.rs` and
`crates/ears-sdd/tests/determinism.rs` hold unit tests whose bodies call the comparison API. A test
that calls a function that does not exist does not compile, and a non-compiling test file breaks
the whole suite rather than failing one test. These files appear with their first test in the
task that introduces the API (T004 for `contain.rs`, T011 for `determinism.rs`). The test is still
written before the implementation it checks, within that task.

The implement command's preamble asks for a passing tasks gate before implementation. For this
feature that condition cannot hold before T004, for the reason above, and the plan says so
explicitly. Selectors stay absent and visible until then; they are not removed or downgraded to
manual verification.

T002 and T003 are complete. The corpus runner now keeps `detail` on `CONTAIN_*` findings and the
top-level `comparisons` array, sorts containment findings by comparison and effect as well, and
leaves every existing case's normalized shape unchanged. Two new tests in
`crates/ears-sdd/tests/conformance.rs` hold it to that. `containment_evidence_is_not_discarded`
was run red first, with evidence retention switched off, and failed on the first mutation (the
witness). `reports_without_containment_keep_their_normalized_shape` guards the old shape. Exit
status was already checked by the runner and is now covered by the same test.

Results: `cargo fmt --check` clean, `cargo clippy -D warnings` clean, `cargo test -p ears-sdd` 102
passed (92 unit, 4 corpus runner, 5 differential, 1 read-only), 0 failed. `cargo nextest` is not
installed on this machine, so the configured test command was not run; `cargo test` runs the same
targets.
