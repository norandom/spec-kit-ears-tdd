# Feature Specification: Artifact Verification

**Feature**: `002-artifact-verification`
**Status**: Specified

## Summary

Before a downloaded artifact is used, the registry compares its digest against the manifest, verifies its signature against an authenticated publisher key, and decides what to serve, store, replicate, and record based on how far the supplying mirror is trusted.

## Requirements

- **REQ-001**: The registry shall compute the digest of a downloaded artifact with the algorithm named in the manifest before it evaluates that artifact's signature.
- **REQ-002**: If the computed digest of a downloaded artifact differs from the digest recorded in the manifest, then the registry shall reject the artifact.
- **REQ-003**: If the computed digest of a downloaded artifact differs from the digest recorded in the manifest, then the registry shall discard the downloaded bytes instead of writing them into the verified cache.
- **REQ-004**: If the manifest names md5 as the digest algorithm, then the registry shall reject the artifact even when the digest comparison succeeds.
- **REQ-005**: If the manifest names sha1 as the digest algorithm and legacy mirror support is disabled, then the registry shall reject the artifact.
- **REQ-006**: When the manifest names sha256 or blake3 as the digest algorithm and the digest comparison, the signature check, and publisher authentication all succeed, the registry shall accept the artifact.
- **REQ-007**: Where legacy mirror support is enabled, the registry shall accept an artifact whose sha1 digest comparison, signature check, and publisher authentication all succeed.
- **REQ-008**: The registry shall compare a computed digest against a recorded digest with a constant-time comparison.
- **REQ-009**: The registry shall record the digest algorithm and the digest value it used in the verification record of each artifact.
- **REQ-010**: The registry shall document which digest algorithms it accepts for a newly published artifact and which it accepts only for an existing release.
- **REQ-011**: If an artifact carries a signature that does not verify against the publisher key recorded in the manifest, then the registry shall reject the artifact.
- **REQ-012**: If an artifact carries no signature and legacy mirror support is disabled, then the registry shall reject the artifact.
- **REQ-013**: If an artifact carries a signature that verifies under a key not bound to an authenticated publisher, then the registry shall reject the artifact.
- **REQ-014**: The registry shall report the key identifier, the signature algorithm, and the failure reason for each signature that fails verification.
- **REQ-015**: Where legacy mirror support is enabled, the registry shall mark an artifact served without a signature as unverified provenance in the response metadata.
- **REQ-016**: If the transport to a mirror presents a certificate chain that does not verify, then the registry shall deny the request that would fetch an artifact from that mirror.
- **REQ-017**: While a mirror is absent from the trusted-mirror list, the registry shall withhold every artifact obtained from that mirror.
- **REQ-018**: When a mirror is trusted, its transport is verified, its replication lag is at most 300 seconds, and no incident is active, the registry shall replicate an accepted artifact to that mirror.
- **REQ-019**: If a mirror reports a replication lag above 3600 seconds, then the registry shall skip replication to that mirror.
- **REQ-020**: The registry shall publish the trusted-mirror list and the time that list last changed on its status endpoint.
- **REQ-021**: The registry shall record in the audit log which mirror supplied the bytes of each artifact it verifies.
- **REQ-022**: When a client requests an artifact from a trusted mirror over a verified transport with a matching digest, a valid signature, and a supported client version, the registry shall serve the artifact.
- **REQ-023**: If an artifact is yanked and the request does not pin its exact digest, then the registry shall withhold that artifact from the response.
- **REQ-024**: The registry shall state the yank reason recorded in the manifest whenever it withholds a yanked artifact.
- **REQ-025**: When the digest comparison, the signature check, and publisher authentication of an artifact succeed and the storage quota is not exceeded, the registry shall store the artifact in the verified cache.
- **REQ-026**: While a retention hold applies to a cached artifact, the registry shall retain that artifact.
- **REQ-027**: When the storage quota is exceeded, a cached artifact has never been downloaded, its age is above 365 days, and no retention hold applies, the registry shall delete that cached artifact.
- **REQ-028**: If the access token presented with a verification request is expired, then the registry shall deny the request.
- **REQ-029**: When a read-scoped access token is unexpired, presented over a verified transport, and within its rate limit, the registry shall allow the artifact verification request.
- **REQ-030**: The registry shall emit an audit entry for every artifact verification outcome it reaches.
- **REQ-031**: The registry shall report a verification failure as a stable machine-readable reason code rather than as free text.
- **REQ-032**: The registry shall name, for each artifact in its index listing, the digest algorithm that artifact was verified with.
