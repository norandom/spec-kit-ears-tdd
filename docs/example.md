# A contradiction, end to end

This is a complete run against a two feature project. Every block of output below was copied from a
terminal, not written by hand. You can rebuild the project from the files on this page in about five
minutes.

The example is small enough to hold in your head and shaped exactly like the real case: two
features, written at different times, by different people or different agent sessions, that never
appear in the same review.

## The situation

A managed workstation has a feature for exploit mitigation and a feature for native development.

Feature 001 says that when the arbitrary code guard mitigation is enforced, the workstation blocks
dynamic code generation. That is what the mitigation is for.

Feature 002 says that when the native toolchain is building, the workstation permits dynamic code
generation. That is what a just in time compiler backend needs to exist.

Neither author was wrong. Nobody reviewed both.

## The specifications

```markdown title="specs/001-exploit-protection/spec.md"
# Feature Specification: Exploit Protection

## Requirements

- **REQ-001**: When the arbitrary-code-guard mitigation is enforced, the workstation shall block
  dynamic code generation for the process.
- **REQ-002**: The workstation shall record every mitigation change in the audit log.
```

```markdown title="specs/002-native-development/spec.md"
# Feature Specification: Native Development

## Requirements

- **REQ-001**: When the native toolchain is building, the workstation shall permit dynamic code
  generation for the compiler process.
```

Both are valid EARS. Each has one trigger, one subject, one `shall`.

## The vocabulary

The two features have to agree on what their words mean before anything can compare them. Terms are
declared once, for the project, with a definition and a domain.

```toml title=".specify/vocabulary.toml"
schema_version = "1.0"

[terms.mitigation-enforced]
label = "Mitigation enforced"
definition = "Whether the arbitrary-code-guard mitigation is enforced for a process."
domain = { kind = "bool" }

[terms.toolchain-building]
label = "Toolchain building"
definition = "Whether a native toolchain is compiling or running a just-in-time backend."
domain = { kind = "bool" }
```

This is the part that looks like paperwork and is not. Without it, feature 001 says `mitigation` and
feature 002 says `code guard`, and no tool can tell that they are the same condition. The check
would run, find nothing, and report a pass.

## The intentions

Each requirement records the one goal it serves. This is not documentation. It is what lets the tool
tell a mistake apart from a trade-off later.

```toml title=".specify/intentions.toml"
schema_version = "1.0"

[intentions.reduce-attack-surface]
statement = "Untrusted code cannot be generated or executed at runtime."
rationale = """
Arbitrary code guard is the mitigation that stops an exploit turning a read primitive into
execution. Disabling it per-process is the documented escape hatch, and every escape hatch that is
not written down becomes permanent.
"""

[intentions.native-toolchain-works]
statement = "Developers can build and debug native code on the workstation."
rationale = """
A workstation nobody can compile on gets replaced by an unmanaged one. The mitigation is worth
having only if the machine stays usable.
"""
```

Each requirement points at its terms and its intention in the traceability file it already has:

```toml title="specs/001-exploit-protection/traceability.toml"
[requirements.REQ-001]
verification = "automated"
tests = ["tests/Test-ExploitProtection.ps1::BlocksDynamicCode"]
tags = ["exploit-protection", "mitigation-enforced"]
intent = "reduce-attack-surface"
```

## The models

A model says what a requirement asserts and when. The guard language is small on purpose: boolean
connectives, comparison against literal constants, nothing else.

```toml title="specs/001-exploit-protection/model.toml"
schema_version = "1.0"

[effects]
block_dynamic_code = { conflicts_with = ["permit_dynamic_code"] }
permit_dynamic_code = {}
emit_audit_entry = {}

[requirements.REQ-001]
when = "mitigation-enforced"
then = "block_dynamic_code"

[requirements.REQ-002]
then = "emit_audit_entry"
```

```toml title="specs/002-native-development/model.toml"
schema_version = "1.0"

[effects]
permit_dynamic_code = {}

[requirements.REQ-001]
when = "toolchain-building"
then = "permit_dynamic_code"
```

One line does the work: `block_dynamic_code` conflicts with `permit_dynamic_code`. That is a
statement about the world, made once, by whoever knew it.

## Each specification passes alone

```console
$ ears-sdd validate --phase plan --feature specs/001-exploit-protection
EARS/TDD plan gate: PASS
Scope: specs/001-exploit-protection (from --feature)

$ ears-sdd validate --phase plan --feature specs/002-native-development
EARS/TDD plan gate: PASS
Scope: specs/002-native-development (from --feature)
```

Correctly so. There is no contradiction inside either feature. Any review that reads one
specification at a time reaches the same verdict, and reaches it honestly.

## Together they do not

```console
$ ears-sdd validate --phase plan --all
EARS/TDD plan gate: FAIL
Scope: specs/*/spec.md (all matching specifications)
Features: 2  Requirements: 3  Errors: 1  Warnings: 0
- MERGE_CONFLICT_UNADJUDICATED [specs/001-exploit-protection:REQ-001] .specify/ears-sdd.toml:
  REQ-001 and REQ-001 contradict each other and nothing says which wins.
  Declare precedence between `native-toolchain-works` and `reduce-attack-surface`.
```

The machine readable form carries the part that makes it actionable:

```json
{
  "code": "MERGE_CONFLICT_UNADJUDICATED",
  "detail": {
    "specifications": ["specs/001-exploit-protection", "specs/002-native-development"],
    "shared_terms": ["mitigation-enforced", "toolchain-building"],
    "witness": "mitigation-enforced = true, toolchain-building = true",
    "declare_precedence_between": [
      { "a": "native-toolchain-works", "b": "reduce-attack-surface" }
    ]
  }
}
```

The `witness` is the whole argument. It is a concrete state of the world in which both requirements
fire: the mitigation is enforced and the toolchain is building at the same time. That is a developer
laptop on a hardened image, which is to say a Tuesday.

You cannot argue with a witness. It is not a heuristic or a warning that something looks suspicious.
It is an assignment you can check by hand in ten seconds, and either it is reachable in your system
or one of the two guards is wrong.

## Resolving it

There are exactly two honest resolutions, and the tool refuses to pick for you.

### Fix a requirement

If the state is reachable and you did not mean it, one requirement is wrong. Narrow a guard. Feature
001 might exclude processes on the toolchain allow list, which is what the real mitigation does
through per program policy.

### Record the decision

If both requirements are right and the tension is real, say which goal wins:

```toml title=".specify/intentions.toml"
[[precedence]]
over = "reduce-attack-surface"
under = "native-toolchain-works"
reason = "A weakened mitigation is worse than a documented per-program exclusion."
```

```console
$ ears-sdd validate --phase plan --all
EARS/TDD plan gate: PASS
Scope: specs/*/spec.md (all matching specifications)
Features: 2  Requirements: 3  Errors: 0  Warnings: 0
- MERGE_CONFLICT_ADJUDICATED [specs/001-exploit-protection:REQ-001] .specify/ears-sdd.toml:
  REQ-001 and REQ-001 contradict each other; `reduce-attack-surface` takes precedence,
  so this is a recorded decision rather than a defect.
```

The gate passes, and the conflict does not disappear. It is now an advisory that names the decision
and the reason, in a file that is reviewed and versioned. The next person to read feature 002 learns
that its behaviour is bounded by a deliberate choice, instead of discovering it from a broken build.

That distinction is the reason intentions exist. Without them the tool can only say that two
requirements clash. With them it can say whether a human already decided, and refuse to accept a
decision nobody made.

### The case precedence cannot fix

If both requirements serve the same intention, no precedence resolves anything, because a goal
cannot outrank itself. The tool says so directly and calls it a defect:

```text
MODEL_CONFLICT_DEFECT: REQ-004 and REQ-011 contradict each other and both serve
`reduce-attack-surface`. No precedence can adjudicate this: one goal cannot outrank itself,
so one of the two rules is wrong.
```

## Why review would not have caught this

The two requirements sit in different files, written at different times, and they share no word. One
says `mitigation-enforced`, the other says `toolchain-building`. Searching for one never finds the
other.

There is no sentence to notice. The contradiction is not in the text. It exists only in the states
where both guards hold, and finding it means asking whether such a state exists at all.

That question has a mechanical answer, which is the argument for asking a tool.

!!! note "This exact case was a bug in this tool"

    Until August 2026, `ears-sdd` decomposed requirements into independent groups by shared guard
    terms alone, then checked each group. These two share no term, so they landed in different
    groups, each satisfiable, and the merged run reported no findings.

    The fix was to keep requirements whose effects conflict in the same group regardless of their
    guards, and to make the internal soundness contract reject any partition that separates such a
    pair. Both are covered by the conformance corpus, in `merge-disjoint-terms-conflict`.

    It is worth stating plainly because it is the failure mode this whole project is about: a check
    that returns clean is indistinguishable from a system that is clean, and the only defence is to
    keep asking what the check would have missed.

## At scale

A real project using this layer, a managed Windows workstation with 12 features:

```console
$ ears-sdd validate --phase spec --all
EARS/TDD spec gate: PASS
Scope: specs/*/spec.md (all matching specifications)
Features: 12  Requirements: 397  Errors: 0  Warnings: 0
```

397 requirements is past the point where anyone holds the set in their head. The pairwise
comparisons are not the interesting number. The interesting number is that nobody has to do them.

## Next

- [Finding contradictions](concepts/contradictions.md) explains what the tool does with these models
  and why the answer is trustworthy.
- [Grounding](concepts/grounding.md) explains the vocabulary and intention layers.
