# M1: Date Layer Hardening

## 1. 目标

在不推翻 Android 日期层的前提下，强化当前 `calendar-date-query` 能力，使它成为后续排盘引擎可信依赖。

## 2. 依赖

- M0 closeout。
- ADR 0002 保持 accepted。
- ADR 0004 确认 Android 日期层是 current baseline，星历方案是 target。
- ADR 0008 关闭 DG-002：V1 官方验证范围为 1901-2100。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M1-WP1 | 扩展日期查询响应元数据：数据源、算法版本、规则说明、支持范围 |
| M1-WP2 | 增补 Android edge-case golden tests：春节前后、立春附近、闰月、2033 预留 |
| M1-WP3 | 明确非法日期、范围外日期、数据缺失时的错误 envelope |
| M1-WP4 | 更新 `/api/capabilities` 中日期层能力说明 |
| M1-WP5 | 文档说明当前 supported 与星历 target 的差异 |

## 4. 非目标

- 不实现完整四柱排盘。
- 不实现时柱。
- 不实现 IANA 时区历史。
- 不实现真太阳时。
- 不替换为星历引擎。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `calendar-date-query` | supported | supported, stronger metadata |
| `astronomy-engine` | target | target |
| `chart-create` | planned | planned |

## 6. 防回退

- 保留 `project_data_matches_android_edge_cases_for_three_pillars`。
- 若新增样例与 Android 输出冲突，先登记差异，不得直接改期望。
- 任何范围扩展必须登记数据来源和验证等级。

## 7. 治理同步

- `93-capability-promotion-ledger.md`
- `module-tree.md`
- `overview-full-feature-tree.md`
- `README.md`
- `data/README.md`
- 如规则变化，更新 ADR 0002 或新增 ADR。

## 8. 验收

- `cargo test` 包含日期层黄金样例。
- `GET /api/calendar/query?date=2025-01-01` 仍返回 `甲辰/乙丑/庚午`。
- 非法日期返回显式错误。
- 范围外日期返回显式 `out_of_range`，且不得宣称 1901-2100 外高置信支持。
- `tools/check-project.ps1` 通过。

## 9. 进入 M2 条件

- 日期层 supported 边界清楚。
- DG-001、DG-003 至少达到 target-proposed。
- ChartRequest/BirthProfile 需要的日期字段已在研究台账中确认。
