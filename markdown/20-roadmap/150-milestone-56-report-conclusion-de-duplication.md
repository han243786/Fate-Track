# M56 - Report Conclusion De-duplication And Topic Personality

## 1. Goal

M56 continues the M41+ report-polish line as a quality-only loop. The goal is to reduce visible conclusion repetition and make wealth, family, and career closeouts read as distinct topic judgments while preserving relationship-report as the current golden sample.

## 2. Scope

- Relationship report:
  - Keep the M41 six-block structure unchanged.
  - Compress the `结论` block so it no longer repeats full spouse-star, expression, support, and annual-trigger paragraphs.
  - Preserve low-risk relationship rhythm advice, quoted relation terms, and no deterministic romance claim.
- Wealth, family, and career reports:
  - Replace generic `在这份...专项里` closeout phrases with topic-specific synthesis.
  - Keep `本专题的大运流年` before `结论`.
  - Keep each topic ending at `结论`.
- Public body gates:
  - Reject stale conclusion-template phrases and old relationship conclusion repetition.

## 3. Non-Goals

- No route, DTO top-level, capability, or frontend surface change.
- No supported promotion.
- No public `score_internal` or 0-100 score.
- No raw `GET /api/luck/cycles` mutation.
- No flow-month, flow-day, event prediction, financial result, family event, career result, partner identity, or high-risk advice claim.

## 4. Acceptance

- Regenerated `main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` use one consistent sample birth profile.
- All five regenerated JSON samples return top-level audit `passed`; `relationship`, `wealth`, `family`, and `career` end with `结论`.
- Relationship `结论` keeps:
  - `真正适合您的关系`
  - `稳定回应`
  - `现实承接`
- Wealth `结论` keeps:
  - `预算边界`
  - `产出节奏`
  - `资源流动`
- Family `结论` keeps:
  - `家庭线索的合参重点`
  - `从情绪拉扯回到可以承接的位置`
- Career `结论` keeps:
  - `事业线索的合参重点`
  - `从压力变成可持续行动`
- All five text samples scan as 0 M56/M55/M54 regression hits for:
  - `在这份金钱专项里`
  - `在这份家庭专项里`
  - `在这份事业专项里`
  - `表达与安全感则落在日常相处里`
  - `以目前资料来看，这份情感专项可以把重点放在`
  - `在同一张桌上慢慢理清`
  - `大运首段`
  - `年龄段约为1至10岁`
  - `约在 1 至 10 岁`
  - `天干处先露出`
  - `月支这一处`
  - `日支这一处`
  - `先看天干`
  - `再看五行关系`
  - `score_internal`
  - `0-100`
- Targeted gates pass: `cargo fmt`, `cargo test report -- --nocapture`, `cargo test topic_report -- --nocapture`, `cargo test relationship -- --nocapture`.
