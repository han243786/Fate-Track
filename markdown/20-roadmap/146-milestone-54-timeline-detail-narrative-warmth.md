# M54 - Timeline Detail Narrative Warmth

## 1. Goal

M54 continues the M41+ report-polish line as a quality-only loop. The goal is to make the main chart annual-trigger detail and wealth/family/career topic timeline detail read like annual reading prose instead of calculation-layer lists.

## 2. Scope

- Main chart report:
  - Keep `年度引动` restricted and explicit-year only.
  - Replace bridge wording such as `这一章会把` and `牵动会先落在这些位置` with reader-facing annual rhythm prose.
  - Present major-luck and annual-trigger readings as `命理结构上` / `五行流向上` / `藏干里` / `地支关系上`, not repeated `从...看` checklist openings.
- Wealth, family, and career reports:
  - Keep `本专题的大运流年` before `结论`.
  - Replace shared `2026年的时间气候，先从这些层次落下去看` style prose with topic-specific annual rhythm prose.
  - Keep the M53 topic-specific concerns while explaining the computed ten-god, five-element, hidden-stem, and branch-relation lines as connected reading flow.

Relationship remains the golden sample and must keep the M41 six-block structure, M42/M44/M47 gates, non-repeated opening, quoted relation terms, and single opening reminder.

## 3. Non-Goals

- No route, DTO top-level, capability, or frontend surface change.
- No supported promotion.
- No public `score_internal` or 0-100 score.
- No raw `GET /api/luck/cycles` mutation.
- No flow-month, flow-day, event prediction, financial result, family event, career result, partner identity, or high-risk advice claim.

## 4. Acceptance

- Regenerated `main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`.
- `main.txt` contains annual-trigger wording such as:
  - `年度引动把您选定的2026年放回命盘里`
  - `2026年靠近命盘时`
  - `天干处先露出十神主题`
  - `命理结构上`
  - `五行流向上`
  - `藏干里`
- `wealth.txt`, `family.txt`, and `career.txt` contain topic timeline wording such as:
  - `会怎样在2026年变得更明显`
  - `把2026年放进`
  - `年度节奏`
  - `天干处先露出十神主题`
- All five text samples scan as 0 M54/M53/M52 regression hits for:
  - `这一章会把`
  - `牵动会先落在这些位置`
  - `2026年的时间气候`
  - `先从这些层次落下去看`
  - `天干处先露出的`
  - `随着2026年的节奏`
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
- `wealth.json`, `family.json`, and `career.json` still end with `结论`.
- Targeted gates pass: `cargo fmt`, `cargo test report -- --nocapture`, `cargo test topic_report -- --nocapture`, `cargo test relationship -- --nocapture`.
