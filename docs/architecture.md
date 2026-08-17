# Architecture

Three independently versioned concerns:

1. `specify-cli` is an external tool. It is required only by `ears-sdd init`, which installs
   components through Spec Kit's own commands rather than editing its registries.
2. Spec Kit components express agent guidance and workflow composition through supported manifests.
3. The validator provides deterministic evidence independent of an agent.

`ears-sdd` is a single binary with the policy components compiled into it. Validation reads files
and writes nothing; it has no runtime dependency on Spec Kit, on Python, or on any interpreter.

## Why there are no launcher scripts

There used to be four: a `.sh` and a `.ps1` copied into every consuming project, and another pair
inside the extension. All four existed to locate a Python interpreter, and between them they carried
the two worst defects this project has had. A Windows clone with `core.autocrlf=true` rewrote the
POSIX ones with CRLF, so they failed on Linux with `env: 'sh\r': No such file or directory`, and the
bytes propagated into consuming projects verbatim. Their execute bit was lost on any project
initialized from Windows, so a POSIX clone got `Permission denied` on the documented command.

A single binary on `PATH` removes the need for all of them, and the defect class with it. A test
asserts no interpreter script is ever embedded again, and CI asserts none is committed outside Spec
Kit's own vendored scripts and the conformance fixtures.

## Translation boundary

Requirements define observable behavior. Plans translate requirements into design decisions. Tasks
translate the plan into test-first work. Tests may name requirement IDs for traceability. Production
code may contain neither requirement IDs nor copied EARS sentences.

The final gate enforces this by scanning configured production roots, honouring ignore rules, with a
single multi-pattern pass per file. Requirement identifiers restart at `REQ-001` in every feature, so
findings are qualified by the feature that owns them and deduplicated on that qualification;
otherwise one leaked identifier is reported once per feature that happens to declare it.

## Scope is part of the claim

A gate that silently evaluates less than the project contains is worse than no gate, because it
looks identical to one that passed. Scope resolves as `--feature` > `SPECIFY_FEATURE_DIRECTORY` >
`.specify/feature.json` > the configured glob, with `--all` overriding all of them. The resolved
scope and its source are printed on every run and recorded in the machine-readable report, and a
narrowed run raises `SPEC_SCOPE`. A `--feature` that resolves outside the project is refused.

## Safety properties

- Validation is read-only and never runs project tests implicitly.
- A check that did not run is reported, never silently skipped: a missing production root, an
  undecodable source file, and an oversize file each raise a warning.
- Bootstrap operations are printed before execution, flushed so the order survives a pipe.
- Existing policy configuration is preserved; a second `init` is an upgrade, not an abort.
- No global Spec Kit, agent, shell, or Git configuration is changed.

## Evidence

The machine-readable report carries `schema_version` and a `provenance` block: validator name and
version, an RFC 3339 timestamp, the resolved scope with its source, and counts of specifications and
production files actually read. The behavioural contract itself lives in `conformance/cases/` as
data — fixture, invocation, expected result — so a second implementation can be held to it exactly.
