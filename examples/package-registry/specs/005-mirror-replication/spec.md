# Feature Specification: Mirror Replication

**Feature**: `005-mirror-replication`
**Status**: Specified

## Summary

The registry copies published artifacts and their manifests to configured mirrors, verifies each copy end to end, gates replication and mirror access on trust, transport, signature, and quota conditions, and keeps clients off mirrors whose replication lag exceeds the freshness threshold.

## Requirements

- **REQ-001**: The registry shall create one replication job for each artifact version and configured mirror pair.
- **REQ-002**: When a publisher publishes a new artifact version, the registry shall copy that version and its manifest to every trusted mirror over a verified connection.
- **REQ-003**: If the publisher of an artifact version was not authenticated at publication time, then the registry shall skip replication of that version to every mirror.
- **REQ-004**: If a mirror is not marked trusted, then the registry shall skip replication of the artifact to that mirror.
- **REQ-005**: If the certificate presented by a mirror endpoint fails verification, then the registry shall skip replication to that endpoint.
- **REQ-006**: If a manifest carries a signature that does not verify against the publisher key, then the registry shall skip replication of that manifest.
- **REQ-007**: If a mirror reports its storage quota as exceeded, then the registry shall skip replication to that mirror until the quota clears.
- **REQ-008**: While an incident is active, the registry shall skip replication to every mirror.
- **REQ-009**: When a replication job fails, the registry shall retry that job with exponential backoff for at most five attempts.
- **REQ-010**: If a replication job exhausts its retry budget, then the registry shall report that mirror as degraded on its status endpoint.
- **REQ-011**: When a mirror finishes downloading an artifact whose recomputed digest equals the manifest digest and whose manifest signature verifies, the mirror shall store that copy in its content-addressed pool.
- **REQ-012**: If the digest recomputed by a mirror differs from the digest recorded in the manifest, then the mirror shall discard the downloaded copy.
- **REQ-013**: If a manifest names md5 as its digest algorithm, then the registry shall reject that manifest for mirror replication.
- **REQ-014**: Where legacy mirror support is enabled, the registry shall accept a manifest whose digest algorithm is sha1 for replication to that legacy mirror.
- **REQ-015**: Where legacy mirror support is enabled, the registry shall mark every sha1 manifest as deprecated in the mirror catalog listing.
- **REQ-016**: The registry shall record the digest algorithm of every mirrored manifest in the mirror catalog.
- **REQ-017**: When a client requests an artifact from a trusted mirror whose replication lag is at most 900 seconds and whose stored digest matches the manifest, the mirror shall serve that artifact.
- **REQ-018**: While the replication lag of a mirror exceeds 900 seconds, the registry shall withhold that mirror from the download redirect pool.
- **REQ-019**: The registry shall publish the current replication lag of every mirror on its status endpoint.
- **REQ-020**: When the replication lag of a mirror exceeds 900 seconds, the registry shall raise an operator alert naming that mirror and its lag.
- **REQ-021**: The registry shall report the number of artifact versions pending replication for each mirror.
- **REQ-022**: While the catalog index of a mirror is stale, the registry shall report that mirror as out of date on its status endpoint.
- **REQ-023**: When a trusted mirror pulls the replication feed over a verified connection with an unexpired read-scoped or admin-scoped token and within its rate limit, the registry shall allow that request.
- **REQ-024**: If the access token presented by a mirror has expired, then the registry shall deny that replication feed request.
- **REQ-025**: While a retention hold applies to an artifact version, the mirror shall retain its copy of that version.
- **REQ-026**: When a mirrored copy passes 730 days of age with no recorded downloads, no retention hold, and no yank marker, the mirror shall delete that copy.
- **REQ-027**: The registry shall retain the origin copy of every published artifact version irrespective of mirror garbage collection.
- **REQ-028**: When a publisher yanks an artifact version, the registry shall propagate the yank marker to every mirror in the next replication cycle.
- **REQ-029**: Where a client version is unsupported by the mirror protocol, the registry shall omit mirror URLs from the manifest it returns to that client.
- **REQ-030**: While a maintenance window is open, the registry shall defer mirror catalog rebuilds until the window closes.
- **REQ-031**: The registry shall write an audit-log entry for every replication attempt recording the mirror, the digest, and the outcome.
- **REQ-032**: If the audit sink is unavailable, then the registry shall buffer replication audit entries on local disk until the sink accepts them.
