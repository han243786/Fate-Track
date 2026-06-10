# 递归循环 Closeout 日志

> 本文件为追加式日志。每轮递归必须写入 LoopResult。当前只记录流程纳入，不包含业务代码推进。

## LOOP-000

| Field | Value |
| --- | --- |
| `mode` | `design_only` |
| `scope` | 将递归式开发协议纳入治理体系 |
| `completed_scope` | 新增递归协议、游标、循环日志，并同步 ADR、Policy、Matrix、模块树、工程树、README 和产品树入口 |
| `changed_files` | `95-recursive-development-protocol.md`, `96-recursive-cursor.md`, `97-loop-closeout-log.md`, `docs/decisions/0007-recursive-development-protocol.md`, roadmap/governance references |
| `capability_status_changes` | none |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 10 tests passed, frontend 4 tests passed, governance scaffold OK |
| `governance_updates` | ADR 0007, General Policy GP-CHG-008..010, Standard Matrix STD-019..021, Process Matrix recursive checklist, Module Tree, Engineering Tree, README, product tree |
| `risk_updates` | no new runtime risk; process risk reduced by cursor tracking |
| `unresolved_decision_gates` | none for protocol integration |
| `next_cursor` | remain `design_only`; wait for user confirmation before code development |

## LOOP-001

| Field | Value |
| --- | --- |
| `mode` | `design_only` |
| `scope` | 将递归协议转成可执行运行手册 |
| `completed_scope` | 新增 `98-recursive-loop-runbook.md`，并同步 roadmap index、roadmap README、工程全量树 |
| `changed_files` | `markdown/20-roadmap/98-recursive-loop-runbook.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 10 tests passed, frontend 4 tests passed, governance scaffold OK |
| `governance_updates` | roadmap index + roadmap README + engineering tree now reference the loop runbook |
| `risk_updates` | no runtime risk; process ambiguity reduced by explicit Read/Slice/Preflight/Govern/Validate/Closeout runbook |
| `unresolved_decision_gates` | none for LOOP-001 |
| `next_cursor` | `LOOP-002`, `design_only`, M1 preflight dry-run; code remains forbidden |

## LOOP-002

| Field | Value |
| --- | --- |
| `mode` | `design_only` |
| `scope` | 对 M1 Date Layer Hardening 做递归预检 dry-run |
| `completed_scope` | 新增 `99-milestone-01-preflight-dry-run.md`，明确 M1-WP1 是下一轮最小代码切片，DG-002 阻止完整 M1 closeout 和范围外高置信声明 |
| `changed_files` | `markdown/20-roadmap/99-milestone-01-preflight-dry-run.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 10 tests passed, frontend 4 tests passed, governance scaffold OK |
| `governance_updates` | roadmap dry-run file + index + README + engineering tree updated |
| `risk_updates` | R-S0-001 remains active for future M1 implementation; no runtime risk introduced |
| `unresolved_decision_gates` | DG-002 remains open; it does not block M1-WP1 metadata work but blocks M1 closeout |
| `next_cursor` | `LOOP-003`, still `design_only` until user authorizes code; recommended next slice is M1-WP1 |

## LOOP-003

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M1-WP1 calendar date-query response metadata |
| `completed_scope` | Added `meta` to `/api/calendar/query` with date-layer source, algorithm version, ruleset id, 1901-2100 support range, rule notes, boundary policy, confidence, and limitations; added Rust contract test |
| `changed_files` | `backend/src/api/calendar.rs`, `README.md`, `data/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/97-loop-closeout-log.md`, `markdown/20-roadmap/96-recursive-cursor.md` |
| `capability_status_changes` | none; `calendar-date-query-v1-meta` evidence is partially satisfied but `/api/capabilities` declaration remains pending |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 11 tests passed, frontend 4 tests passed, governance scaffold OK |
| `governance_updates` | README, data README, module tree, engineering tree, capability ledger evidence note |
| `risk_updates` | No calculation algorithm changed; Android three-pillar golden cases remain green; metadata explicitly blocks hour-pillar/timezone/true-solar/full-chart claims |
| `unresolved_decision_gates` | DG-002 remains open; it does not block metadata or capability declaration work but blocks full M1 closeout |
| `next_cursor` | `LOOP-004`, `single_loop`, M1-WP4 capability declaration for `calendar-date-query-v1-meta` |

## LOOP-004

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M1-WP4 capability declaration for `calendar-date-query-v1-meta` |
| `completed_scope` | Added `calendar-date-query-v1-meta` to `/api/capabilities` as a supported traceability capability for `/api/calendar/query`; added Rust catalog test; updated README, module tree, engineering tree, and capability ledger |
| `changed_files` | `backend/src/api/capabilities.rs`, `README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | `calendar-date-query-v1-meta`: target -> supported |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 12 tests passed, frontend 4 tests passed, governance scaffold OK |
| `governance_updates` | capability ledger promoted `calendar-date-query-v1-meta`; README/module tree/engineering tree clarify that the capability is traceability only and shares the existing date-query route |
| `risk_updates` | No calculation scope expanded; no full chart/hour-pillar/timezone-history/true-solar/astronomy support claimed |
| `unresolved_decision_gates` | DG-002 remains open and blocks full M1 closeout |
| `next_cursor` | `LOOP-005`, `single_loop`, M1-WP2 additional Android edge-case golden tests |

## LOOP-005

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M1-WP2 additional Android edge-case golden tests |
| `completed_scope` | Embedded all 49 Android edge cases from `D:\myproject\Perpetual calendar\data\edge_case_test.txt`; asserted lunar month/day, leap flag, year/month/day Gan-Zhi, and selected solar terms; fixed fixed-solar-term month index wrap for late December to match the edge manifest |
| `changed_files` | `backend/src/calendar/lunar_data.rs`, `backend/src/calendar/ganzhi.rs`, `data/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 12 tests passed, frontend 4 tests passed, governance scaffold OK |
| `governance_updates` | data README, module tree, engineering tree, capability ledger evidence note |
| `risk_updates` | Android date-layer confidence increased; discovered and fixed late-December month-pillar mismatch against Android edge manifest; no full chart/hour-pillar/timezone-history/true-solar/astronomy scope added |
| `unresolved_decision_gates` | DG-002 remains open and blocks full M1 closeout |
| `next_cursor` | `LOOP-006`, `single_loop`, M1-WP3 explicit error envelope and range-boundary API tests |

## LOOP-006

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M1-WP3 explicit date-query error envelope and range-boundary API tests |
| `completed_scope` | Added `out_of_range` error code; added App-level API tests for missing date, invalid date, out-of-range dates, supported boundary dates, and missing lunar data source |
| `changed_files` | `backend/src/error.rs`, `backend/src/api/calendar.rs`, `backend/src/app.rs`, `README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 17 tests passed, frontend 4 tests passed, governance scaffold OK |
| `governance_updates` | README, module tree, engineering tree, capability ledger evidence note |
| `risk_updates` | Error semantics are clearer; out-of-range no longer shares route `not_found`; no calculation scope expanded |
| `unresolved_decision_gates` | DG-002 remains open and blocks full M1 closeout |
| `next_cursor` | `LOOP-007`, `single_loop`, frontend date-layer probe wiring without full chart UI |

## LOOP-007

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | Frontend date-layer probe wiring without full chart UI |
| `completed_scope` | Added `ApiClient.calendarDate`, state wiring, DOM mapping, Date Layer panel markup, render path, responsive styles, and API client tests for the supported calendar query route |
| `changed_files` | `frontend/src/api/client.js`, `frontend/src/state.js`, `frontend/src/main.js`, `frontend/src/ui/dom.js`, `frontend/src/ui/render.js`, `frontend/src/styles.css`, `frontend/index.html`, `frontend/tests/api-client.test.mjs`, `README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | `frontend-date-layer-probe`: unlisted -> supported surface |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 17 tests passed, frontend 6 tests passed, governance scaffold OK; local HTTP check passed on backend `127.0.0.1:8787` and frontend `127.0.0.1:5173` |
| `governance_updates` | README, module tree, engineering tree, capability ledger evidence/current supported surface |
| `risk_updates` | Frontend now exposes only date-layer output and ruleset metadata; no full chart/hour-pillar/timezone-history/true-solar/astronomy UI added |
| `unresolved_decision_gates` | DG-002 remains open and blocks full M1 closeout |
| `next_cursor` | `LOOP-008`, `single_loop`, M1 closeout readiness and DG-002 validation-range decision preparation |

## LOOP-008

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | Recursive scale optimization and goal readiness audit |
| `completed_scope` | Added a recursive scale/readiness audit; upgraded loop sizing from smallest useful slice to largest stable invariant; tightened goal_run entry rules so goal mode cannot bypass decision gates or milestone closeout |
| `changed_files` | `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/95-recursive-development-protocol.md`, `markdown/20-roadmap/98-recursive-loop-runbook.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 17 tests passed, frontend 6 tests passed, governance scaffold OK |
| `governance_updates` | Protocol, runbook, roadmap README/index, module tree, engineering tree, recursive cursor, and closeout log now reference recursive scale and goal readiness |
| `risk_updates` | Recursion cost reduced by allowing larger single-invariant loops; goal_run remains blocked until readiness audit is `ready`, DG-002 is resolved or explicitly scoped so it does not block the first goal milestone, and a milestone_loop closeout is proven |
| `unresolved_decision_gates` | DG-002 remains open and blocks full M1 closeout |
| `next_cursor` | `LOOP-009`, `single_loop`, DG-002 validation-range decision execution and M1 closeout readiness |

## LOOP-009

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | DG-002 validation-range decision execution and M1 closeout readiness |
| `completed_scope` | Closed DG-002 by ADR 0008 with V1 official validated range set to 1901-2100; added M1 closeout readiness evidence for WP1-WP5 and confirmed M1 can enter milestone_loop closeout if full gate stays green |
| `changed_files` | `docs/decisions/0008-v1-official-validation-range.md`, `markdown/20-roadmap/90-decision-gates.md`, `markdown/20-roadmap/11-milestone-01-closeout-readiness.md`, `README.md`, `data/README.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/02-milestone-01-date-layer-hardening.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `calendar-date-query` and `calendar-date-query-v1-meta` remain supported within 1901-2100 |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 17 tests passed, frontend 6 tests passed, governance scaffold OK |
| `governance_updates` | ADR, decision gates, risk register, readiness evidence, README/data README, capability ledger, milestone doc, roadmap index/README, module tree, engineering tree, cursor, closeout log |
| `risk_updates` | 1901-2100 official validation ambiguity removed; wider range remains unsupported until M9 astronomy/ephemeris work |
| `unresolved_decision_gates` | none blocking M1 closeout; DG-001 and DG-003 remain target-proposed for M2 context |
| `next_cursor` | `LOOP-010`, `milestone_loop`, M1 milestone closeout trial |

## LOOP-010

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M1 milestone closeout trial |
| `completed_scope` | Closed M1 Date Layer Hardening with formal milestone closeout evidence; proved one milestone_loop can close a milestone while preserving full gate, cursor, loop log, roadmap index, module tree, engineering tree, and capability ledger sync |
| `changed_files` | `markdown/20-roadmap/12-milestone-01-closeout.md`, `markdown/20-roadmap/11-milestone-01-closeout-readiness.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; M1-supported date-layer capabilities remain supported and full chart remains planned |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 17 tests passed, frontend 6 tests passed, governance scaffold OK |
| `governance_updates` | M1 closeout file, readiness status, recursive goal readiness audit, capability ledger, roadmap index/README, module tree, engineering tree, cursor, closeout log |
| `risk_updates` | milestone_loop is now proven for one milestone closeout; goal_run remains blocked until explicit goal scope and milestone stop points are defined |
| `unresolved_decision_gates` | DG-004 remains open for M2 closeout; DG-001 and DG-003 remain target-proposed |
| `next_cursor` | `LOOP-011`, `milestone_loop`, M2 ruleset and chart-basis preflight only |

## LOOP-011

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M2 ruleset and chart-basis contract |
| `completed_scope` | Added M2 preflight, closed DG-001/DG-003/DG-004 through ADR 0009, implemented restricted `chart-basis-preview` route, upgraded `bazi.rs` from skeleton to M2 contract layer, declared the restricted capability, and closed M2 with formal milestone evidence |
| `changed_files` | `backend/src/domain/bazi.rs`, `backend/src/api/chart_basis.rs`, `backend/src/api/mod.rs`, `backend/src/api/capabilities.rs`, `backend/src/app.rs`, `docs/decisions/0004-v1-calculation-ruleset-target.md`, `docs/decisions/0009-m2-chart-basis-contract.md`, `README.md`, `markdown/20-roadmap/03-milestone-02-ruleset-and-chart-basis.md`, `markdown/20-roadmap/13-milestone-02-preflight.md`, `markdown/20-roadmap/14-milestone-02-closeout.md`, `markdown/20-roadmap/90-decision-gates.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | `chart-basis-preview`: planned -> restricted |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 24 tests passed, frontend 6 tests passed, governance scaffold OK |
| `governance_updates` | ADR 0009, ADR 0004 status, decision gates, M2 milestone/preflight/closeout, README, module tree, engineering tree, capability ledger, cursor, closeout log |
| `risk_updates` | M2 contract now rejects lunar input and true solar time explicitly; full chart/hour pillar/timezone history remain planned for M3/M9 |
| `unresolved_decision_gates` | DG-007 remains open for M3/M9 architecture choice; DG-008 remains open for M9 astronomy engine |
| `next_cursor` | `LOOP-012`, `milestone_loop`, M3 chart-engine preflight |

## LOOP-012

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M3 chart-engine preflight |
| `completed_scope` | Closed DG-007 for M3 through ADR 0010, kept current HTTP skeleton, added M3 preflight with selected chart-engine slice, and confirmed M3 can implement chart-create core without Axum migration, timezone history, true solar time, astronomy replacement, storage, analysis, or luck cycles |
| `changed_files` | `docs/decisions/0010-http-architecture-through-chart-engine.md`, `markdown/20-roadmap/90-decision-gates.md`, `markdown/20-roadmap/15-milestone-03-preflight.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 24 tests passed, frontend 6 tests passed, governance scaffold OK |
| `governance_updates` | ADR 0010, decision gates, M3 preflight, roadmap index/README, engineering tree, cursor, closeout log |
| `risk_updates` | Framework migration risk removed from M3; current HTTP skeleton remains authoritative until a later ADR changes it |
| `unresolved_decision_gates` | DG-008 remains open for M9 astronomy engine |
| `next_cursor` | `LOOP-013`, `milestone_loop`, M3 chart-engine implementation slice: chart-create core |

## LOOP-013

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M3 chart-create core |
| `completed_scope` | Implemented `GET /api/charts`, promoted `chart-create` to supported, added chart result/domain logic, exact-time hour pillar, unknown-hour null plus candidates, metadata/warnings/ambiguity flags, unsupported-output declarations, API/domain tests, ADR 0011, and M3 closeout evidence |
| `changed_files` | `backend/src/domain/bazi.rs`, `backend/src/api/charts.rs`, `backend/src/api/mod.rs`, `backend/src/api/capabilities.rs`, `backend/src/app.rs`, `docs/decisions/0011-m3-chart-engine-core.md`, `README.md`, `markdown/20-roadmap/04-milestone-03-chart-engine.md`, `markdown/20-roadmap/16-milestone-03-closeout.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | `chart-create`: planned -> supported |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 29 tests passed, frontend 6 tests passed, governance scaffold OK |
| `governance_updates` | ADR 0011, M3 milestone/closeout, README, module tree, engineering tree, capability ledger, cursor, closeout log |
| `risk_updates` | Unknown hour no longer fabricates noon; unsupported timezone history, true solar time, lunar input, storage, analysis, and luck cycles remain explicit |
| `unresolved_decision_gates` | DG-005 open for luck cycles; DG-010 open before analysis implementation; DG-008 open for M9 astronomy engine |
| `next_cursor` | `LOOP-014`, `milestone_loop`, M4 analysis-engine preflight |

## LOOP-014

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M4 structured analysis engine |
| `completed_scope` | Closed DG-010 through ADR 0012, implemented `GET /api/analysis/snapshot`, promoted `analysis-snapshot` to supported, added deterministic element/ten-god/hidden-stem/relation/sensitivity metrics, fixed analysis cards, disclaimer id, forbidden-output audit, domain/API tests, and M4 closeout evidence |
| `changed_files` | `backend/src/domain/analysis.rs`, `backend/src/domain/mod.rs`, `backend/src/api/analysis.rs`, `backend/src/api/mod.rs`, `backend/src/api/capabilities.rs`, `backend/src/api/charts.rs`, `backend/src/app.rs`, `docs/decisions/0012-structured-analysis-only.md`, `README.md`, `markdown/20-roadmap/05-milestone-04-analysis-engine.md`, `markdown/20-roadmap/17-milestone-04-preflight.md`, `markdown/20-roadmap/18-milestone-04-closeout.md`, `markdown/20-roadmap/90-decision-gates.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | `analysis-snapshot`: planned -> supported |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 34 tests passed, frontend 6 tests passed, governance scaffold OK |
| `governance_updates` | ADR 0012, DG-010, M4 preflight/closeout, README, module tree, engineering tree, capability ledger, cursor, closeout log |
| `risk_updates` | Generated prose and high-risk deterministic claims remain prohibited; unknown-hour sensitivity is exposed; luck cycles remain planned because DG-005 is open |
| `unresolved_decision_gates` | DG-006 open before storage implementation; DG-005 open for luck cycles; DG-008 open for M9 astronomy engine |
| `next_cursor` | `LOOP-015`, `milestone_loop`, M5 case-storage preflight |

## LOOP-015

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M5 local volatile case storage and settings |
| `completed_scope` | Closed DG-006 for M5 through ADR 0013; implemented restricted `GET /api/cases` and `GET /api/settings`; added local volatile repositories, immutable chart/analysis snapshot refs, private-note list redaction, archive/delete semantics, preference validation, capability declarations, API/domain tests, and M5 closeout evidence |
| `changed_files` | `backend/src/domain/cases.rs`, `backend/src/domain/settings.rs`, `backend/src/api/cases.rs`, `backend/src/api/settings.rs`, `backend/src/api/mod.rs`, `backend/src/api/capabilities.rs`, `backend/src/app.rs`, `docs/decisions/0013-local-volatile-case-storage.md`, `README.md`, `markdown/20-roadmap/06-milestone-05-case-storage.md`, `markdown/20-roadmap/19-milestone-05-preflight.md`, `markdown/20-roadmap/20-milestone-05-closeout.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | `case-management`: planned -> restricted; `settings`: planned -> restricted |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 46 tests passed, frontend 6 tests passed, governance scaffold OK |
| `governance_updates` | ADR 0013, decision gate DG-006, M5 milestone/preflight/closeout, README, module tree, engineering tree, capability ledger, cursor, closeout log |
| `risk_updates` | M5 remains local and volatile only; no account, database persistence, cloud sync, cross-device sync, public sharing, share tokens, luck cycles, or generated analysis were introduced |
| `unresolved_decision_gates` | DG-009 target-proposed before M6 share implementation; DG-005 open for luck cycles; DG-008 open for M9 astronomy engine |
| `next_cursor` | `LOOP-016`, `milestone_loop`, M6 share-privacy preflight |

## LOOP-016

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M6 local volatile share privacy |
| `completed_scope` | Closed DG-009 for M6 through ADR 0014; implemented restricted `GET /api/share/preview`; added local volatile share repository, hash-only token storage, redacted public DTOs, expiration, revocation, noindex/non-editable flags, unavailable-response unification, capability declaration, API/domain tests, and M6 closeout evidence |
| `changed_files` | `backend/src/domain/share.rs`, `backend/src/domain/mod.rs`, `backend/src/api/share.rs`, `backend/src/api/cases.rs`, `backend/src/api/mod.rs`, `backend/src/api/capabilities.rs`, `backend/src/app.rs`, `docs/decisions/0014-share-token-privacy-boundary.md`, `README.md`, `markdown/20-roadmap/07-milestone-06-share-privacy.md`, `markdown/20-roadmap/21-milestone-06-preflight.md`, `markdown/20-roadmap/22-milestone-06-closeout.md`, `markdown/20-roadmap/90-decision-gates.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | `share-preview`: planned -> restricted |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 6 tests passed, governance scaffold OK |
| `governance_updates` | ADR 0014, DG-009, M6 milestone/preflight/closeout, README, module tree, engineering tree, capability ledger, risk register, recursive scale audit, cursor, closeout log |
| `risk_updates` | M6 remains local and volatile only; public DTOs omit private note, raw title, tags, private case id, exact birth-time/location fields, and snapshot id; revoked/expired/invalid tokens share unavailable response |
| `unresolved_decision_gates` | DG-005 open for luck cycles; DG-008 open for M9 astronomy engine |
| `next_cursor` | `LOOP-017`, `milestone_loop`, M7 frontend-workspace preflight |

## LOOP-017

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M7 restricted frontend workspace |
| `completed_scope` | Implemented M7 frontend workspace consuming existing supported/restricted APIs; added chart input, chart workspace, analysis metrics/cards, local case save/list, redacted share preview, date-layer probe, data metadata, capability boundary rendering, responsive styles, API client route tests, browser desktop/mobile verification, and M7 closeout evidence |
| `changed_files` | `frontend/index.html`, `frontend/src/api/client.js`, `frontend/src/state.js`, `frontend/src/main.js`, `frontend/src/ui/dom.js`, `frontend/src/ui/render.js`, `frontend/src/styles.css`, `frontend/src/utils/format.js`, `frontend/tests/api-client.test.mjs`, `frontend/tests/format.test.mjs`, `README.md`, `markdown/20-roadmap/08-milestone-07-frontend-workspace.md`, `markdown/20-roadmap/23-milestone-07-preflight.md`, `markdown/20-roadmap/24-milestone-07-closeout.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | `frontend-chart-workspace`: planned -> restricted; `frontend-share-preview`: planned -> restricted; `glossary` remains planned |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 8 tests passed, governance scaffold OK; browser desktop/mobile checks passed |
| `governance_updates` | M7 preflight/closeout, README, module tree, engineering tree, capability ledger, risk register, recursive scale audit, cursor, closeout log |
| `risk_updates` | Frontend overclaim risk recorded as R-P1-009; UI consumes APIs and must not claim luck cycles, durable sharing, cloud sync, account storage, glossary, true solar time, timezone history, range expansion, or astronomy replacement |
| `unresolved_decision_gates` | DG-005 open for luck cycles; DG-008 open for M9 astronomy engine |
| `next_cursor` | `LOOP-018`, `milestone_loop`, M8 validation-release preflight |

## LOOP-018

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M8 validation release and release candidate freeze |
| `completed_scope` | Added release candidate checker, release document, frontend workspace markup/overclaim tests, M8 preflight/closeout, release boundary sync across README/module tree/engineering tree/product tree/capability ledger/risk register, and promoted `release-candidate` as a governance/release capability only |
| `changed_files` | `docs/release/v1-release-candidate.md`, `tools/check-release-candidate.ps1`, `tools/check-project.ps1`, `frontend/tests/workspace-markup.test.mjs`, `README.md`, `markdown/20-roadmap/09-milestone-08-validation-release.md`, `markdown/20-roadmap/25-milestone-08-preflight.md`, `markdown/20-roadmap/26-milestone-08-closeout.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | `release-candidate`: planned -> supported as governance/release surface; no new backend business API was added |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK; M8 browser probe confirmed 9 panels, capability boundary, no luck-cycle/durable-sharing supported claim, and no mobile 390px overflow |
| `governance_updates` | Release candidate document/checker, M8 preflight/closeout, README, module tree, engineering tree, product tree, capability ledger, risk register, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-010 records release candidate overclaim risk; release checker now guards capability freezes, README boundaries, frontend overclaim tests, and share privacy evidence |
| `unresolved_decision_gates` | DG-005 open for luck cycles; DG-008 open and blocking M9 astronomy-engine implementation |
| `next_cursor` | `LOOP-019`, `milestone_loop`, M9 astronomy-upgrade preflight |

## LOOP-019

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 astronomy-upgrade preflight |
| `completed_scope` | Closed DG-008 for M9 preflight through ADR 0015 with a parallel-first strategy; added M9 preflight, generated astronomy manifest schema, Android-vs-astronomy comparison report template, astronomy preflight checker, and governance sync across README/data README/module tree/engineering tree/product tree/roadmap/capability ledger/risk register |
| `changed_files` | `docs/decisions/0015-m9-astronomy-parallel-strategy.md`, `data/generated/astronomy/README.md`, `data/generated/astronomy/manifest.schema.json`, `data/generated/astronomy/comparison-report-template.md`, `tools/check-astronomy-preflight.ps1`, `tools/check-project.ps1`, `README.md`, `data/README.md`, `markdown/20-roadmap/10-milestone-09-astronomy-upgrade.md`, `markdown/20-roadmap/27-milestone-09-preflight.md`, `markdown/20-roadmap/90-decision-gates.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK |
| `governance_updates` | ADR 0015, DG-008 status, M9 preflight, generated-data schema/template, new preflight checker, README/data README, module tree, engineering tree, product tree, capability ledger, risk register, standard matrix, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-011 records that astronomy preflight artifacts must not be mistaken for generated engine evidence; Android baseline remains accepted-current |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-020`, `milestone_loop`, M9 generated astronomy evidence planning |

## LOOP-020

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 astronomy source/tooling decision |
| `completed_scope` | Added ADR 0016 selecting GB/T 33661-2017, NASA/JPL Horizons API, IAU SOFA ANSI C, and NAIF CSPICE/SPICE as the M9 source stack; added `source-policy.json`; expanded astronomy preflight checker to validate source policy, ADR 0016, source ids, and first generated range; synchronized README/data README/module tree/engineering tree/product tree/roadmap/capability ledger |
| `changed_files` | `docs/decisions/0016-m9-astronomy-source-stack.md`, `data/generated/astronomy/source-policy.json`, `data/generated/astronomy/README.md`, `data/generated/astronomy/manifest.schema.json`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `markdown/20-roadmap/27-milestone-09-preflight.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK |
| `governance_updates` | ADR 0016, source policy, manifest schema, preflight checker, README/data README, module tree, engineering tree, product tree, roadmap README, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | Source stack is now explicit but still not generated engine evidence; Android baseline remains accepted-current and replacement still requires a later ADR |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-021`, `milestone_loop`, M9 source availability and manifest instance planning |

## LOOP-021

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 source availability probe |
| `completed_scope` | Added optional `tools/probe-astronomy-sources.ps1`, documented why it is not part of the deterministic full gate, probed the selected source stack, recorded source availability evidence, and synchronized README/data README/module tree/engineering tree/roadmap/capability ledger/recursive audit |
| `changed_files` | `tools/probe-astronomy-sources.ps1`, `README.md`, `data/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/28-milestone-09-source-availability.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | Optional source probe returned summary `warning`: required JPL Horizons docs/API, IAU SOFA, and NAIF were reachable; GB/T page returned a nonblocking warning. `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK |
| `governance_updates` | Source probe tool, source availability evidence file, README/data README, module tree, engineering tree, roadmap index/README, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | External-source availability evidence is now separated from deterministic full gates; GB/T auto-probe warning requires manual/browser review when used for generated evidence |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-022`, `milestone_loop`, M9 generated manifest instance planning |

## LOOP-022

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 not-accepted generated manifest draft |
| `completed_scope` | Added `data/generated/astronomy/manifests/astronomy-engine-v0-draft.json` as the first manifest planning instance; expanded manifest schema and astronomy preflight checker to enforce `not_accepted`, no generation command, no artifact hashes, template-only comparison report, and required acceptance blockers; synchronized data README, README, module tree, engineering tree, roadmap, capability ledger, risk register, recursive audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/manifests/astronomy-engine-v0-draft.json`, `data/generated/astronomy/manifest.schema.json`, `data/generated/astronomy/README.md`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/29-milestone-09-manifest-draft.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/92-risk-register.md`, `README.md`, `data/README.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK |
| `governance_updates` | Draft manifest, manifest schema, preflight checker, M9 manifest evidence, README/data README, module tree, engineering tree, roadmap index/README, capability ledger, risk register, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-012 records draft manifest overclaim risk; checker prevents draft manifest from being treated as generated/accepted evidence |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-023`, `milestone_loop`, M9 generated artifact shape and generation command planning |

## LOOP-023

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 generated artifact shape and generation command planning |
| `completed_scope` | Added `data/generated/astronomy/generation-plan.json` defining draft planned artifacts, future command shape, required hashes, acceptance evidence, and forbidden runtime claims; expanded astronomy preflight checker to enforce `draft_not_runnable`, `not_implemented` script status, `not_generated` planned artifacts, required hashes, and forbidden runtime claims; synchronized README, module tree, engineering tree, roadmap, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/generation-plan.json`, `data/generated/astronomy/README.md`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/30-milestone-09-generation-plan.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `README.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK |
| `governance_updates` | Generation plan, preflight checker, M9 generation evidence, README, module tree, engineering tree, roadmap index/README, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | Planned generator scope is fixed without runnable generation or accepted artifacts; runtime claims remain forbidden |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-024`, `milestone_loop`, M9 generation script skeleton dry-run |

## LOOP-024

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 generation script skeleton dry-run |
| `completed_scope` | Added `tools/generate-astronomy-tables.ps1` as a dry-run-only generator skeleton; updated generation plan command to include `-DryRun`; expanded astronomy preflight checker to execute the dry-run and fail if it writes artifacts, changes acceptance status, sees existing planned artifacts, or reports a mismatched planned artifact count; synchronized README, data README, module tree, engineering tree, roadmap, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `tools/generate-astronomy-tables.ps1`, `data/generated/astronomy/generation-plan.json`, `data/generated/astronomy/README.md`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/31-milestone-09-generator-dry-run.md`, `README.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; generator dry-run reported 4 planned artifacts, no writes, no acceptance change, and no existing planned artifacts |
| `governance_updates` | Dry-run generator skeleton, generation plan, preflight checker, M9 generator evidence, README/data README, module tree, engineering tree, roadmap index/README, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | Generator skeleton is executable only in dry-run mode; no generated files, hashes, comparison report, or runtime integration were added |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-025`, `milestone_loop`, M9 comparison and golden-case planning |

## LOOP-025

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 comparison, golden-case, and replay planning |
| `completed_scope` | Added `comparison.schema.json`, `golden-cases-plan.json`, and `replay-policy-draft.md`; expanded astronomy preflight checker to validate comparison required fields, difference categories, golden categories as `not_generated`, and replay policy prohibition on silent Android baseline replacement; synchronized README, module tree, engineering tree, roadmap, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/comparison.schema.json`, `data/generated/astronomy/golden-cases-plan.json`, `data/generated/astronomy/replay-policy-draft.md`, `data/generated/astronomy/README.md`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/32-milestone-09-comparison-golden-replay-plan.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `README.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK |
| `governance_updates` | Comparison schema, golden-case plan, replay policy draft, preflight checker, M9 planning evidence, README, module tree, engineering tree, roadmap index/README, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | Comparison/golden/replay requirements are now planning-only and machine-checked; no generated rows, replay tests, or accepted artifacts were added |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-026`, `milestone_loop`, M9 comparison dry-run scaffold |

## LOOP-026

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 comparison dry-run scaffold |
| `completed_scope` | Added `tools/compare-astronomy-dry-run.ps1` as a dry-run-only comparison scaffold; expanded astronomy preflight checker to execute it and fail if it reports comparison rows, writes files, or claims accepted evidence; synchronized README, module tree, engineering tree, roadmap, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `tools/compare-astronomy-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/33-milestone-09-comparison-dry-run.md`, `README.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; comparison dry-run reported 0 rows, no writes, and no accepted evidence |
| `governance_updates` | Comparison dry-run scaffold, preflight checker, M9 comparison dry-run evidence, README, module tree, engineering tree, roadmap index/README, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | Comparison scaffold is executable only as zero-row dry-run; no generated comparison data or accepted evidence was added |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-027`, `milestone_loop`, M9 golden-case dry-run scaffold |

## LOOP-027

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 golden-case dry-run scaffold |
| `completed_scope` | Added `tools/golden-cases-dry-run.ps1` as a dry-run-only golden-case scaffold; expanded astronomy preflight checker to execute it and fail if it reports generated rows, writes files, claims accepted evidence, or mismatches category count; synchronized README, module tree, engineering tree, roadmap, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `tools/golden-cases-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/34-milestone-09-golden-dry-run.md`, `README.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; golden dry-run reported 6 categories, 0 generated rows, no writes, and no accepted evidence |
| `governance_updates` | Golden-case dry-run scaffold, preflight checker, M9 golden dry-run evidence, README, module tree, engineering tree, roadmap index/README, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | Golden-case scaffold is executable only as zero-row dry-run; no generated golden data or accepted evidence was added |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-028`, `milestone_loop`, M9 replay-policy dry-run or M9 final pre-closeout audit |

## LOOP-028

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 replay-policy dry-run scaffold |
| `completed_scope` | Added `tools/replay-policy-dry-run.ps1` as a dry-run-only replay-control scaffold; expanded astronomy preflight checker to execute it and fail if it reports replay tests, writes files, claims accepted evidence, permits replacement, or mismatches required control count; synchronized README, data README, module tree, engineering tree, product tree, roadmap, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `tools/replay-policy-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/35-milestone-09-replay-policy-dry-run.md`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; replay-policy dry-run reported 5 controls, 0 replay tests, no writes, no accepted evidence, and replacement disallowed |
| `governance_updates` | Replay-policy dry-run scaffold, preflight checker, M9 replay-policy dry-run evidence, README/data README, module tree, engineering tree, product tree, roadmap index/README, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | Replay-policy scaffold is executable only as zero-test dry-run; no replay-test evidence, generated astronomy data, accepted evidence, or Android baseline replacement was added |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-029`, `milestone_loop`, M9 final pre-closeout audit |

## LOOP-029

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 final pre-closeout audit |
| `completed_scope` | Added machine-readable `precloseout-audit.json` and human-readable `36-milestone-09-pre-closeout-audit.md`; expanded astronomy preflight checker to enforce full M9 closeout remains blocked while preflight closeout review is ready; synchronized README, data README, generated-data README, M9 milestone file, module tree, engineering tree, product tree, roadmap, standard matrix, risk register, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/precloseout-audit.json`, `markdown/20-roadmap/36-milestone-09-pre-closeout-audit.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/10-milestone-09-astronomy-upgrade.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; pre-closeout audit reports full M9 closeout blocked, preflight review ready, generated artifacts not accepted, Android replacement disallowed |
| `governance_updates` | Pre-closeout machine audit, M9 audit evidence, astronomy preflight checker, README/data README/generated README, M9 milestone file, module tree, engineering tree, product tree, roadmap index/README, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-013 records the risk that pre-closeout audit could be mistaken for full astronomy-engine closeout; checker enforces `full_closeout_allowed=false` and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-030`, `milestone_loop`, M9 generated-data implementation planning decision |

## LOOP-030

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 generated-data implementation planning decision |
| `completed_scope` | Added ADR 0017 and `implementation-plan.json` selecting continued M9 generated-data planning with generator contract planning as the next stage; expanded astronomy preflight checker to enforce planning-only status, target capability status, stage coverage, and forbidden runtime/artifact changes; synchronized README, data README, generated-data README, M9 milestone file, module tree, engineering tree, product tree, roadmap, standard matrix, risk register, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `docs/decisions/0017-m9-generated-data-implementation-path.md`, `data/generated/astronomy/implementation-plan.json`, `markdown/20-roadmap/37-milestone-09-generated-data-implementation-plan.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/10-milestone-09-astronomy-upgrade.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; implementation plan remains planning-only and selects generator contract planning as next |
| `governance_updates` | ADR 0017, implementation plan JSON, M9 implementation plan evidence, astronomy preflight checker, README/data README/generated README, M9 milestone file, module tree, engineering tree, product tree, roadmap index/README, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-014 records the risk that implementation planning is mistaken for permission to accept generated artifacts; checker enforces `planning_only` and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-031`, `milestone_loop`, M9 generator contract planning |

## LOOP-031

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 generator contract planning |
| `completed_scope` | Added `generator-contract.json` defining required inputs, planned outputs, canonical JSON encoding, `sha256` hash policy, manifest update rules, and forbidden contract-stage actions; updated implementation plan to mark `generator-contract` as `contract_defined`; expanded dry-run generator and astronomy preflight checker to enforce the contract; synchronized README, data README, generated-data README, M9 milestone file, module tree, engineering tree, product tree, roadmap, standard matrix, risk register, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/generator-contract.json`, `data/generated/astronomy/implementation-plan.json`, `markdown/20-roadmap/38-milestone-09-generator-contract.md`, `tools/generate-astronomy-tables.ps1`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/10-milestone-09-astronomy-upgrade.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; generator dry-run reported generator contract id, sha256, 4 planned artifacts, no existing artifacts, no writes, and no acceptance change |
| `governance_updates` | Generator contract JSON, implementation plan stage update, M9 generator contract evidence, dry-run generator, astronomy preflight checker, README/data README/generated README, M9 milestone file, module tree, engineering tree, product tree, roadmap index/README, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-015 records generator contract overclaim risk; checker and dry-run generator enforce `contract_only`, `not_generated`, `sha256`, no writes, and no acceptance change |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-032`, `milestone_loop`, M9 source adapter contract planning |

## LOOP-032

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 source adapter contract planning |
| `completed_scope` | Added `source-adapter-contract.json` defining GB/T, Horizons, SOFA, and SPICE source adapter boundaries; updated implementation plan to mark `source-adapter-contract` as `contract_defined`; expanded astronomy preflight checker to enforce contract-only status, selected source coverage, generator contract linkage, no runtime dependency, no output claim, and no external API call in full gate; synchronized README, data README, generated-data README, M9 milestone file, module tree, engineering tree, product tree, roadmap, standard matrix, risk register, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/source-adapter-contract.json`, `data/generated/astronomy/implementation-plan.json`, `markdown/20-roadmap/39-milestone-09-source-adapter-contract.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/10-milestone-09-astronomy-upgrade.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; source adapter contract covers GB/T, Horizons, SOFA, and SPICE with no runtime dependency and no output claims |
| `governance_updates` | Source adapter contract JSON, implementation plan stage update, M9 source adapter contract evidence, astronomy preflight checker, README/data README/generated README, M9 milestone file, module tree, engineering tree, product tree, roadmap index/README, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-016 records source adapter overclaim risk; checker enforces `contract_only`, selected source coverage, no runtime dependency, no output claim, and no external API call in full gate |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-033`, `milestone_loop`, M9 artifact writer dry-run planning |

## LOOP-033

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 artifact writer dry-run planning |
| `completed_scope` | Added `artifact-writer-plan.json` and `tools/artifact-writer-dry-run.ps1` to preview output paths and `sha256` hash status without creating directories, writing files, computing hashes, updating manifest hash state, or claiming accepted evidence; updated implementation plan to mark `artifact-writer-dry-run` as `dry_run_defined`; expanded astronomy preflight checker to execute and enforce the dry-run; synchronized README, data README, generated-data README, M9 milestone file, module tree, engineering tree, product tree, roadmap, standard matrix, risk register, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/artifact-writer-plan.json`, `tools/artifact-writer-dry-run.ps1`, `data/generated/astronomy/implementation-plan.json`, `markdown/20-roadmap/40-milestone-09-artifact-writer-dry-run.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/10-milestone-09-astronomy-upgrade.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; artifact writer dry-run reported 4 planned artifacts, output directory absent, no writes, zero hashes computed, and no accepted evidence |
| `governance_updates` | Artifact writer plan, artifact writer dry-run tool, implementation plan stage update, M9 artifact writer evidence, astronomy preflight checker, README/data README/generated README, M9 milestone file, module tree, engineering tree, product tree, roadmap index/README, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-017 records artifact writer dry-run overclaim risk; checker enforces `dry_run_only`, `no_write_preview`, no writes, zero hashes, no existing planned artifacts, and no accepted evidence |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-034`, `milestone_loop`, M9 comparison runner dry-run planning |

## LOOP-034

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 comparison runner dry-run planning |
| `completed_scope` | Added `comparison-runner-plan.json`; enhanced `tools/compare-astronomy-dry-run.ps1` to bind Android baseline metadata, future comparison artifact path, required binding fields, and zero-row dry-run policy; updated implementation plan to mark `comparison-runner-dry-run` as `dry_run_defined`; expanded astronomy preflight checker to enforce runner plan linkage, Android baseline bindings, required fields, forbidden dry-run actions, and zero-row output; synchronized README, data README, generated-data README, M9 milestone file, module tree, engineering tree, product tree, roadmap, standard matrix, risk register, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/comparison-runner-plan.json`, `tools/compare-astronomy-dry-run.ps1`, `data/generated/astronomy/implementation-plan.json`, `markdown/20-roadmap/41-milestone-09-comparison-runner-dry-run.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/10-milestone-09-astronomy-upgrade.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; comparison dry-run reported Android baseline bindings, future comparison artifact path, rows compared 0, difference rows 0, no writes, and no accepted evidence |
| `governance_updates` | Comparison runner plan, comparison dry-run update, implementation plan stage update, M9 comparison runner evidence, astronomy preflight checker, README/data README/generated README, M9 milestone file, module tree, engineering tree, product tree, roadmap index/README, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-018 records comparison runner dry-run overclaim risk; checker enforces `dry_run_only`, Android baseline bindings, zero rows, zero differences, no writes, and no accepted evidence |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-035`, `milestone_loop`, M9 golden-row materialization readiness planning |

## LOOP-035

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 golden-row materialization readiness planning |
| `completed_scope` | Added `golden-row-readiness-plan.json`; enhanced `tools/golden-cases-dry-run.ps1` to report readiness categories while keeping generated rows at zero; updated implementation plan to mark `golden-row-materialization` as `readiness_defined`; expanded astronomy preflight checker to enforce readiness-only status, category coverage, blocked/not-generated categories, required preconditions, forbidden readiness-stage actions, and zero-row dry-run output; synchronized README, data README, generated-data README, M9 milestone file, module tree, engineering tree, product tree, roadmap, standard matrix, risk register, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/golden-row-readiness-plan.json`, `tools/golden-cases-dry-run.ps1`, `data/generated/astronomy/implementation-plan.json`, `markdown/20-roadmap/42-milestone-09-golden-row-readiness.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/10-milestone-09-astronomy-upgrade.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; golden dry-run reported 6 required categories, 6 readiness categories, generated rows 0, no writes, and no accepted evidence |
| `governance_updates` | Golden row readiness plan, golden dry-run update, implementation plan stage update, M9 golden readiness evidence, astronomy preflight checker, README/data README/generated README, M9 milestone file, module tree, engineering tree, product tree, roadmap index/README, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-019 records golden-row readiness overclaim risk; checker enforces `readiness_only`, not-generated categories, blocked readiness, zero rows, no writes, and no accepted evidence |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-036`, `milestone_loop`, M9 replay-test materialization readiness planning |

## LOOP-036

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 replay-test materialization readiness planning |
| `completed_scope` | Added `replay-test-readiness-plan.json`; enhanced `tools/replay-policy-dry-run.ps1` to report replay readiness controls while keeping replay tests executed at zero; updated implementation plan to mark `replay-test-materialization` as `readiness_defined`; expanded astronomy preflight checker to enforce readiness-only status, Android baseline bindings, replay prerequisites, forbidden readiness-stage actions, unexecuted/blocked controls, and zero-test dry-run output; synchronized README, data README, generated-data README, M9 milestone file, module tree, engineering tree, product tree, roadmap, standard matrix, risk register, capability ledger, recursive audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/replay-test-readiness-plan.json`, `tools/replay-policy-dry-run.ps1`, `data/generated/astronomy/implementation-plan.json`, `markdown/20-roadmap/43-milestone-09-replay-test-readiness.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/10-milestone-09-astronomy-upgrade.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; replay dry-run reported 5 controls, 5 readiness controls, replay tests executed 0, no writes, no accepted evidence, and replacement disallowed |
| `governance_updates` | Replay test readiness plan, replay dry-run update, implementation plan stage update, M9 replay readiness evidence, astronomy preflight checker, README/data README/generated README, M9 milestone file, module tree, engineering tree, product tree, roadmap index/README, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-020 records replay readiness overclaim risk; checker enforces `readiness_only`, unexecuted/blocked controls, zero replay tests, replacement disallowed, and no accepted evidence |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M9 replacement ADR still required before any Android baseline replacement |
| `next_cursor` | `LOOP-037`, `milestone_loop`, M9 preflight milestone closeout decision |

## LOOP-037

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M9 preflight milestone closeout decision |
| `completed_scope` | Added `preflight-closeout-decision.json`, `44-milestone-09-preflight-closeout.md`, and `45-milestone-10-generated-astronomy-implementation.md`; expanded astronomy preflight checker to enforce M9 closes only as preflight, full engine closeout remains false, generated artifacts are not accepted, runtime route behavior cannot change, Android replacement remains disallowed, and real implementation routes to M10; synchronized README/data README/generated README, M9 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/preflight-closeout-decision.json`, `markdown/20-roadmap/44-milestone-09-preflight-closeout.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/10-milestone-09-astronomy-upgrade.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; preflight closeout decision reports M9 preflight closed, full engine closeout false, generated artifacts not accepted, Android replacement disallowed, runtime route changes disallowed, and M10 as next milestone |
| `governance_updates` | M9 preflight-only closeout decision, M10 generated astronomy implementation milestone, astronomy preflight checker, README/data README/generated README, M9 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-021 records the risk that M9 preflight closeout is mistaken for full astronomy-engine completion; checker enforces full engine closeout false, no generated-data acceptance, no runtime route changes, no Android replacement, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires replacement ADR before any Android baseline replacement or default runtime behavior change |
| `next_cursor` | `LOOP-038`, `milestone_loop`, M10 generated astronomy implementation entry |

## LOOP-038

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 generated astronomy implementation entry |
| `completed_scope` | Added `generator-implementation-entry.json` and `46-milestone-10-generator-entry.md`; enhanced `tools/generate-astronomy-tables.ps1` with `-PrepareImplementation` as a guarded non-dry-run entrypoint; expanded astronomy preflight checker to execute both M9 dry-run and M10 guarded entry and fail if M10 writes files, computes hashes, changes manifest acceptance, changes runtime behavior, drops target capability status, or proceeds while source snapshot manifest is missing; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/generator-implementation-entry.json`, `markdown/20-roadmap/46-milestone-10-generator-entry.md`, `tools/generate-astronomy-tables.ps1`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; `tools/generate-astronomy-tables.ps1 -PrepareImplementation` reported `implementation_entry_guarded`, dry_run false, source snapshot manifest missing, generation blocked true, planned artifacts 4, no writes, hashes 0, acceptance unchanged, runtime unchanged |
| `governance_updates` | M10 generator implementation entry JSON, M10 generator entry evidence, guarded generator script, astronomy preflight checker, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-022 records the risk that M10 guarded generator entry is mistaken for generated astronomy data; checker enforces blocked generation, missing source snapshot manifest, no writes, hashes 0, acceptance unchanged, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires source snapshot manifest, local adapter evidence, generated artifacts, hashes, comparison report, golden rows, replay tests, and replacement ADR before any runtime replacement |
| `next_cursor` | `LOOP-039`, `milestone_loop`, M10 source snapshot manifest boundary |

## LOOP-039

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 source snapshot manifest boundary |
| `completed_scope` | Added `source-snapshot-manifest.schema.json`, `source-snapshot-manifest-plan.json`, `tools/source-snapshot-manifest-dry-run.ps1`, and `47-milestone-10-source-snapshot-boundary.md`; expanded astronomy preflight checker to validate schema fields, selected source coverage, source adapter linkage, 1901-2100 range alignment, missing actual source snapshot manifest, no source snapshot materialization, no generated astronomy artifact writes, hashes 0, acceptance unchanged, runtime unchanged, and target capability status; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/source-snapshot-manifest.schema.json`, `data/generated/astronomy/source-snapshot-manifest-plan.json`, `tools/source-snapshot-manifest-dry-run.ps1`, `markdown/20-roadmap/47-milestone-10-source-snapshot-boundary.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; `tools/source-snapshot-manifest-dry-run.ps1` reported `source_snapshot_manifest_boundary_dry_run`, planned sources 4, manifest exists false, manifest directory exists false, no writes, source snapshots materialized 0, generated artifacts 0, hashes 0, acceptance unchanged, runtime unchanged |
| `governance_updates` | Source snapshot manifest schema, source snapshot manifest boundary plan, source snapshot dry-run tool, M10 source snapshot evidence, astronomy preflight checker, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-023 records the risk that source snapshot manifest boundary is mistaken for source materialization; checker enforces manifest absent, source snapshots materialized 0, generated artifacts 0, hashes 0, no writes, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires source snapshot materialization, local adapter evidence, generated artifacts, hashes, comparison report, golden rows, replay tests, and replacement ADR before any runtime replacement |
| `next_cursor` | `LOOP-040`, `milestone_loop`, M10 source snapshot manifest materialization decision |

## LOOP-040

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 source snapshot manifest materialization decision |
| `completed_scope` | Materialized `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json` as metadata only; updated `source-snapshot-manifest-plan.json` and `generator-implementation-entry.json`; enhanced `tools/source-snapshot-manifest-dry-run.ps1` and astronomy preflight checker to validate metadata-only status, selected source coverage, all sources `not_materialized`, runtime/output claims false, generated artifacts 0, hashes 0, acceptance unchanged, runtime unchanged, and target capability status; added `48-milestone-10-source-snapshot-manifest.md`; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`, `data/generated/astronomy/source-snapshot-manifest-plan.json`, `data/generated/astronomy/generator-implementation-entry.json`, `tools/source-snapshot-manifest-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/48-milestone-10-source-snapshot-manifest.md`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/46-milestone-10-generator-entry.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; `tools/source-snapshot-manifest-dry-run.ps1` reported `source_snapshot_manifest_metadata_dry_run`, manifest status `metadata_only_no_source_payload`, manifest exists true, planned sources 4, manifest sources 4, source snapshots materialized 0, generated artifacts 0, hashes 0, acceptance unchanged, runtime unchanged |
| `governance_updates` | Metadata-only source snapshot manifest, source snapshot plan update, generator entry update, source snapshot dry-run update, astronomy preflight checker, M10 source snapshot manifest evidence, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-024 records the risk that metadata-only source snapshot manifest is mistaken for source payload evidence; checker enforces all sources `not_materialized`, runtime dependency false, output claim false, generated artifacts 0, hashes 0, acceptance unchanged, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires source payload materialization, local adapter evidence, generated artifacts, hashes, comparison report, golden rows, replay tests, and replacement ADR before any runtime replacement |
| `next_cursor` | `LOOP-041`, `milestone_loop`, M10 source payload materialization policy |

## LOOP-041

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 source payload materialization policy |
| `completed_scope` | Added `source-payload-materialization-policy.json`, `tools/source-payload-materialization-dry-run.ps1`, and `49-milestone-10-source-payload-policy.md`; expanded astronomy preflight checker to validate per-source payload coverage, payload directory absent, payload files absent, source payloads 0, payload hashes 0, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged, and target capability status; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/source-payload-materialization-policy.json`, `tools/source-payload-materialization-dry-run.ps1`, `markdown/20-roadmap/49-milestone-10-source-payload-policy.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; `tools/source-payload-materialization-dry-run.ps1` reported `source_payload_materialization_policy_dry_run`, planned payloads 4, payload directory exists false, existing payload files 0, source payloads materialized 0, payload hashes 0, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged |
| `governance_updates` | Source payload materialization policy, source payload dry-run tool, M10 source payload evidence, astronomy preflight checker, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-025 records the risk that source payload policy is mistaken for source payload materialization; checker enforces payload directory absent, payload files 0, source payloads 0, payload hashes 0, generated artifacts 0, acceptance unchanged, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires per-source payload schemas, source payload materialization, local adapter evidence, generated artifacts, hashes, comparison report, golden rows, replay tests, and replacement ADR before any runtime replacement |
| `next_cursor` | `LOOP-042`, `milestone_loop`, M10 per-source payload schema definition |

## LOOP-042

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 per-source payload schema definition |
| `completed_scope` | Added four `source-payload-schemas/*.schema.json` files for GB/T rule references, JPL Horizons validation samples, IAU SOFA routine versions, and NAIF CSPICE kernel/toolkit boundaries; linked every planned payload in `source-payload-materialization-policy.json` to its schema; enhanced `tools/source-payload-materialization-dry-run.ps1` and astronomy preflight checker to validate schema existence, schema/source/kind matching, common required fields, forbidden claims, schema file count, and continued absence of payload files, hashes, generated artifacts, acceptance changes, runtime changes, and capability promotion; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/source-payload-schemas/gb-t-33661-2017-rule-reference.schema.json`, `data/generated/astronomy/source-payload-schemas/jpl-horizons-validation-samples.schema.json`, `data/generated/astronomy/source-payload-schemas/iau-sofa-routine-version.schema.json`, `data/generated/astronomy/source-payload-schemas/naif-cspice-kernel-boundary.schema.json`, `data/generated/astronomy/source-payload-materialization-policy.json`, `tools/source-payload-materialization-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/50-milestone-10-source-payload-schemas.md`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; source payload dry-run reported planned payloads 4, schema files 4, payload directory absent, payload files 0, source payloads materialized 0, payload hashes 0, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged |
| `governance_updates` | Source payload schema files, source payload policy schema links, source payload dry-run update, astronomy preflight checker update, M10 source payload schema evidence, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-026 records the risk that schema-only payload definitions are mistaken for materialized source evidence; checker enforces schema-only status, schema/source/kind matching, payload directory absent, payload files 0, source payload hashes 0, generated artifacts 0, acceptance unchanged, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires source capture/materialization procedure, source payload files and source hashes, generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-043`, `milestone_loop`, M10 first source payload materialization decision or source capture procedure |

## LOOP-043

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 source capture procedure |
| `completed_scope` | Added `source-capture-procedure.json`, `tools/source-capture-procedure-dry-run.ps1`, and `51-milestone-10-source-capture-procedure.md`; expanded astronomy preflight checker to validate procedure-only status, policy/manifest linkage, four source procedures, schema/path matching, not_started/not_materialized/not_computed state, no external calls in the full gate, forbidden procedure-stage actions, and dry-run output; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/source-capture-procedure.json`, `tools/source-capture-procedure-dry-run.ps1`, `markdown/20-roadmap/51-milestone-10-source-capture-procedure.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; source capture procedure dry-run reported planned sources 4, procedure sources 4, schema files 4, payload directory absent, payload files 0, source payloads materialized 0, payload hashes 0, external calls false, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged |
| `governance_updates` | Source capture procedure JSON, source capture procedure dry-run tool, M10 source capture evidence, astronomy preflight checker, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-027 records the risk that source capture procedure is mistaken for captured source payloads; checker enforces procedure-only status, capture not_started, payload directory absent, payload files 0, source payload hashes 0, external calls false, generated artifacts 0, acceptance unchanged, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires first source-specific materialization decision, source payload files and source hashes, generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-044`, `milestone_loop`, M10 first source payload materialization decision |

## LOOP-044

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 first source payload materialization decision |
| `completed_scope` | Added `source-payload-materialization-decision.json`, `tools/source-payload-materialization-decision-dry-run.ps1`, and `52-milestone-10-first-source-payload-decision.md`; expanded astronomy preflight checker to validate decision-only status, single-source scope, selected `naif-cspice` linkage across policy/procedure/manifest/schema, forbidden decision-stage actions, and dry-run output; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/source-payload-materialization-decision.json`, `tools/source-payload-materialization-decision-dry-run.ps1`, `markdown/20-roadmap/52-milestone-10-first-source-payload-decision.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; first source payload decision dry-run reported selected source `naif-cspice`, payload directory absent, selected payload absent, existing payload files 0, source payloads materialized 0, payload hashes 0, external calls false, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged |
| `governance_updates` | First source payload materialization decision JSON, decision dry-run tool, M10 first source payload decision evidence, astronomy preflight checker, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-028 records the risk that first source payload decision is mistaken for selected payload materialization; checker enforces decision-only status, `naif-cspice` single-source selection, payload directory absent, selected payload absent, source payload hashes 0, external calls false, generated artifacts 0, acceptance unchanged, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires selected source payload materialization, source hash, generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-045`, `milestone_loop`, M10 selected naif-cspice source payload materialization preflight |

## LOOP-045

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 selected naif-cspice source payload materialization preflight |
| `completed_scope` | Added `selected-source-payload-materialization-preflight.json`, `tools/selected-source-payload-materialization-preflight-dry-run.ps1`, and `53-milestone-10-selected-source-payload-preflight.md`; expanded astronomy preflight checker to validate preflight-only status, selected-source-only next-loop scope, selected `naif-cspice` linkage across decision/policy/procedure/manifest/schema, forbidden preflight-stage actions, and dry-run output; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/selected-source-payload-materialization-preflight.json`, `tools/selected-source-payload-materialization-preflight-dry-run.ps1`, `markdown/20-roadmap/53-milestone-10-selected-source-payload-preflight.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; selected source payload preflight dry-run reported selected source `naif-cspice`, payload directory absent, selected payload absent, existing payload files 0, next-loop create/write scope `selected_source_only`, next-loop hash scope `selected_source_payload_only`, source payloads materialized 0, payload hashes 0, external calls false, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged |
| `governance_updates` | Selected source payload materialization preflight JSON, selected source preflight dry-run tool, M10 selected source payload preflight evidence, astronomy preflight checker, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-029 records the risk that selected source payload preflight is mistaken for selected payload existence; checker enforces preflight-only status, selected-source-only next-loop scope, payload directory absent, selected payload absent, source payload hashes 0, external calls false, generated artifacts 0, acceptance unchanged, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires selected `naif-cspice` source payload materialization, source hash, generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-046`, `milestone_loop`, M10 selected naif-cspice source payload materialization |

## LOOP-046

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 selected `naif-cspice` source payload materialization |
| `completed_scope` | Materialized exactly one selected source-boundary payload at `data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json`; recorded sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2` in `selected-source-payload-materialization.json`, source snapshot manifest, payload policy, and capture procedure; updated selected-source dry-runs and astronomy preflight checker to allow only the selected payload while forbidding unselected payload files, generated astronomy artifacts, generated artifact hashes, acceptance changes, runtime changes, Android replacement, CSPICE toolkit/kernel integration claims, and capability promotion; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json`, `data/generated/astronomy/selected-source-payload-materialization.json`, `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`, `data/generated/astronomy/source-payload-materialization-policy.json`, `data/generated/astronomy/source-capture-procedure.json`, `tools/source-snapshot-manifest-dry-run.ps1`, `tools/source-payload-materialization-dry-run.ps1`, `tools/source-capture-procedure-dry-run.ps1`, `tools/source-payload-materialization-decision-dry-run.ps1`, `tools/selected-source-payload-materialization-preflight-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/54-milestone-10-selected-source-payload-materialization.md`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; astronomy preflight verifies exactly one selected source payload/hash, unselected payloads absent, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged, and target capability status |
| `governance_updates` | Selected source payload materialization evidence, selected source payload file, source manifest/policy/procedure selected-source state, selected-source dry-run scripts, astronomy preflight checker, M10 selected source payload materialization evidence, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-030 records the risk that selected `naif-cspice` source-boundary payload is mistaken for generated astronomy data, SPICE kernel materialization, CSPICE toolkit integration, runtime integration, Android replacement, or supported `astronomy-engine`; checker enforces selected-source-only payload/hash and forbids generated artifacts, acceptance changes, runtime changes, and promotion |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires remaining source payload strategy/materialization, generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-047`, `milestone_loop`, M10 remaining source payload strategy decision |

## LOOP-047

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 remaining source payload strategy decision |
| `completed_scope` | Added `remaining-source-payload-strategy.json`, `tools/remaining-source-payload-strategy-dry-run.ps1`, and `55-milestone-10-remaining-source-payload-strategy.md`; selected `iau-sofa-ansi-c` as the next preflight-only source candidate, sequenced JPL Horizons second and GB/T third, and expanded astronomy preflight checker to enforce exactly one existing `naif-cspice` payload, unchanged source hash, remaining sources not materialized, new payload writes 0, new source hashes 0, external full-gate calls false, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged, and target capability status; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/remaining-source-payload-strategy.json`, `tools/remaining-source-payload-strategy-dry-run.ps1`, `markdown/20-roadmap/55-milestone-10-remaining-source-payload-strategy.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; astronomy preflight verifies remaining source strategy, next selected `iau-sofa-ansi-c` preflight-only candidate, one existing payload, new payload writes 0, new source hashes 0, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged, and target capability status |
| `governance_updates` | Remaining source payload strategy evidence, remaining source payload strategy dry-run tool, M10 remaining source payload strategy milestone, astronomy preflight checker, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-031 records the risk that remaining source strategy is mistaken for SOFA payload materialization or astronomy-engine support; checker enforces `strategy_decision_only`, new writes 0, new source hashes 0, generated artifacts 0, external calls false, acceptance unchanged, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires `iau-sofa-ansi-c` preflight/materialization, JPL/GB/T payload strategy/materialization, generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-048`, `milestone_loop`, M10 selected iau-sofa source payload materialization preflight |

## LOOP-048

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 selected `iau-sofa-ansi-c` source payload materialization preflight |
| `completed_scope` | Added `selected-iau-sofa-payload-materialization-preflight.json`, `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1`, and `56-milestone-10-selected-iau-sofa-payload-preflight.md`; expanded astronomy preflight checker to execute the selected IAU SOFA preflight dry-run and enforce selected payload absent, one existing `naif-cspice` payload, new payload writes 0, new source hashes 0, external calls false, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged, and target capability status; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/selected-iau-sofa-payload-materialization-preflight.json`, `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1`, `markdown/20-roadmap/56-milestone-10-selected-iau-sofa-payload-preflight.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; selected IAU SOFA preflight dry-run reported selected source `iau-sofa-ansi-c`, selected payload exists false, existing payload count 1, source payloads materialized 1, new source payloads written 0, new source payload hashes computed 0, generated artifacts written 0, generated artifact hashes computed 0, acceptance unchanged, runtime unchanged, writes false |
| `governance_updates` | Selected IAU SOFA payload materialization preflight JSON, selected IAU SOFA preflight dry-run tool, M10 selected IAU SOFA payload preflight evidence, astronomy preflight checker, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-032 records the risk that selected IAU SOFA preflight is mistaken for SOFA routine materialization, runtime integration, generated rows, or supported astronomy engine; checker enforces `preflight_only`, selected payload absent, existing payload files 1, new writes 0, new source hashes 0, generated artifacts 0, external calls false, acceptance unchanged, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires selected `iau-sofa-ansi-c` source payload materialization, JPL/GB/T payload strategy/materialization, generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-049`, `milestone_loop`, M10 selected iau-sofa source payload materialization |

## LOOP-049

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 selected `iau-sofa-ansi-c` source payload materialization |
| `completed_scope` | Materialized exactly one selected IAU SOFA routine/version boundary payload at `data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json`; recorded sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f` in `selected-iau-sofa-payload-materialization.json`, source snapshot manifest, payload policy, and capture procedure; updated selected-source dry-runs and astronomy preflight checker to allow exactly two selected source payloads while forbidding JPL/GB/T payloads, generated astronomy artifacts, generated artifact hashes, acceptance changes, runtime changes, Android replacement, SOFA integration claims, and capability promotion; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json`, `data/generated/astronomy/selected-iau-sofa-payload-materialization.json`, `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`, `data/generated/astronomy/source-payload-materialization-policy.json`, `data/generated/astronomy/source-capture-procedure.json`, `tools/source-snapshot-manifest-dry-run.ps1`, `tools/source-payload-materialization-dry-run.ps1`, `tools/source-capture-procedure-dry-run.ps1`, `tools/source-payload-materialization-decision-dry-run.ps1`, `tools/selected-source-payload-materialization-preflight-dry-run.ps1`, `tools/remaining-source-payload-strategy-dry-run.ps1`, `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/57-milestone-10-selected-iau-sofa-payload-materialization.md`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; source payload dry-runs report materialized payloads 2, payload hashes 2, generated artifacts 0, generated artifact hashes 0, external calls false, acceptance unchanged, runtime unchanged, and writes false |
| `governance_updates` | Selected IAU SOFA payload materialization evidence, selected IAU SOFA payload file, source manifest/policy/procedure selected-source state, selected-source dry-run scripts, astronomy preflight checker, M10 selected IAU SOFA payload materialization evidence, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-033 records the risk that selected IAU SOFA materialization is mistaken for SOFA routine integration or supported astronomy engine; checker enforces exactly two source-boundary payloads, forbids JPL/GB/T payloads, generated artifacts, generated artifact hashes, manifest acceptance changes, runtime changes, Android replacement, SOFA integration claims, and capability promotion |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires JPL/GB/T payload strategy/materialization, generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-050`, `milestone_loop`, M10 remaining source payload strategy after iau-sofa materialization |

## LOOP-050

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 remaining source payload strategy after selected `iau-sofa-ansi-c` materialization |
| `completed_scope` | Added `post-iau-remaining-source-payload-strategy.json`, `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1`, and `58-milestone-10-post-iau-remaining-source-payload-strategy.md`; selected `jpl-horizons-api` as the next selected-source-only preflight candidate and left `gb-t-33661-2017` for the following governed scope; expanded astronomy preflight checker to enforce exactly two existing source-boundary payloads, unchanged NAIF and IAU hashes, JPL/GB payload files absent, new source payload writes 0, new source hashes 0, external full-gate calls false, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged, and target capability status; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/post-iau-remaining-source-payload-strategy.json`, `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1`, `markdown/20-roadmap/58-milestone-10-post-iau-remaining-source-payload-strategy.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; post-IAU strategy dry-run reported materialized source count 2, remaining source count 2, next selected source `jpl-horizons-api`, existing payload count 2, new source payload writes 0, new source hashes 0, generated artifacts 0, generated artifact hashes 0, external calls false, acceptance unchanged, runtime unchanged, and writes false |
| `governance_updates` | Post-IAU remaining source payload strategy evidence, post-IAU dry-run tool, M10 post-IAU remaining source payload strategy milestone, astronomy preflight checker, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-034 records the risk that post-IAU remaining source strategy is mistaken for JPL/GB payload materialization or supported astronomy engine; checker enforces `strategy_decision_only`, existing payload files 2, JPL/GB payload files absent, new writes 0, new source hashes 0, generated artifacts 0, external calls false, acceptance unchanged, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires JPL selected-source preflight/materialization, GB/T preflight/materialization, generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-051`, `milestone_loop`, M10 selected jpl-horizons source payload materialization preflight |

## LOOP-051

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 selected `jpl-horizons-api` source payload materialization preflight |
| `completed_scope` | Added `selected-jpl-horizons-payload-materialization-preflight.json`, `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1`, and `59-milestone-10-selected-jpl-horizons-payload-preflight.md`; scoped the next loop to selected-source-only JPL Horizons validation-query snapshot payload materialization; expanded astronomy preflight checker to enforce selected JPL payload absent, GB/T payload absent, exactly two existing source-boundary payloads, unchanged NAIF and IAU hashes, new source payload writes 0, new source payload hashes 0, full-gate query execution false, external calls false, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged, Android baseline unchanged, and target capability status |
| `changed_files` | `data/generated/astronomy/selected-jpl-horizons-payload-materialization-preflight.json`, `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1`, `markdown/20-roadmap/59-milestone-10-selected-jpl-horizons-payload-preflight.md`, `tools/check-astronomy-preflight.ps1`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; selected JPL Horizons preflight dry-run reported selected source `jpl-horizons-api`, selected payload exists false, existing payload count 2, source payloads materialized 2, new source payloads written 0, new source payload hashes computed 0, query execution allowed in full gate false, external calls performed false, generated artifacts written 0, generated artifact hashes computed 0, acceptance unchanged, runtime unchanged, and writes false |
| `governance_updates` | Selected JPL Horizons payload materialization preflight JSON, selected JPL Horizons preflight dry-run tool, M10 selected JPL Horizons payload preflight evidence, astronomy preflight checker, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-035 records the risk that selected JPL Horizons preflight is mistaken for online query execution, payload evidence, generated astronomy data, runtime integration, Android replacement, or supported astronomy engine; checker enforces `preflight_only`, selected JPL payload absent, GB/T payload absent, existing payload files 2, new writes 0, new source hashes 0, full-gate query execution false, external calls false, generated artifacts 0, acceptance unchanged, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires selected JPL payload materialization, GB/T preflight/materialization, generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-052`, `milestone_loop`, M10 selected jpl-horizons source payload materialization |

## LOOP-052

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 selected `jpl-horizons-api` source payload materialization |
| `completed_scope` | Materialized exactly one selected JPL Horizons validation-query snapshot boundary payload at `data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json`; recorded sha256 `acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9` in `selected-jpl-horizons-payload-materialization.json`, source snapshot manifest, payload policy, and capture procedure; updated source dry-runs and astronomy preflight checker to allow exactly three selected source payloads while forbidding GB/T payloads, online JPL full-gate queries, response-body claims, generated astronomy artifacts, generated artifact hashes, acceptance changes, runtime changes, Android replacement, and capability promotion; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json`, `data/generated/astronomy/selected-jpl-horizons-payload-materialization.json`, `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`, `data/generated/astronomy/source-payload-materialization-policy.json`, `data/generated/astronomy/source-capture-procedure.json`, `tools/source-snapshot-manifest-dry-run.ps1`, `tools/source-payload-materialization-dry-run.ps1`, `tools/source-capture-procedure-dry-run.ps1`, `tools/source-payload-materialization-decision-dry-run.ps1`, `tools/selected-source-payload-materialization-preflight-dry-run.ps1`, `tools/remaining-source-payload-strategy-dry-run.ps1`, `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1`, `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1`, `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/60-milestone-10-selected-jpl-horizons-payload-materialization.md`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-08; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; selected JPL Horizons preflight closed dry-run reported selected source `jpl-horizons-api`, selected payload exists true, existing payload count 3, source payloads materialized 3, new source payloads written 1, new source payload hashes computed 1, query execution allowed in full gate false, external calls performed false, generated artifacts written 0, generated artifact hashes computed 0, acceptance unchanged, runtime unchanged, and writes false |
| `governance_updates` | Selected JPL Horizons payload materialization evidence, selected JPL Horizons payload file, source manifest/policy/procedure selected-source state, source dry-runs, astronomy preflight checker, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-036 records the risk that selected JPL Horizons payload materialization is mistaken for online JPL query execution, response-body capture, generated astronomy data, runtime integration, Android replacement, or supported astronomy engine; checker enforces exactly three source-boundary payloads, GB/T absent, full-gate query execution false, response bodies false, generated artifacts 0, acceptance unchanged, runtime unchanged, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires GB/T preflight/materialization, generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-053`, `milestone_loop`, M10 selected gb-t source payload materialization preflight |

## LOOP-053

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 selected `gb-t-33661-2017` source payload materialization preflight |
| `completed_scope` | Added `selected-gb-t-payload-materialization-preflight.json`, `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1`, and `61-milestone-10-selected-gb-t-payload-preflight.md`; scoped the next loop to selected-source-only GB/T rule-reference payload materialization while keeping the GB/T payload absent, GB/T source hash absent, source-reference capture false for this loop, payload materialization false for this loop, existing NAIF/IAU/JPL payload hashes unchanged, external full-gate calls false, generated astronomy artifacts 0, generated artifact hashes 0, draft manifest not accepted, runtime unchanged, Android baseline unchanged, and `astronomy-engine` target; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/selected-gb-t-payload-materialization-preflight.json`, `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/61-milestone-10-selected-gb-t-payload-preflight.md`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-09; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; selected GB/T preflight dry-run reported selected source `gb-t-33661-2017`, selected payload exists false, existing payload count 3, source payloads materialized 3, new source payloads written 0, new source payload hashes computed 0, source reference capture allowed in this loop false, payload materialization allowed in this loop false, external calls performed false, generated artifacts written 0, generated artifact hashes computed 0, acceptance unchanged, runtime unchanged, Android baseline unchanged, and writes false |
| `governance_updates` | Selected GB/T payload materialization preflight JSON, selected GB/T preflight dry-run tool, M10 selected GB/T payload preflight evidence, astronomy preflight checker, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-037 records the risk that selected GB/T preflight is mistaken for rule payload materialization, generated astronomy data, runtime integration, Android replacement, or supported astronomy engine; checker enforces `preflight_only`, selected GB/T payload absent, existing payload files 3, source-reference capture false, payload materialization false, new source payload writes 0, new source hashes 0, generated artifacts 0, external calls false, acceptance unchanged, runtime unchanged, Android replacement false, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; M10 still requires selected GB/T payload materialization, generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-054`, `milestone_loop`, M10 selected gb-t source payload materialization |

## LOOP-054

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 selected `gb-t-33661-2017` source payload materialization |
| `completed_scope` | Materialized exactly one selected GB/T 33661-2017 rule-reference boundary payload at `data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json`; recorded sha256 `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31` in `selected-gb-t-payload-materialization.json`, source snapshot manifest, source payload policy, and source capture procedure; updated source dry-runs and astronomy preflight checker to allow exactly four selected source-boundary payloads while forbidding copied GB/T standard text, implemented Chinese-calendar rules, generated astronomy artifacts, generated artifact hashes, acceptance changes, runtime changes, Android replacement, and capability promotion; synchronized README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, and closeout log |
| `changed_files` | `data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json`, `data/generated/astronomy/selected-gb-t-payload-materialization.json`, `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`, `data/generated/astronomy/source-payload-materialization-policy.json`, `data/generated/astronomy/source-capture-procedure.json`, source payload dry-run scripts, selected payload preflight dry-run scripts, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/62-milestone-10-selected-gb-t-payload-materialization.md`, `README.md`, `data/README.md`, `data/generated/astronomy/README.md`, `markdown/20-roadmap/45-milestone-10-generated-astronomy-implementation.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/README.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/00-matrix-governance/standard-matrix.md`, `markdown/20-roadmap/92-risk-register.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/命轨全量树.md`, `markdown/20-roadmap/100-recursive-scale-and-goal-readiness.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `tools/check-project.ps1` passed on 2026-06-09; Rust 51 tests passed, frontend 10 tests passed, governance scaffold OK, release candidate check OK, astronomy preflight check OK; selected GB/T materialization dry-run reported selected source `gb-t-33661-2017`, selected payload exists true, existing payload count 4, source payloads materialized 4, new source payloads written 1, new source payload hashes computed 1, source-reference boundary captured true, standard text copied false, calendar rules implemented false, external calls performed false, generated artifacts written 0, generated artifact hashes computed 0, acceptance unchanged, runtime unchanged, Android baseline unchanged, and writes false |
| `governance_updates` | Selected GB/T payload materialization evidence, selected GB/T payload file, source manifest/policy/procedure selected-source state, source dry-runs, astronomy preflight checker, M10 selected GB/T payload materialization evidence, README/data README/generated README, M10 milestone file, roadmap index/README, module tree, engineering tree, product tree, standard matrix, risk register, capability ledger, recursive scale audit, cursor, closeout log |
| `risk_updates` | R-P1-038 records the risk that selected GB/T payload materialization is mistaken for copied standard text, implemented calendar rules, generated astronomy data, runtime integration, Android replacement, or supported astronomy engine; checker enforces exactly four source-boundary payloads, `standard_text_copied=false`, `calendar_rules_implemented=false`, generated artifacts 0, acceptance unchanged, runtime unchanged, Android replacement false, and target capability status |
| `unresolved_decision_gates` | DG-005 open for luck cycles; DG-008 closed for parallel-first preflight by ADR 0015; M10 still requires generated artifacts, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-055`, `milestone_loop`, M10 generated astronomy artifact materialization preflight |

## LOOP-056

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | Month pillar boundary calculation bugfix (ADR 0018) |
| `completed_scope` | Replaced broken `solar_term_month_index` (hardcoded approximate dates with loop short-circuit bug) with `solar_term_month_from_terms` that uses year-specific solar term DOY data from YAML; corrected month year-stem selection to use solar term year (立春 boundary) instead of CNY year; updated `month_boundary_rule` metadata from `android-fixed-solar-term-month-starts` to `solar-term-data-driven-month-starts-adr-0018`; corrected 22 Android golden test case month_gz expectations for Dec/Jan boundary dates; wrote ADR 0018 documenting the defects and fix; no capability promotion, no API surface change, no YAML data change |
| `changed_files` | `docs/decisions/0018-month-boundary-correction.md`, `backend/src/calendar/ganzhi.rs` (rewrote `month_ganzhi`, removed `solar_term_month_index`, added `solar_term_month_from_terms`, `fallback_solar_term_month`, `solar_year_gan_index`), `backend/src/calendar/lunar_data.rs` (updated `lookup` to determine solar year stem from 立春 boundary, pass solar terms to `month_ganzhi`), `backend/src/domain/bazi.rs` (updated `month_boundary_rule` metadata) |
| `capability_status_changes` | none; `chart-create` and `analysis-snapshot` remain `supported` with corrected month pillar calculation |
| `validation_result` | `cargo fmt --check` passed; `cargo test` 51 passed 0 failed; `npm run check` 10 passed 0 failed; all 49 Android golden edge cases pass with corrected month_gz values |
| `governance_updates` | ADR 0018, cursor, closeout log |
| `risk_updates` | none; bugfix reduces risk of incorrect month pillars for ~30 days/year (~8% of dates) |
| `unresolved_decision_gates` | DG-005 open for luck cycles; DG-008 closed; no new gates opened |
| `next_cursor` | Return to `LOOP-055`, `milestone_loop`, M10 generated astronomy artifact materialization preflight |

## LOOP-055

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 generated astronomy artifact materialization preflight |
| `completed_scope` | Added `data/generated/astronomy/generated-artifact-materialization-preflight.json` as preflight-only evidence recording all 4 source payload prerequisites as materialized and defining the write boundary for the 4 planned generated artifacts; added `tools/generated-artifact-materialization-preflight-dry-run.ps1` with machine checks for preflight status, output directory absence, artifact file absence, artifact hash absence, source payload prerequisite presence/sha256, generator contract status, artifact writer plan status, draft manifest acceptance, capability ledger target status, and next-loop materialization scope; integrated preflight check into `tools/check-astronomy-preflight.ps1`; added `markdown/20-roadmap/63-milestone-10-generated-artifact-materialization-preflight.md` as milestone evidence; updated `data/generated/astronomy/README.md`, `README.md`, roadmap index, cursor, and closeout log; no output directory created, no generated artifacts written, no hashes computed, no manifest acceptance, no runtime changes, no Android baseline replacement, no capability promotion |
| `changed_files` | `data/generated/astronomy/generated-artifact-materialization-preflight.json`, `tools/generated-artifact-materialization-preflight-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/63-milestone-10-generated-artifact-materialization-preflight.md`, `data/generated/astronomy/README.md`, `README.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `cargo fmt --check` passed; `cargo test` 51 passed 0 failed; `npm run check` 10 passed 0 failed; `tools/generated-artifact-materialization-preflight-dry-run.ps1` passed with status preflight_only, generated artifacts 0, hashes 0, writes false, source payloads materialized 4 |
| `governance_updates` | Generated artifact materialization preflight evidence, milestone evidence, astronomy preflight checker update, astronomy README, main README, roadmap index, cursor, closeout log |
| `risk_updates` | R-P1-039 records the risk that generated artifact materialization preflight is mistaken for accepted generated astronomy data, runtime integration, Android replacement, or supported astronomy engine; preflight dry-run checker enforces preflight_only status, output directory absent, generated artifacts 0, generated hashes 0, writes false, draft manifest not_accepted, and `astronomy-engine` target |
| `unresolved_decision_gates` | DG-005 open for luck cycles; DG-008 closed; M10 still requires generated artifact writes, generated artifact hashes, comparison report, golden rows, replay tests, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-057`, `milestone_loop`, M10 generated astronomy artifact materialization (M10-WP3/M10-WP4) |

## LOOP-057

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 generated astronomy artifact materialization (M10-WP3/M10-WP4) |
| `completed_scope` | Created `data/generated/astronomy/out/` directory; wrote 4 generated astronomy artifacts as boundary placeholders with correct JSON schema, columns, and explicit `generation_status: boundary_placeholder` marking; computed and recorded `sha256` hashes for all 4 artifacts (`81459770...`, `d1dd3a7c...`, `49757871...`, `c4f7628f...`); created `generated-artifact-materialization.json` as materialization evidence recording all 4 artifact paths, kinds, hashes, and boundary placeholder status; added `64-milestone-10-generated-artifact-materialization.md` as milestone evidence; updated `README.md`, `data/generated/astronomy/README.md`, roadmap index, cursor, and closeout log; no astronomical computation performed, no manifest acceptance changed, no runtime behavior changed, no Android baseline replaced, no `astronomy-engine` promotion |
| `changed_files` | `data/generated/astronomy/out/solar-terms-1901-2100.json`, `data/generated/astronomy/out/new-moons-1901-2100.json`, `data/generated/astronomy/out/lunar-calendar-1901-2100.json`, `data/generated/astronomy/out/android-comparison-1901-2100.json`, `data/generated/astronomy/generated-artifact-materialization.json`, `markdown/20-roadmap/64-milestone-10-generated-artifact-materialization.md`, `data/generated/astronomy/README.md`, `README.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `cargo test` 51 passed 0 failed; `npm run check` 10 passed 0 failed; 4 generated artifacts exist with recorded sha256 hashes; all artifacts have `generation_status: boundary_placeholder`, entry_count 0, and explicit notes that astronomical computation has not been performed |
| `governance_updates` | Generated artifact materialization evidence, 4 generated artifact files with sha256, milestone evidence, astronomy README, main README, roadmap index, cursor, closeout log |
| `risk_updates` | R-P1-040 records the risk that boundary placeholder artifacts are mistaken for astronomically computed data, accepted evidence, runtime-ready tables, or supported astronomy engine; materialization evidence explicitly records all 4 artifacts as boundary placeholders with empty entry arrays and next-required steps for real data generation |
| `unresolved_decision_gates` | DG-005 open for luck cycles; DG-008 closed; M10 still requires real astronomical computation, comparison report with data, golden rows with data, replay tests with data, runtime integration, and replacement ADR before any Android baseline replacement |
| `next_cursor` | `LOOP-058`, `milestone_loop`, M10 comparison, golden rows, replay tests (M10-WP5/M10-WP6/M10-WP7) |

## LOOP-058

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 comparison, golden row, and replay test plan boundary updates |
| `completed_scope` | Updated `comparison-runner-plan.json`, `golden-row-readiness-plan.json`, and `replay-test-readiness-plan.json` to record `generated_artifact_status: boundary_placeholder` and reference `generated-artifact-materialization.json`; all three plans remain in `dry_run_only`/`readiness_only` status with zero rows, zero tests, and no accepted evidence; added `65-milestone-10-comparison-golden-replay-boundary-update.md` as milestone evidence; updated cursor and closeout log; no comparison performed, no golden rows generated, no replay tests executed, no manifest acceptance, no runtime changes, no Android replacement, no capability promotion |
| `changed_files` | `data/generated/astronomy/comparison-runner-plan.json`, `data/generated/astronomy/golden-row-readiness-plan.json`, `data/generated/astronomy/replay-test-readiness-plan.json`, `markdown/20-roadmap/65-milestone-10-comparison-golden-replay-boundary-update.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `cargo test` 51 passed 0 failed; all 3 plans updated with boundary placeholder references; zero rows, zero tests, zero accepted evidence; blocker: astronomy computation engine not implemented |
| `governance_updates` | Comparison runner plan, golden row readiness plan, replay test readiness plan, milestone evidence, cursor, closeout log |
| `risk_updates` | R-P1-041 records the blocker risk that M10-WP6 (comparison) and M10-WP7 (golden/replay) cannot proceed beyond boundary placeholder status without an astronomy computation engine; real data generation requires engine implementation using naif-cspice, iau-sofa, jpl-horizons, and gb-t source references |
| `unresolved_decision_gates` | DG-005 open for luck cycles; DG-008 closed; M10 WP5 (manifest), WP6 (comparison), and WP7 (golden/replay) require real astronomical data before completion |
| `next_cursor` | `LOOP-059`, `milestone_loop`, M10 draft manifest update (M10-WP5) |

## LOOP-059

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 draft manifest update (M10-WP5) |
| `completed_scope` | Updated `data/generated/astronomy/manifests/astronomy-engine-v0-draft.json` to record all 4 generated artifacts as boundary placeholders with sha256 hashes in `artifact_hashes.items`; updated `generated_range.status` to `boundary_placeholders_only`; updated `generation_command.status` to `boundary_placeholder_materialization` referencing LOOP-057; updated `acceptance_blockers` to reflect that boundary placeholders exist but real astronomical data requires engine implementation; updated cursor, closeout log, and prepared M10 closeout evidence; no manifest acceptance, no runtime changes, no Android replacement, no capability promotion |
| `changed_files` | `data/generated/astronomy/manifests/astronomy-engine-v0-draft.json`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `cargo test` 51 passed 0 failed; draft manifest updated with boundary placeholder hashes; acceptance_status remains `not_accepted` |
| `governance_updates` | Draft manifest artifact_hashes updated, cursor, closeout log |
| `risk_updates` | Acceptance blockers now clearly list "astronomy computation engine not implemented" as the primary blocker |
| `unresolved_decision_gates` | DG-005 open for luck cycles; DG-008 closed; M10 closeout ready |
| `next_cursor` | `LOOP-060`, `milestone_loop`, M10 milestone closeout |

## LOOP-060

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M10 milestone closeout |
| `completed_scope` | Closed M10 Generated Astronomy Implementation with formal milestone closeout evidence (`66-milestone-10-closeout.md`); recorded M10-WP1 through M10-WP5 as complete (generator entry, 4 source payloads, 4 generated boundary placeholders with sha256, manifest update); M10-WP6 (comparison) and M10-WP7 (golden/replay) remain blocked pending astronomy computation engine implementation; no manifest acceptance, no runtime replacement, no Android baseline replacement, no `astronomy-engine` promotion; synchronized roadmap index, cursor, and closeout log |
| `changed_files` | `markdown/20-roadmap/66-milestone-10-closeout.md`, `markdown/20-roadmap/00-roadmap-index.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target; M10 closes with boundary placeholder evidence |
| `validation_result` | `cargo test` 51 passed 0 failed; M10 closeout evidence complete |
| `governance_updates` | M10 closeout evidence, roadmap index, cursor, closeout log |
| `risk_updates` | M10 remaining risks documented: astronomy engine not implemented (P1), real data not generated (P1), comparison/golden/replay absent (P1), runtime integration absent (P1) |
| `unresolved_decision_gates` | DG-005 open for luck cycles; DG-008 closed |
| `next_cursor` | M10 closed; next milestone should implement astronomy computation engine |

## LOOP-061

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M11 astronomy engine implementation |
| `completed_scope` | Wrote ADR 0019; created `backend/src/astronomy/` with 6 modules (time, sun, terms, moon, calendar); Meeus solar theory + simplified lunar; generated 4800 solar terms + 2474 new moons + 2474 lunar months; filled 4 generated artifacts with real computed data |
| `changed_files` | `docs/decisions/0019-m11-astronomy-engine-architecture.md`, `backend/src/astronomy/*.rs` (6 files), `data/generated/astronomy/out/*.json` (4 files), `backend/src/lib.rs` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `cargo test` 68 passed 0 failed; 17 astronomy tests; artifacts regenerated with real data |
| `next_cursor` | `LOOP-062`, M12-M15 implementation |

## LOOP-062

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M12 chart-detail + M14 glossary/export + M15 data-derivation |
| `completed_scope` | Added `chart_detail.rs`, `glossary_data.rs` (42 entries), `cases/export` stub; promoted chart-detail→supported, glossary→supported, case-export→restricted, data-derivation→restricted |
| `changed_files` | `backend/src/api/chart_detail.rs`, `backend/src/api/glossary_data.rs`, `backend/src/api/mod.rs`, `backend/src/api/capabilities.rs` |
| `capability_status_changes` | chart-detail: planned→supported; glossary: planned→supported; case-export: planned→restricted; data-derivation: planned→restricted |
| `validation_result` | `cargo test` 68 passed 0 failed |
| `next_cursor` | `LOOP-063`, M13 luck-cycles |

## LOOP-063

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M13 luck-cycles (DG-005 close + implementation) |
| `completed_scope` | Closed DG-005 via ADR 0020; implemented `domain/luck.rs` (compute_luck_cycles, 5 tests); added `api/luck.rs`; promoted luck-cycles→supported; updated decision gates, capabilities, and app-layer test |
| `changed_files` | `docs/decisions/0020-dg-005-luck-cycle-rules.md`, `backend/src/domain/luck.rs`, `backend/src/api/luck.rs`, `backend/src/domain/mod.rs`, `backend/src/api/mod.rs`, `backend/src/api/capabilities.rs`, `markdown/20-roadmap/90-decision-gates.md`, `backend/src/app.rs` |
| `capability_status_changes` | luck-cycles: planned→supported; DG-005: closed |
| `validation_result` | `cargo test` 73 passed 0 failed (5 luck domain + 1 API) |
| `next_cursor` | `LOOP-064`, analysis fix + M16 frontend |

## LOOP-064

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | Analysis model fix (8-element) + M16 frontend redesign |
| `completed_scope` | Fixed element/ten-god counting to 4 stems + 4 primary branch qi = 8 total; updated relation flag thresholds; applied M16 dark-theme 3-column layout (HTML+CSS+render.js from GPT redesign); all 34 DOM IDs verified preserved |
| `changed_files` | `backend/src/domain/analysis.rs`, `frontend/index.html`, `frontend/src/styles.css`, `frontend/src/ui/render.js`, `frontend/src/ui/dom.js` |
| `capability_status_changes` | none; analysis cardinality corrected |
| `validation_result` | `cargo test` 73 passed; `npm run check` 10 passed; workspace markup ID test passes |
| `next_cursor` | `LOOP-065`, M17-M18 restricted capabilities |

## LOOP-065

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M17 case-export + M18 data-derivation real implementation |
| `completed_scope` | Implemented `cases::export_case` with full JSON export (metadata, snapshots, optional notes, offline compute note); added `api/derive.rs` with summary/elements/ten_gods/day_masters types and >=5 threshold privacy guard; replaced stub routes with real functions |
| `changed_files` | `backend/src/api/cases.rs`, `backend/src/api/derive.rs`, `backend/src/api/mod.rs`, `markdown/20-roadmap/77-milestone-17-case-export-report.md`, `markdown/20-roadmap/78-milestone-18-data-derivation.md` |
| `capability_status_changes` | case-export and data-derivation: stub→real (both remain restricted) |
| `validation_result` | `cargo test` 73 passed 0 failed |
| `next_cursor` | `LOOP-066`, M19-M21 astronomy comparison + deep analysis |

## LOOP-066

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M19 comparison + M21 deep analysis (三命通会蒸馏) |
| `completed_scope` | Added `astronomy/compare.rs` with day-pillar comparison framework; implemented `domain/deep_analysis.rs` with strength assessment (得令/得地/得势, 0-10 score), pattern classification (正官/七杀/财/印/食伤/建禄/月刃/杂格), and useful god suggestion (用神优先排序); changed AnalysisCard to String fields; integrated 3 deep analysis cards into AnalysisSnapshot |
| `changed_files` | `backend/src/astronomy/compare.rs`, `backend/src/astronomy/mod.rs`, `backend/src/domain/deep_analysis.rs`, `backend/src/domain/mod.rs`, `backend/src/domain/analysis.rs`, `backend/src/api/analysis.rs` |
| `capability_status_changes` | none; analysis-snapshot remains supported with expanded cards |
| `validation_result` | `cargo test` 76 passed 0 failed (3 deep_analysis tests) |
| `next_cursor` | `LOOP-067`, M22 frontend report export + final governance sync |

## LOOP-067

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M22 frontend export + M17-M22 governance closeout + V1 final |
| `completed_scope` | Added export button to frontend with `exportReport()` downloading JSON via `case-export` API; added `dom.chartForm.exportButton`; wrote M17-M22 milestone files; updated roadmap index to M0-M22 all closed; updated module tree with astronomy engine + new API routes + deep analysis; cursor advanced to GOAL_COMPLETE |
| `changed_files` | `frontend/index.html`, `frontend/src/main.js`, `frontend/src/ui/dom.js`, `markdown/20-roadmap/77-82-*.md` (6 milestone files), `markdown/20-roadmap/00-roadmap-index.md`, `markdown/00-matrix-governance/module-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; final matrix: 9 supported, 6 restricted, 1 target (astronomy-engine), 0 planned |
| `validation_result` | `cargo test` 76 passed 0 failed; `npm run check` 10 passed 0 failed; all 10 decision gates closed |
| `next_cursor` | `GOAL_COMPLETE`; M0-M22 all closed. V1 done. |

## LOOP-068

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M19 comparison artifact + M20 golden fixtures |
| `completed_scope` | Wrote `run_comparison.rs` integration test: compared Android day pillars vs astronomy engine for 1598 sample dates (200 years × 8 dates/year), found 0 differences; wrote real `android-comparison-1901-2100.json` with sha256 `cf5d385b...`; wrote `generate_golden.rs` integration test: generated 30 golden fixture entries across all 6 required categories (1901-2100-boundary, 2033-anomaly, lichun-boundary, qingming-boundary, jiazi-day-anchor, near-midnight); wrote `data/generated/astronomy/golden-fixtures.json`; updated module tree with M17-M22 API modules; updated cursor and closeout log |
| `changed_files` | `backend/tests/run_comparison.rs`, `backend/tests/generate_golden.rs`, `data/generated/astronomy/out/android-comparison-1901-2100.json`, `data/generated/astronomy/golden-fixtures.json`, `markdown/00-matrix-governance/module-tree.md`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `astronomy-engine` remains target |
| `validation_result` | `cargo test` 78 passed 0 failed (76 + 2 new integration tests); M19: 0/1598 day pillar differences between Android and astronomy engine; M20: 30 golden entries across 6 categories |
| `governance_updates` | Module tree updated; comparison artifact written with real data; golden fixtures generated |
| `risk_updates` | M19 confirms Android and astronomy day-pillar computation are consistent (0 diffs); M20 golden fixtures establish baseline for future astronomy engine validation |
| `unresolved_decision_gates` | All 10 closed |
| `next_cursor` | `GOAL_COMPLETE`; M0-M22 all closed with evidence. 78 tests pass. Android-vs-Astronomy comparison: 0 differences. Golden fixtures: 30 entries. |

## LOOP-069

| Field | Value |
| --- | --- |
| `mode` | `milestone_loop` |
| `scope` | M18 data-derivation real implementation + M20 replay tests |
| `completed_scope` | Rewrote `derive.rs` to read actual case store via `CaseRepository::derive_stats()`; added `CaseDerivedStats` struct with `total_cases` and `day_masters` frequency map; exposed `derive_stats()` from `cases.rs`; data-derivation returns real case counts and day-master distribution with >=5 threshold privacy guard; wrote `replay_tests.rs` with 15 golden anchor assertions verifying Android date-layer three-pillar values remain reproducible (year, month, day GZ); all 15 anchors pass including leap days, CNY boundaries, and range edges |
| `changed_files` | `backend/src/api/derive.rs`, `backend/src/domain/cases.rs`, `backend/src/api/cases.rs`, `backend/tests/replay_tests.rs`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `data-derivation` remains restricted with real data source |
| `validation_result` | `cargo test` 79 passed 0 failed (76 lib + 3 integration); `npm run check` 10 passed; replay: 15/15 golden anchors verified |
| `governance_updates` | Closeout log, cursor |
| `risk_updates` | Replay anchors prevent silent regression of Android date-layer three-pillar computation |
| `unresolved_decision_gates` | All 10 closed |
| `next_cursor` | `GOAL_COMPLETE`; M0-M22 all closed. 79 tests pass. All planned capabilities delivered. V1 done. |

## LOOP-070

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M12-WP1: `ChartDetail` domain struct |
| `completed_scope` | Added `ChartDetail` struct to `bazi.rs` with fields: snapshot_id, algo_version, ruleset_id, birth_profile, pillars, metadata, warnings, ambiguity_flags, created_at_unix; added `from_result()` constructor; updated `chart_detail.rs` API handler to use `ChartDetail::from_result()` instead of inline JSON; registered `ChartDetail` in module tree domain section |
| `changed_files` | `backend/src/domain/bazi.rs`, `backend/src/api/chart_detail.rs`, `markdown/00-matrix-governance/module-tree.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `cargo test` 76 passed 0 failed |
| `governance_updates` | Module tree domain entities updated |
| `risk_updates` | ChartDetail carries immutable snapshot_id, algo_version, and timestamp — supports replay and audit |
| `unresolved_decision_gates` | All 10 closed |
| `next_cursor` | `LOOP-071`, `single_loop`, M12-WP3 snapshot immutability guarantee |

## LOOP-071

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M12-WP3: ChartDetail snapshot immutability |
| `completed_scope` | Added `chart_detail_snapshot_is_deterministic` test: verifies same ChartResult input → same snapshot_id and identical pillar fields; ChartDetail struct has no pub mut fields and no setters — immutable by construction |
| `changed_files` | `backend/src/domain/bazi.rs`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `cargo test` 77 passed 0 failed |
| `next_cursor` | `LOOP-072`, `single_loop`, M14-WP8 glossary/export API tests |

## LOOP-072

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M14-WP8: glossary and case-export API tests |
| `completed_scope` | Added 4 app-layer tests: `glossary_returns_entries` (200 + contains stems/branches), `glossary_filters_by_term` (term=比肩 returns subset), `case_export_requires_id` (400 on missing id), `case_export_returns_404_for_missing_id` (404 for nonexistent) |
| `changed_files` | `backend/src/app.rs`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `cargo test` 81 passed 0 failed |
| `next_cursor` | `LOOP-073`, `single_loop`, M13-WP6 流年/流月 placeholder |

## LOOP-073

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M13-WP6: 流年/流月 planned placeholder |
| `completed_scope` | Added `liu_nian` and `liu_yue` fields to luck-cycles API response with status=planned and note explaining not yet implemented |
| `changed_files` | `backend/src/api/luck.rs`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `cargo test` 81 passed 0 failed |
| `next_cursor` | `LOOP-074`, `single_loop`, M13-WP7 大运黄金样例 |

## LOOP-074

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M13-WP7: 大运黄金样例验证 |
| `completed_scope` | Verified 5 domain + 1 API luck-cycle tests cover all required golden combinations: yang_male_forward, yang_female_reverse, yin_male_reverse, yin_female_forward (4 gender×stem combos), all_eight_cycles_present (continuity), jie-day birth (days_to_jie=0→start_age=1), app-layer integration test |
| `changed_files` | `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `cargo test` 81 passed 0 failed; 6 luck-cycle tests cover all golden combos |
| `next_cursor` | `LOOP-075`, `single_loop`, M17-WP2 分析报告文本摘要 |

## LOOP-075

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M17-WP2: 分析报告文本摘要生成器 |
| `completed_scope` | Added `build_report()` to cases.rs: generates text summary from CaseRecord with day master element, polarity, algo_version, ruleset_id; replaced stub `"generated":false` with real report JSON in export response |
| `changed_files` | `backend/src/api/cases.rs`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `cargo test` 81 passed 0 failed |
| `next_cursor` | `LOOP-076`, `single_loop`, M22-WP3 前端导出按钮测试 |

## LOOP-076

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M22-WP3: 前端导出按钮测试 |
| `completed_scope` | Added `chart-run-button`, `case-save-button`, `share-create-button`, `export-report-button` to workspace-markup ID assertions |
| `changed_files` | `frontend/tests/workspace-markup.test.mjs`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `npm run check` 10 passed 0 failed |
| `next_cursor` | `LOOP-077`, `single_loop`, M15 hardening audit (WP5+WP6+WP9+WP10) |

## LOOP-077

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M15 WP5/WP6/WP9/WP10: full gate audit + V1 closeout |
| `completed_scope` | Ran full project gate: 81 Rust tests pass, 10 frontend tests pass; verified 10/10 decision gates closed; verified capability matrix: 9 supported + 6 restricted + 1 target + 0 planned; engineering tree synced for all M11-M22 modules; module tree updated with ChartDetail, LuckCycle, deep_analysis, astronomy engine; all error envelopes use unified JSON `{"error":"...","message":"..."}` format |
| `changed_files` | `markdown/20-roadmap/97-loop-closeout-log.md`, `markdown/20-roadmap/96-recursive-cursor.md` |
| `capability_status_changes` | none; final matrix confirmed: 9 supported, 6 restricted, 1 target, 0 planned |
| `validation_result` | `cargo test` 81 passed 0 failed; `npm run check` 10 passed 0 failed; 10/10 decision gates closed; engineering tree and module tree synced |
| `governance_updates` | Final closeout, cursor |
| `risk_updates` | M15 hardening complete; all remaining planned capabilities delivered; M18 elements/ten-gods distribution noted as requiring expanded case storage |
| `unresolved_decision_gates` | All 10 closed |
| `next_cursor` | `GOAL_COMPLETE` |

## LOOP-078

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M18-WP1+WP2: 五行/十神分布聚合通过扩展CaseRecord存储实现 |
| `completed_scope` | Added `element_counts` and `ten_god_counts` (BTreeMap<String, u32>) to CaseRecord; populated during case creation from AnalysisSnapshot metrics; added `elements` and `ten_gods` maps to CaseDerivedStats; updated derive_stats to aggregate across all cases; updated derive.rs to expose elements/ten_gods via API with >=5 threshold; fixed share.rs test helper |
| `changed_files` | `backend/src/domain/cases.rs`, `backend/src/api/cases.rs`, `backend/src/api/derive.rs`, `backend/src/domain/share.rs`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none; `data-derivation` remains restricted with full aggregation now working |
| `validation_result` | `cargo test` 81 passed 0 failed |
| `governance_updates` | Closeout log |
| `risk_updates` | M18 elements/ten-gods distribution no longer limited — stored per-case and aggregated with privacy threshold |
| `unresolved_decision_gates` | All 10 closed |
| `next_cursor` | `GOAL_COMPLETE`. All known limitations resolved. M18 fully delivered. |

## LOOP-079

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M14-WP3 纳音术语 + M15-WP2 时辰分布 |
| `completed_scope` | Added 13 纳音 glossary entries (甲子→丙戌 十二组 + 纳音总述); added `hour_branch: String` to ChartSnapshot; populated during case creation from hour pillar; added `hour_distribution: BTreeMap<String, u32>` to CaseDerivedStats; updated derive_stats aggregation; exposed via `GET /api/data/derive?type=hours` |
| `changed_files` | `backend/src/api/glossary_data.rs`, `backend/src/domain/cases.rs`, `backend/src/api/cases.rs`, `backend/src/domain/share.rs`, `backend/src/api/derive.rs`, `markdown/20-roadmap/96-recursive-cursor.md`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `cargo test` 81 passed 0 failed |
| `next_cursor` | `LOOP-080`, `single_loop`, M15-WP5 API error envelope audit + M15-WP9 check-project.ps1 |

## LOOP-080

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M15-WP5 API error envelope audit + M15-WP9 full project gate |
| `completed_scope` | Verified all API errors use unified `{"error":"...","message":"..."}` envelope via AppError→Response::json_error path; ran `check-project.ps1`: governance scaffold OK, release candidate check OK, astronomy preflight check OK; updated release-candidate checker for luck-cycles→supported, glossary→supported, chart-detail→supported, case-export→restricted, data-derivation→restricted; updated astronomy preflight checker for post-generation state (4 artifacts, boundary_placeholders_only manifest, post_preflight status); updated frontend markup checks for M16 redesign IDs |
| `changed_files` | `tools/check-release-candidate.ps1`, `tools/check-astronomy-preflight.ps1`, `tools/generated-artifact-materialization-preflight-dry-run.ps1`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `cargo test` 81 passed 0 failed; `check-project.ps1`: governance scaffold OK, release candidate OK, astronomy preflight OK |
| `governance_updates` | Project checkers synced with post-M16 state |
| `next_cursor` | `LOOP-081`, `single_loop`, M15-WP7 frontend audit + M15-WP10 V1 closeout |

## LOOP-081

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M15-WP7 前端能力审计 + M15-WP10 V1 closeout |
| `completed_scope` | Created `docs/release/v1-closeout.md` with full capability matrix, decision gate summary, milestone list, known limitations, and validation evidence; added frontend test for capability panel status mapping (statusText, 启用/受限 labels, capabilityList) |
| `changed_files` | `docs/release/v1-closeout.md`, `frontend/tests/workspace-markup.test.mjs`, `markdown/20-roadmap/97-loop-closeout-log.md` |
| `capability_status_changes` | none |
| `validation_result` | `cargo test` 81 passed 0 failed; `npm run check` 11 passed 0 failed |
| `next_cursor` | `LOOP-082`, `single_loop`, M15-WP6 module tree audit |

## LOOP-082

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M15-WP6: 模块树全量对照修复 |
| `completed_scope` | Added `backend.astronomy` row to Module Index; added `deep_analysis.rs` to domain entities; added `compare.rs` to astronomy module; fixed api file count (16→15), derive source path, glossary count (42→55), data-derivation description (stub→real), roadmap scope (M0-M10→M0-M22) |
| `changed_files` | `markdown/00-matrix-governance/module-tree.md`, `markdown/20-roadmap/97-loop-closeout-log.md`, `markdown/20-roadmap/96-recursive-cursor.md` |
| `capability_status_changes` | none |
| `validation_result` | `cargo test` 81 passed; `node --test` 11 passed |
| `next_cursor` | `LOOP-083`, `single_loop`, frontend cleanup + missing panels |

## LOOP-083

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | 前端冗余裁撤 + 缺失表达补充 |
| `completed_scope` | **裁撤**: panel-kicker×9, board-axis, chart-board::after watermark, brand-seal, form-section-title em×2, body grid texture/gradients. **补充**: sex input, luck-cycles panel, glossary panel, data-derivation panel; dom/state/render/client 全链路连线; API client +luckCycles/+glossary/+deriveData |
| `changed_files` | `frontend/index.html`, `frontend/src/styles.css`, `frontend/src/ui/dom.js`, `frontend/src/state.js`, `frontend/src/ui/render.js`, `frontend/src/api/client.js`, `frontend/src/main.js` |
| `capability_status_changes` | none |
| `validation_result` | `cargo test` 81 passed; `npm run check` 11 passed |

## LOOP-084

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M23: 天文引擎 target→supported 晋级（WP1 ADR + WP2 集成决策 + WP3 能力晋级 + WP4 回归） |
| `completed_scope` | **WP1**: 撰写 ADR 0021 (Option C: 保持并行，运行时替换另立 ADR)。**WP2**: 集成决策在 ADR 中明确——Android 日期层保持运行时默认，天文引擎作为独立 supported 计算能力。**WP3**: `capabilities.rs` 新增第 16 条条目 (astronomy-engine=supported) + Rust 测试；capability ledger §1 新增条目；v1-closeout 更新；README 更新 (10 supported/6 restricted + 全量 API 路由)；module-tree 补全缺失路由 + 更新计数 (16→17)。**WP4**: `cargo test` 82 passed (含新增 capability test)；`npm run check` 10 passed |
| `changed_files` | `docs/decisions/0021-m23-astronomy-engine-promotion.md` (new), `backend/src/api/capabilities.rs`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `docs/release/v1-closeout.md`, `README.md`, `markdown/00-matrix-governance/module-tree.md` |
| `capability_status_changes` | `astronomy-engine`: target → supported |
| `validation_result` | `cargo test --lib` 82 passed 0 failed; `npm run check` 10 passed 0 failed |
| `governance_updates` | ADR 0021, capability ledger §1, v1-closeout matrix, README capability counts + API list, module-tree route table + domain count |
| `risk_updates` | R-P1-011 (astronomy preflight mistaken for engine) 随 M23 晋级自动关闭 |
| `unresolved_decision_gates` | none |
| `next_cursor` | `LOOP-085`, `single_loop`, M24 chart-report 内容生成实现 |

## LOOP-085

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M24: 排盘口语化报告（WP1 后端 + WP2 文字块 + WP3 拼接规则 + WP4 测试 + WP5 前端按钮渲染） |
| `completed_scope` | **WP1**: 新增 `backend/src/api/report.rs`，实现 `GET /api/charts/report`。**WP2**: 9 个口语化文字块——命盘概览、日主介绍、五行分布、十神关系、地支藏干、日主强弱、格局初判、用神参考、大运走势。全部硬编码模板，禁止确定性断言。**WP3**: 块间以换行拼接，报告首部固定免责声明。**WP4**: 4 项 Rust 测试（生成无错误、含免责声明、禁用词审计通过、未知时辰处理）+ `npm run check` 10 passed。**WP5**: 前端新增「查看命盘报告」按钮（排盘按钮下方）+ `renderReport` 渲染函数 + 报告面板 DOM |
| `changed_files` | `backend/src/api/report.rs` (new), `backend/src/api/mod.rs`, `frontend/index.html`, `frontend/src/ui/dom.js`, `frontend/src/api/client.js`, `frontend/src/ui/render.js`, `frontend/src/main.js`, `markdown/00-matrix-governance/module-tree.md`, `markdown/10-overview/overview-full-feature-tree.md`, `markdown/20-roadmap/93-capability-promotion-ledger.md`, `README.md` |
| `capability_status_changes` | `chart-report`: new → restricted |
| `validation_result` | `cargo test --lib` 86 passed 0 failed; `npm run check` 10 passed 0 failed |
| `governance_updates` | module-tree API count 15→16 + report route; engineering tree +report.rs; capability ledger §1 new entry + §5 marked closed; README M0-M24 closed + report API route |
| `risk_updates` | none (report hard-coded, no AI/LLM, audit passed) |
| `unresolved_decision_gates` | none |
| `next_cursor` | `LOOP-086`, `design_only`. M0-M24 全部完成。边界锁定。 |

## LOOP-086

| Field | Value |
| --- | --- |
| `mode` | `single_loop` |
| `scope` | M25: GPT Pro 前端视觉升级（WP1 HTML + WP2 CSS + WP3 render.js + WP4 dom.js + WP5 测试） |
| `completed_scope` | 采用 GPT Pro 设计系统完整替换前端视觉层。**WP1**: 3 栏布局（360px 侧栏 + 弹性工作区 + 335px 大运时间轨），品牌徽标、命主档案表单、操作按钮组。**WP2**: 墨绿底配金/翠/朱砂/水色，元素色编码卡片，背景光晕和点阵纹理，伪元素装饰边框。**WP3**: pillarCard 改为色编码卡片（木/火/土/金/水色调），elementBars 条形图，godChips 芯片，hiddenStemTable 矩阵表，insightCard 洞察卡，luckTimeline 时间轴节点线。**WP4**: dom.js 映射全部新选择器。**WP5**: 测试全部适配通过（10/10）。保留全部 required DOM ID 和 aria-labelledby 契约。 |
| `changed_files` | `frontend/index.html`, `frontend/src/styles.css`, `frontend/src/ui/render.js`, `frontend/src/ui/dom.js`, `frontend/src/main.js` (minor: gender button sync) |
| `capability_status_changes` | none (纯视觉升级) |
| `validation_result` | `cargo test --lib` 86 passed; `npm run check` 10 passed 0 failed |
| `governance_updates` | roadmap index M25 entry + dependency graph |
| `risk_updates` | none |
| `unresolved_decision_gates` | none |
| `next_cursor` | `LOOP-087`, `design_only`. M0-M25 全部完成。边界锁定。 |
