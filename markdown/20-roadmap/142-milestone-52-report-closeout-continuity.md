# M52 - Report Closeout Continuity

## 1. Goal

M52 continues the five-report polishing loop after M51. The target is not a new feature; it is a quality-only pass that makes report endings feel continuous and complete.

## 2. Scope

- Main chart report:
  - Remove remaining instruction-list wording from ten-god, useful-god, and annual-trigger chapters.
  - Keep selected years explicit, such as `2026年`, instead of generic `这一年`.
- Wealth, family, and career reports:
  - Move `本专题的大运流年` before `结论`, so the conclusion becomes the final reader-facing close.
  - Remove stale timeline-template phrases such as `不能只看流年`, `不必急着找事件结论`, `这一章只说明`, and `时间气候可以按这个顺序读`.
  - Replace generic conclusion sensitivity copy with topic-specific closeout language.

Relationship remains the golden sample and must keep the M41 six-block structure.

## 3. Non-Goals

- No route, DTO top-level, capability, or frontend surface change.
- No supported promotion.
- No public `score_internal` or 0-100 score.
- No raw `GET /api/luck/cycles` mutation.
- No flow-month, flow-day, event prediction, financial result, family event, career result, partner identity, or high-risk advice claim.

## 4. Acceptance

- Regenerated `main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`.
- All five text samples scan as 0 M52 hits for:
  - `基本脉络如下`
  - `第一优先`
  - `第二优先`
  - `原局引动主要看`
  - `先看这几层关系`
  - `不能只看流年`
  - `不必急着找事件结论`
  - `这一章只说明`
  - `时间气候可以按这个顺序读`
  - `当前资料可以按完整四柱合参`
  - `这一年`
  - `score_internal`
  - `0-100`
- `wealth.json`, `family.json`, and `career.json` end with `结论`.
- Targeted gates pass: `cargo fmt`, `cargo test report -- --nocapture`, `cargo test topic_report -- --nocapture`, `cargo test relationship -- --nocapture`.
