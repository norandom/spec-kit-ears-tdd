# Spec Kit EARS/TDD Constitution

## Core Principles

### I. One binary, no interpreter

`ears-sdd` ships as a single static executable. Validation invokes nothing: not Python, not a
shell, not Spec Kit. Only `ears-sdd init` shells out, to `specify`, because installing components
is Spec Kit's job. No launcher script may be embedded in the binary or committed outside Spec
Kit's own vendored scripts and the conformance fixtures; a test and CI assert this.

### II. Validation is read-only

Validation never creates, deletes, or modifies any file in the assessed project, and never runs
project tests implicitly. It reads files and writes nothing.

### III. Scope is part of the claim

A gate that silently evaluates less than the project contains looks identical to one that passed.
Every run prints the resolved scope and its source (`--feature` > `SPECIFY_FEATURE_DIRECTORY` >
`.specify/feature.json` > the configured glob, `--all` overriding all) and records them in the
machine-readable report; a narrowed run raises `SPEC_SCOPE`, and a `--feature` resolving outside
the project is refused. A check that did not run is reported as work — missing production root,
undecodable source, oversize file — never folded into a pass.

### IV. The behavioural contract is data

`conformance/cases/` holds a project fixture, the invocation, and the expected result for every
behavioural case, run through the CLI by a test. Expected results are written from the rule the
behaviour must encode, not from the implementation's output, so a second implementation in another
language can be held to the same contract by writing a runner of the same size.

### V. Translation boundary

Requirements define observable behavior; plans translate requirements into design; tasks translate
the plan into test-first work. Tests may name requirement IDs for traceability. Production code
contains neither requirement IDs nor copied EARS sentences; the final gate scans the configured
production roots to enforce this.

### VI. Deterministic verdicts, budgets in states

A verdict must not depend on the machine. Resource limits are state counts known by multiplication
before any search starts; an exceeded budget is a failure, and raising the budget is not the fix —
it converts a known gap into an invisible one. The constraint search uses binary decision diagrams,
not an SMT solver, to keep the single-binary property on the whole platform matrix.

### VII. Extend Spec Kit, never fork it

The policy is registered through Spec Kit's own preset, extension, and workflow components, installed
with `specify preset/extension/workflow add`. Registries are never edited directly. Extension event
hooks are not used for enforcement (they cannot invoke a binary on `PATH` without a forbidden
launcher script); enforcement lives in the workflow's shell steps and in CI, which fails visibly.

### VIII. Validation before human gates

The deterministic checker runs before a human approves: findings first, then judgement. Rejecting a
gate pauses the run for a resume rather than aborting it.

## Additional Constraints

- Requirements hold EARS form: one trigger, one subject, one `shall`. A sentence that will not fit
  is usually two requirements or an unresolved question.
- Words mean one thing: requirements declare the vocabulary terms they are about and the single
  intention they serve; an undeclared term fails the gate, and a requirement serves at most one
  intention so that adjudication has a unique maximum.
- Contradictions surface before implementation: constraint models are merged across every
  specification and searched for a state where two incompatible requirements both apply; the
  conflict relation is symmetric and precedence is pairwise, never a global ranking.
- CI is split across a Dagger Linux leg and native Windows/macOS legs, because path separators, line
  endings, and filesystem case sensitivity are exactly what the tool exists to get right.

## Development Workflow

1. Specify the feature (`/speckit-specify`), then run the spec gate before review.
2. Plan (`/speckit-plan`), then run the plan gate.
3. Decompose into test-first tasks (`/speckit-tasks`); conformance fixtures are written first and
   must fail until the matching implementation lands; then run the tasks gate.
4. Implement (`/speckit-implement`) red-green; production code never contains requirement IDs.
5. Run the final gate (`ears-sdd validate --phase final --all`) and CI; both must pass visibly.

## Governance

This constitution supersedes all other practices in this repository. Amendments require a
documented reason, approval, and a migration plan for existing artifacts. The validator itself
does not gate on this file; the gates it enforces (EARS form, identifiers, traceability, separation,
scope, read-only) are the machine-checkable subset of these principles, and the rest is enforced by
review and CI.

**Version**: 1.0.0 | **Ratified**: 2026-11-02 | **Last Amended**: 2026-11-02
