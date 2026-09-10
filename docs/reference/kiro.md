# Assess a Kiro project

A Kiro tree is not a Spec Kit project. Specifications live under `.kiro/specs/<feature>/`,
requirements are written in `requirements.md`, and lifecycle is recorded in `spec.json`. Pointing
the validator at that tree used to look for `specs/*/spec.md`, find none, and either error or
report an empty success.

From 1.0.0 the same binary discovers the Kiro tree, parses the requirement grammars those documents
actually use, and computes an **active baseline**. It does not write into the assessed project. It
does not search Kiro prose for logical contradictions.

```sh
ears-sdd validate --project /path/to/kiro-project --phase spec
```

```text
EARS/TDD spec gate: PASS
Scope: .kiro/specs/*/requirements.md (all matching specifications)
Disabled: vocabulary, constraints (not checked)
Baseline: 19 included, 5 excluded (initialized: …; reserved: …; superseded: …)
Features: 24  Requirements: 1524  Errors: 0  Warnings: 0
- KIRO_CHECKS_OMITTED .kiro/specs: Spec Kit identifier, verification-mapping, task-coverage, and
  separation checks were not applied to Kiro specifications.
```

A Spec Kit project without a Kiro tree is unchanged. Mixed trees are discovered under both layouts;
Spec Kit checks still run on `spec.md`, not on Kiro documents.

## What PASS means

The tree was read, every feature directory was examined, and every superseded specification has a
live successor. It is not a claim that the requirements are jointly satisfiable. Vocabulary and
constraint analysis are off until someone authors models, and the run says so.

`KIRO_CHECKS_OMITTED` is advisory. Kiro documents do not use `REQ-NNN` or `traceability.toml`, so
those checks would fail every file for lacking artifacts they never had.

## The active baseline

| `spec.json` phase | In force? |
| --- | --- |
| `superseded` | No. Needs a live successor. |
| `reserved` | No. Empty slot; no successor required. |
| `initialized` | No. No successor required. |
| anything else, or no `spec.json` | Yes. |

A native Kiro specification with no phase metadata is in force. Unreadable `spec.json` is an error
and is excluded until it can be read.

## Supersession is a link

Replacement is not a comment. The checker builds a directed link from a live successor to a
superseded predecessor:

- Phase notes on a superseded specification that mention a discovered feature directory name.
- Spec-level claims in a live specification's requirements prose **outside** acceptance criteria
  (`supersedes`, `superseded`, `superseding`) that name a discovered feature.

Two findings, both errors:

| Code | Meaning |
| --- | --- |
| `KIRO_SUPERSESSION_DANGLING` | Superseded, and nothing in the active baseline replaces it. |
| `KIRO_SUPERSESSION_UNRECIPROCATED` | A live spec claims to replace another spec that is still live. |

A recut is valid: one predecessor naming several live successors. An amendment that `extends` a
baseline requirement in the same file is a refinement, not a replacement. The word "supersede"
inside an acceptance criterion (replacing a stored run, for example) is product behaviour, not a
spec retirement.

## What this does not do

- It does not convert the project to Spec Kit, and it does not write `.specify/` into it.
- It does not lint Kiro acceptance criteria as Spec Kit EARS (`REQ-NNN`, If-then, one `shall`).
- It does not find contradictions in English. That remains the constraint-model merge, aimed later
  at the active baseline once models exist.
- It does not treat `tasks-generated` versus actually delivered as a defect. Phase-drift is out of
  this check.

## `--feature`

```sh
ears-sdd validate --project /path/to/kiro-project --phase spec --feature facdrone-ledger-core
ears-sdd validate --project /path/to/kiro-project --phase spec --feature .kiro/specs/facdrone-ledger-core
```

A basename matches exactly one Kiro feature directory. A path that resolves outside the project is
still refused.

## Findings

The `KIRO_` codes are listed under [findings](findings.md#kiro_-cc-sdd-trees). Scope and baseline
reporting are part of [gates](gates.md#scope-is-part-of-the-claim).
