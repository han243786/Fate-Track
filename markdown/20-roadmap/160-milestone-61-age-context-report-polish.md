# Milestone 61: Age Context Report Polish

## Status

- Type: quality-only report-copy loop.
- Loop: LOOP-129.
- Capability delta: none.
- Affected reports: `relationship-report`, `wealth-report`, `family-report`, `career-report`.
- Golden reference: M41-M60 report-copy baselines, with `relationship-report` remaining the first golden slice.

## Goal

M61 closes the gap where a very young chart sample could still receive adult-facing topic prose. The fixed sample uses birth date `2025-01-01` and selected year `2026`, which yields nominal age about 2. The report must not read that context as reality-level romance, income, investment, job, promotion, or career result.

The target is to keep the same chart, topic-report route, timeline evidence, restricted status, and audit boundary, while making visible prose adapt to early-stage age context:

- Relationship reads early emotional response, attachment safety, and boundary formation instead of real romance status.
- Wealth reads resource sense, care resources, sharing rules, interest support, and early boundaries instead of income or investment.
- Family reads care stability, response, emotional expression, and boundary formation.
- Career reads learning tasks, rule sense, expression training, peer boundary, and support systems instead of jobs or career outcomes.

## Scope

| Area | Required polish |
| --- | --- |
| Age context | Add an internal topic age context derived from selected report year and birth year. |
| Relationship report | Replace early-stage adult relationship guidance such as single/attached status with emotional response, attachment safety, and boundary language. |
| Wealth report | Replace early-stage adult money/investment prose across overview, middle chapters, annual trigger, topic timeline, and conclusion with resource-sense language. |
| Family report | Keep early-stage family reading centered on care, response, expression, and boundaries. |
| Career report | Replace early-stage job/workplace prose across overview, middle chapters, annual trigger, topic timeline, and conclusion with learning/growth language. |
| Tests | Add/update domain and API gates requiring early-stage anchors and rejecting adult-context regressions for the 2025/2026 sample. |
| Samples | Regenerate `main`, `relationship`, `wealth`, `family`, and `career` real samples from one consistent profile. |
| Governance | Sync roadmap index/README, risk register, capability ledger, module/full trees, recursive cursor, and closeout log. |

## Forbidden Regression Phrases

M61 specifically blocks early-stage samples from returning to adult context such as:

- `如果目前单身`
- `若已有关系`
- `工作场景`
- `现实职位高低`
- `真正适合您的事业节奏`
- `真正适合您的资源节奏`
- `真正适合您的家庭节奏`
- `现实回报`
- `长期经营`
- `可交付`
- `团队边界`

Protective negations may still mention adult domains where the sentence clearly says the report is not reading them, such as `不把它读成...`.

## Acceptance Gates

- `cargo fmt`
- `cargo test topic_report -- --nocapture`
- `cargo test report -- --nocapture`
- Regenerated five real report samples under `target/report-polish-samples/`
- Sample scan proves M61 required early-stage anchors present and adult-context regression phrases absent.
- Relationship early-stage sample does not contain `如果目前单身` or `若已有关系`.
- Wealth/career early-stage samples use resource/growth language instead of adult income/job framing.
- All regenerated topic JSON samples return top-level forbidden-output audit `passed`.
- Governance scaffold check passes.
- `git diff --check` passes.

## Non-Goals

- No new route.
- No DTO expansion.
- No capability status change.
- No public score.
- No `GET /api/luck/cycles` mutation.
- No adult-branch removal.
- No flow-month, flow-day, daily-fortune, event-prediction, or deterministic outcome scope.
