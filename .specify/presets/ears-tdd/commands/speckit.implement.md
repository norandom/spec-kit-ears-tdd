---
description: Implement approved tasks test-first and enforce the final EARS traceability gate.
strategy: wrap
# Core speckit.implement offers no handoffs, so nothing is being restored here -- this one is added.
# Implementation is the phase that can invalidate traceability, by renaming a test or moving a file
# the selectors name, and leaving it as the only dead end in the chain is what makes the final gate
# something people run days later, if at all.
handoffs:
  - label: Validate Final Gate
    agent: speckit.ears-validate.validate
    prompt: final
    send: true
---

Before executing implementation tasks, run the human validator command shown in the project docs:

```text
ears-sdd validate --phase tasks
```

If validation fails, stop and repair specification, plan, task, or traceability artifacts. Do not
"fix" policy failures by copying requirement prose into production code.

{CORE_TEMPLATE}

After the project's configured tests pass, run:

```text
ears-sdd validate --phase final
```

