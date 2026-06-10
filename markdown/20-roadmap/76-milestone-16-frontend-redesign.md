# M16: Frontend Layout Redesign

## 1. 目标

将前端从 light-mode 单列布局升级为专业命理排盘工作台 dark-mode 三栏布局。仅替换呈现层（HTML + CSS + render.js），不修改业务逻辑、API 调用、状态管理或后端代码。

## 2. 依赖

- M7 前端工作台已实现全部面板。
- M15 V1 能力矩阵已锁定。
- 所有 JS 依赖的 DOM ID 在新 HTML 中保留。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M16-WP1 | 替换 `frontend/index.html`：三栏布局（左侧起盘栏 + 中央盘面分析 + 右侧辅助信息） |
| M16-WP2 | 替换 `frontend/src/styles.css`：dark 主题，宣纸底/朱砂/金线/深木盘面/玉色 |
| M16-WP3 | 替换 `frontend/src/ui/render.js`：pillarCard 组件、metricItem、状态文本映射 |
| M16-WP4 | 前端测试验证：`npm run check` + 浏览器手动检查 |

## 4. 非目标

- 不修改 `frontend/src/main.js`
- 不修改 `frontend/src/ui/dom.js`
- 不修改 `frontend/src/api/client.js`
- 不修改 `frontend/src/state.js`
- 不修改后端 Rust 代码
- 不添加新的 API 能力

## 5. 能力状态

无变化。`frontend-chart-workspace` 保持 restricted。

## 6. 验证

```bash
cd frontend
npm run check      # 语法 + 测试
# 手动检查: node server.mjs, 打开 http://127.0.0.1:5173
```
