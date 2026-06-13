# M35: Timeline Lexicon And Rule Engine

## 0. Status

`closed by LOOP-099 as internal foundation`. DG-012 is closed by ADR 0022. M35 implements backend-internal `domain::timeline` lexicon/rule-engine structures and tests only; it does not create a public API, frontend UI, report chapter, `/api/capabilities` declaration, or user-visible capability promotion.

## 1. Goal

建立大运/流年解释层的规则词典和组合式信号引擎。它要把原局、大运、指定年份之间的干支、十神、五行、合冲刑害、藏干根气、强弱格局和用神关系转换成可测试、可追踪、可白话化的结构信号。

## 2. Dependencies

| Dependency | Why |
| --- | --- |
| M34 | DG-012, capability split, API boundary |
| M3 chart-create | 原局四柱和日主 |
| M4 analysis-snapshot | 十神、五行、藏干、关系基础 |
| M13 luck-cycles | 大运序列和起运信息 |
| M21 deep-analysis | 强弱、格局、用神 |
| M24 chart-report | 白话报告表达规范 |

## 3. Scope

| WP | Work Package |
| --- | --- |
| M35-WP1 | 定义 `TimelineSignal`：信号类别、强度等级、来源、主题适用性、风险标签 |
| M35-WP2 | 定义 `TimelineEvidence`：原局柱位、大运干支、流年干支、关系类型、触发路径 |
| M35-WP3 | 定义 `PlainReading`：专业句、白话句、边界句、用户可读提示 |
| M35-WP4 | 建立 `timeline-lexicon`：十神、五行生克、天干地支、藏干、合冲刑害、根气、格局用神解释片段 |
| M35-WP5 | 建立组合规则：显干触发、地支触发、藏干触发、原局关系触发、大运与流年叠加触发 |
| M35-WP6 | 建立 trace 和 rule version：每条解释都能回溯到规则与证据 |
| M35-WP7 | 测试禁止静态断语爆炸：规则组合必须可维护，不得以千条 hard-coded 成败文本替代结构引擎 |

## 4. Rule Primitive Inventory

| Primitive | Required Handling |
| --- | --- |
| 10 天干 | 五行、阴阳、十神映射、透出触发 |
| 12 地支 | 藏干、根气、冲合刑害、宫位触发 |
| 60 甲子 | 只作为干支组合，不直接绑定固定命运断语 |
| 十神 | 以日主为中心生成主题相关含义 |
| 五行生克 | 说明资源流向、支持与约束，不给吉凶定论 |
| 合冲刑害 | 说明关系被激活、拉扯、变化或压力，不断事件 |
| 原局宫位 | 年月日时作为结构位置，不断他人命运 |
| 大运/流年 | 作为时间背景与触发层，不宣称完整流月/流日系统 |

## 5. Output Contract Draft

| Field | Meaning | Boundary |
| --- | --- | --- |
| `signals[]` | 结构信号列表 | qualitative only |
| `evidence[]` | 每条信号的命理依据 | 必须可追踪 |
| `readings[]` | 专业解释 + 白话解释 | 不含确定性成败 |
| `warnings[]` | 未知时辰、边界、流派差异 | 必须显式 |
| `rule_version` | 规则版本 | 用于回归 |
| `audit` | forbidden-claim result | 必须通过 |

## 6. Non-Goals

- 不做前端 UI。
- 不开放公开 API，除非 M34/DG-012 已指定并在本里程碑 closeout 中验收。
- 不生成完整报告长文，只输出可组合结构件。
- 不做流月、流日、事件预测、择日或用户建议。

## 7. Capability Status

| Capability | Before | After Closeout |
| --- | --- | --- |
| `luck-reading` | planned | planned or restricted internal foundation only |
| `annual-trigger-reading` | planned | planned |
| `topic-timeline-reading` | planned | planned |

M35 的重点是可复用内核。除非 closeout 明确证明 API、文案、安全审计和前端边界齐备，否则不晋级用户可见能力。

## 8. Validation

```powershell
cargo test timeline -- --nocapture
npm.cmd run check --prefix frontend
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

验收必须证明：

- 至少覆盖十神、五行、藏干、合冲刑害、原局/大运/流年三层 evidence。
- 每条 reading 都有 trace。
- 禁用词审计覆盖成败、发财、结婚、离婚、生死、升职、失业、疾病等确定性表述。
- 规则词典可版本化，不把 `score_internal` 暴露给 public API。

## 9. LOOP-099 Closeout

| Evidence | Result |
| --- | --- |
| DG-012 | Closed by `docs/decisions/0022-dg-012-timeline-reading-boundary.md`. |
| Internal module | Added `backend/src/domain/timeline.rs` and `pub mod timeline;`. |
| Lexicon | `timeline_lexicon()` and `lexicon_entry()` cover ten gods, five elements, branch relations, hidden stems, pattern, and useful-god concepts. |
| Signal engine | `build_timeline_foundation()` composes major-luck, explicit annual pillar, original-chart anchors, hidden stems, branch relations, five-element flow, and major-luck/annual overlay into qualitative structures. |
| Output contract | Internal `TimelineSignal`, `TimelineEvidence`, `PlainReading`, `TimelineRuleVersion`, warnings, and `ForbiddenOutputAudit` are implemented. |
| Safety | `audit_timeline_text()` rejects deterministic money, relationship, family, career, disease/death, legal/investment style claims through base and timeline-specific forbidden patterns. |
| Tests | `cargo test timeline -- --nocapture` passed 6 tests. |
| Public exposure | None. No route, UI, report chapter, `/api/capabilities` entry, raw `luck-cycles` mutation, public score, or silent year default was added. |

M35 is therefore closed only as an internal foundation. `luck-reading`, `annual-trigger-reading`, and `topic-timeline-reading` remain planned/restricted-upper-bound user-visible capabilities until M36-M40 close their own gates.
