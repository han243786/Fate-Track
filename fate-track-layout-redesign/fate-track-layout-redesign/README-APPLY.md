# 命轨前端排版重做方案

这是一版只改前端呈现层的重排版方案：

- 替换 `frontend/index.html`
- 替换 `frontend/src/styles.css`
- 不修改 `frontend/src/main.js`
- 不修改 `frontend/src/ui/dom.js`
- 不修改 `frontend/src/ui/render.js`
- 不修改 API、状态、业务逻辑

## 设计目标

视觉风格定位为“专业命理排盘软件工作台”：

- 左侧：连接与起盘参数，适合作为固定起盘栏。
- 中央：四柱命盘主视觉，四柱以年、月、日、时分栏展示。
- 中央下方：格局、五行、十神、藏干等结构化分析卡片。
- 右侧：案例库、脱敏分享、历法数据、能力边界。
- 色彩：宣纸底、朱砂按钮、金线分隔、深木命盘、玉色状态。

## 应用方式

在仓库根目录执行：

```bash
cp frontend/index.html frontend/index.html.bak
cp frontend/src/styles.css frontend/src/styles.css.bak
cp fate-track-layout-redesign/frontend/index.html frontend/index.html
cp fate-track-layout-redesign/frontend/src/styles.css frontend/src/styles.css
cd frontend
npm run check
npm start
```

Windows PowerShell：

```powershell
Copy-Item -Force .\frontend\index.html .\frontend\index.html.bak
Copy-Item -Force .\frontend\src\styles.css .\frontend\src\styles.css.bak
Copy-Item -Force .\fate-track-layout-redesign\frontend\index.html .\frontend\index.html
Copy-Item -Force .\fate-track-layout-redesign\frontend\src\styles.css .\frontend\src\styles.css
cd frontend
npm.cmd run check
npm.cmd start
```

## 兼容性说明

JS 依赖的所有 ID 都已经保留，例如：

- `api-base`, `refresh-button`, `health-pill`, `health-label`
- `chart-date`, `chart-time`, `chart-timezone`, `chart-time-precision`
- `chart-run-button`, `case-save-button`, `share-create-button`
- `pillar-grid`, `chart-status`, `chart-warnings`
- `analysis-metrics`, `analysis-cards`
- `case-list`, `share-preview`
- `calendar-date`, `calendar-query-button`, `calendar-status`
- `year-count`, `term-count`, `data-version`, `data-bytes`, `data-range`, `source-path`
- `capability-list`

因此这版属于布局和样式升级，不改变数据流。
