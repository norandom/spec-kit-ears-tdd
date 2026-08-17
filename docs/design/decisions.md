# Decisions and their reasons

Choices that are not obvious from the code, with the reasoning that produced them. Several were
forced by a defect rather than chosen, and those are marked.

## A policy layer, not a fork

Spec Kit supports presets, extensions, and workflows as first class components. This project uses
those, and installs through Spec Kit's own commands rather than editing its registries.

A fork would have been easier for about two months, and then it would have been a fork.

## One binary, no interpreter

`ears-sdd` ships as a single static executable. Validation invokes nothing: not Python, not a shell,
not Spec Kit. Only `ears-sdd init` shells out, to `specify`, because installing components is Spec
Kit's job.

This was forced. The project previously shipped four launcher scripts, a `.sh` and a `.ps1` copied
into every consuming project plus another pair inside the extension, all four existing only to
locate a Python interpreter. Between them they carried the two worst defects this project has had:

- A Windows clone with `core.autocrlf=true` rewrote the POSIX scripts with CRLF, so they failed on
  Linux with `env: 'sh\r': No such file or directory`, and the bytes propagated into consuming
  projects verbatim.
- Their execute bit was lost on any project initialized from Windows, so a POSIX clone got
  `Permission denied` on the documented command.

A test now asserts no interpreter script is ever embedded in the binary, and CI asserts none is
committed outside Spec Kit's own vendored scripts and the conformance fixtures.

## Binary decision diagrams rather than an SMT solver

Covered in full under [finding contradictions](../concepts/contradictions.md#why-not-an-smt-solver).
In short: the fragment is finite and boolean after finitization, determinism is a hard requirement
because the conformance corpus states expected results as data, witnesses fall out of a path walk,
and a native solver library would end the single binary property on exactly the platform matrix where
this project has had its worst bugs.

## A budget in states, not seconds

A wall clock timeout makes a verdict depend on the machine. The same commit would pass on a
workstation and fail on a slower CI runner, and the conformance corpus could not state an expected
result at all.

State counts are known by multiplication before any search starts, so the tool refuses a component it
cannot afford instead of discovering that halfway through.

## An exceeded budget is a failure, and raising it is not the fix

The budget is a declared limit on what the project is willing to leave unchecked. Raising it to clear
a finding converts a known gap into an invisible one.

The same principle runs through the whole tool: a check that did not run is reported as work, never
folded into a pass. Skipped files, narrowed scope, and unmodelled requirements are all visible for
the same reason.

## Requirements reference exactly one intention

A requirement may serve at most one goal. This looks restrictive and it is the reason adjudication
means anything.

When two requirements contradict, the tool asks whether the goals involved have a unique maximum
under the declared precedence. That question stops meaning anything if a requirement serves three
goals at once, because there is no single pair to compare.

The winner must be a unique maximum, not a maximal element. If two goals both outrank a third but not
each other, nothing has been decided, and the tool says so rather than picking one.

## Precedence is pairwise, never a global ranking

A global priority list forces an answer for every pair, including the pairs nobody has thought about.
Pairwise declaration lets a project record the decisions it has actually made and leave the rest
open, which is more honest and produces a better error message: the tool names the specific pair that
needs ordering.

## The conflict relation is symmetric

Preference based reasoning is only sound over a symmetric attack relation, and requirement
contradiction genuinely is symmetric. A contradicts B exactly when B contradicts A.

A one sided declaration is therefore read symmetrically. Otherwise the answer would depend on which
file someone happened to write first.

## Requirements whose effects conflict are never split apart

Forced by a defect. Decomposition originally grouped requirements by shared guard terms alone, which
is sound only if requirements in different groups cannot interact. Two requirements asserting
conflicting effects do interact, whatever their guards look like, because the state making both
guards true forces both effects.

The result was the worst available outcome: each group came back satisfiable, and the merged run
reported a pass it had not earned. The internal soundness contract did not catch it, because it
counted memberships rather than checking that conflicting pairs stayed together.

Both are fixed, and `merge-disjoint-terms-conflict` in the conformance corpus pins the shape.

## The events subsystem is not used

Spec Kit has an extension event mechanism, and a stop event hook is the obvious way to make the gate
run without relying on an agent choosing to. It does not work here.

The dispatcher resolves a command's first script token as a path under the extension directory,
requires that path to exist, and launches it as `bash <path>` or `pwsh -File <path>`. There is no way
to invoke a program on `PATH`. Wiring the hook means shipping a launcher script whose only job is to
exec the binary, which is the file class the integrity check forbids for the reasons above.

The failure mode decided it. With the event declared and no resolvable script, the extension installs
reporting success, the agent configuration gains a stop hook, and every firing answers
`No script found for event command` and exits zero. A gate that presents as enabled while enforcing
nothing is worse than an absent one, because it is indistinguishable from a passing one.

Enforcement lives instead in the workflow's shell steps, which exit with the validator's status, and
in CI, which fails visibly when absent.

## Validation runs before the human gates

Asking someone to approve requirements a deterministic checker has not yet examined spends their
attention on exactly what the tool is about to catch. Reversed, the reviewer sees the findings first
and spends their judgement on what a checker cannot decide.

Rejecting a gate pauses the run for a resume rather than aborting it. Aborting discards the whole
run, so a reviewer who spots one wrong requirement has to start again, which teaches people to
approve.

## The behavioural contract is data

`conformance/cases/` holds a project fixture, the invocation, and the expected result. A test runs
every case through the command line interface.

This exists so a second implementation in another language can be held to the same contract by
writing a runner of that size. It also caught a case where the corpus itself asserted a defect,
because the expected result was written from the implementation's behaviour rather than from the
rule it was meant to encode.

## Scope is printed on every run

A gate that silently evaluates less than the project contains is worse than no gate, because it looks
identical to one that passed.

Spec Kit gitignores the file holding the current feature pointer, so without `--all` the same commit
is evaluated over one feature locally and every feature in CI. Both print a pass. The resolved scope
and its source are therefore printed on every run and recorded in the report, and a narrowed run
raises a warning.

## CI is split across Dagger and native runners

The Linux leg runs through a Dagger module, and Windows and macOS run native `cargo`.

This split is forced rather than chosen. Dagger executes Linux containers only, GitHub's macOS
runners cannot run Docker, and its Windows runners cannot run Linux containers. Since this tool
exists partly to get path separators, line endings, and filesystem case sensitivity right, the
platforms it most needs testing on are exactly the ones Dagger cannot reach.

The split has an unplanned benefit. The Dagger leg pulls one action plus an install script, where
each native leg pulls three actions from the same host, so it survives the rate limiting that
intermittently fails the others.
