# M22: Frontend Report Export

## 1. 目标

前端支持导出分析报告。报告完全由本地计算生成（调用 M17 导出 API + M21 深层分析），纯前端组装，不依赖服务端渲染。

## 2. 依赖

- M17 case-export 已提供后端导出 API。
- M21 deep-analysis 已提供深层分析卡片。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M22-WP1 | 前端导出按钮：生成并下载 JSON 分析报告 |
| M22-WP2 | 报告含：命盘四柱、五行/十神/藏干指标、深层分析卡片、算法版本元数据 |
| M22-WP3 | 前端测试：导出按钮存在、JSON 格式有效 |

## 4. 非目标

- 不导入
- 不批量导出
- 不生成 PDF/富格式
- 不涉及在线服务

## 5. 能力状态

`frontend-chart-workspace` 保持 restricted。
