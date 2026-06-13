# M45 Closeout - Report System-Tone Cleanup Gate

## 1. Scope

M45 closes LOOP-113 as a quality-only cleanup across the real generated main chart report and all four full topic reports. It starts from regenerated assembled reports in `target/report-polish-samples/` and removes remaining system-like wording from visible user copy.

## 2. Implementation

- Rewrote main chart report sections so day-master strength, pattern, useful-god, major-luck, and annual-trigger text no longer says `当前算法`, `系统给出`, `综合评分`, or `当前报告没有收到`.
- Replaced main ten-god score-style summaries such as `比肩(1分)` with natural Chinese counts such as `比肩一处`.
- Rewrote topic ten-god group summaries from `相关信号共 ... 处` / `未见明显显性信号` into reader-facing prose such as `在盘中有一处线索` and `暂不明显`.
- Rewrote topic timeline overlay prose from `共找到` / `今年最值得留意` into concrete-year wording such as `2026年重点看的牵动`.
- Replaced `降级参考` and the no-sensitivity system sentence with natural boundary language.
- Normalized visible luck-stage labels so `第1运丙子` style evidence renders as `第一运·丙子`.
- Extended app/domain guards to reject the M45 system-tone terms in final public report bodies.

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

| Sample | Forbidden hits | ASCII words |
| --- | ---: | ---: |
| `main.txt` | 0 | 0 |
| `relationship.txt` | 0 | 0 |
| `wealth.txt` | 0 | 0 |
| `family.txt` | 0 | 0 |
| `career.txt` | 0 | 0 |

The final scan checked for M45 system-tone terms, old M44 relationship regressions, internal English ids, public score terms, year-spacing regressions, and Arabic-number luck-stage labels. Port `8794` was used only for temporary sample generation and no listener remained after generation.

## 5. Validation

Targeted gates passed:

- `cargo test report -- --nocapture`
- `cargo test timeline -- --nocapture`

Heavy gates passed:

- `cargo test -- --nocapture` passed 117 Rust tests plus generated-artifact, golden-fixture, Android replay, comparison-artifact, and doc tests.
- `npm.cmd run check --prefix frontend` passed 19 frontend tests.
- `cargo check -p minggui-desktop` passed.
- `powershell -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .` passed.
- `powershell -ExecutionPolicy Bypass -File tools/check-project.ps1` exited 0 after printing pre-existing `cargo fmt --check` diffs; Rust, frontend, governance scaffold, release candidate, and astronomy preflight gates passed.
- No `cargo fmt` was applied.

## 6. Next Cursor

Next loop: LOOP-114. Future report polish must continue to start from regenerated real samples, preserve M41-M45 report-copy gates, and keep all topic/timeline capabilities restricted unless a new milestone and ADR explicitly change the boundary.
