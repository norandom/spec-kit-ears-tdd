# Data model: Baseline Containment

Extend the existing crate; these records do not imply a public trait hierarchy or new persistence.

| Entity | Data and relationships | Validation |
|---|---|---|
| Comparison request | Stable ID, kind, reference source, candidate source(s); optional requirement pair for an amendment | Unique ID; kind is one of `revision`, `policy`, `supersession`, `amendment`; one reference; nonempty candidates; multiple candidates only for supersession |
| Source | Logical feature, specification path, model path, vocabulary path; assessed project root when external | Paths resolved before loading; source origin retained; no inferred vocabulary fallback |
| Model snapshot | Declared requirement IDs, parsed requirements, effect declarations, canonical terms, invalid effects, unmodelled IDs | Validate against that side's specification and vocabulary before comparison |
| Canonical term | Stable ID, compatible domain kind, source-specific domain, declared replacement chain | Missing/cyclic replacement or incompatible kind produces drift; no label matching |
| Effect region | Effect ID, source-qualified contributors, union of individually domain-masked source regions | Empty contributor list means false; invalid contributor means incomplete, not false |
| Shared question | Sorted variables and finitized values, side masks, reference/candidate regions, size, budget | Common encoding; checked size before evaluation or BDD allocation |
| Derogation | Candidate requirement key, reference requirement target, reason | Same effect, known IDs, trimmed reason length >= 10; usage only decided on a completed question |
| Witness | Ordered term-to-value assignment, integer representative and interval, contributing requirement IDs | Re-evaluate attributed guards, side membership, and the negative side of the difference |
| Comparison result | Reference/candidate provenance, direction status, completeness, effect results, excluded requirement identities | No whole-comparison containment assertion from incomplete input |
| Kind orientation | Per kind: which of the two directions produce findings, and whether the conflicting-effect question is asked | Fixed table in code, never configurable; recorded on every comparison result |

Source-qualified requirement identity includes side, source feature and local identifier. Equal
local IDs in different successors remain distinct. An unmodelled requirement is counted once per
side/source in a comparison, even if several effects depend on the affected input. Project summary
counts deduplicate those identities across repeated comparisons of the same source.

Comparison states are `equal`, `narrowed`, `widened`, `changed`, `indeterminate`, and `uncompared`.
The state describes the computed differences and is the same under every kind; the kind decides
which of those differences also became findings. A `policy` comparison in state `narrowed` has no
findings and passes; a `supersession` comparison in state `widened` likewise.
The first four require every relevant question to complete and no unresolved source coverage.
`changed` means both directions have a nonempty difference, possibly on different effects.
`indeterminate` records a budget refusal; `uncompared` records invalid, missing or incomplete input
and takes precedence if both occur. Completed per-effect results stay visible alongside either.

Each result separately records baseline conflicts and declared widening decisions. Declaring a
widening does not rename it `equal` or `contained`. A new declaration with an empty activation
region is recorded separately even when the regions are equal.
