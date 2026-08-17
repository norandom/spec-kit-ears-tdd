# Gates

Four phases, each a superset of the one before. Every gate is read only and never runs your tests.

```sh
ears-sdd validate --phase <spec|plan|tasks|final> --all
```

`validate` exits non zero when the gate fails. `status` reports the same result and always exits
zero, for when you want the information without failing a step.

## What each phase checks

| Phase | Adds |
| --- | --- |
| `spec` | Requirement identifiers, EARS form, one `shall`, no competing modals, vocabulary terms resolve |
| `plan` | Verification mapping complete, constraint models checked within and across specifications |
| `tasks` | Every requirement covered by a task in `tasks.md` |
| `final` | Named test files exist, test command declared, no requirement prose or identifiers in production code |

The contradiction search starts at `plan`. That is the first phase where requirements are stable
enough to model, and still early enough that changing one is cheap.

## Scope is part of the claim

Every run prints the specifications it evaluated and where that scope came from:

```text
EARS/TDD spec gate: PASS
Scope: specs/*/spec.md (all matching specifications)
Features: 12  Requirements: 397  Errors: 0  Warnings: 0
```

Scope resolves in this order:

1. `--feature <path>`
2. `SPECIFY_FEATURE_DIRECTORY`
3. `.specify/feature.json`
4. The configured `spec_glob`

`--all` overrides all four.

!!! warning "This matters more than it looks"

    Spec Kit gitignores `.specify/feature.json`. Without `--all`, the same commit is evaluated over
    one feature on the author's machine and over every feature in CI, and both runs print a pass.

    A narrowed run raises a `SPEC_SCOPE` warning rather than passing quietly. A `--feature` that
    resolves outside the project is refused.

## Severities

| Severity | Effect on the gate |
| --- | --- |
| Error | Fails |
| Warning | Fails |
| Advisory | Does not fail, still reported |

Advisories cover recorded decisions and declared exemptions. They are meant to be read, not ignored.
A conflict that someone adjudicated stays visible for exactly that reason.

## Outcomes that are not a pass

An analysis that did not reach a verdict is not a verdict:

- A budget was exceeded. Part of the model was not checked.
- A file was skipped. A missing production root, an undecodable file, or an oversize file each mean
  something was not read.
- The scope was narrowed. The run covered less than the project.

Each is reported as work. None is folded into a pass.

## Machine readable output

```sh
ears-sdd validate --phase final --all --json
```

The report carries `schema_version` and a `provenance` block: validator name and version, an RFC 3339
timestamp, the resolved scope with its source, and counts of specifications and production files
actually read.

That block exists so a report cannot claim a pass over files it never opened, and so two runs against
different commits do not produce identical evidence.

## In CI

```sh
ears-sdd validate --project . --phase final --all
```

`ears-sdd init --ci` writes a GitHub Actions workflow that does this, pinned to the validator version
that generated it. See [install and gate a project](../getting-started.md#4-make-the-gate-run).
