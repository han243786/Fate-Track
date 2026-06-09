# M9 Comparison Dry-Run Evidence

## 1. Scope

This evidence belongs to LOOP-026. It adds a comparison dry-run scaffold for Android-vs-astronomy comparison output shape.

It does not compare generated rows, does not write files, and does not create accepted evidence.

## 2. Artifact

| Artifact | Status | Purpose |
| --- | --- | --- |
| `tools/compare-astronomy-dry-run.ps1` | dry-run only | Emits a schema-shaped comparison object with zero rows and no accepted evidence. |

## 3. Dry-Run Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\compare-astronomy-dry-run.ps1 -ProjectRoot . -Manifest data/generated/astronomy/manifests/astronomy-engine-v0-draft.json
```

## 4. Latest Dry-Run Result

- mode: `comparison_dry_run_only`
- rows compared: 0
- difference rows: 0
- writes performed: false
- accepted evidence: false

## 5. Validation

`tools/check-astronomy-preflight.ps1` invokes this dry-run and fails if it reports comparison rows, writes files, or claims accepted evidence.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 6. Next Work

LOOP-027 may add golden-case dry-run scaffolding. It must not generate actual golden rows or change `astronomy-engine` status.
