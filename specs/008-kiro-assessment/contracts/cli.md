# CLI contract

No new subcommand. No new flags.

```text
ears-sdd validate --project <path> --phase spec
ears-sdd validate --project <path> --phase spec --all
ears-sdd validate --project <path> --phase spec --feature <name-or-path>
```

## Layout detection

If `<path>/.kiro/specs/` is a directory, its child directories are Kiro specifications.

`--all` and the default whole-project path (no feature pointer) include every such directory. A Kiro-only project does not require `--all` to see the whole tree: there is no Spec Kit feature pointer to inherit.

## `--feature`

Accepted forms, in order:

1. A path contained in the project that is a directory holding `requirements.md` (Kiro) or `spec.md` (Spec Kit).
2. A path contained in the project that is those files themselves.
3. A basename matching exactly one Kiro feature directory under `.kiro/specs/`.

Outside-project paths still produce `SPEC_OUTSIDE_PROJECT`.

## Read-only

The process must not create, delete, or modify any file under `--project`. This includes not writing `.specify/`, `spec.json`, caches, or `__pycache__`.
