# Milestone Closeout: M3 Chart Engine

## 1. Scope

**Milestone**: M3 Chart Engine.

**Implemented scope**:

- Added `GET /api/charts` as M3 chart-create core.
- Promoted `chart-create` from planned to supported.
- Built chart output on M2 `ChartBasis` and M1 Android date-layer pillars.
- Returned year, month, day, and exact-time hour pillars.
- Returned `hour: null`, all hour-pillar candidates, and `unknown_hour` ambiguity flag for unknown hour.
- Added M3 calculation metadata, warnings, unsupported outputs, API tests, and domain tests.
- Kept current HTTP skeleton per ADR 0010 and chart engine scope per ADR 0011.

**Explicit non-goals**:

- `chart-detail` persisted snapshots.
- Analysis output.
- Luck cycles.
- Case storage.
- IANA timezone-history resolution.
- True solar time.
- Lunar input conversion.
- Astronomy or ephemeris replacement.

## 2. Capability Status

| Capability | Before | After | Evidence |
| --- | --- | --- | --- |
| `chart-create` | planned | supported | `backend/src/domain/bazi.rs`, `backend/src/api/charts.rs`, Rust tests, ADR 0011 |
| `chart-basis-preview` | restricted | restricted | M2 contract remains available. |
| `chart-detail` | planned | planned | Persistent snapshots remain future scope. |
| `analysis-snapshot` | planned | planned | M4 owns analysis. |

## 3. Recursive Cursor

| Field | Value |
| --- | --- |
| `loop_id` | `LOOP-013` |
| `mode` | `milestone_loop` |
| `cursor_before` | `LOOP-013`, M3 chart-create core |
| `cursor_after` | `LOOP-014`, `milestone_loop`, M4 analysis-engine preflight |
| `next_resume_instruction` | Read M3 closeout, then start M4 preflight. Build analysis on supported `chart-create` output without adding luck cycles, storage, sharing, true solar time, timezone-history resolution, or astronomy replacement. |

## 4. Governance Sync

| Document | Updated? | Notes |
| --- | --- | --- |
| `markdown/20-roadmap/*` | yes | M3 milestone, capability ledger, closeout, cursor, and loop log updated. |
| `markdown/00-matrix-governance/module-tree.md` | yes | `GET /api/charts` supported route and M3 `bazi.rs` entities recorded. |
| `markdown/10-overview/overview-full-feature-tree.md` | yes | `api/charts.rs`, ADR 0011, and chart surface recorded. |
| `README.md` | yes | Supported chart-create route and boundaries documented. |
| ADR / research intake | yes | ADR 0011 added; M9 remains responsible for astronomy/timezone-history upgrades. |
| recursive cursor / loop log | yes | LOOP-013 closeout and LOOP-014 cursor recorded. |

## 5. Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

**Result**: `tools/check-project.ps1` passed on 2026-06-08; Rust 29 tests passed, frontend 6 tests passed, governance scaffold OK.

## 6. Regression Protection

| Risk | Protection |
| --- | --- |
| Android date-layer regression | Existing 49 edge cases remain in Rust tests. |
| Unknown hour fabricated as noon | Domain and API tests require `hour: null` plus candidates. |
| Hour pillar drift | Domain test protects exact-time five-rat derivation. |
| Capability overclaim | `/api/capabilities`, README, module tree, and closeout list unsupported outputs. |
| Framework migration drift | ADR 0010 keeps current HTTP skeleton through M3. |

## 7. Remaining Risks

| Risk | Severity | Next action |
| --- | --- | --- |
| Analysis semantics not implemented | P1 | Implement M4 structured analysis only after safety policy review. |
| Hidden stems/ten gods not implemented | P1 | Add in M4 as analysis prerequisites. |
| Persistent chart snapshots absent | P2 | Implement in M5. |
| Timezone-history and true solar time absent | P1 | Keep unsupported until later scoped work/M9. |

## 8. Next Milestone Entry Check

| Required condition | Met? |
| --- | --- |
| Day Master can be retrieved from day pillar | yes |
| Pillar/StemBranch structure is stable | yes |
| Unknown-hour behavior is explicit | yes |
| Safety interpretation policy is available for M4 | yes |
| Full gate green | yes |
