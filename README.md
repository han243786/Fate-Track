# 命轨 · Fate Track

命轨是一套基于传统命理学的排盘与结构化分析工具，采用 Rust 后端 + 原生 JavaScript 前端的单体架构。项目的设计目标是：**只读排盘工作台，固定模板解释，不做生成式推断。**

---

## 快速开始

**方式一：桌面应用（推荐）**

已封装的 Windows 预览版可直接下载：

- [Fate-Track-Windows-x64.zip](https://github.com/han243786/Fate-Track/blob/main/release-artifacts/desktop-windows/63cb0cb/Fate-Track-Windows-x64.zip)
- [SHA256SUMS.txt](https://github.com/han243786/Fate-Track/blob/main/release-artifacts/desktop-windows/63cb0cb/SHA256SUMS.txt)
- [BUILD-MANIFEST.md](https://github.com/han243786/Fate-Track/blob/main/release-artifacts/desktop-windows/63cb0cb/BUILD-MANIFEST.md)

SHA256：

```text
0027647628d3614a93f861ff1babc43ab9f3412f58cd86f8ba4d350e05a3a766  Fate-Track-Windows-x64.zip
```

本地开发运行：

```powershell
cargo run -p minggui-desktop
```

启动后自动打开原生窗口，无需 Node、无需手动启动后端。

**方式二：开发模式**

```powershell
# 终端 1：启动后端
cargo run -p minggui-backend

# 终端 2：启动前端
cd frontend
node server.mjs
```

浏览器打开 `http://127.0.0.1:5173`。

---

## 发布

Windows 桌面预览版已随源码推送到 `release-artifacts/desktop-windows/63cb0cb/`。如果需要手动挂到 GitHub Release：

1. 打开 [Releases](https://github.com/han243786/Fate-Track/releases)
2. 选择或创建 `v1.0.0-preview`
3. Release title 使用 `Fate Track v1.0.0 Preview`
4. 上传 `dist/desktop-windows/Fate-Track-Windows-x64.zip`
5. 上传 `dist/desktop-windows/SHA256SUMS.txt`
6. 勾选 `Set as a pre-release`
7. 发布后入口为 [v1.0.0-preview](https://github.com/han243786/Fate-Track/releases/tag/v1.0.0-preview)

本地重新封装命令：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\package-desktop-windows.ps1
```

该脚本会先跑严格 Clippy、完整 Rust 测试、前端检查、治理门禁和 release build，再生成 Windows zip 与 SHA256。

---

## 能力边界

| 状态 | 数量 | 能力 |
|---|---|---|
| 已支持 | 10 | 健康检查、农历数据元信息、公历日期查询、日期层元数据、四柱排盘、命盘快照、结构化分析、大运排盘、术语查询、天文引擎 |
| 受限 | 14 | 排盘基础预览、案例管理、分享预览、用户设置、案例导出、数据衍生、排盘报告、大运解释、年度引动解释、四专项大运流年叠加、情感专项推演、金钱专项推演、家庭专项推演、事业专项推演 |
| 规划中 | 0 | — |

M0-M28 已全部关闭并发布 `v1.0.0-preview`。Post-preview M29-M33 已完成 topic-report 基础与情感、金钱、家庭、事业四专项 restricted 实现；LOOP-097 后，工作台专项栏仅展示结构信号，完整专项报告通过独立报告页阅读。LOOP-098 已铺设 M34-M40 大运/流年解释层里程碑；LOOP-099 已关闭 DG-012 并完成 M35 内部 timeline foundation；LOOP-100 完成 M36 主盘大运解释，`luck-reading` 通过 `/api/charts/report?reading_year=YYYY` 作为 restricted 能力承载；LOOP-101 完成 M37 年度引动解释，`annual-trigger-reading` 通过 `/api/charts/report?year=YYYY` 作为 restricted 能力承载；LOOP-102 完成 M38 四专项大运流年叠加，`topic-timeline-reading` 通过 `/api/charts/topic-report?topic=...&year=YYYY` 作为 restricted 能力承载；LOOP-103 完成 M39 timeline report UI；LOOP-104 完成 M40 timeline quality gate closeout，新增 golden samples、禁用词、no score、no overclaim、bounded-output 和浏览器样例证据；LOOP-105 完成前端质量修正：重新起盘会清空旧专项栏，报告页可见英文内部标识已中文化；LOOP-106 新增 timeline 词典文案质量门禁，拦截机器化标签、年份病句和内部工程口径；LOOP-107 完成大规模 timeline 词典优化，将词典扩到 28 个组合式条目并强化生成文案密度与词典原文门禁；LOOP-108 完成报告级强约束，把主盘报告与四专项完整报告的最终 API 正文纳入可见文案门禁，拦截僵硬标签、泛化年份、机器口吻、内部工程语和 public score 泄露；LOOP-109 完成 M41 情感专项报告叙事打磨，将 `relationship-report` 正文收束为总断、伴侣议题、夫妻宫、表达/边界/安全感、年度情感引动、结论六块，并把情感时间叠加合入年度章；LOOP-110 完成 M42 情感真实输出再打磨，基于实际样本清除标记、筛选、提取、等号计数和潜在英文口吻；LOOP-111 完成 M43 剩余报告真实输出文案门禁，主盘、情感、金钱、家庭、事业五份 assembled report 样本均为 0 禁用命中、0 ASCII word；LOOP-112 完成 M44 情感专项真实输出二次门禁，解决固定开头复读，并将 `冲`、`合`、`刑害`、`六冲`、`自刑` 等术语在解释句和证据句中引号化；LOOP-113 完成 M45 五份真实报告系统口吻清理，压下算法、系统、评分和计数表口吻，并修正大运标签粘连；LOOP-114 完成 M46 五份真实报告清单口吻叙事化，压下年度引动锚点清单、专题时间线统计和“出现几处”台账；LOOP-115 完成 M47 情感报告黄金样例基线，情感伴侣星、表达和安全感摘要不再露出 `不作主线` / `有一处落点` 等计数字段；LOOP-116 完成 M48 三专项计数字段叙事基线，金钱、家庭、事业将十神落点字段改为专题气质解释；LOOP-117 完成 M49 年度/大运流年叙事基线，主盘年度引动与三专项大运流年从清单证据改为连贯读盘顺序；LOOP-118 完成 M50 三专项解盘凝聚，金钱、家庭、事业从说明书口吻改为总断、专题入口和关键词结论；LOOP-119 完成 M51 主盘报告语气凝聚，主盘章节从教学说明改为读盘正文，十神摘要从计数台账改为结构信号；LOOP-120 完成 M52 报告收束连续性，主盘旧列表/排序口吻继续压实，金钱、家庭、事业的大运流年置于结论前并以专题结论收尾；LOOP-121 完成 M53 报告密度与专题化，主盘五行解释合并为分组读法，三专项时间段改成 `落到2026年` 的专题建议；LOOP-122 完成 M54 时间细节叙事暖化，主盘年度引动和三专项大运流年从层次清单进一步改为年度节奏读法；LOOP-123 完成 M55 当前大运口径一致与年度段落拆解，专题报告改用真实起运上下文并按选定年份取当前大运；LOOP-124 完成 M56 报告结论去复读与切面个性化，情感结论压缩复述，金钱/家庭/事业结论改成贴题收束；LOOP-125 完成 M57 时间线正文去舞台化，主盘年度引动与三专项大运流年压下 `年度本身先露出的`、`推到台前`、`不是罗列符号` 等教学式句子；LOOP-126 完成 M58 主盘长段压缩，主盘十神、大运、年度引动保留 trace/evidence 但可见正文压成读盘摘要。以上均不改变能力状态。raw `GET /api/luck/cycles` 仍保持纯大运排盘。详见[能力晋级台账](markdown/20-roadmap/93-capability-promotion-ledger.md)。

LOOP-113 已完成 M45 五份真实报告系统口吻清理：主盘不再向用户展示 `当前算法`、`系统给出`、`综合评分` 等说明口吻，四专项不再展示 `相关信号共`、`未见明显显性信号`、`共找到`、`今年最值得留意` 等机器化摘要，并把大运标签统一为 `第一运·丙子` 这类可读形式；能力状态不变。
LOOP-114 已完成 M46 五份真实报告清单口吻叙事化：主盘年度引动改为阅读顺序，四专项时间叠加不再展示十条/五条统计，并压下“出现几处”台账；能力状态不变。
LOOP-117 已完成 M49 年度/大运流年叙事基线：主盘年度引动与三专项大运流年已从项目符号清单转为“按顺序读”的连贯段落，不再向用户露出 `主要牵动如下`、`盘面上先看这几股牵动` 这类清单口吻；能力状态不变。
LOOP-118 已完成 M50 三专项解盘凝聚：金钱、家庭、事业报告新增 `资源入口`、`互动位置`、`事业用力方式` 等解盘式章节，并把 `日常读法`、`日常看`、`这些牵动提醒您` 等说明书口吻纳入门禁；能力状态不变。
LOOP-119 已完成 M51 主盘报告语气凝聚：主盘命盘概览、日主、五行、十神、大运和年度引动章节从教学说明改为读盘正文，并把 `这一章看的是`、`放到日常理解里`、`比肩一处` 等口吻纳入门禁；能力状态不变。
LOOP-120 已完成 M52 报告收束连续性：主盘继续压下 `基本脉络如下`、`第一优先`、`原局引动主要看` 等旧列表/排序口吻；金钱、家庭、事业报告的大运流年章节前置到 `结论` 之前，三专项最终可见章节均以专题结论收尾；能力状态不变。
LOOP-122 已完成 M54 时间细节叙事暖化：主盘年度引动改成 `2026年靠近命盘时` 的年度节奏读法，三专项大运流年改成 `把2026年放进...专项来看` 的专题节奏说明，并把 `这一章会把`、`牵动会先落在这些位置`、`2026年的时间气候`、`先从这些层次落下去看`、`先看天干`、`再看五行关系` 纳入门禁；能力状态不变。
LOOP-123 已完成 M55 当前大运口径一致与年度段落拆解：topic-report 不再用固定 `days_to_jie = 0` 计算大运，情感、金钱、家庭、事业均按选定年份取 `当前大运`，并把 `大运首段`、`年龄段约为1至10岁`、`天干处先露出` 等旧口径纳入门禁；能力状态不变。

LOOP-124 已完成 M56 报告结论去复读与切面个性化：情感报告保持六块黄金样例但压缩结论复述，金钱、家庭、事业结论分别围绕预算/产出/资源流动、支持/表达/边界/责任、压力承接/技能交付/协作边界收束，并把 `在这份...专项里`、`表达与安全感则落在日常相处里`、`在同一张桌上慢慢理清` 等模板句纳入门禁；能力状态不变。

LOOP-125 已完成 M57 时间线正文去舞台化：主盘年度引动与金钱、家庭、事业大运流年保留同一批命理证据，但不再回到 `年度本身先露出的`、`流年天干把十神主题推到台前`、`五行关系继续说明力量怎样靠近`、`不是罗列符号`、`推到台前` 等教学式舞台口吻；能力状态不变。

LOOP-126 已完成 M58 主盘长段压缩：主盘 `十神关系`、`大运走势`、`年度引动` 保留 restricted trace/evidence，但可见正文从教材式长段压成读盘摘要，并把 `读这一章时`、`这条线已经进入命盘视野`、`这条十神线索`、`命理结构上，当前阶段大运` 等回退口吻纳入门禁；能力状态不变。

LOOP-127 已完成 M59 三专项中段个性化：金钱、家庭、事业中段从术语定义和标签冒号改为贴题读盘，并把 `财星分正财和偏财`、`传统上会把`、`官杀代表责任`、`技能表达：`、`同辈边界：` 等回退口吻纳入门禁；能力状态不变。

LOOP-128 已完成 M60 三专项大运流年读者口吻：金钱、家庭、事业 `本专题的大运流年` 从 `从「...」专项来看`、`十神与五行这一层`、`藏干...合到一起时` 等层级说明，改成直接围绕 2026 年的资源节奏、家庭安顿和事业承接展开；能力状态不变。

LOOP-129 已完成 M61 年龄语境报告打磨：当样本为 2025 出生、2026 观察的早年阶段时，情感、金钱、家庭、事业专项不再按成人恋爱、收入、投资、岗位或职业结果读法呈现，改为情绪回应、资源感、稳定照护、学习任务、规则感和表达训练；能力状态不变。

---

## 接口总览

```
GET /api/health                                    已支持
GET /api/capabilities                              已支持
GET /api/lunar-data/meta                           已支持
GET /api/calendar/query?date=                      已支持    （Android 日期层，1901-2100）
GET /api/charts/basis/preview?date=&timezone=      受限
GET /api/charts?date=&timezone=&time_precision=    已支持    （四柱排盘）
GET /api/charts/detail?date=&timezone=             已支持    （不可变快照）
GET /api/analysis/snapshot?date=&timezone=         已支持    （结构化指标 + 深层分析）
GET /api/luck/cycles?date=&timezone=&sex=          已支持    （大运排盘）
GET /api/glossary?term=&category=                  已支持    （55 条术语）
GET /api/cases?action=                             受限      （本地易失存储）
GET /api/cases/export?id=                          受限
GET /api/settings?action=                          受限      （本地易失偏好）
GET /api/share/preview?action=                     受限      （脱敏分享）
GET /api/data/derive?type=                         受限      （≥5 条阈值）
GET /api/charts/report?date=&timezone=&reading_year=&year= 受限 （口语化中文报告 + 大运解释 + 年度引动）
GET /api/charts/topic-report?topic=relationship|wealth|family|career&year=  受限  （四专项推演 + 大运流年叠加）
```

日期层基于 Android 万年历端口，验证范围为 1901-2100。天文引擎作为独立已支持能力存在，不替代 Android 日期层为运行时默认。

---

## 验证

```powershell
cargo clippy --all-targets -- -D warnings
cargo test                         # 后端 118 项 + 集成验证
npm.cmd run check --prefix frontend # 前端 20 项
powershell -File tools/check-project.ps1   # 全项目门禁
```

---

## 项目结构

```
FT/
  backend/           Rust API 服务、领域模型、天文引擎、日历计算
  frontend/          JS 浏览器 UI、API 客户端、状态管理、渲染模块
  data/raw/          农历基础数据
  data/generated/    天文引擎生成数据
  docs/decisions/    架构决策记录
  docs/release/      发布候选与交付边界
  markdown/          治理文档、产品树、里程碑路线图
  release-artifacts/ 受控发布产物与构建清单
  tools/             项目检查脚本
```

---

## 导航

**入门**
- [里程碑总索引](markdown/20-roadmap/00-roadmap-index.md)
- [能力晋级台账](markdown/20-roadmap/93-capability-promotion-ledger.md)
- [V1 收口记录](docs/release/v1-closeout.md)

**治理**
- [防回退与治理锁](markdown/20-roadmap/91-anti-regression-and-governance-lock.md)
- [决策门](markdown/20-roadmap/90-decision-gates.md)
- [递归开发协议](markdown/20-roadmap/95-recursive-development-protocol.md)
- [模块树与契约](markdown/00-matrix-governance/module-tree.md)

**架构**
- [架构决策记录](docs/decisions/)
- [工程目录地图](markdown/10-overview/overview-full-feature-tree.md)
