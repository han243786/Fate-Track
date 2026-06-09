# M2 Preflight: Ruleset and Chart Basis

## 1. Cursor

| Field | Value |
| --- | --- |
| `loop_id` | `LOOP-011` |
| `mode` | `milestone_loop` |
| `milestone` | M2 |
| `invariant` | Establish a restricted chart-basis contract without implementing full chart calculation. |

## 2. Decision Gate Audit

| Gate | Status | M2 Handling |
| --- | --- | --- |
| DG-001 | target-proposed | M2 will use `ft-v1-default` as the single target ruleset id. |
| DG-003 | target-proposed | M2 will document default day boundary as local civil midnight and keep Zi-start as unsupported advanced mode. |
| DG-004 | open | Lunar input remains unsupported/planned; M2 must explicitly reject it. |

## 3. Selected Slice

Use one L-sized loop for M2 contract basis:

| Work Package | Included? | Notes |
| --- | --- | --- |
| M2-WP1 | yes | Define `RulesetId`, `AlgoVersion`, and `CalculationMetadata`. |
| M2-WP2 | yes | Define `BirthProfile`, `ChartRequest`, and `ChartBasis` fields with privacy level. |
| M2-WP3 | yes | Validate date, time precision, timezone field, true-solar-time flag, and lunar input. |
| M2-WP4 | yes | Add a restricted preview API contract and error behavior. |
| M2-WP5 | yes | Keep target `/api/v1/...` relationship documented; current prototype route stays under `/api/charts/basis/preview`. |

## 4. Explicit Non-Goals

- No complete four-pillar calculation.
- No hour pillar.
- No IANA timezone history.
- No true solar time.
- No persisted chart.
- No lunar input support while DG-004 remains open.

## 5. Validation

Required:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

M2 may continue to implementation after this preflight because the selected slice is explicit and remains inside one invariant.

