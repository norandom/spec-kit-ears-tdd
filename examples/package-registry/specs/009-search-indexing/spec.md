# Feature Specification: Search Indexing

**Feature**: `009-search-indexing`
**Status**: Specified

## Summary

The registry indexes each verified published artifact version for search, declares and reports index staleness, and keeps queries answerable from the last committed generation while a reindex is running.

## Requirements

- **REQ-001**: The registry shall maintain exactly one search index document for each published artifact version.
- **REQ-002**: When an artifact version is committed to storage with a matching digest and a valid signature, the indexer shall write that version into the current index generation.
- **REQ-003**: If the digest recomputed at index time differs from the digest recorded in the manifest, then the indexer shall defer indexing that artifact version.
- **REQ-004**: If an artifact version carries a signature that fails verification, then the indexer shall defer indexing that version until a valid signature is recorded.
- **REQ-005**: If the digest algorithm recorded for an artifact version is md5 or sha1, then the indexer shall defer indexing that version until a sha256 digest is recorded.
- **REQ-006**: Where an artifact version carries no signature, the indexer shall place its document in the unverified partition of the index.
- **REQ-007**: While the registry is inside a maintenance window, the indexer shall hold new index documents in the pending queue.
- **REQ-008**: If the index storage quota is exceeded, then the indexer shall defer index writes until shard space is reclaimed.
- **REQ-009**: If the audit sink is unavailable, then the indexer shall defer indexing rather than commit documents without an audit trail.
- **REQ-010**: When an artifact version enters the index, the registry shall write an audit entry naming that version and the index generation that received it.
- **REQ-011**: The registry shall record every index generation commit in the audit log with its document count and elapsed build time.
- **REQ-012**: When the replication lag of the index backing store exceeds 900 seconds, the registry shall mark the index stale.
- **REQ-013**: While the index is stale, the search endpoint shall serve matches from the last committed index generation.
- **REQ-014**: While the index is stale, the search endpoint shall return the identifier of the served generation and its age in seconds with every result set.
- **REQ-015**: While a reindex is running against a current index, the search endpoint shall serve matches from the generation committed before that reindex started.
- **REQ-016**: While a reindex is running, the registry shall report the count of rebuilt documents and the estimated completion time on the operator status endpoint.
- **REQ-017**: When a reindex commits a new generation, the registry shall promote that generation to the served generation in a single atomic switch.
- **REQ-018**: If a reindex exhausts the index storage quota before committing, then the registry shall discard the partially built generation.
- **REQ-019**: If an artifact version is yanked, then the search endpoint shall withhold that version from default result sets.
- **REQ-020**: If an artifact version is yanked, then the indexer shall retain its index document so that exact-version lookups keep resolving.
- **REQ-021**: Where a query names an exact package version, the search endpoint shall return a yanked version carrying a yanked marker.
- **REQ-022**: Where an unyanked artifact version older than 1095 days has no recorded downloads and carries no retention hold, the indexer shall delete its document from the primary index.
- **REQ-023**: If a retention hold applies to an artifact version, then the indexer shall retain its index document irrespective of that version's age.
- **REQ-024**: Where a legacy mirror is enabled and trusted and its replication lag is at most 300 seconds, the registry shall replicate each committed index generation to that mirror.
- **REQ-025**: If a legacy mirror is enabled and is not trusted, then the registry shall skip replication of index generations to that mirror.
- **REQ-026**: Where a search query carries an anonymous token scope, the search endpoint shall restrict its result set to artifact versions visible without authentication.
- **REQ-027**: If the query rate limit for a client is exceeded, then the search endpoint shall deny the query with a retry-after hint.
- **REQ-028**: If a reindex is requested with a token scope below admin, then the registry shall refuse to start that reindex.
- **REQ-029**: If a search request arrives over a transport that is not TLS verified, then the registry shall deny that request.
- **REQ-030**: If an incident is active, then the search endpoint shall deny unbounded wildcard queries.
- **REQ-031**: Where a search request comes from a supported client over verified transport and within its rate limit, the search endpoint shall allow the query.
- **REQ-032**: The registry shall publish the index refresh interval and the staleness threshold in its service description document.
