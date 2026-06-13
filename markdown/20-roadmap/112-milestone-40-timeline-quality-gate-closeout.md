# M40: Timeline Quality Gate And Closeout

## 0. Status

`closed` by LOOP-104. M40 is the hardening and closeout milestone for the M34-M39 timeline reading slice.

## 1. Goal

把大运/流年解释层从“能跑”收束到“可长期推进”。M40 不追求新增内容，而是建立 golden samples、禁用词、安全边界、性能、回归和治理同步，使后续 goal run 不会因时间解释层失控而回退。

## 2. Dependencies

| Dependency | Why |
| --- | --- |
| M34 | DG-012 and capability boundary |
| M35 | timeline rule engine |
| M36 | primary luck reading |
| M37 | annual trigger reading |
| M38 | topic timeline overlay |
| M39 | report UI |

## 3. Scope

| WP | Work Package |
| --- | --- |
| M40-WP1 | 建立 timeline golden samples：基础原局、强弱差异、未知时辰、冲合刑害、用神相关、大运/年度叠加 |
| M40-WP2 | 建立 forbidden output suite：财务、婚恋、家庭、职业、医疗、生死、确定成败、具体日期 |
| M40-WP3 | 建立 no-overclaim checks：不宣称完整流月/流日，不污染 raw `luck-cycles` |
| M40-WP4 | 建立 no public score checks：`score_internal`、0-100 运势分、排序分不得进入 public response 或 UI |
| M40-WP5 | 性能门禁：组合式规则不得因 10 x 12 或 60 甲子展开造成不可控模板爆炸 |
| M40-WP6 | 文案门禁：每个解释必须有专业术语、白话解释和边界提醒 |
| M40-WP7 | 关闭 M34-M40 closeout：roadmap、risk、capability、module tree、full tree、cursor、release note 同步 |

## 4. Golden Sample Matrix

| Case Type | Required Evidence |
| --- | --- |
| baseline chart | 原局无明显冲合时仍能输出克制解释 |
| strong day master | 强弱影响解释语气，但不输出人生高低 |
| weak day master | 说明承载和支持结构，不做失败断言 |
| unknown hour | 时柱、子女宫、晚年/表达相关解释降级 |
| major clash | 合冲刑害可解释为结构拉扯，不断事件 |
| favorable structure | 用神相关说明保持“有利于观察某结构”，不说必成 |
| topic overlay | 四专项同源信号，不各自发明结论 |

## 5. Closeout Capability Rules

| Capability | Allowed Closeout |
| --- | --- |
| `luck-reading` | restricted only |
| `annual-trigger-reading` | restricted only |
| `topic-timeline-reading` | restricted only |
| `luck-cycles` | remains supported raw calculation only |

任何 supported promotion 都必须另开里程碑和 ADR，不能在 M40 顺手升级。

## 6. Non-Goals

- 不新增命理算法范围。
- 不扩展流月、流日、择日、每日推送。
- 不做生成式 LLM 文案。
- 不做现实建议和结果保证。

## 7. Validation

```powershell
cargo test timeline -- --nocapture
cargo test topic_report -- --nocapture
npm.cmd run check --prefix frontend
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
cargo check -p minggui-desktop
```

验收必须证明：

- M34-M39 所有 capability、risk、module tree、full tree、cursor 和 closeout 文档一致。
- 所有 forbidden output 和 no public score 测试通过。
- full project gate 通过。
- 若 `cargo fmt --check` 仍打印既有 diff，不得在无关代码上执行格式化或回退。

## 8. LOOP-104 Closeout Summary

M40 关闭为质量门禁和治理收口，不新增命理算法、API route、前端业务能力、`/api/capabilities` 条目或 supported 晋级。

| WP | Result |
| --- | --- |
| M40-WP1 | `m40_timeline_public_quality_gate_covers_golden_samples` 覆盖主盘 baseline、未知时辰、year-only 以及四专项 topic overlay 样例 |
| M40-WP2 | public response forbidden suite 覆盖财富、婚恋、家庭、职业、确定结果、完整流月/流日和每日运势高风险词 |
| M40-WP3 | raw `GET /api/luck/cycles` 继续被测试为只含 raw 大运排盘，不含 reading、topic overlay、专业/白话解释或 score |
| M40-WP4 | 后端 public response 和前端 UI source 均测试 `score_internal`、0-100 运势分不出现 |
| M40-WP5 | `m40_timeline_quality_gate_keeps_compositional_output_bounded` 约束 timeline lexicon、signals、evidence、readings 和文本体量，防止 10 x 12 / 60 甲子模板爆炸 |
| M40-WP6 | domain gate 要求每条 reading 同时保留 `专业说法`、`白话说` 和“不是真实事件预告”的边界句 |
| M40-WP7 | roadmap、risk、capability、module tree、overview full tree、full product tree、cursor、release note 和 closeout log 已同步 |

M34-M40 timeline slice 已完成：`luck-reading`、`annual-trigger-reading`、`topic-timeline-reading` 仍全部为 restricted；`luck-cycles` 仍为 supported raw calculation；运行时保持 10 supported、14 restricted、0 planned。
