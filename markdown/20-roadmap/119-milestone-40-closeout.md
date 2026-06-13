# M40 Closeout: Timeline Quality Gate

## 1. Closeout Status

M40 is closed by `LOOP-104`.

This milestone is closeout-only. It adds regression gates and governance evidence for the M34-M39 timeline reading slice, but it does not add a backend calculation rule, public API route, frontend business capability, `/api/capabilities` entry, supported promotion, public score, or raw `GET /api/luck/cycles` field.

## 2. Scope Completed

| Work Package | Result |
| --- | --- |
| M40-WP1 | Added public golden sample coverage for main chart baseline, unknown hour, annual year-only, and all four topic timeline reports |
| M40-WP2 | Added public forbidden-output checks for deterministic wealth, romance, family, career, result guarantee, and flow-month/day fortune terms |
| M40-WP3 | Reinforced no-overclaim checks: no full flow-month/day, daily fortune, event prediction, or raw `luck-cycles` pollution |
| M40-WP4 | Reinforced no-public-score checks across backend public responses and frontend UI sources |
| M40-WP5 | Added bounded-output domain gate to keep timeline readings compositional instead of static 10 x 12 or 60-jiazi template expansion |
| M40-WP6 | Required every timeline reading draft to retain professional wording, plain-language wording, and boundary language |
| M40-WP7 | Synced roadmap, risk register, capability ledger, module tree, full trees, cursor, release note, and closeout log |

## 3. Quality Gates Added

| Gate | File | Evidence |
| --- | --- | --- |
| M40 public response sweep | `backend/src/app.rs` | `m40_timeline_public_quality_gate_covers_golden_samples` |
| M40 compositional bounded-output gate | `backend/src/domain/timeline.rs` | `m40_timeline_quality_gate_keeps_compositional_output_bounded` |
| M40 frontend boundary gate | `frontend/tests/workspace-markup.test.mjs` | `keeps M40 timeline quality-gate boundaries in public UI sources` |

## 4. Boundary Assertions

| Boundary | Evidence |
| --- | --- |
| No capability change | Runtime remains 10 supported, 14 restricted, 0 planned |
| Restricted only | `luck-reading`, `annual-trigger-reading`, and `topic-timeline-reading` remain restricted |
| Raw route isolation | `GET /api/luck/cycles` public response is checked for no reading fields, topic overlay, professional/plain text, score, or 0-100 terms |
| Explicit year | Chart/topic timeline samples keep explicit `reading_year` and/or `year`; topic report still requires `year=YYYY` |
| No public score | Backend response tests and frontend source tests reject `score_internal` and `0-100` |
| No overclaim | Tests reject `流月运势`, `流日运势`, `每日运势`, deterministic wealth/romance/family/career terms, result guarantee, and certain-event language |
| No template explosion | Domain test caps lexicon size and per-draft signal/evidence/reading/text volume |

## 5. Golden Browser Samples

| Surface | URL | Expected Markers |
| --- | --- | --- |
| Main chart report | `/report.html?date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact&sex=male&reading_year=2026&year=2026` | `时间解释导航`, `大运证据`, `年度证据`, no `score_internal`, no `0-100`, no `流月运势`, no `每日运势` |
| Topic report | `/topic-report.html?topic=career&date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact&sex=unspecified&year=2026` | `本专题的时间解释`, `证据追踪`, `timeline-core-v1`, `专业解释`, `白话解释`, `边界提醒`, no public score |
| Workbench | `/` | Current luck meta and short summary only; no full topic trace, no full report block, no public score |

## 6. Validation Evidence

| Command | Result |
| --- | --- |
| `cargo test m40_timeline -- --nocapture` | passed; 2 targeted M40 tests |
| `npm.cmd run check --prefix frontend` | passed; 17 frontend tests after the topic-report boundary wording alignment |
| `cargo test -- --nocapture` | passed; 116 Rust tests plus generated-artifact, golden-fixture, Android replay, comparison-artifact, and doc tests |
| `cargo check -p minggui-desktop` | passed |
| `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1` | passed; printed pre-existing `cargo fmt --check` diffs, then Rust/frontend/governance/release/astronomy gates passed; no `cargo fmt` was applied |
| Browser verification | passed on `127.0.0.1:5173` for main chart report, career topic report, and workbench at 1280x720 and 390x844; explicit year inputs present, required timeline markers present, forbidden terms absent, and no horizontal overflow |

## 7. Next Step

M34-M40 timeline slice is closed. The next recursion cursor should wait for the next user-selected post-preview slice or a quality-only audit. No new timeline capability may be promoted to supported without a new milestone and ADR.
