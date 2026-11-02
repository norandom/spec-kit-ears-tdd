# Validate Envelope Containment

Run commands from the repository root. This feature depends on `009-baseline-containment`; none of
its behaviour exists until that engine's first slice is green.

## Current planning checks

```sh
cargo run -q -p ears-sdd -- validate --phase spec --all
cargo run -q -p ears-sdd -- validate --phase plan --feature specs/010-envelope-containment
cargo run -q -p ears-sdd -- validate --phase tasks --feature specs/010-envelope-containment
cargo test -p ears-sdd
```

The spec check passes. The plan and tasks checks report absent selectors as `TRACE_TEST_FILE`
until each task writes its evidence; that list is the work. Do not disable `require_test_files`,
remove mappings, or substitute manual verification. T009's selectors stay missing until a native
workflow file is checked in; that is recorded as blocked, not as done.

## After the reader is implemented

Add an envelope comparison to `.specify/containment.toml` as shown in
[contracts/envelope.md](contracts/envelope.md), with an envelope vocabulary beside the definition:

```sh
cargo run -q -p ears-sdd -- validate --project conformance/cases/contain-envelope-widened/project --phase plan --all --json
```

| Scenario | Check the report for |
|---|---|
| Extra tool in a phase | `CONTAIN_WIDENED` on `tool:<name>` with `PHASE-<phase>` as contributor; gate fails |
| Weaker transition guard | `CONTAIN_WIDENED` on `transition:<from>:<to>` with a witness satisfying the new guard only |
| Tool removed from a phase | `CONTAIN_NARROWED` as a warning; gate passes |
| Any envelope | `reachability_computed: false`, `CONTAIN_REACHABILITY_OMITTED`, every unmodelled path named |
| Unknown field such as `fork` | `CONTAIN_UNMODELLED_CONSTRUCT`, `complete: false`, results over the rest still present |
| Term absent from the envelope vocabulary | `CONTAIN_UNDECLARED_TERM`; that transition uncompared; tools still compared |
| Source without a vocabulary | `CONTAIN_INPUT_INVALID`; comparison withheld |
| Unknown `format` | `CONTAIN_DEFINITION_UNREADABLE` naming the format; comparison withheld |
| Any adapter | `adapter = { id, version }` on the source record |

## Completion

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run
cargo run -q -p ears-sdd -- validate --phase final --all
```
