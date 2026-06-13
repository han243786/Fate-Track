# M57 Closeout - Timeline Prose De-staging

## 1. Scope

M57 closes LOOP-125 as a quality-only timeline prose de-staging loop. It starts from regenerated real samples after M56, keeps all M41-M56 report-copy gates active, and adds the M57 timeline prose de-staging gate.

## 2. Implementation

- Main chart report:
  - Reworded `年度引动` evidence from staged phrases such as `年度本身先露出的`, `流年天干把十神主题推到台前`, and `五行关系继续说明力量怎样靠近` into direct selected-year reading prose.
  - Removed broad `台前` phrasing from annual-trigger visible copy and timeline lexicon copy.
  - Replaced `这张命盘里的「自己」` with `命盘中的「自己」` to satisfy the existing public-copy gate.
- Wealth, family, and career reports:
  - Reworked `本专题的大运流年` evidence story so the same shared timeline evidence lands in resource rhythm, family position, and work-pressure language.
  - Removed stale `不是罗列符号`, `先把预算意识`, `先看家里哪些话`, and `先看任务标准` style transitions.
- Public body gates:
  - Added topic-report guards for the stale M57 timeline phrases.
  - Added timeline lexicon guards for `推到台前`, `走到台前`, and `拿到台前`.

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

| Sample | Audit | Blocks | Last Block |
| --- | --- | ---: | --- |
| `main.json` | passed | 10 | `年度引动` |
| `relationship.json` | passed | 6 | `结论` |
| `wealth.json` | passed | 8 | `结论` |
| `family.json` | passed | 8 | `结论` |
| `career.json` | passed | 8 | `结论` |

`main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` scanned as 0 M57/M56/M55 hits for `年度本身先露出的`, `流年天干把十神主题推到台前`, `天干把十神主题推到台前`, `主题怎样被推到台前`, `拿到台前观察`, `五行说明这股力量怎样靠近日主`, `五行关系继续说明力量怎样靠近`, `再往下看，地支关系、藏干和当前大运`, `再往下看，藏干、原局位置和大运同场`, `不是罗列符号`, `先把预算意识`, `先看家里哪些话`, `先看任务标准`, `推到台前`, `走到台前`, `拿到台前`, `这张命盘里的`, `score_internal`, and `0-100`.

Representative visible output now includes:

- `年度线索会落在日主怎样承接压力、资源和行动节奏上`
- `天干看主题怎样被唤起，五行看这股力量怎样贴近日主`
- `把2026年放进「金钱」专项来看，年度线索要回到资源入口、能力产出和合作分配上`
- `把2026年放进「家庭」专项来看，年度线索要回到支持从哪里来、话怎样说清、责任怎样分明`
- `把2026年放进「事业」专项来看，年度线索要回到任务标准、技能交付、资源落地和协作边界上`

## 5. Validation

Gates passed:

- `cargo fmt`
- `cargo test report -- --nocapture`
- `cargo test timeline -- --nocapture`
- `cargo test topic_timeline_overlay -- --nocapture`

Governance gates:

- `powershell -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .`
- `git diff --check` (exit 0; line-ending warnings only)

## 6. Next Cursor

Next loop: LOOP-126. Future report polish should preserve M47 relationship golden sample, M48 topic count-field narrative baseline, M49 annual/timeline narrative baseline, M50 wealth/family/career advice-cohesion baseline, M51 main-report tone-cohesion baseline, M52 closeout-continuity baseline, M53 report density/topic-specificity baseline, M54 timeline-detail warmth baseline, M55 current-luck consistency baseline, M56 conclusion de-duplication baseline, and M57 timeline prose de-staging baseline. Next likely slice is deeper main-report annual-trigger condensation or wealth/family/career middle-chapter personality polishing without changing capability boundaries.
