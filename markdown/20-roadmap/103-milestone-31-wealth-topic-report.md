# M31: Wealth Topic Report

## 0. Status

`closed` by LOOP-095. `wealth-report` is now a restricted capability through `GET /api/charts/topic-report?topic=wealth&year=YYYY`.

## 1. Goal

实现金钱专项命理推演报告。报告聚焦财星、资源意识、经营模式、风险偏好、阶段性财务主题，但不得输出投资建议或收益断言。

## 2. Dependencies

| Dependency | Why |
| --- | --- |
| M29 topic report foundation | 共享报告合约、安全审计、2 x 2 入口 |
| M3 chart-create | 四柱和日主 |
| M4 analysis-snapshot | 十神、藏干、五行关系 |
| M13 luck-cycles | 大运背景 |
| M21 deep-analysis | 日主强弱、格局、用神 |
| M24 chart-report | 报告表达范式 |

## 3. Scope

| WP | Work Package |
| --- | --- |
| M31-WP1 | 定义金钱取象规则：正财、偏财、食伤生财、比劫夺财、印星护身、官杀约束 |
| M31-WP2 | 判断财星透出、藏干、根气和位置，不把数量直接等同财富大小 |
| M31-WP3 | 结合日主强弱解释「能否承载财星」的传统含义 |
| M31-WP4 | 结合大运和当前/指定流年说明财务主题是否被引动 |
| M31-WP5 | 输出章节式报告：财星总览、正偏财、资源流动、承载能力、引动提示、白话结论 |
| M31-WP6 | 前端点击「金钱推演」后渲染报告 |
| M31-WP7 | 测试 forbidden claims：不得断言发财、破财、投资收益、债务、职业收入或具体金额 |

## 4. Professional Content Requirements

报告必须解释：

- 正财：稳定资源、现实事务、可管理收入倾向。
- 偏财：机会资源、流动资源、外部交换和经营意识。
- 食伤生财：表达、技能、产出与资源转化的传统链路。
- 比劫夺财：竞争、资源分配、自我投入与协作边界。
- 日主强弱：解释为结构承载能力，不等于现实赚钱能力。
- 财星透藏根气：说明主题显性/隐性和稳定程度。

## 5. Non-Goals

- 不做金融、投资、借贷、保险、税务或法律建议。
- 不承诺财富增长、亏损、暴富或破产。
- 不给具体交易、资产配置或商业决策。
- 不把财星旺弱等同现实财富多少。

## 6. Capability Status

| Capability | Before | After Closeout |
| --- | --- | --- |
| `wealth-report` | planned | restricted |

Restricted 的含义：报告是传统结构解释，不是财务建议。

## 7. Validation

```powershell
cargo test --lib
cd frontend && npm run check
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

验收必须证明：

- 报告包含正财、偏财、食伤生财、比劫、承载能力和引动提示。
- 财务免责声明清楚。
- 禁用词审计覆盖发财、破财、稳赚、亏损、投资建议等高风险断语。

LOOP-095 closeout evidence:

- Backend shared TopicReport route returns `wealth-report` as restricted with explicit `year`, no public `score_internal`, and forbidden-output audit passed.
- `/api/capabilities` exposes `wealth-report` as restricted.
- Frontend 2 x 2 topic entry enables 金钱 and renders through the shared topic-report panel.
- `cargo test -- --nocapture` passed: 94 Rust unit tests plus integration artifact/golden/replay/comparison tests.
- `npm.cmd run check --prefix frontend` passed: 12 frontend tests.
