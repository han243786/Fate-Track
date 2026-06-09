# M9 Comparison, Golden Case, and Replay Plan Evidence

## 1. Scope

This evidence belongs to LOOP-025. It defines planning artifacts required before any astronomy generated data can be accepted.

It is not generated astronomy data and does not promote `astronomy-engine`.

## 2. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/comparison.schema.json` | schema_only | Required shape for Android-vs-astronomy comparison results. |
| `data/generated/astronomy/golden-cases-plan.json` | planned_not_generated | Required golden-case categories and minimum case counts. |
| `data/generated/astronomy/replay-policy-draft.md` | draft only | Policy for preserving old `android-date-layer-v1` replay. |

## 3. Golden Categories

- 1901-2100 boundary
- 2033 anomaly
- Lichun boundary
- Qingming boundary
- Jiazi day anchor
- near-midnight solar/lunar event

## 4. Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-astronomy-preflight.ps1 -ProjectRoot .
```

The checker verifies that comparison fields exist, all difference categories are represented, golden categories remain `not_generated`, and the replay policy forbids silent Android baseline replacement.

## 5. Next Work

LOOP-026 may add dry-run comparison scaffolding or a generator implementation proposal. It must not produce accepted artifacts before hashes, comparison rows, golden rows, and replay tests exist.
