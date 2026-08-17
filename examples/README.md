# Examples

Two runnable projects. Both are checked by CI on every change, so the output quoted in the
documentation cannot drift from what the tool actually reports.

Neither has an implementation. They ship specifications, vocabulary, intentions, and constraint
models, which is what the gates read.

## `package-registry`

Twelve features, 383 requirements, written feature by feature the way Spec Kit produces them. Every
specification passes on its own. Merged, exactly one pair contradicts, and the two requirements sit
five features apart and share no vocabulary at the surface.

```sh
ears-sdd validate --project examples/package-registry --phase spec --all   # PASS, 383 requirements
ears-sdd validate --project examples/package-registry --phase plan --all   # FAIL, 1 error
```

Written up as [a needle in 383 requirements](../docs/tutorial/needle-in-a-haystack.md).

## `legacy-order-system`

A brownfield adoption. Two database tables written seven years apart both hold a customer's postal
address, under different column names and with different lifecycles. Twenty requirements
retro-specified from the schema, and the contradiction that falls out is a schema decision rather
than a bug in either subsystem.

```sh
ears-sdd validate --project examples/legacy-order-system --phase tasks --all   # FAIL, 2 errors
```

Written up as [brownfield: duplicate storage](../docs/tutorial/brownfield-duplicate-storage.md).

## Why they fail on purpose

Both projects are meant to fail their merged gate. That is what they demonstrate. CI asserts the
specific findings rather than a pass, so a change that stops finding a contradiction breaks the
build rather than quietly making the tutorials wrong.
