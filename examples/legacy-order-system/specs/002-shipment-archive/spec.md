# Feature Specification: Shipment Archive

**Feature**: `002-shipment-archive`
**Status**: Retro-specified from `src/schema/shipment_archive.sql` and the 2016 fulfilment service

## Summary

The shipment archive records the address each parcel was actually dispatched to, and keeps it for
as long as the delivery can be disputed. This specification was written by reading the existing
schema and the retention job; it predates the customer directory by three years.

## Requirements

- **REQ-001**: When a shipment is dispatched, the shipment archive shall record the delivery address as it stood at dispatch.
- **REQ-002**: The shipment archive shall retain a completed shipment's personal address for seven years after completion.
- **REQ-003**: While a shipment record is inside its retention period, the shipment archive shall reject any modification to its recorded delivery address.
- **REQ-004**: When a shipment record passes seven years since completion, the retention job shall delete its recorded delivery address.
- **REQ-005**: While a dispute is open against a shipment, the shipment archive shall retain that shipment's personal address regardless of its age.
- **REQ-006**: When a carrier dispute is raised, the shipment archive shall produce the delivery address recorded at dispatch.
- **REQ-007**: The shipment archive shall record one delivery address per shipment rather than referencing the customer's current address.
- **REQ-008**: When a shipment is dispatched, the shipment archive shall record the dispatch time.
- **REQ-009**: The shipment archive shall restrict read access to the delivery address to services holding the fulfilment scope.
- **REQ-010**: Where a shipment has no recorded delivery outcome, the shipment archive shall report it as in flight rather than completed.
