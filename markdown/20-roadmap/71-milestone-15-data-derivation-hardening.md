# M15: Data Derivation and Hardening

## 1. 目标

实现数据衍生能力，并对全系统进行加固收口。M15 是 V1 功能集的最后一个里程碑，目标是所有 declared capabilities 达到 supported 或 restricted，无 planned 遗留。

## 2. 依赖

- M1-M14 全部关闭。
- 所有 planned capability 已有实现。
- 无未关闭的阻塞性决策门（DG-005 除外，须在 M13 关闭）。

## 3. 范围

### Data Derivation (数据衍生)

| Work Package | 内容 |
| --- | --- |
| M15-WP1 | `GET /api/data/derive` 路由：从已有数据衍生统计/聚合视图 |
| M15-WP2 | 衍生类型：五行分布统计、十神分布统计、日主频率、时辰分布 |
| M15-WP3 | 衍生数据不暴露个体案例，仅输出聚合结果 |
| M15-WP4 | API 测试 + 隐私校验 |

### Hardening (加固收口)

| Work Package | 内容 |
| --- | --- |
| M15-WP5 | 全量 API 错误 envelope 一致性和边界审计 |
| M15-WP6 | 所有 public surface 文档同步（module tree、engineering tree、README、capability ledger） |
| M15-WP7 | 前端能力面板与 `/api/capabilities` 一致性审计 |
| M15-WP8 | `data-derivation` 能力晋级为 restricted |
| M15-WP9 | 全量门禁：`cargo test` + `npm run check` + `check-project.ps1` + `check-astronomy-preflight.ps1` |
| M15-WP10 | V1 closeout：汇总全部能力状态、已知限制、后续路线 |

## 4. 非目标

- 不添加新的计算算法。
- 不引入第三方数据分析库。
- 不提供用户自定义查询语言。
- 不打开新的架构迁移。

## 5. 能力状态（M15 目标终态）

| Capability | Target Status |
| --- | --- |
| `health` | supported |
| `lunar-data-meta` | supported |
| `calendar-date-query` | supported |
| `calendar-date-query-v1-meta` | supported |
| `chart-basis-preview` | restricted |
| `chart-create` | supported |
| `chart-detail` | supported |
| `analysis-snapshot` | supported |
| `luck-cycles` | supported |
| `case-management` | restricted |
| `share-preview` | restricted |
| `settings` | restricted |
| `glossary` | supported |
| `case-export` | restricted |
| `data-derivation` | restricted |
| `astronomy-engine` | target（仍需替换 ADR） |

## 6. 防回退

- 衍生数据不得反推个体出生信息。
- 聚合最小阈值不得低于 5 条记录。
- 所有 API 错误必须使用统一的 JSON envelope。
- 能力面板不得显示不存在的功能。

## 7. 治理同步

- 全量治理文件最终同步。
- V1 release candidate 更新。
- 能力台账终态锁定。

## 8. 验收

- 全部 16 项能力状态与代码一致。
- `data-derivation` 返回聚合数据且不泄露个体。
- 全门禁绿色。
- V1 closeout 完成。
