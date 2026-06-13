# M38 Closeout: Topic Timeline Overlay

## 1. Status

`closed by LOOP-102`.

M38 closes `topic-timeline-reading` as a restricted post-preview capability. It is carried by the existing explicit-year topic-report route and adds a full-report-only chapter for relationship, wealth, family, and career reports.

## 2. Implemented Scope

| Area | Evidence |
| --- | --- |
| Domain | `backend/src/domain/topic_report.rs` adds `TopicTimelineOverlay` and applies a shared helper across all four topic builders |
| Shared timeline reuse | Topic overlays consume `build_annual_trigger_foundation()` plus `TimelineSignal` and `TimelineEvidence`; they do not recalculate independent timeline rules |
| Report chapter | Full topic reports add `本专题的大运流年` with `专业解释`, `白话解释`, `边界提醒`, structure signals, and shared evidence |
| Workspace boundary | The workbench remains structure-signal-only; full text blocks and trace stay on `topic-report.html` |
| Raw boundary | `GET /api/luck/cycles` remains raw supported calculation and does not contain `topic-timeline-reading`, `topic-timeline-overlay`, `topic_timeline`, or `score_internal` |
| Capability | `backend/src/api/capabilities.rs` declares `topic-timeline-reading` as restricted on `/api/charts/topic-report?topic=relationship|wealth|family|career&year=YYYY` |

## 3. Capability Delta

| Capability | Before | After |
| --- | --- | --- |
| `luck-cycles` | supported raw calculation | unchanged |
| `luck-reading` | restricted | unchanged |
| `annual-trigger-reading` | restricted | unchanged |
| `topic-timeline-reading` | planned | restricted |
| `relationship-report` | restricted | unchanged, richer full-report section |
| `wealth-report` | restricted | unchanged, richer full-report section |
| `family-report` | restricted | unchanged, richer full-report section |
| `career-report` | restricted | unchanged, richer full-report section |

V1 preview matrix remains frozen at 10 supported and 7 restricted. Post-preview current runtime becomes 10 supported, 14 restricted, and 0 planned capabilities.

## 4. Boundary Checks

- Topic overlay requires explicit `topic` and `year=YYYY`.
- No raw `GET /api/luck/cycles` interpretation fields.
- No public `score_internal`.
- No 0-100 fate score.
- No full flow-month, flow-day, daily fortune, date selection, or event-prediction claim.
- No finance, romance, family fate, career-result, medical, or legal advice.
- Four topic reports reuse shared timeline evidence instead of static success/failure tables.

## 5. Validation

Executed before closeout:

```powershell
cargo test topic_timeline -- --nocapture
npm.cmd run check --prefix frontend
cargo test -- --nocapture
cargo check -p minggui-desktop
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

Result:

- `cargo test topic_timeline -- --nocapture`: 3 passed.
- `npm.cmd run check --prefix frontend`: 15 passed.
- `cargo test -- --nocapture`: 114 Rust tests passed plus generated-artifact, golden-fixture, Android replay, comparison-artifact, and doc tests.
- `cargo check -p minggui-desktop`: passed.
- `tools/check-project.ps1`: exited 0; Rust 114 tests, frontend 15 tests, governance scaffold, release candidate, and astronomy preflight passed. The script printed pre-existing `cargo fmt --check` diffs; no `cargo fmt` was applied.
- Browser verification passed on `http://127.0.0.1:5173`: `topic-report.html?topic=career&date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact&sex=unspecified&year=2026` rendered 10 report sections and included `本专题的大运流年`, `专业解释`, `白话解释`, `边界提醒`, and `timeline-core-v1`. It did not show `score_internal`, `0-100`, `流月运势`, or `每日运势`.

## 6. Next

Proceed to M39 timeline report UI only after reading this closeout, M39 milestone, ADR 0022, capability ledger, risk register, and recursive cursor. M39 must stay UI/readability-focused: no new capability, no supported promotion, no raw `luck-cycles` mutation, no public score, and no full flow-month/day claim.
