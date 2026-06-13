# 研究报告纳入台账

> 目录名 `reserch` 保留原始拼写，避免破坏用户提供路径。后续若迁移到 `research`，必须同步工程树、模块树和 README。

## 1. 纳入范围

| 编号 | 原始报告 | 中文处理 | 治理用途 |
| --- | --- | --- | --- |
| RPT-001 | `命轨 Fate-Track V1 产品需求与八字算法规格研究报告.md` | 已是中文，作为产品与算法中文源报告 | 产品树、算法目标、数据模型、测试策略 |
| RPT-002 | `Fate-Track V1 Design Report.md` | `zh-CN/Fate-Track V1 Design Report.zh-CN.md` | 功能树、用户故事、架构、前端 IA、安全解释 |
| RPT-003 | `Fate-Track V1 Product Spec and Engineering Plan.md` | `zh-CN/Fate-Track V1 Product Spec and Engineering Plan.zh-CN.md` | API 合约、工程分层、隐私安全、验证计划 |
| RPT-004 | `Rust 四柱八字判定引擎的研究与实现方案.md` | 原文已为中文；规范副本 `zh-CN/Rust 四柱八字判定引擎的研究与实现方案.zh-CN.md` | M29-M33 四专项推演：规则层、工程评分层、trace、风险和验收样例 |

## 2. 治理采纳状态

| 主题 | 研究结论 | 治理状态 | 当前处理 |
| --- | --- | --- | --- |
| 当前日期层 | Android 万年历仓库能保证当前三柱样例稳定 | accepted-current | 保留 ADR 0002，不被新研究直接推翻 |
| 长期历法真源 | 使用星历/天文事件生成节气、朔和黄金表 | target | 进入 ADR 0004，作为后续升级路线 |
| V1 默认规则档 | 年柱立春、月柱节令、日界 00:00、时柱民用时，真太阳时可选 | target | 进入 ADR 0004，未实现前不得标 supported |
| 未知时辰 | 不伪造时柱，输出部分命盘和敏感性摘要 | target | 进入规则与 API 目标，当前仍 planned |
| API 版本 | 目标合约使用 `/api/v1/...` 和稳定错误 envelope | target | 当前原型 `/api/...` 保留，目标合约进入治理 |
| 领域实体 | BirthProfile、ChartRequest、BaziChart、Pillar、TenGod 等 | accepted-design | 领域骨架可按此扩展 |
| 隐私分级 | 出生时间、地点、时区、命盘输出为敏感或敏感衍生数据 | accepted-policy | 进入 ADR 0005 与 General Policy |
| 日志禁令 | 不记录出生 request body、分享 token、完整命盘 JSON、私有备注 | accepted-policy | 进入 ADR 0005 与 General Policy |
| 分享 | 公开分享默认脱敏、不可变快照、token 高熵且 hash 存储 | target | 进入 ADR 0005，待后端能力实现 |
| 安全文案 | 禁止确定性、诊断式、死亡/疾病/财富保证等断言 | accepted-policy | 进入 ADR 0005 与 General Policy |
| 前端 IA | 命盘工作台、规则徽标、边界警告、术语抽屉、移动端 stepper | target | 进入产品树和模块树目标 |
| 黄金测试 | 1901–2100、立春/节令边界、甲子日锚点、2033 异常、时区回放 | target | 进入标准矩阵和后续测试目录规划 |
| 四专项规则层 | 五行阴阳、十神、传统藏干、月令通根、六合/六冲/三刑/六害 | accepted-design | 进入 `04-topic-report-engine-governance-intake.md`，作为 M29 公共基础 |
| 四专项工程评分 | 原局基线 + 大运均值 + 流年均值 + trace | adapted | 仅作 restricted topic report 内部启发式，不作为古籍定论或用户可见命运分 |
| 情感 blended 模式 | 未知性别时同时参考财星与官杀，核心看日支夫妻宫 | adapted | M30 使用中性解释；有性别时仅轻量偏置，不做婚恋断言 |
| topic 内部流年触发 | 消费指定/当前流年，与原局宫位和十神关系触发 | adapted | 只服务四专项报告，不宣称完整流年/流月能力 |

## 3. 当前项目与研究目标的差异

| 项目现状 | 研究目标 | 处理原则 |
| --- | --- | --- |
| 后端为基础 Rust HTTP 骨架，未引入 Axum | 目标后端为 Axum + 服务层 + 多 crate workspace | 当前不立即重构；先把目标登记为架构路线 |
| 日期层来自 Android skip table | 长期目标为星历派生真源 + 黄金表 | 保留 Android 作为 accepted-current；星历方案作为 target |
| API 当前为 `/api/health`、`/api/calendar/query` 等 | 目标为 `/api/v1/...` | 当前能力不改名；目标 API 合约进入 ADR 和模块树 |
| 前端当前是能力看板 | 目标为命盘工作台 | 现有前端只声明 supported/planned；工作台作为后续实现 |
| 领域模型只是骨架 | 研究给出详细字段与隐私分类 | 后续代码变更必须按字段隐私等级和规则元数据推进 |

## 4. 实现前置门槛

| 能力 | 标记为 supported 前必须满足 |
| --- | --- |
| 排盘创建 | Rust 后端真实实现；API 返回 `ruleset_id`、`algo_version`、时区和边界元数据；覆盖年/月/日/时回归样例 |
| 农历输入 | 验证闰月合法性；非法闰月可解释拒绝；月柱仍按节气说明 |
| 真太阳时 | 记录经度、时间方程/偏移分钟数；显示哪些柱发生变化；经度缺失时拒绝或禁用 |
| 未知时辰 | 时柱为空；返回候选/稳定性摘要；禁止默认中午 |
| 大运 | 明确 `luck_ruleset_id`、顺逆规则、起运年龄算法；测试边界样例 |
| 分享 | token hash 存储；默认隐藏姓名、精确时间、地点、备注；公开页不暴露私有案例状态 |
| 分析文案 | 输出结构化指标；附免责声明；通过禁用短语审查 |
| 持久化 | 出生信息和命盘快照按敏感数据处理；日志不含原始请求体 |

## 5. 后续实现优先级

| 优先级 | 任务 | 依据 |
| --- | --- | --- |
| P0 | 把当前 `/api/calendar/query` 扩展为带规则元数据的日期层响应 | 当前已有 Android 日期层和三柱样例 |
| P0 | 定义 `ft-v1-default` 规则档常量和元数据结构 | 三份报告共同要求可复现规则 |
| P0 | 建立黄金样例目录与测试清单 | 研究报告反复强调边界回归 |
| P0 | 把 BirthProfile / ChartRequest / BaziChart 字段隐私分级落入 Rust domain | 后续 API 与分享依赖 |
| P1 | 排盘创建 API 与命盘详情 API | 进入核心产品闭环 |
| P1 | 分享脱敏快照与 token 管理 | 隐私风险高，需要规则先行 |
| P1 | 前端命盘输入 stepper 与边界提示 | 直接提升可信度 |
| P2 | 星历派生黄金表和 Axum 多 crate 重构 | 价值高但涉及架构迁移 |

## 6. 未决问题

- 是否正式把 V1 默认规则档命名为 `ft-v1-default`。
- V1 验证范围最终采用 1900–2100 还是 1901–2100。
- 大运顺逆与起运年龄使用哪一派默认规则。
- 是否在 V1 直接开放农历输入，还是先只通过日历工具间接支持。
- 是否立即引入 Axum，还是等当前 HTTP 骨架跑通第一版命盘 API 后迁移。
- 账号、云同步、保存上限和数据删除策略尚未最终指定。
- LOOP-093 已采纳 topic-report 统一 route：`GET /api/charts/topic-report`，通过 `topic` 参数区分四专项。
- LOOP-093 已采纳年度引动显式 `year`：API 不隐式取当前年，前端可预填但必须随请求发送。
