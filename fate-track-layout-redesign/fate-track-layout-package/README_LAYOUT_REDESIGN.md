# 命轨前端排版 + 样式重做包

本包只重做前端排版与视觉风格，不修改 JavaScript 业务逻辑、API 调用、后端能力边界。

## 文件

```text
frontend/index.html
frontend/src/styles.css
style-preview.html
```

## 应用方式

在项目根目录执行：

```powershell
Copy-Item -Force .\frontend\index.html .\frontend\index.html.bak
Copy-Item -Force .\frontend\src\styles.css .\frontend\src\styles.css.bak
Copy-Item -Force .\frontend\index.html <项目目录>\frontend\index.html
Copy-Item -Force .\frontend\src\styles.css <项目目录>\frontend\src\styles.css
```

或者直接把本包中的 `frontend/index.html` 与 `frontend/src/styles.css` 覆盖到仓库同名路径。

## 设计说明

新版采用命理排盘软件常见工作台结构：

- 顶部：品牌、功能模块、后端健康状态。
- 左侧：连接、起盘参数、日期层查询。
- 中部：四柱命盘主盘、结构化分析。
- 右侧：案例库、分享预览、数据源。
- 底部：能力边界。

所有 JS 依赖的 id 均保留：`api-base`、`refresh-button`、`health-pill`、`health-label`、`chart-date`、`chart-time`、`chart-timezone`、`chart-time-precision`、`case-title`、`case-tags`、`case-note`、`chart-run-button`、`case-save-button`、`share-create-button`、`pillar-grid`、`chart-warnings`、`analysis-metrics`、`analysis-cards`、`case-list`、`share-preview`、`calendar-date`、`calendar-query-button`、`calendar-gregorian`、`calendar-lunar`、`calendar-ganzhi`、`calendar-ruleset`、`year-count`、`term-count`、`data-version`、`data-bytes`、`data-range`、`source-path`、`capability-list`。

## 预览

打开 `style-preview.html` 可以查看静态示例效果。实际运行项目时，页面数据仍由现有 `src/main.js` 和 `src/ui/render.js` 渲染。
