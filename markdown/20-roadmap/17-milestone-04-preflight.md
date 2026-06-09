# M4 Preflight: Analysis Engine

## 1. Cursor

| Field | Value |
| --- | --- |
| `loop_id` | `LOOP-014` |
| `mode` | `milestone_loop` |
| `milestone` | M4 |
| `invariant` | Build structured, fixed-template analysis on supported chart-create output without luck cycles or generated prose. |

## 2. Decision Gate Audit

| Gate | Status | M4 Handling |
| --- | --- | --- |
| DG-005 | open | Luck cycles remain planned; no luck engine in M4. |
| DG-010 | closed | ADR 0012 forbids generative expansion; analysis is structured fixed-template only. |
| DG-008 | open for M9 | Keep Android date layer as accepted-current; no astronomy replacement. |

## 3. Selected Slice

Use one L-sized milestone loop for M4 analysis core:

| Work Package | Included? | Notes |
| --- | --- | --- |
| M4-WP1 hidden stems | yes | Use a fixed branch hidden-stem table with deterministic weights. |
| M4-WP2 ten gods | yes | Compute ten-god relation from day master to visible stems and hidden stems. |
| M4-WP3 five elements | yes | Count visible and hidden stem element weights. |
| M4-WP4 relation flags | partial | Start with transparent imbalance/sensitivity flags; deeper combinations can expand later. |
| M4-WP5 safe analysis DTO | yes | Return metrics, cards, sensitivity, disclaimer id. |
| M4-WP6 forbidden output checks | yes | Fixed audit rejects high-risk phrase classes. |
| M4-WP7 luck skeleton | no | DG-005 remains open; luck cycles stay planned. |

## 4. Explicit Non-Goals

- No luck cycles.
- No generated essay.
- No medical, legal, financial, death, fertility, criminality, coercive, or relationship certainty claims.
- No storage, sharing, frontend workspace, true solar time, timezone history, or astronomy replacement.

## 5. Validation

Required:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

M4 implementation may start after this preflight because DG-010 is closed and DG-005 is explicitly out of scope.

