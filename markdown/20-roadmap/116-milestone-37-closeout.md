# M37 Closeout: Annual Trigger Reading

## 1. Status

`closed by LOOP-101`.

M37 closes `annual-trigger-reading` as a restricted post-preview capability. It is carried by the existing chart report route and requires explicit `year=YYYY` before any annual-trigger interpretation is generated.

## 2. Implemented Scope

| Area | Evidence |
| --- | --- |
| Domain | `backend/src/domain/timeline.rs` adds `build_annual_trigger_foundation()` for specified-year triggers against the original chart, current major-luck background, major-luck/year overlay, warning downgrade, readings, and audit |
| API/report | `backend/src/api/report.rs` parses optional explicit `year`, returns `annual_trigger_reading`, and adds the `年度引动` chapter to the chart report |
| Explicit-year boundary | Missing `year` returns `annual_trigger_reading.status = not_requested` and `annual_year_not_requested`; the API does not infer the current year |
| Raw boundary | `backend/src/app.rs` keeps raw `GET /api/luck/cycles` free of `annual-trigger-reading`, `annual_trigger_reading`, `白话说`, and `score_internal` |
| Capability | `backend/src/api/capabilities.rs` declares `annual-trigger-reading` as restricted on `/api/charts/report?year=YYYY` |
| Frontend | `frontend/src/main.js`, `frontend/src/api/client.js`, `frontend/report.html`, and `frontend/src/ui/render.js` send explicit `year` and render a short annual-trigger structure summary |

## 3. Capability Delta

| Capability | Before | After |
| --- | --- | --- |
| `luck-cycles` | supported raw calculation | unchanged |
| `luck-reading` | restricted | unchanged |
| `annual-trigger-reading` | planned | restricted |
| `topic-timeline-reading` | planned | unchanged |

V1 preview matrix remains frozen at 10 supported and 7 restricted. Post-preview current runtime becomes 10 supported, 13 restricted, and 1 planned timeline capability.

## 4. Boundary Checks

- API annual-trigger interpretation requires explicit `year=YYYY`.
- UI default year is only an initial value; the request layer sends it explicitly.
- No raw `GET /api/luck/cycles` interpretation fields.
- No public `score_internal`.
- No 0-100 fate score.
- No full flow-month, flow-day, daily fortune, date selection, or event-prediction claim.
- No finance, romance, family, career, medical, or legal advice.
- Unknown hour downgrades time-branch evidence.

## 5. Validation

Executed before full-gate closeout:

```powershell
cargo test annual_trigger -- --nocapture
cargo test timeline -- --nocapture
npm.cmd run check --prefix frontend
cargo test -- --nocapture
cargo check -p minggui-desktop
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

Result:

- `cargo test annual_trigger -- --nocapture`: 8 passed.
- `cargo test timeline -- --nocapture`: 9 passed.
- `npm.cmd run check --prefix frontend`: 15 passed.
- `cargo test -- --nocapture`: 111 Rust tests passed plus generated-artifact, golden-fixture, Android replay, comparison-artifact, and doc tests.
- `cargo check -p minggui-desktop`: passed.
- `tools/check-project.ps1`: exited 0; Rust 111 tests, frontend 15 tests, governance scaffold, release candidate, and astronomy preflight passed. The script printed pre-existing `cargo fmt --check` diffs; no `cargo fmt` was applied.
- Browser verification passed on `http://127.0.0.1:5173`: report page with `reading_year=2026&year=2026` showed 10 sections, `年度引动`, `引动年：2026`, and `白话说`; workbench showed `年度引动：` short summary; neither surface showed `score_internal`, `0-100`, `流月运势`, or `每日运势`.

## 6. Next

Proceed to M38 topic timeline overlay only after reading this closeout, M38 milestone, ADR 0022, capability ledger, risk register, and recursive cursor. M38 must preserve the explicit-year boundary, no public score, no flow-month/day overclaim, four-topic forbidden boundaries, and raw `luck-cycles` isolation.
