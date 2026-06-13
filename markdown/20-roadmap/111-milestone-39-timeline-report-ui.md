# M39: Timeline Report UI

## 0. Status

`closed by LOOP-103`. M39 started after the backend/report contracts for M36-M38 stabilized and closed as a UI/readability milestone only.

## 1. Goal

把大运/流年解释以可读、可扫、可展开的方式放进主盘报告和四专项报告。工作台保持轻量，完整解释放到报告页，避免主界面重新变成密集文本堆。

## 2. Dependencies

| Dependency | Why |
| --- | --- |
| M36 | 主盘大运解释内容 |
| M37 | 年度引动解释内容 |
| M38 | 四专题时间叠加章节 |
| LOOP-097 | 独立 topic report 页面和工作台摘要边界 |
| M25-M27 | GPT Pro visual upgrade, report portal, colloquial content |

## 3. Scope

| WP | Work Package |
| --- | --- |
| M39-WP1 | 主盘报告新增时间解释导航：大运阶段、年度引动、证据追踪 |
| M39-WP2 | 工作台大运时间轴加入短说明和当前大运高亮，避免长文 |
| M39-WP3 | 四专项报告页新增“本专题的大运流年”章节 |
| M39-WP4 | 年份选择控件必须把 `year` 显式传入 API |
| M39-WP5 | 提供可展开证据：专业术语、白话解释、原局/大运/流年证据 |
| M39-WP6 | 移动端布局：卡片/折叠/时间轴不能遮挡四柱和报告目录 |
| M39-WP7 | 前端测试保护工作台摘要边界、报告页完整内容和 no score display |

## 4. UX Boundary

| Surface | Allowed | Forbidden |
| --- | --- | --- |
| Workbench | 当前大运、短说明、结构信号、进入完整报告按钮 | 长报告、trace 表、免责声明重复堆叠、score |
| Chart report page | 大运阶段解释、年度引动章节、证据折叠 | 事件预测、具体建议、流月/流日 claim |
| Topic report page | 专题大运流年章节、topic lens、边界提醒 | 四专题建议、结果保证、topic 内部自造 scoring |

## 5. Non-Goals

- 不做营销落地页。
- 不新增多 topic dashboard。
- 不把完整报告塞回工作台左下留白区域。
- 不使用 client-side 算法推断命理结果。

## 6. Capability Status

M39 本身不新增业务能力。它只能展示 M36-M38 已经通过 restricted closeout 的能力。

## 8. LOOP-103 Closeout Summary

LOOP-103 implemented M39 by adding a report-page time explanation guide, explicit year controls, and expandable evidence sections for already restricted timeline readings. `frontend/report.html` now surfaces `luck_reading` and `annual_trigger_reading` through a dedicated `时间解释导航` section, while `frontend/src/topic-report-page.js` adds a matching `本专题的时间解释` guide for M38 topic overlays. The workbench remains short-summary only: `frontend/src/ui/render.js` highlights the current major-luck stage, shows visible observation/annual years and evidence counts, and points users to the full report page for detailed evidence.

No backend rule, API route, `/api/capabilities` entry, supported promotion, raw `GET /api/luck/cycles` field, public score, flow-month/day claim, or event prediction was added.

Closeout evidence: `118-milestone-39-closeout.md`.

## 7. Validation

```powershell
npm.cmd run check --prefix frontend
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
cargo check -p minggui-desktop
```

验收必须证明：

- 桌面和移动端文本不重叠。
- 当前年默认值在 UI 层可见，请求仍显式带 `year`。
- 工作台不渲染 full report blocks。
- 报告页不显示 `score_internal` 或 0-100 运势分。
