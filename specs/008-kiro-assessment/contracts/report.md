# Report contract additions

`schema_version` remains `"1.0"`. New fields are omitted when unused so existing conformance cases stay stable.

## `summary` (when any Kiro specification was discovered)

```json
{
  "baseline_included": 2,
  "baseline_excluded": 3
}
```

Counts are of Kiro specifications, not criteria. Spec Kit specifications are not included in either number.

## `features[]` entries for Kiro specifications

```json
{
  "feature": "alpha",
  "spec": ".kiro/specs/alpha/requirements.md",
  "requirements": 4,
  "baseline": "included"
}
```

```json
{
  "feature": "legacy",
  "spec": ".kiro/specs/legacy/requirements.md",
  "requirements": 2,
  "baseline": "excluded",
  "exclusion_reason": "superseded"
}
```

`exclusion_reason` is one of `superseded`, `reserved`, `initialized`, `unreadable-metadata`.

A directory with no `requirements.md` still appears as a feature with `requirements: 0` and the `KIRO_NO_REQUIREMENTS` finding.

## Human-readable extra line

Printed when baseline counts are present, on every such run, pass or fail:

```text
Baseline: 2 included, 3 excluded (superseded: legacy, other; reserved: slot; initialized: seed)
```

The disabled-checks line is unchanged. A Kiro-only project with default configuration still prints:

```text
Disabled: vocabulary, constraints (not checked)
```

## Scope

- Spec Kit-only: `provenance.scope` unchanged (`glob` / `flag` / …).
- Kiro-only, whole tree: `{"source":"glob","value":".kiro/specs/*/requirements.md"}`.
- `--feature` naming one Kiro directory or basename: `{"source":"flag","value":"…"}` as today.
- Mixed tree, whole-project run: `glob` value is the configured Spec Kit glob; Kiro features still appear in `features` and `specs_examined`.
