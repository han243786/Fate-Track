# M30: Relationship Topic Report

## 0. Status

`closed` by LOOP-094. `relationship-report` is now a restricted capability through `GET /api/charts/topic-report?topic=relationship&year=YYYY`. LOOP-095 later closed M31-M33, so wealth/family/career are also restricted through the shared topic-report route.

## 1. Goal

实现情感专项命理推演报告。报告模仿总命理报告形式，但聚焦情感互动、伴侣议题、关系模式和阶段性引动。

## 2. Dependencies

| Dependency | Why |
| --- | --- |
| M29 topic report foundation | 共享 TopicReport 合约、UI 入口、安全审计 |
| M3 chart-create | 四柱与日主 |
| M4 analysis-snapshot | 十神、藏干、五行、关系标记 |
| M13 luck-cycles | 大运背景 |
| M21 deep-analysis | 强弱、格局、用神 |
| M24 chart-report | 章节式报告与免责声明风格 |

## 3. Scope

| WP | Work Package |
| --- | --- |
| M30-WP1 | 定义情感取象规则：日支夫妻宫、配偶星、官杀/财星、比劫影响、印星支持、食伤表达 |
| M30-WP2 | 性别缺省策略：有性别时按传统配偶星取象；无性别时使用关系中性解释，不伪造配偶星 |
| M30-WP3 | 解释合冲刑害对夫妻宫、配偶星和情感表达的影响 |
| M30-WP4 | 结合大运和当前/指定流年，说明情感主题是否被引动 |
| M30-WP5 | 输出章节式报告：情感总览、配偶星、夫妻宫、表达方式、引动提示、白话结论 |
| M30-WP6 | 前端点击「情感推演」后渲染报告，保持 M29 2 x 2 入口和 M24 报告视觉节奏 |
| M30-WP7 | 测试 forbidden claims：不得断言结婚、离婚、出轨、分手、复合或伴侣身份 |

## 4. Professional Content Requirements

报告必须同时包含专业术语和白话解释：

- 日支夫妻宫：解释为关系落点和亲密互动位置。
- 配偶星：解释为传统命理中观察伴侣议题的关系符号。
- 官杀/财星：按输入性别或中性策略解释，不做现实身份断言。
- 比劫：解释为自我、竞争、边界、同辈影响。
- 印星：解释为安全感、支持、接纳和照顾模式。
- 食伤：解释为表达、输出、情绪表达和关系沟通方式。
- 合冲刑害：解释为关系结构被牵动的表现形式，而不是事件判定。

## 5. Non-Goals

- 不做婚恋结果预测。
- 不判断伴侣好坏、是否出轨、是否离婚。
- 不输出诱导分手、复合、结婚或生育决策。
- 不做合盘、择日、流月或具体事件时间表。

## 6. Capability Status

| Capability | Before | After Closeout |
| --- | --- | --- |
| `relationship-report` | planned | restricted |

Restricted 的含义：报告可用，但必须保留免责声明、边界提示和非确定性表达。

LOOP-094 closeout: `/api/capabilities` exposes `relationship-report` as restricted; backend route tests cover explicit year, invalid/missing topic, planned topics returning 501, no public `score_internal`, and forbidden-output audit; frontend tests cover the 2 x 2 entry with only relationship enabled at that time. LOOP-095 updates the same route and frontend entry so all four topic reports are now restricted.

## 7. Validation

```powershell
cargo test --lib
cd frontend && npm run check
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

验收必须证明：

- 报告非空且有免责声明。
- 至少包含夫妻宫、配偶星、十神、合冲刑害、大运/流年引动说明。
- 未知时辰时不伪造时柱相关结论。
- 禁用词审计通过。

LOOP-094 validation evidence:
- `cargo test -- --nocapture` passed: 94 unit tests plus integration artifact/golden/replay/comparison tests.
- `npm.cmd run check --prefix frontend` passed: 12 frontend tests.
