# 开发里程碑目录

从这里开始读：

1. `00-roadmap-index.md`
2. `90-decision-gates.md`
3. `91-anti-regression-and-governance-lock.md`
4. `95-recursive-development-protocol.md`
5. `96-recursive-cursor.md`
6. `97-loop-closeout-log.md`
7. `98-recursive-loop-runbook.md`
8. `100-recursive-scale-and-goal-readiness.md`
9. 如需示例，读 `99-milestone-01-preflight-dry-run.md`
10. 当前要执行的里程碑文件
11. 如关闭 M1，读 `11-milestone-01-closeout-readiness.md`
12. 如复核 M1 closeout，读 `12-milestone-01-closeout.md`
13. 如复核 M2，读 `13-milestone-02-preflight.md` 和 `14-milestone-02-closeout.md`
14. 如复核 M3，读 `15-milestone-03-preflight.md` 和 `16-milestone-03-closeout.md`
15. 如复核 M4，读 `17-milestone-04-preflight.md` 和 `18-milestone-04-closeout.md`
16. 如复核 M8，读 `25-milestone-08-preflight.md`、`26-milestone-08-closeout.md` 和 `docs/release/v1-release-candidate.md`
17. 如进入 M9，读 `27-milestone-09-preflight.md`、`docs/decisions/0015-m9-astronomy-parallel-strategy.md` 和 `data/generated/astronomy/README.md`
18. `93-capability-promotion-ledger.md`
19. `94-closeout-evidence-template.md`

## 执行原则

- 先完成里程碑治理，再落实代码。
- 任何 supported 晋级必须有证据。
- 任何回退必须记录原因、范围和替代保护。
- 任何新 public surface 必须同步模块树和工程树。
- 递归游标处于 `design_only` 时，不推进业务代码。
- 每轮递归采用 largest stable invariant：一个不变量内尽量多落代码，出现第二个不变量立即拆轮。
## M5 Reference

- M5 preflight: `19-milestone-05-preflight.md`
- M5 closeout: `20-milestone-05-closeout.md`
- M5 restricted capabilities: `case-management`, `settings`
- M6 preflight: `21-milestone-06-preflight.md`
- M6 closeout: `22-milestone-06-closeout.md`
- M6 restricted capability: `share-preview`
- M7 preflight: `23-milestone-07-preflight.md`
- M7 closeout: `24-milestone-07-closeout.md`
- M7 restricted capabilities: `frontend-chart-workspace`, `frontend-share-preview`
- M8 preflight: `25-milestone-08-preflight.md`
- M8 closeout: `26-milestone-08-closeout.md`
- M8 supported governance capability: `release-candidate`
- M9 preflight: `27-milestone-09-preflight.md`
- M9 ADRs: `docs/decisions/0015-m9-astronomy-parallel-strategy.md`, `docs/decisions/0016-m9-astronomy-source-stack.md`
- M9 source availability: `28-milestone-09-source-availability.md`
- M9 manifest draft: `29-milestone-09-manifest-draft.md`
- M9 generation plan: `30-milestone-09-generation-plan.md`
- M9 generator dry-run: `31-milestone-09-generator-dry-run.md`
- M9 comparison/golden/replay plan: `32-milestone-09-comparison-golden-replay-plan.md`
- M9 comparison dry-run: `33-milestone-09-comparison-dry-run.md`
- M9 golden-case dry-run: `34-milestone-09-golden-dry-run.md`
- M9 replay-policy dry-run: `35-milestone-09-replay-policy-dry-run.md`
- M9 pre-closeout audit: `36-milestone-09-pre-closeout-audit.md`
- M9 generated-data implementation plan: `37-milestone-09-generated-data-implementation-plan.md`
- M9 generator contract: `38-milestone-09-generator-contract.md`
- M9 source adapter contract: `39-milestone-09-source-adapter-contract.md`
- M9 artifact writer dry-run: `40-milestone-09-artifact-writer-dry-run.md`
- M9 comparison runner dry-run: `41-milestone-09-comparison-runner-dry-run.md`
- M9 golden-row readiness: `42-milestone-09-golden-row-readiness.md`
- M9 replay-test readiness: `43-milestone-09-replay-test-readiness.md`
- M9 preflight closeout: `44-milestone-09-preflight-closeout.md`
- M10 generated astronomy implementation: `45-milestone-10-generated-astronomy-implementation.md`
- M10 generator implementation entry: `46-milestone-10-generator-entry.md`
- M10 source snapshot manifest boundary: `47-milestone-10-source-snapshot-boundary.md`
- M10 source snapshot manifest metadata: `48-milestone-10-source-snapshot-manifest.md`
- M10 source payload materialization policy: `49-milestone-10-source-payload-policy.md`
- M10 source payload schemas: `50-milestone-10-source-payload-schemas.md`
- M10 source capture procedure: `51-milestone-10-source-capture-procedure.md`
- M10 first source payload decision: `52-milestone-10-first-source-payload-decision.md`
- M10 selected source payload preflight: `53-milestone-10-selected-source-payload-preflight.md`
- M10 selected source payload materialization: `54-milestone-10-selected-source-payload-materialization.md`
- M10 remaining source payload strategy: `55-milestone-10-remaining-source-payload-strategy.md`
- M10 selected IAU SOFA payload preflight: `56-milestone-10-selected-iau-sofa-payload-preflight.md`
- M10 selected IAU SOFA payload materialization: `57-milestone-10-selected-iau-sofa-payload-materialization.md`
- M10 post-IAU remaining source payload strategy: `58-milestone-10-post-iau-remaining-source-payload-strategy.md`
- M10 selected JPL Horizons payload preflight: `59-milestone-10-selected-jpl-horizons-payload-preflight.md`
- M10 selected JPL Horizons payload materialization: `60-milestone-10-selected-jpl-horizons-payload-materialization.md`
- M10 selected GB/T payload preflight: `61-milestone-10-selected-gb-t-payload-preflight.md`
- M10 selected GB/T payload materialization: `62-milestone-10-selected-gb-t-payload-materialization.md`
- M9 target capability: `astronomy-engine`
