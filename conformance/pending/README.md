# Pending corpus

Behavioural cases for a feature that is specified but not implemented. Same shape as
`conformance/cases/`: a project fixture, the invocation, and the expected result.

Nothing runs these. `crates/ears-sdd/tests/conformance.rs` reads `conformance/cases/` only, so a
case parked here neither passes nor fails. That is the whole reason the directory exists: the
corpus under `cases/` is a suite that has to be green, and a case written from a rule the binary
does not yet implement would hold it red for the length of the feature.

A case here is still evidence in one sense. `specs/009-baseline-containment/traceability.toml`
names these paths, so the `plan` gate confirms the file exists and the requirement has somewhere to
point. It does not confirm the case passes, and nothing here should be read as if it did.

## Moving a case into the corpus

When the feature it belongs to is implemented, move the directory into `conformance/cases/` and
repoint the selectors in that feature's `traceability.toml`. Then run the corpus. A case that was
written from the rule and diverges on arrival is doing its job: either the implementation is wrong,
or the expected result was, and the divergence is where that argument happens.

Expect the `summary` block to be the part that needs reconciling. Findings can be written from the
rule with confidence; counts emitted by a subsystem that does not exist yet cannot be.

## What is parked here now

`contain-*` — seven cases for `specs/009-baseline-containment`: containment of a revision against
its reference (equal, widened, narrowed), declared and stale derogations, vocabulary drift, and
`contain-policy-upper-bound`, a `kind = "policy"` comparison whose candidate is narrower on one
effect and wider on another. Further cases named in that feature's traceability are not written at
all; those are reported as missing files rather than parked here, because an unwritten case is not
a case.

`contain-envelope-widened` belongs to `specs/010-envelope-containment`, which owns the agent
operating envelope after the 2026-09-22 split. Its selectors live in that feature's traceability.

The design and test-first work breakdown live in each feature's `plan.md` and `tasks.md`.

The six older fixtures predate the input/report contracts. Their upgrade tasks add explicit
comparison manifests, reference vocabularies, schema versions, witness details and comparison
provenance, and correct the effect counts to include declared-but-unasserted effects.
`contain-policy-upper-bound` was written to the contract and shows the target shape, including a
`comparisons` record and a `detail` block on the finding; the runner currently drops both, which is
what task T002/T003 of feature 009 changes. A passing file-existence check does not establish any
of these properties. Keep every case parked until its red/green implementation cycle is complete.
