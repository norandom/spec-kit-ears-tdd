# Feature Specification: Audit Logging

**Feature**: `004-audit-logging`
**Status**: Specified

## Summary

Every change the registry applies to an artifact, a manifest, or an access token is committed to durable, tamper-evident, append-only evidence before it is acknowledged, and the registry degrades in a defined way when the audit sink is unreachable.

## Requirements

- **REQ-001**: The registry shall record an audit entry for every change it applies to an artifact, a manifest, or an access token.
- **REQ-002**: The audit log shall carry in each entry the acting identity, the identifier of the presented access token, the target of the change, the action taken, and the outcome of that action.
- **REQ-003**: The audit log shall record the identifier of an access token in place of the token value itself.
- **REQ-004**: The audit log shall assign each entry a sequence number exactly one greater than the sequence number of the entry that precedes it.
- **REQ-005**: The audit log shall record in each entry the digest of the preceding entry computed under the digest algorithm the manifest declares.
- **REQ-006**: The audit log shall record each entry's timestamp in UTC with millisecond precision alongside the identity of the clock source that produced it.
- **REQ-007**: The registry shall reject any request that would rewrite or remove an individual audit entry that has already been written.
- **REQ-008**: The audit log shall publish the digest of every sealed segment so that a third party can verify that segment without privileged access to the registry.
- **REQ-009**: When a publisher with write scope publishes an artifact and the audit sink is available, the registry shall commit the publication entry to the audit sink before accepting the artifact.
- **REQ-010**: If an upload is rejected because its computed digest differs from the digest the manifest records, then the audit log shall record both digests in the rejection entry.
- **REQ-011**: The audit log shall record for each accepted artifact whether a publisher signature was present and whether that signature verified against a trusted key.
- **REQ-012**: When a publisher yanks an artifact, the registry shall record the acting identity, the stated reason, and the affected version in the audit log.
- **REQ-013**: When an access token is issued, rotated, or revoked, the registry shall record that change together with the scope the token carries.
- **REQ-014**: Where legacy mirror acceptance is enabled, the audit log shall record the originating mirror for every artifact admitted through that path.
- **REQ-015**: The registry shall commit an audit entry to durable storage before acknowledging to the client the state change that entry records.
- **REQ-016**: While the audit sink is unavailable and spool quota remains, the registry shall append each new audit entry to the local durable spool.
- **REQ-017**: If the audit sink is unavailable and spool quota is exhausted, then the registry shall reject the publication rather than accept an artifact whose publication it cannot record.
- **REQ-018**: If the audit sink is unavailable and spool quota is exhausted, then the registry shall deny every request that presents admin scope.
- **REQ-019**: While the audit sink is unavailable, the registry shall keep serving download requests that present read scope or anonymous scope.
- **REQ-020**: While the audit sink is unavailable, the registry shall defer indexing of spooled audit entries until the spool has drained.
- **REQ-021**: When the audit sink becomes reachable again, the registry shall drain the spool in sequence order before writing any newer entry directly to the sink.
- **REQ-022**: If an entry drained from the spool is already present in the audit sink, then the registry shall keep the entry already recorded rather than write a second copy.
- **REQ-023**: When the spool has drained and the audit index is behind the sink, the registry shall index the recovered entries before reporting the audit log as current.
- **REQ-024**: While the audit sink is unavailable, the registry shall report the spool depth and the age of the oldest spooled entry through its health endpoint.
- **REQ-025**: While the audit sink is available, the registry shall allow an audit log query that presents admin scope.
- **REQ-026**: If a request queries the audit log with anonymous scope or read scope, then the registry shall deny that request.
- **REQ-027**: While a retention hold is in force for an artifact, the registry shall retain every audit entry that names that artifact.
- **REQ-028**: While a security incident is active, the registry shall retain every audit entry irrespective of its age.
- **REQ-029**: When every entry in a sealed audit segment is older than the seven year retention period and neither a retention hold nor an active incident covers it, the registry shall delete that segment.
- **REQ-030**: While a mirror is trusted, its transport verifies, and its replication lag is at most one hour, the registry shall replicate each sealed audit segment to that mirror.
- **REQ-031**: If a mirror is absent from the operator's trusted mirror list, then the registry shall skip replication of audit segments to that mirror.
- **REQ-032**: If a mirror's replication lag exceeds one hour, then the registry shall skip replication of audit segments to that mirror until an operator re-seeds it.
