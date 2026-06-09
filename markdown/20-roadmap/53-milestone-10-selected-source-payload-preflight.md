# M10 Selected Source Payload Materialization Preflight

## 1. Scope

This evidence belongs to LOOP-045. It prepares the selected `naif-cspice` source payload for a later materialization loop.

It does not create the payload directory, does not write the selected payload, does not write any other source payload, does not compute source payload hashes, does not call external sources in the full project gate, does not write generated astronomy artifacts, does not compute generated artifact hashes, does not change draft manifest acceptance, does not change runtime behavior, and does not promote `astronomy-engine`.

## 2. Selected Source

| Field | Value |
| --- | --- |
| selected source | `naif-cspice` |
| payload kind | `offline-kernel-toolkit-boundary` |
| future payload path | `data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json` |
| next-loop create scope | `selected_source_only` |
| next-loop write scope | `selected_source_only` |
| next-loop hash scope | `selected_source_payload_only` |

## 3. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/selected-source-payload-materialization-preflight.json` | preflight only | Defines selected-source-only directory/write/hash policy for the next loop. |
| `tools/selected-source-payload-materialization-preflight-dry-run.ps1` | dry-run only | Verifies preflight linkage, selected source, absent payload directory, absent payload files, zero writes/hashes/artifacts, and next-loop scope. |

## 4. Dry-Run Result

The inspection command is:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\selected-source-payload-materialization-preflight-dry-run.ps1 -ProjectRoot .
```

Expected result:

- mode: `selected_source_payload_materialization_preflight_dry_run`
- selected source: `naif-cspice`
- selected payload kind: `offline-kernel-toolkit-boundary`
- payload directory exists: false
- selected payload exists: false
- existing payload files: 0
- next loop create scope: `selected_source_only`
- next loop write scope: `selected_source_only`
- next loop hash scope: `selected_source_payload_only`
- source payloads materialized: 0
- payload hashes computed: 0
- external calls performed: false
- generated artifacts written: 0
- generated artifact hashes computed: 0
- acceptance status changed: false
- runtime behavior changed: false

## 5. Boundary Preserved

This preflight does not materialize the payload. It only narrows the next loop so that exactly one source payload may be materialized as source-boundary evidence.

LOOP-046 may create the payload directory and write only `naif-cspice-kernel-boundary.json` if the payload remains source evidence only and no generated astronomy artifact, generated artifact hash, manifest acceptance change, runtime behavior change, Android baseline replacement, or `astronomy-engine` promotion occurs.

## 6. Validation

`tools/check-astronomy-preflight.ps1` validates the preflight object, selected-source linkage, preflight dry-run, forbidden actions, and zero-output state.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```
