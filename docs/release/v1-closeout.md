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

LOOP-133 已完成公开错误响应与首屏状态再收束：HTTP JSON 错误响应继续保留稳定 `error` code，但公开 `message` 统一为产品级中文提示，不再暴露参数名、route、capability、IO context 或内部异常文字；工作台首屏和大运兜底文案同步从“待生成 / 大运: error”改为“待整理 / 暂时没有整理完成”。新增后端与前端门禁防止后端变量、错误详情和机械状态词回流。能力状态不变，不新增 API、DTO、报告能力、score、存储或云同步能力。

LOOP-134 已完成静态兜底与前端异常文案产品化：桌面壳层静态资源错误 body 从 `Bad Request` / `Not Found` / `Method Not Allowed` 改为中文产品提示，前端 `ApiClient` 非 OK 异常不再携带 `404 Not Found` 这类 HTTP 术语；新增桌面单测与前端门禁防止英文机械错误、窗口形态和内部错误细节回流。能力状态不变，不新增 API、DTO、报告能力、score、存储或云同步能力。

LOOP-135 已完成用户封装包内容收束：本地 Windows 打包脚本和三平台 GitHub Actions 产物不再把 `v1-release-candidate.md`、`v1-closeout.md`、`desktop-packaging.md`、`current-product-boundary.md` 等治理文档放进普通用户包；封装包内 `README.md` 改由 `docs/release/user-package-readme.md` 生成，只保留启动、使用说明、能力边界和常见问题。新增 release 门禁防止治理文档、后端变量、raw/DTO/score 语境通过封装产物外露。能力状态不变，不新增 API、DTO、报告能力、score、存储或云同步能力。

LOOP-136 已完成已发布用户下载产物排空：仓库内 Windows 预览包从旧提交目录迁移到 `release-artifacts/desktop-windows/latest`，旧 zip 中携带的 `docs/release` 治理文档已从当前主分支产物移除；根 README 下载链接与 SHA256 同步指向 `latest`。`tools/check-product-surface.ps1` 现在会直接打开最新 zip，校验包内仅含桌面程序与用户 README，并阻断治理文档、后端变量、raw/DTO/score 语境通过实际下载包回流。能力状态不变，不新增窗口、API、DTO、报告能力、score、存储或云同步能力。

LOOP-137 已完成主盘报告工程词再排空：未知时辰说明从“前端或报告”改为“界面或报告”，主盘报告测试新增 `前端`、`后端`、`DTO`、`score_internal`、`error.message` 禁用词，防止工程语境进入用户可见正文。能力状态不变，不新增窗口、API、DTO、报告能力、score、存储或云同步能力。

LOOP-138 已完成报告页年份来源口吻收束：主盘报告页与专项报告页不再把后端 `year_source=explicit` 直译为“显式年份/显式年度”，统一改为“已选年份”“按当前选择”“页面当前选择”等用户可读表述；前端门禁新增禁用词防止该机器口吻回流。能力状态不变，不新增窗口、API、DTO、报告能力、score、存储或云同步能力。

LOOP-139 已完成报告壳层系统感词汇收束：主盘报告页与专项报告页将“只读卷宗”“专项卷宗”“等待归档”“报告已归档”“时间解释引擎”“投射到当前专题”等系统/工程口吻替换为“报告”“阅读进度”“等待整理”“报告已完成”“时间解读脉络”和专题读法说明；前端门禁与产品表层门禁同步阻断这些词回流。能力状态不变，不新增窗口、API、DTO、报告能力、score、存储或云同步能力。

LOOP-140 已完成专项报告 warning 与来源字段防泄露：专项报告页不再将 `warnings`、`signal.id`、`year_source` 等后端字段作为兜底文案直接展示；内部 warning 标识会先转成“时辰信息不完整”“部分大运或年度线索暂不完整”等用户可读提醒，未知英文或下划线标识统一压成通用谨慎阅读提醒。前端门禁新增 `unknown_hour_timeline_evidence_downgraded`、`topic_timeline_`、`annual_trigger_not_requested` 等禁用项。能力状态不变，不新增窗口、API、DTO、报告能力、score、存储或云同步能力。

LOOP-141 已完成工作台与主盘时间依据兜底再收束：五行/十神指标未知枚举不再回退显示原始 `m.id`，专项短栏的 `signal.label` / `qualitative_level` / `summary` 会先过滤未知英文或下划线标识；主盘报告时间依据不再出现“未收到观察年/引动年”“大运解释层未返回”等机器口吻，证据项中的未知后端字段会压成“命盘结构”“年度线索”或“以章节正文为准”的用户可读兜底。前端门禁同步阻断这些回退点。能力状态不变，不新增窗口、API、DTO、报告能力、score、存储或云同步能力。

LOOP-142 已完成用户说明与下载包 README 口吻再收束：根 README、用户包 README 模板和工作台边界说明从“生成命盘”“系统会”“指定年份”改为“排出命盘”“命轨会”“所选年份”等读者口吻；最新 Windows zip 已重新封装，包内 README 与根 README SHA 同步更新。产品表层门禁新增这些旧词，防止用户说明和封装包 README 回退到机械表达。能力状态不变，不新增窗口、API、DTO、报告能力、score、存储或云同步能力。

LOOP-143 已完成用户可见“评分/账号系统”口吻收束：专项报告时间导航不再出现“评分”字样，改为“不额外加入等级化结论或事件断语”；根 README、用户包 README 和工作台边界说明将“账号系统”收束为“账号功能”。前端与产品表层门禁同步阻断“评分”和“账号系统”回流。能力状态不变，不新增窗口、API、DTO、报告能力、score、存储或云同步能力。

LOOP-144 已完成报告正文与工作台分析卡片的分数化/机械词再收束：工作台“日主强弱”卡片不再展示“综合评分 x/10”，主盘免责声明从“自动生成”改为“整理而成”，未选择年份时不再写“不生成年度解释”，专项时间 trace 从“本追踪”改为“本段”。后端分析、主盘报告、专题报告门禁同步阻断“综合评分”“/10”“自动生成”“不生成年度解释”“本追踪”“评分”等回流。能力状态不变，不新增窗口、API、DTO、报告能力、score、存储或云同步能力。

LOOP-145 已完成完整专项报告结构线索总览兜底再收束：专项报告页的 `signal.label` / `qualitative_level` / `summary` 不再经 `safeText` 直接落入正文，未知英文或下划线标识会转为“结构线索”“观察”“暂无摘要”等用户可读兜底；阅读依据解释也会阻断未知英文标识直出。前端门禁同步锁住该路径。能力状态不变，不新增窗口、API、DTO、报告能力、score、存储或云同步能力。

LOOP-146 已完成用户功能边界措辞再收束：根 README、用户包 README 与工作台边界说明将“在线智能解读”改为“在线解读服务”，避免用户误读为 AI 或在线服务能力；桌面启动异常从“桌面窗口”改为“桌面界面”，减少窗口形态语义外露。前端门禁与产品表层门禁同步阻断“智能解读”回流。能力状态不变，不新增窗口、在线服务、API、DTO、报告能力、score、存储或云同步能力。

LOOP-147 已完成四专项报告版式回退：专项报告页不再按情感、金钱、家庭、事业切换专属 `data-report-theme`，统一使用主盘报告版式；四专项专属的双轨信笺、资产格、横向目录、树形时间线、作战看板、斜切卡片和正文分栏覆盖已移除。前端门禁同步要求专项报告固定 `main` 版式，并阻断横向目录、双列卡片、斜切卡片等旧样式回流。能力状态不变，不新增窗口、API、DTO、报告能力、score、存储或云同步能力。

LOOP-148 已完成统一报告版式的正文铺展修正：主盘与四专项共享的 `main` 报告块不再使用左右分栏和变体换边，正文从侧栏改为卡片内宽幅铺开，并以顶部边线承接标题区；前端门禁同步阻断 `grid-column`、正文侧栏、变体换边和正文分栏回流。能力状态不变，不新增窗口、API、DTO、报告能力、score、存储或云同步能力。

---
Closed: 2026-06-09
