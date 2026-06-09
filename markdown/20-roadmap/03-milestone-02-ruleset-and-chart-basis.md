# M2: Ruleset and Chart Basis

## 1. 目标

建立 `ft-v1-default` 规则档与命盘计算基础对象，但不急于输出完整命盘。M2 是排盘引擎前的“契约层”。

## 2. 依赖

- M1 日期层稳定。
- DG-001 已关闭：V1 默认规则档为 `ft-v1-default`。
- DG-003 已关闭：默认日界为本地民用 `00:00`，子初保持 planned 高级选项。
- DG-004 已关闭：M2 不开放农历输入，农历输入保持 planned/unsupported。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M2-WP1 | 定义 `RulesetId`, `AlgoVersion`, `CalculationMetadata` |
| M2-WP2 | 定义 `BirthProfile`, `ChartRequest`, `ChartBasis` 字段和隐私等级 |
| M2-WP3 | 建立请求校验：日期、时间精度、时区字段、真太阳时开关 |
| M2-WP4 | 目标 API 合约草案：preview/create/detail 的 DTO 和错误 |
| M2-WP5 | 将目标 `/api/v1/...` 与当前 `/api/...` 原型路由关系写清楚 |

## 4. 非目标

- 不实现四柱完整计算。
- 不实现持久化。
- 不实现前端工作台。
- 不开放大运或分析。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `chart-basis-preview` | planned | restricted |
| `chart-create` | planned | planned |
| `settings` | planned | planned |

## 6. 防回退

- `ft-v1-default` 不得在代码或文档中出现多个含义。
- 所有命盘相关 DTO 必须带隐私分级。
- 未实现农历输入时，API/UI 必须显式 unsupported 或 planned。
- 未实现真太阳时时，不能只隐藏字段；必须明确不支持。
- `chart-basis-preview` restricted 不得升级为 `chart-create` supported。

## 7. 治理同步

- ADR 0004 如规则档正式化，需要从 Target 更新为 Accepted 或补新 ADR。
- `General_Policy.md` 若新增敏感字段，必须同步。
- `module-tree.md` 登记 `backend.bazi.engine` 目标 public surface。
- `93-capability-promotion-ledger.md` 更新 `chart-basis-preview` 条件。

## 8. 验收

- DTO/字段文档能追溯到研究报告。
- 错误 envelope 文档存在。
- 未实现能力没有 supported 声明。
- `GET /api/charts/basis/preview` 返回 restricted 合同，且拒绝农历输入、真太阳时和非法精确时间。
- `tools/check-project.ps1` 通过。

## 9. 进入 M3 条件

- `ft-v1-default` 已有唯一规则说明。
- ChartBasis 能表达年/月/日/时所需输入。
- Unknown hour、true solar time、timezone fallback 的未实现行为已定义。
