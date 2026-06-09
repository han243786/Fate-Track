# Milestone Closeout: M4 Analysis Engine

## 1. Scope

**Milestone**: M4 Analysis Engine.

**Implemented scope**:

- Closed DG-010 through ADR 0012 as structured fixed-template analysis only.
- Added `GET /api/analysis/snapshot`.
- Promoted `analysis-snapshot` from planned to supported.
- Added deterministic element, ten-god, hidden-stem, relation, and sensitivity metrics.
- Added fixed analysis cards, disclaimer id, and forbidden-output audit.
- Added domain and API tests for metrics, forbidden-output audit, fixed-card contract, and unknown-hour sensitivity.

**Explicit non-goals**:

- Luck cycles.
- Generated prose.
- Medical, legal, financial, fertility, death, criminality, coercive, or relationship certainty claims.
- Storage, sharing, frontend workspace, true solar time, timezone history, or astronomy replacement.

## 2. Capability Status

| Capability | Before | After | Evidence |
| --- | --- | --- | --- |
| `analysis-snapshot` | planned | supported | `backend/src/domain/analysis.rs`, `backend/src/api/analysis.rs`, Rust tests, ADR 0012 |
| `luck-cycles` | planned | planned | DG-005 remains open. |
| `chart-create` | supported | supported | M4 consumes chart-create and does not change chart behavior. |

## 3. Recursive Cursor

| Field | Value |
| --- | --- |
| `loop_id` | `LOOP-014` |
| `mode` | `milestone_loop` |
| `cursor_before` | `LOOP-014`, M4 analysis preflight |
| `cursor_after` | `LOOP-015`, `milestone_loop`, M5 case-storage preflight |
| `next_resume_instruction` | Read M4 closeout, then start M5 preflight. Resolve or explicitly scope DG-006 before storage implementation, and keep sharing public preview, cloud sync, luck cycles, generated analysis, true solar time, timezone history, and astronomy replacement out of scope unless their gates are closed. |

## 4. Governance Sync

| Document | Updated? | Notes |
| --- | --- | --- |
| `markdown/20-roadmap/*` | yes | M4 milestone, preflight, closeout, decision gates, capability ledger, cursor, and loop log updated. |
| `markdown/00-matrix-governance/module-tree.md` | yes | M4 API and domain ownership recorded. |
| `markdown/10-overview/overview-full-feature-tree.md` | yes | `api/analysis.rs`, `domain/analysis.rs`, ADR 0012, and closeout files recorded. |
| `README.md` | yes | Supported analysis route and safety boundary documented. |
| ADR / research intake | yes | ADR 0012 added; ADR 0005 remains safety policy. |
| recursive cursor / loop log | yes | LOOP-014 closeout and LOOP-015 cursor recorded. |

## 5. Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

**Result**: `tools/check-project.ps1` passed on 2026-06-08; Rust 34 tests passed, frontend 6 tests passed, governance scaffold OK.

## 6. Regression Protection

| Risk | Protection |
| --- | --- |
| Free-form analysis regression | ADR 0012 and fixed-card DTO. |
| High-risk deterministic claims | Forbidden-output audit and tests. |
| Unknown-hour uncertainty hidden | API test requires sensitivity flags. |
| Luck cycle scope creep | DG-005 remains open and `/api/luck/cycles` remains planned. |
| Capability overclaim | Capability ledger, README, module tree, and closeout list non-goals. |

## 7. Remaining Risks

| Risk | Severity | Next action |
| --- | --- | --- |
| Analysis persistence not implemented | P1 | M5 must store immutable snapshots safely. |
| Share redaction not implemented | P1 | M6 must define redaction rules. |
| Luck-cycle rules undecided | P1 | Resolve DG-005 before luck implementation. |

## 8. Next Milestone Entry Check

| Required condition | Met? |
| --- | --- |
| AnalysisSnapshot can be persisted as immutable snapshot | yes |
| Share-safe vs sensitive fields are separable | yes |
| Fixed cards exist for frontend fixtures | yes |
| Full gate green | yes |
