# M9: Astronomy Engine Upgrade

## 1. 目标

在 V1 稳定后，建立星历/天文事件驱动的历法引擎和黄金数据生成链路，为更强的节气、朔、闰月、真太阳时、时区历史支持做升级。

## 2. 依赖

- M8 release candidate 已关闭。
- DG-008 已由 ADR 0015 关闭为 parallel-first preflight；替换 Android baseline 仍需后续 ADR。
- 选择星历/算法来源、验证范围和数据发布策略。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M9-WP1 | 明确天文引擎来源：JPL/SPA/SOFA/其他 |
| M9-WP2 | 生成 1901-2100 或更宽范围黄金表 |
| M9-WP3 | 记录生成命令、hash、版本、输入数据源 |
| M9-WP4 | 与 Android 日期层并行对照 |
| M9-WP5 | 差异分类：Android bug、星历差异、规则差异、时区差异 |
| M9-WP6 | 迁移策略：并行引擎、版本切换、旧命盘复现 |
| M9-WP7 | 真太阳时和精确节气即时刻支持 |
| M9-WP8 | 2033 异常和午夜边界专门回归 |

## 4. 非目标

- 不静默替换 V1 结果。
- 不删除 Android baseline。
- 不把星历未验证范围标高置信。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `astronomy-engine` | target | supported/restricted after evidence |
| `calendar-date-query` | supported | versioned engine-backed or dual-engine |
| `chart-create` | supported | preserves old algo versions |

## 6. 防回退

- 任何输出变化必须可解释为版本变化。
- 旧命盘必须能按旧 `algo_version` 重放或读取快照。
- Android 差异样例不得删除；要作为对照报告保留。
- 生成数据必须有 manifest、hash、生成命令和验证证据。

## 7. 治理同步

- 新增 ADR：星历引擎选择和迁移策略。
- `data/README.md` 登记衍生产物。
- `module-tree.md` 登记 `backend.calendar.astronomy`。
- `standard-matrix.md` 登记新增数据门禁。
- `93-capability-promotion-ledger.md` 更新 astronomy-engine。

## 8. 验收

- 星历黄金表生成可复现。
- Android 对照差异报告完整。
- 2033、立春、清明、甲子日、真太阳时边界测试通过。
- 旧版本命盘可复现。
- `tools/check-project.ps1` 通过。

## 9. 后续方向

- 扩展验证范围。
- 支持多规则档比较。
- 引入更丰富的大运/流年规则。
- 建立离线包或缓存策略。

## 10. Preflight Update

M9 preflight is recorded in `27-milestone-09-preflight.md`.

ADR 0015 selects a parallel-first astronomy upgrade strategy:

- Android date layer remains accepted-current for V1.
- `astronomy-engine` remains target until generated artifacts, hashes, comparison report, and replay policy exist.
- Replacement requires a later ADR and must not silently change `calendar-date-query` or `chart-create` results.

The preflight artifacts are under `data/generated/astronomy/`, and the reproducible check is `tools/check-astronomy-preflight.ps1`.

## 11. Pre-Closeout Audit

LOOP-029 adds `36-milestone-09-pre-closeout-audit.md` and `data/generated/astronomy/precloseout-audit.json`.

The audit result is `full_m9_closeout_blocked_preflight_ready`: M9 can be reviewed as a preflight-only milestone, but full astronomy-engine closeout remains blocked until generated artifacts, hashes, completed Android comparison, golden rows, replay tests, and runtime integration exist.

## 12. Generated-Data Implementation Planning

LOOP-030 adds ADR 0017, `37-milestone-09-generated-data-implementation-plan.md`, and `data/generated/astronomy/implementation-plan.json`.

The selected path is `continue_m9_generated_data_planning`. The next work package is generator contract planning, still with no generated rows, no accepted astronomy artifacts, no Android baseline replacement, and no `astronomy-engine` promotion.

## 13. Generator Contract

LOOP-031 adds `38-milestone-09-generator-contract.md` and `data/generated/astronomy/generator-contract.json`.

The generator contract defines required inputs, planned outputs, canonical JSON encoding, `sha256` hash policy, manifest update rules, and forbidden contract-stage actions before any generated row exists.

## 14. Source Adapter Contract

LOOP-032 adds `39-milestone-09-source-adapter-contract.md` and `data/generated/astronomy/source-adapter-contract.json`.

The source adapter contract maps GB/T, Horizons, SOFA, and SPICE into future reproducible input boundaries while forbidding runtime dependency, output claims, full-gate external API calls, Android replacement, and `astronomy-engine` promotion.

## 15. Artifact Writer Dry-Run

LOOP-033 adds `40-milestone-09-artifact-writer-dry-run.md`, `data/generated/astronomy/artifact-writer-plan.json`, and `tools/artifact-writer-dry-run.ps1`.

The artifact writer dry-run previews output paths and `sha256` hash policy without creating directories, writing artifacts, computing hashes, updating manifest hash state, or claiming accepted generated evidence.

## 16. Comparison Runner Dry-Run

LOOP-034 adds `41-milestone-09-comparison-runner-dry-run.md` and `data/generated/astronomy/comparison-runner-plan.json`.

The comparison runner dry-run binds Android baseline metadata to the future Android-vs-astronomy comparison artifact while keeping rows compared, difference rows, writes, and accepted evidence at zero.

## 17. Golden-Row Materialization Readiness

LOOP-035 adds `42-milestone-09-golden-row-readiness.md` and `data/generated/astronomy/golden-row-readiness-plan.json`.

The readiness plan keeps every required golden category `not_generated` and `blocked_until_generated_rows` until source references, expected Android values, expected astronomy values, and difference classifications can be supplied.

## 18. Replay-Test Materialization Readiness

LOOP-036 adds `43-milestone-09-replay-test-readiness.md` and `data/generated/astronomy/replay-test-readiness-plan.json`.

The readiness plan keeps replay tests unexecuted while binding old-snapshot replay requirements to `android-date-layer-v1`, `ft-date-layer-android-v1`, generated rows, classified comparison output, and a later replacement ADR.

## 19. Preflight-Only Closeout

LOOP-037 adds `44-milestone-09-preflight-closeout.md` and `data/generated/astronomy/preflight-closeout-decision.json`.

M9 is closed only as a preflight milestone. Full astronomy-engine implementation remains open and moves to M10 Generated Astronomy Implementation. `astronomy-engine` remains `target`; no generated artifacts are accepted, Android baseline replacement remains disallowed, and runtime behavior for `calendar-date-query` and `chart-create` is unchanged.
