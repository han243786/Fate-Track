# Milestone 61 Closeout: Age Context Report Polish

## Result

M61 is closed as a quality-only report-copy loop. Early-stage topic reports now adapt the visible prose to nominal age about 2 and avoid adult romance, income, investment, job, or career-result framing.

## Completed Changes

| Area | Evidence |
| --- | --- |
| Age context | `backend/src/domain/topic_report.rs` now derives internal topic age context from selected report year and birth year, with early-stage branches for visible report prose. |
| Relationship report | Early-stage relationship guidance now reads emotional response, attachment safety, stable response, and boundary formation instead of single/attached romance status. |
| Wealth report | Early-stage wealth prose now reads resource sense, care resources, interest support, sharing rules, and boundaries across overview, middle chapters, trigger, timeline, and conclusion. |
| Family report | Early-stage family prose now centers care stability, response, emotional expression, rules, and boundaries. |
| Career report | Early-stage career prose now reads learning tasks, rule sense, expression training, peer boundaries, and support systems instead of workplace/job outcomes. |
| Domain/API gates | `backend/src/domain/topic_report.rs` and `backend/src/app.rs` require early-stage anchors and reject adult-context regressions in the 2025/2026 sample. |
| Samples | `target/report-polish-samples/main.json`, `relationship.json`, `wealth.json`, `family.json`, and `career.json` were regenerated from one consistent profile and extracted to `.txt` samples. |

## Validation

- `cargo fmt` passed.
- `cargo test topic_report -- --nocapture` passed 9 tests.
- `cargo test report -- --nocapture` passed 23 tests.
- Regenerated sample audit: `relationship`, `wealth`, `family`, and `career` returned `forbidden_output_audit.status = passed`; `main` has no topic audit field.
- Sample scan confirmed M61 adult-context regressions absent for:
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
- Sample scan confirmed early-stage anchors present, including `名义年龄约2岁`, `不会把`, `早年阶段`, `成长场景`, `资源不是只指钱`, `不讨论现实恋爱状态`, `照护资源是否稳定`, `学习任务`, and `稳定照护`.

- Final gates passed: governance scaffold check and `git diff --check`.

## Capability Status

No capability changes.

- V1 preview remains 10 supported, 7 restricted.
- Post-preview current runtime remains 10 supported, 14 restricted, 0 planned.
- `relationship-report`, `wealth-report`, `family-report`, `career-report`, `luck-reading`, `annual-trigger-reading`, and `topic-timeline-reading` remain restricted.

## Follow-Up Cursor

Next loop should preserve M47-M61 report-copy baselines. Future report polish should continue to start from regenerated real samples, keep the relationship report as the current golden sample, and avoid any capability expansion without a new milestone and decision gate.
