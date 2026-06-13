# Milestone 59: Topic Middle-Chapter Personality Polish

## Status

- Type: quality-only report-copy loop.
- Loop: LOOP-127.
- Capability delta: none.
- Affected reports: `wealth-report`, `family-report`, `career-report`.
- Golden reference: M41-M58 `relationship-report` narrative baseline.

## Goal

M59 closes the gap where wealth, family, and career middle chapters still read like a terminology manual. The target is not to add new calculation scope. The target is to make the existing ten-god evidence speak in the same reader-facing rhythm as the relationship golden sample:

- Start from what the signal means in the user's situation.
- Keep professional terms, but explain them inside the reading rather than as detached definitions.
- Prevent textbook labels and section stubs from returning to visible report bodies.
- Preserve explicit `year`, restricted status, trace/evidence separation, and no public `score_internal`.

## Scope

| Area | Required polish |
| --- | --- |
| `wealth-report` | Rewrite the wealth-star and wealth-flow middle chapters around resource approach, resource retention, output, allocation, support, and rules. |
| `family-report` | Rewrite support, peer/output, and duty middle chapters around support, boundaries, speech, and realistic responsibility. |
| `career-report` | Rewrite responsibility, skill/resource, and collaboration middle chapters around pressure, delivery, support, resources, and boundaries. |
| Tests | Add domain and API gates requiring the new personal anchors and rejecting stale textbook wording. |
| Samples | Regenerate `main`, `relationship`, `wealth`, `family`, and `career` real samples from one consistent profile. |
| Governance | Sync roadmap index/README, risk register, capability ledger, module/full trees, recursive cursor, and closeout log. |

## Forbidden Regression Phrases

M59 specifically blocks the following visible middle-chapter regressions in wealth/family/career:

- `财星分正财和偏财`
- `传统上会把`
- `印星在家庭专项里主要看`
- `比劫在家庭专项里看`
- `财官在家庭专项里不解释`
- `官杀代表责任`
- `食伤代表表达`
- `比劫代表协作`
- `技能表达：`
- `资源落地：`
- `协作竞争：`
- `同辈边界：`
- `表达方式：`
- `责任方面，`
- `承接方面，`
- `支持与约束方面`

`从十神脉络看` is forbidden for wealth/family/career in this loop, but the relationship golden sample is left unchanged for this pass.

## Acceptance Gates

- `cargo fmt`
- `cargo test topic_report -- --nocapture`
- `cargo test report -- --nocapture`
- Regenerated five real report samples under `target/report-polish-samples/`
- Sample scan proves M59 required anchors present and M59 forbidden phrases absent for wealth/family/career.
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
