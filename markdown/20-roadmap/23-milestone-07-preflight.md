# M7 Preflight: Frontend Workspace

## 1. Cursor

| Field | Value |
| --- | --- |
| `loop_id` | `LOOP-017` |
| `mode` | `milestone_loop` |
| `milestone` | M7 |
| `invariant` | Build a frontend workspace that consumes only supported/restricted backend APIs and keeps unavailable capabilities visibly planned. |

## 2. Decision Gate Audit

| Gate | Status | M7 Handling |
| --- | --- | --- |
| DG-005 | open | Luck cycles remain planned; frontend must not present a success path. |
| DG-008 | open for M9 | No astronomy replacement, true solar time, timezone history, or range expansion. |
| DG-006 | closed for M5 | Case UI remains local volatile/restricted. |
| DG-009 | closed for M6 | Share UI remains redacted/restricted and read-only. |

## 3. Selected Slice

Use one L-sized milestone loop:

| Work Package | Included? | Notes |
| --- | --- | --- |
| M7-WP1 app shell | yes | Dense workspace with connection, chart, analysis, cases, share, calendar, data, and capability panels. |
| M7-WP2 chart input | yes | Date, time, timezone, precision, case metadata. |
| M7-WP3 chart workspace | yes | Consumes `GET /api/charts`; shows pillars, warnings, and unsupported outputs. |
| M7-WP4 analysis cards | yes | Consumes `GET /api/analysis/snapshot`; shows deterministic metrics and fixed cards. |
| M7-WP5 case list/detail entry | partial | Supports local case create/list; archive/delete UI remains future. |
| M7-WP6 share preview | yes | Consumes restricted share create; shows redacted DTO fields only. |
| M7-WP7 calendar page | yes | Preserves date-layer probe. |
| M7-WP8 glossary | no | Backend glossary remains planned. |
| M7-WP9 responsive/accessibility | yes | Browser desktop/mobile DOM checks and semantic labels. |

## 4. Explicit Non-Goals

- No frontend implementation of backend-only algorithms.
- No luck cycles.
- No generated analysis prose.
- No durable sharing, cloud sync, account storage, true solar time, timezone history, or astronomy replacement.
- No glossary support until backend glossary is implemented or explicitly restricted.

## 5. Validation

Required:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Also required:

- Browser verification for desktop and mobile layout.
- API client tests for chart, analysis, case, and share routes.
