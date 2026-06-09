# M10 Selected IAU SOFA Payload Preflight

> LOOP-048 evidence. This file closes only the preflight for the next selected remaining source payload. It does not materialize an IAU SOFA payload, does not compute a new source hash, and does not change runtime behavior.

## 1. Scope

LOOP-048 selects `iau-sofa-ansi-c` for selected-source payload materialization preflight only.

Allowed evidence:

- `data/generated/astronomy/selected-iau-sofa-payload-materialization-preflight.json`
- `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1`
- `tools/check-astronomy-preflight.ps1`

The next loop may materialize only:

- `data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json`

## 2. Current State

The only materialized source payload remains:

- `data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json`
- sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`

The selected IAU SOFA payload remains absent during LOOP-048.

## 3. Guardrails

LOOP-048 forbids:

- writing the IAU SOFA payload file;
- writing JPL Horizons or GB/T payload files;
- computing a new source payload hash;
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
powershell -NoProfile -ExecutionPolicy Bypass -File tools\selected-iau-sofa-payload-materialization-preflight-dry-run.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-astronomy-preflight.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Expected dry-run invariants:

- mode `selected_iau_sofa_payload_materialization_preflight_dry_run`;
- selected source `iau-sofa-ansi-c`;
- selected payload exists `false`;
- existing payload count `1`;
- source payloads materialized `1`;
- new source payloads written `0`;
- new source payload hashes computed `0`;
- generated artifacts written `0`;
- generated artifact hashes computed `0`;
- acceptance status changed `false`;
- runtime behavior changed `false`;
- writes performed `false`.

## 5. Closeout Meaning

Closing this file means only that the next recursive loop can safely attempt selected-source-only IAU SOFA payload materialization.

It is not evidence for:

- SOFA routine integration;
- generated astronomy rows;
- accepted astronomy manifest;
- true solar time support;
- IANA timezone-history support;
- Android baseline replacement;
- supported `astronomy-engine`.
