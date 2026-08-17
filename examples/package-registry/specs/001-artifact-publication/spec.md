# Feature Specification: Artifact Publication

**Feature**: `001-artifact-publication`
**Status**: Specified

## Summary

Accepting a publisher's upload: authenticating the request, charging the namespace quota, verifying the digest and signature before the manifest write, and handling a version that already exists.

## Requirements

- **REQ-001**: The registry shall resolve the scope of the presented access token before reading any uploaded bytes.
- **REQ-002**: When a publication request presents an unexpired token that resolves to a known publisher, arrives over a verified connection carrying write or admin scope, and stays within its rate budget, the registry shall admit the request to the publication pipeline.
- **REQ-003**: If the presented credential does not resolve to a known publisher identity, then the registry shall deny the publication request.
- **REQ-004**: If the presented access token carries anonymous or read scope, then the registry shall deny the publication request.
- **REQ-005**: If the presented access token is past its expiry time, then the registry shall deny the publication request.
- **REQ-006**: If the connection did not complete TLS verification against a trusted certificate chain, then the registry shall deny the publication request.
- **REQ-007**: While a publishing client is over its permitted request rate, the registry shall deny that client's further publication requests until the current window closes.
- **REQ-008**: The registry shall name both the scope the token carried and the scope the operation required in every denial it returns.
- **REQ-009**: Where a publishing namespace has a declared storage quota, the registry shall charge the stored size of each accepted artifact against that quota before acknowledging the publication.
- **REQ-010**: If the publishing namespace is over its allotted storage, then the registry shall reject the upload.
- **REQ-011**: If an upload is refused because its namespace is over quota, then the registry shall discard the staged bytes instead of charging them to that namespace.
- **REQ-012**: When an admitted request carries a strong digest matching the uploaded bytes, a signature that verifies against a trusted publisher key, a namespace within quota, an available audit sink, and a version that is not yanked, the registry shall accept the artifact for publication.
- **REQ-013**: The registry shall compute the digest of the uploaded bytes itself rather than trusting the digest the request declares.
- **REQ-014**: If the digest the registry computes over the uploaded bytes differs from the digest the manifest records for them, then the registry shall reject the upload.
- **REQ-015**: If a publication request records its digest under md5 or sha1, then the registry shall reject the upload.
- **REQ-016**: If the manifest entry accompanying an upload carries no publisher signature, then the registry shall reject the upload.
- **REQ-017**: If a publisher signature is present and does not verify against a trusted publisher key, then the registry shall reject the upload.
- **REQ-018**: When the registry accepts an artifact, the manifest shall record its digest, its digest algorithm, the publishing identity, and the publication time in one atomic write.
- **REQ-019**: When uploaded bytes match the digest recorded for them under a verifying signature and the namespace is within quota, the registry shall store the artifact under its content address.
- **REQ-020**: If uploaded bytes do not match the digest recorded for them, then the registry shall discard the staged bytes without writing a manifest entry.
- **REQ-021**: The manifest shall hold at most one entry for a given package name and version.
- **REQ-022**: When a publication request repeats a version already in the manifest and the uploaded bytes match the digest recorded for it, the registry shall acknowledge the existing entry without writing a second one.
- **REQ-023**: If a publication request names a version already in the manifest and the uploaded bytes differ from the digest recorded for it, then the registry shall reject the upload without altering the existing entry.
- **REQ-024**: If a publication request names a version the publisher has yanked, then the registry shall reject the upload.
- **REQ-025**: While a retention hold covers the version a publication request names, the registry shall retain the bytes already stored under that version.
- **REQ-026**: The registry shall write one audit entry for every publication decision it reaches.
- **REQ-027**: If the durable audit sink is not accepting writes, then the registry shall reject the upload.
- **REQ-028**: While an operator has declared an active security incident, the registry shall reject uploads for the duration of that incident.
- **REQ-029**: When an artifact is accepted outside a maintenance window with no incident in force, the registry shall add its manifest entry to the search index before acknowledging the publication.
- **REQ-030**: While a scheduled maintenance window is open, the registry shall defer indexing of accepted artifacts until that window closes.
- **REQ-031**: Where a manifest entry predates the strong-digest policy, the registry shall leave that entry unchanged while accepting new versions of the same package.
