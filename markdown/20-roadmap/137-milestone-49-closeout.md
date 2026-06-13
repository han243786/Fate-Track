# M49 Closeout - Annual Timeline Narrative Baseline

## 1. Scope

M49 closes LOOP-117 as a five-report annual/timeline narrative-quality loop. It starts from regenerated real samples after M48 and targets the remaining list-like annual reading sections in the main chart report plus wealth/family/career topic reports.

## 2. Implementation

- Rewrote the main chart `年度引动` evidence section from `盘面上先看这几股牵动` plus bullet evidence into a continuous `2026年可以按这个顺序读` paragraph.
- Rewrote topic `本专题的大运流年` blocks from `2026年的主要牵动如下` plus bullet evidence into a continuous `2026年的时间气候可以按这个顺序读` paragraph.
- Preserved the reading order: heavenly stem, five-element relation, hidden-stem background, branch relation, and current major-luck participation.
- Added topic-specific year guidance for wealth, family, and career so the paragraph lands back on resource rhythm, family interaction, or career work rhythm without turning into deterministic advice.
- Extended report/topic tests and app-level visible-body guards against list-tone regressions.

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

`main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` M49 scan returned 0 hits for `主要牵动如下`, `盘面上先看这几股牵动`, `这些牵动只说明`, `· 先看天干`, `· 流年`, `不作主线`, `有一处落点`, `参与这组结构`, `这组结构说明`, and `score_internal`.

Representative visible output now includes:

- `2026年可以按这个顺序读：先看天干...当前大运也参与进来...`
- `2026年的时间气候可以按这个顺序读：先看天干...日支被2026年触动...`
- `2026年落到事业上，重点不是断岗位成败，而是看责任压力、技能表达、资源落地和协作边界能不能形成更稳定的工作节奏。`

## 5. Validation

Targeted gates passed:

- `cargo test topic_report -- --nocapture`
- `cargo test report -- --nocapture`
- `cargo test relationship -- --nocapture`
- `powershell -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .`

## 6. Next Cursor

Next loop: LOOP-118. Future report polish should preserve M47 relationship golden sample, M48 count-field narrative baseline, and M49 annual/timeline narrative baseline. Next likely slice is deeper conclusion/advice cohesion across main, wealth, family, and career reports, again starting from regenerated real samples.
