# M55 Closeout - Current Luck Consistency And Annual Decompression

## 1. Scope

M55 closes LOOP-123 as a quality-only current-luck consistency and annual decompression loop. It starts from regenerated real samples after M54 and keeps all M41-M54 report-copy gates active.

## 2. Implementation

- Topic report API:
  - Replaced fixed `compute_luck_cycles(..., 0)` with `compute_luck_cycle_context(...)` so topic reports use real birth-date-driven luck-cycle context.
  - This fixes the visible fixed-start `1至10岁` drift in topic reports.
- Topic domain:
  - Reused selected-year current-luck selection for relationship palace relations, family/career anchor relations, and wealth/career ten-god trigger summaries.
  - Replaced visible `大运首段` wording with `当前大运`.
  - Topic timeline signal-story prose now splits dense annual evidence into annual ten-god/five-element movement and hidden-stem/branch/current-luck background.
- Main chart annual trigger:
  - Split the annual evidence block into two prose movements.
  - Replaced `天干处先露出十神主题` with `流年天干把十神主题推到台前`.
- Public body gates:
  - Added guards for fixed-start luck wording, `大运首段`, stale M54 wording, and old branch-location phrases.

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

Regenerated samples under `target/report-polish-samples/` with one consistent sample birth profile:

| Sample | Audit | Blocks | Last Block | Warnings |
| --- | --- | ---: | --- | ---: |
| `main.json` | passed | 10 | `年度引动` | 0 |
| `relationship.json` | passed | 6 | `结论` | 0 |
| `wealth.json` | passed | 8 | `结论` | 0 |
| `family.json` | passed | 8 | `结论` | 0 |
| `career.json` | passed | 8 | `结论` | 0 |

`main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` scanned as 0 M55/M54/M53/M52 hits for `大运首段`, `年龄段约为1至10岁`, `约在 1 至 10 岁`, `这一章会把`, `牵动会先落在这些位置`, `2026年的时间气候`, `先从这些层次落下去看`, `天干处先露出`, `月支这一处`, `日支这一处`, `随着2026年的节奏`, `先看天干`, `再看五行关系`, `偏弱表示这类倾向`, `哪里需要放慢`, `哪里需要承接`, `读2026年这一层`, `基本脉络如下`, `第一优先`, `第二优先`, `原局引动主要看`, `先看这几层关系`, `不能只看流年`, `不必急着找事件结论`, `这一章只说明`, `时间气候可以按这个顺序读`, `当前资料可以按完整四柱合参`, `这一年`, `日常读法`, `日常看`, `这些牵动提醒您`, `score_internal`, and `0-100`.

Representative visible output:

- `2026年落在当前大运「丙子」（2至11岁），本段把它作为阶段背景参考。`
- `2026年靠近命盘时，读盘脉络会分成两个方向慢慢浮出来：`
- `年度本身先露出的，是十神和五行怎样靠近日主：流年天干把十神主题推到台前，流年丙午天干「丙」对日主「庚」形成「七杀」结构。`
- `再往下看，藏干、原局位置和大运同场会把背景补出来。`
- `夫妻宫看的是日支「午」，它是本报告观察亲密关系落点的位置。放回这张命盘来看，午与月支「子」形成"六冲"；午与年度地支「午」形成"自刑"；午与当前大运「子」形成"六冲"。`

## 5. Validation

Gates passed:

- `cargo fmt`
- `cargo test report -- --nocapture`
- `cargo test topic_report -- --nocapture`
- `cargo test relationship -- --nocapture`
- `powershell -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .`
- `git diff --check` (exit 0; line-ending warnings only)

## 6. Next Cursor

Next loop: LOOP-124. Future report polish should preserve M47 relationship golden sample, M48 topic count-field narrative baseline, M49 annual/timeline narrative baseline, M50 wealth/family/career advice-cohesion baseline, M51 main-report tone-cohesion baseline, M52 closeout-continuity baseline, M53 report density/topic-specificity baseline, M54 timeline-detail warmth baseline, and M55 current-luck consistency baseline. Next likely slice is deeper repetition reduction and topic-personality polishing across wealth/family/career without changing capability boundaries.
