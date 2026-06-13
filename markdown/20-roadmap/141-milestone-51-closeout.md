# M51 Closeout - Main Report Tone Cohesion

## 1. Scope

M51 closes LOOP-119 as a quality-only polish loop for the main chart report, with a small spillover cleanup in the career report where the same teaching-style phrase appeared.

## 2. Implementation

- Rewrote main chart chapter openings from repeated teaching prompts into direct reader-facing chart-reading prose.
- Replaced the main ten-god inventory summary such as `比肩一处` / `劫财一处` with qualitative ten-god signal language.
- Reworked main major-luck and annual-trigger bridges so the report says what the stage/year means instead of explaining the template.
- Removed the remaining career-report phrase `这一章看的是工作中...` and replaced it with a more natural reading bridge.
- Added M51 regression tests and public-body guards for `这一章看的是`, `这一章先把`, `放到日常理解里`, `最适合当作`, `可以先这样理解`, and main ten-god count-summary wording.

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

`main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` M51 scan returned 0 hits for `这一章看的是`, `这一章先把`, `放到日常理解里`, `最适合当作`, `可以先这样理解`, `比较明显的十神为：`, `比肩一处`, `劫财一处`, `2026年可以按这个顺序读`, `盘面上先看这几股牵动`, `主要牵动如下`, `这些牵动提醒您`, `日常读法`, `日常看`, `score_internal`, and `0-100`.

Representative visible output now includes:

- `主盘的第一层，是年柱、月柱、日柱、时柱这四组时间坐标。`
- `当前命盘中比较明显的十神线索为：比肩带来自我立场和同辈协作...`
- `大运真正提示的是阶段重心...`
- `2026年进入命盘时，先看这几层关系...`
- `这部分呈现的是工作中更容易出现的互动模式...`

## 5. Validation

Gates passed:

- `cargo fmt`
- `cargo test report -- --nocapture`
- `cargo test topic_report -- --nocapture`
- `cargo test relationship -- --nocapture`

Final governance gates passed after doc sync: `powershell -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .`; `git diff --check` (exit 0; line-ending warnings only).

## 6. Next Cursor

Next loop: LOOP-120. Future report polish should preserve M47 relationship golden sample, M48 topic count-field narrative baseline, M49 annual/timeline narrative baseline, M50 wealth/family/career advice-cohesion baseline, and M51 main-report tone-cohesion baseline. Next likely slice is five-report conclusion density and advice continuity.
