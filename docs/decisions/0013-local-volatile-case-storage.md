# ADR 0013: Local Volatile Case Storage

## Status

Accepted.

## Decision

M5 case storage is restricted to local, in-process, volatile storage.

This closes DG-006 for M5:

```text
No account, cloud sync, team collaboration, or cross-device persistence in M5.
```

M5 may support:

- creating a case from current chart and analysis inputs;
- reading stored cases during the same process lifetime;
- immutable chart and analysis snapshots;
- title/tags/note metadata updates;
- archive and delete semantics;
- local user preference read/update.

## Boundary

M5 does not support:

- database persistence;
- accounts;
- cloud sync;
- public sharing;
- share tokens;
- team collaboration;
- CRM or admin back office.

## Evidence Required

- Repository or equivalent storage tests.
- API tests for create/read/archive/delete/preferences.
- No public share or cloud-sync capability promotion.

## Consequences

- `case-management` and `settings` can become `restricted`.
- `share-preview` remains planned until M6.
- M6 may build share previews only from immutable snapshots, not live private state.

