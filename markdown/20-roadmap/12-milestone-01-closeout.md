# Milestone Closeout: M1 Date Layer Hardening

## 1. Scope

**Milestone**: M1 Date Layer Hardening.

**Implemented scope**:

- Strengthened `GET /api/calendar/query?date=YYYY-MM-DD` metadata.
- Preserved Android date-layer year/month/day Gan-Zhi behavior with 49 edge cases.
- Added explicit date-query error behavior for invalid, missing, out-of-range, boundary, and missing-data cases.
- Declared `calendar-date-query-v1-meta` in `/api/capabilities`.
- Added frontend Date Layer probe for the supported date-only API.
- Closed DG-002 by setting V1 official validated range to 1901-2100.

**Explicit non-goals**:

- Full four-pillar chart creation.
- Hour pillar.
- IANA timezone history.
- True solar time.
- Astronomy or ephemeris engine replacement.
- Date support outside the official 1901-2100 validation range.

## 2. Capability Status

| Capability | Before | After | Evidence |
| --- | --- | --- | --- |
| `calendar-date-query` | supported | supported within 1901-2100 with stronger tests and errors | `LOOP-005`, `LOOP-006`, ADR 0008 |
| `calendar-date-query-v1-meta` | target | supported | `LOOP-003`, `LOOP-004` |
| `frontend-date-layer-probe` | unlisted | supported date-layer surface | `LOOP-007` |
| `chart-create` | planned | planned | M1 excludes full chart and hour pillar. |
| `astronomy-engine` | target | target | M9 owns wider validation and engine replacement. |

## 3. Recursive Cursor

| Field | Value |
| --- | --- |
| `loop_id` | `LOOP-010` |
| `mode` | `milestone_loop` |
| `cursor_before` | `LOOP-010`, M1 milestone closeout trial |
| `cursor_after` | `LOOP-011`, `milestone_loop`, M2 preflight only |
| `next_resume_instruction` | Read M1 closeout, then start M2 preflight under milestone_loop without implementing chart code until M2 scope and decision gates are confirmed. |

## 4. Governance Sync

| Document | Updated? | Notes |
| --- | --- | --- |
| `markdown/20-roadmap/*` | yes | Decision gates, readiness, closeout, recursive readiness, cursor, loop log, index, and M1 docs updated. |
| `markdown/00-matrix-governance/module-tree.md` | yes | M1 closeout and ADR 0008 added to roadmap/module surface. |
| `markdown/10-overview/overview-full-feature-tree.md` | yes | ADR 0008 and M1 closeout files added to engineering tree. |
| `markdown/General_Policy.md` | no | Existing data/provenance policies cover this decision. |
| `markdown/00-matrix-governance/standard-matrix.md` | no | Existing STD-012 and recursive standards cover evidence needs. |
| `README.md` | yes | Official range and unsupported wider range documented. |
| ADR / research intake | yes | ADR 0008 added; research intake already separated 1901-2100 from wider target work. |
| recursive cursor / loop log | yes | LOOP-009 and LOOP-010 closeout path recorded. |

## 5. Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

**Result**: `tools/check-project.ps1` passed on 2026-06-08; Rust 17 tests passed, frontend 6 tests passed, governance scaffold OK.

## 6. Regression Protection

| Risk | Protection |
| --- | --- |
| Date-layer three-pillar regression | Android edge-case suite embedded in Rust tests. |
| Boundary and invalid input drift | App-layer tests for missing, invalid, out-of-range, supported boundary, and missing data source. |
| Scope creep into full chart | Capability ledger, README, module tree, and frontend copy keep M1 date-layer only. |
| Wider-range overclaim | ADR 0008, API `out_of_range`, data README, and M9 ownership. |
| Governance desync | Full project gate plus cursor/closeout logging. |

## 7. Remaining Risks

| Risk | Severity | Next action |
| --- | --- | --- |
| Public-source provenance for future astronomy-grade validation | P2 | Address in M9. |
| Full chart convention decisions | P1 | Continue through M2/M3 ruleset and chart-basis milestones. |
| User misunderstanding of unsupported wider range | P2 | Keep UI/API/README range metadata visible. |

## 8. Next Milestone Entry Check

| Required condition | Met? |
| --- | --- |
| Date-layer supported boundary is clear | yes |
| DG-002 closed | yes |
| DG-001 at least target-proposed for M2 | yes |
| DG-003 at least target-proposed for M2 | yes |
| No S0 risk blocks M2 entry | yes |
| Full gate green | yes |
