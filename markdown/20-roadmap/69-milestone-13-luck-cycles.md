# M13: Luck Cycles

## 1. 目标

实现大运排盘。这是八字系统中除四柱外最重要的时间维度，必须通过 DG-005 决策门确定顺逆规则、起运年龄算法和边界处理策略后方可编码。

## 2. 依赖

- M3 chart-create 提供四柱和日主。
- M12 chart-detail 提供可引用的快照。
- **DG-005 必须关闭**：确定大运顺逆与起运年龄默认规则。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M13-WP1 | 关闭 DG-005：通过 ADR 确定顺逆规则、起运算法、边界策略 |
| M13-WP2 | `LuckCycle`、`LuckPillar` 领域模型 |
| M13-WP3 | 大运顺逆判定（年干阴阳 + 性别） |
| M13-WP4 | 起运年龄计算（出生日到下一个/上一个节气的日数 ÷ 3） |
| M13-WP5 | `GET /api/luck/cycles` 路由和 JSON 契约 |
| M13-WP6 | 流年/流月占位（planned 但不实现详细分析） |
| M13-WP7 | API 和领域测试 + 黄金样例 |

## 4. 非目标

- 不实现流年/流月的详细十神分析（保持 planned）。
- 不将大运分析混入 `analysis-snapshot`。
- 不提供大运择日建议。
- 不声称大运推断为确定性预测。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `luck-cycles` | planned | supported |
| `chart-detail` | supported | supported（不变化） |
| `analysis-snapshot` | supported | supported（不变化） |

## 6. 防回退

- 大运结果必须携带 `luck_ruleset_id` 和算法版本。
- 起运年龄不得对同一输入产生不同结果。
- 大运输出不得包含确定性人生断言。
- 黄金样例覆盖顺行/逆行、阴阳年干、男女两性全部组合。

## 7. 治理同步

- `docs/decisions/00XX-dg-005-luck-cycle-rules.md` — DG-005 关闭 ADR。
- `90-decision-gates.md` 关闭 DG-005。
- `backend/src/api/mod.rs` 将 `/api/luck/cycles` 从 unsupported 改为路由。
- `backend/src/api/capabilities.rs` 更新 luck-cycles 状态。
- `93-capability-promotion-ledger.md` 更新 luck-cycles 晋级证据。
- module tree、engineering tree、README 同步。

## 8. 验收

- `GET /api/luck/cycles` 返回大运序列（含起运年龄、每运干支）。
- 顺逆判定覆盖性别×年干全部组合。
- 起运年龄精度到天。
- 黄金样例覆盖边界（节气当天出生、跨年等）。
- `tools/check-project.ps1` 通过。
