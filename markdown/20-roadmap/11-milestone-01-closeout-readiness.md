# M1 Closeout Readiness: Date Layer Hardening

## 1. Status

Current status: `closed_by_milestone_loop`.

M1 was closed by `LOOP-010` milestone_loop after the full project gate stayed green.

## 2. Work Package Evidence

| Work Package | Status | Evidence |
| --- | --- | --- |
| M1-WP1 response metadata | done | `LOOP-003` added traceable `meta` to `GET /api/calendar/query`; Rust contract test protects source, algorithm version, ruleset id, support range, boundary policy, confidence, and limitations. |
| M1-WP2 Android edge golden tests | done | `LOOP-005` embedded all 49 Android edge cases and fixed the late-December month-pillar boundary mismatch. |
| M1-WP3 explicit error envelope | done | `LOOP-006` added `out_of_range` and App-layer tests for missing, invalid, out-of-range, boundary, and missing-data cases. |
| M1-WP4 capability declaration | done | `LOOP-004` declared `calendar-date-query-v1-meta` in `/api/capabilities`; `calendar-date-query` remains supported. |
| M1-WP5 documentation boundary | done | README, data README, module tree, engineering tree, and capability ledger document Android date-layer scope and exclusions. |
| DG-002 validation range | done | ADR 0008 closes official V1 validation range as 1901-2100. |

## 3. Capability Status

| Capability | Before M1 | After M1 | Evidence |
| --- | --- | --- | --- |
| `calendar-date-query` | supported | supported with stronger boundary tests | `LOOP-005`, `LOOP-006` |
| `calendar-date-query-v1-meta` | target | supported | `LOOP-003`, `LOOP-004` |
| `frontend-date-layer-probe` | unlisted | supported date-layer surface | `LOOP-007` |
| `chart-create` | planned | planned | M1 explicitly excludes full chart, hour pillar, timezone history, true solar time, and astronomy replacement. |
| `astronomy-engine` | target | target | M9 remains responsible for engine replacement or wider validation. |

## 4. Decision Gates

| Gate | M1 Requirement | Status |
| --- | --- | --- |
| DG-002 | V1 official validation range decided before M1 closeout | closed by ADR 0008 as 1901-2100 |
| DG-001 | Needed before M2 starts | target-proposed; does not block M1 closeout |
| DG-003 | Needed before M2 closes | target-proposed; does not block M1 closeout |

## 5. Regression Protection

| Risk | Protection |
| --- | --- |
| Three-pillar regression | 49 Android edge cases in Rust tests. |
| Unsupported range confusion | API returns explicit `out_of_range`; README/data docs state 1901-2100. |
| Full-chart scope creep | API capability ledger and frontend probe describe date-layer only. |
| Governance drift | Module tree, engineering tree, capability ledger, cursor, and loop log are updated per loop. |

## 6. Full Gate

Required command:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Latest known result before milestone-loop closeout: `tools/check-project.ps1` passed on 2026-06-08; Rust 17 tests passed, frontend 6 tests passed, governance scaffold OK.

## 7. Remaining Risks

| Risk | Severity | Next action |
| --- | --- | --- |
| Official-range evidence beyond Android table needs stronger public-source provenance | P2 | Keep 1901-2100 as V1 validated range; improve source provenance in M9 astronomy/golden-table work. |
| Wider date support | P2 | Keep unsupported until M9. |
| Full chart conventions | P1 | Continue M2/M3 only after ruleset DTO and chart basis are explicitly scoped. |

## 8. M2 Entry Readiness

| Required condition | Met? | Evidence |
| --- | --- | --- |
| Date-layer supported boundary clear | yes | ADR 0008 + README + data README + API metadata. |
| DG-001 at least target-proposed | yes | `90-decision-gates.md`. |
| DG-003 at least target-proposed | yes | `90-decision-gates.md`. |
| ChartRequest/BirthProfile date fields confirmed in research ledger | yes | Research intake and `backend/domain/bazi.rs` skeleton keep M2 as planned. |
| Full gate green | yes | `tools/check-project.ps1` passed during LOOP-009. |
