# Conformance parity: Rust vs Python

15 cases. Rust is the reference; every divergence below is a defect the Rust
implementation fixes, not a behaviour change to argue about.

- agree: 4
- diverge: 9
- python crashes: 2

| Case | Verdict | Difference |
| --- | --- | --- |
| `all-features-scope` | PY-CRASH | ears-sdd: error: unrecognized arguments: --all |
| `duplicate-identifier` | agree | — |
| `feature-outside-project` | DIVERGE | features rust=0 py=1; rust-only: SPEC_OUTSIDE_PROJECT,SPEC_SCOPE; py-only: SPEC_MISSING |
| `feature-qualified-ids` | PY-CRASH | ears-sdd: error: unrecognized arguments: --all |
| `fenced-code-block` | DIVERGE | ok rust=True py=False; reqs rust=1 py=2; py-only: EARS_SHALL |
| `hyphenated-compound` | DIVERGE | ok rust=True py=False; py-only: EARS_MODAL |
| `markdown-forms` | DIVERGE | reqs rust=5 py=1 |
| `missing-production-root` | DIVERGE | rust-only: PRODUCTION_ROOT_MISSING |
| `modal-in-quoted-literal` | DIVERGE | ok rust=True py=False; py-only: EARS_MODAL |
| `pinned-feature-scope` | DIVERGE | rust-only: SPEC_SCOPE |
| `possessive-apostrophe` | agree | — |
| `selector-traversal` | DIVERGE | ok rust=False py=True; rust-only: TRACE_TEST_ROOT |
| `stale-feature-pointer` | DIVERGE | ok rust=False py=True; reqs rust=0 py=1; features rust=0 py=1; rust-only: FEATURE_MISSING,SPEC_SCOPE |
| `undecodable-spec` | agree | — |
| `wide-identifier` | agree | — |
