---
name: speckit-ears-validate-validate
description: Run the deterministic EARS/TDD validator and report actionable failures.
compatibility: Requires spec-kit project structure with .specify/ directory
metadata:
  author: github-spec-kit
  source: ears-validate:commands/speckit.ears-validate.validate.md
---

## Human command

The user runs the same check without an agent:

```text
ears-sdd validate --phase <spec|plan|tasks|final> [--all]
```

`ears-sdd` is a single self-contained binary. It has no runtime dependency on Python, on Spec Kit,
or on anything else — if it is not on `PATH`, install it rather than looking for a fallback script.

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