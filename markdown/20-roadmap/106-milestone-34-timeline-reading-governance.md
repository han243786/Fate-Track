# M34: Timeline Reading Governance

## 0. Status

`closed by LOOP-099`. M34 closes DG-012 through ADR 0022 and permits M35 internal timeline foundation work only. Public route, UI, report chapter, `/api/capabilities` promotion, and any mutation of raw `GET /api/luck/cycles` remain forbidden until later milestone closeouts.

## 1. Goal

把主盘大运、指定年份引动、四专项大运流年叠加统一纳入 post-preview 路线图，同时防止三类回退：

- 把 M13 `luck-cycles` raw calculation 污染成解释能力。
- 把 M29-M33 topic 内部年度引动误称为完整流年/流月系统。
- 用上千条静态断语硬堆解释，造成维护成本、确定性伤害和治理脱钩。

本阶段只定义边界、术语、能力拆分、数据合同和门禁顺序，不实现业务代码。

## 2. Entry Conditions

| Condition | Evidence |
| --- | --- |
| M13 luck-cycles supported | `GET /api/luck/cycles` 只提供顺逆、起运和 8 步大运 |
| M24 chart-report restricted | 已有命盘报告式阅读结构，可复用表达范式 |
| M29-M33 topic reports restricted | 四专项使用显式 `year`，但仍不是完整流年/流月能力 |
| LOOP-097 UI split | 工作台只显示结构信号，完整报告走独立报告页 |
| DG-012 opened | 大运/流年解释层实施前必须关闭 |

## 3. Design Decision To Close

DG-012 必须给出以下结论：

| Decision | Required Outcome |
| --- | --- |
| Capability split | 保留 `luck-cycles` as supported raw calculation；新增解释能力只能从 planned 进入 restricted |
| Route/API shape | 是否新建解释 route，或复用报告 route；必须避免改写现有 raw `/api/luck/cycles` 语义 |
| Year default | API 必须显式传入 `year`；前端可以预填当前年，但请求层不能静默推断 |
| Internal score | `score_internal`、权重分、0-100 运势分不得进入默认公开 API |
| Flow scope | M34-M40 只覆盖大运解释与指定年份引动，不宣称完整流月、流日或逐日推送 |
| Safety copy | 输出必须是结构观察、窗口提示和白话解释，不给确定性成败、医疗、金融、婚恋、亲属命运或职业结果建议 |

## 4. Target Architecture

解释层采用组合式规则引擎，不采用海量静态断语表。

| Layer | Responsibility | Milestone |
| --- | --- | --- |
| `timeline-core` | 从原局、大运、指定年份生成结构信号和证据 | M35 |
| `timeline-lexicon` | 管理十神、五行、干支、合冲刑害、藏干、根气、格局用神的短解释片段 | M35 |
| `timeline-reading` | 把结构信号组合成主盘大运和年度引动白话读法 | M36-M37 |
| `topic-timeline-overlay` | 将共享时间信号映射到情感、金钱、家庭、事业四主题 | M38 |
| `timeline-report-ui` | 主盘与四专项报告页展示时间解释，不挤压工作台 | M39 |
| `timeline-quality-gate` | golden samples、禁用词、安全边界、性能和回归门禁 | M40 |

## 5. Scope

| WP | Work Package |
| --- | --- |
| M34-WP1 | 关闭 DG-012：确认解释层 route/API、显式 year、内部 score、能力上限和安全边界 |
| M34-WP2 | 建立三项 planned capability：`luck-reading`、`annual-trigger-reading`、`topic-timeline-reading` |
| M34-WP3 | 定义解释引擎输出合同草案：`TimelineSignal`、`TimelineEvidence`、`PlainReading`、`RuleVersion` |
| M34-WP4 | 明确 M13 `luck-cycles` 仍只做 raw computation，不新增解释字段 |
| M34-WP5 | 更新风险、能力台账、模块树、全量树、roadmap 索引和递归游标 |

## 6. Non-Goals

- 不实现新 Rust 模块、API route、前端页面或 `/api/capabilities` 运行时变化。
- 不修改现有 `GET /api/luck/cycles` response。
- 不公开 `score_internal`、权重分、0-100 命运分或排序结论。
- 不实现流月、流日、择日、每日运势、事件预测或时间线推送。
- 不给财务、婚恋、家庭、职业、医疗、法律或生死结论。

## 7. Capability Status

| Capability | Before | After M34 Closeout |
| --- | --- | --- |
| `luck-cycles` | supported raw calculation | unchanged |
| `luck-reading` | unlisted | planned; restricted upper bound |
| `annual-trigger-reading` | unlisted | planned; restricted upper bound |
| `topic-timeline-reading` | unlisted | planned; restricted upper bound |

Planned capability 不得出现在 `/api/capabilities` as available，除非后续里程碑完成实现与 closeout。

## 8. Closeout Requirements

M34 关闭前必须证明：

- DG-012 已关闭或明确继续阻塞实现。
- M35-M40 的依赖顺序、输入、输出、非目标和验证门禁已铺设。
- roadmap index、roadmap README、module tree、full feature tree、product tree、risk register、capability ledger、cursor、closeout log 均已同步。
- 项目门禁通过，且没有业务代码或 runtime capability 变化。

## 9. LOOP-099 Closeout

| Evidence | Result |
| --- | --- |
| DG-012 | Closed by `docs/decisions/0022-dg-012-timeline-reading-boundary.md`. |
| Capability split | `luck-cycles` remains supported raw calculation; `luck-reading`, `annual-trigger-reading`, and `topic-timeline-reading` remain restricted-upper-bound planned capabilities until later closeout. |
| M35 entry | Allowed only for internal backend domain foundation: `TimelineSignal`, `TimelineEvidence`, `PlainReading`, `RuleVersion`, lexicon, composition rules, trace, and audit. |
| Public exposure | Still forbidden in M35: public route, frontend UI, report chapter, `/api/capabilities` promotion, public score, silent year default, flow-month/day/event prediction. |
| Runtime behavior | No runtime behavior is changed by M34 itself. |

M34 closeout readiness is therefore satisfied for entering M35, with all post-M34 implementation constrained by ADR 0022 and the M35 milestone file.
