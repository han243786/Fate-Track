# M36: Primary Chart Luck Reading

## 0. Status

`closed by LOOP-100`. M36 starts after M35 rule engine foundation is accepted and is now implemented as a restricted `luck-reading` surface carried by `GET /api/charts/report?reading_year=YYYY`.

## 1. Goal

把主盘的大运展示从“干支列表”升级为小白可读的十年阶段解释。用户应能看懂当前大运是什么、为什么被触发、它和原局结构有什么关系，以及哪些表达是命理结构观察而不是现实结论。

## 2. Dependencies

| Dependency | Why |
| --- | --- |
| M35 | `TimelineSignal`、词典、trace、审计 |
| M13 | 大运序列、顺逆、起运年龄 |
| M21 | 强弱、格局、用神 |
| M24 | 主盘报告表达范式 |
| M25-M27 | 前端视觉和白话化报告规范 |

## 3. Scope

| WP | Work Package |
| --- | --- |
| M36-WP1 | 识别当前大运和前后阶段，但只做阶段观察，不断具体事件 |
| M36-WP2 | 为每步大运输出结构解释：天干、地支、藏干、十神、五行、原局关系 |
| M36-WP3 | 为当前大运输出白话引导：它像什么样的十年背景，容易让哪些结构被看见 |
| M36-WP4 | 在主盘报告中新增大运解读章节，保持免责声明和非确定性口径 |
| M36-WP5 | 在工作台大运时间轴中加入短解释，但避免挤占四柱和结构信号 |
| M36-WP6 | 保护 raw `GET /api/luck/cycles`：如需解释，使用 DG-012 指定的新 route 或 report route，不改 raw route 语义 |
| M36-WP7 | 回归测试未知时辰、大运仍可排、解释必须降级的情况 |

## 4. User Reading Shape

主盘大运解释至少包含：

- 当前阶段：这一步大运的干支、十神、五行和时间范围。
- 专业解释：透出、藏干、合冲刑害、用神/忌神或结构拉扯。
- 白话解释：用日常语言说明这类结构更像“资源变多”“表达被看见”“责任变重”“边界感被拉扯”等。
- 边界提醒：这不是成败预测，也不是建议用户做某件事。

## 5. Non-Goals

- 不把每个 60 甲子绑定固定吉凶断语。
- 不输出“此运发财、结婚、升职、破财、离婚”等确定性结果。
- 不提供投资、择业、婚恋、医疗或法律建议。
- 不开放完整流年列表、流月或流日。

## 6. Capability Status

| Capability | Before | After Closeout |
| --- | --- | --- |
| `luck-cycles` | supported raw calculation | unchanged |
| `luck-reading` | planned | restricted; carried by `/api/charts/report`, no raw route mutation |

Restricted 的含义：解释是传统结构观察和白话说明，不是运势保证。

## 7. Validation

```powershell
cargo test luck_reading -- --nocapture
npm.cmd run check --prefix frontend
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

验收必须证明：

- raw `luck-cycles` response 不被解释字段污染。
- 主盘报告或指定解释 route 包含当前大运、十神、五行、原局关系、白话解释和边界提醒。
- 禁用词审计通过。
- README、module tree、feature tree、capability ledger 和 closeout 同步。

## 8. LOOP-100 Closeout Summary

| Item | Result |
| --- | --- |
| Route shape | `GET /api/charts/report?reading_year=YYYY` carries `luck_reading`; `GET /api/luck/cycles` remains raw supported calculation |
| Backend | `domain::timeline::build_major_luck_stage_foundation()` returns traceable signals/evidence/readings for current major-luck stage |
| Report | Chapter 9 now lists all major-luck cycles and explains the selected-year current stage with professional wording, plain language, and boundary reminders |
| Frontend | Workbench timeline highlights the computed current stage and shows a short structural summary; full report page passes `reading_year` explicitly |
| Capability | `luck-reading` promoted from planned to restricted in `/api/capabilities` and capability ledger |
| Safety | Public API/UI still excludes `score_internal`, 0-100 fate score, full flow-month/day claims, and event prediction |
