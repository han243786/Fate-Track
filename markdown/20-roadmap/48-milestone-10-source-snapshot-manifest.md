# M10 Source Snapshot Manifest Materialization Evidence

## 1. Scope

This evidence belongs to LOOP-040. It materializes the source snapshot manifest as metadata only.

It creates `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`, but it does not create source payload files, does not write generated astronomy artifacts, does not compute generated artifact hashes, does not change draft manifest acceptance, does not change runtime behavior, and does not promote `astronomy-engine`.

## 2. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json` | metadata only, no source payload | Records selected source provenance and blocks output claims. |
| `data/generated/astronomy/source-snapshot-manifest-plan.json` | manifest materialized metadata only | Records that source payloads remain unmaterialized and generated artifacts remain unwritten. |
| `tools/source-snapshot-manifest-dry-run.ps1` | metadata dry-run | Verifies manifest metadata, selected source coverage, no source payloads, and no generated artifacts. |

## 3. Dry-Run Result

The inspection command is:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\source-snapshot-manifest-dry-run.ps1 -ProjectRoot .
```

Expected result:

- mode: `source_snapshot_manifest_metadata_dry_run`
- manifest status: `metadata_only_no_source_payload`
- manifest exists: true
- planned sources: 4
- manifest sources: 4
- source snapshots materialized: 0
- generated artifacts written: 0
- hashes computed: 0
- acceptance status changed: false
- runtime behavior changed: false

## 4. Boundary Preserved

The manifest records metadata for:

1. GB/T 33661-2017 rule reference.
2. JPL Horizons validation source.
3. IAU SOFA ANSI C routine target.
4. NAIF CSPICE/SPICE offline reproducibility candidate.

Every source remains `not_materialized`, with `runtime_dependency=false` and `output_claim_allowed=false`.

## 5. Validation

`tools/check-astronomy-preflight.ps1` validates that the manifest is metadata only, selected-source coverage matches the source policy and adapter contract, source payloads remain unmaterialized, and no generated-data acceptance has occurred.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 6. Next Work

LOOP-041 should define source payload materialization policy. It must not write generated astronomy artifacts, compute artifact hashes, change draft manifest acceptance, replace Android baseline, or promote `astronomy-engine`.
