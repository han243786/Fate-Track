# M10 Selected JPL Horizons Payload Preflight

> LOOP-051 evidence. This file records selected-source-only preflight for the JPL Horizons validation-query snapshot payload. It does not execute online queries in the full project gate and does not materialize the JPL payload.

## 1. Scope

LOOP-051 adds preflight-only evidence:

- `data/generated/astronomy/selected-jpl-horizons-payload-materialization-preflight.json`
- `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1`

The preflight preserves the two existing materialized source payloads:

- `naif-cspice`, sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`
- `iau-sofa-ansi-c`, sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f`

It scopes the next loop to `jpl-horizons-api` selected-source-only materialization.

## 2. Evidence

- `data/generated/astronomy/selected-jpl-horizons-payload-materialization-preflight.json`
- `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1`
- `data/generated/astronomy/post-iau-remaining-source-payload-strategy.json`
- `data/generated/astronomy/source-payload-schemas/jpl-horizons-validation-samples.schema.json`
- `data/generated/astronomy/source-payload-materialization-policy.json`
- `data/generated/astronomy/source-capture-procedure.json`
- `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`
- `tools/check-astronomy-preflight.ps1`

## 3. Guardrails

LOOP-051 forbids:

- writing JPL Horizons payload files;
- writing GB/T payload files;
- computing new source payload hashes;
- performing external API calls in the full project gate;
- executing online JPL Horizons queries in the full project gate;
- writing generated astronomy artifacts;
- computing generated artifact hashes;
- accepting the draft manifest;
- changing calendar or chart runtime behavior;
- replacing `android-date-layer-v1`;
- promoting `astronomy-engine`.

## 4. Validation

Required checks:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-astronomy-preflight.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Expected invariants:

- selected source `jpl-horizons-api`;
- selected payload exists `false`;
- existing payload count `2`;
- new source payload writes `0`;
- new source payload hashes `0`;
- query execution allowed in full gate `false`;
- generated artifacts `0`;
- generated artifact hashes `0`;
- acceptance unchanged;
- runtime unchanged;
- `astronomy-engine` remains `target`.

## 5. Closeout Meaning

Closing this file means JPL Horizons payload materialization is scoped and guarded for the next loop.

It is not evidence for:

- JPL Horizons query snapshot payload materialization;
- online JPL query execution in the full gate;
- GB/T rule-reference payload materialization;
- generated astronomy rows;
- accepted astronomy manifest;
- Android baseline replacement;
- supported `astronomy-engine`.
