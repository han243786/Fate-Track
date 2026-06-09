# M9 Generator Dry-Run Evidence

## 1. Scope

This evidence belongs to LOOP-024. It adds a generator script skeleton that can inspect the M9 astronomy generation plan in dry-run mode only.

It does not generate astronomy artifacts, does not write hashes, does not change manifest acceptance, and does not provide runtime engine support.

## 2. Artifact

| Artifact | Status | Purpose |
| --- | --- | --- |
| `tools/generate-astronomy-tables.ps1` | dry-run only | Reads source policy, generation plan, and draft manifest; reports planned artifacts without writing files. |

## 3. Dry-Run Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\generate-astronomy-tables.ps1 -ProjectRoot . -Manifest data/generated/astronomy/manifests/astronomy-engine-v0-draft.json -DryRun
```

## 4. Latest Dry-Run Result

- mode: `dry_run_only`
- manifest status: `not_accepted`
- planned artifact count: 4
- output directory exists: false
- writes performed: false
- acceptance status changed: false
- existing planned artifacts: none

## 5. Validation

`tools/check-astronomy-preflight.ps1` invokes the dry-run script and fails if it writes artifacts, changes acceptance status, sees existing planned artifacts, or reports a mismatched planned artifact count.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 6. Next Work

LOOP-025 may add a comparison/golden-case planning layer or start a generator implementation proposal. It must not create accepted artifacts without hashes, completed comparison evidence, and replay policy.
