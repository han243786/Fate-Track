# M44 Closeout - Relationship Copy Second-Pass Gate

## 1. Scope

M44 closes LOOP-112 as a quality-only second pass on real report copy. It starts from user feedback on `relationship-report`:

- the relationship opening had become too repetitive
- relation terms such as `冲` needed visible quotation when used as explanatory concepts

The loop preserves M41's six-block relationship structure, M42's real-output human-copy gate, and M43's remaining-report visible-copy gate.

## 2. Implementation

- Replaced the single repeated relationship opening with structure-aware opening variants based on spouse-palace relations and spouse-star visibility.
- Updated relationship educational copy from bare `被冲牵动` style wording to quoted terms such as `被"冲"牵动`, `被"合"牵动`, and `被"刑害"牵动`.
- Normalized visible relation evidence so branch relations render as `形成"六冲"` / `形成"自刑"` instead of bare `形成六冲` / `形成自刑`.
- Removed remaining stiff public phrases from report copy, including `本次输入`, `本报告只做`, `结构敏感性：`, `目前可追溯证据如下`, accidental `运运`, and unnatural year spacing such as `2026 年`.
- Added app/domain guardrails so the fixed relationship opening, bare relation-trigger phrases, unquoted branch-relation evidence, internal English, machine labels, and public score terms cannot regress in assembled report text.

## 3. Capability Boundary

No capability status changed.

- no new route
- no DTO top-level shape change
- no `/api/capabilities` change
- no supported promotion
- no public `score_internal` or 0-100 score
- no mutation of raw `GET /api/luck/cycles`
- no flow-month, flow-day, event schedule, daily fortune, or deterministic romance claim

Post-preview runtime remains 10 supported, 14 restricted, 0 planned.

## 4. Real Output Evidence

Regenerated samples under `target/report-polish-samples/`:

| Sample | Forbidden hits | ASCII words |
| --- | ---: | ---: |
| `main.txt` | 0 | 0 |
| `relationship.txt` | 0 | 0 |
| `wealth.txt` | 0 | 0 |
| `family.txt` | 0 | 0 |
| `career.txt` | 0 | 0 |

The final scan checked for fixed relationship opener regression, bare `被冲牵动` / `被合牵动` / `被刑害牵动`, unquoted `形成六冲` / `形成自刑` / `形成六合` / `形成三刑` / `形成六害`, `运运`, unnatural year spacing such as `2026 年`, internal English ids, backend/frontend wording, `用户` address drift, public score terms, and prior M42/M43 machine-copy patterns.

## 5. Validation

Real-output gate passed:

- `relationship.txt` now opens with a structure-aware sentence rather than the repeated old opener.
- `relationship.txt` includes quoted relation terms such as `被"冲"牵动`, `形成"六冲"`, and `形成"自刑"`.
- All five samples scanned as 0 M44 forbidden hits, 0 ASCII word hits, and 0 `\d{4}\s+年` year-spacing hits.

Targeted gates passed:

- `cargo test topic_timeline_overlay_reuses_shared_engine_for_all_topics -- --nocapture`
- `cargo test topic_report_all_topics_return_restricted_after_m33 -- --nocapture`
- `cargo test topic_report_relationship_returns_restricted_report_with_explicit_year -- --nocapture`
- `cargo test luck_reading_report_is_restricted_traceable_and_scoreless -- --nocapture`
- `cargo test annual_trigger_report_requires_explicit_year_and_is_scoreless -- --nocapture`

Heavy gates passed:

- `cargo test -- --nocapture` passed 117 Rust tests plus generated-artifact, golden-fixture, Android replay, comparison-artifact, and doc tests.
- `npm.cmd run check --prefix frontend` passed 19 frontend tests.
- `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1` exited 0 after printing pre-existing `cargo fmt --check` diffs; Rust, frontend, governance scaffold, release candidate, and astronomy preflight gates passed.
- `cargo check -p minggui-desktop` passed.
- `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .` passed.
- No `cargo fmt` was applied.

## 6. Next Cursor

Next loop: LOOP-113. Future report polish must continue to start from regenerated real samples, preserve M41-M44 relationship gates, preserve M43 remaining-report gates, and keep all topic/timeline capabilities restricted unless a new milestone and ADR explicitly change the boundary.
