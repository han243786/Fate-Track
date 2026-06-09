# M10 Post-IAU Remaining Source Payload Strategy

> LOOP-050 evidence. This file records the strategy decision after selected IAU SOFA source-boundary materialization. It selects the next remaining source candidate but does not materialize JPL Horizons or GB/T payloads.

## 1. Scope

LOOP-050 adds strategy-decision-only evidence:

- `data/generated/astronomy/post-iau-remaining-source-payload-strategy.json`
- `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1`

The strategy preserves the two existing materialized source payloads:

- `naif-cspice`, sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`
- `iau-sofa-ansi-c`, sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f`

It selects this next sequence:

1. `jpl-horizons-api` as selected-source-only preflight.
2. `gb-t-33661-2017` after the JPL boundary is governed.

## 2. Evidence

- `data/generated/astronomy/post-iau-remaining-source-payload-strategy.json`
- `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1`
- `tools/check-astronomy-preflight.ps1`
- `data/generated/astronomy/source-payload-materialization-policy.json`
- `data/generated/astronomy/source-capture-procedure.json`
- `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`
- `data/generated/astronomy/selected-iau-sofa-payload-materialization.json`
- `data/generated/astronomy/source-payload-schemas/jpl-horizons-validation-samples.schema.json`

## 3. Guardrails

LOOP-050 forbids:

- writing JPL Horizons payload files;
- writing GB/T payload files;
- computing new source payload hashes;
- performing external API calls in the full project gate;
- writing generated astronomy artifacts;
- computing generated artifact hashes;
- accepting the draft manifest;
- changing calendar or chart runtime behavior;
- replacing `android-date-layer-v1`;
- promoting `astronomy-engine`.

## 4. Validation

Required checks:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\post-iau-remaining-source-payload-strategy-dry-run.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-astronomy-preflight.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Expected invariants:

- materialized source payload count `2`;
- remaining source count `2`;
- next selected source `jpl-horizons-api`;
- next loop action `selected_source_payload_preflight_only`;
- existing payload count `2`;
- new source payload writes `0`;
- new source payload hashes `0`;
- generated artifacts `0`;
- generated artifact hashes `0`;
- external full-gate calls `false`;
- acceptance unchanged;
- runtime unchanged;
- `astronomy-engine` remains `target`.

## 5. Closeout Meaning

Closing this file means the post-IAU source sequence is governed and machine-checked.

It is not evidence for:

- JPL Horizons payload materialization;
- GB/T rule-reference payload materialization;
- an online JPL query executed in the full gate;
- generated astronomy rows;
- accepted astronomy manifest;
- Android baseline replacement;
- supported `astronomy-engine`.
