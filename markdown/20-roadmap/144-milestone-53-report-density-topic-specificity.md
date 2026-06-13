# M53 - Report Density And Topic Specificity

## 1. Goal

M53 continues the post-M41 report polishing line as a quality-only loop. The goal is to reduce repetitive explanation density in the main chart report and make wealth, family, and career timeline prose sound more like topic-specific reading instead of a shared template.

## 2. Scope

- Main chart report:
  - Compress repeated five-element explanation into grouped prose by level.
  - Keep weak elements readable as trainable tendencies rather than repeated `偏弱表示这类倾向` style explanation.
- Wealth, family, and career reports:
  - Keep `本专题的大运流年` before `结论`.
  - Replace shared timeline filler such as `哪里需要放慢` / `哪里需要承接` with topic-specific guidance.
  - Use explicit selected-year wording such as `落到2026年`, not generic or awkward year-layer wording.

Relationship remains the golden sample and must keep the M41 six-block structure, M42/M44/M47 gates, non-repeated opening, and quoted relation terms.

## 3. Non-Goals

- No route, DTO top-level, capability, or frontend surface change.
- No supported promotion.
- No public `score_internal` or 0-100 score.
- No raw `GET /api/luck/cycles` mutation.
- No flow-month, flow-day, event prediction, financial result, family event, career result, partner identity, or high-risk advice claim.

## 4. Acceptance

- Regenerated `main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`.
- `main.txt` groups five-element readings instead of repeating per-element weak explanations.
- `wealth.txt`, `family.txt`, and `career.txt` contain topic-specific annual/timeline guidance:
  - wealth: `预算意识`, `技能产出`, `合作分配`, `资源节奏`
  - family: `家里哪些话`, `责任要提前分清`, `情绪需要被安放`, `关系位置`
  - career: `任务标准`, `技能输出`, `资源承接`, `可持续行动`
- All five text samples scan as 0 M53/M52 regression hits for:
  - `偏弱表示这类倾向`
  - `哪里需要放慢`
  - `哪里需要承接`
  - `读2026年这一层`
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
- `wealth.json`, `family.json`, and `career.json` still end with `结论`.
- Targeted gates pass: `cargo fmt`, `cargo test report -- --nocapture`, `cargo test topic_report -- --nocapture`, `cargo test relationship -- --nocapture`.
