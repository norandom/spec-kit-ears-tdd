# Install and gate a project

This page takes a project, new or existing, to a state where a contradiction between two
specifications fails the build. It assumes nothing about EARS or constraint solving. Concepts come
later.

## What you are installing

Two things, and the difference between them is the whole design.

**The binary** is installed once per machine and runs the gates. It has no runtime dependency on
Spec Kit, Python, or any interpreter.

**The policy** is three components registered into Spec Kit, once per project. This is the part that
matters, and it is easy to miss because the binary gets all the attention. Registration is what puts
EARS into the specification step and test-first into the task step. Spec Kit composes the policy into
the commands your agent already runs, so the agent writes EARS requirements because its own
instructions now say to, not because a human remembered to ask.

| Component | Registered as | What it changes |
| --- | --- | --- |
| `ears-tdd` | preset | Composes EARS and traceability postconditions into `speckit.specify`, `plan`, `tasks` and `implement` |
| `ears-validate` | extension | Adds `speckit.ears-validate.validate` as a command the agent can run |
| `ears-sdd` | workflow | The specify to implement cycle, with validation steps and review gates |

A validator nobody registered is a linter someone runs occasionally. A registered policy changes
what the agent produces in the first place, which is the cheaper place to fix a requirement.

## 1. Install Spec Kit

```sh
uv tool install specify-cli==0.16.3
```

`ears-sdd` declares a supported range of `>=0.16.3,<0.17.0` and the components refuse to install
outside it. `ears-sdd doctor` reports the version your project was initialized with, so a mismatch
shows up as a sentence rather than as a failure you have to diagnose.

## 2. Install the binary

=== "Linux and macOS"

    ```sh
    curl --proto '=https' --tlsv1.2 -LsSf \
      https://github.com/norandom/spec-kit-ears-tdd/releases/latest/download/ears-sdd-installer.sh | sh
    ```

=== "Windows"

    ```powershell
    powershell -c "irm https://github.com/norandom/spec-kit-ears-tdd/releases/latest/download/ears-sdd-installer.ps1 | iex"
    ```

=== "From source"

    ```sh
    cargo install --git https://github.com/norandom/spec-kit-ears-tdd ears-sdd
    ```

These install the most recent release. Swap `latest/download` for
`download/v0.2.0` to pin a specific version, and see
[all releases](https://github.com/norandom/spec-kit-ears-tdd/releases) for what is available.

!!! note "The CI gate pins deliberately"

    `ears-sdd init --ci` writes a workflow pinned to the version of the binary that generated it,
    and that is the opposite choice on purpose. A developer installing the tool wants the current
    release. A gate whose verdict can change without a commit to your repository is not a gate: the
    same tree would pass today and fail tomorrow, and the failure would look like the author's
    fault. Re-run `ears-sdd init --ci` after upgrading to move the pin deliberately.

It is one binary. There are no launcher scripts to copy into your project and nothing to mark
executable. Validation never invokes Python, a shell, or Spec Kit.

## 3. Register the policy

```sh
ears-sdd init --project . --integration codex --ci
```

This registers the three components through Spec Kit's own `preset add`, `extension add` and
`workflow add` commands, and writes a starting configuration. It never edits Spec Kit's registries
directly, so an uninstall is Spec Kit's `remove` and not a hunt through its state.

The project does not have to exist yet:

- In a directory with no `.specify`, `init` runs `specify init` first and registers on top of it.
- In a project that already has one, it registers only, and leaves the existing setup alone.

Every command it runs is printed before it runs, so what changed is on screen rather than inferred.

`--ci` also writes `.github/workflows/ears-sdd.yml`, pinned to the version of the binary that wrote
it. Leave it off if you do not use GitHub Actions, but read
[why the gate needs somewhere to run](#6-make-the-gate-run) before you decide.

Running `init` a second time is an upgrade, not an error. Files you have edited are kept and reported
as kept.

## 4. Confirm the registration

Ask Spec Kit, not the binary. These are its own listings, and they are the evidence that the policy
is part of the toolchain rather than sitting beside it:

```console
$ specify preset list
EARS Requirements and TDD (ears-tdd) v0.2.0 — enabled — priority 5

$ specify extension list
✓ EARS/TDD Validator (v0.1.0)  ears-validate

$ specify workflow list
EARS/TDD SDD Cycle (ears-sdd) v0.3.0
```

The effect on your agent is visible in the composed commands. Spec Kit wraps its own specification
command with the preset's postcondition, so the instruction the agent reads now ends with this:

```console
$ grep -A3 'EARS postcondition' .agents/skills/speckit-specify/SKILL.md
## EARS postcondition

Before finishing, ensure each normative requirement has a unique `REQ-NNN` ID, contains exactly
one `shall`, and uses an EARS form documented in the resolved specification template.
```

and the task command with the test-first one:

```console
$ grep -A3 'Test-first postcondition' .agents/skills/speckit-tasks/SKILL.md
## Test-first postcondition

Tests are mandatory. Place each failing-test task before its corresponding implementation task.
Every behavior task must identify its `REQ-NNN` coverage and test selector.
```

That is the registration doing its work. The gate checks the result; the preset is what makes the
result likely to pass.

## 5. Check what you got

```sh
ears-sdd doctor
```

```text
ears-sdd 0.2.0 checking /home/you/project

  [ok  ] Spec Kit project       .specify is present
  [ok  ] Spec Kit version       0.16.3, within >=0.16.3,<0.17.0
  [ok  ] Policy preset          ears-tdd is installed
  [ok  ] Validator extension    ears-validate is installed
  [ok  ] Workflow               ears-sdd is installed
  [ok  ] Configuration          .specify/ears-sdd.toml is present
  [ok  ] Specifications         12 found; `--all` evaluates every one
  [ok  ] Automated enforcement  .github/workflows/ears-sdd.yml runs the validator

8 checks: 8 ok, 0 warning(s), 0 failure(s)
```

Every warning names the command that resolves it. The check worth reading twice is the last one,
for the reason in the next section.

## 6. Make the gate run

A project with no specifications passes every gate. So does a project whose gate nobody runs. Both
look exactly like a project that is clean.

The preset tells your agent to validate, and the agent command gives it a way to. Neither is a
mechanism. An agent that decides it has finished simply does not run the check, and no amount of
instruction in a prompt changes that.

Enforcement therefore lives in two places that fail loudly:

- The `ears-sdd` workflow runs the validator as shell steps, so a failing gate stops the run.
- CI runs `ears-sdd validate --project . --phase final --all` on every push and pull request.

If you skipped `--ci`, add it now:

```sh
ears-sdd init --ci
```

## 7. Run your first gate

```sh
ears-sdd validate --phase spec --all
```

```text
EARS/TDD spec gate: PASS
Scope: specs/*/spec.md (all matching specifications)
Features: 12  Requirements: 397  Errors: 0  Warnings: 0
```

Expect failures on an existing project. That is the point of running it. Each finding names the
file, the feature, the requirement, and what to change. Start with the
[findings reference](reference/findings.md) if a code is unfamiliar.

!!! warning "Always pass `--all`"

    Without it, the run evaluates whichever single feature Spec Kit currently points at, and reports
    a pass for the project on the strength of one specification. Spec Kit gitignores the file that
    holds that pointer, so the same commit is checked over one feature on your machine and over
    every feature in CI. A narrowed run prints a `SPEC_SCOPE` warning rather than passing quietly,
    and every run prints the scope it used.

## 8. Work the cycle

There are four gates, one per phase, each a superset of the one before. Run them where they belong:

| Phase | Run it after | It answers |
| --- | --- | --- |
| `spec` | Requirements are written | Is every requirement well formed and grounded? |
| `plan` | Design is done | Does every requirement map to a verification, and do the requirements agree with each other? |
| `tasks` | Tasks are generated | Is every requirement covered by a task before anyone writes code? |
| `final` | Implementation is done | Do the named tests exist, and did requirement prose stay out of production code? |

The contradiction search starts at `plan`, which is the first point where requirements are stable
enough to model and still cheap to change.

If you installed the workflow, this sequencing is already wired:

```sh
specify workflow run ears-sdd
```

## 9. Add grounding when you are ready

Everything above works without a vocabulary. Tags are optional per requirement, so a project that
declares none is unaffected by the grounding layer, and the requirement and traceability gates carry
their own weight from day one.

Add it when you want the checks that need it: comparing requirements across features, and finding
contradictions between them. Both depend on two specifications meaning the same thing by the same
word, which is what a vocabulary is.

If a layer is in the way before then, switch it off rather than working around it:

```toml title=".specify/ears-sdd.toml"
[checks]
traceability = false
```

Every run then prints `Disabled: traceability (not checked)`, and the report records it. See
[configuration](reference/configuration.md#turning-layers-off).

### Two ways in, and the better one is less obvious

**Declare first.** Write the vocabulary before the requirements, and every requirement draws its
tags from a closed list. `init` installs `.specify/vocabulary.toml.sample` and
`.specify/intentions.toml.sample` to start from.

This is what the [twelve feature example](tutorial/needle-in-a-haystack.md) did: 28 terms fixed
before 383 requirements were written. An undeclared tag became impossible by construction rather
than something the gate caught afterwards. If you are starting a new feature set, prefer this.

**Extract after.** Read the vocabulary out of requirements that already exist, which is the only
option on a brownfield codebase:

```sh
ears-sdd vocab-init --all > proposed-vocabulary.toml
```

Candidates come from the conditions in `When`/`While`/`Where`/`If` clauses, the subject of each
requirement, and anything in backticks. They are ranked by how many requirements mention each one,
with that count printed above every entry, so the concepts your specifications turn on are at the
top and the tail is where the extraction guessed. Definitions come out empty, and an empty
definition fails the gate, so a scaffold cannot be committed unread.

Run it again after editing and it proposes only what is new. Terms you have declared, and
alternative labels you have already decided on, are left out.

**Import.** If your domain has a published vocabulary, neither of the above is the best start:

```sh
ears-sdd vocab-import domain-concepts.ttl > .specify/vocabulary.toml
```

See [grounding](concepts/grounding.md#starting-from-a-vocabulary-that-already-exists) for what
survives the import and what does not.

## Next

[A contradiction, end to end](example.md) builds a two feature project where each specification
passes alone and the pair does not.
