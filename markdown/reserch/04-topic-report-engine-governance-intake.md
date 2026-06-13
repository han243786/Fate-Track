# RPT-004 四专项判定引擎研究纳入

## 1. Source

| Field | Value |
| --- | --- |
| Report ID | RPT-004 |
| Original | `markdown/reserch/Rust 四柱八字判定引擎的研究与实现方案.md` |
| Chinese handling | 原文已为中文；规范副本为 `markdown/reserch/zh-CN/Rust 四柱八字判定引擎的研究与实现方案.zh-CN.md` |
| Intake loop | LOOP-092 |
| Target milestones | M29-M33 |
| Runtime status | Not implemented |

## 2. Research Summary

报告建议把四柱八字专项推演拆成两层：

| Layer | Governance Interpretation |
| --- | --- |
| 稳定规则层 | 天干地支、五行阴阳、十神映射、传统藏干、月令、通根、六合、六冲、三刑、六害等可以进入 deterministic rule tables |
| 工程评分层 | 原局、大运、流年触发转成四域分数与 trace；权重是工程启发式，不是古籍原文公式 |

本项目采纳其“规则透明 + 固定模板 + trace 可解释”的方向，但不直接复制报告中的完整 Rust crate，也不把四域分数解释成现实预测。

## 3. Adoption Matrix

| Topic | Report Proposal | Decision | Project Treatment |
| --- | --- | --- | --- |
| 十神映射 | 以日主为中心，按生克和阴阳生成十神表 | accepted | 作为 M29-M33 公共规则源；优先复用现有 `analysis-snapshot` 十神结果 |
| 传统藏干 | 默认传统藏干表，允许 override | adapted | V1 topic report 先使用现有藏干表；override 作为 future target，不进入本轮实现 |
| 月令与通根 | 月令为主，主气/中气/余气与通根计入强弱 | adapted | 可增强 M21 deep-analysis，但权重必须登记为 heuristic |
| 地支关系 | 六合、六冲、六害、三刑、自刑 pairwise 检测 | accepted | M29 公共基础可落固定表；高阶会合解冲暂缓 |
| 高阶会合解冲 | 三支/四支上下文规约 | deferred | 不进入 M29-M33；防止范围失控 |
| 四域分数 | 原局 50 基线 + 大运均值 + 流年均值 | adapted | 可作为内部排序/强弱提示，不输出“命运分数”式用户文案 |
| 性别字段 | 情感默认 blended；有性别时轻量偏置财/官杀 | adapted | 前端已有 sex；报告必须支持缺省/未知，不能伪造性别取象 |
| 大运/流年 | 消费已归一化序列，不计算起运 | adapted | 大运复用 M13；流年只作为 topic 内部年度触发，不宣称完整流年/流月能力 |
| 评分权重 | 报告给出一组默认权重 | adapted | 权重可作为 M29 heuristic draft；实现前必须测试和审计 |
| 完整代码 | 提供独立 Rust 库示例 | rejected as direct copy | 不直接复制；只抽取规则、DTO 和测试思想，按现有 `backend` 模块树重写 |

## 4. Shared Rule Extraction

四专项共享以下流程：

1. 读取既有四柱和日主。
2. 建立十神映射。
3. 读取藏干、五行、根气和月令支持。
4. 评估日主强弱、喜忌和格局。
5. 读取原局合冲刑害。
6. 读取大运和指定/当前流年触发。
7. 对四专项输出 trace-backed blocks。

### Accepted Fixed Tables

| Rule | Scope |
| --- | --- |
| 十神 | 比肩、劫财、食神、伤官、正财、偏财、正官、七杀、正印、偏印 |
| 地支关系 | 六合、六冲、六害、三刑、自刑 |
| 宫位 | 年柱、月柱、日支、时柱分别作为家族/父母环境、成长与事业环境、夫妻宫/自我亲密落点、晚辈与后段安排参考 |
| 周期触发 | 大运天干为显神，地支为藏干触发和与原局地支关系触发；流年同理但限 topic 内部使用 |

## 5. Topic Rule Extraction

### Relationship

| Input | Adopted Rule |
| --- | --- |
| Core palace | 日支夫妻宫是情感专项核心宫位 |
| Ten gods | 正官/七杀、正财/偏财为伴侣议题；食伤为表达；比劫为自我边界与竞争；印星为支持与安全感 |
| Gender | 有性别时可轻量偏置财/官杀；未知时采用 blended 中性解释 |
| Relations | 日支与月/时/流年/大运发生六合、六冲、六害、三刑时，作为关系主题被牵动 |
| Output | 只描述互动结构、表达方式、阶段主题，不判断结婚、离婚、出轨、分手或复合 |

### Wealth

| Input | Adopted Rule |
| --- | --- |
| Ten gods | 正财、偏财为资源主题；食神/伤官为产出转化；比肩/劫财为竞争与资源分配 |
| Strength | 日主偏弱时谨慎解释财星承载；日主偏强时关注资源调度与输出转化 |
| Favorable elements | 喜忌元素只作为传统结构提示，不做现实收益判断 |
| Cycle trigger | 大运/流年天干或藏干触发财星、食伤、比劫时作为资源主题被引动 |
| Output | 不给投资、借贷、收益、亏损、金额、职业收入或商业决策建议 |

### Family

| Input | Adopted Rule |
| --- | --- |
| Palaces | 年/月柱为家庭背景与长辈环境；日支为亲密生活落点；时柱为晚辈/长期安排参考 |
| Ten gods | 印星为支持、照顾和接纳；比劫为同辈边界；食伤为表达和晚辈主题；财官为现实责任与秩序 |
| Relations | 年/月/日/时宫位被合冲刑害触发时，解释为家庭互动结构变化 |
| Unknown hour | 时辰未知时，时柱相关家庭结论必须降级或隐藏 |
| Output | 不预测亲属健康、生死、生育、离散、冲突或家庭结果 |

### Career

| Input | Adopted Rule |
| --- | --- |
| Core palace | 月柱为事业环境与外部秩序的重要参考；时柱可作长期安排和后段发力参考 |
| Ten gods | 正官/七杀为责任、压力、规则；印星为学习、资质、支持；食伤为技能表达；财星为资源落地；比劫为协作竞争 |
| Strength and pattern | 日主强弱、格局和用神只解释用力方式，不判断现实能力高低 |
| Relations | 月柱、时柱与大运/流年发生合冲刑害时，作为事业主题被牵动 |
| Output | 不保证升职、失业、跳槽、行业选择、考试、创业或收入结果 |

## 6. Heuristic Weight Policy

报告中的权重可以作为实现参考，但本项目采用以下限制：

| Rule | Decision |
| --- | --- |
| 权重必须配置化或集中常量化 | accepted |
| API 可返回 trace 和 qualitative level | accepted |
| 用户文案直接展示 0-100 命运分数 | rejected for M29-M33 |
| 权重来源声称为古籍定值 | forbidden |
| 权重变化必须有测试快照 | required |

建议 M29 实现时把输出分为：

- `trace`: 面向审计和调试。
- `signals`: 面向前端报告组装。
- `score_internal`: 内部排序或强弱提示，默认不展示给用户。

## 7. Acceptance Samples

每个 topic 至少需要以下样例类型：

| Topic | Positive Sample A | Positive Sample B | Downgrade Sample | Forbidden Claim Sample |
| --- | --- | --- | --- | --- |
| relationship | 日支与流年六合，报告说明关系主题被柔和牵动 | 配偶星透出，报告解释伴侣议题显性 | 未知性别时 blended，不用男命/女命断语 | 包含“必然结婚/离婚/出轨/复合”必须失败 |
| wealth | 财星透出且食伤触发，报告解释资源转化主题 | 比劫触发，报告解释资源分配和竞争 | 日主偏弱时不得夸大承财 | 包含“稳赚/发财/破财/投资建议/具体金额”必须失败 |
| family | 印星明显，报告解释支持系统 | 月柱与流年合冲，报告解释家庭环境被牵动 | 未知时辰时隐藏时柱晚辈判断 | 包含“亲属会生病/死亡/生育/离散”必须失败 |
| career | 官杀透出，报告解释责任压力 | 食伤与财星触发，报告解释技能和资源落地 | 无流年输入时只解释原局和大运 | 包含“必升职/失业/行业选择/考试结果/收入结果”必须失败 |

## 8. DG-011 Decision

**Decision**: closed for design and restricted implementation planning.

**Chosen option**: 四专项允许进入 post-preview restricted 能力，采用固定规则表、集中 heuristic 权重、trace-backed blocks 和禁用断言审计。

**Rejected options**:

- 直接复制报告中的完整 Rust crate。
- 把四域评分直接展示为命运分数。
- 把 topic 内部流年触发宣称为完整流年/流月系统。
- 把研究报告中的工程权重宣称为古籍固定公式。

**Required implementation constraints**:

- 必须复用现有 chart、analysis、luck、deep_analysis 基础。
- 必须保留免责声明和非确定性表达。
- 必须通过 forbidden-claim audit。
- 必须在 closeout 前同步 `/api/capabilities`、README、模块树和前端能力文案。

## 9. Adopted Implementation Preflight Decisions

用户在 LOOP-093 采纳以下实现预检提议：

| Decision Item | Adopted Decision | Impact |
| --- | --- | --- |
| topic-report route shape | 使用统一 route：`GET /api/charts/topic-report?topic=relationship|wealth|family|career&date=&time=&timezone=&time_precision=&sex=&year=` | 四专项共享 DTO、trace、warnings、免责声明和 forbidden-claim audit；四个 capability 仍在能力台账中分别登记 |
| 年度引动默认值 | API 层不隐式取当前年，必须显式传入 `year`；前端可预填当前年，但请求必须携带该参数 | 保持可复现，避免同一 URL 跨年漂移；响应需回显 `year` 与 `year_source` |
| `score_internal` API 边界 | 不进入默认公开 API；后端可内部使用，输出面向前端的 `signals`、`trace`、`qualitative_level` | 避免用户把 0-100 理解为命运分数；如未来公开需另开决策、文案和测试 |

## 10. Remaining Implementation Questions

| Question | Status |
| --- | --- |
| 现有 M21 强弱/格局是否需要扩展权重 | 可在 M29-M33 递归中分批适配 |
