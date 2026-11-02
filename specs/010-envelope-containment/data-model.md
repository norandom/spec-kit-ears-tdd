# Data model: Envelope Containment

Extends the 009 data model; only the additions are listed.

| Entity | Data and relationships | Validation |
|---|---|---|
| Envelope source | Definition path, envelope vocabulary path, optional format, optional derogation sidecar | Vocabulary required; format defaults to interchange; unknown format is unreadable |
| Envelope | Version, ID, initial phase, map of phases, allowed tools, local guarded transitions, unmodelled paths | Closed phase set; valid endpoints; grammar/type checks; no reachability computation |
| Envelope model | Reserved `envelope-phase` enum term; `tool:<name>` and `transition:<from>:<to>` effects; `PHASE-<name>` contributors | Not a `ModelFile`; one phase contributes many effects |
| Envelope vocabulary | Ordinary vocabulary terms with domains | Structural checks only; never subject to unused-term findings; `envelope-phase` reserved |
| Adapter | Format name, identity, version, read function | Fixed in-code registry; identity and version recorded per source |
| Unmodelled construct | Side, native or JSON-pointer path, adapter that reported it | Sorted; marks comparison incomplete, does not withhold it |

An envelope comparison's `status` uses the 009 states. `complete` is false while any unmodelled
construct exists on either side. The orientation is that of `revision`.
