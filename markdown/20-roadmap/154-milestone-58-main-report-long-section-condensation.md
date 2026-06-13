# M58 - Main Report Long-Section Condensation

## 1. Goal

M58 continues the M41+ report-polish line as a quality-only loop. The goal is to make the main chart report easier to finish reading by condensing the longest visible sections while preserving relationship-report as the current golden sample and keeping all timeline trace/evidence available for governance.

## 2. Scope

- Main chart report:
  - Condense `十神关系` from per-ten-god mini glossary prose into grouped signal reading.
  - Condense `大运走势` visible copy by using plain-reading summary instead of professional + plain double output.
  - Condense `年度引动` visible copy by keeping annual evidence story and using plain-reading summary for the user-facing interpretation.
  - Replace generic `这条十神线索` wording with concrete signal names such as `七杀信号`.
- Relationship report:
  - Preserve M41 six-block golden sample, M44 relation-term quoting, M47 count-field baseline, and M56 compressed conclusion.
- Wealth, family, and career reports:
  - Preserve M48-M57 baselines and do not rewrite topic chapters in this loop.

## 3. Non-Goals

- No route, DTO top-level, capability, or frontend surface change.
- No supported promotion.
- No public `score_internal` or 0-100 score.
- No raw `GET /api/luck/cycles` mutation.
- No flow-month, flow-day, event prediction, financial result, family event, career result, partner identity, or high-risk advice claim.
- No removal of internal `signals`, `evidence`, `readings`, or audit trace from restricted report carriers.

## 4. Acceptance

- Regenerated `main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` use one consistent sample birth profile.
- All five regenerated JSON samples return top-level audit `passed`.
- Main visible sections are materially shorter than the M57 samples:
  - `十神关系` should no longer include one paragraph per active ten-god.
  - `大运走势` should no longer render professional + plain reading pairs.
  - `年度引动` should no longer render professional + plain reading pairs.
- Main report must keep:
  - `若收成几组力量`
  - `把这些线索收回到实际读法`
  - `2026年的天干让七杀信号更醒目` when the sample annual stem maps to 七杀.
- All five text samples scan as 0 M58/M57/M56 regression hits for:
  - `读这一章时`
  - `这条线已经进入命盘视野`
  - `这条十神线索`
  - `命理结构上，当前阶段大运`
  - `五行流向上，`
  - `藏干里，`
  - `地支关系上，`
  - `年度本身先露出的`
  - `流年天干把十神主题推到台前`
  - `五行关系继续说明力量怎样靠近`
  - `不是罗列符号`
  - `推到台前`
  - `走到台前`
  - `拿到台前`
  - `score_internal`
  - `0-100`
- Targeted gates pass: `cargo fmt`, `cargo test report -- --nocapture`, sample scan, governance scaffold, and `git diff --check`.
