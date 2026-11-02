# Containment input and report contract

This is the implementation target for feature 009, not a currently supported command surface.
The input and report schema choices below are local to this validator.

## Activation and source selection

`[checks] containment = true` enables comparisons at `plan`, `tasks`, and `final`. It defaults to
false and is listed in `provenance.disabled_checks` when off. Enabling it requires vocabulary and
constraints; an incoherent combination produces `CHECK_DEPENDENCY`.

The existing `validate` and `status` commands read `.specify/containment.toml` under `--project`.
Missing, empty, malformed, or unknown-version comparison declarations withhold a comparison claim;
they are not successful checks of zero comparisons. No commands are launched and no input is written.

```toml
schema_version = "1.0"

[[comparisons]]
id = "alpha-revision"
kind = "revision"
reference = { feature = "specs/001-alpha", spec = "specs/001-alpha/reference.md", model = "specs/001-alpha/reference.toml", vocabulary = ".specify/reference-vocabulary.toml" }
candidates = [{ feature = "specs/001-alpha", spec = "specs/001-alpha/spec.md", model = "specs/001-alpha/model.toml", vocabulary = ".specify/vocabulary.toml" }]
```

`kind` is `revision`, `policy`, `supersession`, or `amendment`. The value `envelope` is reserved for
`010-envelope-containment`; until that feature lands, a manifest declaring it is refused with
`CONTAIN_INPUT_INVALID` and a reason naming the feature, not as an unknown kind. Reference and
candidate vocabularies are always explicit; pointing both at the same file is a deliberate choice,
not a fallback.

Each kind fixes the orientation of the comparison. The engine always computes both differences;
the kind decides which become findings:

| Kind | `candidate \ reference` (widening) | `reference \ candidate` (narrowing) | Conflicting-effect question |
|---|---|---|---|
| `revision` | `CONTAIN_WIDENED` (error) or `CONTAIN_WIDENED_DECLARED` | `CONTAIN_NARROWED` (warning) | not asked |
| `policy` | `CONTAIN_WIDENED` (error) or `CONTAIN_WIDENED_DECLARED` | none; state count recorded | asked; `CONTAIN_BASELINE_CONFLICT` (error) |
| `supersession` | none; state count recorded | `CONTAIN_SUPERSESSION_GAP` (error) | not asked |
| `amendment` | `CONTAIN_WIDENED` (error) or `CONTAIN_WIDENED_DECLARED` | none; state count recorded | not asked |

The table is code, not configuration. Each comparison record carries `kind` and
`reports = ["widening"]`, `["narrowing"]`, or `["widening", "narrowing"]` so the reader knows what
would have failed it. A suppressed direction still contributes to `status` (a `policy` comparison
whose candidate is strictly inside the policy has status `narrowed` and no findings). Model/specification paths are relative to the assessing project. A missing named model
produces a finding, while an existing model with zero entries is a valid empty model.

For an externally assessed tree, a source additionally declares `project`, relative to the assessing
project (an absolute path is also accepted). Only its `spec` path is resolved within that tree;
the model and vocabulary paths still resolve under the assessor. The source root and all read
artifacts appear in provenance. This permits `.kiro/.../requirements.md` to be read without adding
files to that tree. Unknown source fields, unsafe spec traversal outside the declared assessed
root, and duplicate comparison IDs are errors. Explicitly selected external roots are permitted;
implicit path escape is not.

Normal discovery still chooses features. A narrowed scope selects comparisons whose candidate
feature is in that scope; a supersession also matches its predecessor. Required references and
successors are then read in full and named as comparison inputs, without broadening the ordinary
requirements gate silently. An enabled run selecting no comparisons reports `CONTAIN_UNCOMPARED`.

For a policy, `reference` names the policy model, its specification document, and its vocabulary.
Keep the policy outside the project's `spec_glob` (for example `.specify/policy/spec.md` and
`.specify/policy/model.toml`), because anything inside the glob is discovered as a feature and
enters the merge gate, where a maximum policy's deliberately wide guards would be reported as
cross-specification conflicts. The policy's requirement identifiers are the targets a derogation in
a candidate model names. A candidate effect the policy does not declare is a new effect, and every
state in which the candidate asserts it is an undeclarable widening.

For a supersession, `reference` names the predecessor and `candidates` maps the live successor
features to their model sources. Resolve live descendants from Kiro's graph and verify this list is
complete; missing mappings are uncompared. Reject cycles and invalid links. Compare the union,
retaining feature-qualified identities and masking each successor by its own domain before union.
Collect partition boundaries from every successor snapshot. Report `CONTAIN_SUPERSESSION_GAP` in place of ordinary
narrowing. If a successor has unmodelled requirements, uncovered regions are uncompared, not proven
gaps. Positive coverage from available models may be shown but is not a complete-baseline verdict.

For a Spec Kit amendment, also declare `reference_requirement` and `candidate_requirement`. Both
must exist on their named sides; compare only that pair. For a Kiro source, require an
`amendment_heading` selector such as `"A1"` and use its already-parsed `CcsddRequirement::extends`
links. The selected amendment heading contributes the union of its criteria,
compared against the union of criteria under all baseline headings it explicitly extends. Do not
pretend a heading-level link maps individual criterion numbers one-to-one. Unknown targets are
uncompared. Multiple amendments require separate comparison entries with distinct IDs, so one
amendment cannot cover another's omissions. Unrelated requirements in those files do not enter
the comparison's coverage counts.

For an amendment, the expected relation is refinement: the amendment asserts its effect in a
subset of the states the extended requirement does. Narrowing is therefore not a finding. Widening
is, and a derogation in the amendment's model naming the extended requirement declares it.

Envelope sources are specified in
[010-envelope-containment](../../010-envelope-containment/contracts/envelope.md).

## Reference snapshots

The validator reads reference files from the paths the manifest names and never resolves a revision
from version control. A snapshot is produced by the project when it approves a revision:

```sh
cp specs/001-alpha/model.toml specs/001-alpha/reference.toml
cp specs/001-alpha/spec.md    specs/001-alpha/reference.md
git add specs/001-alpha/reference.toml specs/001-alpha/reference.md
```

The reference vocabulary is snapshotted the same way when a term the model depends on changes.
Committing the snapshot with the approval gives it a diff, an author, and a review. A
`baseline` subcommand that performs this copy is a candidate follow-up, not part of this feature.

## Model extensions and derogations

Retain the existing model schema and add the optional table already used by the pending fixtures:

```toml
[derogations.REQ-001]
reference = "REQ-001"
reason = "This broader permission was agreed in review."
```

Keys name candidate requirements in that model. `reference` names a requirement in the comparison's
single reference source. The same-effect reference target bounds what this exception can waive.
Unknown candidate IDs, malformed entries, and short reasons produce `CONTAIN_DEROGATION_INVALID`;
unknown reference IDs produce `CONTAIN_DEROGATION_UNKNOWN`. A target asserting another effect is
invalid. No exception can waive a genuinely new effect lacking a same-effect reference requirement.

For candidate contributor `q`, test whether `q AND NOT reference(effect(q))` is nonempty under
the shared encoding and side domains. A widening is declared only if every contributor with such
a region has a valid derogation. Evaluate usage over the whole region, not only the chosen witness.
Unused entries produce `CONTAIN_DEROGATION_STALE` only after that question completes. Duplicate
coverage does not remove findings or inflate counters. Exceptions never suppress baseline conflicts.

## Questions and witnesses

Construct one question per effect in the union of declared effect IDs, including empty regions.
Under the `policy` kind only, construct one additional question per cross-side conflicting-effect
pair using the symmetric union of conflict declarations. Budgets apply independently to those
questions. Under every other kind no conflicting-effect question is constructed, and the report's
effect count is unaffected either way.

Use all guard constants from both sides plus domain boundary cuts. Shared values are sorted by
canonical term ID, then false/true, lexicographic enum member, or integer lower bound. Saturating
state counts are computed before constructing any BDD or evaluating a guard. Overflow is refusal,
even if the configured budget is `u64::MAX`. No state of an over-budget question is evaluated.
Equality with the budget is allowed. A declared effect without dependencies ranges over one empty
assignment, so budget zero refuses that question. Two valid empty models declaring no effects and
no unmodelled requirements can be equal without asking any effect question; missing models cannot.
An unknown reference declaration set does not establish that a candidate effect is new.

An effect witness is the least satisfying assignment in this value order. Integer values carry
`representative`, `low`, and `high`; the representative is the lower bound. Confirm both the
positive attributed guards and the absence of the opposing activation region with the independent
evaluator before reporting. A mismatch produces `MODEL_INTERNAL` and withholds the result.

Report all positive contributors at the witness in sorted source-qualified order. A conflict
witness names contributors on both sides. Invalid effects do not enter arithmetic as empty regions;
unaffected effects can still be checked and reported individually.

## Output

Keep report schema `1.0` and add optional containment fields only when this layer runs, following
the existing optional model and Kiro fields. Older reports retain their shape. In addition to
`summary.effects_compared`, report `summary.effects_uncompared` and `summary.uncompared` (requirements
that did not participate). Effects count per comparison, including declared-but-empty regions;
conflicting-effect questions do not inflate the effect count. Do not equate an effect count with a
requirement count. A zero-model comparison still lists its unmodelled requirement identities.

Add a top-level `comparisons` array sorted by comparison ID. Each record has `id`, `kind`,
`reports`, `reference`, `candidates`, `status`, `complete`, `effects_compared`, `effects_uncompared`,
and `requirements_uncompared`. `reports` lists the directions the kind turns into findings; a
direction that was computed and suppressed is visible through `status` and the per-effect state
counts. Sources identify the actual model/definition, spec, vocabulary and
declared source root. No timestamp or local checkout path is treated as the identity of a revision.
Caller-supplied source files are the reference; the validator does not claim a Git revision it did
not resolve.

Containment findings use existing `Finding.detail` for `comparison`, `effect`, `witness`,
`candidate_requirements` and/or `reference_requirements`. Refusals carry `reason` and affected
effects; budget findings also carry `states`, `budget` and `largest_contributors`. Preserve the
existing singular `requirement` as the first attributed local ID, with the full identities in
detail. Deduplication and stable sorting include comparison ID, effect and conflict partner;
otherwise multiple tool/transition findings for one phase would collapse.

| Finding | Severity | Meaning |
|---|---|---|
| `CONTAIN_REFERENCE_MISSING` | error | Requested reference absent or unreadable |
| `CONTAIN_INPUT_INVALID` | error | Malformed comparison/source model or unsupported input version |
| `CONTAIN_WIDENED` | error | An undeclared widening |
| `CONTAIN_WIDENED_DECLARED` | advisory | Every widening contributor covered by a valid derogation |
| `CONTAIN_NARROWED` | warning | Dropped activation region for review |
| `CONTAIN_EFFECT_NEW` | advisory | Candidate declares an effect absent from the reference; widening remains independently checked |
| `CONTAIN_BASELINE_CONFLICT` | error | Opposing effects active across sides; `policy` kind only |
| `CONTAIN_DEROGATION_UNKNOWN` | error | Unknown reference target |
| `CONTAIN_DEROGATION_INVALID` | error | Invalid candidate target, effect or reason |
| `CONTAIN_DEROGATION_STALE` | warning | Completed question has no widening for this exception |
| `CONTAIN_BUDGET_EXCEEDED` | error | Question refused before evaluation |
| `CONTAIN_VOCAB_DRIFT` | error | A referenced term lacks a compatible identity on the other side |
| `CONTAIN_UNCOMPARED` | warning | An input, effect or requirement did not participate; no unqualified claim |
| `CONTAIN_SUPERSESSION_GAP` | error | Fully modelled successor union loses predecessor behavior |

The envelope findings (`CONTAIN_UNDECLARED_TERM`, `CONTAIN_GUARD_INVALID`,
`CONTAIN_UNMODELLED_CONSTRUCT`, `CONTAIN_REACHABILITY_OMITTED`, `CONTAIN_DEFINITION_UNREADABLE`)
are defined by `010-envelope-containment` and share this namespace.

Retain existing gate exit semantics: `Report.ok` is false for errors; warnings remain visible without
becoming errors. `status` always exits zero. A comparison's withheld status does not become
containment merely because the process succeeded. Human output states the reference, kind,
reported directions, status, coverage and witnesses. Correct the general gates documentation's current
claim that every warning fails the process; it disagrees with `lib.rs` and existing fixtures.

## Corpus assertions

When the expected report contains containment comparisons, normalize and compare the full comparison
records and containment `detail` fields as well as the existing finding identity. Normalize only
genuinely variable absolute roots/timestamps, never effect IDs, witnesses, source filenames, or
coverage. Assert process status independently from normalized JSON `ok`. Existing cases without
containment retain their current normalization. Repeat-run, independent-evaluator and read-only
properties remain executable Rust tests; a JSON fixture alone cannot establish those properties.
