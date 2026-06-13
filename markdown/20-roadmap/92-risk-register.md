# 风险台账

## 1. S0 风险

| Risk ID | 风险 | 触发信号 | 缓解措施 | 责任落点 |
| --- | --- | --- | --- | --- |
| R-S0-001 | 日期层三柱回退 | Android 边界样例失败或被删除 | 保留黄金样例；任何替换必须 ADR | `backend.calendar.*` |
| R-S0-002 | 前端误标 supported | UI 展示后端未实现能力 | capability ledger + `/api/capabilities` 校验 | `frontend.ui`, `backend.api` |
| R-S0-003 | 敏感出生资料进入日志 | 请求体、地点、token、完整命盘 JSON 出现在日志 | 日志禁令、审查、测试或扫描 | `backend.observability` |
| R-S0-004 | 分享泄露私有状态 | 公开 token 能读到私有备注、精确时间或实时案例状态 | 默认脱敏、不可变快照、token hash | `backend.share` |
| R-S0-005 | 高风险确定性断言 | 分析输出疾病、死亡、金融、法律、关系确定性结论 | 禁用短语审查、固定免责声明 | `backend.analysis.engine`, `frontend.ui` |

## 2. P1 风险

| Risk ID | 风险 | 触发信号 | 缓解措施 | 责任落点 |
| --- | --- | --- | --- | --- |
| R-P1-001 | 研究目标和实现状态混淆 | 文档写目标 API 但用户以为可用 | target/planned/supported 分层 | `governance.research` |
| R-P1-002 | 规则元数据不足 | 命盘结果无法复现 | `ruleset_id`、`algo_version`、时区、边界元数据 | `backend.bazi.engine` |
| R-P1-003 | 时区历史错误 | 固定 offset 代替 IANA TZ | 降级警告和 replay metadata | `backend.calendar.astronomy` |
| R-P1-004 | 未知时辰伪精确 | 默认中午或默认子时 | 时柱 null + 候选/稳定性摘要 | `backend.bazi.engine` |
| R-P1-005 | 大运规则争议 | 没有 `luck_ruleset_id` | 决策门 DG-005 关闭后再实现 | `backend.luck.engine` |
| R-P1-006 | Axum 重构过早 | 路由未稳定就大迁移 | 先保当前骨架，按 M9 或独立 ADR 迁移 | `backend.app` |
| R-P1-039 | 四专项研究报告被直接当作运行时规则 | 文档或代码跳过翻译、采纳矩阵和 DG-011 | 先做 M29 research intake，标注 accepted/adapted/rejected/deferred | `governance.research`, `governance.roadmap` |
| R-P1-040 | 四专项报告暗示完整流年/流月系统 | topic report 输出年度事件表、流月流日或具体时间断言 | 年度引动层只服务四专项 restricted 报告；不得宣称完整流年/流月能力 | `backend.topic-report`, `frontend.ui` |
| R-P1-041 | 金钱报告被理解为金融建议 | 输出投资、收益、亏损、债务或具体金额判断 | 固定财务免责声明；禁用词审计；财富主题只解释结构不做建议 | `backend.topic-report.wealth` |
| R-P1-042 | 情感报告被理解为婚恋建议 | 输出结婚、离婚、出轨、分手、复合或伴侣身份断言 | 固定婚恋免责声明；只解释关系结构和互动模式 | `backend.topic-report.relationship` |
| R-P1-043 | 家庭报告断言亲属命运 | 输出亲属健康、生死、生育、离散或家庭变故 | 固定家庭边界；未知时辰降级；不得把他人命运归因到用户命盘 | `backend.topic-report.family` |
| R-P1-044 | 事业报告被理解为职业结果保证 | 输出升职、失业、考试、行业、收入或跳槽成败断言 | 固定职业免责声明；只解释责任、表达、资源和协作结构 | `backend.topic-report.career` |
| R-P1-045 | 启发式权重被包装成古法定论 | 文案声称分数、权重或强弱阈值来自古籍固定公式 | 权重集中常量化并标注 heuristic；默认不向用户展示 0-100 命运分；测试保护 forbidden-claim 和 trace 解释 | `backend.topic-report`, `governance.research` |
| R-P1-046 | 大运/流年解释层输出确定性未来断言 | timeline reading 出现必然成败、发财、结婚、离婚、升职、失业、疾病、生死或具体日期 | M34-M40 只允许结构观察、窗口提示和白话解释；M40 forbidden suite 必须覆盖高风险词 | `backend.timeline-reading`, `frontend.report-ui`, `governance.roadmap` |
| R-P1-047 | 10 x 12 或 60 甲子解释变成模板爆炸 | 为每个干支组合硬编码大量断语，维护成本失控 | M35 采用组合式 `timeline-core` + `timeline-lexicon`；M40 性能和可维护性门禁检查静态断语爆炸 | `backend.timeline-core`, `governance.capability-ledger` |
| R-P1-048 | M13 raw `luck-cycles` 被解释字段污染 | `/api/luck/cycles` response 出现报告文本、score、topic overlay 或年度引动结论 | DG-012 未关闭前禁止实现；M36 必须保护 raw route，只通过指定解释 route/report surface 承载阅读层 | `backend.api.luck`, `backend.timeline-reading` |
| R-P1-049 | 年度引动被误解为完整流年/流月系统 | 文档、UI 或 API 暗示已支持流月、流日、逐日推送、事件时间线 | M37 只接受显式 `year`，M40 no-overclaim checks；流月/流日必须另开里程碑和门禁 | `backend.annual-trigger`, `frontend.report-ui` |
| R-P1-050 | 四专题 timeline overlay 变成高风险现实决策建议 | 专题大运流年章节输出投资承诺、婚恋事件断言、亲属命运断言或职业结果建议 | M38 只消费共享信号并做 topic lens；四专题原有 forbidden audit 继续适用并扩展到 timeline 章节；M41 允许情感报告保留低风险关系节奏建议，但禁止确定性事件和身份断言 | `backend.topic-timeline`, `frontend.topic-report` |

## 3. P2 风险

| Risk ID | 风险 | 缓解措施 |
| --- | --- | --- |
| R-P2-001 | 前端工作台信息密度过高 | 移动端 stepper、折叠卡、规则徽标 |
| R-P2-002 | 术语解释与分析脱节 | GlossaryEntry 与分析字段双向引用 |
| R-P2-003 | 1901-2100 外用户误解 | ADR 0008 锁定 V1 官方验证范围；API 返回 `out_of_range`；README/UI 显示验证范围和置信等级 |
| R-P2-004 | 2033 异常处理延后 | 放入 M1/M9 黄金样例计划 |
| R-P2-005 | 左下 2 x 2 入口挤压主界面 | 稳定网格尺寸、移动端折叠或下移，按钮未实现前显示 planned/disabled |
| R-P2-006 | 大运/流年解释仍然太术语化或机器化，小白不可读 | 每条 timeline reading 必须自然解释术语、直接面向“您”、带边界句；LOOP-106 文案门禁拦截硬标签、泛化年份、病句、`用户` 口吻、后端/前端说明和内部引擎名；LOOP-107 进一步要求词典原文和生成读法都有足够解释密度 |

## 3.1 LOOP-094 Risk Evidence

| Risk ID | LOOP-094 Evidence |
| --- | --- |
| R-S0-002 | Frontend markup test verifies only `topic-relationship-button` is enabled; wealth/family/career buttons are disabled. `/api/capabilities` exposes `relationship-report` only; tests assert wealth/family/career are absent. |
| R-P1-040 | Topic-report route requires explicit `year`; relationship report text frames annual trigger as internal topic observation, not full flow-year/month system. |
| R-P1-042 | Relationship route/domain tests assert forbidden-output audit passes and no deterministic romance terms such as 必然结婚 or 出轨 appear. |
| R-P1-045 | Public topic-report response tests assert `score_internal` is absent; frontend renders qualitative signals/trace rather than numeric fate scores. |
| R-P2-005 | CSS adds stable 2 x 2 shortcut grid and desktop locked-height topic panel; frontend workspace tests protect required topic IDs and disabled states. |

## 3.2 LOOP-095 Risk Evidence

| Risk ID | LOOP-095 Evidence |
| --- | --- |
| R-S0-002 | `/api/capabilities` now exposes `relationship-report`, `wealth-report`, `family-report`, and `career-report` as restricted; frontend markup tests verify all four 2 x 2 topic buttons are enabled only after this backend capability state exists. |
| R-P1-040 | The shared topic-report route still requires explicit `year`; all four topic reports frame annual trigger text as an internal topic observation and do not claim a complete flow-year/month system. |
| R-P1-041 | Wealth route tests assert forbidden terms such as `稳赚`, `发财`, `破财`, `投资建议`, and `具体金额` are absent; disclaimer frames the output as structure reading rather than financial advice. |
| R-P1-043 | Family route tests assert forbidden terms such as `亲属会生病`, `亲属会死亡`, `必然生育`, and `家庭离散` are absent; unknown-hour handling downgrades hour/palace sensitivity rather than fabricating family claims. |
| R-P1-044 | Career route tests assert forbidden terms such as `必升职`, `必然失业`, `跳槽必成`, `考试必过`, and `收入必涨` are absent; wording stays on responsibility, skill, resource, collaboration, and structure. |
| R-P1-045 | App-layer tests cover all four topic responses and assert `score_internal` is absent; public responses remain qualitative with trace evidence only. |
| R-P2-005 | Frontend 2 x 2 entry is tested with all four `data-topic` buttons enabled and stable after the M30 compact layout; browser verification confirms all four buttons visible/enabled at 1280 x 720 and clickable in the local app. |

## 3.3 LOOP-096 Risk Evidence

| Risk ID | LOOP-096 Evidence |
| --- | --- |
| R-P2-002 | Four topic reports now include a dedicated daily-reading block with `日常读法`, `换成日常语言`, and `落到生活里`; app-layer tests assert these plain-language anchors remain present in relationship, wealth, family, and career outputs. |
| R-P1-041 | Wealth plain-language hardening translates 正财/偏财/食伤/比劫 into resource-mode observations while preserving the no-financial-advice boundary and forbidden-term audit. |
| R-P1-043 | Family plain-language hardening translates 宫位/印星/比劫/食伤/财官 into user-side interaction observations while preserving the no-relative-fate boundary and unknown-hour downgrade. |
| R-P1-044 | Career plain-language hardening translates 官杀/印星/食伤/财星/比劫 into responsibility, support, skill, resource, and collaboration observations while preserving the no-career-result boundary. |

## 3.4 LOOP-097 Risk Evidence

| Risk ID | LOOP-097 Evidence |
| --- | --- |
| R-P1-040 | `topic-report.html` still passes an explicit `year` to the existing topic-report route; no full flow-year/month route or event timeline was introduced. |
| R-P1-041/R-P1-043/R-P1-044 | The full topic report page reuses backend restricted text and disclaimers; the workspace panel now shows only qualitative `signals`, reducing the chance that dense report text is mistaken for advice inside the dashboard. |
| R-P1-045 | The new frontend entry and report page consume the public topic-report DTO only and add no `score_internal` display, public score, or client-side scoring layer. |
| R-P2-005 | Frontend tests assert the left-bottom `查看专项报告` entry exists, the full topic report page is separate, and the workspace renderer no longer consumes `report.blocks` or trace rows. |

## 3.5 LOOP-098 Risk Evidence

| Risk ID | LOOP-098 Evidence |
| --- | --- |
| R-P1-046 | M34-M40 are documented as planned/restricted-upper-bound only; DG-012 blocks implementation until deterministic future-claim boundaries are closed. |
| R-P1-047 | M35 requires a compositional `timeline-core` + `timeline-lexicon` design and explicitly rejects thousand-row static success/failure templates. |
| R-P1-048 | M34 and M36 state that `GET /api/luck/cycles` remains raw supported calculation and must not receive reading, score, or topic overlay fields. |
| R-P1-049 | M37 requires explicit API `year` and states that annual trigger reading is not flow-month, flow-day, daily fortune, or event prediction. |
| R-P1-050 | M38 requires topic overlays to consume shared timeline signals and preserve finance, relationship, family, and career forbidden-claim boundaries. |
| R-P2-006 | M35-M39 require professional terms, plain-language explanation, and boundary reminders in every timeline reading. |

## 3.6 LOOP-099 Risk Evidence

| Risk ID | LOOP-099 Evidence |
| --- | --- |
| R-P1-046 | `audit_timeline_text()` combines base audit and timeline-specific forbidden patterns; `timeline_audit_rejects_deterministic_claims` rejects deterministic money/career language. |
| R-P1-047 | `build_timeline_foundation()` composes ten-god, five-element, hidden-stem, branch-relation, and luck/year overlay signals instead of 10 x 12 or 60-jiazi static fortune tables; `major_luck_and_annual_overlay_is_compositional` guards this. |
| R-P1-048 | M35 added only `backend/src/domain/timeline.rs`; no `backend.api.luck` route or `GET /api/luck/cycles` DTO was changed. |
| R-P1-049 | `annual_trigger_requires_explicit_pillar_input` proves annual-trigger signals appear only when an explicit annual pillar is supplied to the internal engine; no API year default was added. |
| R-P1-050 | Timeline output remains internal and topic-neutral in M35; topic overlay is still deferred to M38, so no relationship/wealth/family/career advice surface was introduced. |
| R-P2-006 | `PlainReading` stores professional text, plain-language text, and boundary reminder; `timeline_foundation_builds_traceable_signals_without_public_score` asserts all readings include `白话说` and no public score terms. |

## 3.7 LOOP-100 Risk Evidence

| Risk ID | LOOP-100 Evidence |
| --- | --- |
| R-P1-046 | `luck_reading_report_is_restricted_traceable_and_scoreless` and app-layer report tests assert M36 output is restricted, audited, and excludes deterministic forbidden text such as `必然发财` and full flow-month claims. |
| R-P1-047 | `build_major_luck_stage_foundation()` reuses the compositional signal/evidence/readings engine instead of static 60-jiazi fortune tables; the M36 test requires ten-god, five-element, hidden-stem, and trace evidence. |
| R-P1-048 | `luck_cycles_returns_supported_after_m13` now asserts raw `GET /api/luck/cycles` does not contain `luck-reading`, `luck_reading`, `白话说`, or `score_internal`; M36 interpretation is carried only by `/api/charts/report`. |
| R-P1-049 | Frontend and report route pass `reading_year` explicitly for primary luck reading; M37 annual-trigger-reading remains planned and no full annual/month/day route was introduced. |
| R-P1-050 | M36 does not map timeline signals into relationship/wealth/family/career topic lenses; topic overlay remains planned for M38. |
| R-P2-006 | Workbench shows a short current-stage structure summary, while the chart report chapter includes professional wording, `白话说`, and boundary reminders for the selected major-luck stage. |

## 3.8 LOOP-101 Risk Evidence

| Risk ID | LOOP-101 Evidence |
| --- | --- |
| R-P1-046 | `annual_trigger_report_requires_explicit_year_and_is_scoreless` and app-layer report tests assert M37 output is restricted, audited, and excludes deterministic forbidden text such as `必然发财`, full flow-month claims, and daily fortune claims. |
| R-P1-047 | `build_annual_trigger_foundation()` reuses compositional timeline signals/evidence/readings instead of static 60-jiazi fortune tables; the M37 test requires original annual-trigger signals, current-luck signals, and major-luck/year overlay evidence. |
| R-P1-048 | `luck_cycles_returns_supported_after_m13` now asserts raw `GET /api/luck/cycles` does not contain `annual-trigger-reading`, `annual_trigger_reading`, `白话说`, or `score_internal`; M37 interpretation is carried only by `/api/charts/report`. |
| R-P1-049 | `annual_trigger_is_not_inferred_without_explicit_year` proves missing API `year` returns `not_requested`; frontend and report page send current UI year explicitly as `year`. |
| R-P1-050 | M37 does not map timeline signals into relationship/wealth/family/career topic lenses; topic overlay remains planned for M38. |
| R-P2-006 | Workbench shows only a short annual structure summary, while the chart report chapter includes professional wording, `白话说`, and boundary reminders for the selected annual trigger. |

## 3.9 LOOP-102 Risk Evidence

| Risk ID | LOOP-102 Evidence |
| --- | --- |
| R-P1-046 | `topic_timeline_overlay_reuses_shared_engine_for_all_topics` and app-layer M38 tests assert the four topic overlays remain restricted, audited, scoreless, and free of full flow-month/day claims. |
| R-P1-047 | `TopicTimelineOverlay` consumes `build_annual_trigger_foundation()` plus shared `TimelineSignal`/`TimelineEvidence`; topic reports do not introduce 60-jiazi static fortune tables or independent timeline math. |
| R-P1-048 | `luck_cycles_returns_supported_after_m13` now asserts raw `GET /api/luck/cycles` does not contain `topic-timeline-reading`, `topic-timeline-overlay`, `topic_timeline`, or `score_internal`; M38 interpretation is carried only by `/api/charts/topic-report`. |
| R-P1-049 | Topic timeline overlay still requires explicit topic-report `year=YYYY`; no silent current-year inference, flow-month, flow-day, daily fortune, or event timeline route was added. |
| R-P1-050 | The overlay maps shared evidence through relationship/wealth/family/career lenses and app tests assert no public score or forbidden high-risk terms; full reports get `本专题的大运流年`, while the workbench remains structure-signal only. |
| R-P2-006 | Every M38 overlay chapter includes `专业解释`, `白话解释`, and `边界提醒`; M39 can focus on UI readability rather than inventing new calculation rules. |

## 3.10 LOOP-103 Risk Evidence

| Risk ID | LOOP-103 Evidence |
| --- | --- |
| R-S0-002 | M39 changed frontend presentation only and did not update `/api/capabilities`; frontend tests assert the timeline report UI exists without adding a new capability surface. |
| R-P1-046 | Report-page evidence panels display backend `professional`, `plain`, and `boundary` readings; the UI adds no deterministic future text, score, or event prediction. |
| R-P1-047 | M39 consumes existing backend `signals`, `evidence`, `readings`, `trace`, and `warnings` instead of adding client-side 60-jiazi or 10 x 12 static interpretation tables. |
| R-P1-048 | Workbench only shows current major-luck short summary, visible years, and evidence counts; raw `GET /api/luck/cycles` remains untouched. |
| R-P1-049 | Main and topic report pages expose explicit year controls that refresh URLs with `reading_year` and/or `year`; no silent API year inference or flow-month/day route was introduced. |
| R-P1-050 | Topic report UI labels the overlay as topic evidence and boundary reminder only; it does not turn relationship, wealth, family, or career overlays into advice. |
| R-P2-006 | `timeline-report-guide`, `timeline-evidence-detail`, and mobile one-column CSS make professional/plain/boundary text and evidence more scannable for beginner users. |

## 3.11 LOOP-104 Risk Evidence

| Risk ID | LOOP-104 Evidence |
| --- | --- |
| R-S0-002 | M40 adds only tests/governance and no `/api/capabilities` entry; app-layer M40 checks assert `luck-reading`, `annual-trigger-reading`, and `topic-timeline-reading` remain restricted rather than supported. |
| R-P1-046 | `m40_timeline_public_quality_gate_covers_golden_samples` sweeps main report, unknown-hour report, annual year-only report, and all four topic overlays for deterministic wealth, romance, family, career, result-guarantee, and certainty terms. |
| R-P1-047 | `m40_timeline_quality_gate_keeps_compositional_output_bounded` caps lexicon size and per-draft signal/evidence/reading/text volume, preserving the shared compositional engine instead of static 10 x 12 or 60-jiazi text expansion. |
| R-P1-048 | The M40 app-layer gate rechecks raw `GET /api/luck/cycles` for absence of `luck_reading`, `annual_trigger_reading`, `topic-timeline-overlay`, professional/plain text, `score_internal`, and 0-100 terms. |
| R-P1-049 | M40 public and frontend source gates reject `流月运势`, `流日运势`, and `每日运势`; report UI only uses negative boundary wording for out-of-scope flow-month/day/event claims. |
| R-P1-050 | The four topic overlay samples remain under topic-report carrier with explicit `year`, shared `timeline-core-v1` source, forbidden-output audit, no score, and no relationship/finance/family/career advice terms. |
| R-P2-006 | Domain M40 gate requires every timeline reading draft to include professional wording, plain-language wording, and boundary wording; frontend M40 test keeps report/topic boundary text visible. |

## 3.12 LOOP-105 Risk Evidence

| Risk ID | LOOP-105 Evidence |
| --- | --- |
| R-S0-002 | LOOP-105 changes frontend state/rendering only and does not update `/api/capabilities`; capability matrix remains 10 supported, 14 restricted, 0 planned. |
| R-P1-040 | Recalculating the chart now clears stale topic output, preventing an old topic/year/time reading from appearing under a new chart input. |
| R-P1-049 | Visible report/workbench copy localizes internal timeline identifiers instead of exposing them as user-facing capability claims; full flow-month/day claims remain absent. |
| R-P1-050 | Async topic responses are version-guarded, so a previous topic request cannot repopulate stale relationship/wealth/family/career overlay text after chart input changes. |
| R-P2-006 | Frontend tests and browser verification cover the cleared topic panel and localized report chrome, improving beginner readability without adding a new interpretation layer. |

## 3.13 LOOP-106 Risk Evidence

| Risk ID | LOOP-106 Evidence |
| --- | --- |
| R-S0-002 | LOOP-106 changes wording/tests/governance only and does not update `/api/capabilities`; capability matrix remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | Timeline reading output keeps forbidden-output audit and no-score/no-overclaim checks while replacing stiff labels with natural structure wording. |
| R-P1-047 | The new domain quality gate checks generated `TimelineReadingDraft` text without adding static 60-jiazi or 10 x 12 interpretation tables. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; app tests still assert no timeline reading text, score, or topic overlay leaks into raw luck output. |
| R-P1-049 | Explicit selected-year wording now uses concrete year text such as `2026年年柱丙午` in generated readings; no silent year default or flow-month/day route was added. |
| R-P1-050 | Topic overlay blocks remove internal-engine wording from natural text while preserving trace source internally for governance; no relationship/finance/family/career advice was introduced. |
| R-P2-006 | `assert_timeline_copy_quality` rejects stiff labels, generic selected-year wording, `用户` address, backend/frontend copy, internal engine ids, and sentence patterns like `主题里` / `读盘时`; frontend source tests reject hard `专业解释`/`白话解释` labels in public timeline UI. |

## 3.14 LOOP-107 Risk Evidence

| Risk ID | LOOP-107 Evidence |
| --- | --- |
| R-S0-002 | LOOP-107 changes only `backend/src/domain/timeline.rs` and governance docs; no `/api/capabilities`, route, DTO, frontend surface, supported promotion, public score, or raw `luck-cycles` mutation is introduced. |
| R-P1-046 | Generated timeline readings remain under `audit_timeline_text`; the new copy avoids blocked deterministic-claim patterns and keeps boundary text on every reading. |
| R-P1-047 | `timeline_lexicon_copy_is_natural_and_guarded` requires 28-40 compositional lexicon entries, preventing a 60-jiazi or 10 x 12 static text explosion while adding five-element relation coverage. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains untouched; dictionary optimization is consumed only by the existing report-carried restricted timeline readings. |
| R-P1-049 | No flow-month, flow-day, daily fortune, event schedule, or silent year default is added; annual wording continues to use explicit year input where the caller provides it. |
| R-P2-006 | `assert_timeline_copy_quality` now also checks professional/plain text density, direct `您` address, stiff-copy patterns, and generated text; `timeline_lexicon_copy_is_natural_and_guarded` checks lexicon entries for natural copy, density, duplicate ids, and internal-id leakage. |

## 3.15 LOOP-108 Risk Evidence

| Risk ID | LOOP-108 Evidence |
| --- | --- |
| R-S0-002 | LOOP-108 changes report copy/tests/governance only and does not update `/api/capabilities`, route shape, DTO shape, supported status, public score, or raw `GET /api/luck/cycles`. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | The app-layer public response gate now covers the final main chart report body and all four topic report bodies, rejecting deterministic claims, public score wording, stiff labels, generic-year wording, and internal engineering copy. |
| R-P1-047 | LOOP-108 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, or event-prediction tables; it only hardens existing compositional report text. |
| R-P1-048 | Raw `GET /api/luck/cycles` stays calculation-only; the report-level gate is applied to `GET /api/charts/report` and `GET /api/charts/topic-report` samples, not to raw luck payload mutation. |
| R-P1-049 | Final report copy now prefers concrete selected-year wording where a year exists and rejects generic selected-year phrases; no silent year default or new time-granularity route is added. |
| R-P1-050 | The four topic reports remain restricted interpretive reports with disclaimer, warnings, and no advice/guarantee wording; the workbench structure-signal boundary remains unchanged. |
| R-P2-006 | Main chart and topic report templates were rewritten toward natural `您`-addressed explanatory prose, and the shared app-layer gate prevents future regressions into machine-like labels such as hard professional/plain headings in visible report bodies. |

## 3.16 LOOP-109 Risk Evidence

| Risk ID | LOOP-109 Evidence |
| --- | --- |
| R-S0-002 | M41 changes only relationship-report narrative assembly, frontend relationship report rendering, tests, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-042 | Relationship-report now carries exactly six narrative body blocks and app/domain tests assert the block order, no public `score_internal`, no deterministic romance terms, and no standalone `topic-timeline-overlay` body block for relationship. |
| R-P1-046 | The public response gate still rejects deterministic romance, wealth, family, career, result-guarantee, public-score, and full flow-month/day claims after the relationship copy rewrite. |
| R-P1-047 | M41 reuses existing TopicReport and shared timeline evidence; it does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, or event-prediction tables. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; relationship timeline wording is carried only by the restricted topic-report route with explicit `year`. |
| R-P1-050 | Relationship can include low-risk relationship-rhythm suggestions such as observing stable response and communication boundaries, but tests and forbidden-output audit continue to block deterministic marriage, separation, partner identity, and event claims. |
| R-P2-006 | The relationship full report page now renders narrative body blocks only, preserving `signals` and `trace` for the time guide while removing machine-like extra evidence chapters from the relationship body. |

## 3.17 LOOP-110 Risk Evidence

| Risk ID | LOOP-110 Evidence |
| --- | --- |
| R-S0-002 | M42 changes only `relationship-report` copy, app/domain tests, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-042 | Relationship-report keeps the M41 six-block body order and `topic-timeline-overlay` remains in signals/trace only; the new M42 tests add banned machine wording such as 标记为已引动、共享时间线、筛出、当前提取结果、当前关系 and 基础阅读. |
| R-P1-046 | The existing public response gate and forbidden-output audit still reject deterministic romance claims, public score wording, and high-risk event language after the human-copy rewrite. |
| R-P1-047 | M42 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, or event-prediction tables; it only rewrites relationship-report wording around existing computed signals. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; the relationship annual wording is still carried only by the restricted topic-report route with explicit `year`. |
| R-P1-050 | Relationship advice remains low-risk and observational: stable response, clear boundary, communication rhythm, and reality承接; tests continue to block deterministic marriage, separation, partner identity, and event claims. |
| R-P2-006 | Real generated output was inspected at `target/report-polish-samples/relationship.txt`; the sample has six blocks, no M42 banned wording, no ASCII words, no equality-count phrasing, and no potential internal English leakage. |

## 3.18 LOOP-111 Risk Evidence

| Risk ID | LOOP-111 Evidence |
| --- | --- |
| R-S0-002 | M43 changes only visible report copy, app/domain tests, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | The public response gate now covers M43 banned wording for main chart and topic reports: visible report text rejects internal English, machine copy, equality-count evidence, pipe-form evidence, rule-version wording, and backend/frontend leakage. |
| R-P1-047 | M43 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, or event-prediction tables; it only rewrites existing computed evidence into readable Chinese. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M43 only affects `GET /api/charts/report` and `GET /api/charts/topic-report` assembled report prose. |
| R-P1-049 | M43 preserves explicit year input and rewrites visible copy to concrete-year phrasing such as `2026 年`; no silent year default or new time-granularity route is added. |
| R-P1-050 | Wealth, family, and career reports remain restricted interpretive reports with disclaimer and no advice/guarantee wording; no financial, family-event, or career-outcome promise is introduced. |
| R-P2-006 | Real generated outputs were inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five samples scan as 0 forbidden hits and 0 ASCII words after M43. |

## 3.19 LOOP-112 Risk Evidence

| Risk ID | LOOP-112 Evidence |
| --- | --- |
| R-S0-002 | M44 changes only relationship-report copy, shared visible evidence formatting, tests, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-042 | Relationship-report keeps the M41 six-block body order and M42 human-copy gate; M44 adds guards against repeated fixed openers and bare relation-trigger wording such as `被冲牵动`, `被合牵动`, and `被刑害牵动`. |
| R-P1-046 | Public report copy still rejects deterministic romance claims, public score wording, and high-risk event language; the new quoted relation terms explain structure without turning `冲` or `合` into event prediction. |
| R-P1-047 | M44 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, or event-prediction tables; it only rewrites existing computed evidence into clearer Chinese. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M44 affects only assembled report prose carried by `GET /api/charts/report` and `GET /api/charts/topic-report`. |
| R-P1-050 | Relationship advice remains low-risk and observational; M44 improves wording around relation rhythm while tests continue to block deterministic marriage, separation, partner identity, and high-risk reality decisions. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five samples scan as 0 M44 forbidden hits, 0 ASCII words, and 0 year-spacing hits such as `2026 年`, and `relationship.txt` now includes quoted terms such as `被"冲"牵动` and `形成"六冲"`. |

## 3.20 LOOP-113 Risk Evidence

| Risk ID | LOOP-113 Evidence |
| --- | --- |
| R-S0-002 | M45 changes only visible report copy, timeline display labels, tests, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | Public report copy now rejects system-tone phrases such as `当前算法`, `系统给出`, `综合评分`, count-table phrases such as `相关信号共` and `未见明显显性信号`, and old M44 relationship regressions. |
| R-P1-047 | M45 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, or event-prediction tables; it only rewrites existing computed evidence into clearer Chinese. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M45 affects only assembled report prose and public timeline labels carried by `GET /api/charts/report` and `GET /api/charts/topic-report`. |
| R-P1-050 | Wealth, family, career, and relationship reports remain observational and restricted; no financial outcome, family event, career result, romance event, partner identity, or high-risk reality decision is introduced. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five samples scan as 0 M45 forbidden hits and 0 ASCII words, and `main.txt` now renders luck evidence as `第一运·丙子`. |

## 3.21 LOOP-114 Risk Evidence

| Risk ID | LOOP-114 Evidence |
| --- | --- |
| R-S0-002 | M46 changes only visible report copy, tests, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | Public report copy now rejects list/table/debug-like phrases such as `盘中可用的时间线索`, `重点看的牵动`, `关键牵动是`, `这张命盘里的`, `结构上被点亮`, and `出现 4 处`, in addition to the older M41-M45 forbidden terms. |
| R-P1-046 | Final M46 samples also scan JSON/trace evidence for bare relation wording such as `形成六冲` and `被冲牵动`; the public relation copy now keeps `形成"六冲"` / `被"冲"牵动` style wording where those terms are visible. |
| R-P1-049 | M46 replaced negative `完整流月` wording with `逐月细分` boundary wording so no-flow-month scope remains clear without tripping public output audits. |
| R-P1-047 | M46 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, or event-prediction tables; it only rewrites existing computed evidence into clearer Chinese. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M46 affects only assembled report prose carried by `GET /api/charts/report` and `GET /api/charts/topic-report`. |
| R-P1-050 | Relationship advice remains low-risk and observational; M46 improves count/list wording while tests continue to block deterministic marriage, separation, partner identity, and high-risk reality decisions. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five samples scan as 0 M46 forbidden hits and 0 ASCII words. |

## 3.22 LOOP-115 Risk Evidence

| Risk ID | LOOP-115 Evidence |
| --- | --- |
| R-S0-002 | M47 changes only `relationship-report` visible copy, tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | Relationship-report keeps the M41 six-block body order, M42 human-copy gate, and M44 quoted relation-term rule; M47 adds a stricter golden-sample gate against `不作主线`, `有一处落点`, and similar count-field leakage in the relationship body. |
| R-P1-047 | M47 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, partner identity, marriage timing, or event-prediction tables; it only rewrites existing computed relationship evidence into clearer Chinese. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M47 affects only assembled relationship report prose carried by `GET /api/charts/topic-report?topic=relationship&year=YYYY`. |
| R-P1-050 | Relationship advice remains low-risk and observational; M47 improves relationship qualities around attraction, commitment, pressure, boundary, safety, stable response, and real-world承接 while tests continue to block deterministic romance claims and high-risk reality decisions. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; `relationship.txt` scans as 0 M47 forbidden hits and all five JSON samples return top-level audit `passed`. |

## 3.23 LOOP-116 Risk Evidence

| Risk ID | LOOP-116 Evidence |
| --- | --- |
| R-S0-002 | M48 changes only visible report copy, tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | Wealth, family, and career report bodies now reject `不作主线`, `有一处落点`, `有两处落点`, `有三处落点`, `参与这组结构`, and `这组结构说明`, extending the M47 count-field rule beyond relationship-report. |
| R-P1-047 | M48 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, financial outcome, family event, career result, or event-prediction tables; it only rewrites existing computed ten-god evidence into clearer Chinese. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M48 affects only assembled report prose carried by `GET /api/charts/topic-report?topic=...&year=YYYY`. |
| R-P1-050 | Wealth, family, and career readings remain observational and restricted; M48 improves resource, support/responsibility, and career-force wording while tests continue to block deterministic money, family, career, romance, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` scan as 0 M48 forbidden hits and all five JSON samples return top-level audit `passed`. |

## 3.24 LOOP-117 Risk Evidence

| Risk ID | LOOP-117 Evidence |
| --- | --- |
| R-S0-002 | M49 changes only visible annual/timeline report copy, tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | Main chart annual-trigger and topic timeline blocks now reject list-tone phrases such as `主要牵动如下`, `盘面上先看这几股牵动`, `这些牵动只说明`, and bullet evidence such as `· 先看天干`. |
| R-P1-047 | M49 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, financial outcome, family event, career result, or event-prediction tables; it only rewrites existing annual/timeline evidence into clearer Chinese. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M49 affects only assembled report prose carried by `GET /api/charts/report` and `GET /api/charts/topic-report?topic=...&year=YYYY`. |
| R-P1-050 | Annual/timeline readings remain observational and restricted; M49 improves reading-order prose while tests continue to block deterministic money, family, career, romance, flow-month/day, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five scan as 0 M49 forbidden hits and all five JSON samples return top-level audit `passed`. |

## 3.25 LOOP-118 Risk Evidence

| Risk ID | LOOP-118 Evidence |
| --- | --- |
| R-S0-002 | M50 changes only visible wealth/family/career report copy, tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | Wealth/family/career reports now reject explanation-of-template phrases such as `日常读法`, `日常看`, `这些牵动提醒您`, `放回这张命盘看`, `放回家庭结构里`, and `这份报告适合当作`. |
| R-P1-047 | M50 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, financial outcome, family event, career result, or event-prediction tables; it only rewrites existing topic report prose into clearer Chinese. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M50 affects only assembled report prose carried by `GET /api/charts/topic-report?topic=wealth|family|career&year=YYYY`. |
| R-P1-050 | Three-topic readings remain observational and restricted; M50 improves advice cohesion while tests continue to block deterministic money, family, career, romance, flow-month/day, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five scan as 0 M50 forbidden hits and all five JSON samples return top-level audit `passed`. |

## 3.26 LOOP-119 Risk Evidence

| Risk ID | LOOP-119 Evidence |
| --- | --- |
| R-S0-002 | M51 changes only visible main chart report copy, one career bridge phrase, tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | Main report now rejects teaching-manual phrases such as `这一章看的是`, `这一章先把`, `放到日常理解里`, `最适合当作`, and `可以先这样理解`; topic reports also reject the shared `这一章看的是` regression. |
| R-P1-047 | M51 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, financial outcome, family event, career result, or event-prediction tables; it only rewrites existing report prose into clearer Chinese. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M51 affects only assembled report prose carried by `GET /api/charts/report` and one existing career report paragraph. |
| R-P1-050 | Main chart and topic readings remain observational and restricted; M51 improves tone cohesion while tests continue to block deterministic money, family, career, romance, flow-month/day, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five scan as 0 M51 forbidden hits and all five JSON samples return top-level audit `passed`. |

## 3.27 LOOP-120 Risk Evidence

| Risk ID | LOOP-120 Evidence |
| --- | --- |
| R-S0-002 | M52 changes only visible main chart and topic report copy/order, tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | M52 public-copy gates reject `基本脉络如下`, `第一优先`, `第二优先`, `原局引动主要看`, `先看这几层关系`, `不能只看流年`, `不必急着找事件结论`, `这一章只说明`, `时间气候可以按这个顺序读`, `当前资料可以按完整四柱合参`, and `这一年`. |
| R-P1-047 | M52 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, financial outcome, family event, career result, or event-prediction tables; it only rewrites existing report prose and block ordering into clearer Chinese. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M52 affects only assembled report prose carried by `GET /api/charts/report` and `GET /api/charts/topic-report?topic=wealth|family|career&year=YYYY`. |
| R-P1-050 | Main chart and topic readings remain observational and restricted; M52 improves report closeout continuity while tests continue to block deterministic money, family, career, romance, flow-month/day, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five scan as 0 M52 forbidden hits, all five JSON samples return top-level audit `passed`, and wealth/family/career now end with `结论`. |

## 3.28 LOOP-121 Risk Evidence

| Risk ID | LOOP-121 Evidence |
| --- | --- |
| R-S0-002 | M53 changes only visible main chart and topic report copy, tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | M53 public-copy gates reject `偏弱表示这类倾向`, `哪里需要放慢`, `哪里需要承接`, and `读2026年这一层` while preserving M52 stale closeout guards. |
| R-P1-047 | M53 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, financial outcome, family event, career result, or event-prediction tables; it only rewrites existing report prose into clearer Chinese. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M53 affects only assembled report prose carried by `GET /api/charts/report` and `GET /api/charts/topic-report?topic=wealth|family|career&year=YYYY`. |
| R-P1-050 | Main chart and topic readings remain observational and restricted; M53 improves explanation density and topic specificity while tests continue to block deterministic money, family, career, romance, flow-month/day, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five scan as 0 M53/M52 forbidden hits, all five JSON samples return top-level audit `passed`, and wealth/family/career still end with `结论`. |

## 3.29 LOOP-122 Risk Evidence

| Risk ID | LOOP-122 Evidence |
| --- | --- |
| R-S0-002 | M54 changes only visible main chart and topic report copy, tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | M54 public-copy gates reject `这一章会把`, `牵动会先落在这些位置`, `2026年的时间气候`, `先从这些层次落下去看`, `先看天干`, and `再看五行关系` while preserving M52/M53 stale wording guards. |
| R-P1-047 | M54 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, financial outcome, family event, career result, or event-prediction tables; it only rewrites existing annual/timeline detail prose into clearer Chinese. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M54 affects only assembled report prose carried by `GET /api/charts/report` and `GET /api/charts/topic-report?topic=wealth|family|career&year=YYYY`. |
| R-P1-050 | Main chart and topic readings remain observational and restricted; M54 improves annual/timeline detail warmth while tests continue to block deterministic money, family, career, romance, flow-month/day, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five scan as 0 M54/M53/M52 forbidden hits, all five JSON samples return top-level audit `passed`, and wealth/family/career still end with `结论`. |

## 3.30 LOOP-123 Risk Evidence

| Risk ID | LOOP-123 Evidence |
| --- | --- |
| R-S0-002 | M55 changes only topic-report luck context selection, visible report prose, tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | M55 public-copy gates reject `大运首段`, fixed `年龄段约为1至10岁` / `约在 1 至 10 岁` wording, `天干处先露出`, `月支这一处`, and `日支这一处` while preserving M52-M54 stale wording guards. |
| R-P1-047 | M55 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, financial outcome, family event, career result, or event-prediction tables; it fixes selected-year current-luck consistency and decomposes existing annual/timeline detail prose. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M55 affects only assembled report prose and topic-report internal use of real luck-cycle context. |
| R-P1-050 | Main chart and topic readings remain observational and restricted; M55 improves current-luck consistency while tests continue to block deterministic money, family, career, romance, flow-month/day, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five scan as 0 M55/M54/M53/M52 forbidden hits, all five JSON samples return top-level audit `passed`, and relationship/wealth/family/career end with `结论`. |

## 3.31 LOOP-124 Risk Evidence

| Risk ID | LOOP-124 Evidence |
| --- | --- |
| R-S0-002 | M56 changes only visible relationship/wealth/family/career conclusion prose, tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | M56 public-copy gates reject stale conclusion-template phrases including `在这份金钱专项里`, `在这份家庭专项里`, `在这份事业专项里`, `表达与安全感则落在日常相处里`, `以目前资料来看，这份情感专项可以把重点放在`, and `在同一张桌上慢慢理清` while preserving M52-M55 stale wording guards. |
| R-P1-047 | M56 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, financial outcome, family event, career result, or event-prediction tables; it only compresses repeated conclusion prose and makes existing topic closeouts more distinct. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M56 affects only assembled report prose carried by `GET /api/charts/topic-report`. |
| R-P1-050 | Relationship, wealth, family, and career readings remain observational and restricted; M56 improves conclusion specificity while tests continue to block deterministic money, family, career, romance, flow-month/day, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five scan as 0 M56/M55/M54 forbidden hits, all five JSON samples return top-level audit `passed`, and relationship/wealth/family/career end with `结论`. |

## 3.32 LOOP-125 Risk Evidence

| Risk ID | LOOP-125 Evidence |
| --- | --- |
| R-S0-002 | M57 changes only visible main annual-trigger, timeline lexicon, and wealth/family/career topic timeline prose plus tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | M57 public-copy gates reject stale staged timeline phrases including `年度本身先露出的`, `流年天干把十神主题推到台前`, `五行关系继续说明力量怎样靠近`, `不是罗列符号`, `推到台前`, `走到台前`, and `拿到台前` while preserving M49-M56 report-copy gates. |
| R-P1-047 | M57 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, event-prediction, financial outcome, family event, career result, or partner-identity tables; it only rewrites how existing timeline evidence is explained. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M57 affects only assembled report prose carried by `GET /api/charts/report` and `GET /api/charts/topic-report`. |
| R-P1-050 | Main, wealth, family, career, and relationship readings remain observational and restricted; M57 improves beginner-readable timeline prose while tests continue to block deterministic money, family, career, romance, flow-month/day, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five scan as 0 M57/M56/M55 forbidden hits, all five JSON samples return top-level audit `passed`, and relationship/wealth/family/career end with `结论`. |

## 3.33 LOOP-126 Risk Evidence

| Risk ID | LOOP-126 Evidence |
| --- | --- |
| R-S0-002 | M58 changes only visible main-report prose in `十神关系`, `大运走势`, and `年度引动` plus tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | M58 public-copy gates reject long-section and professional-label regressions including `读这一章时`, `这条线已经进入命盘视野`, `这条十神线索`, `命理结构上，当前阶段大运`, `五行流向上，`, `藏干里，`, and `地支关系上，` while preserving M51-M57 report-copy gates. |
| R-P1-047 | M58 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, event-prediction, financial outcome, family event, career result, or partner-identity tables; it only condenses existing visible report prose. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M58 affects only assembled report prose carried by `GET /api/charts/report`. Restricted trace/evidence/readings remain available in the report payload. |
| R-P1-050 | Main, wealth, family, career, and relationship readings remain observational and restricted; M58 improves beginner-readable main report density while tests continue to block deterministic money, family, career, romance, flow-month/day, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; all five scan as 0 M58/M57/M56 forbidden hits, all five JSON samples return top-level audit `passed`, and relationship/wealth/family/career end with `结论`. |

## 3.34 LOOP-127 Risk Evidence

| Risk ID | LOOP-127 Evidence |
| --- | --- |
| R-S0-002 | M59 changes only visible wealth/family/career middle-chapter prose plus tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | M59 public-copy gates reject stale textbook middle-chapter phrases including `财星分正财和偏财`, `传统上会把`, `印星在家庭专项里主要看`, `比劫在家庭专项里看`, `财官在家庭专项里不解释`, `官杀代表责任`, `食伤代表表达`, `比劫代表协作`, `技能表达：`, `资源落地：`, `协作竞争：`, `同辈边界：`, `表达方式：`, `责任方面，`, `承接方面，`, and `支持与约束方面` while preserving M48-M58 report-copy gates. |
| R-P1-047 | M59 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, event-prediction, financial outcome, family event, career result, or partner-identity tables; it only rewrites existing visible topic-report prose. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M59 affects only assembled report prose carried by `GET /api/charts/topic-report`. Restricted trace/evidence/readings remain available in the report payload. |
| R-P1-050 | Wealth, family, career, relationship, and main chart readings remain observational and restricted; M59 improves beginner-readable topic middle chapters while tests continue to block deterministic money, family, career, romance, flow-month/day, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; wealth/family/career sample scan confirmed all M59 required anchors present and M59 forbidden textbook phrases absent, and all five JSON samples return top-level audit `passed`. |

## 3.35 LOOP-128 Risk Evidence

| Risk ID | LOOP-128 Evidence |
| --- | --- |
| R-S0-002 | M60 changes only visible wealth/family/career topic-timeline prose plus tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | M60 public-copy gates reject stale topic-timeline scaffolding including `从「金钱」专项来看`, `从「家庭」专项来看`, `从「事业」专项来看`, `把2026年放进`, `十神与五行这一层`, `五行相处的方式提示`, `本段把它作为阶段背景参考`, and `年度线索要回到` while preserving M47-M59 report-copy gates. |
| R-P1-047 | M60 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, event-prediction, financial outcome, family event, career result, or partner-identity tables; it only rewrites existing visible topic-timeline prose. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M60 affects only assembled report prose carried by `GET /api/charts/topic-report`. Restricted trace/evidence/readings remain available in the report payload. |
| R-P1-050 | Wealth, family, career, relationship, and main chart readings remain observational and restricted; M60 improves beginner-readable topic timeline chapters while tests continue to block deterministic money, family, career, romance, flow-month/day, and high-risk reality claims. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; wealth/family/career sample scan confirmed all M60 required anchors present and M60 forbidden timeline scaffolds absent, and all five JSON samples return top-level audit `passed`. |

## 3.36 LOOP-129 Risk Evidence

| Risk ID | LOOP-129 Evidence |
| --- | --- |
| R-S0-002 | M61 changes only visible topic-report prose, tests, regenerated samples, and governance docs; no `/api/capabilities`, route shape, top-level DTO, supported status, public score, or raw `GET /api/luck/cycles` mutation is introduced. Runtime remains 10 supported, 14 restricted, 0 planned. |
| R-P1-046 | M61 public-copy gates require early-stage age-context anchors such as `名义年龄约2岁`, `早年阶段`, `资源不是只指钱`, `成长场景`, and `不讨论现实恋爱状态`, while rejecting adult-context regressions including `如果目前单身`, `若已有关系`, `工作场景`, `现实职位高低`, `长期经营`, `现实回报`, and `团队边界`. |
| R-P1-047 | M61 does not add static 60-jiazi, 10 x 12, flow-month, flow-day, daily fortune, event-prediction, financial outcome, family event, career result, partner-identity, or child-outcome prediction tables; it only adapts existing visible prose to selected-year age context. |
| R-P1-048 | Raw `GET /api/luck/cycles` remains calculation-only; M61 affects only assembled report prose carried by `GET /api/charts/topic-report`. Restricted trace/evidence/readings remain available in the report payload. |
| R-P1-050 | Relationship, wealth, family, and career readings remain observational and restricted; early-stage reports now avoid adult romance, income, investment, job, or career-result framing and instead read emotional response, resource sense, stable care, learning tasks, rule sense, and expression training. |
| R-P2-006 | Real generated outputs were regenerated and inspected at `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`; sample scan confirmed M61 required anchors present and adult-context regression phrases absent, and all four topic JSON samples return top-level audit `passed`. |

## 4. 风险升级规则

- P2 风险影响 supported 能力可信度时升级为 P1。
- P1 风险涉及隐私泄露、错误命盘、确定性伤害文案时升级为 S0。
- S0 风险未清零不得关闭里程碑。
## M5 Risk Addendum

| Risk ID | Risk | Trigger | Mitigation | Owner |
| --- | --- | --- | --- | --- |
| R-P1-007 | Local volatile storage mistaken for durable storage | Documentation or UI implies account, database, cloud sync, or cross-device persistence | Keep `case-management` and `settings` restricted; ADR 0013 and capability ledger must name local volatile boundary | `backend.api`, `backend.domain.cases`, `backend.domain.settings` |
| R-P1-008 | Restricted share preview mistaken for durable public sharing | Documentation or UI implies permanent links, public directory, account ownership, analytics, or durable tokens | Keep `share-preview` restricted; ADR 0014 requires hash-only local volatile tokens, redacted DTOs, noindex, revocation, and unavailable-response unification | `backend.api.share`, `backend.domain.share`, `frontend.share-preview` |
| R-P1-009 | Frontend workspace overclaims backend capability | UI presents luck cycles, durable sharing, cloud sync, account storage, glossary, true solar time, timezone history, range expansion, or astronomy replacement as available | Frontend capability labels must come from `/api/capabilities` or capability ledger; M7 closeout keeps workspace restricted | `frontend.ui`, `frontend.api`, `governance.capability-ledger` |
| R-P1-010 | Release candidate overclaims mixed capability status | Release notes, README, or UI implies planned/restricted capabilities are fully supported | M8 release checker, release document, capability ledger, module tree, engineering tree, and closeout must freeze supported/restricted/planned labels together | `governance.release`, `tools.governance`, `governance.capability-ledger` |
| R-P1-011 | Astronomy preflight mistaken for generated engine evidence | ADR, schema, or template is cited as if a real astronomy table or runtime engine exists | Keep `astronomy-engine` target; M9 preflight checker verifies no generated table is accepted and replacement needs later ADR plus hashes/comparison report | `governance.astronomy-preflight`, `data.generated.astronomy`, `tools.governance` |
| R-P1-012 | Draft generated manifest mistaken for accepted generated data | A manifest JSON exists and is cited as proof of astronomy-engine support | Draft manifest must remain `not_accepted`; checker requires no generation command, no artifact hashes, template-only comparison, and explicit acceptance blockers | `data.generated.astronomy`, `tools.governance`, `governance.capability-ledger` |
| R-P1-013 | M9 pre-closeout audit mistaken for full astronomy-engine closeout | Audit says preflight ready and is cited as if M9 full acceptance passed | `precloseout-audit.json` must keep `full_closeout_allowed=false`, `capability_status=target`, and all full acceptance blockers as missing until real generated evidence exists | `governance.astronomy-preflight`, `data.generated.astronomy`, `tools.governance` |
| R-P1-014 | Implementation plan mistaken for permission to generate or accept artifacts | ADR 0017 or `implementation-plan.json` is cited as if generated rows may be accepted | Implementation plan must remain `planning_only`; checker requires generator contract stage before any generated artifact acceptance or runtime replacement | `governance.astronomy-preflight`, `data.generated.astronomy`, `tools.governance` |
| R-P1-015 | Generator contract mistaken for generated artifacts | Contract lists outputs and hashes and is cited as if those outputs exist | Generator contract must remain `contract_only`; checker and dry-run generator require all planned outputs to stay `not_generated` with no writes or acceptance change | `data.generated.astronomy`, `tools.governance`, `tools.generate-astronomy` |
| R-P1-016 | Source adapter contract mistaken for source integration | Contract lists Horizons/SOFA/SPICE and is cited as if those sources are integrated or queried in gates | Source adapter contract must remain `contract_only`; checker requires runtime_dependency=false, output_claim_allowed=false, and no external API call in full gate | `data.generated.astronomy`, `tools.governance`, `governance.astronomy-preflight` |
| R-P1-017 | Artifact writer dry-run mistaken for generated artifact evidence | Dry-run previews output paths and hash algorithm and is cited as if artifacts or hashes exist | Artifact writer plan must remain `dry_run_only`; checker requires no writes, zero hashes computed, no existing planned artifacts, and no accepted evidence | `data.generated.astronomy`, `tools.governance`, `tools.artifact-writer` |
| R-P1-018 | Comparison runner dry-run mistaken for completed Android-vs-astronomy report | Dry-run binds Android baseline metadata and is cited as if comparison rows were evaluated | Comparison runner plan must remain `dry_run_only`; checker requires rows compared 0, difference rows 0, no writes, and no accepted evidence | `data.generated.astronomy`, `tools.governance`, `tools.compare-astronomy` |
| R-P1-019 | Golden-row readiness mistaken for generated golden cases | Readiness categories are listed and cited as if golden rows or boundary tests exist | Golden readiness plan must remain `readiness_only`; checker requires all categories not generated, blocked, zero generated rows, no writes, and no accepted evidence | `data.generated.astronomy`, `tools.governance`, `tools.golden-cases` |
| R-P1-020 | Replay-test readiness mistaken for executed replay tests | Readiness controls are listed and cited as if old snapshots were replayed | Replay readiness plan must remain `readiness_only`; checker requires replay tests executed 0, controls unexecuted/blocked, replacement disallowed, and no accepted evidence | `data.generated.astronomy`, `tools.governance`, `tools.replay-policy` |
| R-P1-021 | M9 preflight closeout mistaken for full astronomy-engine completion | `44-milestone-09-preflight-closeout.md` or `preflight-closeout-decision.json` is cited as if generated artifacts, hashes, comparison, golden rows, replay tests, or runtime integration exist | Preflight closeout decision must keep full engine closeout=false, generated artifacts accepted=false, replacement=false, runtime change=false, and route real implementation to M10 | `governance.astronomy-preflight`, `data.generated.astronomy`, `governance.roadmap` |
| R-P1-022 | M10 guarded generator entry mistaken for generated astronomy data | `-PrepareImplementation` runs without `-DryRun` and is cited as if artifacts, hashes, or source integration exist | Generator entry must remain `guarded_entrypoint_only`; checker verifies generation blocked, source snapshot manifest metadata-only, no writes, hashes=0, acceptance unchanged, runtime unchanged, and target capability status | `tools.generate-astronomy`, `data.generated.astronomy`, `tools.governance` |
| R-P1-023 | Source snapshot manifest boundary mistaken for source materialization | Schema/plan lists source ids and is cited as if source snapshots, local routines, kernels, or validation samples exist | Source snapshot manifest plan must remain `manifest_materialized_metadata_only`; checker and dry-run verify metadata manifest present, source snapshots materialized 0, generated artifacts 0, hashes 0, no writes, and target capability status | `data.generated.astronomy`, `tools.source-snapshot`, `tools.governance` |
| R-P1-024 | Metadata-only source snapshot manifest mistaken for source payload evidence | `source-snapshots/source-snapshot-manifest.json` exists and is cited as if GB/T/JPL/SOFA/SPICE payloads were materialized | Manifest must remain `metadata_only_no_source_payload`; every source must remain `not_materialized`, runtime_dependency=false, output_claim_allowed=false, and generated artifacts/hashes remain absent | `data.generated.astronomy`, `tools.source-snapshot`, `tools.governance` |
| R-P1-025 | Source payload policy mistaken for source payload materialization | Payload paths are listed and cited as if source payload files or source hashes exist | Policy must remain `payload_policy_only`; checker and dry-run verify payload directory absent, payload files 0, source payloads 0, payload hashes 0, generated artifacts 0, acceptance unchanged, and runtime unchanged | `data.generated.astronomy`, `tools.source-payload`, `tools.governance` |
| R-P1-026 | Source payload schemas mistaken for materialized source evidence | Schema files exist and are cited as if GB/T rules, JPL samples, SOFA routines, or SPICE kernels were captured locally | Schemas must remain `schema_only`; checker and dry-run verify schema/source/kind matching, payload directory absent, payload files 0, source payloads 0, payload hashes 0, generated artifacts 0, acceptance unchanged, runtime unchanged, and target capability status | `data.generated.astronomy`, `tools.source-payload`, `tools.governance` |
| R-P1-027 | Source capture procedure mistaken for captured source payloads | Capture steps and evidence fields exist and are cited as if source payload files, source hashes, or external validation samples were captured | Procedure must remain `procedure_only`; checker and dry-run verify capture not_started, payload directory absent, payload files 0, source payloads 0, payload hashes 0, external calls false, generated artifacts 0, acceptance unchanged, runtime unchanged, and target capability status | `data.generated.astronomy`, `tools.source-capture`, `tools.governance` |
| R-P1-028 | First source payload decision mistaken for selected payload materialization | `naif-cspice` is selected and cited as if the selected payload file, source hash, toolkit, or kernel exists locally | Decision must remain `decision_only`; checker and dry-run verify single source only, payload directory absent, selected payload absent, source payloads 0, source hashes 0, external calls false, generated artifacts 0, acceptance unchanged, runtime unchanged, and target capability status | `data.generated.astronomy`, `tools.source-payload-decision`, `tools.governance` |
| R-P1-029 | Selected source payload preflight mistaken for selected payload existence | Next-loop selected-source-only scope is cited as if `naif-cspice-kernel-boundary.json`, source hash, toolkit, or kernel exists locally | Preflight must remain `preflight_only`; checker and dry-run verify payload directory absent, selected payload absent, existing payload files 0, source hashes 0, external calls false, generated artifacts 0, acceptance unchanged, runtime unchanged, and target capability status | `data.generated.astronomy`, `tools.selected-source-preflight`, `tools.governance` |
| R-P1-030 | Selected source payload materialization mistaken for generated astronomy or CSPICE integration | `naif-cspice-kernel-boundary.json` or its sha256 is cited as if SPICE kernels, CSPICE toolkit files, generated astronomy rows, accepted manifest evidence, runtime integration, or `astronomy-engine` support exists | Checker requires exactly one source-boundary payload, forbids unselected payload files, generated artifacts, generated artifact hashes, manifest acceptance changes, runtime changes, Android replacement, CSPICE toolkit/kernel integration claims, and capability promotion | `data.generated.astronomy`, `tools.source-payload`, `tools.governance` |
| R-P1-031 | Remaining source strategy mistaken for SOFA payload materialization | `remaining-source-payload-strategy.json` selects `iau-sofa-ansi-c` and is cited as if SOFA routine/version payload, runtime dependency, generated rows, or supported astronomy engine exists | Checker and dry-run require `strategy_decision_only`, existing payload files 1, new payload writes 0, new source hashes 0, generated artifacts 0, external calls false, acceptance unchanged, runtime unchanged, and target capability status | `data.generated.astronomy`, `tools.remaining-source-strategy`, `tools.governance` |
| R-P1-032 | Selected IAU SOFA preflight mistaken for routine materialization or integration | `selected-iau-sofa-payload-materialization-preflight.json` or its dry-run is cited as if `iau-sofa-routine-version.json`, SOFA routine integration, generated rows, runtime dependency, or supported astronomy engine exists | Checker and dry-run require `preflight_only`, selected payload exists false, existing payload files 1, new payload writes 0, new source hashes 0, generated artifacts 0, external calls false, acceptance unchanged, runtime unchanged, and target capability status | `data.generated.astronomy`, `tools.selected-iau-sofa-preflight`, `tools.governance` |
| R-P1-033 | Selected IAU SOFA materialization mistaken for SOFA routine integration | `iau-sofa-routine-version.json` or its sha256 is cited as if SOFA source is vendored, compiled, linked, used at runtime, generated rows exist, or `astronomy-engine` is supported | Checker requires exactly two source-boundary payloads, forbids JPL/GB/T payloads, generated artifacts, generated artifact hashes, manifest acceptance changes, runtime changes, Android replacement, SOFA integration claims, and capability promotion | `data.generated.astronomy`, `tools.source-payload`, `tools.governance` |
| R-P1-034 | Post-IAU remaining source strategy mistaken for JPL/GB payload materialization | `post-iau-remaining-source-payload-strategy.json` selects JPL Horizons and is cited as if JPL query snapshots, GB/T rule references, generated rows, or supported astronomy engine exist | Checker and dry-run require `strategy_decision_only`; after LOOP-052 JPL materialization must be proven only by selected JPL evidence, while GB/T payload remains absent, generated artifacts 0, external calls false, acceptance unchanged, runtime unchanged, and target capability status | `data.generated.astronomy`, `tools.post-iau-remaining-source-strategy`, `tools.governance` |
| R-P1-035 | Selected JPL Horizons preflight mistaken for online query or payload evidence | `selected-jpl-horizons-payload-materialization-preflight.json` or its dry-run is cited as if an online JPL query ran in the full gate, generated rows exist, or `astronomy-engine` is supported | Checker and dry-run require `preflight_only`; after LOOP-052 selected payload exists true but query execution allowed in full gate false, response bodies false, external calls false, generated artifacts 0, acceptance unchanged, runtime unchanged, and target capability status | `data.generated.astronomy`, `tools.selected-jpl-horizons-preflight`, `tools.governance` |
| R-P1-036 | Selected JPL Horizons payload mistaken for online query execution or astronomy support | `jpl-horizons-validation-samples.json` or its sha256 is cited as if JPL response bodies were captured, an online JPL query ran in the full gate, generated rows exist, runtime network dependency is enabled, or `astronomy-engine` is supported | Checker and dry-run require exactly three source-boundary payloads, selected JPL payload exists true, new source payload writes 1, new source hash 1, query execution allowed in full gate false, external calls false, response bodies materialized false, generated artifacts 0, acceptance unchanged, runtime unchanged, Android replacement false, and target capability status | `data.generated.astronomy`, `tools.selected-jpl-horizons-preflight`, `tools.governance` |
| R-P1-037 | Selected GB/T preflight mistaken for rule payload materialization | `selected-gb-t-payload-materialization-preflight.json` or its dry-run is cited as if GB/T rule references, a GB/T source hash, generated rows, Android replacement, or supported astronomy engine exist | Checker and dry-run require `preflight_only`, selected GB/T payload exists false, existing payload files 3, source-reference capture false for this loop, payload materialization false for this loop, new source payload writes 0, new source hashes 0, generated artifacts 0, acceptance unchanged, runtime unchanged, Android replacement false, and target capability status | `data.generated.astronomy`, `tools.selected-gb-t-preflight`, `tools.governance` |
| R-P1-038 | Selected GB/T payload mistaken for copied standard text or implemented calendar rules | `gb-t-33661-2017-rule-reference.json` or its sha256 is cited as if GB/T standard text was copied, Chinese-calendar rules were implemented, generated rows exist, Android baseline was replaced, or `astronomy-engine` is supported | Checker and dry-run require exactly four source-boundary payloads, selected GB/T payload exists true, sha256 `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31`, standard_text_copied=false, generated artifacts 0, acceptance unchanged, runtime unchanged, Android replacement false, and target capability status | `data.generated.astronomy`, `tools.selected-gb-t-preflight`, `tools.governance` |
