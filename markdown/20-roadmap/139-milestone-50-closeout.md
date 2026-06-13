# M50 Closeout - Topic Report Advice Cohesion

## 1. Scope

M50 closes LOOP-118 as a quality-only polish loop for wealth, family, and career topic reports. It starts from regenerated real samples after M49 and uses the relationship report as the current golden style baseline.

## 2. Implementation

- Reworked wealth, family, and career topic report openings into `总断` chapters with direct topic-centered readings.
- Replaced the remaining `日常读法` style chapters with `资源入口`, `互动位置`, and `事业用力方式`.
- Rewrote wealth/family/career annual-trigger and conclusion paragraphs to use topic advice language instead of explanation-of-the-template wording.
- Reduced repeated in-body reality disclaimers by changing the shared sensitivity text to a资料完整性/稳健读法说明, while keeping the opening disclaimers and forbidden-claim audits intact.
- Tightened app/domain tests so old wording such as `日常读法`, `日常看`, `这些牵动提醒您`, `放回这张命盘看`, `放回家庭结构里`, and `这份报告适合当作` cannot regress.

## 3. Capability Boundary

No capability status changed.

- no new route
- no DTO top-level shape change
- no `/api/capabilities` change
- no supported promotion
- no public `score_internal` or 0-100 score
- no mutation of raw `GET /api/luck/cycles`
- no flow-month, flow-day, event schedule, daily fortune, financial outcome, family event, career result, partner identity, or deterministic real-world claim

Post-preview runtime remains 10 supported, 14 restricted, 0 planned.

## 4. Real Output Evidence

Regenerated samples under `target/report-polish-samples/`:

| Sample | Audit | Blocks | Warnings |
| --- | --- | ---: | ---: |
| `main.json` | passed | 10 | 0 |
| `relationship.json` | passed | 6 | 1 |
| `wealth.json` | passed | 8 | 0 |
| `family.json` | passed | 8 | 0 |
| `career.json` | passed | 8 | 0 |

`main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` M50 scan returned 0 hits for `日常读法`, `日常看`, `这些牵动提醒您`, `放回这张命盘看`, `放回家庭结构里`, `这份报告适合当作`, `只作传统文化参考`, `不作现实承诺`, `主要牵动如下`, `盘面上先看这几股牵动`, `不作主线`, `有一处落点`, and `score_internal`.

Representative visible output now includes:

- `【资源入口】` and `您的金钱关键词是：重承接、看边界、靠产出、要规则。`
- `【互动位置】` and `您的家庭关键词是：要支持、重边界、需表达、能承接。`
- `【事业用力方式】` and `您的事业关键词是：有压力、要承接、靠技能、重边界。`

## 5. Validation

Closeout gates passed:

- `cargo test topic_report -- --nocapture`
- `cargo test report -- --nocapture`
- `cargo test relationship -- --nocapture`
- `powershell -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .`
- `git diff --check` (exit 0; line-ending warnings only)

## 6. Next Cursor

Next loop: LOOP-119. Future report polish should preserve M47 relationship golden sample, M48 count-field narrative baseline, M49 annual/timeline narrative baseline, and M50 wealth/family/career advice-cohesion baseline. Next likely slice is main chart report tone and chapter cohesion, again starting from regenerated real samples.
