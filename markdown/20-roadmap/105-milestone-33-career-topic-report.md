# M33: Career Topic Report

## 0. Status

`closed` by LOOP-095. `career-report` is now a restricted capability through `GET /api/charts/topic-report?topic=career&year=YYYY`.

## 1. Goal

实现事业专项命理推演报告。报告聚焦职业主题、责任结构、表达技能、资源调度、阶段性事业引动，但不得给就业、升迁、行业或结果保证。

## 2. Dependencies

| Dependency | Why |
| --- | --- |
| M29 topic report foundation | 共享 TopicReport 合约、2 x 2 入口、安全审计 |
| M3 chart-create | 四柱和日主 |
| M4 analysis-snapshot | 十神、藏干、五行关系 |
| M13 luck-cycles | 大运背景 |
| M21 deep-analysis | 强弱、格局、用神 |
| M24 chart-report | 报告表达范式 |

## 3. Scope

| WP | Work Package |
| --- | --- |
| M33-WP1 | 定义事业取象规则：官杀、印星、食伤、财星、比劫与事业主题的对应 |
| M33-WP2 | 解释财官印、杀印相生、食伤制杀、食伤生财等链路，只作为结构参考 |
| M33-WP3 | 结合日主强弱、格局和用神说明事业用力方式 |
| M33-WP4 | 结合大运和当前/指定流年说明事业主题是否被引动 |
| M33-WP5 | 输出章节式报告：事业总览、责任压力、技能表达、资源调度、协作竞争、引动提示、白话结论 |
| M33-WP6 | 前端点击「事业推演」后渲染报告 |
| M33-WP7 | 测试 forbidden claims：不得断言升职、失业、跳槽成败、行业选择、考试结果或收入结果 |

## 4. Professional Content Requirements

报告必须解释：

- 官杀：责任、规则、压力、职位结构和外部要求。
- 印星：学习、资质、支持系统、承接压力的方式。
- 食伤：表达、技术、产出、创意和解决问题方式。
- 财星：资源、现实落地、商业意识和结果导向。
- 比劫：协作、竞争、自主性和团队边界。
- 格局与用神：说明事业主题的组织方式，不评价人生高低。

## 5. Non-Goals

- 不保证升职、转行、入职、考试、创业或收入结果。
- 不给职业选择、法律、财务、劳动争议或投资建议。
- 不做具体年份的确定事件预测。
- 不把事业强弱等同现实能力或社会价值。

## 6. Capability Status

| Capability | Before | After Closeout |
| --- | --- | --- |
| `career-report` | planned | restricted |

Restricted 的含义：报告是传统结构解释，不是职业规划结论。

## 7. Validation

```powershell
cargo test --lib
cd frontend && npm run check
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

验收必须证明：

- 报告包含官杀、印星、食伤、财星、格局/用神和引动提示。
- 禁用词审计覆盖升职、失业、行业、考试、收入等确定性断语。
- 前端入口可从 2 x 2 按钮进入并返回报告区域。

LOOP-095 closeout evidence:

- Backend shared TopicReport route returns `career-report` as restricted with explicit `year`, no public `score_internal`, and forbidden-output audit passed.
- `/api/capabilities` exposes `career-report` as restricted.
- Frontend 2 x 2 topic entry enables 事业 and renders through the shared topic-report panel.
- `cargo test -- --nocapture` passed: 94 Rust unit tests plus integration artifact/golden/replay/comparison tests.
- `npm.cmd run check --prefix frontend` passed: 12 frontend tests.
