# ADR 0008: V1 Official Validation Range

## Status

Accepted.

## Decision

Fate-Track V1 official validated date-layer range is:

```text
1901-2100
```

This closes `DG-002` for M1.

## Evidence

| Evidence | Scope |
| --- | --- |
| `data/raw/lunar_data.yaml` | Project raw lunar skip table declares 1901-2100. |
| Android source project | Current date layer is ported from `D:\myproject\Perpetual calendar`. |
| `data/edge_case_test.txt` | 49 Android edge cases cover leap days, leap months, CNY boundaries, year-boundary continuity, three pillars, and selected solar terms. |
| Rust regression tests | `project_data_matches_android_edge_cases_for_three_pillars` protects the Android edge manifest. |
| API contract tests | Date-query tests cover valid boundaries, invalid dates, out-of-range dates, and missing data source behavior. |
| Research intake | Research recommends separating official verification from wider future algorithmic reach. |

## Boundary

The 1901-2100 decision is an official validation boundary, not a claim that dates outside the range are impossible to compute later.

Dates outside 1901-2100 remain unsupported for the current Android date-layer API and must return an explicit out-of-range error. Wider validation requires a later astronomy or ephemeris-backed engine, generated table manifest, hashes, and replacement golden tests.

## Consequences

- `GET /api/calendar/query?date=YYYY-MM-DD` remains supported only for the current 1901-2100 date-layer range.
- `/api/capabilities` may continue to list `calendar-date-query` and `calendar-date-query-v1-meta` as supported date-layer capabilities.
- M1 can close without carrying an open validation-range ambiguity into M2.
- M9 remains responsible for any astronomy upgrade or wider validation range.

