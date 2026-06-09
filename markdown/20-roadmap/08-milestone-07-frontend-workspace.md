# M7: Frontend Workspace

## 1. 目标

把前端从能力看板推进为命盘工作台：新建命盘、规则提示、命盘详情、分析卡、案例入口、分享预览、万年历和术语。

## 2. 依赖

- M3 提供 chart API。
- M4 提供 analysis API 或 fixtures。
- M5/M6 提供 cases/share API，或明确 planned 降级。
- GP-FE-001、GP-FE-004、GP-FE-005 必须作为 UI 评审规则。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M7-WP1 | App shell 与路由层 |
| M7-WP2 | 新建命盘 stepper：日期、时间、地点/时区、规则、预览 |
| M7-WP3 | ChartWorkspace：四柱头部、规则徽标、边界 warning、tabs |
| M7-WP4 | Analysis cards：五行、十神、藏干、关系、安全免责声明 |
| M7-WP5 | Case list/detail 前端视图 |
| M7-WP6 | Share preview/public view |
| M7-WP7 | Calendar page：日期查询、农历、干支、节气 |
| M7-WP8 | Glossary drawer/chips |
| M7-WP9 | 移动端与键盘可访问性 |

## 4. 非目标

- 不实现后端不存在的能力。
- 不在前端复制核心排盘算法。
- 不做 AI 长篇断语。
- 不做社交、支付、咨询入口。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `frontend-chart-workspace` | planned | supported 或 restricted |
| `frontend-share-preview` | planned | supported 或 restricted |
| `glossary` | planned | supported 或 restricted |

## 6. 防回退

- UI 能力标签必须来自后端 capability 或本台账。
- 没有后端支持的功能只显示 planned/restricted，不显示成功路径。
- 图表必须有文本替代。
- 分析文案必须使用安全表达和免责声明。
- 移动端不得让文本溢出或遮挡核心信息。

## 7. 治理同步

- `module-tree.md` 登记 `frontend.chart-workspace`, `frontend.share-preview`, `frontend.glossary`。
- `overview-full-feature-tree.md` 登记新文件。
- `README.md` 更新运行和页面说明。
- `93-capability-promotion-ledger.md` 更新前端能力。

## 8. 验收

- `npm.cmd run check`。
- Playwright 或浏览器手动验证关键路径。
- 移动端截图检查。
- 键盘可达性和基本 ARIA 标签检查。
- `tools/check-project.ps1` 通过。

## 9. 进入 M8 条件

- 关键用户路径可从前端完成或明确降级。
- supported/restricted/planned 状态在 UI、API、README、台账一致。
- S0 文案和隐私风险清零。
## 10. M7 Closeout Update

M7 closes through LOOP-017 as a restricted frontend workspace.

Capability status after LOOP-017:

| Capability | Before | After | Boundary |
| --- | --- | --- | --- |
| `frontend-chart-workspace` | planned | restricted | consumes supported/restricted APIs; no local chart/analysis algorithms, luck cycles, durable sharing, cloud sync, account storage, true solar time, timezone history, range expansion, glossary, or astronomy replacement |
| `frontend-share-preview` | planned | restricted | consumes restricted M6 share-preview API; redacted/read-only only |
| `glossary` | planned | planned | backend glossary remains planned |

Acceptance evidence:

- Frontend workspace renders chart input, chart pillars, analysis metrics/cards, local case list, redacted share preview, date-layer probe, data metadata, and capability boundaries.
- API client tests cover chart, analysis, case, and share routes.
- Browser checks confirmed desktop render, save/share interaction, no private note or snapshot id in share preview, and mobile 390px layout without horizontal overflow.
- `tools/check-project.ps1` passed with Rust 51 tests and frontend 8 tests.

