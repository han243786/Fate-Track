# M46 Closeout - Report Narrative List Cleanup Gate

## 1. Scope

M46 closes LOOP-114 as a quality-only cleanup across the real generated main chart report and all four full topic reports. It starts from regenerated assembled report samples and removes remaining list/table/debug-like phrasing from visible user copy.

## 2. Implementation

- Rewrote the main annual-trigger evidence list from anchor rows into a reading sequence: `先看天干`、`再看五行`、`地支内部还藏着一层底色`、`地支关系上`.
- Rewrote topic timeline overlay prose from `盘中可用的时间线索有十条，其中五条...` to concrete topic-centered narration.
- Replaced `2026年重点看的牵动` with `2026年的主要牵动如下`.
- Replaced relationship count-table phrasing such as `出现 4 处`、`正财 0 处`、`食伤出现 1 处` with `不作主线` / `有一处落点`.
- Replaced wealth/family/career template phrases such as `这张命盘里的...信号为`、`结构上被点亮`、`从现有四柱看` and removed `的重心更偏向` double-template wording.
- Extended quoted relation-term handling from assembled report prose into JSON/trace evidence so public expandable evidence does not regress to bare `形成六冲` / `被冲牵动` wording.
- Replaced negative boundary wording that contained the audited phrase `完整流月` with `逐月细分` phrasing, keeping the same no-flow-month capability boundary without tripping public audits.
- Extended app/domain guards to reject the M46 list-tone phrases in final public report bodies.

## 3. Capability Boundary

No capability status changed.

- no new route
- no DTO top-level shape change
- no `/api/capabilities` change
- no supported promotion
- no public `score_internal` or 0-100 score
- no mutation of raw `GET /api/luck/cycles`
- no flow-month, flow-day, event schedule, daily fortune, or deterministic finance/family/career/romance claim

Post-preview runtime remains 10 supported, 14 restricted, 0 planned.

## 4. Real Output Evidence

Regenerated samples under `target/report-polish-samples/`:

| Sample | M46 forbidden hits | ASCII words |
| --- | ---: | ---: |
| `main.txt` | 0 | 0 |
| `relationship.txt` | 0 | 0 |
| `wealth.txt` | 0 | 0 |
| `family.txt` | 0 | 0 |
| `career.txt` | 0 | 0 |

The final scan checked M46 list-tone terms, M45 system-tone terms, old M44 relationship regressions, internal English ids, public score terms, backend/frontend leakage, bare relation evidence such as `形成六冲`, and flow-month overclaim phrases such as `完整流月`. The final regenerated JSON samples all returned top-level `forbidden_output_audit.status = passed`. Port `8799` was used only for temporary sample generation and no listener remained after generation.

## 5. Validation

Targeted gates passed:

- `cargo test topic_report -- --nocapture`
- `cargo test report -- --nocapture`
- regenerated `main.json`, `relationship.json`, `wealth.json`, `family.json`, and `career.json`; all five top-level audits passed
- sample `.txt` scan: 0 M45/M46/internal-English/public-score hits
- sample `.json` scan: 0 bare `形成六冲` / `被冲牵动` / `完整流月` hits

Governance scaffold gate passed after final doc sync:

- `powershell -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .`

## 6. Next Cursor

Next loop: LOOP-115. Future report polish must continue to start from regenerated real samples, preserve M41-M46 report-copy gates, and keep all topic/timeline capabilities restricted unless a new milestone and ADR explicitly change the boundary.
