# 递归循环运行手册

> 本手册把 `95-recursive-development-protocol.md` 转成每轮可执行操作。它用于人和 Codex 在每次循环中按同一顺序行动。

## 1. LOOP 启动口令

每轮启动前，必须把本轮意图归一化为一句话：

```text
启动 LOOP-XXX：在 <mode> 下处理 <milestone/work_package>，允许范围 <allowed_scope>，禁止范围 <forbidden_scope>。
```

如果无法写出这句话，本轮不得开始。

## 2. Read 步骤

按顺序读取：

1. `markdown/20-roadmap/96-recursive-cursor.md`
2. `markdown/20-roadmap/97-loop-closeout-log.md` 的上一轮记录
3. `markdown/20-roadmap/00-roadmap-index.md`
4. 当前里程碑文件
5. `markdown/20-roadmap/90-decision-gates.md`
6. `markdown/20-roadmap/91-anti-regression-and-governance-lock.md`
7. `markdown/20-roadmap/92-risk-register.md`
8. `markdown/20-roadmap/93-capability-promotion-ledger.md`

## 3. Slice 步骤

切片必须满足：

- 可以在一轮内完成。
- 有明确文件范围。
- 有明确禁止范围。
- 可以运行门禁。
- 可以写 closeout。

不合格切片必须继续拆小。

## 3.1 Optimized Slice Size

After `LOOP-005`..`LOOP-007`, the preferred slice is the largest stable invariant, not the smallest possible file edit.

| Size | Allowed when | Stop condition |
| --- | --- | --- |
| `S` | One local function/file or one doc-only decision task | Useful for uncertain or risky areas |
| `M` | One API/contract family with tests and governance sync | Stop before unrelated UI/storage/domain work |
| `L` | One invariant crosses backend/frontend/docs and can still pass full gate | Stop before capability promotion or decision-gate change if not preflighted |
| `XL` | One complete milestone work package with explicit closeout target | Stop at milestone closeout and decision gates |

For `L` or `XL`, write the invariant into preflight before implementation. A slice is too large if it needs two unrelated invariants, cannot update module tree + engineering tree + closeout in the same loop, or would hide a capability status change.

## 4. Preflight 表

| Check | Required answer |
| --- | --- |
| Active mode | `design_only`, `single_loop`, `milestone_loop`, or `goal_run` |
| User authorization | 是否允许代码实现 |
| Active milestone | M0-M9 |
| Active work package | WP ID or process task |
| Decision gates | open/blocked/not applicable |
| S0 risks | zero / blocked |
| Capability delta | none / planned->restricted / restricted->supported |
| Governance files | exact list |
| Validation command | exact command |

## 5. Implement / Govern 步骤

在 `design_only` 中，Implement 只能表示“落实治理文档”，不能表示业务代码实现。

代码阶段的 Implement 必须遵守：

- 只触碰本轮 slice。
- 不顺手推进下一功能。
- 不改 capability 状态，除非 Preflight 已授权。

Govern 必须在 Validate 前完成。

## 6. Validate 步骤

默认完整门禁：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

专项门禁可追加，但不能替代完整门禁，除非 closeout 明确说明原因。

## 7. Closeout 步骤

每轮必须追加 `97-loop-closeout-log.md`，字段至少包括：

- mode
- scope
- completed_scope
- changed_files
- capability_status_changes
- validation_result
- governance_updates
- risk_updates
- unresolved_decision_gates
- next_cursor

## 8. Advance 步骤

更新 `96-recursive-cursor.md`：

- `loop_id` 指向刚完成或下一轮。
- `state` 更新为 `paused`, `reading`, `blocked` 或下一状态。
- `last_green_gate` 更新。
- `last_closeout` 指向最新 LOOP。
- `next_resume_instruction` 写到无需猜测。

## 9. 错误处理

| Situation | Action |
| --- | --- |
| 门禁失败 | 修复本 slice 或标记 blocked |
| 决策门阻塞 | 不实现，转决策任务 |
| 发现 S0 | 停止推进，更新风险台账 |
| 用户改变方向 | 更新 cursor 和 closeout，重新 Read |
| 工作包过大 | 回到 Slice 拆小 |
