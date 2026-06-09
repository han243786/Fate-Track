# M10 Selected JPL Horizons Payload Materialization

> LOOP-052 evidence. This file records selected-source-only materialization for the JPL Horizons validation-query snapshot boundary payload. It does not execute online JPL queries in the full project gate and does not materialize generated astronomy artifacts.

## 1. Scope

LOOP-052 materializes exactly one selected JPL Horizons payload:

- `data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json`
- sha256 `acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9`
- evidence file `data/generated/astronomy/selected-jpl-horizons-payload-materialization.json`

The current materialized source payloads are:

- `naif-cspice`, sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`
- `iau-sofa-ansi-c`, sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f`
- `jpl-horizons-api`, sha256 `acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9`

GB/T remains not materialized.

## 2. Boundary

The JPL payload is an offline validation-query snapshot boundary. It records query ids, fixed query parameters, response metadata requirements, and provenance.

It does not include JPL response bodies, does not execute online queries in `tools/check-project.ps1`, does not create a runtime network dependency, and does not prove generated astronomy rows.

## 3. Evidence

- `data/generated/astronomy/selected-jpl-horizons-payload-materialization.json`
- `data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json`
- `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`
- `data/generated/astronomy/source-payload-materialization-policy.json`
- `data/generated/astronomy/source-capture-procedure.json`
- `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1`
- `tools/source-snapshot-manifest-dry-run.ps1`
- `tools/source-payload-materialization-dry-run.ps1`
- `tools/source-capture-procedure-dry-run.ps1`
- `tools/check-astronomy-preflight.ps1`

## 4. Guardrails

LOOP-052 forbids:

- writing GB/T payload files;
- computing GB/T source payload hashes;
- executing online JPL Horizons queries in the full project gate;
- treating the JPL payload as response-body evidence;
- writing generated astronomy artifacts;
- computing generated artifact hashes;
- accepting the draft manifest;
- changing calendar or chart runtime behavior;
- replacing `android-date-layer-v1`;
- promoting `astronomy-engine`.

## 5. Validation

Required checks:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-astronomy-preflight.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Expected invariants:

- selected source `jpl-horizons-api`;
- selected payload exists `true`;
- existing payload count `3`;
- source payloads materialized `3`;
- new source payload writes `1`;
- new source payload hashes `1`;
- query execution allowed in full gate `false`;
- external calls performed `false`;
- generated artifacts `0`;
- generated artifact hashes `0`;
- acceptance unchanged;
- runtime unchanged;
- `astronomy-engine` remains `target`.

## 6. Closeout Meaning

Closing this file means JPL Horizons selected-source payload materialization is complete as source-boundary evidence only.

It is not evidence for:

- online JPL query execution in the full gate;
- JPL response-body capture;
- GB/T rule-reference payload materialization;
- generated astronomy rows;
- accepted astronomy manifest;
- Android baseline replacement;
- supported `astronomy-engine`.
