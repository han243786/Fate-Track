# M53 Closeout - Report Density And Topic Specificity

## 1. Scope

M53 closes LOOP-121 as a quality-only report density and topic-specificity loop. It starts from regenerated real samples after M52 and keeps all M41-M52 report-copy gates active.

## 2. Implementation

- Main chart report:
  - Replaced repeated per-element explanation with grouped five-element prose.
  - Added `element_distribution_plain`, which groups absent, weak, balanced, and strong elements into natural sentences.
  - Added guards against `偏弱表示这类倾向` returning to visible report copy.
- Wealth, family, and career reports:
  - Rewrote `本专题的大运流年` plain guidance so each topic names the reader-facing concerns for that topic.
  - Wealth now emphasizes budget awareness, skill output, cooperation allocation, rule carrying, and resource rhythm.
  - Family now emphasizes what needs to be said clearly at home, responsibility split, emotional placement, and steadier relationship position.
  - Career now emphasizes task standards, skill output, resource carrying, and sustainable action.
  - Replaced the stiff `读2026年这一层` wording with `落到2026年`.
- Public body gates:
  - Added guards for `偏弱表示这类倾向`, `哪里需要放慢`, `哪里需要承接`, and `读2026年这一层`.

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

`main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` scanned as 0 M53/M52 hits for `偏弱表示这类倾向`, `哪里需要放慢`, `哪里需要承接`, `读2026年这一层`, `基本脉络如下`, `第一优先`, `第二优先`, `原局引动主要看`, `先看这几层关系`, `不能只看流年`, `不必急着找事件结论`, `这一章只说明`, `时间气候可以按这个顺序读`, `当前资料可以按完整四柱合参`, `这一年`, `日常读法`, `日常看`, `这些牵动提醒您`, `score_internal`, and `0-100`.

Representative visible output:

- `木、土、金、水偏弱，不是现实能力缺失，而是这几类力量更需要靠训练、环境支持和长期习惯慢慢养出来。`
- `落到2026年，先把预算意识、技能产出、合作分配和规则承接摆清楚；机会能不能扩大，要看资源节奏是否先稳住。`
- `落到2026年，先看家里哪些话需要说明白，哪些责任要提前分清，哪些情绪需要被安放；重点是让关系位置更稳。`
- `落到2026年，先看任务标准是否清楚、技能输出能否成形、资源承接是否跟得上；压力要落成可持续行动，而不是只停在紧绷感里。`

## 5. Validation

Gates passed:

- `cargo fmt`
- `cargo test topic_report -- --nocapture`
- `cargo test report -- --nocapture`
- `cargo test relationship -- --nocapture`
- `powershell -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .`
- `git diff --check` (exit 0; line-ending warnings only)

## 6. Next Cursor

Next loop: LOOP-122. Future report polish should preserve M47 relationship golden sample, M48 topic count-field narrative baseline, M49 annual/timeline narrative baseline, M50 wealth/family/career advice-cohesion baseline, M51 main-report tone-cohesion baseline, M52 closeout-continuity baseline, and M53 report density/topic-specificity baseline. Next likely slice is paragraph-level warmth in the main annual-trigger and topic timeline details without changing capability boundaries.
