# M38: Topic Timeline Overlay

## 0. Status

`closed by LOOP-102`. M38 starts after M37 annual trigger reading is restricted or otherwise accepted as internal foundation, and closes `topic-timeline-reading` as a restricted post-preview capability carried by the existing topic-report route.

Closeout artifact: `117-milestone-38-closeout.md`.

## 1. Goal

把共享的大运/年度引动结构信号投射到四专项报告：情感、金钱、家庭、事业。四专项不重新发明时间解释规则，只消费 M35-M37 的 `TimelineSignal` 和 `TimelineEvidence`，再用各专题的取象语言解释。

## 2. Dependencies

| Dependency | Why |
| --- | --- |
| M29-M33 | 四专项 TopicReport 合约与安全边界 |
| M35 | 共享 timeline signal/evidence |
| M36 | 大运阶段解释 |
| M37 | 年度引动解释 |
| LOOP-097 | 工作台摘要与完整报告页分层 |

## 3. Scope

| WP | Work Package |
| --- | --- |
| M38-WP1 | 定义 `TopicTimelineOverlay`：topic、shared signals、topic lens、plain reading、warnings |
| M38-WP2 | 情感叠加：夫妻宫、财官杀、表达/边界/互动模式，只解释关系结构 |
| M38-WP3 | 金钱叠加：财星、食伤、比劫、印官约束，只解释资源与承载结构 |
| M38-WP4 | 家庭叠加：年/月/日/时宫位、印星、比劫、食伤、财官责任，只解释家庭互动结构 |
| M38-WP5 | 事业叠加：官杀、印星、食伤、财星、比劫、格局用神，只解释责任、技能、资源和协作 |
| M38-WP6 | 将四专项完整报告加入“本专题的大运流年”章节，工作台仍只保留结构信号 |
| M38-WP7 | 测试四专题禁用建议与结果保证：金融、婚恋、亲属命运、职业结果都不得出现 |

## 4. Topic Mapping Rules

| Topic | Allowed Interpretation | Forbidden Interpretation |
| --- | --- | --- |
| relationship | 互动模式、表达方式、边界、伴侣象征被引动 | 结婚、离婚、出轨、复合、对象身份断言 |
| wealth | 资源进入、分配、表达转化、承载压力 | 投资建议、收益亏损、具体金额、债务判断 |
| family | 沟通、责任、支持、界限、家庭角色张力 | 亲属生死健康、生育、家庭变故断言 |
| career | 责任、资质、技能输出、资源落地、协作竞争 | 升职、失业、考试、行业、收入、跳槽成败 |

## 5. Non-Goals

- 不扩展新 topic。
- 不新增公开 score 或排序。
- 不让四专项自行计算大运/流年，必须复用共享 timeline foundation。
- 不把专题叠加写回 M13 raw `luck-cycles`。

## 6. Capability Status

| Capability | Before | After Closeout |
| --- | --- | --- |
| `topic-timeline-reading` | planned | restricted by LOOP-102 |
| `relationship-report` | restricted | unchanged, optional richer section |
| `wealth-report` | restricted | unchanged, optional richer section |
| `family-report` | restricted | unchanged, optional richer section |
| `career-report` | restricted | unchanged, optional richer section |

## 7. Validation

```powershell
cargo test topic_timeline -- --nocapture
npm.cmd run check --prefix frontend
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

验收必须证明：

- 四专题使用同一 timeline evidence。
- 每个专题都有“专业解释 + 白话解释 + 边界提醒”。
- 工作台仍只显示结构信号，不回退为长报告堆叠。
- `score_internal` 不进入 public API。

## 8. Closeout Summary

LOOP-102 implemented M38 by adding `TopicTimelineOverlay` to `backend/src/domain/topic_report.rs`, reusing `build_annual_trigger_foundation()` and shared `TimelineSignal`/`TimelineEvidence` from M35-M37. The four restricted topic reports now include a full-report chapter `本专题的大运流年`, while the workbench remains a structure-signal-only summary. `/api/capabilities` declares `topic-timeline-reading` as restricted on `/api/charts/topic-report?topic=relationship|wealth|family|career&year=YYYY`. Raw `GET /api/luck/cycles` remains unchanged and scoreless.
