# Install and gate a project

This page takes an existing Spec Kit project to a state where a contradiction between two
specifications fails the build. It assumes nothing about EARS or constraint solving. Concepts come
later.

## Before you start

You need [Spec Kit](https://github.com/github/spec-kit) itself:

```sh
uv tool install specify-cli==0.16.3
```

`ears-sdd` supports Spec Kit 0.16.3 and later 0.16 releases. It says so if you are outside that
range rather than failing in a way you have to diagnose.

## 1. Install the binary

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

## 2. Install the policy

```sh
ears-sdd init --project . --integration codex --ci
```

This installs three Spec Kit components through Spec Kit's own commands, and writes a starting
configuration:

| Component | What it adds |
| --- | --- |
| `ears-tdd` preset | EARS and traceability guidance composed into the specify, plan, tasks and implement commands |
| `ears-validate` extension | The validator as an agent command |
| `ears-sdd` workflow | The specify to implement cycle with validation steps and review gates |

`--ci` also writes `.github/workflows/ears-sdd.yml`, pinned to the version of the binary that wrote
it. Leave it off if you do not use GitHub Actions, but read
[why the gate needs somewhere to run](#4-make-the-gate-run) before you decide.

Running `init` a second time is an upgrade, not an error. Files you have edited are kept and
reported as kept.

## 3. Check what you got

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

## 4. Make the gate run

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

## 5. Run your first gate

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

## 6. Work the cycle

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

## Next

[A contradiction, end to end](example.md) builds a two feature project where each specification
passes alone and the pair does not.
