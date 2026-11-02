# Envelope sources, interchange format 1.0, and adapters

This is a validator input format, not a claim to parse a third-party runtime's native workflow.
Everything not stated here (manifest activation, budgets, witnesses, derogation validation, report
shape, corpus assertions) is in
[009-baseline-containment/contracts/containment.md](../../009-baseline-containment/contracts/containment.md).

## Source shape in the manifest

```toml
[[comparisons]]
id = "bugfix-envelope"
kind = "envelope"
reference = { definition = ".specify/envelope.reference.json", vocabulary = ".specify/envelope-vocabulary.toml" }
candidates = [{ definition = ".specify/envelope.json", vocabulary = ".specify/envelope-vocabulary.toml", format = "ears-sdd-envelope/1.0" }]
```

An envelope source replaces `spec` and `model` with `definition`. `vocabulary` is required and names
the envelope vocabulary; a source without it produces `CONTAIN_INPUT_INVALID` and the comparison is
withheld. `format` is optional and defaults to `ears-sdd-envelope/1.0`, the interchange format
below. A `format` with no registered adapter produces `CONTAIN_DEFINITION_UNREADABLE` with the
format name in `detail.reason`. There is exactly one candidate. An optional `model` names a sidecar
model file holding `[derogations.PHASE-<name>]` entries; nothing else in that file is read.

The envelope vocabulary is an ordinary vocabulary file. It is validated for structure
(`VOCAB_INVALID`, `VOCAB_CYCLE`), and `TERM_UNUSED` is never raised over it because nothing tags
its terms. It is read by the containment layer only; the vocabulary gate does not discover it.

## Interchange format 1.0

```json
{
  "schema_version": "1.0",
  "id": "bugfix",
  "initial": "planning",
  "phases": {
    "planning": {
      "allowed_tools": ["Read", "Grep"],
      "transitions": [{"to": "implement", "when": "test-result == 'fail'"}]
    },
    "implement": {
      "allowed_tools": ["Read", "Edit"],
      "transitions": [{"to": "planning", "when": "test-result == 'pass'"}]
    }
  }
}
```

`schema_version`, `id`, `initial`, and `phases` are required. `phases` is nonempty, and `initial`
names a phase. Phase names match `[a-z][a-z0-9_-]*`, keeping derived identifiers unambiguous.
`allowed_tools` is a required list of nonempty strings; names are case-sensitive and duplicates
are invalid. An empty list is valid. `transitions` defaults to an empty list. Each transition names
an existing `to` phase. Missing `when` means unconditional; a supplied string follows `guard::parse`
and the existing model type rules. Empty or duplicate JSON object keys are refused where they
would make a phase or structural field ambiguous; do not let last-value-wins erase a policy entry.

The reader creates a reserved enum term `envelope-phase` from the phase names. An envelope
vocabulary collision with this name is invalid. It derives requirement `PHASE-<name>` for source
attribution. The internal representation permits multiple effect contributions from that phase; it
is not serialized through `ModelFile.requirements`, whose entries each support only one effect.

| Input | Effect identity | Guard |
|---|---|---|
| Tool `Read` permitted by phase `planning` | `tool:Read` | `envelope-phase == 'planning'` |
| Transition `planning` → `implement` | `transition:planning:implement` | Source phase AND declared guard |

Different source phases use different transition effects. Multiple transitions with the same
source/destination use an OR of their guarded regions. Namespaces prevent a tool called
`transition:planning:implement` from colliding with an actual transition effect.

Guard terms other than `envelope-phase` resolve against the source's envelope vocabulary. An
invalid or undeclared term withholds the entire affected transition effect, including other
branches to that destination. It does not disable unrelated transitions or phase tool permissions.
An enum literal outside its declared domain is a type failure, not an always-false guard.

Unknown fields are recorded by side and JSON-pointer path at every nesting level, including phases
with no tools. A `fork`, for example, is named rather than interpreted. Those fields mark the full
envelope comparison incomplete while allowing results over represented permissions. Invalid types
in supported fields, missing required fields, invalid endpoints, malformed JSON, duplicate keys,
and unknown versions produce `CONTAIN_DEFINITION_UNREADABLE` and withhold the whole definition.
Unknown fields do not license partial interpretation of an unknown schema version.

Every completed comparison states `reachability_computed: false` and emits
`CONTAIN_REACHABILITY_OMITTED`. All phases participate, including phases not reachable from initial.
The recognized `initial` field is retained in source metadata; its reachability consequences are
covered by that explicit omission, not interpreted as a tool permission. Phase-local equality is not
evidence of reachability, liveness, deadlock freedom, or runtime enforcement.

## Adapters

An adapter is a function from bytes to `(EnvelopeModel, Vec<UnmodelledConstruct>)` or an
unreadable-definition error, registered under a format name of the shape `<name>/<version>`. The
registry is a fixed table in code; there is no plugin path and no configuration that adds to it.

| Format | Adapter | Status |
|---|---|---|
| `ears-sdd-envelope/1.0` | built in; the reader above | this feature |
| a state-machine guardrail runtime's native file | to be written from a checked-in sample | blocked on input |

The comparison record's source entry carries `adapter = { id, version }`. An adapter's version is
bumped whenever its mapping changes, because two runs over the same file under different mappings
are not comparable and the report has to make that visible.

A native adapter maps the runtime's states to phases, its per-state tool allowlists to
`allowed_tools`, and its guarded transitions to `transitions` with guards rewritten into the
validator's guard grammar. Every native construct with no counterpart (approval gates, model
routing per state, fork/join, interrupts, edit limits) is returned as an unmodelled construct with
its native path. Nothing is inferred; a guard the adapter cannot rewrite withholds the transition
and names the guard.

## Findings owned by this feature

| Finding | Severity | Meaning |
|---|---|---|
| `CONTAIN_UNDECLARED_TERM` | error | Envelope transition guard references a term the envelope vocabulary does not declare |
| `CONTAIN_GUARD_INVALID` | error | Unsupported envelope guard syntax or invalid type/value |
| `CONTAIN_UNMODELLED_CONSTRUCT` | warning | A definition field or native construct was not modelled |
| `CONTAIN_REACHABILITY_OMITTED` | advisory | Phase-local comparison did not compute reachability |
| `CONTAIN_DEFINITION_UNREADABLE` | error | Definition absent, undecodable, malformed, unsupported version, or unknown format |

`CONTAIN_WIDENED`, `CONTAIN_WIDENED_DECLARED`, `CONTAIN_NARROWED`, `CONTAIN_EFFECT_NEW`,
`CONTAIN_INPUT_INVALID`, `CONTAIN_BUDGET_EXCEEDED` and `CONTAIN_UNCOMPARED` keep their 009
meaning. Envelope records additionally carry `reachability_computed = false`, sorted unmodelled
paths, and the adapter identity per source.
