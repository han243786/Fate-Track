# 命轨全量工程树

> 本文是工程目录地图，产品功能地图保留在 `markdown/命轨全量树.md`。后续研究报告进入项目后，必须同步更新本文和 `markdown/00-matrix-governance/module-tree.md`。

## Project Root

```text
FT/
  backend/
    src/
      api/
        analysis.rs
        calendar.rs
        cases.rs
        capabilities.rs
        chart_basis.rs
        chart_detail.rs
        charts.rs
        derive.rs
        glossary_data.rs
        health.rs
        luck.rs
        lunar.rs
        mod.rs
        report.rs
        settings.rs
        share.rs
      astronomy/
        calendar.rs
        compare.rs
        mod.rs
        moon.rs
        sun.rs
        terms.rs
        time.rs
      calendar/
        civil.rs
        ganzhi.rs
        lunar_data.rs
        mod.rs
      domain/
        analysis.rs
        bazi.rs
        cases.rs
        glossary.rs
        deep_analysis.rs
        luck.rs
        mod.rs
        settings.rs
        share.rs
      http/
        json.rs
        mod.rs
        request.rs
        response.rs
        status.rs
      app.rs
      config.rs
      error.rs
      lib.rs
      main.rs
      server.rs
    tests/
      generate_artifacts.rs
      generate_golden.rs
      replay_tests.rs
      run_comparison.rs
    Cargo.toml
  desktop/
    src/
      main.rs
    Cargo.toml
  data/
    generated/
      astronomy/
        manifests/
          astronomy-engine-v0-draft.json
        source-payload-schemas/
          gb-t-33661-2017-rule-reference.schema.json
          iau-sofa-routine-version.schema.json
          jpl-horizons-validation-samples.schema.json
          naif-cspice-kernel-boundary.schema.json
        source-snapshots/
          payloads/
            gb-t-33661-2017-rule-reference.json
            iau-sofa-routine-version.json
            jpl-horizons-validation-samples.json
            naif-cspice-kernel-boundary.json
          source-snapshot-manifest.json
        artifact-writer-plan.json
        comparison-runner-plan.json
        comparison-report-template.md
        comparison.schema.json
        generation-plan.json
        generator-contract.json
        generator-implementation-entry.json
        golden-cases-plan.json
        golden-row-readiness-plan.json
        implementation-plan.json
        manifest.schema.json
        precloseout-audit.json
        post-iau-remaining-source-payload-strategy.json
        README.md
        remaining-source-payload-strategy.json
        replay-policy-draft.md
        replay-test-readiness-plan.json
        selected-iau-sofa-payload-materialization.json
        selected-iau-sofa-payload-materialization-preflight.json
        selected-gb-t-payload-materialization.json
        selected-gb-t-payload-materialization-preflight.json
        selected-jpl-horizons-payload-materialization.json
        selected-jpl-horizons-payload-materialization-preflight.json
        selected-source-payload-materialization.json
        selected-source-payload-materialization-preflight.json
        source-adapter-contract.json
        source-capture-procedure.json
        source-payload-materialization-decision.json
        source-payload-materialization-policy.json
        source-policy.json
        source-snapshot-manifest-plan.json
        source-snapshot-manifest.schema.json
    raw/
      lunar_data.yaml
      golden-fixtures.json
    out/
      android-comparison-1901-2100.json
      lunar-calendar-1901-2100.json
      new-moons-1901-2100.json
      solar-terms-1901-2100.json
    README.md
  docs/
    decisions/
      0001-stack-and-data-source.md
      0002-android-date-layer-source.md
      0003-v1-research-governance-baseline.md
      0004-v1-calculation-ruleset-target.md
      0005-privacy-safe-interpretation-target.md
      0006-roadmap-and-governance-lock.md
      0007-recursive-development-protocol.md
      0008-v1-official-validation-range.md
      0009-m2-chart-basis-contract.md
      0010-http-architecture-through-chart-engine.md
      0011-m3-chart-engine-core.md
      0012-structured-analysis-only.md
      0013-local-volatile-case-storage.md
      0014-share-token-privacy-boundary.md
      0015-m9-astronomy-parallel-strategy.md
      0016-m9-astronomy-source-stack.md
      0017-m9-generated-data-implementation-path.md
      0018-month-boundary-correction.md
      0019-m11-astronomy-engine-architecture.md
      0020-dg-005-luck-cycle-rules.md
    release/
      v1-release-candidate.md
      v1-closeout.md
  frontend/
    src/
      api/
        client.js
      ui/
        dom.js
        render.js
      utils/
        format.js
      config.js
      main.js
      state.js
      styles.css
    tests/
      api-client.test.mjs
      format.test.mjs
      workspace-markup.test.mjs
    index.html
    package.json
    server.mjs
  markdown/
    00-matrix-governance/
      guidance-matrix.md
      module-tree.md
      process-matrix.md
      README.md
      standard-matrix.md
    01-principles/
      principles-super-standardization.md
    10-overview/
      overview-full-feature-tree.md
    20-roadmap/
      00-roadmap-index.md
      README.md
      01-milestone-00-foundation-lock.md
      02-milestone-01-date-layer-hardening.md
      03-milestone-02-ruleset-and-chart-basis.md
      04-milestone-03-chart-engine.md
      05-milestone-04-analysis-engine.md
      06-milestone-05-case-storage.md
      07-milestone-06-share-privacy.md
      08-milestone-07-frontend-workspace.md
      09-milestone-08-validation-release.md
      10-milestone-09-astronomy-upgrade.md
      11-milestone-01-closeout-readiness.md
      12-milestone-01-closeout.md
      13-milestone-02-preflight.md
      14-milestone-02-closeout.md
      15-milestone-03-preflight.md
      16-milestone-03-closeout.md
      17-milestone-04-preflight.md
      18-milestone-04-closeout.md
      19-milestone-05-preflight.md
      20-milestone-05-closeout.md
      21-milestone-06-preflight.md
      22-milestone-06-closeout.md
      23-milestone-07-preflight.md
      24-milestone-07-closeout.md
      25-milestone-08-preflight.md
      26-milestone-08-closeout.md
      27-milestone-09-preflight.md
      28-milestone-09-source-availability.md
      29-milestone-09-manifest-draft.md
      30-milestone-09-generation-plan.md
      31-milestone-09-generator-dry-run.md
      32-milestone-09-comparison-golden-replay-plan.md
      33-milestone-09-comparison-dry-run.md
      34-milestone-09-golden-dry-run.md
      35-milestone-09-replay-policy-dry-run.md
      36-milestone-09-pre-closeout-audit.md
      37-milestone-09-generated-data-implementation-plan.md
      38-milestone-09-generator-contract.md
      39-milestone-09-source-adapter-contract.md
      40-milestone-09-artifact-writer-dry-run.md
      41-milestone-09-comparison-runner-dry-run.md
      42-milestone-09-golden-row-readiness.md
      43-milestone-09-replay-test-readiness.md
      44-milestone-09-preflight-closeout.md
      45-milestone-10-generated-astronomy-implementation.md
      46-milestone-10-generator-entry.md
      47-milestone-10-source-snapshot-boundary.md
      48-milestone-10-source-snapshot-manifest.md
      49-milestone-10-source-payload-policy.md
      50-milestone-10-source-payload-schemas.md
      51-milestone-10-source-capture-procedure.md
      52-milestone-10-first-source-payload-decision.md
      53-milestone-10-selected-source-payload-preflight.md
      54-milestone-10-selected-source-payload-materialization.md
      55-milestone-10-remaining-source-payload-strategy.md
      56-milestone-10-selected-iau-sofa-payload-preflight.md
      57-milestone-10-selected-iau-sofa-payload-materialization.md
      58-milestone-10-post-iau-remaining-source-payload-strategy.md
      59-milestone-10-selected-jpl-horizons-payload-preflight.md
      60-milestone-10-selected-jpl-horizons-payload-materialization.md
      61-milestone-10-selected-gb-t-payload-preflight.md
      62-milestone-10-selected-gb-t-payload-materialization.md
      90-decision-gates.md
      91-anti-regression-and-governance-lock.md
      92-risk-register.md
      93-capability-promotion-ledger.md
      94-closeout-evidence-template.md
      95-recursive-development-protocol.md
      96-recursive-cursor.md
      97-loop-closeout-log.md
      98-recursive-loop-runbook.md
      99-milestone-01-preflight-dry-run.md
      100-recursive-scale-and-goal-readiness.md
    reserch/
      zh-CN/
        Fate-Track V1 Design Report.zh-CN.md
        Fate-Track V1 Product Spec and Engineering Plan.zh-CN.md
      00-research-intake.md
      README.md
      Fate-Track V1 Design Report.md
      Fate-Track V1 Product Spec and Engineering Plan.md
      命轨 Fate-Track V1 产品需求与八字算法规格研究报告.md
    templates/
      closeout-template.md
      proposal-template.md
    General_Policy.md
    命轨全量树.md
  tools/
    check-governance-scaffold.ps1
    check-astronomy-preflight.ps1
    check-project.ps1
    check-release-candidate.ps1
    probe-astronomy-sources.ps1
    generate-astronomy-tables.ps1
    source-snapshot-manifest-dry-run.ps1
    source-payload-materialization-dry-run.ps1
    source-capture-procedure-dry-run.ps1
    source-payload-materialization-decision-dry-run.ps1
    artifact-writer-dry-run.ps1
    compare-astronomy-dry-run.ps1
    golden-cases-dry-run.ps1
    replay-policy-dry-run.ps1
    remaining-source-payload-strategy-dry-run.ps1
    post-iau-remaining-source-payload-strategy-dry-run.ps1
    selected-gb-t-payload-materialization-preflight-dry-run.ps1
    selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1
    selected-iau-sofa-payload-materialization-preflight-dry-run.ps1
    selected-source-payload-materialization-preflight-dry-run.ps1
    inventory-project.ps1
  .gitignore
  Cargo.lock
  Cargo.toml
  README.md
```

## Top-Level Responsibilities

| Path | Kind | Responsibility | Module Tree |
| --- | --- | --- | --- |
| `backend/` | directory | Rust 后端，包含入口、配置、HTTP、API、Android 日期层移植、农历数据、领域模型骨架。 | `backend.*` |
| `frontend/` | directory | JavaScript 前端，包含静态服务、API client、状态、渲染、格式化和基础测试。 | `frontend.*` |
| `data/` | directory | 项目内数据区；`raw/` 保存外部农历基础数据的原始副本。 | `data.lunar.raw` |
| `data/generated/astronomy/` | directory | M9 星历生成物 manifest schema、对照报告模板和未来黄金表落点。 | `data.generated.astronomy` |
| `docs/` | directory | 架构决策记录与后续研究报告落点。 | `governance.decisions` |
| `docs/release/` | directory | V1 release candidate、验证命令、能力冻结和降级规则。 | `governance.release` |
| `markdown/` | directory | 治理体系、产品全量树、策略矩阵、流程矩阵和模板。 | `governance.matrix` |
| `tools/` | directory | 治理和项目检查脚本。 | `tools.governance` |
| `.gitignore` | file | 排除 Rust/Node 构建产物、缓存和日志。 | `system.workspace` |
| `Cargo.lock` | file | Rust 应用依赖锁文件。 | `system.workspace` |
| `Cargo.toml` | file | Rust workspace 根配置。 | `system.workspace` |
| `README.md` | file | 项目入口、运行命令和基础检查命令。 | `system.workspace` |

## Source Classification

| Path | Class | Rule |
| --- | --- | --- |
| `backend/src/**/*.rs` | source | 必须通过 `cargo fmt --check` 与 `cargo test`。 |
| `frontend/src/**/*.js` | source | 必须通过 `npm.cmd run check`。 |
| `frontend/tests/**/*.mjs` | test | 前端纯函数和状态逻辑测试。 |
| `frontend/server.mjs` | source/tool | 本地静态服务，不得暴露项目根以外文件。 |
| `data/raw/lunar_data.yaml` | raw data | 禁止应用代码直接改写；衍生产物必须登记生成命令。 |
| `data/generated/astronomy/*` | preflight data governance | 当前仅含 schema/template；不得作为已生成星历表使用。 |
| `markdown/命轨全量树.md` | product map | 功能范围变化时同步更新。 |
| `markdown/00-matrix-governance/*` | governance | 模块、规则、门禁或流程变化时同步更新。 |
| `markdown/20-roadmap/*` | roadmap | 开发里程碑、决策门、风险、防回退锁、capability 晋级台账和递归游标。 |
| `docs/release/v1-release-candidate.md` | release | M8 release candidate 边界冻结、验证命令和降级规则。 |
| `tools/check-release-candidate.ps1` | gate | M8 release candidate 状态冻结和过度宣称检查。 |
| `markdown/reserch/*` | research | 原始研究、中文译文和治理纳入台账；实现前必须登记采纳状态。 |
| `target/`, `node_modules/`, `dist/`, `.cache/` | generated/cache | 不进入治理全量树，不作为源文件评审对象。 |

## Research Report Insertion Points

| Research Topic | First Code Landing Zone | Governance Update |
| --- | --- | --- |
| 农历/节气算法 | `backend/src/calendar/` | `backend.calendar.*` |
| 八字排盘算法 | `backend/src/domain/bazi.rs` + future `backend/src/bazi/` | `backend.domain.bazi` |
| API 合约 | `backend/src/api/` + `frontend/src/api/` | `backend.api`, `frontend.api` |
| 页面与交互 | `frontend/src/ui/` + `frontend/index.html` | `frontend.ui` |
| 存储与案例 | `backend/src/domain/cases.rs` + future storage module | `backend.domain.cases` |
| 隐私与分享 | `backend/src/domain/cases.rs` + future share module | `backend.domain.share` |
| 研究报告纳入 | `markdown/reserch/00-research-intake.md` + `docs/decisions/*` | `governance.research` |
| 开发里程碑 | `markdown/20-roadmap/*` + `docs/decisions/0006-roadmap-and-governance-lock.md` | `governance.roadmap` |

## Current Supported Calendar Surface

Date-layer query responses include `meta.source`, `meta.algorithm_version`, `meta.ruleset_id`, `meta.support_range`, `meta.boundary_policy`, and `meta.limitations` for the current Android date-only baseline. This surface does not imply full chart, hour-pillar, timezone-history, true-solar-time, or astronomy-engine support.

`/api/capabilities` exposes `calendar-date-query-v1-meta` as a supported traceability capability for the same date query route.

The backend date-layer regression suite embeds the 49-case Android edge manifest from `D:\myproject\Perpetual calendar\data\edge_case_test.txt`.

Date-query API tests cover missing date, invalid date, out-of-range dates, valid boundary dates, and missing lunar data source.

The frontend Date Layer panel calls `ApiClient.calendarDate` and renders the supported date-only result plus ruleset metadata.

| Route | Real Files | Source |
| --- | --- | --- |
| `GET /api/calendar/query?date=YYYY-MM-DD` | `backend/src/api/calendar.rs`, `backend/src/calendar/*` | Android 万年历日期层移植 |
| `GET /api/charts/basis/preview` | `backend/src/api/chart_basis.rs`, `backend/src/domain/bazi.rs` | M2 `ft-v1-default` contract |
| `GET /api/charts` | `backend/src/api/charts.rs`, `backend/src/domain/bazi.rs`, `backend/src/calendar/*` | M3 chart engine core |
| `GET /api/analysis/snapshot` | `backend/src/api/analysis.rs`, `backend/src/domain/analysis.rs` | M4 structured analysis |
| `GET /api/cases` | `backend/src/api/cases.rs`, `backend/src/domain/cases.rs` | M5 local volatile case storage |
| `GET /api/settings` | `backend/src/api/settings.rs`, `backend/src/domain/settings.rs` | M5 local volatile preferences |
| `GET /api/share/preview` | `backend/src/api/share.rs`, `backend/src/domain/share.rs`, `backend/src/domain/cases.rs` | M6 local volatile redacted share preview |

## Current Supported Chart Surface

`GET /api/charts` supports chart creation for Gregorian input inside 1901-2100 using the current Android date layer and `ft-v1-default`. Exact time returns an hour pillar. Unknown hour returns `hour: null` and all hour-pillar candidates. The route records warnings and unsupported outputs; IANA timezone history, true solar time, lunar input conversion, persisted chart detail, analysis, luck cycles, and astronomy replacement remain unsupported.

## Current Supported Analysis Surface

`GET /api/analysis/snapshot` supports deterministic structured analysis on the current chart output. It returns element, ten-god, hidden-stem, relation, sensitivity, fixed-card, disclaimer, and forbidden-output-audit data. It does not return generated prose, luck cycles, storage, sharing, medical/legal/financial/death/fertility/relationship certainty claims, true solar time, timezone history, or astronomy replacement.

## Current Restricted Storage Surface

`GET /api/cases` and `GET /api/settings` are M5 restricted local volatile surfaces. Cases store immutable chart/analysis snapshot references with algorithm versions, support create/detail/list/metadata update/archive/delete actions, and omit private notes from list responses. Settings store local preferences for default calendar, privacy default, language, and theme. These routes do not provide database persistence, accounts, cloud sync, cross-device sync, public sharing, share tokens, luck cycles, or generated analysis.

## Current Restricted Share Surface

`GET /api/share/preview` is an M6 restricted local volatile share surface. It creates share records from immutable M5 case snapshots, returns raw tokens only in create responses, stores only token hashes, supports revocation and public redacted DTO reads, and returns `noindex:true` plus `editable:false`. Public DTOs omit private notes, raw titles, tags, private case ids, exact birth-time/location fields, and snapshot ids. Missing, expired, invalid, and revoked tokens share the same unavailable response shape.

## Current Restricted Frontend Workspace

`frontend/index.html` and `frontend/src/*` now provide an M7 restricted workspace. It consumes backend APIs for chart creation, structured analysis, local volatile cases, redacted share preview, date-layer query, data metadata, and capability boundaries. It does not implement chart or analysis algorithms locally and does not claim luck cycles, durable public sharing, account storage, cloud sync, true solar time, timezone history, glossary, wider date range, or astronomy replacement.

## Current Release Candidate Surface

`docs/release/v1-release-candidate.md` records the M8 V1 release candidate boundary. It freezes supported date-layer/chart/analysis capabilities, restricted case/settings/share/frontend surfaces, and planned/target future capabilities. `tools/check-release-candidate.ps1` is included in `tools/check-project.ps1` and verifies closeout artifacts, frozen capability statuses, README route boundaries, frontend overclaim tests, and share privacy test evidence.

`release-candidate` is a governance and delivery capability only. It does not add a new backend business API and does not change `/api/capabilities` beyond the existing V1 business surfaces.

## Current Astronomy Preflight Surface

ADR 0015 closes DG-008 for M9 preflight by selecting a parallel-first astronomy upgrade strategy. ADR 0016 selects the source stack: GB/T 33661-2017, NASA/JPL Horizons API, IAU SOFA ANSI C, and NAIF CSPICE/SPICE. Android remains the accepted-current V1 baseline. `data/generated/astronomy/source-policy.json`, `generation-plan.json`, `manifest.schema.json`, `manifests/astronomy-engine-v0-draft.json`, and `comparison-report-template.md` define the minimum evidence expected before any generated astronomy table or engine can be promoted.

`tools/check-astronomy-preflight.ps1` is included in the full project gate. It verifies the parallel-first ADR, source-stack ADR, source policy, DG-008 status, manifest required fields, difference taxonomy, comparison report template, and that `astronomy-engine` remains target until generated evidence exists.

`tools/probe-astronomy-sources.ps1` is an optional network probe for M9 source availability. It is not part of the full project gate.

`tools/generate-astronomy-tables.ps1` is currently a dry-run-only generator skeleton. The astronomy preflight checker invokes it and verifies that it performs no writes and does not change manifest acceptance status.

`comparison.schema.json`, `golden-cases-plan.json`, and `replay-policy-draft.md` define the evidence required before generated astronomy data can be accepted. They are planning artifacts only.

`tools/compare-astronomy-dry-run.ps1` is currently a dry-run-only comparison scaffold. It emits zero comparison rows and no accepted evidence.

`tools/golden-cases-dry-run.ps1` is currently a dry-run-only golden-case scaffold. It emits zero generated rows and no accepted evidence.

`tools/replay-policy-dry-run.ps1` is currently a dry-run-only replay-policy scaffold. It emits required replacement controls while executing zero replay tests and no accepted evidence.

`data/generated/astronomy/precloseout-audit.json` records that M9 is ready only for preflight closeout review. Full astronomy-engine closeout stays blocked until generated artifacts, hashes, comparison rows, golden rows, replay tests, and runtime integration exist.

`data/generated/astronomy/implementation-plan.json` records the planning-only generated-data path selected by ADR 0017. It keeps the next work on generator contract planning and still forbids accepted artifacts or runtime replacement.

`data/generated/astronomy/generator-contract.json` defines the contract-only generator input/output, manifest update, and `sha256` hash rules before any generated row exists.

`data/generated/astronomy/source-adapter-contract.json` defines contract-only source boundaries for GB/T, Horizons, SOFA, and SPICE without adding runtime dependencies or external calls to the full gate.

`data/generated/astronomy/artifact-writer-plan.json` and `tools/artifact-writer-dry-run.ps1` preview output paths and `sha256` hash status without creating directories, writing files, or accepting artifacts.

`data/generated/astronomy/comparison-runner-plan.json` binds Android baseline metadata to the future comparison artifact while `tools/compare-astronomy-dry-run.ps1` still emits zero rows and no accepted evidence.

`data/generated/astronomy/golden-row-readiness-plan.json` keeps every golden category not generated and blocked until generated rows and comparison evidence exist.

`data/generated/astronomy/replay-test-readiness-plan.json` keeps replay tests unexecuted while preserving the old Android algorithm/ruleset replay boundary.

`data/generated/astronomy/preflight-closeout-decision.json` and `markdown/20-roadmap/44-milestone-09-preflight-closeout.md` close M9 only as a preflight milestone. Real generated astronomy implementation moves to M10, and `astronomy-engine` remains `target` until generated artifacts, hashes, comparison report, golden rows, replay tests, runtime integration, and replacement policy evidence exist.

`data/generated/astronomy/generator-implementation-entry.json` and `tools/generate-astronomy-tables.ps1 -PrepareImplementation` add the first M10 guarded generator implementation entry. The entry is non-dry-run in shape but remains blocked with no source snapshot manifest, no artifact writes, hashes 0, manifest acceptance unchanged, runtime unchanged, and `astronomy-engine` target.

`data/generated/astronomy/source-snapshot-manifest.schema.json`, `source-snapshot-manifest-plan.json`, `source-snapshots/source-snapshot-manifest.json`, and `tools/source-snapshot-manifest-dry-run.ps1` define and materialize the M10 source snapshot manifest metadata. The manifest now records exactly four selected source-boundary payloads and hashes for `naif-cspice`, `iau-sofa-ansi-c`, `jpl-horizons-api`, and `gb-t-33661-2017`; no generated artifacts, generated artifact hashes, manifest acceptance, runtime behavior, or capability status changed.

`data/generated/astronomy/source-payload-materialization-policy.json`, `data/generated/astronomy/source-payload-schemas/*.schema.json`, `data/generated/astronomy/selected-source-payload-materialization.json`, `data/generated/astronomy/selected-iau-sofa-payload-materialization.json`, `data/generated/astronomy/selected-jpl-horizons-payload-materialization.json`, `data/generated/astronomy/selected-gb-t-payload-materialization.json`, source payload files, and `tools/source-payload-materialization-dry-run.ps1` define the M10 selected-source payload materialization state. Exactly four source-boundary payloads exist for `naif-cspice`, `iau-sofa-ansi-c`, `jpl-horizons-api`, and `gb-t-33661-2017`; generated artifacts, generated artifact hashes, manifest acceptance, runtime behavior, and capability status remain unchanged.

`data/generated/astronomy/source-capture-procedure.json` and `tools/source-capture-procedure-dry-run.ps1` define the M10 source capture procedure. It records `naif-cspice`, `iau-sofa-ansi-c`, `jpl-horizons-api`, and `gb-t-33661-2017` as completed for source-boundary payload materialization; external calls are not part of the full gate, generated artifacts remain absent, manifest acceptance is unchanged, runtime behavior is unchanged, and `astronomy-engine` remains target.

`data/generated/astronomy/source-payload-materialization-decision.json` and `tools/source-payload-materialization-decision-dry-run.ps1` define the M10 first source payload decision. The decision is `decision_only`: `naif-cspice` is selected as the first single-source candidate, but the payload directory and selected payload remain absent, source hashes remain 0, external calls are not part of the full gate, generated artifacts remain absent, manifest acceptance is unchanged, runtime behavior is unchanged, and `astronomy-engine` remains target.

`data/generated/astronomy/selected-source-payload-materialization-preflight.json` and `tools/selected-source-payload-materialization-preflight-dry-run.ps1` define the M10 selected-source payload preflight. The preflight remains LOOP-045 historical evidence; LOOP-046 closes it by materializing only `naif-cspice-kernel-boundary.json` as source-boundary evidence. This is not SPICE kernel/toolkit integration, generated astronomy data, runtime integration, Android replacement, or `astronomy-engine` support.

`data/generated/astronomy/remaining-source-payload-strategy.json` and `tools/remaining-source-payload-strategy-dry-run.ps1` define the M10 remaining source payload strategy. The strategy chooses `iau-sofa-ansi-c` as the next preflight-only source candidate, then JPL Horizons, then GB/T; it writes no new payload files, computes no new source hashes, performs no external full-gate calls, writes no generated artifacts, changes no manifest acceptance or runtime behavior, and keeps `astronomy-engine` target.

`data/generated/astronomy/selected-iau-sofa-payload-materialization-preflight.json` and `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1` define the M10 selected IAU SOFA payload preflight. The preflight keeps `data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json` absent in LOOP-048, observes exactly one existing `naif-cspice` payload, writes no new payload files, computes no new source hashes, performs no external full-gate calls, writes no generated artifacts, changes no manifest acceptance or runtime behavior, and keeps `astronomy-engine` target.

`data/generated/astronomy/selected-iau-sofa-payload-materialization.json`, `data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json`, and updated source payload dry-runs define the LOOP-049 IAU SOFA source-boundary payload materialization state. Exactly two source-boundary payloads now exist: `naif-cspice` and `iau-sofa-ansi-c`; JPL Horizons and GB/T remain absent, generated artifacts and generated artifact hashes remain zero, manifest acceptance and runtime behavior are unchanged, and `astronomy-engine` remains target.

`data/generated/astronomy/post-iau-remaining-source-payload-strategy.json` and `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1` define the LOOP-050 post-IAU remaining source strategy. It selects JPL Horizons as the next selected-source-only preflight candidate and leaves GB/T for the following governed scope while keeping JPL/GB payload files absent, new source payload writes 0, new source hashes 0, external full-gate calls false, generated artifacts 0, manifest acceptance unchanged, runtime behavior unchanged, and `astronomy-engine` target.

`data/generated/astronomy/selected-jpl-horizons-payload-materialization-preflight.json` and `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1` define the LOOP-051 selected JPL Horizons payload preflight. It keeps the JPL payload absent, keeps GB/T absent, forbids online JPL query execution in the full project gate, keeps new source payload writes 0, new source hashes 0, generated artifacts 0, manifest acceptance unchanged, runtime behavior unchanged, and `astronomy-engine` target.

`data/generated/astronomy/selected-jpl-horizons-payload-materialization.json`, `data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json`, and updated source payload dry-runs define the LOOP-052 selected JPL Horizons source-boundary payload materialization state. Exactly three source-boundary payloads now exist: `naif-cspice`, `iau-sofa-ansi-c`, and `jpl-horizons-api`; GB/T remains absent, no JPL response bodies are captured, no online JPL query executes in the full gate, generated artifacts and generated artifact hashes remain zero, manifest acceptance and runtime behavior are unchanged, and `astronomy-engine` remains target.

`data/generated/astronomy/selected-gb-t-payload-materialization-preflight.json` and `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1` define the LOOP-053 selected GB/T rule-reference payload preflight. It keeps `gb-t-33661-2017-rule-reference.json` absent, source-reference capture false for this loop, payload materialization false for this loop, new source payload writes 0, new source hashes 0, external full-gate calls false, generated artifacts 0, manifest acceptance unchanged, runtime behavior unchanged, Android baseline unchanged, and `astronomy-engine` target.

`data/generated/astronomy/selected-gb-t-payload-materialization.json`, `data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json`, and updated source payload dry-runs define the LOOP-054 selected GB/T rule-reference boundary materialization state. Exactly four source-boundary payloads now exist: `naif-cspice`, `iau-sofa-ansi-c`, `jpl-horizons-api`, and `gb-t-33661-2017`; GB/T standard text is not copied, Chinese-calendar rules are not implemented, generated artifacts and generated artifact hashes remain zero, manifest acceptance and runtime behavior are unchanged, Android baseline is unchanged, and `astronomy-engine` remains target.
