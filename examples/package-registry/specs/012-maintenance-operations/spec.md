# Feature Specification: Maintenance Operations

**Feature**: `012-maintenance-operations`
**Status**: Specified

## Summary

Defines what the registry serves, refuses, deletes, replicates, indexes, and records while a scheduled maintenance window is open, including the admin break-glass path for hotfixes during an active incident.

## Requirements

- **REQ-001**: While a maintenance window is open, the registry shall serve download requests for already published releases from the read-only storage replica.
- **REQ-002**: If an artifact is yanked during an open maintenance window, then the registry shall withhold that artifact and return the recorded yank reason.
- **REQ-003**: If a stored artifact no longer matches the digest recorded in its manifest during a maintenance window, then the registry shall withhold that artifact from every download path.
- **REQ-004**: While a maintenance window is open, the registry shall stamp every download response with the window identifier and the scheduled end of the window.
- **REQ-005**: If a client announces a protocol version the registry no longer supports during a maintenance window, then the registry shall return an upgrade notice naming the lowest supported client version.
- **REQ-006**: While a maintenance window is open, the registry shall allow anonymous and read-scoped download requests that arrive within the rate limit over a verified connection with an unexpired token.
- **REQ-007**: While an incident is active inside a maintenance window, the registry shall allow admin-scoped requests presented over a verified connection with an unexpired token.
- **REQ-008**: If a request presents a write-scoped token while a maintenance window is open, then the registry shall deny the request with a status the client retries after the window closes.
- **REQ-009**: If a request arrives during a maintenance window with an expired access token, then the registry shall deny the request before any storage lookup.
- **REQ-010**: If a client without admin scope exceeds its rate limit during a maintenance window, then the registry shall deny the request.
- **REQ-011**: While a maintenance window is open, the registry shall name the scheduled end of the window in the Retry-After header of every denied write request.
- **REQ-012**: The registry shall record the token scope, the client identifier, and the window identifier in the audit log for every request it denies during a maintenance window.
- **REQ-013**: If a publish request presents a write-scoped token during a maintenance window, then the registry shall reject the artifact and leave the existing version in place.
- **REQ-014**: If an upload arrives without a publisher signature during a maintenance window, then the registry shall reject the artifact and name the missing signature in the error body.
- **REQ-015**: If an upload declares md5 or sha1 as its digest algorithm during a maintenance window, then the registry shall reject the artifact.
- **REQ-016**: When an authenticated operator publishes a signed hotfix with an admin-scoped token during an active incident inside a maintenance window, the registry shall accept the artifact.
- **REQ-017**: If the computed digest of an upload differs from the digest declared in its manifest during a maintenance window, then the registry shall discard the uploaded bytes before they reach the object store.
- **REQ-018**: If the audit sink is unavailable during a maintenance window, then the registry shall spool audit entries to durable local storage for replay after the sink returns.
- **REQ-019**: When a hotfix is accepted during a maintenance window, the registry shall record the operator identity, the incident identifier, and the window identifier in the audit log.
- **REQ-020**: While a maintenance window is open, the registry shall delete every yanked artifact that is older than 180 days, has never been downloaded, and carries no retention hold.
- **REQ-021**: If an artifact carries a retention hold during a maintenance window, then the registry shall retain that artifact whatever its age or download count.
- **REQ-022**: The registry shall publish a dry-run list of the artifacts the next reclamation pass would delete at least one day before the maintenance window opens.
- **REQ-023**: When the registry deletes an artifact during a maintenance window, the audit log shall carry that artifact's digest, its digest algorithm, and its age in days.
- **REQ-024**: When replication lag to a trusted mirror exceeds 300 seconds during a maintenance window with no incident active, the registry shall replicate the outstanding backlog to that mirror over a verified connection.
- **REQ-025**: If a mirror is absent from the trusted mirror list during a maintenance window, then the registry shall skip replication to that mirror.
- **REQ-026**: Where the legacy mirror protocol is enabled, the registry shall list every mirror it skipped during the maintenance window together with the reason for each skip.
- **REQ-027**: If replication lag to a trusted mirror exceeds 3600 seconds during a maintenance window, then the registry shall raise an operator alert naming the mirror and the observed lag.
- **REQ-028**: While a maintenance window is open with no incident active, the registry shall queue index updates instead of applying them to the live search index.
- **REQ-029**: If the search index is stale during a maintenance window, then the registry shall stamp every search response with the age of the index snapshot that answered it.
- **REQ-030**: When a request carrying an admin-scoped token is processed during a maintenance window, the registry shall emit an audit entry naming the operator, the window, and the affected artifact.
- **REQ-031**: The registry shall publish the planned start and end of every maintenance window at a stable URL at least 24 hours before that window opens.
- **REQ-032**: When a maintenance window closes, the registry shall report the number of artifacts deleted, index updates deferred, and replications skipped during that window.
