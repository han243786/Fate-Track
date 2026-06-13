# M43 Remaining Report Human Copy Gate

Status: closed by LOOP-111. Closeout: `125-milestone-43-closeout.md`.

## 1. 目标

在 M42 已完成情感报告真实输出文案门禁后，继续处理剩余四个报告出口：

- 主盘完整报告
- 金钱专项完整报告
- 家庭专项完整报告
- 事业专项完整报告

M43 不新增计算规则、不新增 API、不改变 capability 状态。它只基于真实生成样本，把仍然暴露在报告正文里的机器口吻、内部英文、等号计数和病句压下去。

## 2. 输入样本

本轮以 `target/report-polish-samples/` 下重新生成的真实输出为第一审读对象：

- `main.txt`
- `wealth.txt`
- `family.txt`
- `career.txt`

当前可见问题：

- 主盘报告年度章节暴露 `timeline-core-v1`。
- 金钱、家庭、事业专题时间章节暴露 `annual-trigger`、`annual-current-luck`。
- 金钱、家庭、事业正文存在 `当前提取结果`、`观察年度`、`筛出` 等机器口吻。
- 多个报告使用 `正官=1`、`子 与 午 = 六冲` 这类等号计数或证据格式。
- 部分正文出现 “结构。。 ”、中英文半角括号、年龄段 `1-10岁` 等不够自然的展示。

## 3. 范围

### 允许

- 修改主盘报告的时间解释章节文案和证据展示。
- 修改金钱、家庭、事业专题报告正文模板、年度引动模板、专题大运流年章节和证据展示。
- 新增 app/domain 门禁，覆盖四份真实输出中的 M43 禁用口吻。
- 保持四专项 report route、chart report route 和前端 payload shape 不变。

### 禁止

- 不改 `relationship-report` 的 M41/M42 六块正文基线。
- 不新增 public route。
- 不改变 `GET /api/charts/report` 或 `GET /api/charts/topic-report` 顶层 DTO 合约。
- 不改变 `GET /api/luck/cycles` raw supported 语义。
- 不新增或晋级 capability。
- 不公开 `score_internal`、0-100 分、排序分。
- 不输出确定性财富、家庭、职业、婚恋事件或现实承诺。
- 不扩展为完整流月、流日、择日、每日推送或事件时间表。

## 4. 文案门禁

主盘、金钱、家庭、事业四份 assembled report 不得出现：

- `timeline-core-v1`
- `annual-trigger`
- `annual-current-luck`
- `score_internal`
- `当前提取结果`
- `观察年度`
- `筛出`
- `共享证据`
- `共享时间线`
- `后端`
- `前端`
- `用户`
- `指定年份`
- `这一年`
- `读盘时`
- `主题里`
- `正官=`
- `七杀=`
- `正财=`
- `偏财=`
- `食神=`
- `伤官=`
- `比肩=`
- `劫财=`
- `正印=`
- `偏印=`
- ` = `

四份报告仍必须保留：

- 具体年份，如 `2026年`
- 年柱，如 `丙午`
- 当前大运
- 十神、五行、宫位或专题核心术语
- restricted 边界、无 public score、无确定性现实事件承诺

## 5. 验收

- 主盘、金钱、家庭、事业四份真实样本重新生成后，M43 禁用口吻命中为 0。
- 四份样本中 ASCII word 命中为 0，除 JSON 字段外 assembled report 不暴露内部英文。
- `relationship-report` M42 门禁继续通过。
- 全量治理门禁通过：`cargo test -- --nocapture`、`npm.cmd run check --prefix frontend`、`tools/check-project.ps1`、`cargo check -p minggui-desktop`。

## 6. Closeout 要求

关闭 M43 时至少同步：

- `README.md`
- `docs/release/v1-closeout.md`
- `markdown/00-matrix-governance/module-tree.md`
- `markdown/10-overview/overview-full-feature-tree.md`
- `markdown/命轨全量树.md`
- `markdown/20-roadmap/00-roadmap-index.md`
- `markdown/20-roadmap/89-post-preview-documentation-freeze.md`
- `markdown/20-roadmap/92-risk-register.md`
- `markdown/20-roadmap/93-capability-promotion-ledger.md`
- `markdown/20-roadmap/96-recursive-cursor.md`
- `markdown/20-roadmap/97-loop-closeout-log.md`
- `markdown/20-roadmap/README.md`
