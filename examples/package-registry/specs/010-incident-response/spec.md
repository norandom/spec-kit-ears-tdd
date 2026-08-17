# Feature Specification: Incident Response

**Feature**: `010-incident-response`
**Status**: Specified

## Summary

While an operator has declared an active incident, the registry freezes the publication path, tightens what it will serve and replicate, preserves everything an investigator will need, and keeps the authenticated read path open.

## Requirements

- **REQ-001**: When an operator declares an incident, the registry shall apply incident posture to every request it receives after the declaration timestamp.
- **REQ-002**: While an incident is active, the registry shall publish the incident identifier and the declaration timestamp on its public status endpoint.
- **REQ-003**: While an incident is active, the registry shall write an audit entry for every request it handles.
- **REQ-004**: If the audit sink is unavailable while an incident is active, then the registry shall reject every publication request.
- **REQ-005**: If the audit sink stops accepting writes while an incident is active, then the registry shall raise an operator alert naming the unreachable sink.
- **REQ-006**: While an incident is active, the registry shall reject a publication request presented with a write-scoped token.
- **REQ-007**: If a publication request during an active incident does not resolve to an authenticated publisher, then the registry shall reject the artifact.
- **REQ-008**: When an authenticated publisher presents an admin-scoped token for an artifact whose digest matches and whose signature verifies while the audit sink is reachable during an active incident, the registry shall accept the remediation artifact.
- **REQ-009**: If an uploaded artifact's computed digest differs from the digest recorded in the manifest while an incident is active, then the registry shall discard the upload.
- **REQ-010**: While an incident is active, the registry shall allow a download request that presents an unexpired read-scoped token over a verified connection within its rate limit.
- **REQ-011**: If a presented access token is expired while an incident is active, then the registry shall deny the request.
- **REQ-012**: If a connection fails transport verification while an incident is active, then the registry shall deny the request.
- **REQ-013**: If a client exceeds its permitted request rate while an incident is active, then the registry shall deny the request.
- **REQ-014**: While an incident is active, the registry shall serve an artifact recorded under sha256 or blake3 whose digest matches, whose signature is present and verifies, that is not yanked, and that comes from a trusted mirror lagging no more than 300 seconds.
- **REQ-015**: While an incident is active, the registry shall withhold every yanked artifact from resolution.
- **REQ-016**: While an incident is active, the registry shall withhold an artifact whose manifest entry carries no publisher signature.
- **REQ-017**: If a manifest entry records its digest under md5 or sha1 while an incident is active, then the registry shall withhold that artifact.
- **REQ-018**: If a mirror's replication lag exceeds 300 seconds while an incident is active, then the registry shall withhold the artifacts that mirror serves.
- **REQ-019**: While an incident is active, the registry shall retain every artifact that scheduled collection would otherwise delete.
- **REQ-020**: While an incident is active, the registry shall skip replication to any mirror absent from the trusted mirror list.
- **REQ-021**: Where a legacy mirror is enabled during an active incident, the registry shall skip replication to that legacy mirror.
- **REQ-022**: While an incident is active, the registry shall replicate an artifact whose digest matches and whose signature verifies to every trusted mirror that is not a legacy mirror.
- **REQ-023**: If an artifact's publisher signature does not verify while an incident is active, then the registry shall defer indexing that artifact.
- **REQ-024**: When an unyanked artifact whose digest matches and whose signature verifies is accepted during an active incident, the registry shall index that artifact ahead of the deferred queue.
- **REQ-025**: While an incident is active, the manifest shall record the incident identifier against every entry it changes.
- **REQ-026**: When an incident is closed, the registry shall publish a report naming every artifact withheld under incident posture together with the condition that withheld it.
- **REQ-027**: Where a legacy mirror is enabled, the registry shall list each pre-migration artifact withheld under incident posture so that an operator can restore it once the incident closes.
- **REQ-028**: If a client presents an unsupported protocol version while an incident is active, then the registry shall include the incident advisory address in the error response.
- **REQ-029**: While an incident is active, the registry shall present incident status ahead of maintenance status on its status endpoint.
- **REQ-030**: While an incident is active, the registry shall report the search index as stale until the deferred index queue is drained.
- **REQ-031**: While an incident is active, the registry shall report every namespace over its storage quota on the operator's incident dashboard.
- **REQ-032**: If an artifact withheld under incident posture has never been downloaded, then the registry shall list that artifact with its age in days in the incident report.
