# M55 - Current Luck Consistency And Annual Decompression

## 1. Goal

M55 continues the M41+ report-polish line as a quality-only loop. The goal is to make all five reports use the same selected-year current-luck coordinate and make annual-trigger detail easier to read without reducing professional signal density.

## 2. Scope

- Topic report API:
  - Use the same real luck-cycle context as the main chart report instead of computing topic luck cycles with fixed `days_to_jie = 0`.
  - Keep `topic`, explicit `year`, and all existing route shapes unchanged.
- Relationship, wealth, family, and career reports:
  - Replace visible `大运首段` references with selected-year `当前大运`.
  - Use selected-year current luck for spouse-palace, family/career anchor relations, and wealth/career ten-god trigger summaries.
  - Keep relationship as the golden sample: six-block structure, one opening reminder, quoted relation terms, and no count-field leakage.
- Main chart and topic annual/timeline sections:
  - Split dense annual evidence into two readable prose movements: annual ten-god/five-element approach, then hidden-stem/branch/current-luck background.
  - Replace stale `天干处先露出十神主题` with `流年天干把十神主题推到台前`.

## 3. Non-Goals

- No route, DTO top-level, capability, or frontend surface change.
- No supported promotion.
- No public `score_internal` or 0-100 score.
- No raw `GET /api/luck/cycles` mutation.
- No flow-month, flow-day, event prediction, financial result, family event, career result, partner identity, or high-risk advice claim.

## 4. Acceptance

- Regenerated `main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` use one consistent sample birth profile.
- All five regenerated JSON samples return top-level audit `passed`; `relationship`, `wealth`, `family`, and `career` end with `结论`.
- Topic reports no longer expose fixed-start luck wording:
  - no `年龄段约为1至10岁`
  - no `约在 1 至 10 岁`
  - no `大运首段`
- Main chart annual-trigger and topic timeline prose contain:
  - `2026年靠近命盘时`
  - `年度本身先露出的`
  - `流年天干把十神主题推到台前`
  - `再往下看`
  - `2026年落在当前大运`
- All five text samples scan as 0 M55/M54/M53/M52 regression hits for:
  - `大运首段`
  - `年龄段约为1至10岁`
  - `约在 1 至 10 岁`
  - `这一章会把`
  - `牵动会先落在这些位置`
  - `2026年的时间气候`
  - `先从这些层次落下去看`
  - `天干处先露出`
  - `月支这一处`
  - `日支这一处`
  - `先看天干`
  - `再看五行关系`
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
- Targeted gates pass: `cargo fmt`, `cargo test report -- --nocapture`, `cargo test topic_report -- --nocapture`, `cargo test relationship -- --nocapture`.
