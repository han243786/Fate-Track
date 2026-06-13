# Data Sources

## Lunar Calendar Raw Data

| Field | Value |
| --- | --- |
| Project copy | `data/raw/lunar_data.yaml` |
| Original source path | `D:\myproject\Perpetual calendar\data\yaml\lunar_data.yaml` |
| Declared range | 1901-2100 |
| Format | YAML skip table |
| Purpose | Gregorian to lunar lookup, Gan-Zhi year, zodiac, lunar month lengths, solar-term offsets |
| Lifecycle | Raw source copy; do not mutate in application code |

The raw table is the current source of truth for the first backend lunar-calendar capability. Any generated Rust, JSON, database, or cache artifact derived from this file must record its source file, generation command, and validation evidence.

ADR 0008 closes the V1 official validated date-layer range as `1901-2100`. Wider validation must be introduced as a later astronomy or ephemeris-backed engine, not as a silent expansion of this raw Android baseline.

## Android Date Layer Reference

The current Rust date-layer port follows:

- `D:\myproject\Perpetual calendar\app\src\main\java\com\perpetualcalendar\app\lunar\LunarCalendar.java`
- `D:\myproject\Perpetual calendar\app\src\main\java\com\perpetualcalendar\app\lunar\GanZhi.java`
- `D:\myproject\Perpetual calendar\data\edge_case_test.txt`

These references protect year, month, and day Gan-Zhi consistency until a later research report provides a stronger replacement.

The Rust regression suite now embeds all 49 cases from `D:\myproject\Perpetual calendar\data\edge_case_test.txt`, covering leap days, leap months, year-boundary continuity, pre-CNY dates, CNY dates, extreme CNY windows, and explicit Xiaohan solar-term samples. The suite asserts lunar month/day numbers, leap-month flags, year/month/day Gan-Zhi, and selected solar terms.

## Current Date-Layer Metadata

`GET /api/calendar/query?date=YYYY-MM-DD` exposes the active date-layer metadata in the response:

| Field | Current Value |
| --- | --- |
| `source` | `android-date-layer-port` |
| `algorithm_version` | `android-date-layer-v1` |
| `ruleset_id` | `ft-date-layer-android-v1` |
| `support_range` | 1901-2100 |
| `boundary_policy` | `date-only-gregorian-query-no-timezone` |

This metadata describes the date-only Android baseline. It must not be used to claim hour-pillar, timezone-history, true-solar-time, or full chart support.

## Astronomy Generated-Data Governance And Artifacts

`data/generated/astronomy/` contains historical M9/M10 governance preflight artifacts plus later generated astronomy outputs. Historical preflight documents keep their original anti-overclaim language; the current preview release status is defined by `docs/release/v1-release-candidate.md`, `docs/release/v1-closeout.md`, ADR 0021, and `93-capability-promotion-ledger.md`.

The directory includes:

- `source-policy.json`
- `generation-plan.json`
- `implementation-plan.json`
- `generator-contract.json`
- `generator-implementation-entry.json`
- `source-adapter-contract.json`
- `source-snapshot-manifest.schema.json`
- `source-snapshot-manifest-plan.json`
- `source-snapshots/source-snapshot-manifest.json`
- `source-payload-materialization-policy.json`
- `source-payload-schemas/*.schema.json`
- `source-capture-procedure.json`
- `source-payload-materialization-decision.json`
- `selected-source-payload-materialization-preflight.json`
- `selected-source-payload-materialization.json`
- `source-snapshots/payloads/naif-cspice-kernel-boundary.json`
- `remaining-source-payload-strategy.json`
- `post-iau-remaining-source-payload-strategy.json`
- `selected-jpl-horizons-payload-materialization-preflight.json`
- `selected-jpl-horizons-payload-materialization.json`
- `source-snapshots/payloads/jpl-horizons-validation-samples.json`
- `selected-gb-t-payload-materialization-preflight.json`
- `selected-gb-t-payload-materialization.json`
- `selected-iau-sofa-payload-materialization-preflight.json`
- `selected-iau-sofa-payload-materialization.json`
- `source-snapshots/payloads/iau-sofa-routine-version.json`
- `artifact-writer-plan.json`
- `comparison-runner-plan.json`
- `comparison.schema.json`
- `golden-cases-plan.json`
- `golden-row-readiness-plan.json`
- `replay-policy-draft.md`
- `replay-test-readiness-plan.json`
- `precloseout-audit.json`
- `preflight-closeout-decision.json`
- `manifest.schema.json`
- `manifests/astronomy-engine-v0-draft.json`: not-accepted planning manifest instance.
- `comparison-report-template.md`
- `README.md`
- `out/solar-terms-1901-2100.json`
- `out/new-moons-1901-2100.json`
- `out/lunar-calendar-1901-2100.json`
- `out/android-comparison-1901-2100.json`

M23 promotes `astronomy-engine` to supported as an independent verified engine. The Android date layer remains the accepted runtime baseline for `/api/calendar/query`; any replacement still requires a separate ADR and rollback plan.

The draft manifest must remain `not_accepted` until a real generation command, generated artifact, artifact hashes, comparison report, golden cases, and runtime/replay policy exist.

The M9 generator, comparison, golden-case, and replay-policy scripts are dry-run-only scaffolds. They must not write generated files, execute replay tests, allow replacement, produce accepted rows, or change `astronomy-engine` status.

`precloseout-audit.json` records that full M9 astronomy-engine closeout is blocked while preflight is ready. It must not be used as generated-data acceptance evidence.

`preflight-closeout-decision.json` records that M9 is closed only as a preflight milestone and that real generated astronomy implementation moves to M10. It must not be used as generated-data acceptance, runtime replacement, wider-range, true-solar-time, timezone-history, or `astronomy-engine` support evidence.

`implementation-plan.json` records the planning-only path toward generated-data implementation. It keeps `astronomy-engine` as target and requires generator contract work before any artifact acceptance.

`generator-contract.json` records the contract-only input/output, manifest update, and `sha256` hash rules. It must not be used as proof that generated rows exist.

`generator-implementation-entry.json` records the M10 guarded non-dry-run generator entrypoint. It allows implementation entry inspection but must not be used as proof that source snapshots, generated artifacts, hashes, comparison evidence, golden rows, replay tests, or runtime integration exist.

`source-adapter-contract.json` records the contract-only source boundary for GB/T, Horizons, SOFA, and SPICE. It forbids runtime dependency and output claims in the contract stage.

`source-snapshot-manifest.schema.json`, `source-snapshot-manifest-plan.json`, and `source-snapshots/source-snapshot-manifest.json` define and materialize the M10 source snapshot manifest metadata. The manifest records selected source-boundary payloads for `naif-cspice` and `iau-sofa-ansi-c`; it does not materialize generated astronomy artifacts, generated artifact hashes, or accepted evidence.

`source-payload-materialization-policy.json` defines per-source payload files and now permits `source-snapshots/payloads/naif-cspice-kernel-boundary.json` with sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`, `source-snapshots/payloads/iau-sofa-routine-version.json` with sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f`, `source-snapshots/payloads/jpl-horizons-validation-samples.json` with sha256 `acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9`, and `source-snapshots/payloads/gb-t-33661-2017-rule-reference.json` with sha256 `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31`. Generated artifacts remain 0, and no accepted evidence exists.

`source-payload-schemas/*.schema.json` defines per-source payload shapes only. Schema files are not payload materialization and do not prove source hashes, generated artifacts, or runtime support.

`source-capture-procedure.json` defines source capture steps and records selected `naif-cspice` and `iau-sofa-ansi-c` boundary payload materialization only. It does not call external sources in full gates, generate astronomy artifacts, or prove runtime support.

`source-payload-materialization-decision.json` selects the first single-source payload candidate only. It does not create the payload directory, write the selected payload, compute source hashes, call external sources in full gates, generate astronomy artifacts, or prove runtime support.

`selected-source-payload-materialization-preflight.json` defines selected-source-only preconditions for the LOOP-046 materialization loop. It remains preflight evidence only.

`selected-source-payload-materialization.json` and `source-snapshots/payloads/naif-cspice-kernel-boundary.json` record the LOOP-046 selected `naif-cspice` source-boundary payload and source hash only. They are not SPICE kernels, CSPICE toolkit files, generated astronomy artifacts, accepted manifest evidence, runtime integration, or `astronomy-engine` support evidence.

`remaining-source-payload-strategy.json` records the LOOP-047 remaining source sequence only. It selects `iau-sofa-ansi-c` for the next preflight candidate, then JPL Horizons, then GB/T; it does not write new payloads, compute new source hashes, call external services in full gates, generate artifacts, accept the draft manifest, or prove runtime support.

`post-iau-remaining-source-payload-strategy.json` records the LOOP-050 remaining source sequence after IAU SOFA materialization only. It selects JPL Horizons as the next selected-source-only preflight candidate and leaves GB/T for a later governed scope; it does not write JPL or GB/T payload files, compute new source hashes, call external services in full gates, generate artifacts, accept the draft manifest, change runtime behavior, replace Android baseline, or prove runtime support.

`selected-jpl-horizons-payload-materialization-preflight.json` records the LOOP-051 selected JPL Horizons preflight only. It allows the next loop to materialize only the `jpl-horizons-api` validation-query snapshot payload while keeping that payload absent in LOOP-051; it does not execute online JPL queries in full gates, compute a new source hash, generate artifacts, accept the draft manifest, change runtime behavior, replace Android baseline, or prove runtime support.

`selected-jpl-horizons-payload-materialization.json` and `source-snapshots/payloads/jpl-horizons-validation-samples.json` record the LOOP-052 selected JPL Horizons validation-query snapshot boundary payload and source hash only. They do not include JPL response bodies, execute online JPL queries in full gates, enable runtime network dependency, generate astronomy artifacts, accept the draft manifest, change runtime behavior, replace Android baseline, or prove `astronomy-engine` support.

`selected-gb-t-payload-materialization-preflight.json` records the LOOP-053 selected GB/T preflight only. It allows the next loop to materialize only the `gb-t-33661-2017` calendar rule-reference payload while keeping that payload absent in LOOP-053; it captures no rule text in the full gate, computes no GB/T source hash, generates no astronomy artifacts, accepts no draft manifest, changes no runtime behavior, replaces no Android baseline, and proves no runtime support.

`selected-gb-t-payload-materialization.json` records the LOOP-054 selected GB/T materialization evidence only. It materializes `source-snapshots/payloads/gb-t-33661-2017-rule-reference.json` as a rule-reference boundary with sha256 `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31`; it copies no standard text, implements no Chinese-calendar algorithm, generates no astronomy artifacts, accepts no draft manifest, changes no runtime behavior, replaces no Android baseline, and proves no runtime support.

`selected-iau-sofa-payload-materialization-preflight.json` records the LOOP-048 selected IAU SOFA preflight only. It allows the next loop to materialize only the `iau-sofa-ansi-c` local routine/version boundary payload while keeping that payload absent in LOOP-048; it does not compute a new source hash, call external services in full gates, generate artifacts, accept the draft manifest, change runtime behavior, or prove runtime support.

`selected-iau-sofa-payload-materialization.json` and `source-snapshots/payloads/iau-sofa-routine-version.json` record the LOOP-049 selected IAU SOFA routine/version boundary payload and source hash only. They do not vendor SOFA source, compile or link SOFA routines, enable runtime dependency, generate astronomy artifacts, accept the draft manifest, change runtime behavior, replace Android baseline, or prove runtime support.

`artifact-writer-plan.json` records dry-run-only output path and hash preview rules. It must not create directories, write files, compute hashes for nonexistent artifacts, or update manifest hash state.

`comparison-runner-plan.json` records dry-run-only Android baseline bindings for future comparison artifacts. It must not read generated astronomy rows, write comparison artifacts, or classify real differences.

`golden-row-readiness-plan.json` records readiness-only gates for future golden rows. It keeps categories blocked and not generated until generated rows, source references, expected values, and difference classification are available.

`replay-test-readiness-plan.json` records readiness-only gates for future old-snapshot replay tests. It keeps replay tests unexecuted until old snapshot metadata, generated rows, classified comparison output, and replacement ADR evidence exist.

ADR 0016 selects the source stack for generated evidence planning:

- GB/T 33661-2017 as the modern Chinese-calendar rule reference.
- NASA/JPL Horizons API as the online validation source.
- IAU SOFA ANSI C as the standards-routine target.
- NAIF CSPICE/SPICE kernels as the offline reproducibility candidate.

Network availability for those external sources can be checked manually with:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\probe-astronomy-sources.ps1 -ProjectRoot .
```

This probe is intentionally not part of `tools/check-project.ps1`, because project quality gates must not fail only because an external site is temporarily unavailable.

The GB/T standard page is treated as a reference-page warning in the probe, while JPL Horizons, IAU SOFA, and NAIF are treated as required availability checks for the selected source/tooling path.
