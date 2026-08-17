# A needle in 383 requirements

The [worked example](../example.md) uses two specifications, small enough to hold in your head. The
objection to it is fair: of course a tool finds a contradiction between two rules you can read side
by side.

This one is twelve specifications and 383 requirements. Every specification passes on its own. One
pair contradicts, and the pair sits five features apart and shares no vocabulary at the surface.

The example is in the repository at `examples/package-registry`. You can run every command below.

## The project

A package registry, specified feature by feature the way Spec Kit produces them:

```text
examples/package-registry/
├── .specify/
│   ├── ears-sdd.toml
│   ├── vocabulary.toml      28 terms, written before the specifications
│   └── intentions.toml      8 intentions, 5 precedence pairs
└── specs/
    ├── 001-artifact-publication/     31 requirements
    ├── 002-artifact-verification/    32 requirements
    ├── 003-access-control/           32 requirements
    ...
    └── 012-maintenance-operations/   32 requirements
```

The vocabulary was written first, and that ordering is the point. Twelve features cannot agree on
what a word means by each picking a good one. Terms are fixed up front, every requirement draws its
tags from that closed list, and an undeclared tag fails the gate.

## Every specification passes

```console
$ ears-sdd validate --phase spec --all
EARS/TDD spec gate: PASS
Scope: specs/*/spec.md (all matching specifications)
Features: 12  Requirements: 383  Errors: 0  Warnings: 0
```

383 requirements, all in EARS form, all grounded in declared terms, all mapped to a verification.
Nothing here is sloppy, and nothing here is wrong.

Checked one feature at a time, the answer is the same:

```console
$ ears-sdd validate --phase plan --feature specs/002-artifact-verification
EARS/TDD plan gate: PASS

$ ears-sdd validate --phase plan --feature specs/007-supply-chain-hardening
EARS/TDD plan gate: PASS
```

This is the state a project is in when everyone has done their job.

## Merged, one thing is wrong

```console
$ ears-sdd validate --phase plan --all
EARS/TDD plan gate: FAIL
Scope: specs/*/spec.md (all matching specifications)
Features: 12  Requirements: 383  Errors: 1  Warnings: 0

- MERGE_CONFLICT_UNADJUDICATED [specs/002-artifact-verification:REQ-007]
  REQ-007 and REQ-002 contradict each other and nothing says which wins.
  Declare precedence between `collision-resistant-integrity` and `installable-existing-releases`.
```

One error out of 383 requirements, and it took 336 milliseconds.

## The two requirements

From feature 002, written when SHA-1 was still what the mirrors served:

> **REQ-007**: Where legacy mirror support is enabled, the registry shall accept an artifact whose
> sha1 digest comparison, signature check, and publisher authentication all succeed.

From feature 007, written later by people who had read the SHA-1 collision work:

> **REQ-002**: If the manifest records a published artifact's digest under md5 or sha1, then the
> registry shall reject that publication.

Neither is wrong. Neither mentions the other. Feature 002 is internally consistent: it rejects SHA-1
when legacy mirror support is off, and accepts it when the operator has opted in. Feature 007 is
internally consistent too.

They are five features apart in a directory listing, and nobody reviews feature 002 and feature 007
in the same sitting.

## The witness

```json
{
  "witness": "digest-algorithm = 'sha1', digest-matches = true, legacy-mirror-enabled = true,
              publisher-authenticated = true, signature-present = true, signature-valid = true",
  "specifications": ["specs/002-artifact-verification", "specs/007-supply-chain-hardening"],
  "declare_precedence_between": [
    { "a": "collision-resistant-integrity", "b": "installable-existing-releases" }
  ]
}
```

Read it as a sentence: a properly signed artifact, from an authenticated publisher, whose SHA-1
digest matches, arriving at an installation that has legacy mirror support switched on.

Feature 002 says accept it. Feature 007 says reject it. Both apply. That state is not exotic, it is
the ordinary state of an installation midway through the migration.

## Why the tool finds it and a search does not

The two guards share no term.

```toml
# 002-artifact-verification
when = "legacy-mirror-enabled and digest-algorithm == 'sha1' and digest-matches and ..."

# 007-supply-chain-hardening
when = "digest-algorithm == 'md5' or digest-algorithm == 'sha1'"
```

They overlap only on `digest-algorithm`, and they use it to say opposite things about the same
value. Grepping for `sha1` finds both, but it also finds every other requirement mentioning a digest
algorithm, and it cannot tell you which pairs can hold at once.

The contradiction is not in the text. It exists only in the states where both guards hold, and
finding it means asking whether such a state exists.

!!! note "This needed a fix to the tool"

    Until version 0.3.0 this contradiction was not found. Requirements were grouped for analysis by
    the terms their guards shared, and analysing a group meant enumerating its whole state space.
    Merged across twelve features that came to 100,663,296 states, over the budget, so the merge was
    refused and reported nothing conclusive.

    The mistake was conflating grouping with deciding. Every question the analysis asks is at most
    pairwise, and the pair above ranges over six terms, not twenty-two. Across all 765
    conflicting-effect pairs in this example the total is 12,324 states. The explosion was
    manufactured by materialising a product nobody had asked for.

## Resolving it

The report names the decision that is missing, not the answer. There are three honest resolutions
and the tool refuses to choose between them.

**Decide which goal wins.** If security wins outright, say so, and accept that legacy mirrors break:

```toml
[[precedence]]
over = "collision-resistant-integrity"
under = "installable-existing-releases"
reason = "A forgeable digest is not an integrity control. Legacy mirrors must be re-signed."
```

The gate then passes with an advisory recording the decision and its reason, and feature 002's
REQ-007 becomes a requirement someone has knowingly overruled rather than one nobody noticed.

**Scope the carve-out.** Narrow REQ-007 so the overlap disappears, for instance to artifacts
published before a cutoff. The [migration section](../example.md#resolving-it) walks through this,
including what happens when the scoping is applied to only one side.

**Delete the carve-out.** If the migration is finished, REQ-007 is describing a state of the world
that no longer exists.

What you cannot do is leave it. The gate fails until someone decides, which is the whole point: this
is a decision about competing goals, and it was previously being made by whichever code path
happened to run first.

## What is deliberately not modelled

Ten of the twelve features declare no constraint model, and the report says so:

```text
- MERGE_UNMERGED  Specification declares no constraint model and was excluded from the merge.
```

That is honest rather than tidy. Modelling is incremental: you model the axis where being wrong is
expensive, and a requirement with no model entry is reported as unmodelled rather than assumed
consistent. Two features carry models here because the digest policy is where this registry can be
silently wrong.

## Next

[The brownfield example](brownfield-duplicate-storage.md) starts from an existing system with no
specifications at all, and the contradiction it finds turns out to be a schema decision made in
2016.
