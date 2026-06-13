# Milestone 60: Topic Timeline Reader-Facing Polish

## Status

- Type: quality-only report-copy loop.
- Loop: LOOP-128.
- Capability delta: none.
- Affected reports: `wealth-report`, `family-report`, `career-report`.
- Golden reference: M41-M59 `relationship-report` narrative baseline.

## Goal

M60 closes the gap where wealth, family, and career `本专题的大运流年` chapters still sound like the engine explaining its reading order. The target is to keep the same shared timeline evidence and restricted carrier, but make the visible chapter read like a direct topic interpretation:

- Tell the user what 2026 means inside money, family, or career.
- Keep professional terms visible where useful, but do not expose layer names as prose scaffolding.
- Replace algorithmic connectors such as `从「金钱」专项来看`, `十神与五行这一层`, and `藏干、原局位置和当前大运合到一起时`.
- Preserve explicit `year`, restricted status, trace/evidence separation, no public score, and raw `GET /api/luck/cycles` calculation-only semantics.

## Scope

| Area | Required polish |
| --- | --- |
| `wealth-report` timeline | Rewrite the topic timeline chapter around 2026 resource rhythm, budget boundary, output continuity, cooperation allocation, and rule support. |
| `family-report` timeline | Rewrite the topic timeline chapter around 2026 support, household speech, responsibility placement, boundaries, and emotional settling. |
| `career-report` timeline | Rewrite the topic timeline chapter around 2026 task pressure, skill delivery, resource landing, collaboration boundary, and sustainable action. |
| Tests | Add domain and API gates requiring reader-facing anchors and rejecting stale timeline scaffold wording. |
| Samples | Regenerate `main`, `relationship`, `wealth`, `family`, and `career` real samples from one consistent profile. |
| Governance | Sync roadmap index/README, risk register, capability ledger, module/full trees, recursive cursor, and closeout log. |

## Forbidden Regression Phrases

M60 specifically blocks the following visible topic-timeline regressions in wealth/family/career:

- `从「金钱」专项来看`
- `从「家庭」专项来看`
- `从「事业」专项来看`
- `从「`
- `把2026年放进`
- `十神与五行这一层`
- `五行相处的方式提示`
- `藏干、原局位置和当前大运合到一起时`
- `藏干、宫位关系和当前大运合到一起时`
- `本段把它作为阶段背景参考`
- `这里看的不是单点事件`
- `年度线索要回到`

The relationship report is not rewritten in this pass; it remains the current golden sample and keeps its existing six-block structure.

## Acceptance Gates

- `cargo fmt`
- `cargo test topic_report -- --nocapture`
- `cargo test report -- --nocapture`
- Regenerated five real report samples under `target/report-polish-samples/`
- Sample scan proves M60 required timeline anchors present and M60 forbidden phrases absent for wealth/family/career.
- All regenerated JSON samples return top-level forbidden-output audit `passed`.
- Governance scaffold check passes.
- `git diff --check` passes.

## Non-Goals

- No new route.
- No DTO expansion.
- No capability status change.
- No public score.
- No `GET /api/luck/cycles` mutation.
- No new timeline, flow-month, flow-day, daily-fortune, or event-prediction scope.
