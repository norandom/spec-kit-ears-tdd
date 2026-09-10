# Quickstart: Kiro project assessment

Prove the feature against the conformance fixtures, not against a live sibling repo.

## Prerequisites

- `ears-sdd` built from this workspace (`cargo build -p ears-sdd`).
- Conformance fixtures under `conformance/cases/kiro-*/`.

## Happy path: live Kiro tree, no Spec Kit config

```text
ears-sdd validate --project conformance/cases/kiro-discovers-tree/project --phase spec
```

Expected:

- Gate PASS.
- Scope names `.kiro/specs/*/requirements.md`.
- `Disabled: vocabulary, constraints (not checked)`.
- `KIRO_CHECKS_OMITTED` advisory.
- Every feature directory with a requirements document listed.
- No file under `project/` changed.

## Baseline: superseded, reserved, initialized

```text
ears-sdd validate --project conformance/cases/kiro-active-baseline/project --phase spec
```

Expected:

- Live and no-phase specifications included.
- `superseded`, `reserved`, `initialized` excluded and named.
- Human line reports included and excluded counts.

## Supersession defects

```text
ears-sdd validate --project conformance/cases/kiro-supersession-dangling/project --phase spec
ears-sdd validate --project conformance/cases/kiro-supersession-unreciprocated/project --phase spec
```

Expected: FAIL, with `KIRO_SUPERSESSION_DANGLING` and `KIRO_SUPERSESSION_UNRECIPROCATED` respectively.

## Recut (valid replacement)

```text
ears-sdd validate --project conformance/cases/kiro-supersession-recut/project --phase spec
```

Expected: PASS. One superseded predecessor naming two live successors produces no dangling finding.

## Spec Kit regression

```text
ears-sdd validate --project conformance/cases/all-features-scope/project --phase spec --all
```

Expected: unchanged from today’s stored `expected.json`.

## Optional manual smoke (not a gate)

If `../facdrone` is present and must not be modified:

```text
ears-sdd validate --project ../facdrone --phase spec
```

Expected: discovers the `.kiro/specs` tree, excludes `phase: superseded|reserved|initialized`, reports constraint analysis as not checked, exits without writing. Do not treat this run as CI evidence.
