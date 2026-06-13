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
        timeline.rs
        topic_report.rs
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
        topic_report.rs
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
| `data/generated/astronomy/*` | generated astronomy governance/data | 包含历史 preflight 治理件、source-boundary payload、computed astronomy outputs 和 M23 supported 证据；Android 日期层运行时替换仍需单独 ADR。 |
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

## Current Restricted Topic Report Surface

`GET /api/charts/topic-report` is the M29 shared topic-report route. It requires explicit `topic` and `year`, returns qualitative `signals`/`trace`, disclaimer, warnings, and forbidden-output audit, and excludes public `score_internal` or 0-100 fate scores. After M33, `topic=relationship`, `topic=wealth`, `topic=family`, and `topic=career` are implemented as restricted topic reports and exposed through the 2 x 2 frontend entry and `/api/capabilities`. LOOP-096 adds基础解释 anchors to the topic reports; LOOP-109 later changes `relationship-report` into a six-block narrative (`总断`、`伴侣议题`、`夫妻宫`、`表达、边界与安全感`、`年度情感引动`、`结论`) and folds topic timeline evidence into the annual emotional-trigger block. LOOP-110 adds a relationship-only real-output copy gate that rejects 标记/筛选/提取 wording, equality-count phrasing, and potential internal English leakage. LOOP-111 extends real-output copy gates to wealth/family/career assembled reports and keeps the M42 relationship baseline intact. LOOP-112 adds a relationship second-pass gate that prevents fixed opener repetition and requires relation terms such as `冲` / `六冲` to be quoted in explanatory report prose. LOOP-113 extends the final body gate to system/algorithm/score/count-table tone and readable major-luck labels. LOOP-114 extends the same gate to list/table/debug-like narration such as `盘中可用的时间线索`, `关键牵动是`, and `出现 4 处`. LOOP-115 raises relationship-report as the current golden sample by replacing spouse-star/expression/support count fields such as `不作主线` and `有一处落点` with relationship-quality prose. LOOP-116 extends that count-field baseline to wealth/family/career by replacing ten-god count fields and middle-layer bridges such as `参与这组结构` with topic-quality prose. LOOP-117 rewrites main annual-trigger and topic timeline sections from list evidence into reading-order narrative prose. LOOP-118 rewrites wealth/family/career openings, topic-entry chapters, annual guidance, and conclusions into advice-cohesion prose while gating `日常读法`, `日常看`, and `这些牵动提醒您` regressions. LOOP-119 rewrites the main chart report teaching bridges, ten-god count summary, and annual-trigger stale wording into direct reading prose while gating `这一章看的是`, `这一章先把`, `放到日常理解里`, `最适合当作`, and `可以先这样理解` regressions. LOOP-120 keeps report closeout continuous by placing wealth/family/career timeline overlays before `结论`, making topic conclusions the final visible chapter, and gating stale phrases such as `基本脉络如下`, `第一优先`, `不能只看流年`, `当前资料可以按完整四柱合参`, and `这一年`. LOOP-121 groups the main five-element explanation and makes wealth/family/career timeline guidance topic-specific through `落到2026年` prose, while gating `偏弱表示这类倾向`, `哪里需要放慢`, `哪里需要承接`, and `读2026年这一层`. LOOP-122 warms the main annual-trigger and wealth/family/career topic timeline details into annual-rhythm prose through `2026年靠近命盘时` and `把2026年放进...专项来看`, while gating `这一章会把`, `牵动会先落在这些位置`, `2026年的时间气候`, `先从这些层次落下去看`, `先看天干`, and `再看五行关系`. LOOP-123 makes topic reports use real luck-cycle context and selected-year current luck while gating `大运首段`, fixed `1至10岁` wording, and stale annual-detail phrases such as `天干处先露出`. LOOP-124 compresses relationship conclusion repetition and makes wealth/family/career closeouts topic-specific while gating `在这份...专项里` conclusion templates. LOOP-125 de-stages main annual-trigger and wealth/family/career topic timeline prose while gating `年度本身先露出的`, `推到台前`, and `不是罗列符号` regressions. LOOP-126 condenses main chart `十神关系`, `大运走势`, and `年度引动` visible prose while retaining restricted trace/evidence and gating `读这一章时`, `这条线已经进入命盘视野`, `这条十神线索`, and professional-label regressions. LOOP-097 keeps the workspace topic panel as a structure-signal summary only, and moves the full restricted reading flow to `topic-report.html?topic=...`. LOOP-108 gates final topic-report response bodies against stiff labels, generic-year wording, internal engineering copy, public scores, and machine-like phrasing.

LOOP-127 extends the same topic-report quality line to wealth/family/career middle chapters: visible report prose now frames resources, support, responsibility, delivery, and collaboration as topic-specific reading rhythm instead of textbook definitions, with guards against `财星分正财和偏财`, `传统上会把`, `官杀代表责任`, `技能表达：`, and `同辈边界：` regressions. No route, DTO, capability status, public score, or raw `luck-cycles` behavior changed.

LOOP-128 extends the same quality-only line to wealth/family/career `本专题的大运流年`: visible timeline prose now opens with direct 2026 topic rhythm instead of engine-layer scaffolding, with guards against `从「金钱」专项来看`, `从「家庭」专项来看`, `从「事业」专项来看`, `把2026年放进`, `十神与五行这一层`, and `本段把它作为阶段背景参考` regressions. No route, DTO, capability status, public score, or raw `luck-cycles` behavior changed.

LOOP-129 extends the quality-only line to age-context report polish: the 2025/2026 early-stage sample now reads relationship as emotional response and attachment safety, wealth as resource sense and sharing boundaries, family as stable care and response, and career as learning tasks, rule sense, expression training, and support systems. Adult-context regressions such as `如果目前单身`, `若已有关系`, `工作场景`, `现实职位高低`, `长期经营`, `现实回报`, and `团队边界` are gated for that early-stage sample. No route, DTO, capability status, public score, or raw `luck-cycles` behavior changed.

`frontend/index.html`, `frontend/report.html`, `frontend/topic-report.html`, `frontend/src/api/client.js`, `frontend/src/main.js`, `frontend/src/topic-report-page.js`, `frontend/src/ui/dom.js`, `frontend/src/ui/render.js`, and `frontend/src/styles.css` provide the post-preview 2 x 2 topic entry, left-bottom `查看专项报告` navigation, workspace structure-signal panel, and chart-report-style full topic report page. After M33, relationship, wealth, family, and career buttons are all enabled as restricted entries and all requests go through the explicit-year topic-report API. LOOP-103 adds M39 timeline report UI: main/topic report time guides, explicit year controls, expandable evidence, and short workbench summaries. LOOP-104 adds M40 frontend source gates for no public score, no overclaimed flow-month/day fortune, and explicit boundary wording. LOOP-105 clears stale topic output on chart recalculation and localizes visible internal English markers in the workbench and report pages. LOOP-106 removes engineering copy and hard professional/plain labels from public timeline evidence UI. LOOP-107 changes backend dictionary/readings only; frontend surfaces keep the same restricted payload shape. LOOP-108 keeps frontend copy aligned with daily-language wording while the backend app-layer gate checks the final public report bodies. LOOP-109 makes the relationship full report narrative-only, so `signals`/`trace` remain available for governance and time guide evidence but are not rendered as extra relationship body chapters.

## Restricted Primary Luck Reading Surface

LOOP-098 lays down M34-M40 for the next heavy engineering slice: primary chart luck reading, annual trigger reading, and topic timeline overlays. LOOP-099 closes DG-012 through ADR 0022 and implements the M35 internal `domain::timeline` foundation. LOOP-100 completes M36 by exposing only primary chart `luck-reading` as a restricted report-carried surface on `GET /api/charts/report?reading_year=YYYY`. LOOP-101 completes M37 by exposing `annual-trigger-reading` as a restricted report-carried surface on `GET /api/charts/report?year=YYYY`. LOOP-102 completes M38 by exposing `topic-timeline-reading` as a restricted topic-report-carried surface on `GET /api/charts/topic-report?topic=...&year=YYYY`. LOOP-103 completes M39 as UI/readability only, with no capability change. LOOP-104 completes M40 as quality-gate closeout only: public golden samples, forbidden/no-score/no-overclaim checks, bounded-output protection, browser samples, and governance sync. LOOP-105 is frontend quality-only: no capability change, stale topic panels clear after chart recalculation, and visible internal English markers are localized. LOOP-106 is lexicon-copy quality-only: generated timeline readings must use natural reader-facing Chinese and avoid stiff labels, generic year wording, backend/frontend copy, and internal engine ids. LOOP-107 is large-scale dictionary quality-only: `timeline-lexicon` expands to 28 compositional entries, generated readings use richer professional/plain guidance, and tests guard lexicon text, copy density, and internal-id leakage. LOOP-108 is report-level quality-only: final chart report and four topic report response bodies must pass visible-copy gates before closeout. LOOP-109 is relationship-report narrative quality-only, LOOP-110 is relationship-report real-output copy quality-only, LOOP-111 is remaining-report real-output copy quality-only, LOOP-112 is relationship-report copy second-pass quality-only, LOOP-113 is five-report system-tone cleanup quality-only, LOOP-114 is five-report narrative-list cleanup quality-only, LOOP-115 is relationship-report golden-sample quality-only, LOOP-116 is remaining-topic count-field narrative quality-only, LOOP-117 is annual timeline narrative quality-only, LOOP-118 is wealth/family/career advice-cohesion quality-only, LOOP-119 is main-report tone-cohesion quality-only, LOOP-120 is report closeout-continuity quality-only, LOOP-121 is report density/topic-specificity quality-only, LOOP-122 is timeline-detail narrative-warmth quality-only, LOOP-123 is current-luck consistency quality-only, and LOOP-124 is conclusion de-duplication quality-only: none changes route, DTO top-level, `/api/capabilities`, raw luck route, score, or supported status. Raw `GET /api/luck/cycles` remains a supported calculation route without reading text, score, annual trigger, or topic overlay fields.

| Capability | Target Milestone | Status | Boundary |
| --- | --- | --- | --- |
| `luck-reading` | M36 | restricted | Consumes M13 raw luck cycles through chart-report carrier; explains current major-luck stage with trace, professional/plain readings, warning downgrade, and forbidden-claim audit |
| `annual-trigger-reading` | M37 | restricted | Requires explicit `year`; explains original-chart and current-luck annual triggers without flow-month/day or event prediction claims |
| `topic-timeline-reading` | M38 | restricted | Maps shared timeline signals into relationship, wealth, family, and career report language while preserving topic safety boundaries |

Module tree:

```
backend.timeline-reading (internal foundation)
  timeline-core (implemented in backend/src/domain/timeline.rs)
  timeline-lexicon (implemented in backend/src/domain/timeline.rs; 28 compositional entries after LOOP-107)
  primary-luck-reading (M36 restricted via /api/charts/report)
  annual-trigger-reading (M37 restricted via /api/charts/report)
  topic-timeline-overlay (M38 restricted via /api/charts/topic-report)
frontend.timeline-report-ui
  workbench short major-luck summary (M36)
  chart report major-luck chapter (M36)
  chart report annual-trigger chapter (M37)
  topic report timeline chapter (M38; relationship folded into annual emotional-trigger chapter after M41)
governance.timeline-quality-gate
  DG-012 (closed by ADR 0022)
  golden samples (closed by LOOP-104)
  forbidden output suite (closed by LOOP-104)
  no public score checks (closed by LOOP-104)
  bounded-output checks (closed by LOOP-104)
```

No `score_internal`, 0-100 fortune score, flow-month, flow-day, daily push, date selection, event prediction, financial advice, relationship advice, family fate claim, or career-result guarantee may enter public API or UI through this restricted surface.

## Current Release Candidate Surface

`docs/release/v1-release-candidate.md` and `docs/release/v1-closeout.md` record the V1 preview release boundary. They freeze the final 10 supported and 7 restricted capabilities after M0-M28, while preserving historical M8/M9/M10 preflight evidence as stage-specific governance records. `tools/check-release-candidate.ps1` is included in `tools/check-project.ps1` and verifies closeout artifacts, frozen capability statuses, README route boundaries, frontend overclaim tests, and share privacy test evidence.

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
