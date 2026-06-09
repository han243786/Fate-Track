# M10 Selected IAU SOFA Payload Materialization

> LOOP-049 evidence. This file records selected-source-only materialization for the IAU SOFA routine/version boundary payload. It does not integrate SOFA routines, does not write generated astronomy rows, and does not change runtime behavior.

## 1. Scope

LOOP-049 materializes only:

- `data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json`

It records only this new source payload hash:

- sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f`

The existing NAIF payload remains unchanged:

- `data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json`
- sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`

## 2. Evidence

- `data/generated/astronomy/selected-iau-sofa-payload-materialization.json`
- `data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json`
- `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`
- `data/generated/astronomy/source-payload-materialization-policy.json`
- `data/generated/astronomy/source-capture-procedure.json`
- `tools/source-snapshot-manifest-dry-run.ps1`
- `tools/source-payload-materialization-dry-run.ps1`
- `tools/source-capture-procedure-dry-run.ps1`
- `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1`
- `tools/check-astronomy-preflight.ps1`

## 3. Guardrails

LOOP-049 forbids:

- writing JPL Horizons payload files;
- writing GB/T payload files;
- writing generated astronomy artifacts;
- computing generated artifact hashes;
- accepting the draft manifest;
- changing calendar or chart runtime behavior;
- replacing `android-date-layer-v1`;
- promoting `astronomy-engine`;
- claiming SOFA routine integration;
- claiming runtime dependency is enabled.

## 4. Validation

Required checks:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\source-payload-materialization-dry-run.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\selected-iau-sofa-payload-materialization-preflight-dry-run.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-astronomy-preflight.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Expected invariants:

- materialized source payload count `2`;
- payload hashes computed `2`;
- new LOOP-049 source payload writes `1`;
- new LOOP-049 source payload hashes `1`;
- JPL/GB payloads absent;
- generated artifacts `0`;
- generated artifact hashes `0`;
- external full-gate calls `false`;
- acceptance unchanged;
- runtime unchanged;
- `astronomy-engine` remains `target`.

## 5. Closeout Meaning

Closing this file means IAU SOFA is represented by a local routine/version boundary payload.

It is not evidence for:

- SOFA source vendoring;
- SOFA compilation or linking;
- runtime routine integration;
- generated astronomy rows;
- accepted astronomy manifest;
- true solar time support;
- IANA timezone-history support;
- Android baseline replacement;
- supported `astronomy-engine`.
