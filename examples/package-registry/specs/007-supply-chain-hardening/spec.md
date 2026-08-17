# Feature Specification: Supply Chain Hardening

**Feature**: `007-supply-chain-hardening`
**Status**: Specified

## Summary

The tightened integrity policy for the package registry: only collision-resistant digest algorithms are accepted or served, every artifact carries a publisher signature that verifies against an enrolled key, and an active incident freezes publication, replication, and deletion while every decision leaves an audit entry.

## Requirements

- **REQ-001**: The registry shall permit only sha256 and blake3 as digest algorithms for newly published artifacts.
- **REQ-002**: If the manifest records a published artifact's digest under md5 or sha1, then the registry shall reject that publication.
- **REQ-003**: If an artifact's computed digest differs from the digest recorded in the manifest, then the registry shall reject that publication.
- **REQ-004**: If a manifest entry carries no publisher signature, then the registry shall reject that publication.
- **REQ-005**: If a publisher signature is present and does not verify against a trusted publisher key, then the registry shall reject that publication.
- **REQ-006**: When an authenticated publisher submits an artifact carrying a matching sha256 or blake3 digest and a verifying signature to a registry with an available audit sink and no active incident, the registry shall accept the artifact.
- **REQ-007**: The registry shall store every accepted artifact together with the signature and digest that justified its acceptance.
- **REQ-008**: The registry shall verify each publisher signature against a key enrolled for that publisher before the artifact's publication timestamp.
- **REQ-009**: The manifest shall record the signing key identifier and signature algorithm of every accepted artifact.
- **REQ-010**: If a publisher key is revoked, then the registry shall reject publications signed by that key from the revocation timestamp onward.
- **REQ-011**: If the manifest records a requested artifact's digest under md5 or sha1, then the registry shall withhold that artifact from download.
- **REQ-012**: If a requested artifact's computed digest differs from the digest recorded in the manifest, then the registry shall withhold that artifact from download.
- **REQ-013**: If a requested artifact's publisher signature does not verify at download time, then the registry shall withhold that artifact from download.
- **REQ-014**: When a download resolves an artifact whose sha256 or blake3 digest matches the manifest and whose signature verifies on a replica no more than 900 seconds behind the primary, the registry shall serve the artifact.
- **REQ-015**: If a mirror's manifest is more than 900 seconds behind the primary, then the mirror shall withhold artifacts from download until it resynchronises.
- **REQ-016**: If a mirror is absent from the operator's trusted mirror list, then the registry shall skip replication to that mirror.
- **REQ-017**: If the manifest records an artifact's digest under md5 or sha1, then the registry shall skip replication of that artifact to every mirror.
- **REQ-018**: While a security incident is active, the registry shall skip replication to every mirror.
- **REQ-019**: When a trusted mirror synchronises an artifact carrying a verifying signature and a sha256 or blake3 digest outside an active incident, the registry shall replicate that artifact to the mirror.
- **REQ-020**: If a publication request arrives over a connection that failed TLS verification, then the registry shall deny that request.
- **REQ-021**: If a publication request presents no credential resolving to a known publisher identity, then the registry shall deny that request.
- **REQ-022**: When an authenticated publisher presents an unexpired write-scoped or admin-scoped token from a supported client over a TLS-verified connection outside an active incident, the registry shall allow the publication request.
- **REQ-023**: While a security incident is active, the registry shall reject every publication until an operator clears the incident.
- **REQ-024**: While a security incident is active, the registry shall retain every artifact that its retention policy would otherwise delete.
- **REQ-025**: While a security incident is active, the registry shall emit an audit entry for every artifact it serves.
- **REQ-026**: If the durable audit sink is not accepting writes, then the registry shall reject every publication until that sink recovers.
- **REQ-027**: When a publisher signature fails verification, the registry shall emit an audit entry recording the key identifier the signature claimed.
- **REQ-028**: The audit log shall record the artifact identity, the digest algorithm, and the presented digest for every publication rejected under the digest policy.
- **REQ-029**: The registry shall publish the date on which artifacts recorded under sha1 stop being served no later than 90 days before that date.
- **REQ-030**: The registry shall document the resigning procedure a publisher follows to move an existing release from a sha1 manifest entry to a sha256 entry.
- **REQ-031**: The registry shall name the integrity check that failed in the error it returns for a withheld download.
- **REQ-032**: Where legacy mirror support is enabled, the registry shall report the count of artifacts still recorded under a deprecated digest algorithm in its weekly integrity report.
