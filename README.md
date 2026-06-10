# 命轨

命轨当前采用 Rust 后端、JavaScript 前端，并接入 `heavy-scale-exploitation-governance-1` 作为项目治理体系。

## Project Layout

| Path | Responsibility |
| --- | --- |
| `backend/` | Rust API service, HTTP skeleton, domain models, and lunar-data capability source |
| `frontend/` | JavaScript browser UI, API client, state, render modules, and frontend tests |
| `data/raw/` | Raw lunar-calendar source data |
| `markdown/` | Governance, product tree, policies, and closeout templates |
| `tools/` | Project checks, governance inventory, and scaffold checks |
| `docs/decisions/` | Architecture decision records |
| `docs/release/` | Release candidate notes and frozen delivery boundaries |
| `data/generated/astronomy/` | M9/M10 astronomy source policy, draft generation plan, not-accepted manifest draft, source capture procedure, comparison/golden/replay plans, preflight closeout decision, generated-data manifest schema, and comparison-report template |

## Research Intake

Returned research reports are stored under `markdown/reserch/`. English reports have Chinese translations under `markdown/reserch/zh-CN/`, and governance handling is tracked in `markdown/reserch/00-research-intake.md`.

Research-derived targets are recorded in:

- `docs/decisions/0003-v1-research-governance-baseline.md`
- `docs/decisions/0004-v1-calculation-ruleset-target.md`
- `docs/decisions/0005-privacy-safe-interpretation-target.md`

## Roadmap

End-to-end development milestones are under `markdown/20-roadmap/`.

M0-M24 closed. V1 capability matrix: 10 supported, 7 restricted, 0 target, 0 planned. 边界已锁定，不再受理功能性新增需求。

Start with:

- `markdown/20-roadmap/00-roadmap-index.md`
- `markdown/20-roadmap/90-decision-gates.md`
- `markdown/20-roadmap/91-anti-regression-and-governance-lock.md`
- `markdown/20-roadmap/93-capability-promotion-ledger.md`
- `markdown/20-roadmap/95-recursive-development-protocol.md`
- `markdown/20-roadmap/96-recursive-cursor.md`
- `markdown/20-roadmap/97-loop-closeout-log.md`

Code implementation should follow the active milestone and must not promote a capability to supported before Rust/API/test/capability evidence is complete.

Recursive development starts in `design_only` mode. Until the user confirms implementation, the recursive cursor forbids business code, API behavior, frontend feature changes, and capability promotion.

## Release Candidate

The V1 release candidate is recorded in `docs/release/v1-release-candidate.md`. It freezes the current supported/restricted/planned capability boundary after M8 validation. `release-candidate` is a governance and delivery surface; it does not add a new backend business API.

## Run

```powershell
cargo run -p minggui-backend
```

```powershell
cd frontend
node server.mjs
```

Open `http://127.0.0.1:5173`.

The frontend runs as a restricted workspace with chart creation, structured analysis (incl. deep-analysis cards), luck cycles, local volatile case save/list, redacted share preview, date-layer query, glossary, data derivation, capability boundary display, and JSON case export. It does not implement backend algorithms locally and does not claim durable sharing, accounts, cloud sync, true solar time, timezone history, wider date range, or astronomy replacement.

## Supported API

```text
GET /api/health                                    — supported
GET /api/capabilities                              — supported
GET /api/lunar-data/meta                           — supported
GET /api/calendar/query?date=YYYY-MM-DD             — supported (Android date-layer, 1901-2100)
GET /api/charts/basis/preview?date=&timezone=       — restricted
GET /api/charts?date=&timezone=&time_precision=     — supported (4 pillars)
GET /api/charts/detail?date=&timezone=              — supported (immutable snapshot)
GET /api/analysis/snapshot?date=&timezone=          — supported (structured metrics + deep analysis)
GET /api/luck/cycles?date=&timezone=&sex=           — supported (ADR 0020)
GET /api/glossary?term=&category=                   — supported (55 entries)
GET /api/cases?action=list|create|detail|update_metadata|archive|delete  — restricted
GET /api/cases/export?id=                           — restricted
GET /api/settings?action=get|update                 — restricted
GET /api/share/preview?action=create|public|revoke  — restricted
GET /api/data/derive?type=                          — restricted (>=5 threshold)
GET /api/charts/report?date=&timezone=               — restricted (colloquial CN report, 9 blocks)
```

The first date-layer implementation is ported from the Android perpetual-calendar source under `D:\myproject\Perpetual calendar`, with golden cases covering leap days, leap months, CNY boundaries, cross-year continuity, and solar-term dates.

The V1 official validated date-layer range is `1901-2100` per ADR 0008. Dates outside that range are unsupported for the current Android date-layer API and are reserved for a later astronomy or ephemeris-backed upgrade.

`GET /api/calendar/query` returns a `meta` object that records the current date-layer source, algorithm version, ruleset id, supported range, boundary policy, and known limitations. This metadata is traceability for the date layer only; it does not promote full four-pillar chart creation, hour-pillar calculation, timezone history, true solar time, or the future astronomy engine.

`/api/capabilities` declares both `calendar-date-query` and `calendar-date-query-v1-meta` as supported date-layer capabilities. They share the same query route; the `v1-meta` capability only means the response carries traceable date-layer metadata.

Date-query errors use the standard JSON envelope `{"error": "...", "message": "..."}`. Missing or invalid `date` returns `bad_request`/400, supported-table misses return `out_of_range`/404, and missing lunar source data returns `io_error`/500.

The frontend includes a date-layer probe that calls `GET /api/calendar/query` and displays Gregorian, lunar, Gan-Zhi, and ruleset metadata. It is a supported date-layer surface only, not full chart UI.

`GET /api/charts/basis/preview` is a restricted M2 contract route. It returns `ft-v1-default` calculation metadata, the normalized birth-profile/chart-request basis, the official 1901-2100 validation range, and explicit unsupported outputs. It does not calculate full four pillars, hour pillar, IANA timezone history, true solar time, lunar input conversion, or persisted charts.

`GET /api/charts` is the M3 chart-create route. It returns year/month/day pillars from the accepted Android date layer, exact-time hour pillar when `time_precision=exact`, and `hour:null` with all hour candidates when `time_precision=unknown`. It records warnings and unsupported outputs; IANA timezone history, true solar time, lunar input conversion, persisted charts, analysis, and luck cycles remain unsupported.

`GET /api/analysis/snapshot` is the M4 structured analysis route. It returns deterministic five-element, ten-god, hidden-stem, relation, sensitivity, and fixed-card data with a disclaimer id and forbidden-output audit. It does not return generated prose, luck cycles, medical/legal/financial/death/fertility/relationship certainty claims, storage, sharing, true solar time, timezone history, or astronomy replacement.

`GET /api/cases` is the M5 restricted local case-management route. It uses in-process volatile storage only, creates immutable chart and analysis snapshot references with algorithm versions, supports detail/list/metadata update/archive/delete actions, and omits private notes from list responses. It does not provide database persistence, accounts, cloud sync, cross-device sync, public sharing, share tokens, luck cycles, or generated analysis.

`GET /api/settings` is the M5 restricted local preference route. It stores local volatile preferences for default calendar, privacy default, language, and theme. It does not provide account-level settings, cloud sync, or cross-device preference persistence.

`GET /api/share/preview` is the M6 restricted local share-preview route. It creates local volatile share records from immutable case snapshots, returns the raw token only at creation time, stores only a token hash, supports public redacted DTO reads and revocation, and marks public DTOs as `noindex` and non-editable. Public DTOs omit private notes, raw titles, tags, private case ids, exact birth-time/location fields, and snapshot ids. Missing, expired, invalid, and revoked tokens use the same unavailable response.

## Checks

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

The full check runs:

```powershell
cargo fmt --check
cargo test
cd frontend
npm.cmd run check
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-governance-scaffold.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-release-candidate.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-astronomy-preflight.ps1 -ProjectRoot .
```

M9 source availability can be probed manually without making the full local gate depend on external network availability:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\probe-astronomy-sources.ps1 -ProjectRoot .
```

M9 astronomy generation currently supports dry-run inspection only:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\generate-astronomy-tables.ps1 -ProjectRoot . -Manifest data/generated/astronomy/manifests/astronomy-engine-v0-draft.json -DryRun
```

M9 comparison currently supports dry-run shape inspection only:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\compare-astronomy-dry-run.ps1 -ProjectRoot . -Manifest data/generated/astronomy/manifests/astronomy-engine-v0-draft.json
```

M9 golden-case planning currently supports dry-run inspection only:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\golden-cases-dry-run.ps1 -ProjectRoot .
```

M9 replay policy currently supports dry-run inspection only:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\replay-policy-dry-run.ps1 -ProjectRoot . -Manifest data/generated/astronomy/manifests/astronomy-engine-v0-draft.json
```

M9 pre-closeout audit is recorded at `data/generated/astronomy/precloseout-audit.json` and `markdown/20-roadmap/36-milestone-09-pre-closeout-audit.md`. It explicitly blocks full astronomy-engine closeout while allowing only preflight closeout review.

M9 generated-data implementation planning is recorded at `data/generated/astronomy/implementation-plan.json`, `docs/decisions/0017-m9-generated-data-implementation-path.md`, and `markdown/20-roadmap/37-milestone-09-generated-data-implementation-plan.md`. It keeps `astronomy-engine` as target and selects generator contract planning as the next step.

M9 generator contract planning is recorded at `data/generated/astronomy/generator-contract.json` and `markdown/20-roadmap/38-milestone-09-generator-contract.md`. It defines inputs, planned outputs, `sha256` hash policy, and manifest update rules without generating rows.

M10 generator implementation entry is recorded at `data/generated/astronomy/generator-implementation-entry.json` and `markdown/20-roadmap/46-milestone-10-generator-entry.md`. It exposes `tools/generate-astronomy-tables.ps1 -PrepareImplementation` as a guarded non-dry-run entry shape while still writing no artifacts, computing no hashes, changing no manifest status, and keeping `astronomy-engine` as target.

M9 source adapter contract planning is recorded at `data/generated/astronomy/source-adapter-contract.json` and `markdown/20-roadmap/39-milestone-09-source-adapter-contract.md`. It maps selected sources into future reproducible input boundaries without runtime dependency or external-gate calls.

M10 source snapshot manifest materialization is recorded at `data/generated/astronomy/source-snapshot-manifest.schema.json`, `data/generated/astronomy/source-snapshot-manifest-plan.json`, `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`, `tools/source-snapshot-manifest-dry-run.ps1`, `markdown/20-roadmap/47-milestone-10-source-snapshot-boundary.md`, and `markdown/20-roadmap/48-milestone-10-source-snapshot-manifest.md`. It records selected-source provenance as metadata only while keeping source payload files, generated astronomy artifacts, hashes, manifest acceptance, runtime replacement, and `astronomy-engine` promotion absent.

M10 source payload materialization policy is recorded at `data/generated/astronomy/source-payload-materialization-policy.json`, `tools/source-payload-materialization-dry-run.ps1`, and `markdown/20-roadmap/49-milestone-10-source-payload-policy.md`. It defines future per-source payload paths while keeping the payload directory, payload files, source payload hashes, generated astronomy artifacts, generated artifact hashes, manifest acceptance, runtime replacement, and `astronomy-engine` promotion absent.

M10 source payload schemas are recorded under `data/generated/astronomy/source-payload-schemas/` and `markdown/20-roadmap/50-milestone-10-source-payload-schemas.md`. They define the future source payload shapes while keeping payload files, hashes, generated astronomy artifacts, manifest acceptance, runtime replacement, and `astronomy-engine` promotion absent.

M10 source capture procedure is recorded at `data/generated/astronomy/source-capture-procedure.json`, `tools/source-capture-procedure-dry-run.ps1`, and `markdown/20-roadmap/51-milestone-10-source-capture-procedure.md`. It defines per-source capture steps and first-payload preconditions while keeping payload files, source hashes, external full-gate calls, generated astronomy artifacts, manifest acceptance, runtime replacement, and `astronomy-engine` promotion absent.

M10 first source payload materialization decision is recorded at `data/generated/astronomy/source-payload-materialization-decision.json`, `tools/source-payload-materialization-decision-dry-run.ps1`, and `markdown/20-roadmap/52-milestone-10-first-source-payload-decision.md`. It selects `naif-cspice` as the first single-source payload candidate while keeping the payload directory, payload files, source hashes, external full-gate calls, generated astronomy artifacts, manifest acceptance, runtime replacement, and `astronomy-engine` promotion absent.

M10 selected source payload materialization preflight is recorded at `data/generated/astronomy/selected-source-payload-materialization-preflight.json`, `tools/selected-source-payload-materialization-preflight-dry-run.ps1`, and `markdown/20-roadmap/53-milestone-10-selected-source-payload-preflight.md`. It permits the next loop to focus on selected-source-only `naif-cspice` payload materialization while keeping the payload directory, selected payload, source hashes, external full-gate calls, generated astronomy artifacts, manifest acceptance, runtime replacement, and `astronomy-engine` promotion absent in LOOP-045.

M10 selected source payload materialization is recorded at `data/generated/astronomy/selected-source-payload-materialization.json`, `data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json`, `tools/source-payload-materialization-dry-run.ps1`, `tools/source-capture-procedure-dry-run.ps1`, `tools/source-payload-materialization-decision-dry-run.ps1`, `tools/selected-source-payload-materialization-preflight-dry-run.ps1`, and `markdown/20-roadmap/54-milestone-10-selected-source-payload-materialization.md`. It materializes exactly one `naif-cspice` source-boundary payload with sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`; it does not materialize SPICE kernels or CSPICE tooling, does not write generated astronomy artifacts, does not compute generated artifact hashes, does not change manifest acceptance or runtime behavior, and does not promote `astronomy-engine`.

M10 remaining source payload strategy is recorded at `data/generated/astronomy/remaining-source-payload-strategy.json`, `tools/remaining-source-payload-strategy-dry-run.ps1`, and `markdown/20-roadmap/55-milestone-10-remaining-source-payload-strategy.md`. It selects `iau-sofa-ansi-c` as the next preflight-only source candidate, then JPL Horizons, then GB/T; it writes no new payloads, computes no new source hashes, performs no external full-gate calls, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, and keeps `astronomy-engine` target.

M10 selected IAU SOFA payload materialization preflight is recorded at `data/generated/astronomy/selected-iau-sofa-payload-materialization-preflight.json`, `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1`, and `markdown/20-roadmap/56-milestone-10-selected-iau-sofa-payload-preflight.md`. It scopes the next loop to selected-source-only `iau-sofa-ansi-c` local routine/version boundary materialization while keeping the SOFA payload absent in LOOP-048; it writes no new payloads, computes no new source hashes, performs no external full-gate calls, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, and keeps `astronomy-engine` target.

M10 selected IAU SOFA payload materialization is recorded at `data/generated/astronomy/selected-iau-sofa-payload-materialization.json`, `data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json`, `tools/source-payload-materialization-dry-run.ps1`, `tools/source-capture-procedure-dry-run.ps1`, `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1`, and `markdown/20-roadmap/57-milestone-10-selected-iau-sofa-payload-materialization.md`. It materializes exactly one IAU SOFA routine/version boundary payload with sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f`; it does not vendor, compile, link, or integrate SOFA routines, does not write generated astronomy artifacts, does not compute generated artifact hashes, does not change manifest acceptance or runtime behavior, and does not promote `astronomy-engine`.

M10 post-IAU remaining source payload strategy is recorded at `data/generated/astronomy/post-iau-remaining-source-payload-strategy.json`, `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1`, and `markdown/20-roadmap/58-milestone-10-post-iau-remaining-source-payload-strategy.md`. It selects JPL Horizons as the next selected-source-only preflight candidate and leaves GB/T for the following governed scope; it writes no JPL or GB/T payloads, computes no new source hashes, performs no external full-gate calls, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, and keeps `astronomy-engine` target.

M10 selected JPL Horizons payload materialization preflight is recorded at `data/generated/astronomy/selected-jpl-horizons-payload-materialization-preflight.json`, `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1`, and `markdown/20-roadmap/59-milestone-10-selected-jpl-horizons-payload-preflight.md`. It scopes the next loop to selected-source-only JPL validation-query snapshot materialization while keeping the JPL payload absent in LOOP-051; it performs no online JPL query in the full gate, writes no JPL or GB/T payloads, computes no new source hashes, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, and keeps `astronomy-engine` target.

M10 selected JPL Horizons payload materialization is recorded at `data/generated/astronomy/selected-jpl-horizons-payload-materialization.json`, `data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json`, `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1`, and `markdown/20-roadmap/60-milestone-10-selected-jpl-horizons-payload-materialization.md`. It materializes exactly one offline validation-query snapshot boundary payload with sha256 `acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9`; it includes no JPL response bodies, executes no online JPL query in the full gate, writes no generated astronomy artifacts, computes no generated artifact hashes, changes no manifest acceptance or runtime behavior, replaces no Android baseline, and does not promote `astronomy-engine`.

M10 selected GB/T payload materialization preflight is recorded at `data/generated/astronomy/selected-gb-t-payload-materialization-preflight.json`, `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1`, and `markdown/20-roadmap/61-milestone-10-selected-gb-t-payload-preflight.md`. It scopes the next loop to selected-source-only GB/T 33661-2017 rule-reference payload materialization while keeping the GB/T payload absent in LOOP-053; it captures no rule text in the full gate, computes no GB/T source hash, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, replaces no Android baseline, and keeps `astronomy-engine` target.

M10 selected GB/T payload materialization is recorded at `data/generated/astronomy/selected-gb-t-payload-materialization.json`, `data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json`, and `markdown/20-roadmap/62-milestone-10-selected-gb-t-payload-materialization.md`. It materializes only the GB/T rule-reference boundary with sha256 `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31`; it copies no standard text, implements no calendar algorithm, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, replaces no Android baseline, and keeps `astronomy-engine` target.

M10 generated astronomy artifact materialization preflight is recorded at `data/generated/astronomy/generated-artifact-materialization-preflight.json`, `tools/generated-artifact-materialization-preflight-dry-run.ps1`, and `markdown/20-roadmap/63-milestone-10-generated-artifact-materialization-preflight.md`. It defines the write boundary for the 4 planned generated astronomy artifacts while keeping the output directory, artifact files, artifact hashes, manifest acceptance, runtime behavior, Android baseline, and `astronomy-engine` promotion forbidden in LOOP-055.

M10 generated astronomy artifact materialization is recorded at `data/generated/astronomy/generated-artifact-materialization.json`, `data/generated/astronomy/out/solar-terms-1901-2100.json`, `data/generated/astronomy/out/new-moons-1901-2100.json`, `data/generated/astronomy/out/lunar-calendar-1901-2100.json`, `data/generated/astronomy/out/android-comparison-1901-2100.json`, and `markdown/20-roadmap/64-milestone-10-generated-artifact-materialization.md`. It materializes 4 generated astronomy artifacts as boundary placeholders with sha256 hashes `81459770...`, `d1dd3a7c...`, `49757871...`, and `c4f7628f...`; all artifacts have `generation_status: boundary_placeholder` with empty entry arrays; no astronomical computation has been performed, no manifest acceptance changed, no runtime behavior changed, no Android baseline replaced, and `astronomy-engine` remains target.

M9 artifact writer dry-run planning is recorded at `data/generated/astronomy/artifact-writer-plan.json`, `tools/artifact-writer-dry-run.ps1`, and `markdown/20-roadmap/40-milestone-09-artifact-writer-dry-run.md`. It previews output paths and `sha256` hash policy without creating directories, writing files, or accepting artifacts.

M9 comparison runner dry-run planning is recorded at `data/generated/astronomy/comparison-runner-plan.json` and `markdown/20-roadmap/41-milestone-09-comparison-runner-dry-run.md`. It binds Android baseline metadata to the future comparison artifact while keeping rows and differences at zero.

M9 golden-row readiness planning is recorded at `data/generated/astronomy/golden-row-readiness-plan.json` and `markdown/20-roadmap/42-milestone-09-golden-row-readiness.md`. It keeps all required golden categories blocked and not generated until generated rows and comparison evidence exist.

M9 replay-test readiness planning is recorded at `data/generated/astronomy/replay-test-readiness-plan.json` and `markdown/20-roadmap/43-milestone-09-replay-test-readiness.md`. It keeps replay tests unexecuted while binding old snapshot replay requirements to `android-date-layer-v1`.

M9 preflight closeout is recorded at `data/generated/astronomy/preflight-closeout-decision.json` and `markdown/20-roadmap/44-milestone-09-preflight-closeout.md`. It closes M9 only as a preflight milestone, keeps `astronomy-engine` as target, and routes real generated astronomy implementation to M10 without changing runtime behavior.

M10 closeout is recorded at `markdown/20-roadmap/66-milestone-10-closeout.md`. M10 delivers: 4 source boundary payloads materialized with sha256; 4 generated astronomy artifacts as boundary placeholders with sha256; draft manifest updated with artifact metadata; generator implementation entry guarded; comparison/golden/replay plans updated. Real astronomical data generation, comparison, golden rows, and replay tests remain for a subsequent milestone pending astronomy computation engine implementation.
