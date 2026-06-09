# M11: Astronomy Engine Implementation

## 1. 目标

实现天文学计算引擎，将 M10 的边界占位件填充为真实数据。使用 4 个源载荷（naif-cspice / iau-sofa / jpl-horizons / gb-t 33661-2017）作为计算参照，生成 1901-2100 完整天文数据。

## 2. 依赖

- M10 关闭：4 源载荷物化、4 生成件边界占位、manifest 已记录占位状态。
- DG-008 已关闭（并行优先，替换需后续 ADR）。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M11-WP1 | 节气穿越时刻计算引擎（使用 SPICE/SOFA 算法参照，输出 TT 时间标度） |
| M11-WP2 | 新月时刻计算引擎（地月日三角几何，TT 时间标度） |
| M11-WP3 | 中国农历历法推导（基于节气+新月+GB/T 置闰规则，生成月表） |
| M11-WP4 | 填充 `out/solar-terms-1901-2100.json` 真实数据，更新 sha256 |
| M11-WP5 | 填充 `out/new-moons-1901-2100.json` 真实数据，更新 sha256 |
| M11-WP6 | 填充 `out/lunar-calendar-1901-2100.json` 真实数据，更新 sha256 |
| M11-WP7 | 执行 Android vs 天文学对照，填充 `out/android-comparison-1901-2100.json`，差异分类 |

## 4. 非目标

- 不替换 Android 日期层作为运行时基线。
- 不扩大验证范围（保持 1901-2100）。
- 不支持真太阳时或 IANA 时区历史。
- 不引入外部网络调用到完整门禁。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `astronomy-engine` | target | target（真实数据生成后仍为 target，需替换 ADR 方可晋级） |
| `calendar-date-query` | supported | supported（不变化） |

## 6. 防回退

- 引擎输出必须可复现（固定源载荷版本 + 确定性算法）。
- 对照报告不得静默标记为"无差异"。
- 黄金样例必须覆盖 M9 定义的 6 个类别。
- 旧 Android 日期层三柱样例仍全量通过。

## 7. 治理同步

- `data/generated/astronomy/manifests/astronomy-engine-v0-draft.json` 更新。
- `data/generated/astronomy/generated-artifact-materialization.json` 更新哈希。
- `data/generated/astronomy/comparison-runner-plan.json` 更新为真实对照。
- `data/generated/astronomy/golden-row-readiness-plan.json` 物化黄金行。
- `data/generated/astronomy/replay-test-readiness-plan.json` 执行重放测试。
- `README.md`、module tree、engineering tree、capability ledger 同步。

## 8. 验收

- `cargo test` 包含引擎计算回归测试。
- 4 个生成件 entry 数组非空，sha256 已更新。
- 对照报告有分类差异行。
- 黄金样例覆盖全部 6 个类别。
- `tools/check-project.ps1` 通过。
