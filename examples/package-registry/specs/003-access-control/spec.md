# Feature Specification: Access Control

**Feature**: `003-access-control`
**Status**: Specified

## Summary

The registry resolves every request to exactly one token scope over a verified connection, bounds token lifetime and revocation, and permits publication, download, replication, and deletion only within what that scope authorises.

## Requirements

- **REQ-001**: The registry shall resolve every request to exactly one token scope before applying any other access rule.
- **REQ-002**: If a request arrives over a connection that has not completed TLS verification, then the registry shall deny the request.
- **REQ-003**: When a token is presented over a connection that has not completed TLS verification, the registry shall record that token identifier in the audit log as exposed.
- **REQ-004**: Where a token has been recorded as exposed, the registry shall treat that token as expired for every request presenting it afterwards.
- **REQ-005**: If a request presents an access token past its expiry time, then the registry shall deny the request rather than resolving that request to the anonymous scope.
- **REQ-006**: The registry shall evaluate token expiry against its own clock rather than against any timestamp supplied by the client.
- **REQ-007**: The registry shall issue every access token with an expiry no more than ninety days after the time of issue.
- **REQ-008**: Where a presented token expires within seven days, the registry shall report the remaining lifetime of that token in the response headers.
- **REQ-009**: The registry shall record the subject, the scope, the issue time, and the expiry of every token it issues in the audit log.
- **REQ-010**: When a publish request presents an unexpired write-scoped token from an authenticated publisher over a verified connection outside an incident and outside a maintenance window, the registry shall accept the artifact.
- **REQ-011**: If a publish request presents a token whose scope is anonymous or read, then the registry shall reject the artifact.
- **REQ-012**: If a publish request carries no credential resolving to a known publisher identity, then the registry shall reject the artifact.
- **REQ-013**: While a security incident is active, the registry shall deny every request presenting a write-scoped token.
- **REQ-014**: While a scheduled maintenance window is in effect, the registry shall deny every request presenting a write-scoped token.
- **REQ-015**: When an anonymous request asks for a published artifact over a verified connection while no incident is active and the artifact is not yanked, the registry shall serve the artifact.
- **REQ-016**: While a security incident is active, the registry shall withhold every artifact from requests resolved to the anonymous scope.
- **REQ-017**: When an unexpired read-scoped token is presented over a verified connection, the registry shall allow the request to the download and metadata endpoints.
- **REQ-018**: When an unexpired admin-scoped token is presented over a verified connection while the audit sink is accepting writes, the registry shall allow the administrative request.
- **REQ-019**: If the audit sink is not accepting writes and the request presents an admin-scoped token, then the registry shall deny the request.
- **REQ-020**: When a deletion request presents an unexpired admin-scoped token over a verified connection while the audit sink is accepting writes and no retention hold applies, the registry shall delete the artifact.
- **REQ-021**: If a deletion request presents a token whose scope is not admin, then the registry shall retain the artifact.
- **REQ-022**: When a request presenting an admin-scoped token completes, the registry shall append an entry naming the token subject, the scope, and the affected artifact to the audit log.
- **REQ-023**: When a trusted mirror presents an unexpired pull token over a verified connection, the registry shall replicate the artifact to that mirror.
- **REQ-024**: If a mirror is absent from the trusted mirror list or its pull connection is not TLS verified, then the registry shall skip replication to that mirror.
- **REQ-025**: The registry shall issue every mirror pull credential with the read scope.
- **REQ-026**: If a token request asks for a scope wider than the scope of the token authorising that request, then the registry shall deny the token request.
- **REQ-027**: Where the legacy mirror setting is enabled, the registry shall accept a change to that setting only from a request presenting an admin-scoped token.
- **REQ-028**: When an operator revokes an access token, the registry shall stop honouring that token within sixty seconds of the revocation.
- **REQ-029**: The registry shall store every access token as a salted digest rather than as recoverable plaintext.
- **REQ-030**: The registry shall list the subject, the scope, the issue time, and the last use of every active token in the operator token report.
- **REQ-031**: If a client offers no TLS version at 1.2 or above, then the registry shall refuse the connection before reading any token from it.
- **REQ-032**: The registry shall omit private-namespace artifacts from every search response answered under the anonymous scope.
