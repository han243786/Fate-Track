# ADR 0014: Share Token Privacy Boundary

## Status

Accepted.

## Decision

M6 share-preview is restricted to local in-process volatile share records built from immutable M5 case snapshots.

This closes DG-009 for M6:

```text
Share tokens are returned only at creation time, stored only as hashes, expire by policy, can be revoked, and public reads return only redacted snapshot DTOs with no private case state.
```

## Boundary

M6 may support:

- creating a local volatile share from an existing case snapshot;
- returning the raw token only in the create response;
- storing only `token_hash`;
- revoking a share by token;
- expiring a share by `expires_at_unix`;
- reading a public redacted DTO by token;
- returning `noindex: true` and `editable: false` in public DTOs.

M6 does not support:

- permanent public links;
- database persistence;
- accounts or ownership;
- cloud sync;
- cross-device sync;
- public directory/community browsing;
- comments or analytics;
- exposing private notes, raw title, private case id, exact birth time, location, or live case state.

## Unavailable Response Rule

Missing, expired, revoked, or invalid tokens must use the same public-facing unavailable response shape. The response must not reveal whether a private case exists.

## Consequences

- `share-preview` can become `restricted`.
- `case-management` and `settings` remain restricted.
- M7 frontend may render public share DTOs without reading private case state.
- Durable sharing requires a later ADR.
