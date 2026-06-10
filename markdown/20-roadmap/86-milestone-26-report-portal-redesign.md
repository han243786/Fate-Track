# M26: Report Portal Redesign — 玄金星轨卷宗

## 1. 目标

将报告页从普通文章页升级为沉浸式门户长卷。报告页独立于首页一屏仪表盘，采用可滚动章节式布局，包含封面、固定目录、章节卡片和滚动动画。首页与报告页形成两种不同的阅读体验：仪表盘 vs 门户。

## 2. 依赖

- M24 chart-report 后端口语化报告已实现。
- M25 GPT Pro 前端视觉升级已完成。
- M26.1 补丁修复目录定位和章节居中。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M26-WP1 | 修复视口锁：`html:not(.report-root)` 排除报告页，报告页恢复自然滚动 |
| M26-WP2 | 报告封面：5 环罗盘背景、标题刻印动画、meta 芯片依次点亮、向下启封按钮 |
| M26-WP3 | 悬浮导航：毛玻璃 header、金/翠/朱砂渐变色进度条、百分比读标 |
| M26-WP4 | 侧栏目录：sticky 索引、等宽编号、玉青节点指示器、IntersectionObserver active 追踪 |
| M26-WP5 | 章节卡片：frame/aura/scan 三层特效、编号水印、kicker→title→rule→body→footer 序列揭示 |
| M26-WP6 | 滚动动画：CSS custom properties 驱动 parallax 星尘、hero 溶解、罗盘呼吸、scroll-snap |
| M26-WP7 | 归档页尾：朱砂「归」印章 + 章节计数 |
| M26-WP8 | 移动端适配：单列、自然滚动、目录不 fixed |
| M26.1 | 目录 fixed 修复 + 章节居中 + 滚动到中心 |

## 4. 非目标

- 不修改后端 API
- 不新增能力
- 不影响首页布局

## 5. 能力状态

无变化。`chart-report` 保持 restricted。
