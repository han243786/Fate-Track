# Post-Preview Documentation Freeze

## 1. Status

`v1.0.0-preview` 已发布到 GitHub。当前封版状态以以下文件为准：

- `README.md`
- `docs/release/v1-release-candidate.md`
- `docs/release/v1-closeout.md`
- `markdown/20-roadmap/00-roadmap-index.md`
- `markdown/20-roadmap/93-capability-promotion-ledger.md`
- `markdown/20-roadmap/96-recursive-cursor.md`
- `markdown/20-roadmap/97-loop-closeout-log.md`

M0-M28 已关闭。V1 preview 运行时能力矩阵为 10 supported、7 restricted、0 target、0 planned。M25-M28 是视觉、内容、报告门户和桌面壳封装，不新增能力状态。

## 2. Boundary Lock Superseded For Four Planned Slices

本文件原本用于锁定 `v1.0.0-preview` 后的功能边界。用户随后明确取消功能边界锁，并授权四个功能切面进入 post-preview 规划边界：情感、金钱、家庭、事业。

因此，以下 freeze 规则继续保护 V1 preview 运行时事实，但不再阻止 M29-M33 的治理铺设。M29-M33 仍必须遵守：先研究报告 intake，后 DG-011，后实现；实现完成前不得宣称可用。LOOP-098 后，M34-M40 timeline reading 进入 roadmap；LOOP-099 关闭 DG-012 并实现 M35 内部 foundation；LOOP-100 完成 M36 `luck-reading` restricted 暴露；LOOP-101 完成 M37 `annual-trigger-reading` restricted 暴露；LOOP-102 完成 M38 `topic-timeline-reading` restricted 暴露；LOOP-103 完成 M39 UI/readability 收口但不新增 capability；LOOP-104 完成 M40 quality gate closeout，仅新增回归门禁和治理证据；LOOP-105 完成前端质量修正，仅修复 stale topic panel 与可见英文标识；LOOP-106 完成 timeline 词典文案质量门禁；LOOP-107 完成大规模 timeline 词典优化，仅扩展组合式词典与自然中文质量门禁；LOOP-108 完成报告级强约束，仅把主盘与四专项最终报告正文纳入 public response 文案门禁；LOOP-109 完成 M41 情感专项报告叙事打磨，仅重排 `relationship-report` 六块正文并收束提示位置；LOOP-110 完成 M42 情感真实输出再打磨，仅新增 `relationship-report` 文案门禁；LOOP-111 完成 M43 剩余报告真实输出再打磨，仅新增主盘、金钱、家庭、事业 assembled report 文案门禁；LOOP-112 完成 M44 情感真实输出二次门禁，仅解决固定开头复读和合冲刑害术语引号化；LOOP-113 完成 M45 五份真实报告系统口吻清理，仅压下算法、系统、评分、计数表口吻和大运标签粘连；LOOP-114 完成 M46 五份真实报告清单口吻叙事化，仅压下锚点清单、时间线统计、`出现几处` 台账和模板化双冒号；LOOP-115 完成 M47 情感报告黄金样例基线，仅压下 `relationship-report` 的 `不作主线` / `有一处落点` 等计数字段；LOOP-116 完成 M48 三专项计数字段叙事基线，仅压下 `wealth-report`、`family-report`、`career-report` 的落点计数和中间层承接；LOOP-117 完成 M49 年度/大运流年叙事基线，仅压下主盘年度引动与三专项大运流年的清单证据口吻；LOOP-118 完成 M50 三专项解盘凝聚，仅压下金钱、家庭、事业的说明书口吻和重复现实提醒；LOOP-119 完成 M51 主盘报告语气凝聚，仅压下主盘报告说明书桥接、十神计数摘要和年度引动僵硬句式；LOOP-120 完成 M52 报告收束连续性，仅压下主盘旧列表/排序口吻、三专项时间章节尾随结论和泛化结论句式；LOOP-121 完成 M53 报告密度与专题化；LOOP-122 完成 M54 时间细节叙事暖化；LOOP-123 完成 M55 当前大运口径一致与年度段落拆解；LOOP-124 完成 M56 报告结论去复读与切面个性化；LOOP-125 完成 M57 时间线正文去舞台化；LOOP-126 完成 M58 主盘长段压缩，均不改变本文件记录的 V1 preview 冻结矩阵。

## 2.1 Original Boundary Lock Record

当前功能边界锁死：

- 不新增 public API。
- 不新增前端业务功能。
- 不新增 capability。
- 不把 restricted 自动晋级 supported。
- 不替换 Android 日期层运行时默认。
- 不把即将提出的四个功能切面写入实现。

允许的工作仅限：

- 文档同步。
- 旧口径纠偏。
- 门禁脚本文案与封版状态对齐。
- 缺陷修复或回归保护，但必须另开递归游标并登记 closeout。

## 3. Historical Evidence Rule

M9-M10 文档中存在大量“`astronomy-engine` remains target”“no generated astronomy table has been accepted yet”等历史阶段语句。这些语句是当时 preflight/materialization 阶段的防过度宣称证据，不应被直接删除。

当前状态解释顺序如下：

1. 历史里程碑文件保留当轮事实。
2. M23 `83-milestone-23-astronomy-engine-promotion.md` 和 ADR 0021 记录最终晋级。
3. release 文档、capability ledger 和 README 记录当前封版状态。
4. 若历史句子与封版状态表面冲突，以“历史阶段事实 + 当前封版覆盖层”解释。

## 4. Four-Slice Intake Rule

用户已提供四个功能切面方向，并授权纳入边界：情感、金钱、家庭、事业。RPT-004 深度研究报告已完成原文归档、中文规范副本、治理抽取、采纳矩阵、DG-011 设计关闭和验收样例。LOOP-093 已采纳 M29 implementation preflight 决策：统一 route、API 显式 `year`、`score_internal` 不进入默认公开 API。LOOP-094 已完成 M29/M30：`relationship-report` 进入 restricted。LOOP-095 已完成 M31-M33：`wealth-report`、`family-report`、`career-report` 进入 restricted。LOOP-097 将工作台专项栏收束为结构信号摘要，并把完整专项报告迁移到独立报告页。当前 post-preview 四专项均为 restricted；V1 preview 冻结矩阵仍保持 10 supported、7 restricted。

四个切面进入代码前，必须先做文档 intake：

1. 为每个切面登记目标、非目标、能力状态上限、风险和验收证据。
2. 判断它们是新能力、restricted 晋级、现有能力增强，还是纯体验/性能/治理工作。
3. 更新 roadmap、module tree、capability ledger、risk register 和 decision gates。
4. 跑完整质量门禁。
5. closeout 通过后，才允许进入代码递归。

深度研究报告回来后，必须额外完成，LOOP-092 已完成：

1. 原文归档到 `markdown/reserch`。
2. 中文翻译到 `markdown/reserch/zh-CN`。
3. 抽取十神、宫位、运年引动、合冲刑害、透藏根气规则。
4. 标注 accepted / adapted / rejected / deferred。
5. 更新 DG-011、风险台账、能力台账和 M29 验收样例。

## 4.1 Timeline Reading Intake Rule

用户已采纳大运/流年重型优化方案，并要求先铺设里程碑。LOOP-098 已将该方向拆成 M34-M40：

- M34 timeline reading governance and DG-012
- M35 timeline lexicon and rule engine
- M36 primary chart luck reading
- M37 annual trigger reading
- M38 topic timeline overlay
- M39 timeline report UI
- M40 timeline quality gate and closeout

DG-012 已由 ADR 0022 关闭，M35 已完成内部 `domain::timeline` foundation。M36 已完成主盘 `luck-reading` restricted 暴露。M37 已完成年度 `annual-trigger-reading` restricted 暴露。M38 已完成四专项 `topic-timeline-reading` restricted 暴露。M39 已完成报告页时间导航、显式年份控件、可展开证据和工作台短摘要。M40 已由 LOOP-104 收口为质量门禁和治理证据：public response golden samples、forbidden/no-overclaim/no-public-score、bounded-output、frontend UI source boundary 和 browser verification。LOOP-105 已完成前端质量修正：重新起盘清空旧专项栏，报告页展示层把内部英文标识转中文。LOOP-106 已完成 timeline 词典文案质量门禁。LOOP-107 已完成大规模 timeline 词典优化：后续词典扩展必须保持组合式条目、自然解释、具体信号名、直接面向“您”、足够解释密度，并避免硬标签、泛化年份、病句、`用户` 口吻、后端/前端说明和内部引擎名进入自然解释。LOOP-108 已完成报告级强约束：后续主盘报告和四专项完整报告正文变更必须通过最终 API response 门禁，避免僵硬标签、泛化年份、机器口吻、内部工程语、public score 和高风险断言进入可见文本。LOOP-109 已完成 M41 情感专项单切面打磨：情感完整报告正文只保留六块，允许低风险关系节奏建议，但禁止确定性婚恋事件、伴侣身份和高风险决策断言。LOOP-110 已完成 M42 情感真实输出再打磨：后续情感报告正文还必须避免标记/筛选/提取、等号计数和潜在英文口吻。LOOP-111 已完成 M43 剩余报告真实输出再打磨：后续主盘、金钱、家庭、事业正文还必须避免内部英文、pipe-form 证据、等号证据、机器口吻、半角年龄段和可见后端变量。LOOP-112 已完成 M44 情感真实输出二次门禁：后续情感报告正文还必须避免固定开头复读，并保持 `被"冲"牵动`、`形成"六冲"` 这类引号化术语表达。LOOP-113 已完成 M45 五份真实报告系统口吻清理：后续主盘和四专项正文还必须避免算法、系统、评分、计数表口吻和大运标签粘连。LOOP-114 已完成 M46 五份真实报告清单口吻叙事化：后续主盘和四专项正文还必须避免锚点清单、时间线统计、`出现几处` 台账、模板化双冒号和 `这张命盘里的` 这类机器化承接。LOOP-115 已完成 M47 情感报告黄金样例基线：后续情感报告正文还必须避免 `不作主线`、`有一处落点` 等计数字段回到用户可见正文。LOOP-116 已完成 M48 三专项计数字段叙事基线：后续金钱、家庭、事业正文还必须避免 `不作主线`、`有一处落点`、`有两处落点`、`有三处落点`、`参与这组结构`、`这组结构说明` 等字段回到用户可见正文。LOOP-117 已完成 M49 年度/大运流年叙事基线：后续主盘年度引动和专题大运流年正文还必须避免 `主要牵动如下`、`盘面上先看这几股牵动`、`这些牵动只说明` 和项目符号证据回到用户可见正文。LOOP-118 已完成 M50 三专项解盘凝聚：后续金钱、家庭、事业正文还必须避免 `日常读法`、`日常看`、`这些牵动提醒您`、`放回这张命盘看`、`放回家庭结构里` 和 `这份报告适合当作` 回到用户可见正文。LOOP-119 已完成 M51 主盘报告语气凝聚：后续主盘正文还必须避免 `这一章看的是`、`这一章先把`、`放到日常理解里`、`最适合当作`、`可以先这样理解`、十神计数摘要和 `2026年可以按这个顺序读` 回到用户可见正文。LOOP-120 已完成 M52 报告收束连续性：后续主盘和专题正文还必须避免 `基本脉络如下`、`第一优先`、`第二优先`、`原局引动主要看`、`先看这几层关系`、`不能只看流年`、`不必急着找事件结论`、`这一章只说明`、`时间气候可以按这个顺序读`、`当前资料可以按完整四柱合参`、`这一年` 回到用户可见正文；金钱、家庭、事业必须保持大运流年章节在 `结论` 前，并以专题结论收尾。LOOP-121 已完成 M53 报告密度与专题化：后续主盘和专题正文还必须避免 `偏弱表示这类倾向`、`哪里需要放慢`、`哪里需要承接`、`读2026年这一层` 回到用户可见正文；主盘五行解释必须保持分组读法，金钱、家庭、事业大运流年必须保持专题化 `落到2026年` 表达。以下规则继续作为后续扩展锁：

LOOP-122 已完成 M54 时间细节叙事暖化：后续主盘年度引动和三专项大运流年正文还必须避免 `这一章会把`、`牵动会先落在这些位置`、`2026年的时间气候`、`先从这些层次落下去看`、`先看天干`、`再看五行关系` 回到用户可见正文；主盘年度引动必须保持 `2026年靠近命盘时` 的年度节奏读法，金钱、家庭、事业大运流年必须保持 `把2026年放进...专项来看` 的专题节奏表达。

LOOP-123 已完成 M55 当前大运口径一致与年度段落拆解：后续专题报告必须使用真实起运上下文和选定年份当前大运，不得回到固定 `days_to_jie = 0`、`大运首段`、`年龄段约为1至10岁`、`约在 1 至 10 岁`、`天干处先露出`、`月支这一处`、`日支这一处` 等旧口径；主盘年度引动和三专项大运流年必须保持年度证据分段叙事，不改变 route、DTO、capability、score 或 raw `luck-cycles` 行为。

LOOP-124 已完成 M56 报告结论去复读与切面个性化：后续情感报告必须保持六块黄金样例且结论不得回到长段复述；金钱、家庭、事业结论必须保持贴题收束，不得回到 `在这份金钱专项里`、`在这份家庭专项里`、`在这份事业专项里`、`表达与安全感则落在日常相处里`、`以目前资料来看，这份情感专项可以把重点放在`、`在同一张桌上慢慢理清` 等模板收束；不改变 route、DTO、capability、score 或 raw `luck-cycles` 行为。

LOOP-125 已完成 M57 时间线正文去舞台化：后续主盘年度引动和三专项大运流年必须保持直接读盘口吻，不得回到 `年度本身先露出的`、`流年天干把十神主题推到台前`、`五行关系继续说明力量怎样靠近`、`不是罗列符号`、`推到台前`、`走到台前`、`拿到台前` 等教学式舞台口吻；不改变 route、DTO、capability、score 或 raw `luck-cycles` 行为。

LOOP-126 已完成 M58 主盘长段压缩：后续主盘 `十神关系`、`大运走势`、`年度引动` 必须保持可读密度，不得回到逐条十神小词典、专业句与白话句双层堆叠、`读这一章时`、`这条线已经进入命盘视野`、`这条十神线索`、`命理结构上，当前阶段大运`、`五行流向上，`、`藏干里，`、`地支关系上，` 等机器式解释；不改变 route、DTO、capability、score、trace/evidence 或 raw `luck-cycles` 行为。

LOOP-127 已完成 M59 三专项中段个性化：后续金钱、家庭、事业中段必须保持贴题读盘口吻，不得回到 `财星分正财和偏财`、`传统上会把`、`印星在家庭专项里主要看`、`比劫在家庭专项里看`、`财官在家庭专项里不解释`、`官杀代表责任`、`食伤代表表达`、`比劫代表协作`、`技能表达：`、`资源落地：`、`协作竞争：`、`同辈边界：`、`表达方式：`、`责任方面，`、`承接方面，`、`支持与约束方面` 等术语教材和标签冒号口吻；不改变 route、DTO、capability、score、trace/evidence 或 raw `luck-cycles` 行为。

LOOP-128 已完成 M60 三专项大运流年读者口吻：后续金钱、家庭、事业 `本专题的大运流年` 必须保持直接读 2026 年专题节奏，不得回到 `从「金钱」专项来看`、`从「家庭」专项来看`、`从「事业」专项来看`、`把2026年放进`、`十神与五行这一层`、`五行相处的方式提示`、`藏干、原局位置和当前大运合到一起时`、`藏干、宫位关系和当前大运合到一起时`、`本段把它作为阶段背景参考`、`这里看的不是单点事件` 或 `年度线索要回到` 等层级说明口吻；不改变 route、DTO、capability、score、trace/evidence 或 raw `luck-cycles` 行为。

1. `GET /api/luck/cycles` 保持 raw supported calculation，不承载解释文本、score 或 topic overlay。
2. 年度引动 API 必须显式传入 `year`，前端默认当前年也必须随请求传递；M37 已用 `not_requested` 状态保护缺省 `year`。
3. `score_internal`、0-100 运势分、排序分不得公开。
4. 不宣称完整流月、流日、择日、每日推送或事件预测。
5. `luck-reading`、`annual-trigger-reading`、`topic-timeline-reading` 的上限为 restricted，不能在 M34-M40 内顺手晋级 supported。

## 5. Next Cursor

下一轮默认不是直接实现，而是：

```text
LOOP-128 single_loop
scope: M60 topic timeline reader-facing polish closed as quality-only; M34-M40 timeline slice remains stable at 10 supported, 14 restricted, 0 planned. Next scope must be user-selected and should start from regenerated real samples or governance intake, not silent timeline capability expansion.
forbidden: supported promotion without new ADR, raw luck-cycles mutation, public score, silent year default, full flow-month/day/event prediction, deterministic romance/finance/family/career claims, or high-risk topic advice
```
