# 命轨模块树

> 逻辑白箱所有权图，不是文件树。文件树见 `markdown/10-overview/overview-full-feature-tree.md`。研究报告补入后，必须把新增 public surface、输入、输出和回归保护登记到对应节点。

## 1. Root Domains

| Domain | Owns |
| --- | --- |
| `system` | workspace、运行入口、构建命令、交付说明 |
| `backend` | Rust API、HTTP、配置、领域模型、农历数据读取、未来排盘能力 |
| `frontend` | JS API client、状态、渲染、交互、静态服务 |
| `data` | 原始数据、衍生产物、数据生命周期 |
| `governance` | 全量树、模块树、策略矩阵、流程矩阵、ADR、研究纳入台账、中文译文、开发里程碑 |
| `tools` | 项目检查、脚手架检查、清单生成 |

## 2. Module Index

| Module ID | Real Files | Responsibility |
| --- | --- | --- |
| `system.workspace` | `.gitignore`, `Cargo.toml`, `Cargo.lock`, `README.md` | 工作区、入口说明、忽略规则 |
| `backend.entrypoint` | `backend/src/main.rs`, `backend/src/lib.rs`, `backend/src/config.rs` | 后端启动、配置、模块导出 |
| `backend.app` | `backend/src/app.rs`, `backend/src/server.rs`, `backend/src/error.rs` | 请求调度、TCP 服务、统一错误 |
| `backend.http` | `backend/src/http/*` | HTTP 请求解析、响应、状态码、JSON 字符串工具 |
| `backend.api` | `backend/src/api/*` (16 files) | M1-M24 全部 API 路由：health, capabilities, lunar, calendar, chart_basis, charts, chart_detail, analysis, luck, cases, derive, share, settings, glossary_data, report |
| `backend.calendar.civil` | `backend/src/calendar/civil.rs` | 纯公历日期校验、闰年、日期差、年内序号 |
| `backend.calendar.ganzhi` | `backend/src/calendar/ganzhi.rs` | Android 万年历同源的年月日干支计算 |
| `backend.calendar.lunar_data` | `backend/src/calendar/lunar_data.rs`, `data/raw/lunar_data.yaml` | 农历 raw data 读取、元信息解析、日期层查询 |
| `backend.domain` | `backend/src/domain/*` (9 files) | 八字、案例、大运、深层分析、设置、共享、术语 |
| `backend.astronomy` | `backend/src/astronomy/*` (7 files) | M11 天文引擎：时标、太阳、节气、月球、农历推导、对照 |
| `frontend.shell` | `frontend/index.html`, `frontend/server.mjs`, `frontend/package.json` | 前端页面壳、本地静态服务、脚本入口 |
| `frontend.api` | `frontend/src/api/client.js`, `frontend/src/config.js`, `frontend/tests/api-client.test.mjs` | Backend API client for date-layer, chart, analysis, local cases, restricted share preview, and capability tests |
| `frontend.state` | `frontend/src/state.js`, `frontend/src/main.js` | 应用状态与启动编排 |
| `frontend.ui` | `frontend/src/ui/*`, `frontend/src/styles.css` | M7 workspace DOM mapping, rendering, restricted/supported labels, and responsive layout |
| `frontend.utils` | `frontend/src/utils/format.js`, `frontend/tests/format.test.mjs` | 纯格式化函数和测试 |
| `data.lunar.raw` | `data/README.md`, `data/raw/lunar_data.yaml` | 农历基础数据真源 |
| `governance.matrix` | `markdown/**`, `docs/decisions/*` | 治理文档和决策记录 |
| `governance.research` | `markdown/reserch/*`, `markdown/reserch/zh-CN/*`, `markdown/reserch/00-research-intake.md`, `docs/decisions/0003-v1-research-governance-baseline.md`, `docs/decisions/0004-v1-calculation-ruleset-target.md`, `docs/decisions/0005-privacy-safe-interpretation-target.md` | 研究报告原文、中文译文、采纳状态、目标规则和隐私解释政策 |
| `governance.roadmap` | `markdown/20-roadmap/*`, `docs/decisions/0006-roadmap-and-governance-lock.md`, `docs/decisions/0007-recursive-development-protocol.md` | 从 M0 到 M22 的开发里程碑、决策门、风险、防回退、能力晋级台账和递归游标 |
| `governance.release` | `docs/release/v1-release-candidate.md`, `markdown/20-roadmap/25-milestone-08-preflight.md`, `markdown/20-roadmap/26-milestone-08-closeout.md` | V1 release candidate、能力冻结、降级规则和 M8 收口证据 |
| `governance.astronomy-preflight` | `docs/decisions/0015-m9-astronomy-parallel-strategy.md`, `docs/decisions/0016-m9-astronomy-source-stack.md`, `data/generated/astronomy/*`, `tools/check-astronomy-preflight.ps1`, `markdown/20-roadmap/27-milestone-09-preflight.md` | M9 星历并行策略、源栈决策、manifest schema、对照报告模板和预检门禁 |
| `tools.governance` | `tools/*.ps1` | 项目检查、脚手架检查、目录盘点、release candidate 检查、astronomy preflight 检查 |

## 3. Backend Module Contracts

### Module ID: `backend.entrypoint`

**Inputs**:
| Input | Source | Type | Constraints |
| --- | --- | --- | --- |
| `FT_BACKEND_ADDR` | environment | string | 可选，默认 `127.0.0.1:8787` |
| `FT_LUNAR_DATA_PATH` | environment | path | 可选，默认 `data/raw/lunar_data.yaml` |

**Outputs**:
| Output | Destination | Type | Constraints |
| --- | --- | --- | --- |
| `AppConfig` | `backend.app` | struct | 不得在下游重新猜测路径 |

**Regression protection**: `cargo test`。

### Module ID: `backend.app`

**Inputs**:
| Input | Source | Type | Constraints |
| --- | --- | --- | --- |
| raw HTTP bytes | `backend.server` | bytes | 必须显式 bad request |
| parsed request | `backend.http` | `Request` | 只交给 `backend.api` 路由 |

**Outputs**:
| Output | Destination | Type | Constraints |
| --- | --- | --- | --- |
| response | client | HTTP | 错误必须统一 JSON |

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `App::handle` | `backend.server`, tests | 不得直接读取 raw data |
| `parse_and_handle` | `backend.server`, tests | 不得吞掉解析错误 |

**Regression protection**: health route test、planned route unsupported test。

### Module ID: `backend.http`

**Responsibility**: 只负责协议形状，不承载业务。

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `Request::parse` | `backend.app` | 不得解释业务参数 |
| `Response::json` / `Response::json_error` | `backend.app`, `backend.api` | 不得拼业务字段 |
| `json::*` | backend modules | 不得绕过错误模型伪造能力 |

**Regression protection**: request parse test。

### Module ID: `backend.api`

**Supported routes**:
| Route | Source | Status |
| --- | --- | --- |
| `GET /api/health` | `api/health.rs` | supported |
| `GET /api/capabilities` | `api/capabilities.rs` | supported |
| `GET /api/lunar-data/meta` | `api/lunar.rs` | supported |
| `GET /api/calendar/query?date=YYYY-MM-DD` | `api/calendar.rs` | supported |
| `GET /api/charts/basis/preview` | `api/chart_basis.rs` | restricted |
| `GET /api/charts` | `api/charts.rs` | supported |
| `GET /api/analysis/snapshot` | `api/analysis.rs` | supported |
| `GET /api/cases` | `api/cases.rs` | restricted |
| `GET /api/settings` | `api/settings.rs` | restricted |
| `GET /api/share/preview` | `api/share.rs` | restricted |
| `GET /api/charts/detail` | `api/chart_detail.rs` | supported |
| `GET /api/luck/cycles` | `api/luck.rs` | supported |
| `GET /api/glossary` | `api/glossary_data.rs` | supported |
| `GET /api/cases/export` | `api/cases.rs` | restricted |
| `GET /api/data/derive` | `api/derive.rs` | restricted |
| `data/generated/astronomy/out/*` | `astronomy/*` | supported (ADR 0021) |
| `GET /api/charts/report` | `api/report.rs` | restricted |

`GET /api/calendar/query` response metadata is limited to the current Android date-layer baseline: source, algorithm version, ruleset id, 1901-2100 range, date-only boundary policy, and limitations. It must not be treated as full chart, hour-pillar, timezone-history, true-solar-time, or astronomy-engine support.

ADR 0008 closes the V1 official validated date-layer range as 1901-2100. Wider validation belongs to M9 astronomy/ephemeris work.

`/api/capabilities` declares `calendar-date-query` and `calendar-date-query-v1-meta` as supported capabilities for the same route. The `v1-meta` declaration is a traceability capability, not a new calculation scope.

`/api/charts/basis/preview` declares the M2 `chart-basis-preview` contract as restricted. It records `ft-v1-default`, birth-profile fields, chart-basis metadata, supported contract outputs, and unsupported calculation outputs. It must not be treated as full chart support.

`/api/charts` declares the M3 `chart-create` core as supported for Gregorian dates inside 1901-2100. It returns year/month/day pillars from the Android date layer, exact-time hour pillar, unknown-hour candidates, metadata, warnings, ambiguity flags, and unsupported-output declarations.

`/api/analysis/snapshot` declares the M4 `analysis-snapshot` surface as supported for structured metrics and fixed cards only. It must not return generated prose or high-risk deterministic claims.

`/api/cases` declares the M5 `case-management` surface as restricted. It uses local in-process volatile storage, immutable chart/analysis snapshot references, archive/delete semantics, and list responses that omit private notes. It must not be treated as database persistence, account storage, cloud sync, cross-device sync, public sharing, share-token support, luck cycles, or generated analysis.

`/api/settings` declares the M5 `settings` surface as restricted. It stores only local volatile preferences and must not be treated as account-level or cross-device settings.
`/api/share/preview` declares the M6 `share-preview` surface as restricted. It uses local volatile hash-only token records, expiration, revocation, noindex/non-editable public DTOs, and redacted immutable snapshot data. It must not expose private notes, raw titles, tags, private case ids, exact birth-time/location fields, snapshot ids, durable storage, account state, cloud sync, cross-device sync, public directories, comments, analytics, luck cycles, or generated analysis.

Date-query error envelope is `{"error": "...", "message": "..."}`: missing/invalid date uses `bad_request`/400, supported-table misses use `out_of_range`/404, and missing lunar data source uses `io_error`/500.

| `GET /api/charts/detail` | `api/chart_detail.rs` | supported |
| `GET /api/luck/cycles` | `api/luck.rs` | supported |
| `GET /api/glossary` | `api/glossary_data.rs` | supported |
| `GET /api/cases/export` | `api/cases.rs` | restricted |
| `GET /api/data/derive` | `api/derive.rs` | restricted |

M12 chart-detail returns immutable snapshot with algo_version, ruleset_id, birth profile, pillars, warnings, and ambiguity flags. M13 luck-cycles (ADR 0020, DG-005 closed) returns 大运 direction, starting-age, and 8 cycles. M14 glossary returns 55 structured terminology entries with term/category search. M14 case-export returns JSON export with optional private notes and analysis report. M15 data-derivation returns real aggregated statistics (day_masters, elements, ten_gods, hours) with >=5 threshold privacy protection. M24 chart-report returns colloquial Chinese chart report with 9 text blocks, disclaimer, and forbidden-output audit; restricted capability, pure hard-coded templates, no AI/LLM.

**Forbidden lateral links**: 不得直接读取前端文件；不得把 planned 能力返回为 supported。

**Regression protection**: `cargo test`, including date-query bad request, out-of-range, boundary success, and missing-data envelope tests.

### Module ID: `backend.calendar.civil`

**Inputs**:
| Input | Source | Type | Constraints |
| --- | --- | --- | --- |
| ISO date | API/user | string | `YYYY-MM-DD` valid Gregorian date |

**Outputs**:
| Output | Destination | Type | Constraints |
| --- | --- | --- | --- |
| `CivilDate` | calendar modules | struct | No timezone dependency |
| day delta / day-of-year | `backend.calendar.ganzhi`, `backend.calendar.lunar_data` | integer | Must match Gregorian calendar |

**Regression protection**: leap day and cross-year date-delta tests.

### Module ID: `backend.calendar.ganzhi`

**Inputs**:
| Input | Source | Type | Constraints |
| --- | --- | --- | --- |
| `CivilDate` | `backend.calendar.civil` | struct | valid Gregorian date |
| `YearEntry` | `backend.calendar.lunar_data` | struct | sourced from raw table |

**Outputs**:
| Output | Destination | Type | Constraints |
| --- | --- | --- | --- |
| day Gan-Zhi | date layer API | string | Android epoch `1900-01-01`, index `10` |
| month Gan-Zhi | date layer API | string | solar-term-data-driven (ADR 0018), year-specific DOY from YAML |

**Regression protection**: Android edge-case manifest for leap days, leap months, year-boundary continuity, CNY windows, and selected solar terms. Month boundary corrected per ADR 0018.

### Module ID: `backend.calendar.lunar_data`

**Inputs**:
| Input | Source | Type | Constraints |
| --- | --- | --- | --- |
| `data/raw/lunar_data.yaml` | `data.lunar.raw` | YAML text | 只读 |

**Outputs**:
| Output | Destination | Type | Constraints |
| --- | --- | --- | --- |
| `LunarDataMeta` | `backend.api` | struct | 仅元信息 |
| `LunarTable` | `backend.api.calendar` | struct | 1901-2100 skip table |
| `LunarDateResult` | `backend.api.calendar` | struct | 农历日期、三柱、生肖、节气 |
| `LunarDataStats` | tests/calendar modules | struct | 解析 raw data 顶层结构 |

**Android source**: `D:\myproject\Perpetual calendar\app\src\main\java\com\perpetualcalendar\app\lunar\LunarCalendar.java` and `GanZhi.java`.

**Regression protection**: lunar stats parse test, table lookup test, 49-case Android edge manifest for lunar month/day, leap flags, three pillars, and selected solar terms.

### Module ID: `backend.domain`

**Current entities**:
| File | Entities | Status |
| --- | --- | --- |
| `bazi.rs` | `RulesetId`, `CalculationMetadata`, `BirthProfile`, `ChartRequest`, `ChartBasis`, `ChartResult`, `ChartDetail`, `BaziChart`, `Pillar` | M3 chart core + M12 immutable snapshot |
| `analysis.rs` | `AnalysisSnapshot`, `WeightedMetric`, `AnalysisCard`, forbidden-output audit | M4 structured analysis |
| `cases.rs` | `CaseRecord`, `CaseRepository`, immutable chart/analysis snapshot refs, `SharePreset` | M5 local volatile case storage |
| `settings.rs` | `UserPreference` | M5 local volatile preferences |
| `share.rs` | `ShareRecord`, `ShareRepository`, `RedactedShareSnapshot` | M6 local volatile share preview |
| `glossary.rs` | `GlossaryEntry` | M14 glossary skeleton |
| `luck.rs` | `LuckCycle`, `compute_luck_cycles` | M13 luck cycles (ADR 0020, DG-005 closed) |
| `deep_analysis.rs` | `StrengthAssessment`, `PatternInfo`, `UsefulGodHint` | M21 deep analysis (三命通会/子平法蒸馏) |

**Rule**: All 17 capabilities are now supported or restricted. No planned or target capabilities remain. `astronomy-engine` promoted from target to supported per M23 (ADR 0021).

## 3.1 Implemented: Astronomy Engine

### Module ID: `backend.astronomy`

| File | Responsibility |
| --- | --- |
| `time.rs` | Gregorian↔Julian Date, ΔT (NASA polynomial), leap seconds, DOY utilities |
| `sun.rs` | Meeus standard solar theory: geocentric longitude (<1'), distance, nutation/aberration |
| `terms.rs` | Solar term crossing finder: 24 terms/year via bisection on apparent longitude |
| `moon.rs` | Simplified lunar theory (Meeus Ch.47, ~60 terms): longitude, new moon finder |
| `calendar.rs` | Lunar calendar derivation: month table from terms + new moons, GB/T leap rules |
| `compare.rs` | M19 Android-vs-astronomy day-pillar comparison: 1598 samples, 0 differences |

**Source**: ADR 0019 (M11 engine architecture).

**Regression protection**: `cargo test astronomy` — 17 tests covering JD roundtrip, solar monotonicity, term count, new moon count, lunar month validity.

## 3.2 Delivered: Generated Data

### Module ID: `data.generated.astronomy`

| Artifact | Entries | Engine | Status |
| --- | --- | --- | --- |
| `out/solar-terms-1901-2100.json` | 4,800 | `backend.astronomy.terms` | computed |
| `out/new-moons-1901-2100.json` | 2,474 | `backend.astronomy.moon` | computed |
| `out/lunar-calendar-1901-2100.json` | 2,474 | `backend.astronomy.calendar` | computed |
| `out/android-comparison-1901-2100.json` | stub | — | placeholder |

Four source-boundary payloads materialized with sha256. Manifest updated with boundary placeholder → computed transition. `astronomy-engine` remains target pending replacement ADR.

## 4. Frontend Module Contracts

### Module ID: `frontend.shell`

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `node server.mjs` | developer | 不得暴露 `frontend/` 之外文件 |
| `npm.cmd run check` | developer/CI | 不得跳过模块语法和测试 |

**Regression protection**: `npm.cmd run check`。

### Module ID: `frontend.api`

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `ApiClient.health` | `frontend.state` | 不得吞掉 HTTP 错误 |
| `ApiClient.lunarDataMeta` | `frontend.state` | 不得直接读取 raw YAML |
| `ApiClient.capabilities` | `frontend.state` | 不得把 planned 改写为 supported |
| `ApiClient.calendarDate` | `frontend.state` | 只调用 supported date-layer API，不得生成完整命盘或时柱 |
| `ApiClient.chartCreate` / `analysisSnapshot` | `frontend.state` | Must call backend APIs; must not implement chart or analysis algorithms locally |
| `ApiClient.createCase` / `listCases` | `frontend.state` | Must keep case-management restricted/local volatile |
| `ApiClient.createShare` | `frontend.state` | Must keep share-preview restricted/redacted and must not expose private state |

**Regression protection**: `node --check src/api/client.js`, `node --test frontend/tests/api-client.test.mjs`。

### Module ID: `frontend.state`

**Responsibility**: 编排 API 调用、保存 API base、触发渲染。

**Forbidden lateral links**: 不得承载命理算法；不得写 raw data。

**Regression protection**: `node --check src/main.js`、`node --check src/state.js`。

### Module ID: `frontend.ui`

**Responsibility**: DOM mapping, health/data/capability rendering, M7 chart workspace, analysis cards, local case list, redacted share preview, date-layer probe, and responsive styles.

**Forbidden lateral links**: 不得调用 `fetch`；不得保存持久状态。
**M7 lock**: UI labels must reflect backend capabilities or the capability ledger; no luck cycles, durable sharing, cloud sync, account storage, true solar time, timezone history, range expansion, or astronomy replacement may be presented as available.

**Regression protection**: `node --check src/ui/dom.js`、`node --check src/ui/render.js`、`frontend/tests/workspace-markup.test.mjs`、frontend check、浏览器截图检查。

### Module ID: `governance.release`

**Responsibility**: 冻结 V1 release candidate 的支持边界、验证命令、回滚/降级规则和 M8 closeout 证据。

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `docs/release/v1-release-candidate.md` | 开发者、发布检查 | 不得宣称业务 API 中没有证据的能力 |
| `tools/check-release-candidate.ps1` | `tools/check-project.ps1` | 不得跳过 M1-M7 closeout、capability status、README 边界或分享脱敏证据 |

**Regression protection**: `powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-release-candidate.ps1 -ProjectRoot .`。

### Module ID: `frontend.utils`

**Responsibility**: 可测试纯函数。

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `formatBytes` | `frontend.ui` | 不得访问 DOM |
| `formatRange` | `frontend.ui` | 不得访问 API |

**Regression protection**: `node --test`。

## 5. Data and Governance Contracts

### Module ID: `data.lunar.raw`

**Rule**: `data/raw/lunar_data.yaml` 是当前农历数据真源；应用运行时不得改写。任何衍生产物必须新增 `data.lunar.derived` 节点。

### Module ID: `data.generated.astronomy`

**Current status**: preflight only; no generated astronomy table is accepted.

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `data/generated/astronomy/manifest.schema.json` | `tools/check-astronomy-preflight.ps1` | 不得缺少 engine/version/source/range/command/hash/report/taxonomy 字段 |
| `data/generated/astronomy/source-policy.json` | `tools/check-astronomy-preflight.ps1` | 不得缺少 GB/T 33661、JPL Horizons、IAU SOFA、NAIF CSPICE 或 1901-2100 首批生成范围 |
| `data/generated/astronomy/generation-plan.json` | `tools/check-astronomy-preflight.ps1` | 必须保持 `draft_not_runnable`，不得宣称生成脚本已实现或 artifact 已生成 |
| `data/generated/astronomy/implementation-plan.json` | `tools/check-astronomy-preflight.ps1` | 必须保持 `planning_only`、继续 M9 generated-data planning，且 `astronomy-engine` target |
| `data/generated/astronomy/generator-contract.json` | `tools/check-astronomy-preflight.ps1`, `tools/generate-astronomy-tables.ps1` | 必须保持 `contract_only`、`sha256` hash、planned outputs 为 `not_generated` |
| `data/generated/astronomy/generator-implementation-entry.json` | `tools/check-astronomy-preflight.ps1`, `tools/generate-astronomy-tables.ps1` | 必须保持 `guarded_entrypoint_only`、source snapshot manifest metadata-only、writes=false、hashes=0、runtime behavior unchanged、`astronomy-engine` target |
| `data/generated/astronomy/source-adapter-contract.json` | `tools/check-astronomy-preflight.ps1` | 必须覆盖 GB/T、Horizons、SOFA、SPICE，且不得启用 runtime dependency 或 output claim |
| `data/generated/astronomy/source-snapshot-manifest.schema.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-snapshot-manifest-dry-run.ps1` | 必须保持 `schema_only`，覆盖 source manifest/source required fields、selected source ids、acceptance requirements 和 forbidden actions |
| `data/generated/astronomy/source-snapshot-manifest-plan.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-snapshot-manifest-dry-run.ps1` | 必须保持 `manifest_materialized_metadata_only`，sources `not_materialized`，runtime/output claims false |
| `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-snapshot-manifest-dry-run.ps1` | 必须保持 source-boundary metadata；只允许 NAIF、IAU SOFA、JPL Horizons、GB/T 四个 selected payload path/hash，runtime dependency/output claim false |
| `data/generated/astronomy/source-payload-materialization-policy.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-payload-materialization-dry-run.ps1` | 必须只允许四个 selected source-boundary payload/hash；不得写 generated artifact、改变 acceptance/runtime 或晋级 capability |
| `data/generated/astronomy/source-payload-schemas/*.schema.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-payload-materialization-dry-run.ps1` | 必须保持 `schema_only`，schema/source/kind 与 policy 匹配；不得被当作 source payload、source hash、generated artifact、runtime integration 或 capability promotion |
| `data/generated/astronomy/source-capture-procedure.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-capture-procedure-dry-run.ps1` | 必须覆盖四个 source 的 capture steps、selected payload materialization status 和 hash；不得 external full-gate call、generated artifact、runtime change 或 capability promotion |
| `data/generated/astronomy/source-payload-materialization-decision.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-payload-materialization-decision-dry-run.ps1` | 必须保持 `decision_only`，只选择 `naif-cspice` 作为单源候选；不得创建 payload directory、payload、hash、external full-gate call、generated artifact、runtime change 或 capability promotion |
| `data/generated/astronomy/selected-source-payload-materialization-preflight.json` | `tools/check-astronomy-preflight.ps1`, `tools/selected-source-payload-materialization-preflight-dry-run.ps1` | 必须保持 `preflight_only`，仅允许下一轮 selected-source-only；本轮不得创建 payload directory、selected payload、hash、external full-gate call、generated artifact、runtime change 或 capability promotion |
| `data/generated/astronomy/selected-source-payload-materialization.json` | `tools/check-astronomy-preflight.ps1` | Must record only the selected `naif-cspice` source-boundary payload and one source hash; must not be treated as generated astronomy artifact, runtime integration, Android replacement, or `astronomy-engine` support |
| `data/generated/astronomy/remaining-source-payload-strategy.json` | `tools/check-astronomy-preflight.ps1`, `tools/remaining-source-payload-strategy-dry-run.ps1` | Must remain `strategy_decision_only`, select `iau-sofa-ansi-c` as next preflight candidate, and keep new payload writes/hashes at 0 |
| `data/generated/astronomy/post-iau-remaining-source-payload-strategy.json` | `tools/check-astronomy-preflight.ps1`, `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1` | Must remain `strategy_decision_only`, select `jpl-horizons-api` as next preflight candidate, keep JPL/GB payload files absent, and keep new payload writes/hashes at 0 |
| `data/generated/astronomy/selected-jpl-horizons-payload-materialization-preflight.json` | `tools/check-astronomy-preflight.ps1`, `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1` | Must remain `preflight_only`, keep the JPL payload absent in LOOP-051, keep full-gate online query execution false, and scope the next loop to selected-source-only materialization |
| `data/generated/astronomy/selected-gb-t-payload-materialization-preflight.json` | `tools/check-astronomy-preflight.ps1`, `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1` | Must remain `preflight_only`, keep the GB/T payload absent in LOOP-053, keep source-reference capture and payload materialization false for this loop, and scope the next loop to selected-source-only materialization |
| `data/generated/astronomy/selected-gb-t-payload-materialization.json` | `tools/check-astronomy-preflight.ps1` | Must record only selected GB/T rule-reference boundary materialization; must not copy standard text, implement calendar rules, write generated artifacts, replace Android baseline, or promote `astronomy-engine` |
| `data/generated/astronomy/selected-iau-sofa-payload-materialization-preflight.json` | `tools/check-astronomy-preflight.ps1`, `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1` | Must remain `preflight_only`, keep the IAU SOFA payload absent in LOOP-048, and scope the next loop to selected-source-only materialization |
| `data/generated/astronomy/selected-iau-sofa-payload-materialization.json` | `tools/check-astronomy-preflight.ps1` | Must record only selected IAU SOFA routine/version boundary materialization; must not be treated as SOFA routine integration, generated astronomy artifact, runtime integration, Android replacement, or `astronomy-engine` support |
| `data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-payload-materialization-dry-run.ps1` | Source-boundary evidence only; must not vendor SOFA source, compile/link routines, enable runtime dependency, or claim generated astronomy support |
| `data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-payload-materialization-dry-run.ps1` | Rule-reference boundary evidence only; must not copy GB/T standard text, implement Chinese-calendar rules, enable runtime dependency, or claim generated astronomy support |
| `data/generated/astronomy/artifact-writer-plan.json` | `tools/check-astronomy-preflight.ps1`, `tools/artifact-writer-dry-run.ps1` | 必须保持 `dry_run_only`、`no_write_preview`、`sha256`，不得写文件或计算 hash |
| `data/generated/astronomy/comparison-runner-plan.json` | `tools/check-astronomy-preflight.ps1`, `tools/compare-astronomy-dry-run.ps1` | 必须保持 `dry_run_only`，绑定 Android baseline，且 rows/differences 为 0 |
| `data/generated/astronomy/golden-row-readiness-plan.json` | `tools/check-astronomy-preflight.ps1`, `tools/golden-cases-dry-run.ps1` | 必须保持 `readiness_only`，所有 category 为 `not_generated` 和 `blocked_until_generated_rows` |
| `data/generated/astronomy/replay-test-readiness-plan.json` | `tools/check-astronomy-preflight.ps1`, `tools/replay-policy-dry-run.ps1` | 必须保持 `readiness_only`，replay tests 为 0，且 replacement 不允许 |
| `data/generated/astronomy/manifests/astronomy-engine-v0-draft.json` | `tools/check-astronomy-preflight.ps1` | 必须保持 `not_accepted`，不得宣称生成命令、artifact hash 或完成对照报告 |
| `data/generated/astronomy/comparison-report-template.md` | M9 implementation | 不得被当作已生成差异报告 |
| `data/generated/astronomy/comparison.schema.json` | `tools/check-astronomy-preflight.ps1` | 必须覆盖对照结果字段、差异分类和 resolution status |
| `data/generated/astronomy/golden-cases-plan.json` | `tools/check-astronomy-preflight.ps1` | 必须保持 `planned_not_generated` 且每类黄金样例为 `not_generated` |
| `data/generated/astronomy/replay-policy-draft.md` | `tools/check-astronomy-preflight.ps1` | 必须禁止静默替换 Android baseline |
| `data/generated/astronomy/precloseout-audit.json` | `tools/check-astronomy-preflight.ps1` | 必须保持 full closeout blocked、preflight closeout allowed、`astronomy-engine` target |
| `data/generated/astronomy/preflight-closeout-decision.json` | `tools/check-astronomy-preflight.ps1` | 必须保持 M9 仅 preflight closeout、full engine closeout=false、next milestone=M10、`astronomy-engine` target |
| `tools/probe-astronomy-sources.ps1` | developers/reviewers | 不得接入 full gate；GB/T 页面可用性按 warning，JPL/SOFA/NAIF 按 required |
| `tools/generate-astronomy-tables.ps1` | `tools/check-astronomy-preflight.ps1`, developers | 允许 M9 dry-run 和 M10 guarded implementation entry；不得写入 `data/generated/astronomy/out/*`、计算 hash、修改 manifest acceptance 或改变 runtime 行为 |
| `tools/source-snapshot-manifest-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | 当前仅允许 dry-run；不得创建 source snapshot directory、manifest、snapshot 文件或 generated artifact |
| `tools/source-payload-materialization-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | 当前仅允许 dry-run；不得创建 payload directory、payload 文件、source hash 或 generated artifact |
| `tools/source-capture-procedure-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | 当前仅允许 dry-run；不得创建 payload directory、payload 文件、source hash、external full-gate call 或 generated artifact |
| `tools/source-payload-materialization-decision-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | 当前仅允许 dry-run；不得创建 payload directory、selected payload、source hash、external full-gate call 或 generated artifact |
| `tools/selected-source-payload-materialization-preflight-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | 当前仅允许 dry-run；不得创建 payload directory、selected payload、source hash、external full-gate call 或 generated artifact |
| `tools/remaining-source-payload-strategy-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | Dry-run inspection only; must keep one existing payload and must not write new payloads, compute new hashes, call external sources, or write generated artifacts |
| `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | Dry-run inspection only; must keep two existing payloads, route next work to JPL preflight, and must not write JPL/GB payloads, compute new hashes, call external sources, or write generated artifacts |
| `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | Dry-run inspection only; must keep JPL payload absent, full-gate query execution false, new hashes 0, generated artifacts 0, acceptance unchanged, and runtime unchanged |
| `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | Dry-run inspection only; after LOOP-049 must verify the IAU SOFA payload exists with the selected hash while writes=false, generated artifacts 0, acceptance unchanged, and runtime unchanged |
| `tools/artifact-writer-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | 当前仅允许 dry-run；不得创建 output directory、写 artifact、计算 hash 或宣称 accepted evidence |
| `tools/compare-astronomy-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | 当前仅允许 dry-run；不得生成 comparison rows、写文件或宣称 accepted evidence |
| `tools/golden-cases-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | 当前仅允许 dry-run；不得生成 golden rows、写文件或宣称 accepted evidence |
| `tools/replay-policy-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | 当前仅允许 dry-run；不得执行 replay tests、允许替换、写文件或宣称 accepted evidence |
| `data/generated/astronomy/README.md` | developers/reviewers | 不得宣称星历引擎、扩展范围、真太阳时或时区历史已支持 |

**Regression protection**: `powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-astronomy-preflight.ps1 -ProjectRoot .`。

### Module ID: `governance.astronomy-preflight`

**Responsibility**: 关闭 DG-008 的 M9 预检路径，确保星历升级先以并行策略、manifest schema 和对照报告模板进入治理，而不是静默替换 Android baseline。

**Regression protection**: `tools/check-astronomy-preflight.ps1` and `tools/check-project.ps1`。

### Module ID: `governance.research`

**Inputs**:
| Input | Source | Type | Constraints |
| --- | --- | --- | --- |
| source report | `markdown/reserch/*` | Markdown | 保留原文，不在原报告上直接覆盖治理结论 |
| Chinese translation | `markdown/reserch/zh-CN/*` | Markdown | 必须可追溯到原始报告 |
| intake ledger | `markdown/reserch/00-research-intake.md` | Markdown table | 必须区分 accepted-current、target、planned、deferred |
| ADR | `docs/decisions/*` | Markdown | 目标能力不得宣称 supported |

**Outputs**:
| Output | Destination | Type | Constraints |
| --- | --- | --- | --- |
| policy clauses | `markdown/General_Policy.md` | rules | 只登记可执行约束 |
| standard entries | `standard-matrix.md` | rules | gate/audit 必须可判定 |
| target modules | `module-tree.md` | ownership | planned target 不等同 implemented module |
| implementation backlog | future proposal | planning | 必须引用研究报告或 ADR |

**Regression protection**: governance scaffold check and manual trace review.

### Module ID: `governance.roadmap`

**Inputs**:
| Input | Source | Type | Constraints |
| --- | --- | --- | --- |
| current project baseline | ADR/module tree/research intake | docs | 必须反映真实 supported 能力 |
| implementation request | user/proposal | change intent | 必须映射到一个里程碑 |
| decision gates | `90-decision-gates.md` | table | 未关闭 gate 不得被代码绕过 |
| risk register | `92-risk-register.md` | table | S0 未清零不得 closeout |

**Outputs**:
| Output | Destination | Type | Constraints |
| --- | --- | --- | --- |
| milestone scope | implementation proposal | plan | 必须包含非目标和验收 |
| capability promotion | `93-capability-promotion-ledger.md`, API/UI docs | status | 证据不足不得晋级 |
| closeout evidence | PR/closeout docs | Markdown | 必须记录验证命令和剩余风险 |

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `00-roadmap-index.md` | developers/reviewers | 不得被实现细节绕过 |
| milestone files | implementation planning | 不得跳阶段宣称能力 |
| `91-anti-regression-and-governance-lock.md` | all changes | 不得削弱门禁、隐私、日期层样例 |
| `95-recursive-development-protocol.md` | recursive development | 不得跳过 Read/Preflight/Govern/Validate/Closeout |
| `96-recursive-cursor.md` | recursive development | 不得在 `design_only` 下推进业务代码 |
| `97-loop-closeout-log.md` | recursive development | 不得开始下一轮却不读取上一轮结果 |
| `98-recursive-loop-runbook.md` | recursive development | 不得扩大 slice 而不声明单一不变量 |
| `100-recursive-scale-and-goal-readiness.md` | recursive development | `goal_run` readiness 未标 `ready` 前不得一路平推 |
| `11-milestone-01-closeout-readiness.md` | milestone closeout | M1 milestone_loop 关闭前必须读取 |
| `12-milestone-01-closeout.md` | milestone closeout | M1 关闭后进入 M2 前必须读取 |
| `44-milestone-09-preflight-closeout.md` | milestone closeout | M9 只能作为 preflight 关闭，不得作为 astronomy-engine 支持证据 |
| `45-milestone-10-generated-astronomy-implementation.md` | implementation planning | M10 进入真实生成数据实施前必须保留 Android baseline 和 target capability 边界 |
| `46-milestone-10-generator-entry.md` | implementation evidence | M10 guarded entry 不得被当作 source snapshot、artifact、hash、comparison、golden row、replay 或 runtime integration 证据 |
| `47-milestone-10-source-snapshot-boundary.md` | implementation evidence | M10 source snapshot boundary 不得被当作 source snapshot materialization、generated artifact、hash 或 runtime integration 证据 |
| `48-milestone-10-source-snapshot-manifest.md` | implementation evidence | M10 source snapshot manifest metadata 不得被当作 source payload materialization、generated artifact、hash 或 runtime integration 证据 |
| `49-milestone-10-source-payload-policy.md` | implementation evidence | M10 source payload policy 不得被当作 source payload materialization、source hash、generated artifact、hash 或 runtime integration 证据 |
| `50-milestone-10-source-payload-schemas.md` | implementation evidence | M10 source payload schemas 不得被当作 source payload files、source hashes、generated artifact、runtime integration 或 `astronomy-engine` promotion 证据 |
| `51-milestone-10-source-capture-procedure.md` | implementation evidence | M10 source capture procedure 不得被当作 source payload materialization、source hash、generated artifact、runtime integration 或 `astronomy-engine` promotion 证据 |
| `52-milestone-10-first-source-payload-decision.md` | implementation evidence | M10 first source payload decision 不得被当作 selected payload file、source hash、generated artifact、runtime integration 或 `astronomy-engine` promotion 证据 |
| `53-milestone-10-selected-source-payload-preflight.md` | implementation evidence | M10 selected source payload preflight 不得被当作 selected payload file、source hash、generated artifact、runtime integration 或 `astronomy-engine` promotion 证据 |

**Regression protection**: `tools/check-project.ps1`, closeout evidence review, capability ledger review, recursive cursor review.

### Module ID: `data.generated.astronomy.loop046`

**Current status**: selected-source payload materialized for `naif-cspice` only; no generated astronomy artifact is accepted.

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-snapshot-manifest-dry-run.ps1` | Must record `selected_source_payload_materialized`, exactly one selected `naif-cspice` source payload path/hash, runtime dependency false, and output claim false |
| `data/generated/astronomy/source-payload-materialization-policy.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-payload-materialization-dry-run.ps1` | Must record selected-source-only materialization: only `naif-cspice-kernel-boundary.json`, one source payload hash, generated artifacts 0 |
| `data/generated/astronomy/source-capture-procedure.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-capture-procedure-dry-run.ps1` | Must record selected `naif-cspice` source-boundary payload materialized/hash computed, keep other sources not started, and forbid external full-gate calls, generated artifacts, runtime change, or capability promotion |
| `data/generated/astronomy/selected-source-payload-materialization.json` | `tools/check-astronomy-preflight.ps1` | Must record LOOP-046 selected `naif-cspice` materialization evidence, one payload, one source hash, and no generated artifacts, acceptance change, runtime change, or capability promotion |
| `data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-payload-materialization-dry-run.ps1` | Source-boundary evidence only; must not be treated as SPICE kernel, CSPICE toolkit, generated astronomy artifact, runtime integration, or supported capability evidence |
| `markdown/20-roadmap/54-milestone-10-selected-source-payload-materialization.md` | implementation evidence | Must not be treated as SPICE kernel, CSPICE toolkit, generated astronomy artifact, runtime integration, Android replacement, or `astronomy-engine` promotion evidence |

**Regression protection**: `tools/check-astronomy-preflight.ps1`, selected-source dry-runs, `tools/check-project.ps1`.

### Module ID: `data.generated.astronomy.loop047`

**Current status**: remaining-source payload strategy decision only; no second source payload is materialized.

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `data/generated/astronomy/remaining-source-payload-strategy.json` | `tools/check-astronomy-preflight.ps1`, `tools/remaining-source-payload-strategy-dry-run.ps1` | Must select `iau-sofa-ansi-c` as next preflight-only source candidate while keeping new payload writes 0, new source hashes 0, generated artifacts 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target |
| `tools/remaining-source-payload-strategy-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | Dry-run inspection only; must not write payloads, compute new hashes, call external sources, write generated artifacts, or change runtime behavior |
| `markdown/20-roadmap/55-milestone-10-remaining-source-payload-strategy.md` | implementation evidence | Must not be treated as IAU SOFA payload materialization, JPL/GB/T capture, generated astronomy artifact, runtime integration, Android replacement, or `astronomy-engine` promotion evidence |

**Regression protection**: `tools/remaining-source-payload-strategy-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `tools/check-project.ps1`.

### Module ID: `data.generated.astronomy.loop048`

**Current status**: selected IAU SOFA source payload materialization preflight only; no second source payload is materialized.

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `data/generated/astronomy/selected-iau-sofa-payload-materialization-preflight.json` | `tools/check-astronomy-preflight.ps1`, `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1` | Must remain `preflight_only`; must keep `iau-sofa-routine-version.json` absent, new payload writes 0, new source hashes 0, generated artifacts 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target |
| `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | Dry-run inspection only; must not write payloads, compute new hashes, call external sources, write generated artifacts, change acceptance, or change runtime behavior |
| `markdown/20-roadmap/56-milestone-10-selected-iau-sofa-payload-preflight.md` | implementation evidence | Must not be treated as IAU SOFA routine integration, payload materialization, generated astronomy artifact, runtime integration, Android replacement, or `astronomy-engine` promotion evidence |

**Regression protection**: `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `tools/check-project.ps1`.

### Module ID: `data.generated.astronomy.loop049`

**Current status**: selected IAU SOFA source-boundary payload materialized; no generated astronomy artifact is accepted.

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-payload-materialization-dry-run.ps1` | Source-boundary evidence only; must not vendor SOFA source, compile/link SOFA routines, enable runtime dependency, write generated artifacts, change acceptance, or promote `astronomy-engine` |
| `data/generated/astronomy/selected-iau-sofa-payload-materialization.json` | `tools/check-astronomy-preflight.ps1` | Must record LOOP-049 selected IAU SOFA materialization evidence, one new payload, one new source hash, and no generated artifacts, acceptance change, runtime change, or capability promotion |
| `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-snapshot-manifest-dry-run.ps1` | Must record exactly two selected source payload paths/hashes: `naif-cspice` and `iau-sofa-ansi-c`; JPL/GB/T remain `not_materialized` |
| `data/generated/astronomy/source-payload-materialization-policy.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-payload-materialization-dry-run.ps1` | Must allow only selected NAIF and IAU SOFA payloads; JPL/GB/T payloads, generated artifacts, runtime change, and capability promotion remain forbidden |
| `markdown/20-roadmap/57-milestone-10-selected-iau-sofa-payload-materialization.md` | implementation evidence | Must not be treated as SOFA routine integration, generated astronomy artifact, runtime integration, Android replacement, or `astronomy-engine` promotion evidence |

**Regression protection**: source payload dry-runs, `tools/check-astronomy-preflight.ps1`, `tools/check-project.ps1`.

### Module ID: `data.generated.astronomy.loop050`

**Current status**: post-IAU remaining-source payload strategy decision only; JPL Horizons and GB/T payloads remain absent.

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `data/generated/astronomy/post-iau-remaining-source-payload-strategy.json` | `tools/check-astronomy-preflight.ps1`, `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1` | Must select `jpl-horizons-api` as next selected-source-only preflight candidate while keeping JPL/GB payload writes 0, new source hashes 0, generated artifacts 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target |
| `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | Dry-run inspection only; must not write JPL/GB payloads, compute new hashes, call external sources, write generated artifacts, change acceptance, or change runtime behavior |
| `markdown/20-roadmap/58-milestone-10-post-iau-remaining-source-payload-strategy.md` | implementation evidence | Must not be treated as JPL payload materialization, GB/T payload materialization, generated astronomy artifact, runtime integration, Android replacement, or `astronomy-engine` promotion evidence |

**Regression protection**: `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `tools/check-project.ps1`.

### Module ID: `data.generated.astronomy.loop051`

**Current status**: selected JPL Horizons source payload materialization preflight only; JPL Horizons and GB/T payloads remain absent.

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `data/generated/astronomy/selected-jpl-horizons-payload-materialization-preflight.json` | `tools/check-astronomy-preflight.ps1`, `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1` | Must remain `preflight_only`; must keep JPL payload absent, GB/T payload absent, online JPL query execution out of the full gate, new source hashes 0, generated artifacts 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target |
| `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | Dry-run inspection only; must not write payloads, compute new hashes, call external sources, write generated artifacts, change acceptance, or change runtime behavior |
| `markdown/20-roadmap/59-milestone-10-selected-jpl-horizons-payload-preflight.md` | implementation evidence | Must not be treated as JPL payload materialization, online JPL query execution, GB/T payload materialization, generated astronomy artifact, runtime integration, Android replacement, or `astronomy-engine` promotion evidence |

**Regression protection**: `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `tools/check-project.ps1`.

### Module ID: `data.generated.astronomy.loop052`

**Current status**: selected JPL Horizons validation-query snapshot boundary payload materialized; no online JPL query runs in the full gate and no generated astronomy artifact is accepted.

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-payload-materialization-dry-run.ps1` | Offline query-parameter boundary evidence only; must not be treated as JPL response-body capture, runtime network dependency, generated astronomy artifact, Android replacement, or supported `astronomy-engine` evidence |
| `data/generated/astronomy/selected-jpl-horizons-payload-materialization.json` | `tools/check-astronomy-preflight.ps1` | Must record LOOP-052 selected JPL materialization evidence, one new payload, one new source hash, no full-gate online query, no response bodies, and no generated artifacts, acceptance change, runtime change, Android replacement, or capability promotion |
| `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-snapshot-manifest-dry-run.ps1` | Must record exactly three selected source payload paths/hashes: `naif-cspice`, `iau-sofa-ansi-c`, and `jpl-horizons-api`; GB/T remains `not_materialized` |
| `data/generated/astronomy/source-payload-materialization-policy.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-payload-materialization-dry-run.ps1` | Must allow only selected NAIF, IAU SOFA, and JPL Horizons payloads; GB/T payloads, generated artifacts, runtime change, and capability promotion remain forbidden |
| `markdown/20-roadmap/60-milestone-10-selected-jpl-horizons-payload-materialization.md` | implementation evidence | Must not be treated as online JPL query execution, JPL response-body capture, GB/T payload materialization, generated astronomy artifact, runtime integration, Android replacement, or `astronomy-engine` promotion evidence |

**Regression protection**: source payload dry-runs, `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `tools/check-project.ps1`.

### Module ID: `data.generated.astronomy.loop053`

**Current status**: selected GB/T 33661 rule-reference payload materialization preflight only; the GB/T payload and source hash remain absent.

**Public surface**:
| Surface | Caller | Forbidden |
| --- | --- | --- |
| `data/generated/astronomy/selected-gb-t-payload-materialization-preflight.json` | `tools/check-astronomy-preflight.ps1`, `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1` | Must remain `preflight_only`; must keep GB/T payload absent, source-reference capture false, payload materialization false, new source hashes 0, generated artifacts 0, acceptance unchanged, runtime unchanged, Android baseline unchanged, and `astronomy-engine` target |
| `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1` | `tools/check-astronomy-preflight.ps1`, developers | Dry-run inspection only; must not write payloads, compute GB/T source hashes, call external sources, write generated artifacts, change acceptance, runtime behavior, or Android baseline |
| `markdown/20-roadmap/61-milestone-10-selected-gb-t-payload-preflight.md` | implementation evidence | Must not be treated as GB/T rule payload materialization, generated astronomy artifact, runtime integration, Android replacement, or `astronomy-engine` promotion evidence |

**Regression protection**: `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1`, `tools/check-astronomy-preflight.ps1`, `tools/check-project.ps1`.

### Module ID: `tools.governance`

**Public surface**:
| Surface | Input | Output |
| --- | --- | --- |
| `tools/check-project.ps1` | project root | Rust、JS、治理全套检查 |
| `tools/check-governance-scaffold.ps1` | project root | governance scaffold pass/fail |
| `tools/check-release-candidate.ps1` | project root | M8 release candidate status freeze pass/fail |
| `tools/check-astronomy-preflight.ps1` | project root | M9 astronomy preflight pass/fail |
| `tools/probe-astronomy-sources.ps1` | project root | optional external source availability report |
| `tools/generate-astronomy-tables.ps1` | project root + manifest + dry-run or guarded entry | optional dry-run generation plan report; guarded M10 entry report |
| `tools/source-snapshot-manifest-dry-run.ps1` | project root | optional dry-run source snapshot manifest boundary report |
| `tools/source-payload-materialization-dry-run.ps1` | project root | optional dry-run source payload materialization policy report |
| `tools/source-capture-procedure-dry-run.ps1` | project root | optional dry-run source capture procedure report |
| `tools/source-payload-materialization-decision-dry-run.ps1` | project root | optional dry-run first source payload decision report |
| `tools/selected-source-payload-materialization-preflight-dry-run.ps1` | project root | optional dry-run selected source payload preflight report |
| `tools/remaining-source-payload-strategy-dry-run.ps1` | project root | optional dry-run remaining source strategy report |
| `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1` | project root | optional dry-run post-IAU remaining source strategy report |
| `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1` | project root | optional dry-run selected JPL Horizons payload preflight/materialization closed report |
| `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1` | project root | optional dry-run selected GB/T payload preflight report |

### Module ID: `data.generated.astronomy.loop054`

**Current status**: selected GB/T 33661 rule-reference boundary payload materialized; all four source-boundary payloads now exist, while generated artifacts, generated artifact hashes, manifest acceptance, runtime replacement, Android baseline replacement, and `astronomy-engine` promotion remain absent.

| Public surface | Caller | Constraint |
| --- | --- | --- |
| `data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json` | `tools/check-astronomy-preflight.ps1`, `tools/source-payload-materialization-dry-run.ps1` | Must remain boundary-only reference evidence; no copied standard text and no implemented calendar algorithm |
| `data/generated/astronomy/selected-gb-t-payload-materialization.json` | `tools/check-astronomy-preflight.ps1` | Must record LOOP-054 selected GB/T materialization evidence, one new payload, one new source hash, and no generated artifacts, acceptance change, runtime change, Android replacement, or capability promotion |
| `markdown/20-roadmap/62-milestone-10-selected-gb-t-payload-materialization.md` | implementation evidence | Must not be treated as generated astronomy artifact, runtime integration, Android replacement, GB/T rules implementation, or `astronomy-engine` promotion evidence |

**Regression protection**: `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1`, source payload dry-runs, `tools/check-astronomy-preflight.ps1`, `tools/check-project.ps1`.
| `tools/artifact-writer-dry-run.ps1` | project root | optional dry-run output path/hash preview |
| `tools/compare-astronomy-dry-run.ps1` | project root + manifest | optional dry-run comparison shape report |
| `tools/golden-cases-dry-run.ps1` | project root | optional dry-run golden category report |
| `tools/replay-policy-dry-run.ps1` | project root + manifest | optional dry-run replay-control report |
| `tools/inventory-project.ps1` | project root | full tree draft |

**Regression protection**: `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1`。
