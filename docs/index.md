# Why this exists

![A cucumber sliced into segments rendered as streams of binary, the slices reassembling along a green wireframe into a key that enters a lock. Around them float labels reading WHEN, IF, SHALL, TEST, CONFLICT and CONSTRAINT, with a listening ear and a waveform above.](assets/spec-kit-ears-tdd.png)

You asked an agent to write the specification. It did, and the result reads well. Twelve features,
several hundred requirements, every sentence plausible.

Feature 11 says the workstation blocks dynamic code generation whenever the exploit mitigation is
enforced. Feature 5 says the workstation permits it while the native toolchain is building. Both are
correct on their own. Both passed review, because nobody reviews feature 5 and feature 11 in the
same sitting. The machine that has to satisfy both cannot.

You find out when the build breaks on a developer laptop, and the fix is an argument about which
feature was right, held six weeks after either could be changed cheaply.

That is the failure this project addresses. Not bad prose. Not missing documents. Requirements that
are individually fine and jointly impossible, at a scale where no human reads them all at once.

## What it does

`ears-sdd` is a policy layer for [GitHub Spec Kit](https://github.com/github/spec-kit). It adds
three checks that run as a gate, and it is a single binary with no runtime dependencies.

**Requirements hold a fixed shape.** Every normative requirement is written in EARS form: one
trigger, one subject, one `shall`. A sentence that cannot be written that way is usually a sentence
hiding two requirements or an unresolved question. See
[requirements that hold their shape](concepts/ears.md).

**Words mean one thing.** Each requirement declares the vocabulary terms it is about and the single
intention it serves. Both are declared once, centrally, with definitions. An undeclared term fails
the gate. See [grounding](concepts/grounding.md).

**Contradictions surface before implementation.** Requirements can carry a small formal model of
what they assert and when. The tool merges those models across every specification and searches for
a state that satisfies the guards of two requirements whose effects cannot both hold. If it finds
one, it reports both requirements and the exact state that reaches them. See
[finding contradictions](concepts/contradictions.md).

## Why the three belong together

The third check is the one people want. It is also the one that does not work without the first two.

A contradiction search compares what requirements assert. To compare them, the tool has to know that
two features mean the same thing by the same word. If feature 5 says `toolchain` and feature 11 says
`build tools`, nothing can line them up, and the search returns a clean result that means nothing.

That is what grounding buys. Not philosophy. A join key.

The same applies to EARS. A requirement with no identifiable trigger has no guard to model, and a
requirement with two obligations has two effects wearing one identifier. The form is what makes the
sentence mechanically comparable to another sentence.

## What it does not do

It does not run your tests. It checks that a test command is declared and that the tests a
requirement names exist. Execution evidence stays with your test runner.

It does not verify your code against your requirements. It checks the requirements against each
other, and it checks that every requirement is covered by a task and a test.

It does not prove your system correct. The model is a deliberately small fragment, and any
requirement you do not model is reported as unmodelled rather than assumed consistent.

## Where to go next

If you want to install it and see it fail on your own project, start with
[install and gate a project](getting-started.md).

If you want to see the contradiction found before you install anything, read
[a contradiction, end to end](example.md). It is a real run against a two feature project, with the
output copied from the terminal.

If you want to know why the tool uses binary decision diagrams rather than an SMT solver, and why
the vocabulary layer is not an ontology in the formal sense, read
[decisions and their reasons](design/decisions.md).
