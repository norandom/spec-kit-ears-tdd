# Feature Specification: Customer Directory

**Feature**: `001-customer-directory`
**Status**: Retro-specified from `src/schema/customer_directory.sql` and the 2024 erasure endpoint

## Summary

The customer directory holds one current postal address per customer and serves the erasure
endpoint added in 2024. This specification was written by reading the existing schema and endpoint
behaviour; it describes what the system does today, not what someone designed up front.

## Requirements

- **REQ-001**: The customer directory shall hold exactly one current postal address for each customer.
- **REQ-002**: When a customer submits a new postal address, the customer directory shall replace the stored address in place.
- **REQ-003**: When a customer's postal address is replaced, the customer directory shall record the replacement time.
- **REQ-004**: When a verified erasure request completes, the system shall delete that customer's personal address from every store it controls.
- **REQ-005**: If a verified erasure request arrives while the customer has an unfulfilled order, then the system shall defer the erasure until that order completes.
- **REQ-006**: When an erasure completes, the customer directory shall record the erasure time against the customer.
- **REQ-007**: When an erasure completes, the system shall report to the customer which stores were cleared.
- **REQ-008**: The customer directory shall reject a postal address that fails address-format validation.
- **REQ-009**: Where a customer has no recorded postal address, the customer directory shall report the customer as incomplete rather than substituting a default.
- **REQ-010**: The customer directory shall restrict read access to the postal address to services holding the customer-data scope.
