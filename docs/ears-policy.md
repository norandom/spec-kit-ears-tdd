# EARS and verification policy

Each normative requirement is a single Markdown line with a stable ID:

```text
- REQ-001: When the user submits valid input, the service shall persist the record.
```

Supported EARS forms are ubiquitous, event-driven, state-driven, optional-feature, and
unwanted-behavior requirements. Each requirement has exactly one `shall` obligation. Split compound
behavior into separate IDs.

The feature directory also contains `traceability.toml`:

```toml
[requirements.REQ-001]
verification = "automated"
tests = ["tests/test_records.py::test_valid_record_is_persisted"]
```

Manual verification is permitted only when the mapping includes a concrete rationale. The policy
does not claim that a test selector passed; the consuming project's declared `test_command` remains
the source of execution evidence.

## Failure interpretation

- `EARS_*` findings mean the requirement sentence is ambiguous or malformed.
- `TRACE_*` findings mean the requirement/test mapping is incomplete or stale.
- `CODE_REQ_ID` and `CODE_REQ_PROSE` mean specification content crossed into production code.
- `TEST_COMMAND` means the project has not declared its real test command for the final gate.

Fix the artifact that owns the problem. Never suppress a separation finding by moving requirement
text to another production file.

