# Milestone Closeout: M12 Chart Detail

## 1. Scope
Implemented `GET /api/charts/detail` returning immutable chart snapshot with algo_version, ruleset_id, birth profile, pillars, warnings, and ambiguity flags.

## 2. Capability Status
| Capability | Before | After |
|---|---|---|
| `chart-detail` | planned | supported |

## 3. Evidence
- `backend/src/api/chart_detail.rs` — detail() handler
- `backend/src/api/mod.rs` — route registered
- `backend/src/api/capabilities.rs` — status updated
- `backend/src/app.rs` — chart_detail test passes at 200 OK

## 4. Validation
- `cargo test` 68 passed, 0 failed
