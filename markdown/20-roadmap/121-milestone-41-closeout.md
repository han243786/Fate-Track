# M41 Closeout - Relationship Report Narrative Polish

## 1. Scope Closed

M41 关闭为 `relationship-report` 单一切面的叙事打磨，不改变 public route、DTO 顶层结构、capability 状态或 raw `GET /api/luck/cycles` 语义。

已完成：

- 后端情感报告正文固定为六块：`总断`、`伴侣议题`、`夫妻宫`、`表达、边界与安全感`、`年度情感引动`、`结论`。
- 情感报告开头保留一次阅读提醒；正文不再反复插入机器式边界说明。
- `topic-timeline-overlay` 仍保留在 `signals` / `trace` 证据层；情感报告不再单独追加 `本专题的大运流年` 正文块，而是合入 `年度情感引动`。
- 结论段不再暴露 warning/sensitivity 后端变量，改为自然语言条件说明。
- 前端完整情感报告页只把六个 narrative blocks 作为正文章节；时间解释导航仍可读取 trace 证据。
- 测试更新为 M41 结构门禁：六块顺序、overlay 证据保留、无独立 overlay 正文块、无 public score、无确定性婚恋断言。

## 2. Capability Status

| Capability | Before | After | Notes |
| --- | --- | --- | --- |
| `relationship-report` | restricted | restricted | 只改正文组织和可读性 |
| `topic-timeline-reading` | restricted | restricted | relationship lens 的 overlay 被合入年度情感引动；signals/trace 证据保留 |
| `luck-reading` | restricted | restricted | 无变更 |
| `annual-trigger-reading` | restricted | restricted | 无变更 |

Post-preview runtime remains: 10 supported, 14 restricted, 0 planned.

## 3. Files Changed

### Code

- `backend/src/domain/topic_report.rs`
- `backend/src/app.rs`
- `frontend/src/topic-report-page.js`
- `frontend/tests/workspace-markup.test.mjs`

### Governance

- `README.md`
- `docs/release/v1-closeout.md`
- `markdown/00-matrix-governance/module-tree.md`
- `markdown/10-overview/overview-full-feature-tree.md`
- `markdown/命轨全量树.md`
- `markdown/20-roadmap/00-roadmap-index.md`
- `markdown/20-roadmap/89-post-preview-documentation-freeze.md`
- `markdown/20-roadmap/92-risk-register.md`
- `markdown/20-roadmap/93-capability-promotion-ledger.md`
- `markdown/20-roadmap/96-recursive-cursor.md`
- `markdown/20-roadmap/97-loop-closeout-log.md`
- `markdown/20-roadmap/README.md`
- `markdown/20-roadmap/120-milestone-41-relationship-report-narrative-polish.md`
- `markdown/20-roadmap/121-milestone-41-closeout.md`

## 4. Validation

| Command | Result |
| --- | --- |
| `cargo test report -- --nocapture` | Passed: 22 report/topic tests, 95 filtered |
| `npm.cmd run check --prefix frontend` | Passed: 19 frontend tests |
| `cargo test -- --nocapture` | Passed: 117 Rust tests plus generated-artifact, golden-fixture, Android replay, comparison-artifact, and doc tests |
| `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1` | Exited 0; printed pre-existing `cargo fmt --check` diffs; Rust/frontend/governance/release/astronomy gates passed |
| `cargo check -p minggui-desktop` | Passed |
| `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .` | Passed |

No `cargo fmt` was applied.

## 5. Locks Preserved

- No supported promotion.
- No new public API.
- No top-level DTO shape expansion.
- No raw `GET /api/luck/cycles` mutation.
- No public `score_internal` or 0-100 score.
- No full flow-month/day, daily fortune, date selection, or event prediction claim.
- No deterministic marriage, separation, partner identity, or romance event claim.

## 6. Next Cursor

Advance to `LOOP-110`, `single_loop`, waiting for the next user-selected slice.

Recommended next work:

- If continuing relationship polish, use this M41 six-block report as the style baseline and inspect real output samples before large dictionary changes.
- If moving to wealth/family/career polish, open a new milestone or explicit quality loop and preserve each topic's safety boundary.
- If expanding timeline capability, open a new decision gate and ADR before code.
