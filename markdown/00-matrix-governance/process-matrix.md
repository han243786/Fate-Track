# 命轨 Process Matrix

## 1. State Machine

| State | Entry | Exit |
| --- | --- | --- |
| proposed | 需求进入 | 影响范围和档位已声明 |
| designed | 提案足够清晰 | 风险、测试、文档、模块影响已知 |
| implementing | 代码/文档/测试修改中 | 本地门禁通过 |
| validating | 实现完成 | Rust/JS/治理检查通过 |
| auditing | 验证完成 | S0 清零，P1 已登记 |
| closing | 里程碑或交付候选 | closeout 完成 |

## 2. Level Decision

| Trigger | Required Level |
| --- | --- |
| docs-only, no behavior | light |
| Rust API、JS UI、测试、数据、模块所有权变化 | standard |
| 架构、安全、迁移、发布、跨模块、不可逆数据变化 | heavy |

## 3. Proposal Checklist

- Problem.
- Non-goals.
- Impacted users.
- Impacted modules.
- Impacted files.
- Policy clauses.
- Validation commands.
- Rollback or refusal behavior.

## 4. Research Intake Checklist

Use this checklist before implementing any externally supplied research report.

- Source report path.
- Chinese translation path.
- Research intake row in `markdown/reserch/00-research-intake.md`.
- ADR decision or target record.
- Impacted module IDs.
- Capability status: `supported`, `target`, `planned`, `deferred`, or `rejected`.
- Required golden cases or validation evidence.
- Privacy/security classification for new fields.
- Explicit non-goals and unsupported behavior.

Research conclusions must not skip directly from report text to supported UI/API claims.

## 5. Milestone Execution Checklist

Use this checklist before implementing code for any roadmap milestone.

- Read `markdown/20-roadmap/00-roadmap-index.md`.
- Read the active milestone file.
- Check `markdown/20-roadmap/90-decision-gates.md` for blockers.
- Check `markdown/20-roadmap/91-anti-regression-and-governance-lock.md`.
- Check `markdown/20-roadmap/92-risk-register.md` for S0/P1 risks.
- Check `markdown/20-roadmap/93-capability-promotion-ledger.md` before changing capability status.
- List docs that must be synchronized: module tree, engineering tree, README, policy, standards, ADR.
- Plan closeout evidence using `markdown/20-roadmap/94-closeout-evidence-template.md`.

Milestones cannot close with unresolved S0 risks or unsynchronized supported capability claims.

## 6. Recursive Loop Checklist

Use this checklist for every recursive development loop.

- Read `markdown/20-roadmap/95-recursive-development-protocol.md`.
- Read and update `markdown/20-roadmap/96-recursive-cursor.md`.
- Read the previous entry in `markdown/20-roadmap/97-loop-closeout-log.md`.
- Confirm loop mode: `design_only`, `single_loop`, `milestone_loop`, or `goal_run`.
- Confirm allowed scope and forbidden scope.
- Run Preflight Gate before implementation.
- Complete Govern step before validation.
- Run Postflight Gate after validation.
- Write LoopResult to closeout log.
- Advance cursor or mark it `blocked` / `paused`.

In `design_only`, code implementation, API behavior changes, frontend feature advancement, and capability promotion are prohibited.

## 7. Current Validation Set

```powershell
cargo fmt --check
cargo test
cd frontend
npm.cmd run check
cd ..
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .
```

Or run the complete wrapper:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```
