# M37: Annual Trigger Reading

## 0. Status

`closed by LOOP-101`. M37 starts after M36 proves 主盘大运解释 can stay restricted and non-deterministic, and closes after `annual-trigger-reading` is exposed only as a restricted chart-report-carried reading with explicit API `year`.

## 1. Goal

建立指定年份的“引动”解释层。它解释某一年干支如何与原局和当前大运发生关系，帮助用户看懂年度结构被哪些十神、宫位、合冲刑害和五行关系激活。

## 2. Key Boundary

API 必须显式传入 `year`。前端可以默认填当前年份或让用户选择年份，但请求层不得静默推断年份。M37 不是完整流年系统，更不是流月/流日系统。

## 3. Dependencies

| Dependency | Why |
| --- | --- |
| M35 | 年度信号、词典、trace |
| M36 | 当前大运背景解释 |
| M3/M4/M21 | 原局、十神、强弱、格局、用神 |
| M13 | 当前大运定位 |
| M24/M27 | 报告与白话表达规范 |

## 4. Scope

| WP | Work Package |
| --- | --- |
| M37-WP1 | 定义 AnnualTriggerInput：chart、sex、timezone、explicit `year`、rule_version |
| M37-WP2 | 计算年度干支与原局四柱的关系信号 |
| M37-WP3 | 计算年度干支与当前大运的叠加信号 |
| M37-WP4 | 输出年度引动说明：被引动的十神、宫位、五行、合冲刑害、强弱/用神关系 |
| M37-WP5 | 输出白话解释：今年更像在提醒哪类结构，而不是会发生什么事 |
| M37-WP6 | 明确前端默认：当前年只是 UI 初始值，请求必须带 `year` |
| M37-WP7 | 加入 forbidden-claim tests：不得输出具体时间点、事件、成败、收益、婚姻、亲属或职业结果 |

## 5. Output Shape

| Section | Content |
| --- | --- |
| 年份口径 | 指定公历年、年干支、所处大运、规则版本 |
| 原局引动 | 年干支触发哪些柱位、十神、五行或关系 |
| 大运叠加 | 当前大运与年度干支是否同向、拉扯或加重某类结构 |
| 白话说明 | 用“更容易被看见/更需要留意/结构上更突出”的口径解释 |
| 边界提醒 | 不断具体事件，不提供现实建议 |

## 6. Non-Goals

- 不生成 12 个月流月表。
- 不生成每日运势、择日、事件时间点或风险日历。
- 不输出“今年必定发财、结婚、升职、破财、失业”等断语。
- 不公开 `score_internal`。

## 7. Capability Status

| Capability | Before | After Closeout |
| --- | --- | --- |
| `annual-trigger-reading` | planned | restricted |
| `luck-reading` | restricted | unchanged |

## 8. Validation

```powershell
cargo test annual_trigger -- --nocapture
npm.cmd run check --prefix frontend
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

验收必须证明：

- API 或 report contract 必须要求显式 `year`。
- 年度说明同时包含原局关系和当前大运关系。
- 未知时辰会降级宫位/时柱相关解释。
- 禁用词审计和 no public score 断言通过。

## 9. LOOP-101 Closeout Summary

M37 is implemented through the existing restricted `GET /api/charts/report` carrier. The route now accepts optional explicit `year=YYYY`; when omitted, `annual_trigger_reading.status` is `not_requested` and no annual-trigger interpretation is generated. Frontend defaults the UI year to the current year but sends it explicitly as both `reading_year` and `year`.

| Area | Evidence |
| --- | --- |
| Domain | `backend/src/domain/timeline.rs` adds `build_annual_trigger_foundation()` for original-chart annual triggers, current-luck background, major-luck/year overlay, warnings, readings, and audit |
| API/report | `backend/src/api/report.rs` adds `AnnualTriggerReadingReport`, `annual_trigger_reading` JSON, and a new `年度引动` report block |
| Capability | `backend/src/api/capabilities.rs` declares `annual-trigger-reading` as restricted on `/api/charts/report?year=YYYY` |
| Raw boundary | `backend/src/app.rs` asserts raw `GET /api/luck/cycles` has no annual reading, no plain interpretation, and no public score |
| Frontend | `frontend/src/api/client.js`, `frontend/src/main.js`, `frontend/report.html`, and `frontend/src/ui/render.js` pass explicit `year` and render only a short annual structure summary in the workspace |
| Validation | `cargo test annual_trigger -- --nocapture`, `cargo test timeline -- --nocapture`, and `npm.cmd run check --prefix frontend` passed before full-gate closeout |
