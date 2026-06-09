# M9 Golden-Case Dry-Run Evidence

## 1. Scope

This evidence belongs to LOOP-027. It adds a golden-case dry-run scaffold that reports required golden categories without generating rows.

It does not generate golden cases, does not write files, and does not create accepted evidence.

## 2. Artifact

| Artifact | Status | Purpose |
| --- | --- | --- |
| `tools/golden-cases-dry-run.ps1` | dry-run only | Reads `golden-cases-plan.json` and reports required categories with zero generated rows. |

## 3. Dry-Run Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\golden-cases-dry-run.ps1 -ProjectRoot .
```

## 4. Latest Dry-Run Result

- mode: `golden_cases_dry_run_only`
- required category count: 6
- generated rows: 0
- writes performed: false
- accepted evidence: false

## 5. Validation

`tools/check-astronomy-preflight.ps1` invokes this dry-run and fails if it reports generated rows, writes files, claims accepted evidence, or disagrees with the golden plan category count.

## 6. Next Work

LOOP-028 may add a replay-policy dry-run or M9 final pre-closeout audit. It must not generate accepted astronomy artifacts or promote `astronomy-engine`.
