---
description: Generate test-first tasks with complete requirement coverage.
strategy: wrap
# Restated because `wrap` inherits only scripts/agent_scripts/argument-hint; see speckit.specify.md.
handoffs:
  # Ahead of Implement deliberately: this is the last point where a requirement with no covering
  # task is cheap to fix, and the gate that guards production-code changes is the tasks gate.
  - label: Validate Task Coverage
    agent: speckit.ears-validate.validate
    prompt: tasks
    send: true
  - label: Analyze For Consistency
    agent: speckit.analyze
    prompt: Run a project analysis for consistency
    send: true
  - label: Implement Project
    agent: speckit.implement
    prompt: Start the implementation in phases
    send: true
---

{CORE_TEMPLATE}

## Test-first postcondition

Tests are mandatory. Place each failing-test task before its corresponding implementation task.
Every behavior task must identify its `REQ-NNN` coverage and test selector. Do not permit a generic
"add tests later" task.

Tell the user to run:

```text
ears-sdd validate --phase tasks
```

