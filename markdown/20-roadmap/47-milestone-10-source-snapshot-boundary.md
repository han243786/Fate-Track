# M10 Source Snapshot Manifest Boundary Evidence

## 1. Scope

This evidence belongs to LOOP-039. It defines the source snapshot manifest boundary for M10-WP2.

It does not create `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json`, does not create source snapshot files, does not write generated astronomy artifacts, does not compute artifact hashes, does not change manifest acceptance, and does not promote `astronomy-engine`.

## 2. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/source-snapshot-manifest.schema.json` | schema only | Defines the future source snapshot manifest fields and forbidden acceptance-stage actions. |
| `data/generated/astronomy/source-snapshot-manifest-plan.json` | manifest boundary only | Maps selected sources to the future manifest path while keeping materialization blocked. |
| `tools/source-snapshot-manifest-dry-run.ps1` | dry-run only | Verifies that the source snapshot manifest is still absent and no writes occur. |

## 3. Dry-Run Result

The boundary command is:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\source-snapshot-manifest-dry-run.ps1 -ProjectRoot .
```

Expected result:

- mode: `source_snapshot_manifest_boundary_dry_run`
- planned sources: 4
- manifest exists: false
- writes performed: false
- source snapshots materialized: 0
- generated artifacts written: 0
- hashes computed: 0
- acceptance status changed: false
- runtime behavior changed: false

## 4. Boundary Preserved

The future source snapshot manifest must represent:

1. GB/T 33661-2017 rule reference.
2. JPL Horizons validation snapshots.
3. IAU SOFA local routine or pinned version.
4. NAIF CSPICE/SPICE local toolkit and kernel boundary.

Every represented source must keep `runtime_dependency=false` and `output_claim_allowed=false` until later acceptance evidence exists.

## 5. Validation

`tools/check-astronomy-preflight.ps1` validates the schema, plan, dry-run result, selected source coverage, missing manifest state, and no-write/no-claim boundary.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 6. Next Work

LOOP-040 should decide how to materialize the source snapshot manifest without writing generated astronomy artifacts. It must preserve no runtime dependency, no output claim, no Android replacement, and no `astronomy-engine` promotion.
