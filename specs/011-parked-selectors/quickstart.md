# Quickstart: Parked Selectors

```sh
cargo build -p ears-sdd
ears-sdd validate --project conformance/cases/pending-selector-parked/project --phase plan
```

Expected: `ok: true`, one `TRACE_TEST_PARKED` advisory naming the pending selector, no
`TRACE_TEST_FILE`.

```sh
ears-sdd validate --project conformance/cases/pending-selector-parked-disabled/project --phase plan
```

Expected: FAIL with `TRACE_TEST_FILE` (prefixes disabled in that fixture's config).

Repo-level proof — this project itself, with 009/010 parked:

```sh
ears-sdd validate --project . --phase final --all
```

Expected: PASS; advisories include `TRACE_TEST_PARKED` for each `conformance/pending/` selector
named by specs 009/010, and no `TRACE_TEST_FILE`.
