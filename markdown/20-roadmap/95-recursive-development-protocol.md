# 递归式开发协议

> 本协议把开发流程抽象为可追踪递归。它用于后续代码开发、重构和治理同步；在用户发出“敲定方案/开始实现”前，本协议只作为流程约束，不触发业务代码推进。

## 1. 核心递归函数

```text
Develop(cursor):
  1. Read
     读取 roadmap index、当前里程碑、决策门、防回退锁、风险台账、能力晋级台账、上一轮 closeout。

  2. Slice
     选择当前游标下最小可闭环工作包。成熟前一轮只处理一个 WP；成熟后可处理一个 milestone。

  3. Preflight
     检查未关闭决策门、S0/P1 风险、能力状态上限、禁止回退规则、需要同步的治理文档。

  4. Implement
     只实现本轮 slice，不顺手推进下一能力，不提前晋级 supported。

  5. Govern
     同步模块树、工程树、能力台账、ADR/Policy/README/UI 状态。

  6. Validate
     跑完整质量门禁，必要时加专项验证。

  7. Closeout
     写入 loop result：范围、文件、能力变化、验证结果、治理同步、剩余风险。

  8. Advance
     更新递归游标到下一 WP、下一 milestone，或 blocked/paused。

  9. Repeat
     下一轮必须读取更新后的 cursor 和上一轮 closeout。
```

## 2. 递归粒度

| 成熟度 | 一轮递归范围 | 适用阶段 | 规则 |
| --- | --- | --- | --- |
| R0 | 只讨论流程，不写代码 | 当前阶段 | 用户未敲定前只能优化协议 |
| R1 | 单一 work package | 初期开发 | 每轮必须 closeout |
| R2 | 单一 milestone | 流程稳定后 | milestone 内仍保留 WP 证据 |
| R3 | 多 milestone goal run | 用户显式开启 goal 后 | 每个 milestone 必须自动 closeout，不得跳过游标 |

## 2.1 Optimized Loop Scale

After `LOOP-005`..`LOOP-007`, the recursion rule is upgraded from "smallest useful slice" to "largest stable invariant":

```text
Maximize code per loop inside one invariant.
Split as soon as the loop needs a second invariant.
```

| Size | Meaning | Guard |
| --- | --- | --- |
| `S` | one tiny contract | local test |
| `M` | one work-package family | full gate + governance sync |
| `L` | one invariant across code/tests/docs | manifest or supported API evidence |
| `XL` | one full milestone | milestone closeout + no blocking gate |
| `GOAL` | multiple milestones | explicit goal command + goal readiness audit |

Large loops are allowed only when module tree, engineering tree, capability ledger, README/UI docs, validation, closeout, and cursor can all be updated in the same loop.

## 3. 游标字段

递归游标必须至少记录：

| 字段 | 含义 |
| --- | --- |
| `loop_id` | 当前循环编号，如 `LOOP-000` |
| `mode` | `design_only`, `single_loop`, `milestone_loop`, `goal_run` |
| `current_milestone` | 当前里程碑 ID |
| `current_work_package` | 当前工作包 ID，未知时为 `none` |
| `state` | `reading`, `preflight`, `implementing`, `governing`, `validating`, `closing`, `paused`, `blocked` |
| `allowed_scope` | 本轮允许触碰的范围 |
| `forbidden_scope` | 本轮禁止触碰的范围 |
| `active_decision_gates` | 阻塞或影响本轮的决策门 |
| `active_locks` | 本轮必须遵守的防回退锁 |
| `capability_delta` | 本轮能力状态变化，默认为 `none` |
| `required_governance_sync` | 必须同步的治理文件 |
| `validation_commands` | 本轮必须运行的检查 |
| `last_green_gate` | 最近一次完整门禁通过记录 |
| `last_closeout` | 上一轮 closeout 位置 |
| `next_resume_instruction` | 下一轮恢复指令 |

## 4. Preflight Gate

每轮实现前必须回答：

- 当前 slice 属于哪个 milestone 和 work package。
- 是否有未关闭决策门阻止实现。
- 是否有 S0 风险未清零。
- 本轮是否可能触发 supported 晋级。
- 如果触发晋级，`93-capability-promotion-ledger.md` 条件是否齐备。
- 哪些模块树、工程树、ADR、Policy、README、UI 状态必须同步。
- 是否存在用户未明确授权的代码推进。

任一回答不清楚时，本轮只能停在 `preflight` 或 `blocked`，不得进入 `implementing`。

## 5. Postflight Gate

每轮实现后必须完成：

- 质量门禁运行并记录结果。
- 新增/变更 public surface 已同步模块树。
- 新增/删除/迁移文件已同步工程树。
- 能力状态变化已同步 capability ledger 和 `/api/capabilities` 或对应 UI/README。
- 风险台账更新。
- closeout 写入 `97-loop-closeout-log.md` 或独立 closeout 文件。
- 游标更新到下一状态。

## 6. 递归暂停条件

出现以下任一情况，递归必须暂停：

- S0 风险未清零。
- 决策门阻塞当前工作包。
- 完整质量门禁失败，且本轮无法在同一 slice 内修复。
- capability 状态和证据不一致。
- 模块树、工程树、policy 或 ADR 冲突。
- 用户要求只评估流程或重新设计流程。
- 发现可能需要回退已有 supported 能力。

## 7. 递归返回值

每轮 closeout 的返回值必须包含：

```text
LoopResult:
  completed_scope:
  changed_files:
  capability_status_changes:
  validation_result:
  governance_updates:
  risk_updates:
  unresolved_decision_gates:
  next_cursor:
```

下一轮递归必须读取上一轮 `LoopResult`，不得只读取里程碑文件。

## 8. Goal Run 启动条件

只有在用户显式要求开启 goal/一路平推后，才允许进入 `goal_run`。进入前必须满足：

- 至少连续 3 个 single_loop 成功 closeout。
- 没有未清 S0 风险。
- 用户确认递归协议成熟。
- `96-recursive-cursor.md` 已能准确恢复状态。
- `97-loop-closeout-log.md` 至少记录过一次完整循环。
- `100-recursive-scale-and-goal-readiness.md` 的 audit 必须标为 `ready`。
- 未关闭决策门不得阻塞 goal 范围内的第一个 milestone。
- goal scope 必须定义 milestone stop points；`goal_run` 不得跳过 milestone closeout。

## 9. 与现有治理的关系

- 本协议不替代里程碑文件，只控制里程碑执行方式。
- 本协议不替代 closeout 模板，只规定 closeout 必须成为递归返回值。
- 本协议不替代能力晋级台账，只规定晋级必须由台账授权。
- 本协议不替代防回退锁，只让每轮递归主动读取锁。
