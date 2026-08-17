---
description: Create the implementation plan and complete requirement-to-verification traceability.
strategy: wrap
# Restated because `wrap` inherits only scripts/agent_scripts/argument-hint; see speckit.specify.md.
handoffs:
  - label: Validate Traceability
    agent: speckit.ears-validate.validate
    prompt: plan
    send: true
  - label: Create Tasks
    agent: speckit.tasks
    prompt: Break the plan into tasks
    send: true
  - label: Create Checklist
    agent: speckit.checklist
    prompt: Create a checklist for the following domain...
---

{CORE_TEMPLATE}

## Traceability postcondition

Update `traceability.toml` beside the active `spec.md`. Every `REQ-NNN` must map to either one or
more automated test selectors or a justified manual verification. Keep implementation technology
and design detail in `plan.md`; do not copy requirement prose into production code.

Tell the user to run:

```text
ears-sdd validate --phase plan
```

