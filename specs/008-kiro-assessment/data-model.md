# Data model: Kiro Project Assessment

## SpecLocation (existing, extended)

The discovery record for one specification file.

| Field | Type | Notes |
|---|---|---|
| `path` | path | `requirements.md` for Kiro; `spec.md` for Spec Kit |
| `feature` | string | Kiro: directory basename. Spec Kit: relative parent of `spec.md` (unchanged) |
| `family` | `speckit` \| `kiro` | New. Selects which checks run |

## KiroSpecification

One feature directory under the Kiro tree.

| Field | Type | Notes |
|---|---|---|
| `feature` | string | Directory basename |
| `directory` | path | `.kiro/specs/<feature>/` |
| `requirements_path` | path? | Absent → `KIRO_NO_REQUIREMENTS`, continue |
| `phase` | phase? | From `spec.json`. Absent → in force |
| `notes` | string | `spec.json` `notes`, empty if missing |
| `metadata_error` | bool | `spec.json` present but unreadable |

### Phase

```text
superseded | reserved | initialized | other | absent
```

Matching of the three exclusion tokens is case-insensitive. Any other recorded value is `other` and is in force.

### Validation

- Unreadable `spec.json` → `KIRO_METADATA` and treat phase as untrusted (excluded).
- Missing `spec.json` → `phase = absent` (included).

## CcsddRequirement (existing parser IR)

| Field | Type | Notes |
|---|---|---|
| `id` | string | `N.M` or `A{n}.M` |
| `kind` | `Baseline` \| `Amendment` | Amendment is not a supersession |
| `requirement_no` | string | Parent heading number |
| `criterion_no` | string | Criterion number |
| `title` | string | Parent heading title |
| `extends` | string[] | Amendment-only parent requirement numbers |
| `ears_text` | string | Joined criterion sentence |
| `feature` | string | Basename |
| `path` | path | |
| `line`, `end_line` | usize | Physical lines |

Qualified identifier: `{feature}:{id}`.

These records are **not** fed to `ears::validate` in this feature.

## ActiveBaseline

Computed from discovered Kiro specifications only. Spec Kit specifications are not members.

| Field | Type | Notes |
|---|---|---|
| `included` | feature[] | In force |
| `excluded` | { feature, reason }[] | Reasons: `superseded`, `reserved`, `initialized`, `unreadable-metadata` |

### Membership rules

1. `phase ∈ {superseded, reserved, initialized}` → exclude.
2. Unreadable metadata → exclude.
3. Otherwise (including absent phase) → include.
4. Amendment criteria inherit the parent specification’s membership.

## SupersessionGraph

Directed links from a live successor to a superseded predecessor. Both endpoints are discovered feature names.

| Field | Type | Notes |
|---|---|---|
| `links` | { from: feature, to: feature }[] | `from` is the live successor |

### Link construction

1. **From notes (REQ-014)**: for each specification excluded as `superseded`, every discovered feature name that occurs in its phase notes is a successor candidate. Keep those whose feature is in the active baseline. Longest name first.
2. **From claims (REQ-015)**: for each specification in the active baseline, scan requirements prose **outside** acceptance-criteria regions. A spec-level claim is a supersede lemma in that prose together with a discovered feature name. Record `from = claimer`, `to = named`.

### Checks

- A superseded specification with no link to any included specification → `KIRO_SUPERSESSION_DANGLING`.
- A claim whose `to` is discovered and not excluded as superseded → `KIRO_SUPERSESSION_UNRECIPROCATED`.
- Multiple included successors for one predecessor → valid recut, no finding.
- `extends` on an amendment → not a link.

### Non-claims

- Supersede lemmas inside numbered/dotted acceptance criteria.
- Mentions of a superseded feature that lack a supersede lemma (passing history).

## Report (existing, extended)

`summary` gains optional fields, omitted when no Kiro specifications were discovered:

| Field | Type | Notes |
|---|---|---|
| `baseline_included` | usize? | REQ-012 |
| `baseline_excluded` | usize? | REQ-012 |

`FeatureResult` gains optional fields, omitted for Spec Kit features:

| Field | Type | Notes |
|---|---|---|
| `baseline` | `included` \| `excluded`? | |
| `exclusion_reason` | string? | Present only when excluded |

`schema_version` remains `"1.0"`.

## State: specification lifecycle (read, not written)

```text
absent / other ──► included in active baseline
reserved ────────► excluded (no successor required)
initialized ─────► excluded (no successor required)
superseded ──────► excluded (successor link required)
unreadable ──────► excluded (metadata finding)
```

The validator never transitions these states. It only reads them.
