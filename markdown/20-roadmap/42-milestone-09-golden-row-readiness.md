# M9 Golden-Row Materialization Readiness Evidence

## 1. Scope

This evidence belongs to LOOP-035. It adds readiness planning for future golden-row materialization.

It does not generate golden rows, does not write golden fixture files, does not mark categories generated, and does not create accepted evidence.

## 2. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/golden-row-readiness-plan.json` | readiness only | Defines required preconditions and blocked category readiness before golden rows may be materialized. |
| `tools/golden-cases-dry-run.ps1` | dry-run only | Reports golden category requirements and readiness categories with zero generated rows. |

## 3. Readiness Result

- mode: `golden_cases_dry_run_only`
- golden row readiness plan: `m9-golden-row-materialization-readiness-v1`
- required categories: 6
- readiness categories: 6
- category readiness: `blocked_until_generated_rows`
- generated rows: 0
- writes performed: false
- accepted evidence: false

## 4. Validation

`tools/check-astronomy-preflight.ps1` verifies readiness plan linkage, category coverage, blocked/not-generated status, required preconditions, forbidden readiness-stage actions, and zero-row dry-run output.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 5. Next Work

LOOP-036 should add replay-test materialization readiness planning. It must still avoid generated rows and accepted artifacts.
