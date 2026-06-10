# 能力晋级台账

> 本台账定义能力从 planned/target/restricted 晋级到 supported 的最低证据。它是防止治理脱钩的核心文件。

## 1. 当前 supported 能力

| Capability | Route/Surface | Evidence | Lock |
| --- | --- | --- | --- |
| `health` | `GET /api/health` | Rust route + test | 不得删除健康检查 |
| `lunar-data-meta` | `GET /api/lunar-data/meta` | raw data reader + parse test | 不得改写 raw data |
| `calendar-date-query` | `GET /api/calendar/query?date=YYYY-MM-DD` | Android date-layer port + three-pillar edge cases | 不得弱化 Android 黄金样例 |
| `calendar-date-query-v1-meta` | `GET /api/calendar/query?date=YYYY-MM-DD` + `/api/capabilities` | response metadata + capability declaration + Rust tests | 不得扩大为完整排盘、时柱、时区历史、真太阳时或星历支持 |
| `frontend-date-layer-probe` | Frontend Date Layer panel | JS API client + state/render path + frontend tests | 不得冒充完整命盘工作台 |
| `chart-basis-preview` | `GET /api/charts/basis/preview` | M2 restricted contract + Rust tests + ADR 0009 | 不得冒充完整四柱排盘、时柱、农历输入或真太阳时支持 |
| `chart-create` | `GET /api/charts` | M3 chart engine core + exact/unknown hour tests + ADR 0011 | 不得冒充 IANA 时区历史、真太阳时、农历输入、分析、大运或持久化支持 |
| `analysis-snapshot` | `GET /api/analysis/snapshot` | M4 structured metrics/cards + forbidden-output audit + ADR 0012 | 不得返回生成式扩写、高风险确定性断言、大运、存储或分享支持 |
| `case-management` | `GET /api/cases` | M5 local volatile store + immutable snapshot refs + API/domain tests + ADR 0013 | restricted only; no account, database persistence, cloud sync, cross-device sync, public sharing, share tokens, luck cycles, or generated analysis |
| `settings` | `GET /api/settings` | M5 local volatile preferences + API/domain tests + ADR 0013 | restricted only; no account-level settings, cloud sync, or cross-device preference persistence |
| `share-preview` | `GET /api/share/preview` | M6 local volatile share store + hash-only token + redacted public DTO tests + ADR 0014 | restricted only; no durable public links, accounts, database persistence, cloud sync, cross-device sync, public directories, comments, analytics, luck cycles, or generated analysis |
| `frontend-chart-workspace` | Frontend workspace | M7 app shell + chart/analysis/case/share/calendar/data/capability panels + frontend tests + browser checks | restricted; consumes only supported/restricted backend APIs and must not claim luck cycles, durable sharing, cloud sync, account storage, true solar time, timezone history, glossary, range expansion, or astronomy replacement |
| `frontend-share-preview` | Frontend share preview panel | M7 redacted share preview panel + M6 share API + browser privacy check | restricted; redacted/read-only only, no durable public link claim |
| `release-candidate` | `docs/release/v1-release-candidate.md` + `tools/check-release-candidate.ps1` | M8 release freeze, full project gate, M1-M8 closeouts, README/module tree/engineering tree sync | governance/release capability only; no new backend business API or expanded `/api/capabilities` claim |
| `astronomy-engine` | `data/generated/astronomy/out/*` + `backend/src/astronomy/*` | M11 engine implementation (ADR 0019): solar/terms/moon/calendar/compare, 17 tests, 4800+2474+2474 generated artifacts, 1598 Android comparison 0 diff; M23 promotion (ADR 0021) | supported as verified computation capability; Android date layer remains runtime default; runtime replacement requires separate ADR with true solar time, IANA timezone history, and range expansion prerequisites |
| `chart-report` | `GET /api/charts/report` | M24: backend colloquial Chinese report with 9 text blocks (chart-overview, day-master-intro, element-distribution, ten-god-relations, hidden-stems, day-master-strength, pattern-classification, useful-god-hints, luck-cycles), forbidden-output audit, pure hard-coded templates | restricted; no AI/LLM generation; report text must not contain deterministic life claims; disclaimer must always appear first |

## 2. 目标能力晋级条件

| Capability | Current | Earliest Milestone | Required Evidence for `supported` |
| --- | --- | --- | --- |
| `calendar-date-query-v1-meta` | supported | M1 | 返回规则/算法版本元数据；非法日期错误；Android edge cases；模块树更新 |
| `chart-basis-preview` | restricted | M2 | `BirthProfile`/`ChartRequest`/`ChartBasis` DTO；规则档回显；无完整四柱承诺 |
| `chart-create` | supported | M3 | 年/月/日/时四柱；未知时辰策略；时区/边界元数据；黄金样例 |
| `chart-detail` | planned | M3 | 可复现命盘快照；错误 envelope；API contract test |
| `analysis-snapshot` | supported | M4 | 五行/十神/藏干/关系结构化指标；安全文案测试 |
| `luck-cycles` | planned | M4/M5 | `luck_ruleset_id`；顺逆和起运规则；边界测试 |
| `case-management` | restricted | M5 | local volatile storage; immutable chart/analysis snapshot refs; create/detail/list/update/archive/delete tests |
| `settings` | restricted | M5 | local volatile preference DTO; default validation; API update/get tests |
| `share-preview` | restricted | M6 | local volatile hash-only token; redacted public DTO; expiration/revocation/noindex tests |
| `glossary` | planned | M7 | 术语数据源；搜索/详情 API；前端术语入口 |
| `frontend-chart-workspace` | restricted | M7 | app shell; chart input; chart/analysis panels; local cases; redacted share; calendar; responsive/browser checks |
| `frontend-share-preview` | restricted | M7 | consumes restricted M6 share-preview DTO; no private state display |
| `release-candidate` | supported | M8 | 全门禁、E2E、可访问性、closeout、风险清零 |
| `chart-detail` | supported | M12 | `backend/src/api/chart_detail.rs`, app-layer test at 200 OK |
| `glossary` | supported | M14 | `backend/src/api/glossary_data.rs`, 42 structured terminology entries |
| `case-export` | restricted | M14 | `GET /api/cases/export`, JSON export with optional notes |
| `data-derivation` | restricted | M15 | `GET /api/data/derive` stub, aggregate derivation after case population |
| `luck-cycles` | supported | M13 | `backend/src/domain/luck.rs` (5 tests), `backend/src/api/luck.rs`, ADR 0020 closes DG-005 |
| `astronomy-engine` | target | M10 | 生成引擎、hash、1901-2100 黄金表、Android/星历差异 ADR、replay tests、runtime integration |
| `chart-report` | — | M24 | 新增 restricted 能力：`GET /api/charts/report` 后端口语化报告组装、文字块模板、禁用词审计、前端单按钮渲染 |

## 2.1 Recent Evidence Notes

| Capability | Loop | Evidence | Remaining |
| --- | --- | --- | --- |
| `calendar-date-query-v1-meta` | `LOOP-003` | `/api/calendar/query` response includes date-layer source, algorithm version, ruleset id, support range, rule notes, boundary policy, and limitations; Rust unit test protects the payload contract | closed by `LOOP-004` |
| `calendar-date-query-v1-meta` | `LOOP-004` | `/api/capabilities` declares the capability as supported and Rust unit test protects the catalog entry | none for M1-WP4; DG-002 still blocks full M1 closeout |
| `calendar-date-query` | `LOOP-005` | Rust regression suite embeds all 49 Android edge cases for leap days, leap months, year boundaries, pre-CNY/CNY windows, extreme CNY windows, three pillars, and selected solar terms | none for M1-WP2; DG-002 still blocks full M1 closeout |
| `calendar-date-query` | `LOOP-006` | API tests cover missing/invalid date, out-of-range dates, valid boundaries, and missing lunar data source; out-of-range now has explicit `out_of_range` code | none for M1-WP3; DG-002 still blocks full M1 closeout |
| `frontend-date-layer-probe` | `LOOP-007` | Frontend API client, state, DOM mapping, render path, panel markup, CSS, and API client tests call the supported date-query route | remains date-layer only; full chart UI is still planned for M7 |
| `calendar-date-query` | `LOOP-009` | ADR 0008 closes V1 official validated date-layer range as 1901-2100; M1 closeout readiness file confirms WP1-WP5 evidence | M1 can proceed to milestone-loop closeout if full gate stays green |
| M1 Date Layer Hardening | `LOOP-010` | `12-milestone-01-closeout.md` closes M1 through milestone_loop with full gate green | M2 may start with chart-basis/ruleset scope only; full chart remains planned |
| `chart-basis-preview` | `LOOP-011` | Restricted M2 preview route returns `ft-v1-default` metadata, birth-profile/chart-request basis, supported/unsupported outputs, and rejects lunar input, true solar time, and invalid exact time | Full chart, hour pillar, IANA timezone history, and persisted chart remain planned |
| `chart-create` | `LOOP-013` | `GET /api/charts` returns supported M3 chart core with year/month/day pillars, exact-time hour pillar, unknown-hour null plus candidates, warnings, ambiguity flags, and unsupported outputs | `chart-detail`, analysis, luck cycles, storage, IANA timezone history, true solar time, lunar input, and astronomy replacement remain planned |
| `analysis-snapshot` | `LOOP-014` | `GET /api/analysis/snapshot` returns structured element/ten-god/hidden-stem metrics, fixed cards, sensitivity flags, disclaimer id, and forbidden-output audit | Luck cycles, generated prose, storage, sharing, and high-risk certainty claims remain unsupported |
| `case-management` | `LOOP-015` | `GET /api/cases` supports restricted local volatile create/detail/list/update_metadata/archive/delete actions; list omits private notes; chart and analysis snapshot refs preserve algorithm versions | Database persistence, accounts, cloud sync, cross-device sync, public sharing, share tokens, luck cycles, and generated analysis remain unsupported |
| `settings` | `LOOP-015` | `GET /api/settings` supports restricted local volatile preference get/update with validation for calendar, privacy default, language, and theme | Account-level settings, cloud sync, and cross-device preference persistence remain unsupported |
| `share-preview` | `LOOP-016` | `GET /api/share/preview` supports restricted local volatile create/public/revoke actions; public DTOs omit private notes, raw titles, tags, private case ids, exact birth-time/location fields, and snapshot ids; revoked/invalid tokens share the same unavailable response | Durable public links, accounts, database persistence, cloud sync, cross-device sync, public directories, comments, analytics, luck cycles, and generated analysis remain unsupported |
| `frontend-chart-workspace` | `LOOP-017` | Frontend workspace runs chart, analysis, local case save/list, redacted share preview, date-layer query, data metadata, and capability boundary rendering; desktop/mobile browser checks passed | Glossary, luck cycles, durable public sharing, account storage, cloud sync, true solar time, timezone history, range expansion, and astronomy replacement remain unavailable |
| `frontend-share-preview` | `LOOP-017` | Share panel creates a restricted redacted share preview and browser verification showed no private note or snapshot id exposure | Durable links and public directory remain unsupported |
| `release-candidate` | `LOOP-018` | Release document, release checker, M8 preflight/closeout, README/module tree/engineering tree sync, and full project gate freeze V1 statuses | M9 astronomy upgrade remains target scope; luck cycles remain planned |
| `astronomy-engine` | `LOOP-019` | ADR 0015 closes DG-008 for parallel-first preflight; manifest schema, comparison report template, and preflight checker added | Still target; no generated table, runtime engine, wider range, true solar time, or timezone-history support |
| `astronomy-engine` | `LOOP-020` | ADR 0016 and `source-policy.json` select GB/T 33661, JPL Horizons, IAU SOFA, and NAIF CSPICE/SPICE as the source stack for generated evidence planning | Still target; source policy is not a generated table or runtime engine |
| `astronomy-engine` | `LOOP-021` | Optional source probe found required JPL Horizons docs/API, IAU SOFA, and NAIF reachable; GB/T page is a nonblocking warning | Still target; source availability is not generated astronomy data |
| `astronomy-engine` | `LOOP-022` | `astronomy-engine-v0-draft.json` creates a not-accepted manifest planning instance and checker enforces missing generation command/hash/comparison/golden/runtime evidence | Still target; draft manifest is explicitly not accepted |
| `astronomy-engine` | `LOOP-023` | `generation-plan.json` defines planned artifact shapes and future command form while checker enforces `draft_not_runnable` and `not_generated` artifacts | Still target; generation plan is not a runnable generator or accepted data |
| `astronomy-engine` | `LOOP-024` | `tools/generate-astronomy-tables.ps1 -DryRun` reports planned artifacts while checker enforces no writes, no acceptance change, and no existing planned artifacts | Still target; dry-run generator is not generated data |
| `astronomy-engine` | `LOOP-025` | Comparison schema, golden-case plan, and replay-policy draft define acceptance prerequisites while checker keeps them planning-only | Still target; no comparison rows, golden rows, replay tests, or accepted generated data |
| `astronomy-engine` | `LOOP-026` | `tools/compare-astronomy-dry-run.ps1` emits a schema-shaped comparison object with zero rows and no accepted evidence; checker enforces this | Still target; comparison dry-run is not generated comparison evidence |
| `astronomy-engine` | `LOOP-027` | `tools/golden-cases-dry-run.ps1` reports six required golden categories with zero generated rows and no accepted evidence; checker enforces this | Still target; golden dry-run is not generated golden evidence |
| `astronomy-engine` | `LOOP-028` | `tools/replay-policy-dry-run.ps1` reports five required replay/replacement controls with zero replay tests and no accepted evidence; checker enforces replacement remains disallowed | Still target; replay dry-run is not replay-test evidence and does not permit Android baseline replacement |
| `astronomy-engine` | `LOOP-029` | `precloseout-audit.json` and `36-milestone-09-pre-closeout-audit.md` prove full M9 closeout is blocked while preflight closeout review is ready | Still target; audit evidence blocks promotion until generated artifacts, hashes, completed comparison, golden rows, replay tests, and runtime integration exist |
| `astronomy-engine` | `LOOP-030` | ADR 0017 and `implementation-plan.json` select continued M9 generated-data planning and make generator contract planning the next stage | Still target; implementation plan is not generated data and forbids artifact acceptance or runtime replacement |
| `astronomy-engine` | `LOOP-031` | `generator-contract.json` defines required inputs, planned outputs, canonical JSON encoding, `sha256` hash policy, manifest update rules, and forbidden contract-stage actions | Still target; generator contract is not generated data and all outputs remain `not_generated` |
| `astronomy-engine` | `LOOP-032` | `source-adapter-contract.json` defines GB/T, Horizons, SOFA, and SPICE adapter boundaries without runtime dependency, output claims, or full-gate external calls | Still target; source adapter contract is not source integration or generated data |
| `astronomy-engine` | `LOOP-033` | `artifact-writer-plan.json` and `tools/artifact-writer-dry-run.ps1` preview output paths and `sha256` hash status without writing files or computing hashes | Still target; artifact writer dry-run is not generated artifact evidence |
| `astronomy-engine` | `LOOP-034` | `comparison-runner-plan.json` and `tools/compare-astronomy-dry-run.ps1` bind Android baseline metadata to the future comparison artifact with zero rows and zero differences | Still target; comparison runner dry-run is not completed comparison evidence |
| `astronomy-engine` | `LOOP-035` | `golden-row-readiness-plan.json` and `tools/golden-cases-dry-run.ps1` keep six golden categories blocked/not-generated with zero generated rows | Still target; golden readiness is not generated golden-case evidence |
| `astronomy-engine` | `LOOP-036` | `replay-test-readiness-plan.json` and `tools/replay-policy-dry-run.ps1` keep replay controls unexecuted with zero replay tests and replacement disallowed | Still target; replay readiness is not replay-test evidence |
| `astronomy-engine` | `LOOP-037` | `preflight-closeout-decision.json` and `44-milestone-09-preflight-closeout.md` close M9 only as preflight and route real generated-data implementation to M10 | Still target; preflight closeout is not generated-data acceptance, runtime integration, or replacement evidence |
| `astronomy-engine` | `LOOP-038` | `generator-implementation-entry.json` and `tools/generate-astronomy-tables.ps1 -PrepareImplementation` add a guarded non-dry-run implementation entrypoint with generation blocked, no writes, hashes 0, acceptance unchanged, and runtime unchanged | Still target; guarded entry is not source materialization, generated artifact, hash, comparison, golden-row, replay, or runtime integration evidence |
| `astronomy-engine` | `LOOP-039` | `source-snapshot-manifest.schema.json`, `source-snapshot-manifest-plan.json`, and `tools/source-snapshot-manifest-dry-run.ps1` define selected-source manifest boundaries while keeping the actual manifest absent, source snapshots 0, generated artifacts 0, hashes 0, and no writes | Still target; source snapshot boundary is not source materialization, generated artifact, hash, runtime integration, or replacement evidence |
| `astronomy-engine` | `LOOP-040` | `source-snapshots/source-snapshot-manifest.json` materializes selected-source provenance metadata with all sources `not_materialized`, runtime dependency false, output claim false, source payloads 0, generated artifacts 0, hashes 0, acceptance unchanged, and runtime unchanged | Still target; metadata-only source manifest is not source payload materialization, generated artifact, hash, comparison, replay, runtime integration, or replacement evidence |
| `astronomy-engine` | `LOOP-041` | `source-payload-materialization-policy.json` and `tools/source-payload-materialization-dry-run.ps1` define future per-source payload paths while keeping payload directory absent, payload files 0, source payloads 0, payload hashes 0, generated artifacts 0, acceptance unchanged, and runtime unchanged | Still target; payload policy is not source payload materialization, source hash, generated artifact, runtime integration, or replacement evidence |
| `astronomy-engine` | `LOOP-042` | `source-payload-schemas/*.schema.json` define per-source payload shapes for GB/T, JPL Horizons, IAU SOFA, and NAIF CSPICE while checker/dry-run keep schema files 4, payload directory absent, payload files 0, source payloads 0, payload hashes 0, generated artifacts 0, acceptance unchanged, and runtime unchanged | Still target; schema-only payload definitions are not source payload materialization, source hash, generated artifact, runtime integration, or replacement evidence |
| `astronomy-engine` | `LOOP-043` | `source-capture-procedure.json` and `tools/source-capture-procedure-dry-run.ps1` define per-source capture steps and first-payload preconditions while keeping capture not_started, payload directory absent, payload files 0, source payloads 0, payload hashes 0, external calls false, generated artifacts 0, acceptance unchanged, and runtime unchanged | Still target; procedure-only capture planning is not source payload materialization, source hash, generated artifact, runtime integration, or replacement evidence |
| `astronomy-engine` | `LOOP-044` | `source-payload-materialization-decision.json` and `tools/source-payload-materialization-decision-dry-run.ps1` select `naif-cspice` as the first single-source payload candidate while keeping payload directory absent, selected payload absent, source payloads 0, payload hashes 0, external calls false, generated artifacts 0, acceptance unchanged, and runtime unchanged | Still target; decision-only source selection is not selected payload materialization, source hash, generated artifact, runtime integration, or replacement evidence |
| `astronomy-engine` | `LOOP-045` | `selected-source-payload-materialization-preflight.json` and `tools/selected-source-payload-materialization-preflight-dry-run.ps1` define selected-source-only next-loop scope for `naif-cspice` while keeping payload directory absent, selected payload absent, source payloads 0, payload hashes 0, external calls false, generated artifacts 0, acceptance unchanged, and runtime unchanged | Still target; selected-source preflight is not selected payload materialization, source hash, generated artifact, runtime integration, or replacement evidence |
| `astronomy-engine` | `LOOP-046` | `selected-source-payload-materialization.json`, `source-snapshots/payloads/naif-cspice-kernel-boundary.json`, and selected-source dry-runs record exactly one `naif-cspice` source-boundary payload and sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`; unselected payload files, generated artifacts, generated artifact hashes, acceptance changes, runtime changes, and external full-gate calls remain absent | Still target; selected source payload materialization is not SPICE kernel/toolkit integration, generated astronomy data, runtime integration, Android replacement, or supported `astronomy-engine` evidence |
| `astronomy-engine` | `LOOP-047` | `remaining-source-payload-strategy.json` and `tools/remaining-source-payload-strategy-dry-run.ps1` choose `iau-sofa-ansi-c` as the next preflight-only source candidate while keeping existing payload files 1, new payload writes 0, new source hashes 0, external calls false, generated artifacts 0, acceptance unchanged, and runtime unchanged | Still target; remaining source strategy is not SOFA payload materialization, generated astronomy data, runtime integration, Android replacement, or supported `astronomy-engine` evidence |
| `astronomy-engine` | `LOOP-048` | `selected-iau-sofa-payload-materialization-preflight.json` and `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1` prepare selected-source-only `iau-sofa-ansi-c` local routine/version boundary materialization while keeping the selected payload absent, existing payload files 1, new payload writes 0, new source hashes 0, external calls false, generated artifacts 0, acceptance unchanged, and runtime unchanged | Still target; selected IAU SOFA preflight is not SOFA payload materialization, routine integration, generated astronomy data, runtime integration, Android replacement, or supported `astronomy-engine` evidence |
| `astronomy-engine` | `LOOP-049` | `selected-iau-sofa-payload-materialization.json`, `source-snapshots/payloads/iau-sofa-routine-version.json`, and selected-source dry-runs record the selected IAU SOFA routine/version boundary payload and sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f`; JPL/GB/T payload files, generated artifacts, generated artifact hashes, acceptance changes, runtime changes, and external full-gate calls remain absent | Still target; selected IAU SOFA payload materialization is not SOFA source vendoring, routine compilation/linking, runtime integration, generated astronomy data, Android replacement, or supported `astronomy-engine` evidence |
| `astronomy-engine` | `LOOP-050` | `post-iau-remaining-source-payload-strategy.json` and `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1` select JPL Horizons as the next selected-source-only preflight candidate while preserving two existing source-boundary payloads and keeping JPL/GB/T payload files absent, new source payload writes 0, new source hashes 0, external calls false, generated artifacts 0, acceptance unchanged, and runtime unchanged | Still target; post-IAU strategy is not JPL payload materialization, GB/T payload materialization, generated astronomy data, runtime integration, Android replacement, or supported `astronomy-engine` evidence |
| `astronomy-engine` | `LOOP-051` | `selected-jpl-horizons-payload-materialization-preflight.json` and `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1` scope JPL Horizons as the next selected-source-only payload materialization candidate while keeping the JPL payload absent, GB/T payload absent, query execution allowed in full gate false, new source payload writes 0, new source hashes 0, external calls false, generated artifacts 0, acceptance unchanged, and runtime unchanged | Still target; selected JPL Horizons preflight is not JPL query snapshot materialization, online query execution, generated astronomy data, runtime integration, Android replacement, or supported `astronomy-engine` evidence |
| `astronomy-engine` | `LOOP-052` | `selected-jpl-horizons-payload-materialization.json`, `source-snapshots/payloads/jpl-horizons-validation-samples.json`, and selected-source dry-runs record the selected JPL Horizons validation-query snapshot boundary payload and sha256 `acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9`; GB/T payload files, online full-gate JPL queries, response-body claims, generated artifacts, generated artifact hashes, acceptance changes, runtime changes, Android replacement, and external full-gate calls remain absent | Still target; selected JPL Horizons payload materialization is not JPL response-body capture, online query execution, generated astronomy data, runtime integration, Android replacement, or supported `astronomy-engine` evidence |
| `astronomy-engine` | `LOOP-053` | `selected-gb-t-payload-materialization-preflight.json` and `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1` scope GB/T 33661 as the next selected-source-only rule-reference payload candidate while keeping the GB/T payload absent, source-reference capture false for this loop, payload materialization false for this loop, new source payload writes 0, new source hashes 0, external calls false, generated artifacts 0, acceptance unchanged, runtime unchanged, and Android baseline unchanged | Still target; selected GB/T preflight is not GB/T rule payload materialization, generated astronomy data, runtime integration, Android replacement, or supported `astronomy-engine` evidence |
| `astronomy-engine` | `LOOP-054` | `selected-gb-t-payload-materialization.json`, `source-snapshots/payloads/gb-t-33661-2017-rule-reference.json`, and closed selected GB/T dry-run record the selected GB/T rule-reference boundary payload and sha256 `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31`; standard text copy, implemented calendar-rule claims, generated artifacts, generated artifact hashes, acceptance changes, runtime changes, Android replacement, and external full-gate calls remain absent | Still target; selected GB/T payload materialization is not generated astronomy data, rule implementation, runtime integration, Android replacement, or supported `astronomy-engine` evidence |

## 3. 晋级流程

1. 在对应里程碑中列出能力。
2. 实现 Rust/API/test 证据。
3. 更新 `/api/capabilities` 或等价能力声明。
4. 更新本台账 Evidence。
5. 更新前端文案或 UI 状态。
6. 运行完整门禁。
7. 在 closeout 中记录晋级结果。

## 4. 降级流程

如果 supported 能力证据不再成立，必须：

- 立即把 capability 状态降为 restricted/planned。
- 记录触发原因和影响用户。
- 保留失败样例。
- 更新 README、模块树、前端文案。
- 关闭前不得继续宣称 supported。

## 5. V1 最终能力边界锁

M23/M24 是 V1 最终能力切面。两个里程碑关闭后，能力矩阵锁定为：

| 状态 | 数量 | 能力 |
| --- | --- | --- |
| supported | 10 | health, lunar-data-meta, calendar-date-query, calendar-date-query-v1-meta, chart-create, chart-detail, analysis-snapshot, luck-cycles, glossary, astronomy-engine |
| restricted | 7 | chart-basis-preview, case-management, share-preview, settings, case-export, data-derivation, chart-report |
| target | 0 | — |
| planned | 0 | — |

**M23 closed. M24 closed (code complete + governance sync). 边界已锁定。**

边界锁定后不再受理功能性新增需求。此后只允许：
- 治理同步（台账、模块树、工程树、README 修正）
- 缺陷修复
- 性能优化
- 已有 restricted 能力的晋级（按 §3 晋级流程）
