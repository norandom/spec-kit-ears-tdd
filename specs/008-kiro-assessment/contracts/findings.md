# Finding codes: Kiro assessment

New codes. Prefix `KIRO_` names the artifact family. Existing `SPEC_`, `EARS_`, `TRACE_`, `TASK_`, `CODE_` codes are not emitted against Kiro specifications.

| Code | Severity | When | Fields |
|---|---|---|---|
| `KIRO_NO_REQUIREMENTS` | Warning | Feature directory has no `requirements.md` | `feature`, `path` = the directory |
| `KIRO_NO_CRITERIA` | Warning | Baseline requirement heading produced no acceptance criteria | `feature`, `path`, `line` of the heading |
| `KIRO_METADATA` | Error | `spec.json` exists but cannot be read or parsed | `feature`, `path` = the metadata file |
| `KIRO_SUPERSESSION_DANGLING` | Error | Superseded specification has no link to any specification in the active baseline | `feature` of the predecessor, `path` = its `spec.json` or directory |
| `KIRO_SUPERSESSION_UNRECIPROCATED` | Error | Active specification claims to supersede a discovered specification that is not excluded as superseded | `feature` of the claimer; `detail.predecessor` names the other feature |
| `KIRO_CHECKS_OMITTED` | Advisory | At least one Kiro specification was assessed | `path` = `.kiro/specs`; message names the omitted Spec Kit checks |

`CONFIG_MISSING` is not emitted when a Kiro tree is present.

`SPEC_NONE` is emitted only when the Spec Kit glob matches nothing **and** no Kiro specification was discovered.

Kiro specifications must not produce `TRACE_*`, `TASK_*`, `EARS_*`, `REQ_DUPLICATE` (Spec Kit form), `REQ_NONE` (Spec Kit form), or `CODE_*` / `SEPARATION_*` findings.

When constraint analysis does not run, `provenance.disabled_checks` includes `constraints` (existing disclosure). No new finding is required for REQ-024 if that list is present in both human and machine-readable results.
