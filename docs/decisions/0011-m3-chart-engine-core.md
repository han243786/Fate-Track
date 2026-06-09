# ADR 0011: M3 Chart Engine Core

## Status

Accepted.

## Decision

M3 supports chart creation through:

```text
GET /api/charts
```

The supported scope is:

- Gregorian input only;
- V1 official validated range 1901-2100;
- `ft-v1-default` ruleset metadata;
- year, month, and day pillars from the accepted Android date layer;
- exact-time hour pillar using civil two-hour branches and the five-rat stem rule;
- unknown hour returns `hour: null` plus all hour-pillar candidates;
- metadata, warnings, ambiguity flags, and unsupported output declarations.

## Boundary

M3 does not support:

- IANA timezone-history resolution;
- true solar time;
- lunar input conversion;
- astronomy/ephemeris replacement;
- persisted chart detail;
- analysis snapshot;
- luck cycles.

## Evidence

| Evidence | Scope |
| --- | --- |
| `backend/src/domain/bazi.rs` | Chart result, pillar model, exact-hour pillar, unknown-hour candidates, metadata and unsupported outputs. |
| `backend/src/api/charts.rs` | `GET /api/charts` JSON contract. |
| App-layer tests | Exact-time chart response and unknown-hour chart response. |
| Capability catalog | `chart-create` promoted to `supported`; `chart-detail` remains `planned`. |

## Consequences

- M4 may use the day pillar as Day Master source for analysis.
- M5 must create immutable chart snapshots from this chart result instead of recalculating ad hoc.
- M9 remains responsible for any astronomy or timezone-history upgrade.

