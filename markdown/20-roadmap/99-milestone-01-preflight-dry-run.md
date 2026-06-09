# M1 递归预检 Dry-Run

> 本文件由 LOOP-002 生成。它不推进业务代码，只验证递归流程如何为 M1 实现做切片、门禁和治理准备。

## 1. LOOP 启动口令

```text
启动 LOOP-002：在 design_only 下处理 M1 Date Layer Hardening 预检，允许范围为 roadmap dry-run 文档和游标日志，禁止范围为业务代码、API 行为、前端功能和 capability 晋级。
```

## 2. Read 结果

| Source | Result |
| --- | --- |
| `96-recursive-cursor.md` | 当前仍为 `design_only`，不得推进业务代码 |
| `97-loop-closeout-log.md#loop-001` | 上轮完成递归运行手册，门禁通过 |
| `02-milestone-01-date-layer-hardening.md` | M1 目标是强化当前 Android 日期层 |
| `90-decision-gates.md` | DG-002 open，M1 关闭前必须处理验证范围 |
| `91-anti-regression-and-governance-lock.md` | LOCK-001/002/003/011/012 影响 M1 |
| `92-risk-register.md` | R-S0-001、R-P2-003、R-P2-004 相关 |
| `93-capability-promotion-ledger.md` | `calendar-date-query-v1-meta` 目标最早 M1 晋级 |

## 3. Preflight Gate

| Check | Answer |
| --- | --- |
| Active mode | `design_only` |
| User authorization | 未收到“敲定方案/开始实现代码”，所以不允许业务代码 |
| Active milestone | M1 |
| Active work package | dry-run only |
| Decision gates | DG-002 blocks M1 closeout; DG-001/DG-003 affect M2 entry |
| S0 risks | R-S0-001 must remain protected by Android three-pillar tests |
| Capability delta | none |
| Governance files | roadmap dry-run, cursor, closeout log, engineering tree |
| Validation command | `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1` |

## 4. 推荐下一轮最小切片

如果用户后续明确允许开始代码实现，下一轮建议只做：

```text
LOOP-003 / single_loop / M1-WP1
扩展日期查询响应元数据：source、ruleset/algo version、support range、boundary policy note。
```

## 5. M1-WP1 允许范围

| Allowed | Notes |
| --- | --- |
| `backend/src/api/calendar.rs` | 在已有响应中增加元数据字段 |
| `backend/src/calendar/*` | 如需暴露常量或版本，不改变当前算法 |
| `backend/src/api/capabilities.rs` | 仅更新日期层说明，不扩大 supported 范围 |
| `README.md`, `data/README.md` | 说明 Android current baseline 和范围 |
| `module-tree.md`, `overview-full-feature-tree.md` | 同步 public surface |
| `93-capability-promotion-ledger.md` | 记录是否满足 `calendar-date-query-v1-meta` 条件 |

## 6. M1-WP1 禁止范围

- 不实现完整四柱排盘。
- 不实现时柱。
- 不引入 IANA 时区历史。
- 不实现真太阳时。
- 不替换 Android 日期层为星历引擎。
- 不关闭 DG-002，除非同时形成验证范围决策记录。
- 不宣称 1901-2100 外高置信支持。

## 7. M1-WP1 验收建议

- `GET /api/calendar/query?date=2025-01-01` 仍返回 `甲辰/乙丑/庚午`。
- 响应包含可追踪元数据，例如 source、algorithm/version、support range、rule note。
- `cargo test` 保留 Android 三柱样例。
- `tools/check-project.ps1` 通过。
- capability ledger 和 `/api/capabilities` 一致。

## 8. Dry-Run 结论

M1 可以开始，但只能从 M1-WP1 小切片进入。DG-002 不阻止 M1-WP1 的元数据增强，但会阻止整个 M1 closeout 和任何范围外高置信声明。

当前仍未获得代码实现授权，所以游标应保持 `design_only/paused`，等待用户明确指令。

