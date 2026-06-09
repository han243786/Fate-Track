# M9 Preflight: Astronomy Upgrade

## 1. Cursor

| Field | Value |
| --- | --- |
| `loop_id` | `LOOP-019` |
| `mode` | `milestone_loop` |
| `milestone` | M9 |
| `invariant` | Introduce the astronomy upgrade as a governed parallel track without replacing the V1 Android baseline. |

## 2. Decision Gate Audit

| Gate | Status | M9 Handling |
| --- | --- | --- |
| DG-008 | closed for M9 preflight by ADR 0015 | Parallel first; replacement requires later ADR and dual-engine evidence. |
| DG-005 | open | Luck cycles remain planned; do not implement during astronomy preflight. |
| DG-007 | closed for M3 only | No Axum or multi-crate migration in this M9 preflight slice. |
| DG-002 | closed by ADR 0008 | V1 official range remains 1901-2100 until generated astronomy evidence supersedes it. |

## 3. Selected Slice

Use one M-sized preflight loop:

| Work Package | Included? | Notes |
| --- | --- | --- |
| M9-WP1 engine source decision | yes | ADR 0015 selects parallel strategy and validation tier. |
| M9-WP2 generated table | no | Requires actual generated data and source/toolchain selection. |
| M9-WP3 manifest/hash design | yes | Add schema/checker before generated data. |
| M9-WP4 Android comparison | planned | Add report/checker skeleton first; no data diff yet. |
| M9-WP5 difference taxonomy | yes | Define taxonomy in ADR/preflight and later checker. |
| M9-WP6 migration strategy | yes | Android remains accepted-current; replacement later ADR only. |
| M9-WP7 true solar time | no | Remains forbidden until longitude/time-equation policy is separately validated. |
| M9-WP8 2033 regression | planned | Golden case category defined; data not yet generated. |

## 4. Explicit Non-Goals

- No astronomy calculation claim.
- No wider date-range support claim.
- No true solar time or IANA timezone-history implementation.
- No removal or mutation of Android golden cases.
- No change to `calendar-date-query`, `chart-create`, or release candidate supported status.

## 5. First Implementation Slice

Before any astronomy engine code:

1. Add ADR 0015 and close DG-008 for a parallel-first strategy.
2. Add ADR 0016 and source policy for GB/T 33661, JPL Horizons, IAU SOFA, and NAIF CSPICE.
3. Add generated-data manifest schema and validation checker.
4. Add an empty/placeholder comparison report template that cannot be mistaken for generated evidence.
5. Wire the manifest checker into the full project gate only after it validates real required fields.
6. Keep `astronomy-engine` target until generated evidence exists.

## 6. Validation

Required:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

The first M9 loop may close only if the full gate stays green and the capability ledger still keeps `astronomy-engine` as target or explicitly restricted.
