# ADR 0021: M23 Astronomy Engine Promotion

## Status

Accepted.

## Decision

`astronomy-engine` 从 target 晋级为 supported。天文计算引擎作为独立于 Android 日期层的验证计算能力被正式承认。Android 日期层保持为运行时默认；天文引擎作为并行对照引擎和离线生成数据源存在。运行时替换为未来独立 ADR 决策。

## Context

M11 实现了完整的天文计算引擎（ADR 0019）：太阳（VSOP87/Meeus）、月球（Meeus Ch.47 ~60项）、节气（Newton-Raphson 二分法）、农历推导（GB/T 置闰规则）、对照引擎。M10 产出全部生成数据（4800 节气 + 2474 新月 + 2474 农历月）。M19 执行 Android vs 天文引擎对照：1598 样本，0 差异。M20 黄金样例 + 重放测试通过。

引擎当前状态：代码完整、测试充分、对照清零、生成数据产出。唯一缺失的是治理层面的能力声明——它仍被标为 target。

## Options Considered

### Option A: 立即替换 Android 日期层

将 `/api/calendar/query` 的运行时计算从 Android 端口切换为天文引擎。

**Rejected.** 天文引擎尚不支持真太阳时、IANA 时区历史、1901-2100 外范围。替换会引入未知回归风险，违反 LOCK-001。

### Option B: 保持 target，等待全部条件满足

继续将 `astronomy-engine` 标为 target，直到运行时集成、真太阳时、时区历史全部实现。

**Rejected.** 引擎代码、测试、生成数据、对照报告四项证据已齐备。继续标 target 会掩盖实际工程进度，且 target 状态阻碍了 M24 排盘报告中对天文数据的合理引用。

### Option C: 晋级为 supported，保持并行（选定）

将 `astronomy-engine` 标为 supported，承认引擎的计算能力已验证。Android 日期层保持为运行时默认。天文引擎作为独立 supported 能力存在，用于：
- 离线生成天文数据（节气表、新月表、农历月表）
- Android 对照验证
- 未来排盘报告中的天文数据引用
- 为运行时替换提供证据基础

运行时替换留待未来 ADR（需满足：真太阳时、IANA 时区历史、1901-2100 外范围、全门禁对照通过）。

## Chosen Option

**Option C** — 晋级为 supported，保持并行，运行时替换另立 ADR。

## Rationale

1. 代码证据完整：`backend/src/astronomy/` 含 7 个模块文件，17 项测试全绿。
2. 数据证据完整：4800 节气 + 2474 新月 + 2474 农历月，全部 sha256 哈希化。
3. 对照证据完整：Android 1598 样本日柱对照，0 差异。
4. 保持 Android 日期层作为运行时默认，符合 LOCK-001，不会引入回退风险。
5. 并行策略与 DG-008 关闭时的 ADR 0015 一致：「先并行，后替换」。

## Impacted Modules

| Module | Impact |
| --- | --- |
| `backend.api` | `capabilities.rs` 新增 `astronomy-engine` 为 supported |
| `backend.astronomy` | 能力状态从 target→supported，不涉及代码变更 |
| `backend.calendar` | 无变更，保持 Android 日期层为运行时默认 |
| `governance.capability-ledger` | §1 新增条目 |
| `governance.module-tree` | 更新 astronomy 节点状态 |
| `governance.roadmap` | M23 closeout |
| `docs.release` | v1-closeout 更新能力矩阵 |

## Policy Clauses

- `astronomy-engine` 为 supported 计算能力，不替代 Android 日期层为运行时默认。
- 运行时替换必须通过独立 ADR，且满足真太阳时、IANA 时区历史、范围扩展三项前提。
- 天文引擎输出不得在 UI 中宣称为「比 Android 日期层更准确」。
- Android 日期层黄金样例继续受 LOCK-001 保护。

## Required Tests

- `cargo test astronomy` 17 项保持全绿。
- `cargo test` 全部通过。
- `/api/capabilities` 测试验证 `astronomy-engine` 为 supported。
- `tools/check-astronomy-preflight.ps1` 通过。

## Rollback Rule

如果 Android vs 天文引擎对照出现非零差异，`astronomy-engine` 立即降级为 restricted，并登记差异样例。在差异原因查明并修复前不得恢复 supported。

## Docs to Update

- `backend/src/api/capabilities.rs` — 新增 catalog entry
- `markdown/20-roadmap/93-capability-promotion-ledger.md` — §1 新增条目
- `markdown/00-matrix-governance/module-tree.md` — 更新 astronomy 状态
- `docs/release/v1-closeout.md` — 更新能力矩阵
- `README.md` — 更新 supported 计数
- `markdown/10-overview/overview-full-feature-tree.md` — 同步
- `markdown/20-roadmap/96-recursive-cursor.md` — 游标推进
- `markdown/20-roadmap/97-loop-closeout-log.md` — closeout 记录
