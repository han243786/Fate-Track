# M54 Closeout - Timeline Detail Narrative Warmth

## 1. Scope

M54 closes LOOP-122 as a quality-only timeline-detail warmth loop. It starts from regenerated real samples after M53 and keeps all M41-M53 report-copy gates active.

## 2. Implementation

- Main chart report:
  - Rewrote the `年度引动` bridge from calculation-list wording into selected-year annual reading prose.
  - Replaced visible major-luck and annual-trigger `从命理结构看` / `从五行流向看` / `从藏干看` checklist openings in assembled report blocks with `命理结构上` / `五行流向上` / `藏干里` / `地支关系上`.
  - Replaced annual evidence phrases such as `先看天干` / `再看五行` / `地支关系上` with connected annual-rhythm sentences.
- Wealth, family, and career reports:
  - Rewrote `本专题的大运流年` signal-story prose from shared level-list wording into topic timeline reading flow.
  - The topic timeline now explains computed lines as annual rhythm: ten-god theme, five-element approach, hidden-stem background, month-branch movement, and day-branch movement.
  - Replaced the stiff `2026年的时间气候，先从这些层次落下去看` wording with `把2026年放进...专项来看` and `会怎样在2026年变得更明显`.
- Public body gates:
  - Added guards for `这一章会把`, `牵动会先落在这些位置`, `2026年的时间气候`, `先从这些层次落下去看`, `先看天干`, `再看五行关系`, and related M54 regressions.

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

| Sample | Audit | Blocks | Last Block | Warnings |
| --- | --- | ---: | --- | ---: |
| `main.json` | passed | 10 | `年度引动` | 0 |
| `relationship.json` | passed | 6 | `结论` | 1 |
| `wealth.json` | passed | 8 | `结论` | 0 |
| `family.json` | passed | 8 | `结论` | 0 |
| `career.json` | passed | 8 | `结论` | 0 |

`main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` scanned as 0 M54/M53/M52 hits for `这一章会把`, `牵动会先落在这些位置`, `2026年的时间气候`, `先从这些层次落下去看`, `天干处先露出的`, `随着2026年的节奏`, `先看天干`, `再看五行关系`, `偏弱表示这类倾向`, `哪里需要放慢`, `哪里需要承接`, `读2026年这一层`, `基本脉络如下`, `第一优先`, `第二优先`, `原局引动主要看`, `先看这几层关系`, `不能只看流年`, `不必急着找事件结论`, `这一章只说明`, `时间气候可以按这个顺序读`, `当前资料可以按完整四柱合参`, `这一年`, `日常读法`, `日常看`, `这些牵动提醒您`, `score_internal`, and `0-100`.

Representative visible output:

- `年度引动把您选定的2026年放回命盘里，看2026年的年柱怎样与原局、当前大运发生关系。`
- `2026年靠近命盘时，先浮出来的脉络是：天干处先露出十神主题，流年丙午天干「丙」对日主「庚」形成「七杀」结构。`
- `从「金钱」专项来看，2026年要把流年、当前大运和原局放在一起读。这里看的不是单点事件，而是资源模式、产出转化、分配边界与承载节奏会怎样在2026年变得更明显，让年度信息回到金钱的实际节奏上。`
- `把2026年放进「金钱」专项来看，重点不是罗列符号，而是看几条命理线索怎样连成年度节奏。`

## 5. Validation

Gates passed:

- `cargo fmt`
- `cargo test report -- --nocapture`
- `cargo test topic_report -- --nocapture`
- `cargo test relationship -- --nocapture`
- `powershell -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .`
- `git diff --check` (exit 0; line-ending warnings only)

## 6. Next Cursor

Next loop: LOOP-123. Future report polish should preserve M47 relationship golden sample, M48 topic count-field narrative baseline, M49 annual/timeline narrative baseline, M50 wealth/family/career advice-cohesion baseline, M51 main-report tone-cohesion baseline, M52 closeout-continuity baseline, M53 report density/topic-specificity baseline, and M54 timeline-detail warmth baseline. Next likely slice is deeper paragraph-level personalization and repetition reduction across main annual-trigger plus wealth/family/career topic timeline detail without changing capability boundaries.
