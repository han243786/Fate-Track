# M9 Generation Plan Evidence

## 1. Scope

This evidence belongs to LOOP-023. It defines the future generated artifact shape and command form for the M9 astronomy upgrade.

It is not a runnable generation script and not accepted generated data.

## 2. Artifact

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/generation-plan.json` | draft_not_runnable | Defines planned artifact paths, future command shape, hash requirements, and forbidden runtime claims. |

## 3. Planned Artifacts

| Path | Kind | Status |
| --- | --- | --- |
| `data/generated/astronomy/out/solar-terms-1901-2100.json` | solar-term crossing table | not generated |
| `data/generated/astronomy/out/new-moons-1901-2100.json` | new-moon table | not generated |
| `data/generated/astronomy/out/lunar-calendar-1901-2100.json` | derived Chinese-calendar table | not generated |
| `data/generated/astronomy/out/android-comparison-1901-2100.json` | Android-vs-astronomy comparison | not generated |

## 4. Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-astronomy-preflight.ps1 -ProjectRoot .
```

The checker requires:

- generation plan status is `draft_not_runnable`
- intended command script status is `not_implemented`
- every planned artifact remains `not_generated`
- every planned artifact requires a hash
- forbidden runtime claims remain listed

## 5. Next Work

LOOP-024 may start the generation script skeleton only if it keeps a dry-run or unimplemented mode and does not produce accepted artifacts without hashes and comparison evidence.
