# M32: Family Topic Report

## 0. Status

`closed` by LOOP-095. `family-report` is now a restricted capability through `GET /api/charts/topic-report?topic=family&year=YYYY`.

## 1. Goal

实现家庭专项命理推演报告。报告聚焦家庭关系模式、支持系统、长辈/同辈/晚辈主题和沟通方式，不做亲属命运或家庭事件断言。

## 2. Dependencies

| Dependency | Why |
| --- | --- |
| M29 topic report foundation | 共享 TopicReport 合约、年度引动层、安全审计 |
| M3 chart-create | 四柱和日主 |
| M4 analysis-snapshot | 十神、藏干、五行关系 |
| M13 luck-cycles | 大运背景 |
| M21 deep-analysis | 强弱、格局、用神 |
| M24 chart-report | 章节式表达 |

## 3. Scope

| WP | Work Package |
| --- | --- |
| M32-WP1 | 定义家庭取象规则：年柱/月柱/日支/时柱的家庭位置参考 |
| M32-WP2 | 定义十神主题：印星为支持与长辈照顾，比劫为同辈与边界，食伤为表达与晚辈主题，财官为现实责任与秩序 |
| M32-WP3 | 结合合冲刑害解释家庭互动结构被牵动的形式 |
| M32-WP4 | 结合大运和当前/指定流年说明家庭议题是否被引动 |
| M32-WP5 | 输出章节式报告：家庭总览、长辈支持、同辈边界、晚辈/表达、责任秩序、引动提示、白话结论 |
| M32-WP6 | 前端点击「家庭推演」后渲染报告 |
| M32-WP7 | 测试 forbidden claims：不得断言亲属生死、疾病、生育、离散、冲突或家庭结果 |

## 4. Professional Content Requirements

报告必须解释：

- 年柱/月柱：传统上用于观察早年环境、家族背景、长辈关系的参考位置。
- 日支：自我与亲密生活的落点，也会影响家庭互动方式。
- 时柱：传统上用于观察晚辈、长期安排和后段主题，未知时辰必须降级。
- 印星：支持、照顾、接纳、安全感。
- 比劫：同辈、边界、协作和竞争。
- 食伤：表达、沟通、晚辈主题和输出方式。
- 财官：家庭中的现实责任、秩序、角色安排。

## 5. Non-Goals

- 不预测亲属健康、生死、婚育或家庭变故。
- 不给家庭关系处置指令。
- 不做亲子、婚姻、心理、法律或医疗建议。
- 不把家庭成员的命运归因到用户命盘。

## 6. Capability Status

| Capability | Before | After Closeout |
| --- | --- | --- |
| `family-report` | planned | restricted |

Restricted 的含义：报告只解释用户命盘中的家庭互动结构，不断言他人命运。

## 7. Validation

```powershell
cargo test --lib
cd frontend && npm run check
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

验收必须证明：

- 报告包含宫位、印星、比劫、食伤、财官和引动提示。
- 未知时辰时不输出时柱家庭结论。
- 禁用词审计覆盖亲属生死、疾病、生育和家庭冲突断言。

LOOP-095 closeout evidence:

- Backend shared TopicReport route returns `family-report` as restricted with explicit `year`, no public `score_internal`, and forbidden-output audit passed.
- Unknown-hour handling downgrades 时柱晚辈、长期安排 and 后段主题 instead of fabricating conclusions.
- `/api/capabilities` exposes `family-report` as restricted.
- Frontend 2 x 2 topic entry enables 家庭 and renders through the shared topic-report panel.
- `cargo test -- --nocapture` passed: 94 Rust unit tests plus integration artifact/golden/replay/comparison tests.
- `npm.cmd run check --prefix frontend` passed: 12 frontend tests.
