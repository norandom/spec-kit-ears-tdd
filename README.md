# Spec Kit EARS/TDD

A reusable policy layer for [GitHub Spec Kit](https://github.com/github/spec-kit). It keeps
requirements in EARS form, requires explicit requirement-to-test traceability, and prevents
requirement prose from leaking into production code.

This repository extends Spec Kit; it does not fork or vendor it.

`ears-sdd` is a single self-contained binary. Validation has no runtime dependency at all — not on
Python, not on Spec Kit. Only `ears-sdd init` shells out to `specify`, because installing components
is Spec Kit's job.

## What is installed

- `ears-tdd` preset: composes with the upstream specification, plan, task, and implementation
  commands and templates.
- `ears-validate` extension: adds the deterministic, read-only validator and the
  `speckit.ears-validate.validate` agent command.
- `ears-sdd` workflow: adds validation and human approval gates to the normal SDD cycle.

The default output is for people. Add `--json` when another tool or agent consumes it.

## Use it

```sh
ears-sdd init --project /path/to/project --integration codex
ears-sdd validate --phase spec --all
ears-sdd status  --phase final --all
```

The same commands work identically on Windows, Linux, and macOS. There are no launcher scripts to
copy into your project and nothing to mark executable.

After planning, create `traceability.toml` beside each feature's `spec.md`; `init` writes a sample to
`.specify/traceability.toml.sample`. Set the project's real `test_command` in
`.specify/ears-sdd.toml` before using the final gate. The validator checks the command is declared;
it never runs it.

## Gates

| Phase | Checks |
| --- | --- |
| `spec` | Requirement IDs, EARS form, one `shall`, no competing modal verbs |
| `plan` | Spec checks plus complete automated/manual verification mapping |
| `tasks` | Same deterministic mapping gate before implementation |
| `final` | All checks, referenced test files, declared test command, no requirement IDs/prose in production roots |

## Scope, and why it is printed

Every run states which specifications it evaluated and where that scope came from:

```text
EARS/TDD spec gate: PASS
Scope: specs/*/spec.md (all matching specifications)
Features: 4  Requirements: 67  Errors: 0  Warnings: 0
```

Scope resolves as `--feature` > `SPECIFY_FEATURE_DIRECTORY` > `.specify/feature.json` > the
configured glob, and `--all` overrides all of them. This matters more than it looks: Spec Kit
gitignores `feature.json`, so without `--all` the same commit is evaluated over one feature on the
author's machine and over every feature in CI. **Use `--all` in CI.** A narrowed run emits a
`SPEC_SCOPE` warning rather than passing quietly.

## Evidence

`--json` carries a `schema_version` and a `provenance` block — validator version, timestamp, the
scope and its source, and counts of what was actually read. Two runs against different commits or
different configurations no longer produce identical evidence, and a report can no longer claim a
pass over files it never opened.

## Conformance corpus

`conformance/cases/` holds the behavioural contract as data: a project fixture, the invocation, and
the expected result. `crates/ears-sdd/tests/conformance.rs` runs every case through the
command-line interface. Any reimplementation can be held to the same cases by writing a runner of
that size.

## Develop

```sh
cargo nextest run          # unit tests and the conformance corpus
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
dagger -M ci/rust.dag      # the whole Linux CI leg, locally
```

CI runs Dagger on Linux and native `cargo` on Windows and macOS. That split is forced rather than
chosen — Dagger executes Linux containers only, GitHub's macOS runners cannot run Docker, and its
Windows runners cannot run Linux containers. Since this tool exists to get path separators, line
endings, and filesystem case sensitivity right, the platforms it must be tested on are exactly the
ones Dagger cannot reach.

See [EARS policy](docs/ears-policy.md) and [architecture](docs/architecture.md) for the detailed
contracts.
