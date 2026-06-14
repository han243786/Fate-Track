# V1 Closeout

封版标签：`v1.0.0-preview`

本文件记录 `v1.0.0-preview` 交付边界。用户已在 post-preview 阶段取消功能边界锁，并授权四个新功能切面进入规划边界；该授权不改变本文件记录的 V1 preview 运行时事实。

## Capability Matrix

| # | Capability | Status | Route |
|---|-----------|--------|-------|
| 1 | health | supported | GET /api/health |
| 2 | lunar-data-meta | supported | GET /api/lunar-data/meta |
| 3 | calendar-date-query | supported | GET /api/calendar/query?date= |
| 4 | calendar-date-query-v1-meta | supported | GET /api/calendar/query (meta field) |
| 5 | chart-basis-preview | restricted | GET /api/charts/basis/preview |
| 6 | chart-create | supported | GET /api/charts?date=&timezone= |
| 7 | chart-detail | supported | GET /api/charts/detail |
| 8 | analysis-snapshot | supported | GET /api/analysis/snapshot |
| 9 | luck-cycles | supported | GET /api/luck/cycles |
| 10 | case-management | restricted | GET /api/cases |
| 11 | share-preview | restricted | GET /api/share/preview |
| 12 | settings | restricted | GET /api/settings |
| 13 | glossary | supported | GET /api/glossary |
| 14 | case-export | restricted | GET /api/cases/export |
| 15 | data-derivation | restricted | GET /api/data/derive |
| 16 | astronomy-engine | supported | data/generated/astronomy/out/* (ADR 0021) |
| 17 | chart-report | restricted | GET /api/charts/report |

说明：上表对应后端 `/api/capabilities` 真源能力目录，共 17 项。`frontend-chart-workspace` 与 `frontend-share-preview` 属于受治理的前端表面，记录在 `docs/release/v1-release-candidate.md`，但不计入后端 capability catalog 数量。

## Decision Gates

All 10 DG-001 through DG-010 closed.

## Milestones

M0 through M28 closed. V1 preview capability matrix: 10 supported, 7 restricted, 0 target, 0 planned.

## Known Limitations

1. 农历输入不直接支持 (DG-004 closed)
2. 无数据库持久化 — 案例本地易失存储 (DG-006 closed)
3. 无真太阳时 (metadata declares unsupported)
4. 无 IANA 时区历史 (metadata declares not resolved)
5. 天文引擎不替换 Android 日期层 (DG-008 closed, replacement ADR pending)
6. 流年/流月不属于 V1 preview 能力目录。M29-M33 可登记四专项内部年度引动层，但不得宣称完整流年/流月系统。
7. 时辰未知时大运仍可排（仅使用年月日柱），该行为保留为显式边界。

## Validation

```
cargo test --lib:      87 passed, 0 failed
npm run check:         10 passed, 0 failed
cargo check desktop:   minggui-desktop OK
check-project.ps1:     Governance OK, Release OK, Astronomy OK
decision gates:        10/10 closed
```

## Next

Post-preview 已授权规划扩展：

- M29 four topic report foundation
- M30 relationship-report
- M31 wealth-report
- M32 family-report
- M33 career-report
- M34-M40 timeline reading roadmap: DG-012 closed by ADR 0022; M35 internal foundation implemented; M36 `luck-reading` is now restricted through `/api/charts/report?reading_year=YYYY`; M37 `annual-trigger-reading` is now restricted through `/api/charts/report?year=YYYY`; M38 `topic-timeline-reading` is now restricted through `/api/charts/topic-report?topic=...&year=YYYY`; M39 adds report UI/readability only with no capability change; M40 closes quality gates only with no capability change; LOOP-108 adds report-level visible-copy gates only with no capability change; LOOP-109 polishes `relationship-report` narrative structure only with no capability change; LOOP-110 polishes real relationship output copy only with no capability change; LOOP-111 polishes remaining main/wealth/family/career real report output copy only with no capability change; LOOP-112 polishes real relationship output a second time by removing repeated openers and quoting relation terms, with no capability change; LOOP-113 removes system/algorithm/score/count-table tone and major-luck label sticking from five real reports, with no capability change; LOOP-114 removes list/table/debug-like narration from five real reports, with no capability change; LOOP-115 turns relationship count summaries into the current golden-sample relationship prose, with no capability change; LOOP-116 turns wealth/family/career count-field summaries into topic-quality prose, with no capability change; LOOP-117 turns main annual-trigger and topic timeline bullet evidence into reading-order prose, with no capability change; LOOP-118 turns wealth/family/career explanation-of-template wording into topic advice-cohesion prose, with no capability change; LOOP-119 turns main chart report teaching/count wording into direct reading prose, with no capability change; LOOP-120 turns report closeout/order wording into continuous reading prose and keeps wealth/family/career ending at `结论`, with no capability change; LOOP-121 turns main five-element repetition and wealth/family/career shared timeline filler into grouped/topic-specific prose, with no capability change; LOOP-122 turns main annual-trigger and wealth/family/career topic timeline details into warmer annual-rhythm prose, with no capability change; LOOP-123 fixes current-luck consistency and splits dense annual detail into readable prose, with no capability change; LOOP-124 compresses repeated conclusions and gives wealth/family/career topic-specific closeouts, with no capability change; LOOP-125 de-stages main annual-trigger and topic timeline prose by removing staged `先露出` / `台前` / `不是罗列符号` wording, with no capability change; LOOP-126 condenses main chart long sections while retaining trace/evidence, with no capability change
- LOOP-113 / M45 continues the same quality-only line: five regenerated assembled report samples now gate against system-tone copy such as `当前算法`, `系统给出`, `综合评分`, `相关信号共`, `未见明显显性信号`, `共找到`, and `今年最值得留意`; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-114 / M46 continues the same quality-only line: five regenerated assembled report samples now gate against list-tone copy such as `盘中可用的时间线索`, `重点看的牵动`, `关键牵动是`, `出现 4 处`, `这张命盘里的`, and `结构上被点亮`; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-117 / M49 continues the same quality-only line: regenerated main/wealth/family/career output now gates against `主要牵动如下`, `盘面上先看这几股牵动`, `这些牵动只说明`, bullet evidence, and public score leakage while preserving M47/M48 report baselines; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-118 / M50 continues the same quality-only line: regenerated wealth/family/career output now gates against `日常读法`, `日常看`, `这些牵动提醒您`, `放回这张命盘看`, `放回家庭结构里`, and repeated reality-reminder wording while preserving M47/M48/M49 report baselines; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-119 / M51 continues the same quality-only line: regenerated main chart output now gates against `这一章看的是`, `这一章先把`, `放到日常理解里`, `最适合当作`, `可以先这样理解`, visible ten-god count summaries, and stale annual-trigger wording while preserving M47/M48/M49/M50 report baselines; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-120 / M52 continues the same quality-only line: regenerated main/wealth/family/career output now gates against `基本脉络如下`, `第一优先`, `第二优先`, `原局引动主要看`, `先看这几层关系`, `不能只看流年`, `不必急着找事件结论`, `这一章只说明`, `时间气候可以按这个顺序读`, `当前资料可以按完整四柱合参`, and `这一年`; wealth/family/career timeline chapters now sit before `结论`; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-122 / M54 continues the same quality-only line: regenerated main/wealth/family/career output now gates against `这一章会把`, `牵动会先落在这些位置`, `2026年的时间气候`, `先从这些层次落下去看`, `先看天干`, and `再看五行关系`; main annual-trigger and wealth/family/career topic timeline details now read as annual-rhythm prose; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-123 / M55 continues the same quality-only line: topic-report now uses real luck-cycle context and selected-year current luck instead of fixed `days_to_jie = 0`; regenerated samples gate against `大运首段`, fixed `1至10岁` wording, and stale annual-detail phrases; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-124 / M56 continues the same quality-only line: relationship conclusion repetition is compressed, wealth/family/career conclusions are made topic-specific, and regenerated samples gate against stale `在这份...专项里` closeout templates; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-125 / M57 continues the same quality-only line: main annual-trigger and wealth/family/career topic timeline prose now avoid staged `年度本身先露出的`, `推到台前`, and `不是罗列符号` wording; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-126 / M58 continues the same quality-only line: main `十神关系`, `大运走势`, and `年度引动` visible prose are condensed into reading summaries while restricted trace/evidence remains available; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-127 / M59 continues the same quality-only line: wealth/family/career middle chapters are rewritten from textbook definitions and colon labels into topic-specific reader-facing prose, and regenerated samples gate against stale phrases such as `财星分正财和偏财`, `传统上会把`, `官杀代表责任`, `技能表达：`, and `同辈边界：`; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-128 / M60 continues the same quality-only line: wealth/family/career `本专题的大运流年` chapters are rewritten from visible timeline scaffolding into direct 2026 topic rhythm, and regenerated samples gate against stale phrases such as `从「金钱」专项来看`, `从「家庭」专项来看`, `从「事业」专项来看`, `把2026年放进`, `十神与五行这一层`, and `本段把它作为阶段背景参考`; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.
- LOOP-129 / M61 adds age-context report polish for the 2025/2026 early-stage sample: topic reports now avoid adult romance, income, investment, workplace, job, or career-outcome framing and read emotional response, resource sense, stable care, learning tasks, rule sense, and expression training instead; no route, DTO, capability, score, or raw `luck-cycles` behavior changed.

四专项深度研究报告 RPT-004 已完成原文归档、中文规范副本、治理抽取、采纳矩阵、DG-011 设计关闭和验收样例。LOOP-093 已采纳 M29 implementation preflight 决策：使用统一 topic-report route、API 显式传入 `year`、默认公开响应不包含 `score_internal`。LOOP-094 已完成 M29/M30 post-preview 实现：`relationship-report` 进入 restricted。LOOP-095 已完成 M31-M33 post-preview 实现：`wealth-report`、`family-report`、`career-report` 进入 restricted。LOOP-097 将工作台专项栏收束为结构信号摘要，并把完整专项报告迁移到独立报告页。LOOP-098 已采纳大运/流年解释层优化方案并铺设 M34-M40。LOOP-099 已通过 ADR 0022 关闭 DG-012，并完成 M35 内部 `domain::timeline` foundation。LOOP-100 已完成 M36 主盘大运解释：`luck-reading` 进入 restricted，承载于 `/api/charts/report?reading_year=YYYY`；LOOP-101 已完成 M37 年度引动解释：`annual-trigger-reading` 进入 restricted，承载于 `/api/charts/report?year=YYYY`；LOOP-102 已完成 M38 四专项大运流年叠加：`topic-timeline-reading` 进入 restricted，承载于 `/api/charts/topic-report?topic=...&year=YYYY`；LOOP-103 已完成 M39 timeline report UI，只把既有 restricted 时间解释做成报告页导航、显式年份控件和可展开证据；LOOP-104 已完成 M40 quality gate closeout，只新增 public golden sample、forbidden/no-score/no-overclaim、bounded-output、frontend source boundary 和治理证据；LOOP-105 已完成前端质量修正，重新起盘会清空旧专项栏，报告页展示层会把内部英文标识转成中文；LOOP-106 已完成 timeline 词典文案质量门禁，拦截僵硬标签、年份病句和内部工程口径；LOOP-107 已完成大规模 timeline 词典优化，十神、五行、五行流向、地支关系、藏干、格局、用神均以组合式条目和自然解释守住质量门禁；LOOP-108 已完成报告级强约束，主盘报告和四专项完整报告的最终 API 正文都要通过可见文案门禁，避免僵硬标签、泛化年份、机器口吻、内部工程语和 public score 泄露；LOOP-109 已完成 M41 情感报告叙事打磨，`relationship-report` 正文固定为六块，开头只保留一次提醒，`topic-timeline-overlay` 证据仍在 signal/trace 中但不再单独成为情感正文块；LOOP-110 已完成 M42 情感真实输出再打磨，基于实际 `relationship-report` 样本清除标记、筛选、提取、等号计数和潜在英文口吻；LOOP-111 已完成 M43 剩余报告真实输出门禁，主盘、情感、金钱、家庭、事业五份 assembled report 样本均为 0 禁用命中、0 ASCII word，并保持内部 trace 与可见正文分离；LOOP-112 已完成 M44 情感真实输出二次门禁，解决固定开头复读，要求 `冲`、`合`、`刑害`、`六冲`、`自刑` 等关系术语在解释句和证据句中引号化；LOOP-113 已完成 M45 五份真实报告系统口吻清理，禁止 `当前算法`、`系统给出`、`综合评分`、`相关信号共`、`未见明显显性信号` 等口吻回到可见正文，并把大运标签统一为 `第一运·丙子` 这类自然读法；LOOP-114 已完成 M46 五份真实报告清单口吻叙事化，禁止 `盘中可用的时间线索`、`重点看的牵动`、`关键牵动是`、`出现 4 处`、`这张命盘里的`、`结构上被点亮` 等清单/台账/模板口吻回到可见正文；LOOP-115 已完成 M47 情感报告黄金样例基线，禁止 `relationship-report` 再把伴侣星、表达和安全感摘要写成 `不作主线` / `有一处落点` 等计数字段；LOOP-116 已完成 M48 三专项计数字段叙事基线，禁止 `wealth-report`、`family-report`、`career-report` 再把十神线索写成 `不作主线` / `有一处落点` / `参与这组结构` 等中间层字段；LOOP-117 已完成 M49 年度/大运流年叙事基线，禁止主盘年度引动和三专项大运流年再写成 `主要牵动如下` / `盘面上先看这几股牵动` / 项目符号证据；LOOP-118 已完成 M50 三专项解盘凝聚，禁止金钱、家庭、事业正文回到 `日常读法` / `日常看` / `这些牵动提醒您` / 说明书式重复提醒；LOOP-119 已完成 M51 主盘报告语气凝聚，禁止主盘正文回到 `这一章看的是` / `这一章先把` / `放到日常理解里` / `最适合当作` / `可以先这样理解` / 十神计数摘要 / `2026年可以按这个顺序读`；LOOP-120 已完成 M52 报告收束连续性，禁止主盘与三专项正文回到 `基本脉络如下` / `第一优先` / `第二优先` / `原局引动主要看` / `不能只看流年` / `当前资料可以按完整四柱合参` / `这一年`，并要求金钱、家庭、事业以专题结论收尾；LOOP-121 已完成 M53 报告密度与专题化，禁止主盘和三专项正文回到 `偏弱表示这类倾向` / `哪里需要放慢` / `哪里需要承接` / `读2026年这一层`，并要求主盘五行解释分组、三专项时间段贴合专题语境。raw `GET /api/luck/cycles` 不新增解释字段、score、年度引动或专题叠加。该 post-preview 变化不改变本文件记录的 V1 preview 冻结矩阵。

LOOP-122 已完成 M54 时间细节叙事暖化，禁止主盘年度引动和三专项大运流年回到 `这一章会把` / `牵动会先落在这些位置` / `2026年的时间气候` / `先从这些层次落下去看` / `先看天干` / `再看五行关系`，并要求年度细节改用年度节奏读法；能力状态不变，raw `GET /api/luck/cycles` 不新增解释字段、score、年度引动或专题叠加。

LOOP-123 已完成 M55 当前大运口径一致与年度段落拆解，禁止专题报告回到固定 `days_to_jie = 0` 起运口径、`大运首段`、`年龄段约为1至10岁`、`天干处先露出`、`月支这一处`、`日支这一处` 等旧表达；能力状态不变，raw `GET /api/luck/cycles` 不新增解释字段、score、年度引动或专题叠加。

LOOP-124 已完成 M56 报告结论去复读与切面个性化，禁止情感结论回到长段复述，禁止金钱、家庭、事业结论回到 `在这份...专项里`、`在同一张桌上慢慢理清` 等模板收束；能力状态不变，raw `GET /api/luck/cycles` 不新增解释字段、score、年度引动或专题叠加。

LOOP-125 已完成 M57 时间线正文去舞台化，禁止主盘年度引动和三专项大运流年回到 `年度本身先露出的`、`流年天干把十神主题推到台前`、`五行关系继续说明力量怎样靠近`、`不是罗列符号`、`推到台前` 等教学式口吻；能力状态不变。

LOOP-126 已完成 M58 主盘长段压缩，禁止主盘 `十神关系` 回到逐条十神小词典，禁止 `大运走势` / `年度引动` 回到专业句与白话句双层堆叠，禁止 `读这一章时`、`这条线已经进入命盘视野`、`这条十神线索`、`命理结构上，当前阶段大运` 等机器式解释；能力状态不变，raw `GET /api/luck/cycles` 不新增解释字段、score、年度引动或专题叠加。

LOOP-127 已完成 M59 三专项中段个性化，禁止金钱、家庭、事业中段回到 `财星分正财和偏财`、`传统上会把`、`印星在家庭专项里主要看`、`官杀代表责任`、`技能表达：`、`同辈边界：` 等术语教材和标签冒号口吻；能力状态不变，raw `GET /api/luck/cycles` 不新增解释字段、score、年度引动或专题叠加。

LOOP-128 已完成 M60 三专项大运流年读者口吻，禁止金钱、家庭、事业 `本专题的大运流年` 回到 `从「金钱」专项来看`、`从「家庭」专项来看`、`从「事业」专项来看`、`把2026年放进`、`十神与五行这一层`、`五行相处的方式提示`、`本段把它作为阶段背景参考` 等层级说明口吻；能力状态不变，raw `GET /api/luck/cycles` 不新增解释字段、score、年度引动或专题叠加。

LOOP-129 已完成 M61 年龄语境报告打磨，禁止 2025/2026 早年样本回到 `如果目前单身`、`若已有关系`、`工作场景`、`现实职位高低`、`长期经营`、`现实回报`、`团队边界` 等成人语境；能力状态不变，raw `GET /api/luck/cycles` 不新增解释字段、score、年度引动或专题叠加。

LOOP-130 已完成五行视觉主题质量收束：金、木、水、火、土五套主题统一改为显式 HSL 色相、饱和度、明度 token，工作台与主盘/四专项报告页共用同一视觉方向；新增前端门禁防止五行主题退回单色换皮、旧 HEX 色盘或能力扩展。能力状态不变，不新增 API、DTO、报告内容、score 或时间解释能力。

LOOP-131 已完成用户可见产品边界说明：工作台边界提示新增只读原地展开面板，列出当前可查看能力与当前不做事项，并补充前端门禁防止该说明接入 API、使用窗口形态或变成功能扩展入口。能力状态不变，不新增 API、DTO、报告内容、score、时间解释能力或存储/云同步能力。

LOOP-132 已完成用户可见机械口吻与后端错误泄露收束：命盘报告页、专项报告页和工作台专题栏统一把“生成/信号/证据/失败”等工程口径收束为“整理/线索/依据/暂未整理”，前端错误状态不再把后端 `error.message` 直接展示给用户；新增门禁防止窗口形态、AI 式机械语言、后端错误详情和内部变量口吻回流。能力状态不变，不新增 API、DTO、报告能力、score、存储或云同步能力。

---
Closed: 2026-06-09
