# Project policy

The most any specification in this project is permitted to assert. Not a feature: this file lives
outside `spec_glob` and is read only by the `policy` comparison in `.specify/containment.toml`.

- REQ-001: Where an operator is present, a service shall be permitted to write.
- REQ-002: Where an operator is present, a service shall be blocked from deleting.
