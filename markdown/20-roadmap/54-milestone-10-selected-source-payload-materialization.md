# M10 Selected Source Payload Materialization

## 1. Scope

LOOP-046 materializes exactly one selected source payload:

- source: `naif-cspice`
- payload: `data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json`
- evidence: `data/generated/astronomy/selected-source-payload-materialization.json`
- sha256: `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`

This payload is source-boundary evidence only. It records the CSPICE toolkit and SPICE kernel policy boundary without storing toolkit files, kernel files, generated astronomy rows, generated artifact hashes, accepted manifest evidence, runtime integration, or capability promotion.

## 2. Allowed Work

- Create `data/generated/astronomy/source-snapshots/payloads/`.
- Write only `naif-cspice-kernel-boundary.json`.
- Record the selected source payload sha256 in the source manifest, payload policy, capture procedure, and materialization evidence.
- Update dry-run inspection scripts so they allow exactly one selected source payload and reject all unselected payload files.

## 3. Forbidden Work

- Do not write GB/T, JPL Horizons, or IAU SOFA payload files.
- Do not write generated astronomy artifacts under `data/generated/astronomy/out/`.
- Do not compute generated artifact hashes.
- Do not mark `astronomy-engine-v0-draft.json` accepted.
- Do not change `calendar-date-query`, `chart-create`, or frontend runtime behavior.
- Do not claim CSPICE toolkit integration, SPICE kernel materialization, Android baseline replacement, or `astronomy-engine` support.

## 4. Evidence

| Evidence | Status |
| --- | --- |
| `source-snapshots/payloads/naif-cspice-kernel-boundary.json` | selected source payload materialized |
| `selected-source-payload-materialization.json` | selected source materialization evidence |
| `source-snapshots/source-snapshot-manifest.json` | `selected_source_payload_materialized` |
| `source-payload-materialization-policy.json` | selected-source-only payload/hash recorded |
| `source-capture-procedure.json` | selected source completed; other sources not started |
| `tools/source-snapshot-manifest-dry-run.ps1` | selected payload inspection |
| `tools/source-payload-materialization-dry-run.ps1` | selected payload inspection |
| `tools/source-capture-procedure-dry-run.ps1` | selected payload inspection |
| `tools/source-payload-materialization-decision-dry-run.ps1` | selected payload decision inspection |
| `tools/selected-source-payload-materialization-preflight-dry-run.ps1` | preflight close inspection |
| `tools/check-astronomy-preflight.ps1` | full astronomy gate updated |

## 5. Closeout Criteria

- Exactly one source payload file exists: `naif-cspice-kernel-boundary.json`.
- Its sha256 matches `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`.
- GB/T, JPL Horizons, and IAU SOFA payload files are absent.
- Generated astronomy artifacts and generated artifact hashes remain 0.
- Draft manifest acceptance remains `not_accepted`.
- Runtime behavior remains unchanged.
- `astronomy-engine` remains `target`.
- `tools/check-project.ps1` passes.

## 6. Next Slice

The next M10 slice should decide the remaining source payload strategy before any generated astronomy artifact writer is allowed to create output. A safe next target is a single-source decision/preflight for either IAU SOFA local routine version evidence or JPL Horizons offline validation sample evidence.
