# M4: Analysis Engine

## 1. 目标

在已完成命盘的基础上，实现结构化、可测试、非确定性的分析层：藏干、十神、五行、关系摘要和安全文案。

## 2. 依赖

- M3 chart-create 已达到 supported；chart-detail 仍 planned。
- ADR 0005 已作为安全解释政策。
- DG-010 已关闭：M4 只允许结构化固定模板分析。
- DG-005 保持 open；本阶段不包含大运。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M4-WP1 | 藏干表和权重策略 |
| M4-WP2 | 十神关系计算 |
| M4-WP3 | 五行统计：可见干、藏干、基础权重 |
| M4-WP4 | 合冲刑害等 relation flags |
| M4-WP5 | 安全分析卡 DTO：事实、指标、敏感性、免责声明 |
| M4-WP6 | 禁止短语与高风险断言检查 |
| M4-WP7 | 可选：大运规则决策后建立 LuckEngine skeleton |

## 4. 非目标

- 不做医疗、法律、金融、关系确定性断言。
- 不做 AI 自由长篇断语。
- 不做合婚评分。
- 不做流月/流日/流时。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `analysis-snapshot` | planned | supported |
| `luck-cycles` | planned | planned |
| `glossary` | planned | planned |

## 6. 防回退

- 分析输出必须基于结构化指标，不得只返回自由文本。
- 每个分析响应必须包含免责声明或 disclaimer id。
- 禁止短语检查失败不得关闭里程碑。
- 未知时辰必须影响 sensitivity，不得隐藏不确定性。

## 7. 治理同步

- ADR 0005 若有文案策略变化，需要更新。
- `General_Policy.md` 如新增禁令，需要同步。
- `93-capability-promotion-ledger.md` 更新 analysis 条件。
- `module-tree.md` 登记 `backend.analysis.engine`。

## 8. 验收

- 单元测试覆盖十神、五行、藏干。
- 分析安全测试覆盖禁止输出类别。
- API contract 显示 metrics + cards + disclaimer。
- 未知时辰必须进入 sensitivity flags。
- `tools/check-project.ps1` 通过。

## 9. 进入 M5/M6/M7 条件

- AnalysisSnapshot 可持久化为不可变快照。
- 分享脱敏规则能区分敏感字段和 share-safe 字段。
- 前端可用固定 fixtures 渲染分析卡。
