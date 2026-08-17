---
description: Create a specification, then normalize every normative requirement into EARS form.
strategy: wrap
# Restated rather than inherited. Spec Kit's wrap strategy copies only `scripts`, `agent_scripts`
# and `argument-hint` from the command it wraps (presets/__init__.py), so a wrapper that says
# nothing about handoffs silently removes the next-step buttons the core command offered. That
# couples this list to upstream's: if Spec Kit adds a handoff to speckit.specify, it has to be added
# here too. The alternative was leaving every wrapped command a dead end.
handoffs:
  # First, because it is the cheapest and most specific feedback available at this point: it names
  # the requirement and the defect rather than asking someone to re-read the whole spec.
  - label: Validate EARS Requirements
    agent: speckit.ears-validate.validate
    prompt: spec
    send: true
  - label: Clarify Spec Requirements
    agent: speckit.clarify
    prompt: Clarify specification requirements
    send: true
  - label: Build Technical Plan
    agent: speckit.plan
    prompt: Create a plan for the spec. I am building with...
---

{CORE_TEMPLATE}

## EARS postcondition

Before finishing, ensure each normative requirement has a unique `REQ-NNN` ID, contains exactly
one `shall`, and uses an EARS form documented in the resolved specification template. Create the
feature's `traceability.toml`; entries may remain incomplete until planning, but requirement IDs
must already match the specification.

Tell the user to review the artifact and run:

```text
ears-sdd validate --phase spec
```

