# Configuration

Every file here is optional except the first. A project that declares no vocabulary, no intentions,
and no models gets the EARS and traceability gates and hears nothing from the rest.

That is deliberate. The layers are meant to be adopted one at a time.

## `.specify/ears-sdd.toml`

Project policy. Written by `ears-sdd init`, then edited by you.

```toml
spec_glob = "specs/*/spec.md"
traceability_file = "traceability.toml"
require_test_files = true

# Checked for existence, never executed by the validator.
test_command = "cargo nextest run"

# Requirement prose and identifiers must not appear under these roots.
production_roots = ["src"]
test_roots = ["tests"]
source_extensions = [".rs"]

# Maximum states in one component before the tool refuses to search it.
state_space_budget = 1000000
```

| Key | Meaning |
| --- | --- |
| `spec_glob` | Where specifications live |
| `traceability_file` | Name of the per feature mapping file |
| `require_test_files` | Whether a named test file must exist on disk |
| `test_command` | The project's real test command, checked as declared at the final gate |
| `production_roots` | Scanned for leaked requirement prose and identifiers |
| `test_roots` | Allowed to name requirement identifiers, for traceability |
| `source_extensions` | Which files the separation scan reads |
| `state_space_budget` | States per component before the search is refused |

### Turning layers off

```toml
[checks]
traceability = true   # verification mapping, test selectors, manual rationales
vocabulary = true     # declared terms, the tags requirements carry, the intentions they serve
constraints = true    # constraint models, within a specification and merged across all of them
tasks = true          # every requirement covered by a task before implementation
separation = true     # requirement prose and identifiers kept out of production code
```

Every one defaults to true, so omitting the table changes nothing.

They exist because adoption is incremental. A project can gate EARS form on day one and wire
traceability, a vocabulary, and constraint models in whatever order suits it, rather than choosing
between all of it and none of it. A brownfield codebase with no test mapping yet can run
`traceability = false` and still get the requirement checks that cost nothing to adopt.

!!! warning "Switching a layer off never makes the run quieter about it"

    The disabled set is printed on every run, passing or failing:

    ```text
    EARS/TDD plan gate: PASS
    Scope: specs/*/spec.md (all matching specifications)
    Disabled: traceability, vocabulary (not checked)
    Features: 12  Requirements: 400  Errors: 0  Warnings: 0
    ```

    and recorded in the machine-readable report as `provenance.disabled_checks`.

    This is the same rule as printing the scope, for the same reason. A gate that can be narrowed
    silently produces a passing result indistinguishable from a checked one, which is the failure
    this tool exists to prevent. Turning a layer off is a decision, and a decision belongs in the
    evidence.

A misspelt switch is a parse error rather than a setting that quietly does nothing, because
believing a layer is off when it is on is its own kind of wrong.

## `traceability.toml`

One per feature, beside its `spec.md`.

```toml
schema_version = "1.0"

[requirements.REQ-001]
verification = "automated"
tests = ["tests/test_records.py::test_valid_record_is_persisted"]
tags = ["exploit-protection", "mitigation-enforced"]
intent = "reduce-attack-surface"

[requirements.REQ-002]
verification = "manual"
rationale = "Requires a physical device unavailable in automated test environments."
```

`verification` is `automated` or `manual`. Manual is permitted only with a concrete rationale.

The validator checks that a named test file exists and that the selector names a test that is
actually present. It does not claim the test passed. Execution evidence stays with your test runner.

`tags` and `intent` are optional and enable the grounding layer.

## `.specify/vocabulary.toml`

Project terms. A feature may also declare `vocabulary.toml` beside its `spec.md` for terms only it
uses.

```toml
schema_version = "1.0"

[terms.mitigation-enforced]
label = "Mitigation enforced"
definition = "Whether the arbitrary-code-guard mitigation is enforced for a process."
domain = { kind = "bool" }

[terms.operating-mode]
label = "Operating mode"
definition = "Which service mode the workstation is running in."
domain = { kind = "enum", values = ["normal", "maintenance", "degraded"] }

[terms.queue-depth]
label = "Queue depth"
definition = "Number of pending operations awaiting processing."
domain = { kind = "int", min = 0, max = 10000 }

[terms.exploit-protection]
label = "Windows Exploit Protection"
definition = "The Windows mitigation subsystem configured per-system and per-program."
domain = { kind = "entity" }
broader = ["workstation-hardening"]
alt_labels = ["exploit guard"]
```

Domains are `bool`, `enum`, `int` with bounds, and `entity`. A definition is required and may not be
empty.

Declaring the same identifier in two places with different domains is an error. That is precisely
the drift the file exists to prevent.

## `.specify/intentions.toml`

Why requirements exist, and which goal wins when two collide.

```toml
schema_version = "1.0"

[intentions.reduce-attack-surface]
statement = "Untrusted code cannot be generated or executed at runtime."
rationale = """
Longer explanation, written while the reason is still known.
"""

[[precedence]]
over = "reduce-attack-surface"
under = "native-toolchain-works"
reason = "A weakened mitigation is worse than a documented per-program exclusion."
```

`over` wins against `under`. Precedence is pairwise and local, never a global ranking. Leaving a pair
unordered records that nobody has decided, which is more honest than inventing an order. A cycle is
reported rather than ignored.

## `model.toml`

Optional, per feature, beside its `spec.md`. A requirement with no entry is recorded as unmodelled
rather than assumed consistent.

```toml
schema_version = "1.0"

[effects]
block_dynamic_code = { conflicts_with = ["permit_dynamic_code"] }
permit_dynamic_code = {}
emit_audit_entry = {}

[requirements.REQ-001]
when = "mitigation-enforced"
then = "block_dynamic_code"

# No `when` means always. That is not the same as a guard that happens to be true.
[requirements.REQ-002]
then = "emit_audit_entry"
```

The conflict relation is symmetric and read that way regardless of which side declares it.

### The guard language

```text
and  or  not  ( )
==  !=  <  <=  >  >=
a bare term name means that boolean term is true
literals are integers, single-quoted strings, and true / false
```

Anything outside this is rejected rather than interpreted.

Comparisons are against literal constants only. That restriction is what lets a bounded integer be
split into a few regions at the constants its guards mention instead of enumerated across its whole
range. Comparing two terms against each other needs arithmetic the fragment does not have, and the
parser says so:

```text
MODEL_GUARD_INVALID: `queue-depth` is a term; comparisons are against literal constants only.
```

## Separation exemptions

Sometimes production code legitimately cites a requirement, usually where a contract enforces one.
Mark the line and give a reason of at least ten characters:

```rust
// ears-sdd:allow-requirement-id: citing the requirement this contract enforces
/// The contract from REQ-026: a conditional requirement belongs to exactly one component.
```

Exempted findings are reported as advisories with their reasons. A stale or unused exemption is
reported too, so the list does not rot.
