# Architecture

The repository has three independently versioned concerns:

1. `specify-cli` is an exact external dependency. Its tag and commit are recorded by `uv`.
2. Spec Kit components express agent guidance and workflow composition through supported manifests.
3. The standalone Python validator provides deterministic evidence independent of an agent.

The validator is packaged into the extension and uses only Python 3.11+ standard-library modules.
The `ears-sdd` bootstrap command installs the source components through official `specify` commands;
it does not edit Spec Kit's registries directly.

## Translation boundary

Requirements define observable behavior. Plans translate requirements into design decisions.
Tasks translate the plan into test-first work. Tests may name requirement IDs for traceability.
Production code may contain neither requirement IDs nor copied EARS sentences.

The final validator enforces this boundary by scanning configured production roots. It reports an
error when it finds an exact requirement ID or normalized requirement sentence in source files.
Projects configure roots and source extensions in `.specify/ears-sdd.toml`.

## Safety properties

- Validation is read-only and never runs project tests implicitly.
- Bootstrap operations are explicit and printed before execution.
- Existing policy configuration and launchers are preserved.
- No global Spec Kit, Codex, shell, or Git configuration is changed.
- Upstream upgrades are deliberate compatibility changes, never floating `main` dependencies.

