# M6: Share Privacy

## 1. 目标

实现默认脱敏的分享快照、分享 token、过期/撤销和公开只读视图，确保公开面不会泄露私有案例状态。

## 2. 依赖

- M5 提供不可变 chart snapshot。
- ADR 0005 生效。
- DG-009 必须关闭。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M6-WP1 | SharePreset：脱敏字段、过期时间、撤销时间、访问模式 |
| M6-WP2 | share token 生成、hash 存储、读取校验 |
| M6-WP3 | 公开分享 DTO：只包含 share-safe 字段 |
| M6-WP4 | 分享预览和实际公开视图一致性测试 |
| M6-WP5 | 过期/撤销响应不暴露底层案例是否存在 |
| M6-WP6 | `noindex`、不可编辑、不可反向枚举策略 |

## 4. 非目标

- 不做分享分析统计。
- 不做公开社区。
- 不做评论。
- 不做实时私有案例镜像。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `share-preview` | planned | supported 或 restricted |
| `case-management` | supported/restricted | unchanged |
| `frontend-share-preview` | planned | restricted until M7 |

## 6. 防回退

- 分享默认隐藏精确出生时间、地点、私有备注。
- token 不得明文存储。
- 公开页面不能暴露私有内部 ID。
- 撤销或过期不应返回“此案例存在但不可见”这类泄露信息。

## 7. 治理同步

- ADR 0005 如有策略变化必须更新。
- `93-capability-promotion-ledger.md` 更新 share。
- `module-tree.md` 登记 `backend.share`。
- `General_Policy.md` 如新增分享禁令必须更新。

## 8. 验收

- share redaction snapshot tests。
- token hash tests。
- revoked/expired tests。
- public DTO 不含敏感字段。
- `tools/check-project.ps1` 通过。

## 9. 进入 M7 条件

- 前端可以读取公开分享 DTO。
- 分享预览需要的字段和状态明确。
- UI 不需要访问私有案例状态就能渲染分享页。
## 10. M6 Closeout Update

ADR 0014 closes DG-009 for M6 by selecting local volatile hash-only share tokens, expiration, revocation, noindex, and redacted public DTOs.

Capability status after LOOP-016:

| Capability | Before | After | Boundary |
| --- | --- | --- | --- |
| `share-preview` | planned | restricted | local volatile share records; raw token only on create; hash-only storage; redacted public DTO; no durable public links, accounts, database persistence, cloud sync, cross-device sync, public directories, comments, analytics, luck cycles, or generated analysis |
| `case-management` | restricted | restricted | unchanged M5 local volatile storage |
| `settings` | restricted | restricted | unchanged M5 local volatile preferences |

Acceptance evidence:

- `backend/src/domain/share.rs` implements share records, token hashing, expiration, revocation, and redacted snapshots.
- `backend/src/api/share.rs` exposes create/public/revoke actions under `GET /api/share/preview`.
- Public DTOs omit private notes, raw titles, tags, private case ids, exact birth-time/location fields, and snapshot ids.
- Missing, expired, invalid, and revoked tokens use the same unavailable response.
- `tools/check-project.ps1` passed after implementation with Rust 51 tests and frontend 6 tests.

