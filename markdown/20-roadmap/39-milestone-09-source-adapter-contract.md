# M9 Source Adapter Contract Evidence

## 1. Scope

This evidence belongs to LOOP-032. It defines source adapter contracts for the selected M9 source stack before any generated astronomy rows exist.

It does not call external APIs, does not integrate SOFA or SPICE, does not generate rows, and does not promote `astronomy-engine`.

## 2. Contract Artifact

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/source-adapter-contract.json` | contract only | Defines how GB/T, Horizons, SOFA, and SPICE may later provide reproducible generation inputs. |

## 3. Adapter Boundaries

| Source | Adapter role | Runtime dependency? | Output claim allowed? |
| --- | --- | --- | --- |
| `gb-t-33661-2017` | calendar-rule-reference | false | false |
| `jpl-horizons-api` | online-validation-sample-source | false | false |
| `iau-sofa-ansi-c` | standards-routine-target | false | false |
| `naif-cspice` | offline-reproducibility-candidate | false | false |

## 4. Required Before Generated Acceptance

- Adapter input snapshots or local routine versions recorded.
- Adapter provenance recorded in manifest source references.
- Validation sample timestamps recorded.
- Offline dependency versions recorded when used.
- No runtime network dependency.

## 5. Validation

`tools/check-astronomy-preflight.ps1` verifies source adapter coverage, contract-only status, source policy linkage, generator contract linkage, no runtime dependency, no output claim, and no external API call in the full project gate.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 6. Next Work

LOOP-033 should add artifact-writer dry-run planning. It must still avoid generated rows and accepted artifacts.
