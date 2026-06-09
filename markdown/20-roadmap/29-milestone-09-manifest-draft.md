# M9 Generated Manifest Draft Evidence

## 1. Scope

This evidence belongs to LOOP-022. It creates the first generated-table manifest instance as a planning artifact only.

It is not generated astronomy data, not a hash record, not a comparison report, and not runtime engine support.

## 2. Artifact

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/manifests/astronomy-engine-v0-draft.json` | not accepted | Planning instance for the future generated table manifest. |

## 3. Required Blockers

The draft manifest must keep these blockers until real generated evidence exists:

- generation command not selected
- no generated artifact exists
- no artifact hashes exist
- comparison report is template only
- golden cases are not generated
- runtime engine is not integrated

## 4. Validation

The following command validates the draft manifest status:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-astronomy-preflight.ps1 -ProjectRoot .
```

`tools/check-project.ps1` includes the same check.

## 5. Next Work

LOOP-023 should decide the concrete generated artifact shape and generation method. No manifest can become accepted until hashes, comparison report, golden cases, and replay policy exist.
