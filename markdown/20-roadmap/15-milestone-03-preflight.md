# M3 Preflight: Chart Engine

## 1. Cursor

| Field | Value |
| --- | --- |
| `loop_id` | `LOOP-012` |
| `mode` | `milestone_loop` |
| `milestone` | M3 |
| `invariant` | Build chart engine on top of M2 ChartBasis and M1 Android date layer without changing HTTP architecture. |

## 2. Decision Gate Audit

| Gate | Status | M3 Handling |
| --- | --- | --- |
| DG-001 | closed | Use `ft-v1-default`. |
| DG-003 | closed | Use local civil midnight `00:00`; Zi-start remains planned. |
| DG-004 | closed | Gregorian input only; lunar input remains unsupported. |
| DG-007 | closed for M3 | Keep current HTTP skeleton; no Axum/multi-crate migration in M3. |
| DG-008 | open for M9 | Keep Android date layer as accepted-current; no astronomy replacement in M3. |

## 3. Selected Slice

Use one L/XL milestone loop for the first M3 implementation slice:

| Work Package | Included? | Notes |
| --- | --- | --- |
| M3-WP1 year pillar | yes | Reuse Android date-layer year Gan-Zhi; carry ruleset metadata and boundary notes. |
| M3-WP2 month pillar | yes | Reuse Android fixed solar-term month starts; carry limitation metadata. |
| M3-WP3 day pillar | yes | Reuse Android day Gan-Zhi epoch; preserve edge-case tests. |
| M3-WP4 hour pillar | yes | Implement civil two-hour branch and five-rat stem derivation for exact time only. |
| M3-WP5 unknown hour | yes | Return `hour: null` plus candidate/sensitivity summary; never fabricate noon. |
| M3-WP6 API metadata | yes | Return ruleset id, algo version, warnings, ambiguity flags, and unsupported feature list. |
| M3-WP7 golden samples | partial | Start with stable API/domain tests; broader Lichun/Qingming/2033 expansion can remain for later M3 loops if needed. |

## 4. Explicit Non-Goals

- No IANA timezone-history resolution.
- No true solar time.
- No astronomy/ephemeris replacement.
- No analysis snapshot.
- No luck cycles.
- No case persistence.
- No frontend chart workspace unless a later M3/M7 slice explicitly selects it.

## 5. Required Code Landing Zones

| Zone | Purpose |
| --- | --- |
| `backend/src/domain/bazi.rs` | Chart structs, hour pillar, unknown-hour policy, chart metadata. |
| `backend/src/api/chart_basis.rs` or new chart API module | Chart route and JSON contract. |
| `backend/src/api/capabilities.rs` | Promote `chart-create` only when tests and governance evidence are complete. |
| `backend/src/app.rs` | API contract tests. |

## 6. Validation

Required:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

M3 implementation may start after this preflight because DG-007 is closed for M3 and the selected invariant is explicit.

