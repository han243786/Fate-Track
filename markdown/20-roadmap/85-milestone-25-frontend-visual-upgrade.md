# M25: Frontend Visual Upgrade — GPT Pro 设计替换

## 1. 目标

采用 GPT Pro 设计系统替换当前前端视觉呈现。保留全部已有功能、API 调用、状态管理和测试契约，仅替换 HTML 结构、CSS 样式和渲染逻辑。不新增任何能力，不修改后端代码。

## 2. 依赖

- M7 前端工作台全部功能已实现。
- M16 三栏布局重设计已提供基础方向。
- M24 排盘报告独立页面已完成。
- GPT Pro 复刻设计稿位于 `fate-track-ui-replica/`。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M25-WP1 | 替换 `frontend/index.html`：采用 GPT 三栏布局（360px 侧栏 + 弹性工作区 + 335px 大运时间轨），保留全部测试要求的 DOM ID（chart-date, chart-time, chart-sex, chart-run-button, lunar-display, chart-report-button）及 aria-labelledby 引用 |
| M25-WP2 | 替换 `frontend/src/styles.css`：采用 GPT 设计变量系统（墨绿底、金/翠/朱砂/水色、元素色编码、装饰伪元素），适配现有 class 命名约定 |
| M25-WP3 | 重写 `frontend/src/ui/render.js`：适配新 DOM 结构——pillarStage 四柱卡片（元素色编码）、elementBars 条形图、godChips 芯片、insightGrid 洞察卡、luckTimeline 时间轴 |
| M25-WP4 | 适配 `frontend/src/ui/dom.js`：映射新 DOM 选择器 |
| M25-WP5 | 前端测试回归：`npm run check` 10 项全绿 |

## 4. 非目标

- 不新增 REST API 路由
- 不修改后端代码
- 不新增能力（能力矩阵不变）
- 不修改 `report.html`（保持独立报告页面不变）
- 不引入真太阳时、云同步、账户系统等 unsupported 功能
- 不修改 `frontend/src/api/client.js`、`frontend/src/state.js`、`frontend/src/main.js` 的核心逻辑

## 5. 能力状态

无变化。所有能力状态保持 M24 closeout 时的矩阵。`frontend-chart-workspace` 保持 restricted。

## 6. 约束

- LOCK-006：不得在前端文案中承诺后端未实现能力（真太阳时、云同步、星历引擎替换等）
- LOCK-009：不得把解释文案写成确定性断言
- 测试契约：`workspace-markup.test.mjs` 要求的 9 个 DOM ID + 4 个 aria-labelledby + 3 个 forbidden 字符串必须全部满足

## 7. 验证

```bash
cd frontend
npm run check      # 语法 + 测试 10 项
# 浏览器检查: node server.mjs, 打开 http://127.0.0.1:5173
```
