# Grounding: vocabulary and intentions

"Ontology" is an intimidating word for a plain idea: writing down what your terms mean, once, in a
place everything else refers to.

You do not need description logics, a reasoner, or OWL to get the benefit. This project ships the
smallest version that pays for itself, and it is worth being precise about what that version does
and does not claim.

## The problem it solves

Every project accumulates synonyms. One feature says `toolchain`, another says `build tools`, a
third says `compiler environment`. Everyone knows they are the same thing. Nothing in the system
does.

That is survivable while humans read the specifications, because humans resolve synonyms without
noticing. It stops being survivable at two points.

The first is scale. At 400 requirements across 12 features, nobody holds the synonym table any more,
and two requirements about the same condition can disagree without anyone connecting them.

The second is automation. Any tool that compares requirements has to know that two features mean the
same condition. Given `toolchain` in one file and `build tools` in another, a contradiction checker
finds nothing and reports a pass. That is worse than not running it, because now you have evidence.

## Three questions, three layers

The model this project uses separates three things people usually mix:

| Layer | Question | Where it lives |
| --- | --- | --- |
| Tags | What is this requirement about? | `vocabulary.toml`, referenced per requirement |
| Intentions | Why does this requirement exist? | `intentions.toml`, one per requirement |
| EARS | What must the system do? | The requirement sentence |

Keeping them apart is what makes each useful. The requirement says what. The tag says which concept.
The intention says which goal. A conflict between two requirements is then a question about goals,
not about wording.

## Vocabulary

A term has a label, a definition, and a domain:

```toml
[terms.exploit-protection]
label = "Windows Exploit Protection"
definition = "The Windows mitigation subsystem configured per-system and per-program."
domain = { kind = "entity" }
broader = ["workstation-hardening"]
alt_labels = ["exploit guard"]
```

Four things are doing work here.

**The definition is mandatory and may not be empty.** A term with a label and no definition is a tag
pretending to be a concept. That is how a vocabulary rots into two hundred near synonyms, each
obvious to the person who added it.

**The domain says what kind of thing it is.** `entity` for a thing, `bool` for a yes or no condition,
`enum` for one of a fixed set, `int` for a bounded number. This is declarative. It carries no solver
by itself. It is what a constraint model later ranges over, and it is what lets the gate reject a
requirement that compares an enumeration against `true`.

**`broader` builds a hierarchy.** `exploit-protection` is a kind of `workstation-hardening`. The
relation is transitive when the tool reads it, so a query for hardening finds exploit protection
without anyone restating the link.

**`alt_labels` records the synonyms you already have.** This is the honest place for `exploit guard`.
It resolves to one concept instead of becoming a second one.

A term can also be retired. References still resolve, and each one reports a warning naming the
replacement, so a rename is visible without being a hard stop:

```toml
[terms.exploit-guard]
label = "Exploit Guard"
definition = "Superseded name for the Windows Exploit Protection subsystem."
domain = { kind = "entity" }
deprecated = true
replaced_by = "exploit-protection"
```

### The rule that does most of the work

An undeclared tag fails the gate.

That single rule turns vocabulary drift from a code review argument into a deterministic failure. It
is not subtle and it is not clever. It is the reason the vocabulary stays current, because the only
way to use a new word is to define it.

## Intentions

An intention is why a requirement exists, recorded while the reason is still known:

```toml
[intentions.reduce-attack-surface]
statement = "Untrusted code cannot be generated or executed at runtime."
rationale = """
Arbitrary code guard is the mitigation that stops an exploit turning a read primitive into
execution. Disabling it per-process is the documented escape hatch, and every escape hatch that is
not written down becomes permanent.
"""
```

A requirement references at most one intention. That restriction is deliberate and it is load
bearing. When two requirements contradict, the tool asks whether the goals involved have a unique
winner under the declared precedence. That question stops meaning anything if one requirement serves
three goals at once.

### Precedence is where the value is

```toml
[[precedence]]
over = "reduce-attack-surface"
under = "native-toolchain-works"
reason = "A weakened mitigation is worse than a documented per-program exclusion."
```

Precedence is declared pairwise and locally, never as a global ranking. Leaving a pair unordered is
allowed, and often correct: it records that nobody has decided yet, which is more honest than
inventing an order. The relation must stay acyclic, and a cycle is reported rather than ignored,
because a cycle resolves nothing.

When two requirements contradict, this is what separates the three outcomes:

- The goals have a unique winner. The conflict is a recorded trade-off. Advisory, not a failure.
- The goals have no declared order. Nobody has decided. The gate fails and names the pair to order.
- Both requirements serve the same goal. No precedence can help, because a goal cannot outrank
  itself. One of the two requirements is simply wrong.

Without intentions, a tool can only tell you that two requirements clash. With them, it can tell you
whether a human already thought about it.

## Grounding an AI agent

The practical effect on an agent is narrower than the phrase suggests, and more useful.

An agent asked to write a requirement about the toolchain will pick a plausible word. Left alone it
picks a different plausible word next session. The vocabulary gives it a closed set to draw from and
a gate that rejects anything outside the set, so drift is caught at the point it happens rather than
discovered later as an inconsistency.

The definitions matter more than the labels here. An agent reading `toolchain-building: whether a
native toolchain is compiling or running a just-in-time backend` has the boundary of the concept,
including the part a human would have assumed. That is the difference between a tag and a term.

None of this makes an agent correct. It makes an agent consistent, and it makes the specific failure
of quiet synonym invention impossible rather than unlikely.

## Starting from a vocabulary that already exists

Extracting candidates from prose assumes there is prose. On a new project there is not, and in many
domains there is something better: a vocabulary someone has already agreed.

```sh
ears-sdd vocab-import security-concepts.ttl > .specify/vocabulary.toml
```

The importer reads a SKOS concept scheme and maps it onto this format. `skos:prefLabel` becomes the
label, `skos:definition` the definition, `skos:broader` the hierarchy, `skos:altLabel` the synonyms,
`owl:deprecated` and `dcterms:isReplacedBy` the retirement fields. A `skos:narrower` assertion is
read as `broader` on the other concept, since SKOS treats them as inverses and a published
vocabulary may state either. Anything that is not a `skos:Concept`, such as a scheme or a
collection, does not become a term.

Two things do not survive, and both are deliberate.

**Definitions the source did not carry arrive empty**, which fails the gate until someone writes
one. An imported term nobody has read grounds nothing, exactly like a scaffolded one.

**Domains default to `entity`.** SKOS says which concepts exist, not which of them are booleans or
what values they range over. Turning an imported vocabulary into one a guard can be built from is
the work, and the tool does not pretend to do it for you.

Going the other way publishes what you have:

```sh
ears-sdd vocab-export --all --base https://id.example.org/vocab > vocab.ttl
```

The domain travels in a private namespace, so a consumer that does not understand it ignores it and
one that does can round-trip without loss. The round trip is tested: the twelve-feature example's 28
terms export, import, and still produce exactly the same contradiction across its 383 requirements.

## Why the vocabulary is not authored in SKOS

The bridge is deliberately a bridge rather than a migration.

Turtle reviews badly in a pull request, and a gate has no use for a triple store. More to the point,
SKOS has no place for the field the constraint model depends on. A term's domain is a statement
about the values a variable ranges over, which is a datatype constraint rather than a fact about a
concept. Expressing it means SHACL or a private extension, and at that point the file is no longer
standard SKOS anyway.

So TOML stays the authoring format, and SKOS is how the vocabulary arrives and how it leaves.

## What this is not

It is not an ontology in the formal sense, and the project does not claim to be one.

There is no reasoner, and none is needed. Subsumption over `broader` is transitive closure, which is
a graph walk. Equivalence is `alt_labels`. The standard vocabulary property this mirrors, SKOS
`broader`, is itself explicitly non-transitive, so computing the closure here is a reading of the
standard rather than an extension of it.

If you already run a real ontology, this layer is not a replacement and does not want to be. It is
the part of the idea that a small team can adopt in an afternoon and will still be maintaining in a
year.

## Next

[Finding contradictions](contradictions.md) covers what the tool does once the words are pinned
down.
