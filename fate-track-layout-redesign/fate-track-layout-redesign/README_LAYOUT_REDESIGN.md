# Fate Track 命理排盘工作台布局重设计

本包只替换前端展示层：

- `frontend/index.html`：重排页面结构，改为命理排盘软件式工作台。
- `frontend/src/styles.css`：完整视觉样式，深色盘面、四柱牌位、侧栏起盘、右侧辅助信息。

未修改：

- `frontend/src/main.js`
- `frontend/src/ui/dom.js`
- `frontend/src/ui/render.js`
- 后端 Rust 代码
- API 调用逻辑

## 覆盖方式

在项目根目录执行：

```bash
cp frontend/index.html frontend/index.html.bak
cp frontend/src/styles.css frontend/src/styles.css.bak
cp fate-track-layout-redesign/frontend/index.html frontend/index.html
cp fate-track-layout-redesign/frontend/src/styles.css frontend/src/styles.css
```

Windows PowerShell：

```powershell
Copy-Item -Force .\frontend\index.html .\frontend\index.html.bak
Copy-Item -Force .\frontend\src\styles.css .\frontend\src\styles.css.bak
Copy-Item -Force .\fate-track-layout-redesign\frontend\index.html .\frontend\index.html
Copy-Item -Force .\fate-track-layout-redesign\frontend\src\styles.css .\frontend\src\styles.css
```

## 兼容性说明

现有 JS 通过固定 ID 查询页面节点，本设计保留了所有必需 ID，因此无需修改业务逻辑。保留的关键节点包括：

- `#api-base`, `#refresh-button`, `#health-pill`, `#health-label`
- `#chart-date`, `#chart-time`, `#chart-timezone`, `#chart-time-precision`
- `#case-title`, `#case-tags`, `#case-note`
- `#chart-run-button`, `#case-save-button`, `#share-create-button`
- `#pillar-grid`, `#chart-warnings`, `#analysis-metrics`, `#analysis-cards`
- `#calendar-date`, `#calendar-query-button`, `#calendar-gregorian`, `#calendar-lunar`, `#calendar-ganzhi`, `#calendar-ruleset`
- `#year-count`, `#term-count`, `#data-version`, `#data-bytes`, `#data-range`, `#source-path`
- `#case-list`, `#share-preview`, `#capability-list`
