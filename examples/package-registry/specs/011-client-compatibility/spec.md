# Feature Specification: Client Compatibility

**Feature**: `011-client-compatibility`
**Status**: Specified

## Summary

The registry negotiates a protocol version with every client, keeps the read path working for clients whose version is no longer supported, and closes the publication, administrative, and incident paths to them.

## Requirements

- **REQ-001**: The registry shall list every protocol version it implements, with that version's support status, in its discovery document.
- **REQ-002**: When a client presents a protocol version identifier, the registry shall answer under the highest protocol version that both the client and the registry support.
- **REQ-003**: If a request carries no protocol version identifier, then the registry shall answer under the oldest protocol version it still supports.
- **REQ-004**: If a client presents a protocol version newer than any version the registry implements, then the registry shall answer under the newest protocol version it implements.
- **REQ-005**: If a request carries a protocol version identifier matching no defined version, then the registry shall answer with a version-negotiation error naming the oldest supported version.
- **REQ-006**: While a client's protocol version is deprecated and still supported, the registry shall include that version's sunset date in every response it returns to that client.
- **REQ-007**: The registry shall publish a protocol version's sunset date at least 180 days before that version stops being supported.
- **REQ-008**: The registry shall record the negotiated protocol version in every audit entry it writes for a request.
- **REQ-009**: The registry shall report request counts broken down by client protocol version in its operational metrics.
- **REQ-010**: When a client whose protocol version is supported sends a request over a verified connection inside its rate limit, the registry shall allow the request.
- **REQ-011**: When a client whose protocol version is unsupported sends a read-scoped or anonymous request over a verified connection inside its rate limit with no incident active, the registry shall allow the request.
- **REQ-012**: If a connection has not completed TLS verification, then the registry shall deny the request whatever protocol version the client presents.
- **REQ-013**: If a client whose protocol version is unsupported presents a write-scoped token, then the registry shall deny the publication request.
- **REQ-014**: If a client whose protocol version is unsupported presents an admin-scoped token, then the registry shall deny the administrative request.
- **REQ-015**: If a client whose protocol version is unsupported exceeds its request rate limit, then the registry shall deny the request.
- **REQ-016**: If a security incident is active, then the registry shall deny every request from a client whose protocol version is unsupported.
- **REQ-017**: When the registry denies a request from a client whose protocol version is unsupported, the registry shall encode the error in the response format defined by the protocol version that client presented.
- **REQ-018**: If a client whose protocol version is unsupported offers an artifact carrying an md5 manifest digest, then the registry shall reject that artifact.
- **REQ-019**: When an authenticated publisher on a supported client presents a write-scoped token together with a sha256 manifest digest, the registry shall accept the artifact.
- **REQ-020**: If a client whose protocol version is unsupported uploads an artifact body under a write-scoped token, then the registry shall discard that body without persisting it.
- **REQ-021**: When a client whose protocol version is unsupported resolves an artifact that is not yanked with no incident active, the registry shall serve that artifact.
- **REQ-022**: If a client whose protocol version is unsupported resolves a yanked artifact, then the registry shall withhold that artifact rather than leaving the yank flag for the client to honour.
- **REQ-023**: If a security incident is active, then the registry shall withhold artifacts from every client whose protocol version is unsupported.
- **REQ-024**: Where an artifact's manifest digest algorithm is blake3, the manifest served to an unsupported client shall also carry that artifact's sha256 digest.
- **REQ-025**: While a client's protocol version is unsupported, the manifest the registry returns shall omit fields introduced after that version.
- **REQ-026**: Where legacy mirror support is enabled, the registry shall replicate the legacy-format manifest to a trusted mirror whose replication lag is at most 3600 seconds.
- **REQ-027**: If legacy mirror support is enabled for a mirror that is not on the trusted mirror list, then the registry shall skip replication to that mirror.
- **REQ-028**: If a mirror's replication lag exceeds 3600 seconds, then the registry shall skip replication to that mirror until the lag returns inside that bound.
- **REQ-029**: If legacy mirror support is enabled for a mirror that is not on the trusted mirror list, then the registry shall write an audit entry naming that mirror.
- **REQ-030**: When the registry handles a request from a client whose protocol version is unsupported, the registry shall write an audit entry recording the version that client presented.
- **REQ-031**: Where legacy mirror support is enabled, the registry shall retain the legacy-format copy of an artifact.
- **REQ-032**: If legacy mirror support is disabled and a legacy-format copy is older than 1095 days, has never been downloaded, and carries no retention hold, then the registry shall delete that copy.
