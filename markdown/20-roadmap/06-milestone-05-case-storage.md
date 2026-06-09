# M5: Case Storage

## 1. 目标

实现案例、偏好、命盘快照和存储边界。M5 的核心不是“把东西存起来”，而是保证旧命盘可复现、隐私字段可控、删除/归档语义清楚。

## 2. 依赖

- M3 至少提供可复现 chart detail。
- M4 若分析快照进入存储，必须有稳定 DTO。
- DG-006 必须关闭：匿名、本地、账号、云同步范围明确。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M5-WP1 | CaseRecord、BirthProfile、BaziChartSnapshot、UserPreference 存储模型 |
| M5-WP2 | 不可变 chart snapshot 与 `algo_version` 保留 |
| M5-WP3 | 案例创建、读取、更新标题/标签/备注、归档/删除 |
| M5-WP4 | 偏好读取/更新：默认规则、隐私默认值、语言/主题 |
| M5-WP5 | 日志过滤和敏感字段策略 |
| M5-WP6 | 存储迁移或本地存储策略文档 |

## 4. 非目标

- 不默认承诺云同步。
- 不实现团队协作。
- 不暴露后台 CRM。
- 不把公开分享混入私有案例读取。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `case-management` | planned | supported 或 restricted |
| `settings` | planned | supported 或 restricted |
| `share-preview` | planned | planned |

## 6. 防回退

- 旧命盘不得因算法升级被静默重算。
- 出生资料和私有备注不得进入普通日志。
- 删除、归档、隐藏必须有不同语义。
- 如果没有账号，不得写“跨设备同步”。

## 7. 治理同步

- `General_Policy.md` 如存储策略新增，必须更新。
- `module-tree.md` 登记 `backend.storage`。
- `93-capability-promotion-ledger.md` 更新 cases/settings。
- `90-decision-gates.md` 关闭 DG-006。

## 8. 验收

- Repository tests 或等价持久化测试。
- 私有字段不在日志输出。
- 旧 snapshot 可按原 version 读取。
- 删除/归档行为测试。
- `tools/check-project.ps1` 通过。

## 9. 进入 M6 条件

- 分享可基于不可变快照生成。
- 敏感字段分类已稳定。
- token 与公开视图策略已在 DG-009 关闭。
## 10. M5 Closeout Update

ADR 0013 closes DG-006 for M5 by selecting local in-process volatile storage only.

Capability status after LOOP-015:

| Capability | Before | After | Boundary |
| --- | --- | --- | --- |
| `case-management` | planned | restricted | local volatile cases; immutable chart/analysis snapshot refs; no account, database persistence, cloud sync, cross-device sync, public sharing, share tokens, luck cycles, or generated analysis |
| `settings` | planned | restricted | local volatile preferences only; no account-level or cross-device preference persistence |
| `share-preview` | planned | planned | M6 only; DG-009 remains target-proposed |

Acceptance evidence:

- `backend/src/api/cases.rs` exposes create/detail/list/update_metadata/archive/delete.
- `backend/src/domain/cases.rs` preserves chart and analysis snapshot version refs during metadata updates.
- Case list responses omit `private_note`, and deleted cases are omitted.
- `backend/src/api/settings.rs` exposes validated local preference get/update.
- `tools/check-project.ps1` passed after implementation with Rust 46 tests and frontend 6 tests.
