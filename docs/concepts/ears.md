# Requirements that hold their shape

EARS stands for Easy Approach to Requirements Syntax. It is a small set of sentence templates for
writing requirements, published by Alistair Mavin and colleagues at Rolls-Royce in 2009. There is no
tooling lock-in and nothing to buy. It is a writing convention.

The convention is worth adopting for one reason that has nothing to do with tidiness: a sentence
with a fixed shape can be compared to another sentence by a machine.

## The five forms

Every normative requirement fits one of these:

| Form | Template | Example |
| --- | --- | --- |
| Ubiquitous | The `<system>` shall `<response>` | The workstation shall record every mitigation change in the audit log. |
| Event driven | When `<trigger>`, the `<system>` shall `<response>` | When the native toolchain is building, the workstation shall permit dynamic code generation. |
| State driven | While `<state>`, the `<system>` shall `<response>` | While the AI tools category is not selected, the manager shall leave its products unchanged. |
| Optional feature | Where `<feature>`, the `<system>` shall `<response>` | Where OpenCode is enabled, the manager shall provide its CLI only inside the AI environment. |
| Unwanted behaviour | If `<condition>`, then the `<system>` shall `<response>` | If the captured digest does not match, then the resource shall fail without writing. |

Each requirement carries a stable identifier and contains exactly one `shall`:

```markdown
- **REQ-001**: When the arbitrary-code-guard mitigation is enforced, the workstation shall block
  dynamic code generation for the process.
```

## What the form buys you

### The trigger stops being implicit

"The system shall validate input" hides the question of when. On startup? Per request? The event
driven form has nowhere to put the response until you have written the trigger, so you notice the
question while writing rather than during implementation.

This is the property the constraint model depends on. A requirement's trigger becomes its guard, and
a requirement with no identifiable trigger has nothing to model.

### One obligation per identifier

"The system shall validate input and log failures" is two requirements sharing one identifier. That
matters in a practical way: when you map requirements to tests, one of the two behaviours gets a
test and the other does not, and the traceability report shows full coverage.

The gate rejects a second `shall` in one requirement. Split it into two identifiers.

### Modal verbs stop competing

`should`, `may`, and `must` in the same document mean different things to different readers, and
after a while nobody knows which requirements are binding. EARS uses `shall` for obligations and
nothing else. The gate reports competing modals so the ambiguity does not accumulate.

## Why this matters more with an agent

An agent writes fluent requirements. Fluency is the problem. Prose that reads well passes review on
the strength of reading well, and an agent produces a great deal of it quickly.

A fixed form removes the part a reader is bad at judging. It does not make the requirement correct.
It makes the requirement checkable, and it makes a vague requirement visibly vague instead of
smoothly worded.

There is a second effect worth knowing about. An agent asked to write a requirement in a form that
demands an explicit trigger will either find the trigger or invent one. An invented trigger is
usually obvious on sight, where a missing trigger is not.

## What the gate checks

At `--phase spec`:

- Every requirement has a unique identifier within its feature.
- Every requirement contains exactly one `shall`.
- Every requirement opens with a recognised form.
- No competing modal verbs appear in a normative sentence.
- Requirements inside fenced code blocks are ignored, so documented bad examples stay examples.

Findings are prefixed `EARS_`. See the [findings reference](../reference/findings.md).

## What EARS does not do

It does not make requirements true, complete, or consistent. A perfectly formed requirement can be
wrong, and two perfectly formed requirements can contradict each other.

Form is a precondition for the checks that follow. It is not one of them.

## Next

[Grounding](grounding.md) covers the layer that makes two requirements comparable across features:
agreeing on what the words mean.
