# M12: Chart Detail Snapshot

## 1. 目标

实现命盘详情快照 (`chart-detail`)，使排盘结果可被持久化引用、重放和审计。快照必须携带完整的算法版本、规则档和输入参数，确保可复现。

## 2. 依赖

- M3 已提供 chart-create 基础四柱。
- M5 已提供 local-volatile 案例存储。
- 需要 M3 chart-create 输出的稳定 DTO。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M12-WP1 | `ChartDetail` 领域模型：完整四柱、输入参数、算法版本、时间戳 |
| M12-WP2 | `GET /api/charts/detail` 路由和 JSON 契约 |
| M12-WP3 | 快照不可变性保证：创建后字段不可变 |
| M12-WP4 | 快照与案例的引用完整性（case 引用 snapshot_id） |
| M12-WP5 | API 和领域测试 |

## 4. 非目标

- 不添加新的计算能力（复用已有 chart-create 输出）。
- 不添加持久化数据库。
- 不添加快照编辑/更新。
- 不添加快照对比或 diff 功能。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `chart-detail` | planned | supported |
| `chart-create` | supported | supported（不变化） |
| `case-management` | restricted | restricted（不变化） |

## 6. 防回退

- 快照不得在算法升级后被静默重算。
- 快照必须保留原始 `algo_version` 和 `ruleset_id`。
- 不得通过快照 API 暴露案例私有字段。

## 7. 治理同步

- `backend/src/api/mod.rs` 将 `/api/charts/detail` 从 unsupported 改为路由。
- `backend/src/api/capabilities.rs` 更新 chart-detail 状态。
- `93-capability-promotion-ledger.md` 更新 chart-detail 晋级证据。
- module tree、engineering tree、README 同步。

## 8. 验收

- `GET /api/charts/detail` 返回完整快照 DTO。
- 快照携带 `algo_version`、`ruleset_id`、时间戳。
- 快照字段创建后不可变。
- Rust 测试覆盖 chart-detail 正常路径和错误路径。
- `tools/check-project.ps1` 通过。
