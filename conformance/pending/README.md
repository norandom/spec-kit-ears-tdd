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

`contain-envelope-widened` belongs to `specs/010-envelope-containment`, which owns the agent
operating envelope after the 2026-09-22 split. `contain-supersession-uncompared` and the other
unwritten 009/010 cases named in their traceability are parked advisories (`TRACE_TEST_PARKED`),
not errors: an unwritten case is not a case.

The implemented 009 engine (all four kinds, derogations, witnesses, budgets, drift) moved to
`conformance/cases/` as 20 `contain-*` cases. Keep every case parked here until its red/green
implementation cycle is complete.
