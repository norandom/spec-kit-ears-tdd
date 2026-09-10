# Research: Kiro Project Assessment

## Decision: Discover Kiro trees inside `validate`, do not add an `assess` command

- **Rationale**: A second entry point that silently differs from `validate` recreates the unearned-pass problem. `ears-sdd validate --project PATH --phase spec` is already the claim. Layout detection belongs in discovery.
- **Alternatives considered**: A dedicated `ears-sdd assess` subcommand (two claims to keep in sync); requiring the operator to set `spec_glob` in a config file written into the assessed project (forbidden by REQ-021 / REQ-022).

## Decision: Recognize a Kiro tree as `.kiro/specs/` containing feature directories

- **Rationale**: This is the layout facdrone and Kiro’s own docs use. Presence of that directory is the trigger. Each child directory is a specification; `requirements.md` is the requirements document; `spec.json` is optional phase metadata.
- **Alternatives considered**: Glob-only via `spec_glob = ".kiro/specs/*/requirements.md"` (fails native Kiro without a config file, and would still run Spec Kit identifier checks on those files); treating `spec.md` as the Kiro artifact (Kiro does not write `spec.md`).

## Decision: Kiro feature identifiers are the directory basename

- **Rationale**: Facdrone phase notes and relationship prose name `facdrone-ledger-core`, not `.kiro/specs/facdrone-ledger-core`. Successor matching (REQ-014, REQ-015) is exact feature-name occurrence. Spec Kit features keep today’s relative parent path (`specs/001-…`) so existing qualified IDs do not change (REQ-003).
- **Alternatives considered**: Use the same relative-from-root rule for both families (breaks successor matching against real notes); rewrite Spec Kit names to basenames (breaks current reports and the conformance corpus).

## Decision: Keep `ccsdd.rs` as a parser; add a `kiro` module for layout, baseline, and supersession

- **Rationale**: The untracked parser already normalizes G1 / G1-dotted / G2 / G3 / G4 into dotted identifiers and `extends` links. Baseline membership and supersession are a different concern (phase metadata + a name graph) and should not live in the Markdown walker.
- **Alternatives considered**: Fold everything into `discovery.rs` (that file is already the Spec Kit scoping story); convert `CcsddRequirement` into `Requirement` and run `ears::validate` (this spec explicitly omits Spec Kit identifier and EARS-form checks on Kiro criteria).

## Decision: Union discovery; `SPEC_NONE` only when both families are empty

- **Rationale**: A Kiro-only project matches the default `specs/*/spec.md` glob with nothing. Today that is `SPEC_NONE` and a failed run, which is exactly the unearned-failure that hides the tree. Spec Kit-only projects must keep today’s glob behaviour (REQ-003). Mixed trees are an edge case: both families appear in `features` / `specs_examined`.
- **Alternatives considered**: Change the default glob (breaks every Spec Kit project); fail Kiro-only projects until they add Spec Kit files (forbidden).

## Decision: Missing Spec Kit configuration is not an error when a Kiro tree is present

- **Rationale**: `CONFIG_MISSING` is currently an error. Facdrone has no `.specify/ears-sdd.toml`, and this feature may not write one. Default `Config` is enough to discover and assess. `disabled_checks` still lists `vocabulary` and `constraints` (REQ-024).
- **Alternatives considered**: Demote `CONFIG_MISSING` globally to a warning (changes Spec Kit-only behaviour); require a sidecar config outside the assessed tree (out of spec).

## Decision: Omit Spec Kit checks per Kiro specification, disclose once

- **Rationale**: REQ-023 forbids identifier, traceability, tasks, and separation checks on Kiro documents. Those checks would fail every facdrone spec for lacking `REQ-NNN` and `traceability.toml`. One advisory finding (`KIRO_CHECKS_OMITTED`) names the omission so a green result cannot be read as “those layers passed.” On mixed trees the Spec Kit family still receives them.
- **Alternatives considered**: Auto-disable those checks in `Config` for Kiro-only runs (lies about configuration); run them and expect failure (unusable); silence (unearned pass).

## Decision: Do not EARS-lint Kiro acceptance criteria in this feature

- **Rationale**: Kiro’s own rule file allows combined While+When, If-without-then, and named subjects. Joined multi-line criteria often contain more than one `shall`. Linting them with `ears.rs` would be a different spec. This feature stops at parse + baseline + supersession.
- **Alternatives considered**: A “Kiro dialect” EARS profile (scope creep); mapping every criterion into `Requirement` and calling `ears::validate` (false failures).

## Decision: Supersession links are name occurrences, not NLP

- **Rationale**: REQ-014 collects successor names as discovered feature names appearing in phase notes. REQ-015 records a claim when a live specification’s requirements prose *outside acceptance criteria* contains a supersede lemma (`supersede` / `supersedes` / `superseded` / `superseding`) and a discovered feature name. Longest-name-first so a name that is a prefix of another does not steal the match. The word inside an acceptance criterion is product behaviour (facdrone’s run-store `supersede`).
- **Alternatives considered**: A sidecar `supersessions.toml` (operator said Kiro conventions only); LLM classification (non-deterministic, not a gate); treating every mention of a superseded name as a claim (signal-lifecycle mentioning leftover obligations would look like a second successor).

## Decision: Optional report fields; keep `schema_version` `1.0`

- **Rationale**: Existing conformance `expected.json` files compare `summary` objects. New baseline counts go on `summary` as `skip_serializing_if = None`, so Spec Kit cases stay byte-stable. Human output gains a baseline line analogous to `Disabled:`.
- **Alternatives considered**: Schema bump (unnecessary if old fields are unchanged); stuffing baseline into finding `detail` only (REQ-012 wants counts in the result even when there is no defect).

## Decision: Conformance fixtures reproduce Kiro shapes; CI does not open `../facdrone`

- **Rationale**: Facdrone is the learning example and must remain untouched. The corpus already stores behaviour as data. Fixtures encode G1, G2, G3 amendments, `phase: superseded` notes, recuts, reserved/initialized slots, and relationship prose. A later manual smoke against facdrone is optional and not a gate.
- **Alternatives considered**: Running the conformance harness against the live facdrone tree (couples CI to a sibling repo the tool must not modify).

## Decision: Phase tokens are the three exclusion strings, case-insensitive

- **Rationale**: Facdrone uses `superseded`, `reserved`, `initialized` as `spec.json` `phase` values. Every other observed value (`tasks-generated`, `implementation-complete`, `complete`, `implemented`, `requirements-generated`) is in force. Phase-drift (delivered work still marked `tasks-generated`) is out of this spec.
- **Alternatives considered**: Inferring “done” from `completion.md` / approvals (out of scope); treating unapproved live specs as excluded (operator chose otherwise).
