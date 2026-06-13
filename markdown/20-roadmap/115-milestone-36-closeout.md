# M36 Closeout: Primary Chart Luck Reading

## 1. Status

`closed by LOOP-100`.

M36 closes `luck-reading` as a restricted post-preview capability. It is carried by the existing chart report route and does not mutate the raw supported `GET /api/luck/cycles` response.

## 2. Implemented Scope

| Area | Evidence |
| --- | --- |
| Domain | `backend/src/domain/timeline.rs` adds `build_major_luck_stage_foundation()` for current major-luck stage signals, evidence, professional readings, plain-language readings, warnings, and audit |
| Luck context | `backend/src/domain/luck.rs` adds `LuckCycleContext` and shared context computation so raw route and report route use the same start-age/direction basis |
| API/report | `backend/src/api/report.rs` accepts optional explicit `reading_year`, returns `luck_reading`, and expands the main report luck chapter |
| Raw boundary | `backend/src/api/luck.rs` remains raw `luck-cycles`; app tests assert no `luck-reading`, `luck_reading`, `白话说`, or `score_internal` appears there |
| Capability | `backend/src/api/capabilities.rs` declares `luck-reading` as restricted on `/api/charts/report` |
| Frontend | `frontend/src/main.js`, `frontend/src/api/client.js`, `frontend/report.html`, and `frontend/src/ui/render.js` pass explicit `reading_year`, highlight the current major-luck stage, and show a short structure summary |

## 3. Capability Delta

| Capability | Before | After |
| --- | --- | --- |
| `luck-cycles` | supported raw calculation | unchanged |
| `luck-reading` | planned | restricted |
| `annual-trigger-reading` | planned | unchanged |
| `topic-timeline-reading` | planned | unchanged |

V1 preview matrix remains frozen at 10 supported and 7 restricted. Post-preview current runtime becomes 10 supported, 12 restricted, and 2 planned timeline capabilities.

## 4. Boundary Checks

- No raw `GET /api/luck/cycles` interpretation fields.
- No public `score_internal`.
- No 0-100 fate score.
- No full flow-month, flow-day, daily fortune, or event-prediction claim.
- No finance, romance, family, career, medical, or legal advice.
- Main report and workbench wording remain structural and explanatory.

## 5. Validation

Executed:

```powershell
cargo test luck_reading -- --nocapture
cargo test timeline -- --nocapture
cargo test luck_cycles_returns_supported_after_m13 -- --nocapture
cargo test chart_report_carries_restricted_luck_reading_after_m36 -- --nocapture
npm.cmd run check --prefix frontend
cargo check -p minggui-desktop
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

Result:

- `cargo test luck_reading -- --nocapture`: 4 passed.
- `cargo test timeline -- --nocapture`: 7 passed.
- Raw route isolation app tests passed.
- `npm.cmd run check --prefix frontend`: 15 passed.
- `cargo check -p minggui-desktop`: passed.
- `tools/check-project.ps1`: exited 0; Rust 104 unit tests plus generated-artifact, golden-fixture, Android replay, comparison-artifact, doc tests, frontend 15 tests, governance scaffold, release candidate, and astronomy preflight passed. The script printed pre-existing `cargo fmt --check` diffs, including formatting suggestions in newly touched files; no `cargo fmt` was applied to avoid unrelated mechanical churn.

## 6. Next

Proceed to M37 annual trigger reading only after reading this closeout, M37 milestone, ADR 0022, capability ledger, risk register, and recursive cursor. M37 must preserve explicit-year input, no public score, no flow-month/day overclaim, and raw `luck-cycles` isolation.
