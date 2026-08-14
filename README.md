# Spec Kit EARS/TDD

A reusable policy layer for [GitHub Spec Kit](https://github.com/github/spec-kit). It keeps
requirements in EARS form, requires explicit requirement-to-test traceability, and prevents
requirement prose from leaking into production code.

This repository extends Spec Kit; it does not fork or vendor Spec Kit. The development environment
pins the published `specify-cli==0.16.3` release in `pyproject.toml` and `uv.lock`.

## What is installed

- `ears-tdd` preset: composes with the upstream specification, plan, task, and implementation
  commands and templates.
- `ears-validate` extension: adds a deterministic, read-only validator and
  `speckit.ears-validate.validate` agent command.
- `ears-sdd` workflow: adds validation and human approval gates to the normal SDD cycle.
- Project launchers: `ears-sdd.ps1` on Windows and `ears-sdd` on Unix-like systems.

The default output is for people. Add `--json` only when another tool or agent consumes it.

## Develop this repository

```powershell
uv sync --all-extras
uv run ears-sdd --help
uv run pytest
uv run ruff check .
```

## Adopt in another project

Install the immutable `v0.1.0` wheel release:

```powershell
uv tool install https://github.com/norandom/spec-kit-ears-tdd/releases/download/v0.1.0/spec_kit_ears_tdd-0.1.0-py3-none-any.whl
```

Then install the policy into a project:

```powershell
ears-sdd init --project C:\path\to\project --integration codex
```

The command prints every `specify` mutation before it executes it. It initializes Spec Kit only
when needed, installs the three local components, and creates launchers and a project-specific
`.specify/ears-sdd.toml` without overwriting existing policy files.

In the consuming Windows project:

```powershell
.\ears-sdd.ps1 validate --phase spec
.\ears-sdd.ps1 status --phase final
```

On Unix-like systems:

```sh
./ears-sdd validate --phase spec
./ears-sdd status --phase final
```

After planning, create `traceability.toml` beside each feature's `spec.md`. Copy
`config/traceability.toml.sample` as a starting point. Set the consuming project's real
`test_command` in `.specify/ears-sdd.toml` before using the final gate. The validator checks the
command is declared; it does not execute it implicitly.

## Gates

| Phase | Checks |
| --- | --- |
| `spec` | Requirement IDs, EARS form, one `shall`, no competing modal verbs |
| `plan` | Spec checks plus complete automated/manual verification mapping |
| `tasks` | Same deterministic mapping gate before implementation |
| `final` | All checks, referenced test files, declared test command, no requirement IDs/prose in production roots |

The traceability file is TOML so the validator remains Python-standard-library-only in installed
projects. A mapping is either automated with concrete test selectors, or manual with a meaningful
rationale. "Decompile/test later"-style placeholders fail validation.

## Clean upstream inheritance

Spec Kit remains an external, exact release dependency:

```text
specify-cli 0.16.3 release
        ↓ runtime template resolution
EARS/TDD preset + validator + workflow
        ↓ copied by explicit init
Project-specific .specify/ears-sdd.toml
```

To test a new upstream version:

1. Change only the Spec Kit release pin in `pyproject.toml` and the compatible ranges in the component
   manifests.
2. Run `uv lock` and inspect the exact resolved release in `uv.lock`.
3. Run `uv run pytest` and `uv run ruff check .`.
4. Run the clean-install tests against Codex and at least one non-Codex integration.
5. Review the generated command/template diff before releasing.

Do not copy upstream templates into this repository. Command wrappers use `{CORE_TEMPLATE}` and
template additions use Spec Kit's `append` strategy, so upstream remains the lower layer.

## Publishing model

Local development installs components by path because a Spec Kit bundle artifact is a composition
manifest, not an installer for arbitrary sibling component directories. Once the preset,
extension, and workflow have versioned release URLs or catalog entries, add a catalog-backed
`bundle.yml` that pins them. Until then, the bootstrap CLI is the supported local installation
path and avoids pretending an offline bundle can resolve unpublished components.

See [EARS policy](docs/ears-policy.md) and [architecture](docs/architecture.md) for the detailed
contracts.
