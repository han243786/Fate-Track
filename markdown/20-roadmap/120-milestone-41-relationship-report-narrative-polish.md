# M41 Relationship Report Narrative Polish

## 1. 目标

将 `relationship-report` 作为单一切面先行打磨，把情感专项完整报告从结构说明式文本调整为命理师式六块正文：

1. 总断
2. 伴侣议题
3. 夫妻宫
4. 表达、边界与安全感
5. 年度情感引动（正文必须使用显式年份）
6. 结论

本里程碑只改变情感专项正文组织与文案质量，不改变底层排盘、十神、合冲刑害、大运、流年引动计算，不改变 API route、DTO 顶层结构或 capability 状态。

## 2. 范围

### 允许

- 重排 `relationship-report` 的 `blocks`，固定为六个正文块。
- 将原 `本专题的大运流年` 的 shared timeline overlay 整合进显式年份情感引动块。
- 开头 `disclaimer` 保留一次总提醒；正文不反复打断用户阅读。
- 正文允许温和现实策略表达，例如“先观察稳定回应”“看对方是否尊重节奏”“把需求说清楚”。
- 保留 `signals` 与 `trace` 作为证据层，前端仍可折叠查看。
- 更新测试，锁住六块标题、顺序、显式年份、无 public score、无确定性断言。

### 禁止

- 不新增 public API。
- 不新增 capability，不把 `relationship-report` 晋级 supported。
- 不改写 `GET /api/luck/cycles` raw supported 语义。
- 不公开 `score_internal`、0-100 运势分或排序分。
- 不引入完整流月、流日、择日、每日推送或事件时间表。
- 不输出确定性结果：必然结婚、必然离婚、一定发生、结果保证等。
- 不对现实对象做身份断言或事件断言。

## 3. 设计

### 3.1 数据映射

| 正文块 | 数据来源 | 叙事职责 |
| --- | --- | --- |
| 总断 | 日主、日柱、夫妻宫、日主强弱、格局 | 先给用户关系主线，后解释结构来源 |
| 伴侣议题 | 配偶星、性别取象、官杀/财星 | 说明吸引点、伴侣议题和长期适配观察 |
| 夫妻宫 | 日支、月支、时支、大运首段、年度地支关系 | 说明亲密关系落点和相处节奏 |
| 表达、边界与安全感 | 食伤、比劫、印星、用神提示 | 说明表达方式、边界和安全感需求 |
| 年度情感引动 | `year` 年柱、当前大运、夫妻宫关系、topic timeline overlay | 把年度和大运牵动合并成用户可读的年度情感主题，正文直接写出具体年份 |
| 结论 | 前五块摘要、warnings、sensitivity flags | 收束成关系策略与不确定性说明 |

### 3.2 文案规则

- 标题固定六块，不再出现额外 `本专题的大运流年` 正文块。
- 每块先说对用户意味着什么，再补命理依据。
- 专业词必须解释，但不把正文写成术语教程。
- 只在开头总提醒里说明传统文化参考和现实处境优先；正文不反复出现大段边界声明。
- 现实策略必须是观察型、可回撤、低风险表达。
- 年度表达必须用具体年份，例如 `2026年`，不得退回泛化年份口吻。

## 4. 验收

- `relationship-report` API 响应仍为 restricted。
- `blocks` 正文数量为 6，标题顺序与本文件一致。
- 响应仍包含 `signals` 与 `trace`，并保留 `topic-timeline-overlay` trace/source 证据。
- 响应正文包含 `年度情感引动`、`2026年`、`丙午`、当前大运、夫妻宫、配偶星、食伤、比劫、印星。
- 响应正文不包含 `score_internal`、`0-100`、确定性婚恋断语、内部英文标识、后端/前端工程口径、硬标签和泛化年份病句。
- `wealth-report`、`family-report`、`career-report` 不在本里程碑打磨范围内，现有行为保持。
- 全量门禁通过：`cargo test -- --nocapture`、`npm.cmd run check --prefix frontend`、`tools/check-project.ps1`、`cargo check -p minggui-desktop`。

## 5. Closeout 预期

M41 关闭时必须同步：

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
