# Spec Kit EARS/TDD

Requirements an AI agent cannot quietly contradict.

![A cucumber sliced into segments rendered as streams of binary, the slices reassembling along a green wireframe into a key that enters a lock. Around them float labels reading WHEN, IF, SHALL, TEST, CONFLICT and CONSTRAINT, with a listening ear and a waveform above.](docs/assets/spec-kit-ears-tdd.png)

**[Documentation](https://norandom.github.io/spec-kit-ears-tdd/)**

## The problem

You asked an agent to write the specification. It did, and the result reads well. Twelve features,
several hundred requirements, every sentence plausible.

Feature 11 says the workstation blocks dynamic code generation whenever the exploit mitigation is
enforced. Feature 5 says it permits dynamic code generation while the native toolchain is building.
Both are correct alone. Both passed review, because nobody reviews feature 5 and feature 11 in the
same sitting. The machine that has to satisfy both cannot.

You find out when the build breaks, and the fix is an argument held six weeks after either
requirement could be changed cheaply.

## What this does

`ears-sdd` is a policy layer for [GitHub Spec Kit](https://github.com/github/spec-kit). It extends
Spec Kit through its supported preset, extension, and workflow components; it does not fork or vendor
it.

- **Requirements hold a fixed shape.** EARS form: one trigger, one subject, one `shall`. A sentence
  that will not fit is usually two requirements or an unresolved question.
- **Words mean one thing.** Requirements declare the vocabulary terms they are about and the single
  intention they serve. An undeclared term fails the gate.
- **Contradictions surface before implementation.** Constraint models are merged across every
  specification and searched for a state where two incompatible requirements both apply. The report
  names both requirements and the exact state that reaches them.

The third is the one people want, and it does not work without the first two. Comparing requirements
across features requires knowing that two features mean the same thing by the same word.

## Install

```sh
uv tool install specify-cli==0.16.3

curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/norandom/spec-kit-ears-tdd/releases/download/v0.2.0/ears-sdd-installer.sh | sh
```

On Windows:

```powershell
powershell -c "irm https://github.com/norandom/spec-kit-ears-tdd/releases/download/v0.2.0/ears-sdd-installer.ps1 | iex"
```

## Use

```sh
ears-sdd init --project . --integration codex --ci   # install the policy and the CI gate
ears-sdd doctor                                      # what is installed, what is not
ears-sdd validate --phase spec --all                 # run a gate
```

The same commands work identically on Windows, Linux, and macOS. There are no launcher scripts to
copy into your project and nothing to mark executable. Validation has no runtime dependency at all,
not on Python and not on Spec Kit.

Add `--json` when a tool or an agent consumes the result.

**Always pass `--all`.** Spec Kit gitignores the file holding the current feature pointer, so without
it the same commit is evaluated over one feature locally and every feature in CI, and both print a
pass.

## The gates

| Phase | Adds |
| --- | --- |
| `spec` | Requirement identifiers, EARS form, one `shall`, no competing modals, terms resolve |
| `plan` | Verification mapping complete, constraint models checked within and across specifications |
| `tasks` | Every requirement covered by a task before anyone writes code |
| `final` | Named tests exist, test command declared, no requirement prose in production code |

Every run prints the specifications it evaluated and where that scope came from. A gate that silently
evaluates less than the project contains looks identical to one that passed.

## Documentation

| Page | For |
| --- | --- |
| [Why this exists](https://norandom.github.io/spec-kit-ears-tdd/) | The motive, in one page |
| [Install and gate a project](https://norandom.github.io/spec-kit-ears-tdd/getting-started/) | Adopting it |
| [A contradiction, end to end](https://norandom.github.io/spec-kit-ears-tdd/example/) | A real run, output copied from a terminal |
| [Finding contradictions](https://norandom.github.io/spec-kit-ears-tdd/concepts/contradictions/) | What a BDD is and why it suits this problem |
| [Grounding](https://norandom.github.io/spec-kit-ears-tdd/concepts/grounding/) | Vocabulary and intentions, without the ontology jargon |
| [Decisions and their reasons](https://norandom.github.io/spec-kit-ears-tdd/design/decisions/) | Why it is built this way |

## Develop

```sh
cargo nextest run                          # unit tests and the conformance corpus
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
dagger -M ci/rust.dag                      # the whole Linux CI leg, locally
uv run --with-requirements docs/requirements.txt mkdocs serve
```

`conformance/cases/` holds the behavioural contract as data: a project fixture, the invocation, and
the expected result. Any reimplementation can be held to the same cases by writing a runner of that
size.

CI runs Dagger on Linux and native `cargo` on Windows and macOS. That split is forced: Dagger
executes Linux containers only, GitHub's macOS runners cannot run Docker, and its Windows runners
cannot run Linux containers. Since this tool exists partly to get path separators, line endings, and
filesystem case sensitivity right, the platforms it must be tested on are the ones Dagger cannot
reach.

## License

MIT
