# Feature Specification: Rate Limiting

**Feature**: `008-rate-limiting`
**Status**: Specified

## Summary

The registry gives every client a scope-sized request budget over a rolling window, denies and audits traffic that exceeds it, and sheds writes, replication, and indexing under load so that downloads of published artifacts keep working.

## Requirements

- **REQ-001**: The registry shall attribute every request to one client identity taken from the presented access token, or from the source network prefix for requests that carry no token.
- **REQ-002**: The registry shall enforce a per-identity request budget over a rolling sixty-second window.
- **REQ-003**: The registry shall size each budget from the scope of the presented token, with anonymous clients receiving the smallest allowance and admin clients the largest.
- **REQ-004**: Where a client presents an unsupported client version, the registry shall apply the anonymous allowance to that client irrespective of its token scope.
- **REQ-005**: When a request carries an expired access token, the registry shall charge that request to the anonymous budget for its source network prefix.
- **REQ-006**: If a non-admin client exceeds its request budget, then the registry shall deny the request with status 429.
- **REQ-007**: While a read-scoped client on a verified TLS connection remains within its budget, the registry shall admit the request without queueing it.
- **REQ-008**: While an unexpired admin token is presented over a verified TLS connection, the registry shall admit the request even after that identity has exhausted its budget.
- **REQ-009**: If a connection fails TLS verification, then the registry shall deny the request before charging it to any budget.
- **REQ-010**: When a request is denied for exceeding its budget, the registry shall return a Retry-After header stating the whole seconds remaining in the current window.
- **REQ-011**: The registry shall return the budget ceiling, the remaining allowance, and the window reset instant as response headers on every rate limited endpoint.
- **REQ-012**: If a request is denied for exceeding its budget, then the registry shall leave the consumed allowance for that window unchanged.
- **REQ-013**: When a non-admin client is denied for exceeding its budget, the registry shall record the client identity, the endpoint, and the window boundary in the audit log.
- **REQ-014**: When an admin token is admitted past an exhausted budget, the registry shall record that bypass in the audit log.
- **REQ-015**: If the audit sink is unavailable, then the registry shall buffer each throttling decision in durable local storage until that sink accepts it.
- **REQ-016**: While an incident is active, the registry shall deny write-scoped requests so that the remaining capacity serves reads.
- **REQ-017**: While a maintenance window is open, the registry shall deny write-scoped requests with status 503.
- **REQ-018**: While an incident is active, the registry shall keep serving downloads of published artifacts to clients that remain within budget.
- **REQ-019**: If a download request exceeds the budget of a non-admin client, then the registry shall withhold the artifact rather than serve a truncated response.
- **REQ-020**: If an authenticated publisher exceeds its upload budget, then the registry shall reject the artifact without writing it to storage.
- **REQ-021**: While an authenticated publisher is inside both its upload budget and its storage quota and the service is neither in an incident nor in a maintenance window, the registry shall accept the artifact for validation.
- **REQ-022**: If the storage quota of an authenticated publisher is exhausted, then the registry shall reject the artifact before any upload budget is consumed.
- **REQ-023**: While its replication lag exceeds nine hundred seconds, the mirror shall stop accepting new replication batches until that lag falls back below the threshold.
- **REQ-024**: While an incident is active, the registry shall pause replication pushes so that the remaining capacity serves client downloads.
- **REQ-025**: While a trusted mirror is inside its lag budget and no incident is active, the registry shall replicate each newly accepted artifact to that mirror.
- **REQ-026**: While an incident is active, the registry shall defer indexing of newly accepted artifacts until that incident closes.
- **REQ-027**: When an incident closes with the index still stale, the registry shall index the deferred artifacts before it reports the index as current.
- **REQ-028**: The registry shall measure every budget window against a monotonic clock rather than against wall-clock time.
- **REQ-029**: The registry shall stagger the reset instant of each client window across the window length so that budgets do not refill simultaneously.
- **REQ-030**: If the shared counter store becomes unreachable, then the registry shall fall back to per-node budgets sized at the configured share of the global allowance.
- **REQ-031**: The registry shall cap the concurrent upload connections accepted from one client identity at the configured maximum.
- **REQ-032**: The registry shall export per-scope counters of admitted and denied requests for each completed window.
