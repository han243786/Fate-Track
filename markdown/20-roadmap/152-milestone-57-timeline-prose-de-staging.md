# M57 - Timeline Prose De-staging

## 1. Goal

M57 continues the M41+ report-polish line as a quality-only loop. The goal is to make main annual-trigger prose and wealth/family/career topic timeline prose read less like a staged rule explanation and more like direct reading guidance, while preserving the relationship report as the current golden sample.

## 2. Scope

- Main chart report:
  - Keep `年度引动` structure and restricted `annual-trigger-reading` carrier unchanged.
  - Replace stale teaching-style timeline phrases such as `年度本身先露出的`, `流年天干把十神主题推到台前`, `五行关系继续说明力量怎样靠近`, and broad `台前` wording.
  - Keep explicit selected-year wording and no flow-month/day or event-prediction claim.
- Wealth, family, and career reports:
  - Keep `本专题的大运流年` before `结论`.
  - Preserve topic-specific timeline evidence and current-luck context from M55.
  - Reword shared timeline evidence into topic-facing prose for resource rhythm, family position, and work pressure.
- Relationship report:
  - Preserve M41 six-block golden sample and M56 compressed conclusion.

## 3. Non-Goals

- No route, DTO top-level, capability, or frontend surface change.
- No supported promotion.
- No public `score_internal` or 0-100 score.
- No raw `GET /api/luck/cycles` mutation.
- No flow-month, flow-day, event prediction, financial result, family event, career result, partner identity, or high-risk advice claim.

## 4. Acceptance

- Regenerated `main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` use one consistent sample birth profile.
- All five regenerated JSON samples return top-level audit `passed`.
- Main `年度引动` keeps selected-year/current-luck context while avoiding:
  - `年度本身先露出的`
  - `流年天干把十神主题推到台前`
  - `天干把十神主题推到台前`
  - `主题怎样被推到台前`
  - `拿到台前观察`
  - `五行说明这股力量怎样靠近日主`
  - `五行关系继续说明力量怎样靠近`
  - `这张命盘里的`
- Wealth/family/career `本专题的大运流年` keep topic-specific phrases:
  - wealth: `预算意识`, `技能产出`, `合作分配`, `承载节奏`
  - family: `话怎样说清`, `责任怎样分明`, `家庭位置`, `情绪能不能被好好安放`
  - career: `任务标准`, `技能交付`, `资源落地`, `协作边界`
- All five text samples scan as 0 M57/M56/M55 regression hits for:
  - `年度本身先露出的`
  - `流年天干把十神主题推到台前`
  - `天干把十神主题推到台前`
  - `主题怎样被推到台前`
  - `拿到台前观察`
  - `五行说明这股力量怎样靠近日主`
  - `五行关系继续说明力量怎样靠近`
  - `再往下看，地支关系、藏干和当前大运`
  - `再往下看，藏干、原局位置和大运同场`
  - `不是罗列符号`
  - `先把预算意识`
  - `先看家里哪些话`
  - `先看任务标准`
  - `推到台前`
  - `走到台前`
  - `拿到台前`
  - `这张命盘里的`
  - `score_internal`
  - `0-100`
- Targeted gates pass: `cargo fmt`, `cargo test report -- --nocapture`, `cargo test timeline -- --nocapture`, `cargo test topic_timeline_overlay -- --nocapture`.
