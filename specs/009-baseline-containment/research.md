# Research: Baseline Containment

Local source review, 2026-09-18. These are design decisions, not implementation claims.

## Reuse the finite model and both decision procedures

**Decision:** Build one shared `model::Component` per effect question. Form each side's activation
region with `Guard::Or`, conjoining each source's own domain membership before unioning sources.
Use `bdd::Encoding` to decide the two
differences; extend `tests/differential.rs` to check them independently with `enumerate::evaluate`.

**Rationale:** `model::finitize`, `Component::state_space`, `Encoding::encode`, `Encoding::domain`,
and `enumerate::verify_assignment` already provide the necessary pieces. A single encoding is
essential: BDDs from independently ordered variable sets cannot safely be combined.

**Alternatives:** A solver dependency or a second guard language buys nothing for this fragment.
`analysis::decompose` and `validate_merged` answer a different question; neither computes the union
of same-effect guards across a pair of independently sourced snapshots.

## Retain each side's domain

**Decision:** Sort canonical term IDs; sort enum values; order booleans false then true and integer
regions by lower bound. Split integers at guard discontinuities from both sides and at each side's
domain boundaries. Keep membership masks for each source, including masks for unguarded effects over
the other side's dependencies. Exclude a shared-hull interval outside both sides.

**Rationale:** A union domain without masks would make a reference permission hold on a newly added
candidate enum member. Unioning only guard constants also misses a changed integer bound. In a
successor union, one successor's unguarded effect cannot inherit another successor's wider domain.

**Alternatives:** Withholding all domain changes would miss the explicit enum and bound-change
cases. Comparing independently finitized models can hide a real widening.

`model::split_points` and `regions_cover_domain` currently add one to `i64` values unchecked. Before
adding endpoint cuts, cover `i64::MIN` and `i64::MAX` and use checked arithmetic at those boundaries.
Project-local changes to finitize must retain the existing contradiction corpus results.

## Validate snapshots before using them

**Decision:** Separate parsing/type validation from analysis sufficiently to return a validated
snapshot plus affected invalid effects. Load the reference vocabulary independently. Reuse
`analysis::conflict_pairs` and guard type checks after making their needed portions crate-visible.

**Rationale:** `analysis::validate` currently treats every failed model read as absence; its type
findings do not remove invalid guards. Copying that orchestration would permit false empty regions.
`vocabulary::Term` already has `deprecated` and `replaced_by`, but there is no guard canonicalizer.

**Alternatives:** Parsing twice through the entire existing validator duplicates findings and loses
reference provenance. Guessing term identity from labels cannot establish equivalence.

## Explicit comparisons, no revision checkout

**Decision:** Read `.specify/containment.toml` from the assessing project when the new optional check
is enabled. It identifies reference and candidate artifacts and their vocabulary/specification
sources. Ordinary commands remain `validate` and `status`; this feature does not invoke Git or fetch
revisions. A caller supplies an exported snapshot or an existing checkout.

**Rationale:** The validator stays read-only and supports a Kiro tree whose models are held elsewhere.
Explicit comparisons support one-spec adoption without silently trying to baseline every feature.

**Alternatives:** Auto-discovering every `reference.toml` or envelope file cannot distinguish an
intended comparison from abandoned files. New revision-selection CLI flags are unnecessary here.

## Derogations apply to contributors, not witness coincidence

**Decision:** Retain the fixture's `[derogations.<candidate-id>]` shape, with a same-effect reference
target and trimmed reason of at least ten Unicode characters. Check every candidate contributor
over the whole widening. Defer staleness where no complete answer was possible.

**Rationale:** A reference guard is false at a widening witness by definition. Testing whether that
guard holds there would make all derogations ineffective. Testing only the displayed witness would
allow an exception on one branch to conceal an unrelated branch.

**Alternatives:** An effect-wide exception lacks the requirement-level accountability in the spec.
Derogations never suppress cross-baseline conflicts or excuse unreadable input.

## Kiro links and partial unions

**Decision:** Expose the successor links already found in `kiro::evaluate`; retain edges through
superseded nodes and resolve live descendants with a visited set. Sidecars live in the assessing
project. Kiro amendments use the explicit `CcsddRequirement::extends` heading links already parsed
by `ccsdd`; compare the union of criteria under an amendment heading with the union under its named
baseline headings. Spec Kit comparisons can supply explicit requirement pairs.

**Rationale:** The current Kiro evaluation discards its local `links` and retains only direct live
links. It currently discards parsed `extends` relationships. Neither is yet a content comparison.

**Alternatives:** Re-parsing Kiro links in containment risks a different baseline. Missing successor
models cannot be treated as empty: an apparent gap could be covered by an unmodelled successor.

## The kind decides direction, the engine decides difference

**Decision:** Compute both directional differences for every comparison. Apply a fixed, in-code
orientation table keyed by the manifest `kind` to decide which difference produces findings and
whether the conflicting-effect question is asked. Record the kind and its reported directions in
every comparison result.

**Rationale:** A maximum policy is an upper bound; reporting a candidate for staying inside it
would warn on every feature. An amendment refines one requirement; narrowing is its purpose. A
supersession already had this rule (a gap is narrowing in that orientation). Making the rule per
kind and explicit removes the last place where the reader had to infer what a finding meant.

**Alternatives:** Inferring the kind from the paths involved (a reference outside the spec glob
is a policy) is guessable and wrong for a project that keeps snapshots outside the glob too. A
configurable orientation invites a project to switch off the direction it does not like.

## Reference snapshots are files, produced at approval

**Decision:** The manifest names the reference model and the specification it was read against
as paths in the assessing project. The validator never runs Git or resolves a revision. The
quickstart documents the two-file copy that produces a snapshot.

**Rationale:** The validator stays read-only and dependency-free. A snapshot that is a committed
file has a diff, an author, and a review, which a resolved revision hash does not. This is also how
the OpenShell work maintains its maximum policy: as an artifact, not as a lookup.

**Alternatives:** A `baseline` subcommand that performs the copy is a reasonable follow-up. It is
not in this feature because it writes into the project, and the single existing writing command
(`init`) exists only to install Spec Kit components.

## Envelopes moved to their own feature

**Decision:** The workflow interchange format, the envelope reader and its adapter live in
`010-envelope-containment`. This feature's contract lists the kind names 010 will add so the
manifest parser refuses them with a reason rather than as unknown.

**Rationale:** The format had no producer, the transition guards named terms the product
vocabulary would flag as unused, and the reader was fourteen requirements of a fifty-requirement
feature for what is mostly a per-phase set difference. Splitting lets the P1 engine ship and lets
the envelope work be judged against a real runtime's file.

## Evidence must retain the claim

**Decision:** Extend the corpus normalizer for structured containment details and comparison
provenance when containment runs. Keep old cases byte-for-byte stable when the check is disabled.
Add runtime tests for stable witnesses, both-root read-only behavior, and preflight budget refusal.

**Rationale:** `tests/conformance.rs::normalize` currently drops `Finding.detail`, all `features`,
and most provenance. A fixture naming `CONTAIN_WIDENED` currently checks neither its witness nor
which reference was compared. `Finding::dedupe_key` also omits effect and comparison identity.

**Alternatives:** A present `expected.json` only satisfies the mapping's file-existence check.
It is not evidence that the behavior ran. Pending cases remain explicitly unexecuted.
