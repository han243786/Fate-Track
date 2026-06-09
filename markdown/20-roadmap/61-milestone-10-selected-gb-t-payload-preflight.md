# M10 Selected GB/T Payload Preflight

> LOOP-053 evidence. This file records selected-source-only preflight for the GB/T 33661-2017 calendar rule-reference payload. It does not write the GB/T payload file and does not compute a GB/T source payload hash.

## 1. Scope

LOOP-053 adds preflight evidence for exactly one next selected source:

- selected source `gb-t-33661-2017`
- payload kind `calendar-rule-reference`
- schema `data/generated/astronomy/source-payload-schemas/gb-t-33661-2017-rule-reference.schema.json`
- future payload path `data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json`
- preflight evidence `data/generated/astronomy/selected-gb-t-payload-materialization-preflight.json`

The current materialized source payloads remain:

- `naif-cspice`, sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`
- `iau-sofa-ansi-c`, sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f`
- `jpl-horizons-api`, sha256 `acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9`

GB/T remains not materialized in LOOP-053.

## 2. Boundary

The GB/T preflight is a rule-reference capture boundary. It records the required payload fields, required rule-scope fields, next-loop write/hash scope, and forbidden claims.

It does not capture or summarize rule text, does not create `gb-t-33661-2017-rule-reference.json`, does not compute a GB/T source hash, does not write generated astronomy artifacts, does not accept the draft manifest, and does not replace the Android date layer.

## 3. Evidence

- `data/generated/astronomy/selected-gb-t-payload-materialization-preflight.json`
- `data/generated/astronomy/source-payload-schemas/gb-t-33661-2017-rule-reference.schema.json`
- `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`
- `data/generated/astronomy/source-payload-materialization-policy.json`
- `data/generated/astronomy/source-capture-procedure.json`
- `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1`
- `tools/check-astronomy-preflight.ps1`

## 4. Guardrails

LOOP-053 forbids:

- writing the GB/T payload file;
- computing the GB/T source payload hash;
- capturing GB/T rule references inside the full project gate;
- performing external calls in the full project gate;
- writing generated astronomy artifacts;
- computing generated artifact hashes;
- accepting the draft manifest;
- changing calendar or chart runtime behavior;
- replacing `android-date-layer-v1`;
- promoting `astronomy-engine`.

## 5. Validation

Required checks:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\selected-gb-t-payload-materialization-preflight-dry-run.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-astronomy-preflight.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Expected invariants:

- selected source `gb-t-33661-2017`;
- selected payload exists `false`;
- existing payload count `3`;
- source payloads materialized `3`;
- new source payload writes `0`;
- new source payload hashes `0`;
- source reference capture allowed in this loop `false`;
- payload materialization allowed in this loop `false`;
- external calls performed `false`;
- generated artifacts `0`;
- generated artifact hashes `0`;
- acceptance unchanged;
- runtime unchanged;
- Android baseline unchanged;
- `astronomy-engine` remains `target`.

## 6. Closeout Meaning

Closing this file means GB/T rule-reference payload materialization is ready for the next selected-source-only loop.

It is not evidence for:

- GB/T payload existence;
- GB/T source hash existence;
- generated astronomy rows;
- accepted astronomy manifest;
- Android baseline replacement;
- supported `astronomy-engine`.
