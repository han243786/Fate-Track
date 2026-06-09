# M9 Comparison Runner Dry-Run Evidence

## 1. Scope

This evidence belongs to LOOP-034. It adds comparison-runner dry-run planning for future Android-vs-astronomy comparison artifacts.

It does not read generated astronomy rows, does not write a comparison artifact, does not classify real differences, and does not create accepted evidence.

## 2. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/comparison-runner-plan.json` | dry-run only | Defines Android baseline bindings, future comparison artifact path, required binding fields, and forbidden dry-run actions. |
| `tools/compare-astronomy-dry-run.ps1` | dry-run only | Reports comparison runner plan bindings with zero rows and zero differences. |

## 3. Dry-Run Result

- mode: `comparison_dry_run_only`
- comparison runner plan: `m9-android-astronomy-comparison-runner-plan-v1`
- Android algorithm version: `android-date-layer-v1`
- Android ruleset id: `ft-date-layer-android-v1`
- future comparison artifact: `data/generated/astronomy/out/android-comparison-1901-2100.json`
- rows compared: 0
- difference rows: 0
- writes performed: false
- accepted evidence: false

## 4. Validation

`tools/check-astronomy-preflight.ps1` verifies comparison runner plan linkage, Android baseline bindings, required comparison binding fields, forbidden dry-run actions, and zero-row dry-run output.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 5. Next Work

LOOP-035 should add golden-row materialization readiness planning. It must still avoid generated rows and accepted artifacts.
