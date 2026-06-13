# M29: Four Topic Report Foundation

## 0. Status

`closed` by LOOP-094 and fully exercised by LOOP-095. M29 foundation is implemented as the shared `TopicReport` domain/API contract behind `GET /api/charts/topic-report`, with explicit API `year`, qualitative `signals`/`trace`, fixed disclaimer, warnings, forbidden-output audit, and no public `score_internal`. LOOP-097 preserves the same contract while splitting presentation: workspace renders only `signals`; `topic-report.html` renders the complete restricted report.

## 1. Goal

把用户授权的四个专项命理推演纳入 post-preview 边界：情感、金钱、家庭、事业。M29 只建立公共基础，不单独完成某一个专项报告。

四个专项共享同一条可审计推演管线：

1. 排四柱。
2. 定日主。
3. 将其他干支换算成十神。
4. 判断日主强弱、格局、喜忌。
5. 按事项取对应十神和宫位。
6. 看大运和当前/指定流年是否引动。
7. 用合冲刑害、透藏根气判断表现形式和时间提示。

## 2. Entry Conditions

| Condition | Required State |
| --- | --- |
| V1 preview | `v1.0.0-preview` 已发布，M0-M28 已关闭 |
| User boundary override | 用户已明确取消功能边界锁，并授权纳入四项功能 |
| Runtime constraint | 本里程碑开始时不得直接改代码；先完成治理铺设 |
| Existing foundation | M3 四柱、M4 十神/五行、M13 大运、M21 强弱/格局/用神、M24 报告框架已存在 |
| Research report | RPT-004 已提供并完成中文规范副本、治理抽取和采纳边界：`markdown/reserch/04-topic-report-engine-governance-intake.md` |
| Decision gate | DG-011 closed for design; LOOP-093 已采纳统一 route、显式 `year`、`score_internal` 不公开三项 implementation preflight 决策 |

## 3. Scope

| WP | Work Package |
| --- | --- |
| M29-WP1 | 定义四专项公共 TopicReport 领域模型：topic、input、basis、blocks、disclaimer、warnings、algo_version、ruleset_id |
| M29-WP2 | 承接深度研究报告：原文归档、中文翻译、规则抽取、风险抽取、采纳/拒绝矩阵（LOOP-092 已完成） |
| M29-WP3 | 定义未来统一 API 合约：`GET /api/charts/topic-report?topic=relationship|wealth|family|career&date=&time=&timezone=&time_precision=&sex=&year=` |
| M29-WP4 | 定义内部年度引动层：只服务四专项报告；API 必须显式传入 `year`，前端可预填当前年但必须随请求传递，不等同于完整流年/流月系统 |
| M29-WP5 | 定义前端入口：主界面左下留白区域放置 2 x 2 按钮，分别为情感、金钱、家庭、事业 |
| M29-WP6 | 复用 M24 报告呈现形式：免责声明、章节标题、专业术语、白话解释、结论边界 |
| M29-WP7 | 建立禁用断语审计：不得输出疾病、死亡、法律、金融收益保证、婚恋结果保证、职业结果保证或家庭事件断言 |
| M29-WP8 | 同步 roadmap、module tree、capability ledger、risk register、recursive cursor 和 closeout |

## 4. Non-Goals

- 不使用 AI/LLM 生成正文。
- 不提供确定性婚恋、财富、家庭、职业断言。
- 不实现完整流年列表、流月、流日、择日或事件预测系统。
- 不新增账号、云同步、持久化、付费、咨询或推荐系统。
- 不把四专项报告标为 supported；实现完成后的能力状态上限为 restricted。

## 4.1 Research Intake Contract

深度研究报告回来后，必须先进入以下处理流：

| Step | Output |
| --- | --- |
| 原文归档 | `markdown/reserch/` 下保留原文，不直接改写 |
| 中文翻译 | `markdown/reserch/zh-CN/` 下生成中文译文 |
| 规则抽取 | 抽取情感、金钱、家庭、事业四类的十神、宫位、运年引动、合冲刑害、透藏根气规则 |
| 采纳矩阵 | 标注 accepted / adapted / rejected / deferred，不让研究结论自动变成 supported |
| 风险抽取 | 补充确定性断言、金融建议、婚恋建议、亲属命运、职业建议等风险 |
| 验收样例 | 形成每个 topic 至少 2 个正向样例、1 个降级样例、1 个 forbidden-claim 样例 |

本流程已由 LOOP-092 完成。LOOP-093 已完成三项 implementation preflight 决策：统一 route、API 显式 `year`、`score_internal` 不进入默认公开 API。LOOP-094 已按该决策实现 M29 共享基础，并由 M30 首个 topic 验证合同可用。LOOP-095 已用同一合同完成 M31-M33，证明四专项均可在 restricted 边界内复用该基础。

## 4.2 Implementation Preflight Decisions

| Item | Decision | Regression Guard |
| --- | --- | --- |
| Route shape | 使用统一 route `GET /api/charts/topic-report`，通过 `topic` 参数区分 `relationship`、`wealth`、`family`、`career` | 后端测试必须覆盖 4 个 topic、非法 topic 和缺失 topic；前端 2 x 2 按钮只切换 topic |
| Annual trigger year | API 不隐式取当前年；`year` 必填。前端可以预填当前年，但必须显式发送并在响应中回显 `year_source` | 快照测试固定年份；缺失 year 返回可解释错误或降级，不允许 silent current-year fallback |
| Internal score | `score_internal` 不进入默认公开 API；默认输出 `signals`、`trace`、`qualitative_level` | 响应快照不得包含 `score_internal`；如未来公开需要另开决策和文案审计 |

## 5. Capability Status

| Capability | Current Status | Upper Bound | Notes |
| --- | --- | --- | --- |
| `relationship-report` | restricted | restricted | 情感专项命理推演；LOOP-094/M30 已实现 |
| `wealth-report` | restricted | restricted | 金钱专项命理推演；LOOP-095/M31 已实现 |
| `family-report` | restricted | restricted | 家庭专项命理推演；LOOP-095/M32 已实现 |
| `career-report` | restricted | restricted | 事业专项命理推演；LOOP-095/M33 已实现 |

M29 公共基础本身不新增独立用户能力；它是四个 topic capability 的共享基础。M30-M33 已将四个 topic 全部晋级为 restricted。

## 6. UI Contract

四个按钮占据主界面左下角现有留白，采用稳定 2 x 2 网格：

| Position | Button | Topic |
| --- | --- | --- |
| row 1 / col 1 | 情感推演 | `relationship` |
| row 1 / col 2 | 金钱推演 | `wealth` |
| row 2 / col 1 | 家庭推演 | `family` |
| row 2 / col 2 | 事业推演 | `career` |

点击按钮后，前端请求对应 topic report，并在报告区域显示与总命理报告一致的章节式内容。按钮不得在 API 未实现前显示为可用能力。

## 7. Report Contract

每份专项报告至少包含：

| Block | Purpose |
| --- | --- |
| 免责声明 | 明确传统参考、非专业建议、非确定性结论 |
| 推演依据 | 回显四柱、日主、十神、强弱、格局、用神、大运、流年口径 |
| 专业判断 | 使用十神、宫位、合冲刑害、透藏根气等术语 |
| 白话解释 | 每个专业判断后给出普通用户能理解的解释 |
| 引动提示 | 说明大运/流年是否触发相关结构，只给观察窗口，不给事件断言 |
| 不确定性 | 未知时辰、范围外日期、缺少性别或年份时降级说明 |

公开响应默认不得包含 `score_internal` 或 0-100 命运分；面向前端的强弱提示必须用 `signals`、`trace` 和 `qualitative_level` 表达。

## 8. Validation Gates

```powershell
cargo test --lib
cd frontend && npm run check
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

实现阶段必须新增：

- 后端 topic-report route/domain tests。
- 禁用词和高风险断言审计测试。
- 前端 2 x 2 按钮、topic 切换和报告渲染测试。
- unknown hour / missing sex / missing year 降级测试。
- 响应快照测试：统一 route 覆盖 4 个 topic；缺失或非法 `topic`/`year` 不得 silent fallback。
- 响应快照测试：默认公开 API 不包含 `score_internal` 或 0-100 命运分字段。

## 9. Closeout Requirements

- DG-011 已关闭为设计决策，并记录受限实现策略。
- `relationship-report`、`wealth-report`、`family-report`、`career-report` 均为 restricted；不得标为 supported。
- 模块树登记新增 restricted topic-report route、domain、UI surface，并标明四个 topic 均通过统一 route 可用。
- 风险台账登记专项推演的确定性伤害风险、流年过度承诺风险、UI overclaim 风险。
- closeout 写入 `97-loop-closeout-log.md`，cursor 指向 M34/下一阶段待定。
