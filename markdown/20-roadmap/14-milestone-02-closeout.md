# Milestone Closeout: M2 Ruleset and Chart Basis

## 1. Scope

**Milestone**: M2 Ruleset and Chart Basis.

**Implemented scope**:

- Closed DG-001, DG-003, and DG-004 through ADR 0009.
- Formalized `ft-v1-default` for the chart-basis contract.
- Added `CalculationMetadata`, `BirthProfile`, `ChartRequest`, and `ChartBasis`.
- Added restricted route `GET /api/charts/basis/preview`.
- Added input validation for date, time precision, timezone, true-solar-time flag, and lunar input.
- Declared `chart-basis-preview` as `restricted` in `/api/capabilities`.

**Explicit non-goals**:

- Complete four-pillar calculation.
- Hour pillar calculation.
- IANA timezone-history resolution.
- True solar time.
- Lunar input conversion.
- Persisted chart snapshots.
- Frontend chart workspace.

## 2. Capability Status

| Capability | Before | After | Evidence |
| --- | --- | --- | --- |
| `chart-basis-preview` | planned | restricted | `backend/src/domain/bazi.rs`, `backend/src/api/chart_basis.rs`, Rust tests, ADR 0009 |
| `chart-create` | planned | planned | Full chart remains M3. |
| `settings` | planned | planned | Storage/settings remain M5. |
| `calendar-date-query` | supported | supported | M2 did not alter date-layer behavior. |

## 3. Recursive Cursor

| Field | Value |
| --- | --- |
| `loop_id` | `LOOP-011` |
| `mode` | `milestone_loop` |
| `cursor_before` | `LOOP-011`, M2 preflight |
| `cursor_after` | `LOOP-012`, `milestone_loop`, M3 chart-engine preflight |
| `next_resume_instruction` | Read M2 closeout, then start M3 preflight under milestone_loop. Implement full chart only after the M3 slice selects year/month/day/hour behavior and confirms DG-007 impact. |

## 4. Governance Sync

| Document | Updated? | Notes |
| --- | --- | --- |
| `markdown/20-roadmap/*` | yes | M2 preflight, M2 milestone doc, decision gates, capability ledger, cursor, and loop log updated. |
| `markdown/00-matrix-governance/module-tree.md` | yes | Restricted chart-basis API and M2 `bazi.rs` contract recorded. |
| `markdown/10-overview/overview-full-feature-tree.md` | yes | `api/chart_basis.rs`, ADR 0009, M2 preflight, and M2 closeout recorded. |
| `README.md` | yes | Restricted route documented. |
| ADR / research intake | yes | ADR 0009 added; ADR 0004 status updated for M2 contract. |
| recursive cursor / loop log | yes | LOOP-011 closeout and LOOP-012 cursor recorded. |

## 5. Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

**Result**: `tools/check-project.ps1` passed on 2026-06-08; Rust 24 tests passed, frontend 6 tests passed, governance scaffold OK.

## 6. Regression Protection

| Risk | Protection |
| --- | --- |
| `ft-v1-default` ambiguity | ADR 0009 and metadata tests. |
| Full chart overclaim | Capability remains `restricted`; `chart-create` remains `planned`. |
| Lunar input leakage | API returns `unsupported_capability` for `calendar=lunar`. |
| True solar time leakage | API returns `unsupported_capability` for `true_solar_time=true`. |
| Invalid time accepted | App-layer test rejects invalid exact time. |

## 7. Remaining Risks

| Risk | Severity | Next action |
| --- | --- | --- |
| Full chart behavior still unimplemented | P1 | Implement in M3 with Android date-layer evidence and explicit hour policy. |
| IANA timezone history unresolved | P1 | Keep recorded-only in M2; evaluate in M3/M9 before any timezone-sensitive support. |
| Lunar input not available | P2 | Keep planned until a future milestone explicitly scopes conversion and UI copy. |

## 8. Next Milestone Entry Check

| Required condition | Met? |
| --- | --- |
| `ft-v1-default` has one ruleset meaning | yes |
| ChartBasis expresses inputs needed by future year/month/day/hour calculation | yes |
| Unknown hour behavior is represented | yes |
| True solar time unsupported behavior is defined | yes |
| Timezone fallback is recorded-only in M2 | yes |
| Full gate green | yes |
