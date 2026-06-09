# 防回退与治理锁

> 本文件是执行里程碑时的硬防线。它的目标不是增加流程负担，而是防止功能推进时把可信日期层、研究结论、隐私规则和能力状态拆散。

## 1. 防回退规则

| Lock ID | Rule | Applies To | Enforcement |
| --- | --- | --- | --- |
| LOCK-001 | 不得删除、弱化或跳过 Android 日期层三柱黄金样例；替代样例必须先有 ADR。 | calendar/date/bazi | gate+audit |
| LOCK-002 | 不得把 `planned` 或 `target` 能力改成 `supported`，除非 capability ledger 条件全部满足。 | API/frontend/docs | gate+audit |
| LOCK-003 | 不得移除 `ruleset_id`、`algo_version`、边界规则、时区或不确定性元数据。 | chart/calendar APIs | gate+audit |
| LOCK-004 | 不得降低日志和分享脱敏规则来加速实现。 | storage/share/observability | S0 gate |
| LOCK-005 | 不得把研究报告原文直接改写为实现结论；必须通过译文、台账、ADR 和矩阵吸收。 | governance.research | audit |
| LOCK-006 | 不得通过前端静态文案承诺后端未实现能力。 | frontend.ui | gate+audit |
| LOCK-007 | 不得删除门禁命令或把阻断门禁改成非阻断，除非登记等价替代门禁。 | tools/standard matrix | gate |
| LOCK-008 | 不得在里程碑中新增模块或 public surface 而不同步模块树和工程树。 | all modules | audit |
| LOCK-009 | 不得把解释文案写成疾病、死亡、法律、金融、关系或人生确定性断言。 | analysis/share/frontend | S0 gate |
| LOCK-010 | 不得让持久化重新计算旧命盘而不保留原 `algo_version` 和快照可复现性。 | storage/chart | gate+audit |
| LOCK-011 | 不得在递归游标为 `design_only` 时推进业务代码、API 行为、前端功能或 capability 晋级。 | recursive development | S0 gate |
| LOCK-012 | 不得开始新一轮递归而不读取并更新 `96-recursive-cursor.md` 和上一轮 closeout。 | recursive development | gate+audit |

## 2. 治理脱钩症状

出现以下任一现象，必须暂停实现并回到治理同步：

- 代码新增 API，但模块树没有输入/输出/错误/调用方登记。
- UI 展示新能力，但 `/api/capabilities` 或能力晋级台账没有 supported 证据。
- 测试删除或期望改弱，但没有 retired reason。
- 文档宣称星历真源，但运行时代码仍只使用 Android 表且没有说明。
- 新增敏感字段，但 General Policy 或隐私分级没有更新。
- 里程碑 closeout 缺少验证命令和剩余风险。
- 递归游标显示 `design_only`，但出现业务代码、API 行为或 capability 状态推进。
- 新一轮实现没有引用上一轮 LoopResult。

## 3. 回退处理规则

| 回退类型 | 允许条件 | 必须记录 |
| --- | --- | --- |
| 功能降级 | 发现 S0/S1 风险或能力证据不足 | capability ledger、closeout、用户可见状态 |
| 规则变更 | 有更强研究或黄金样例 | ADR、替代样例、旧结果兼容策略 |
| 测试替换 | 新测试覆盖更强或更准确 | retired reason、替代保护、差异说明 |
| 架构延期 | 不影响当前 supported 能力 | roadmap 状态、风险登记、后续关闭条件 |

## 4. 每次实现前检查

- 当前里程碑文件是否已读。
- 当前递归游标是否已读。
- 上一轮 closeout 是否已读。
- 决策门是否影响本次实现。
- 能力晋级台账是否列出 supported 条件。
- 需要同步的治理文档是否已列明。
- 测试和门禁是否不会被削弱。

## 5. 每次实现后检查

- `tools/check-project.ps1` 通过。
- 新 supported 能力有 Rust/API/test/capability 证据。
- 工程树和模块树同步。
- 相关 policy、standard、process 或 guidance 更新。
- closeout 记录剩余风险和未实现边界。
- recursive cursor 已更新到下一状态或 blocked/paused。
