# Astronomy Generated Data Preflight

This directory contains both historical M9/M10 preflight governance artifacts and the later generated astronomy outputs used by the preview release.

Important: historical M9/M10 text below intentionally preserves phrases such as "no generated astronomy table has been accepted yet" because those phrases are part of the preflight anti-overclaim gate. The current release status is recorded in `docs/release/v1-release-candidate.md`, `docs/release/v1-closeout.md`, ADR 0021, and `93-capability-promotion-ledger.md`: `astronomy-engine` is supported as an independent verified engine, while Android date-layer runtime replacement remains a separate ADR decision.

Current status:

```text
no generated astronomy table has been accepted yet
```

Allowed source files in this preflight slice:

- `source-policy.json`: official source stack and first generated-range policy.
- `generation-plan.json`: draft generated artifact shape and command plan.
- `implementation-plan.json`: planning-only generated-data implementation path.
- `generator-contract.json`: contract-only generator input/output, hash, and manifest update policy.
- `generator-implementation-entry.json`: M10 guarded non-dry-run generator entrypoint policy.
- `source-adapter-contract.json`: contract-only selected-source adapter boundary.
- `source-snapshot-manifest.schema.json`: M10 source snapshot manifest schema boundary.
- `source-snapshot-manifest-plan.json`: M10 source snapshot manifest materialization plan.
- `source-snapshots/source-snapshot-manifest.json`: selected-source payload manifest state; `naif-cspice`, `iau-sofa-ansi-c`, `jpl-horizons-api`, and `gb-t-33661-2017` source-boundary payloads are materialized.
- `source-payload-materialization-policy.json`: M10 source payload materialization policy; only the selected `naif-cspice`, `iau-sofa-ansi-c`, `jpl-horizons-api`, and `gb-t-33661-2017` payload files/hashes are present.
- `source-payload-schemas/*.schema.json`: M10 per-source payload schemas, schema-only.
- `source-capture-procedure.json`: M10 per-source capture procedure with selected `naif-cspice` boundary payload materialized.
- `source-payload-materialization-decision.json`: M10 first source payload materialization decision, decision-only.
- `selected-source-payload-materialization-preflight.json`: M10 selected-source payload materialization preflight, preflight-only.
- `selected-source-payload-materialization.json`: M10 selected-source payload materialization evidence for `naif-cspice`.
- `source-snapshots/payloads/naif-cspice-kernel-boundary.json`: source-boundary evidence only; not a SPICE kernel, not a CSPICE toolkit, and not a generated astronomy artifact.
- `remaining-source-payload-strategy.json`: M10 remaining source payload strategy, strategy-decision-only.
- `post-iau-remaining-source-payload-strategy.json`: M10 post-IAU remaining source payload strategy, strategy-decision-only.
- `selected-jpl-horizons-payload-materialization-preflight.json`: M10 selected JPL Horizons payload materialization preflight, preflight-only.
- `selected-jpl-horizons-payload-materialization.json`: M10 selected JPL Horizons validation-query snapshot boundary materialization evidence.
- `source-snapshots/payloads/jpl-horizons-validation-samples.json`: offline query-parameter snapshot boundary evidence only; it contains no JPL response bodies, does not execute online queries in the full gate, and is not a generated astronomy artifact.
- `selected-gb-t-payload-materialization-preflight.json`: M10 selected GB/T rule-reference payload materialization preflight, preflight-only historical evidence.
- `selected-gb-t-payload-materialization.json`: M10 selected GB/T rule-reference boundary materialization evidence.
- `source-snapshots/payloads/gb-t-33661-2017-rule-reference.json`: rule-reference boundary evidence only; it copies no GB/T standard text, implements no Chinese-calendar algorithm, and is not a generated astronomy artifact.
- `generated-artifact-materialization-preflight.json`: M10 generated astronomy artifact materialization preflight, preflight-only historical evidence for LOOP-055.
- `generated-artifact-materialization.json`: M10 generated astronomy artifact materialization evidence for LOOP-057; records 4 boundary placeholder artifacts with sha256 hashes.
- `out/solar-terms-1901-2100.json`: generated boundary placeholder; establishes artifact schema and output path; no solar term data has been astronomically computed.
- `out/new-moons-1901-2100.json`: generated boundary placeholder; establishes artifact schema and output path; no new moon data has been astronomically computed.
- `out/lunar-calendar-1901-2100.json`: generated boundary placeholder; establishes artifact schema and output path; no lunar calendar data has been astronomically derived.
- `out/android-comparison-1901-2100.json`: generated boundary placeholder; establishes artifact schema and output path; no Android-vs-astronomy comparison has been performed.
- `selected-iau-sofa-payload-materialization-preflight.json`: M10 selected IAU SOFA payload materialization preflight, preflight-only.
- `selected-iau-sofa-payload-materialization.json`: M10 selected IAU SOFA source-boundary payload materialization evidence.
- `source-snapshots/payloads/iau-sofa-routine-version.json`: source-boundary evidence only; not SOFA source vendoring, not routine integration, and not a generated astronomy artifact.
- `artifact-writer-plan.json`: dry-run-only output path and hash preview policy.
- `comparison-runner-plan.json`: dry-run-only Android baseline comparison binding policy.
- `comparison.schema.json`: Android-vs-astronomy comparison result schema.
- `golden-cases-plan.json`: required golden-case category plan.
- `golden-row-readiness-plan.json`: readiness-only golden row materialization gate plan.
- `replay-policy-draft.md`: old algorithm replay policy draft.
- `replay-test-readiness-plan.json`: readiness-only old snapshot replay-test gate plan.
- `precloseout-audit.json`: machine-readable audit showing full M9 closeout is blocked while preflight is ready.
- `preflight-closeout-decision.json`: machine-readable LOOP-037 decision closing M9 as preflight only and routing real implementation to M10.
- `manifest.schema.json`: required fields for future generated data manifests.
- `manifests/astronomy-engine-v0-draft.json`: not-accepted planning manifest instance.
- `comparison-report-template.md`: required structure for Android-vs-astronomy comparison reports.

Forbidden until generated evidence exists:

- claiming `astronomy-engine` as supported;
- widening the V1 validated date range beyond 1901-2100;
- replacing `android-date-layer-v1`;
- enabling true solar time or IANA timezone-history behavior.

Future generated manifests must record source policy id, manifest id, acceptance status, artifact type, engine id, engine version, source references, generated range, generation command, artifact hashes, comparison report path, difference taxonomy coverage, evidence requirements, acceptance blockers, and creation time.

The draft manifest must remain `not_accepted` until a real generation command, generated artifact, artifact hashes, comparison report, golden cases, and runtime/replay policy exist.

The generation plan is `draft_not_runnable`. It defines intended artifact paths and a dry-run command shape only; no artifact under `data/generated/astronomy/out/` is accepted.

The dry-run generator skeleton can inspect the plan without writing files:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\generate-astronomy-tables.ps1 -ProjectRoot . -Manifest data/generated/astronomy/manifests/astronomy-engine-v0-draft.json -DryRun
```

Comparison, golden-case, and replay-policy files are planning artifacts only. They do not prove generated astronomy correctness until real rows, hashes, classifications, and replay tests exist.

The replay-policy dry-run can inspect replacement controls without executing replay tests:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\replay-policy-dry-run.ps1 -ProjectRoot . -Manifest data/generated/astronomy/manifests/astronomy-engine-v0-draft.json
```

The M9 pre-closeout audit is intentionally not full closeout evidence. It records `full_m9_closeout_blocked_preflight_ready`, keeps `astronomy-engine` as target, and allows only preflight closeout review until generated artifacts, hashes, comparison rows, golden rows, replay tests, and runtime integration exist.

The M9 preflight closeout decision records `close_m9_as_preflight_only`. It closes only the governance preflight milestone and routes real generated-data implementation to M10. It must not be used as generated-data acceptance, Android replacement, runtime route change, wider-range support, true-solar-time support, timezone-history support, or `astronomy-engine` promotion evidence.

The generated-data implementation plan is `planning_only`. It keeps `astronomy-engine` as target and requires a generator contract before any generated artifact acceptance.

The generator contract is `contract_only`. It requires `sha256` for every planned output, keeps outputs `not_generated`, and forbids runtime replacement or artifact acceptance during the contract stage.

The generator implementation entry is `guarded_entrypoint_only`. It allows `tools/generate-astronomy-tables.ps1 -PrepareImplementation` to exercise the first M10 non-dry-run entry shape, but the command remains blocked, writes no files, computes no hashes, changes no manifest acceptance state, and keeps `astronomy-engine` as target until source payloads and later acceptance evidence exist.

The source adapter contract is `contract_only`. It maps GB/T, Horizons, SOFA, and SPICE into future reproducible input boundaries while forbidding runtime dependencies, output claims, and external API calls in the full project gate.

The source snapshot manifest schema is `schema_only`, and the source snapshot manifest plan is `manifest_materialized_metadata_only`. `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json` now records selected source payloads for `naif-cspice`, `iau-sofa-ansi-c`, `jpl-horizons-api`, and `gb-t-33661-2017`. This is source-boundary evidence only, with no generated astronomy artifact, generated artifact hash, manifest acceptance change, runtime change, or `astronomy-engine` promotion. `tools/source-snapshot-manifest-dry-run.ps1` verifies this selected-payload state.

The source payload materialization policy is `selected_source_payload_materialized`. It permits only `data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json` with sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`, `data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json` with sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f`, `data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json` with sha256 `acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9`, and `data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json` with sha256 `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31`. Generated astronomy artifacts remain unwritten, generated artifact hashes remain 0, manifest acceptance is unchanged, runtime is unchanged, and `astronomy-engine` remains target. `tools/source-payload-materialization-dry-run.ps1` verifies this selected-source-only state.

The source payload schemas are `schema_only`. They define payload shapes for GB/T rule references, JPL Horizons validation samples, IAU SOFA routine versions, and NAIF CSPICE toolkit/kernel boundaries without materializing any payload file or source hash.

The source capture procedure records selected-source payload materialization for `naif-cspice`, `iau-sofa-ansi-c`, `jpl-horizons-api`, and `gb-t-33661-2017`. It keeps external full-gate calls false, generated astronomy artifacts absent, manifest acceptance unchanged, runtime unchanged, and `astronomy-engine` target.

The source payload materialization decision is `decision_only`. It selects `naif-cspice` as the first single-source payload candidate while keeping the payload directory absent, selected payload absent, source hashes 0, external full-gate calls false, generated astronomy artifacts absent, manifest acceptance unchanged, runtime unchanged, and `astronomy-engine` target.

The selected source payload materialization preflight remains `preflight_only` historical evidence for LOOP-045. LOOP-046 closes that preflight by materializing only the `naif-cspice` boundary payload and its source hash.

The selected source payload materialization evidence is `selected_source_payload_materialized`. It records only `naif-cspice-kernel-boundary.json` as source-boundary evidence and forbids other source payloads, generated astronomy artifacts, generated artifact hashes, manifest acceptance changes, runtime changes, Android baseline replacement, CSPICE toolkit integration claims, SPICE kernel materialization claims, and `astronomy-engine` support claims.

The remaining source payload strategy is `strategy_decision_only`. It selects `iau-sofa-ansi-c` as the next preflight candidate and sequences JPL Horizons before GB/T after SOFA. It writes no new payload files, computes no new source hashes, performs no external full-gate calls, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, and keeps `astronomy-engine` as target.

The post-IAU remaining source payload strategy is `strategy_decision_only`. It preserves the two existing source-boundary payloads for `naif-cspice` and `iau-sofa-ansi-c`, selects `jpl-horizons-api` as the next selected-source-only preflight candidate, and leaves GB/T for the following governed scope. It writes no JPL or GB/T payload files, computes no new source hashes, performs no external full-gate calls, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, and keeps `astronomy-engine` as target.

The selected JPL Horizons payload materialization preflight is `preflight_only`. It scopes the next loop to only `jpl-horizons-api` validation-query snapshot payload materialization while keeping the JPL payload absent in LOOP-051. It writes no JPL or GB/T payload files, computes no new source hashes, performs no external full-gate calls, executes no online JPL query in the full gate, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, and keeps `astronomy-engine` as target.

The selected JPL Horizons payload materialization evidence is `selected_source_payload_materialized`. It records `jpl-horizons-validation-samples.json` as an offline validation-query snapshot boundary payload with sha256 `acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9`; it includes no JPL response bodies, executes no online JPL query in the full project gate, enables no runtime network dependency, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, replaces no Android baseline, and does not promote `astronomy-engine`.

The selected GB/T payload materialization preflight is `preflight_only`. It scopes the next loop to only `gb-t-33661-2017` calendar rule-reference payload materialization while keeping `gb-t-33661-2017-rule-reference.json` absent in LOOP-053. It captures no rule text in the full project gate, computes no GB/T source hash, performs no external full-gate call, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, replaces no Android baseline, and keeps `astronomy-engine` as target.

The selected GB/T payload materialization evidence is `selected_source_payload_materialized`. It records `gb-t-33661-2017-rule-reference.json` as a calendar rule-reference boundary payload with sha256 `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31`; it copies no GB/T standard text, implements no Chinese-calendar algorithm, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, replaces no Android baseline, and does not promote `astronomy-engine`.

The selected IAU SOFA payload materialization preflight is `preflight_only`. It scopes the next loop to only `iau-sofa-ansi-c` local routine/version boundary materialization while keeping the SOFA payload absent in LOOP-048. It writes no new payload files, computes no new source hashes, performs no external full-gate calls, writes no generated astronomy artifacts, changes no manifest acceptance or runtime behavior, and keeps `astronomy-engine` as target.

The selected IAU SOFA payload materialization evidence is `selected_source_payload_materialized`. It records `iau-sofa-routine-version.json` as a local routine/version boundary payload with sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f`; it does not vendor SOFA source, compile or link SOFA routines, enable runtime dependency, write generated astronomy artifacts, change manifest acceptance or runtime behavior, or promote `astronomy-engine`.

The artifact writer plan is `dry_run_only`. It previews output paths and `sha256` hash policy while forbidding output directory creation, artifact writes, hash computation for nonexistent files, manifest hash updates, and accepted evidence claims.

The comparison runner plan is `dry_run_only`. It binds `android-date-layer-v1` and `ft-date-layer-android-v1` to the future comparison artifact while keeping rows and differences at zero.

The golden row readiness plan is `readiness_only`. It keeps every required category `not_generated` and `blocked_until_generated_rows` while recording the preconditions for later materialization.

The replay test readiness plan is `readiness_only`. It keeps replay tests unexecuted and binds `android-date-layer-v1`/`ft-date-layer-android-v1` until generated rows, classified comparison output, and a replacement ADR exist.
