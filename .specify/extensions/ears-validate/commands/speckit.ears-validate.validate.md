---
description: Run the deterministic EARS/TDD validator and report actionable failures.
---

## Human command

The user runs the same check without an agent:

```text
ears-sdd validate --phase <spec|plan|tasks|final> [--all]
```

`ears-sdd` is a single self-contained binary. It has no runtime dependency on Python, on Spec Kit,
or on anything else — if it is not on `PATH`, install it rather than looking for a fallback script.

When a run reports a pass that looks too easy — no findings over a project that has clearly never
been checked — the question is whether anything is installed at all:

```text
ears-sdd doctor
```

It names each missing piece and the command that installs it. A project with no specifications
passes every gate vacuously, and `doctor` is what distinguishes that from a project that is clean.

## Agent execution

Interpret `$ARGUMENTS` as the phase; use `final` when it is empty. Run:

```text
ears-sdd validate --project . --phase <phase> --all --json
```

Parse the JSON result and report each finding with its file, its feature, and its requirement ID.

Read `provenance` before trusting the verdict. It records the validator version, the timestamp, and
the scope the run actually used. A result whose `scope.source` is not `glob` evaluated a single
feature, which is narrower than the project — say so when reporting a pass.

If validation fails, change only the specification, plan, task, traceability, or policy
configuration artifacts that caused the failure. Never paste requirement prose or requirement IDs
into production code to make a finding disappear. Do not implement product behavior as part of this
validation command.

## Outcomes that are not a pass

An analysis that did not reach a verdict is not a verdict. Treat these as failures and report them
as work, never as success:

- **A budget was exceeded.** The finding carries `detail` with the component, its variable count,
  its state count, and the terms contributing most to that count. The lever is narrowing the guards
  on the named terms, or splitting the component so fewer terms interact. **Do not raise the
  budget.** The budget is a declared limit on what the project is willing to leave unchecked;
  raising it converts a known gap into an invisible one.
- **A check was skipped.** A missing production root, an undecodable source file, or an oversize
  file each mean something was not read. Report what was skipped rather than reporting a pass over
  it.
- **The scope was narrowed.** A `SPEC_SCOPE` warning means the run covered one feature rather than
  the project. Say so when reporting a pass, and prefer `--all`.

## Exemptions

A separation finding may be declared intentional in the source with a marker and a reason. When the
report shows exempted findings, say how many and why. An exemption is a recorded judgement, so it is
something to review, not something to ignore — and adding one to silence a genuine leak is the one
misuse the mechanism cannot prevent by itself.
