# M43 Closeout - Remaining Report Human Copy Gate

## 1. Scope

M43 closes LOOP-111 as a quality-only report-copy gate. It covers the remaining visible report exits after M42:

- main chart assembled report
- wealth assembled report
- family assembled report
- career assembled report

The relationship report remains protected by the M41 six-block structure and M42 real-output human-copy gate.

## 2. Implementation

- Removed visible `timeline-core-v1`, `annual-trigger`, `annual-current-luck`, pipe-form evidence, equality-count evidence, and raw rule-version wording from assembled report text.
- Rewrote topic timeline evidence into reader-facing Chinese labels such as `流年信号`, `当前大运信号`, and `大运流年叠加`.
- Replaced `当前提取结果`, `观察年度`, `筛出`, and `共享证据` with natural explanatory copy.
- Replaced equality-count summaries such as `正财=0、偏财=1` with natural count phrasing.
- Normalized public evidence formatting: full-width parentheses, age ranges such as `1至10岁`, and relation text such as `子与午形成六冲`.
- Kept internal trace/source fields available for governance and debugging without exposing them in assembled report prose.

## 3. Capability Boundary

No capability status changed.

- no new route
- no DTO top-level shape change
- no `/api/capabilities` change
- no supported promotion
- no public `score_internal` or 0-100 score
- no mutation of raw `GET /api/luck/cycles`
- no full flow-month, flow-day, event schedule, daily fortune, or deterministic event claim

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

The final scan also checked for `timeline-core-v1`, `annual-trigger`, `annual-current-luck`, `major-luck`, `score_internal`, `当前提取结果`, `观察年度`, `筛出`, `共享证据`, `规则版本`, equality-count forms, pipe-form evidence, ASCII age ranges, spaced branch relations, and `本次看的年份`.

## 5. Validation

Real-output gate passed:

- `main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` were regenerated under `target/report-polish-samples/`.
- All five samples scanned as 0 M43 forbidden hits and 0 ASCII word hits.

Targeted gates passed:

- `cargo test topic_timeline_overlay_reuses_shared_engine_for_all_topics -- --nocapture`
- `cargo test annual_trigger_report_requires_explicit_year_and_is_scoreless -- --nocapture`
- `cargo test m40_timeline_public_quality_gate_covers_golden_samples -- --nocapture`

Heavy gates passed:

- `cargo test -- --nocapture` passed 117 Rust tests plus generated-artifact, golden-fixture, Android replay, comparison-artifact, and doc tests.
- `npm.cmd run check --prefix frontend` passed 19 frontend tests.
- `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1` exited 0 after printing pre-existing `cargo fmt --check` diffs; Rust, frontend, governance scaffold, release candidate, and astronomy preflight gates passed.
- `cargo check -p minggui-desktop` passed.
- `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .` passed.
- No `cargo fmt` was applied.

## 6. Next Cursor

Next loop: LOOP-112. Any further copy polish must start by reading the generated report samples first. Capability expansion still requires a new milestone and decision gate.
