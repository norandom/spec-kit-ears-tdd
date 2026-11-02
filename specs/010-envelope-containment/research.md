# Research: Envelope Containment

Decisions recorded at the 2026-09-22 split from 009. Design decisions, not implementation claims.

## Own vocabulary, not the product's

**Decision:** An envelope source names its own vocabulary file. Guard terms resolve against it
alone. `TERM_UNUSED` is never computed over it.

**Rationale:** The vocabulary gate raises `TERM_UNUSED` for any declared term no requirement tags
(`docs/reference/findings.md`). Envelope guard terms describe the agent's process and are tagged
by no requirement, so routing them through the product vocabulary would either fail every run or
push a project to disable the check. The product's concept scheme also has no business holding
`test-result`.

**Alternatives:** Declaring domains inline in the definition ties the interchange format to the
vocabulary format and gives a native adapter nowhere to put domains it cannot infer. A shared
vocabulary with a per-term exemption flag is a configuration knob on a gate, which this project
refuses.

## Adapter first, format second

**Decision:** The interchange format stays as the canonical model an adapter produces. The manifest
names a `format`; the registry is fixed in code; every adapter reports identity, version and the
native constructs it dropped.

**Rationale:** The interchange format has no producer. A comparison over a hand-written copy of a
runtime's workflow checks the copy, not what ran. The adapter is where the feature's value lives,
and its dropped-construct list is the honesty mechanism the rest of the project relies on.

**Alternatives:** Claiming compatibility with a named runtime's schema from memory of its
documentation would produce a reader that is wrong in ways no fixture here can catch. The mapping
is written from a checked-in sample or not at all.

## Set difference where it suffices

**Decision:** Per-phase tool permission differences are set differences. The implementation may
answer them without an encoding, provided the result flows through the same finding, witness,
derogation and provenance path as transition widenings.

**Rationale:** `tool:Read` under `envelope-phase == 'planning'` versus the same on the other side
is a membership question; encoding it costs nothing but also proves nothing. Transition guards are
where the engine is needed.

**Alternatives:** Forcing every question through the BDD keeps one code path at the cost of
obscuring how simple most envelope findings are. Forcing every question through set logic loses
the transition-guard witness.

## Reachability stays out

Unchanged from 009: no image computation over the transition relation. The story text now states
the consequence in both directions (unreachable phases are still compared; newly reachable phases
are not detected unless a guard weakened), because a reader of the previous text could have
assumed the second.
