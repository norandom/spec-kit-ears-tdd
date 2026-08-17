# Feature Specification: Retention and Collection

**Feature**: `006-retention-and-collection`
**Status**: Specified

## Summary

The registry deletes artifacts by age and use, keeps anything held, referenced, downloaded, or unauditable, and propagates each deletion as a tombstone to mirrors and the index.

## Requirements

- **REQ-001**: The registry shall publish the retention policy currently in force, naming every age threshold and the condition that selects it.
- **REQ-002**: When a collection sweep runs outside an active incident with the audit sink reachable and evaluates an unheld artifact older than 365 days that has zero downloads, the registry shall delete that artifact.
- **REQ-003**: When a collection sweep runs outside an active incident with the audit sink reachable and evaluates an unheld yanked artifact older than 30 days that has zero downloads, the registry shall delete that artifact.
- **REQ-004**: While the storage quota is exceeded outside an active incident and with the audit sink reachable, the registry shall delete every unheld artifact older than 90 days that has zero downloads.
- **REQ-005**: While an artifact is 365 days old or younger and is neither yanked nor under storage quota pressure, the registry shall retain that artifact.
- **REQ-006**: If an artifact has a download count greater than zero, then the registry shall retain that artifact through every collection sweep.
- **REQ-007**: While a retention hold is in force on an artifact, the registry shall retain that artifact irrespective of its age, its download count, and the storage quota.
- **REQ-008**: While a yanked artifact is 30 days old or younger and the storage quota is not exceeded, the registry shall retain that artifact so that pinned builds continue to resolve.
- **REQ-009**: While an incident is active, the registry shall retain every artifact a collection sweep selects.
- **REQ-010**: If the audit sink is unavailable, then the registry shall retain every collection candidate until audit entries can be written again.
- **REQ-011**: When a collection sweep selects an artifact for deletion, the registry shall confirm that no manifest in the registry references the digest of that artifact.
- **REQ-012**: The manifest shall list the digest of every artifact it references so that a collection sweep can identify the artifacts still in use.
- **REQ-013**: If a manifest references an artifact that is absent from storage, then the registry shall report that manifest as broken in the retention report.
- **REQ-014**: If the stored bytes of an artifact do not match its recorded digest, then the registry shall withhold that artifact from every download response.
- **REQ-015**: While an artifact is yanked and its stored bytes match its recorded digest, the registry shall serve that artifact to a request naming its exact version.
- **REQ-016**: When a request to place or lift a retention hold presents an unexpired admin token over a verified TLS connection, the registry shall allow that request.
- **REQ-017**: If a request to place or lift a retention hold presents a token whose scope is not admin, then the registry shall deny that request.
- **REQ-018**: When an operator places a retention hold, the registry shall record the identity of that operator, the reason text, and the expiry date of the hold.
- **REQ-019**: If a retention hold reaches its expiry date, then the registry shall report that hold as expired rather than lifting it without an operator request.
- **REQ-020**: The registry shall list every artifact under a retention hold together with the age of that hold in days and the operator who placed it.
- **REQ-021**: The registry shall write one audit entry for every retention decision it reaches, including a decision to retain.
- **REQ-022**: The registry shall name in each deletion entry the requirement identifier and the age threshold that justified the deletion.
- **REQ-023**: The registry shall record in each deletion entry the digest algorithm and the digest of the deleted artifact.
- **REQ-024**: The registry shall record in each deletion entry the download count the sweep observed and the moment it observed that count.
- **REQ-025**: The registry shall keep the audit entry for a deleted artifact for at least 3650 days after the deletion.
- **REQ-026**: When an artifact is deleted, the registry shall replicate the resulting tombstone to every trusted mirror on the current protocol whose replication lag is at most 300 seconds.
- **REQ-027**: Where a mirror runs the legacy protocol, the registry shall skip tombstone replication to that mirror.
- **REQ-028**: The mirror shall report the identifier of the last collection sweep whose tombstones it applied.
- **REQ-029**: When a collection sweep leaves the search index stale outside a maintenance window and outside an active incident, the registry shall rebuild the affected index shard.
- **REQ-030**: While a maintenance window is open, the registry shall defer every index rebuild caused by collection until that window closes.
- **REQ-031**: Where a collection sweep runs in dry-run mode, the registry shall report the artifacts it would delete without removing any bytes.
- **REQ-032**: The registry shall report for each collection sweep the count of artifacts deleted, the count retained, and the count skipped alongside the reason recorded for each outcome.
