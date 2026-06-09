# Fate Track V1 Release Candidate

## Status

Release candidate supported after M8 validation.

## Validation Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## Capability Freeze

| Capability | Status | Evidence |
| --- | --- | --- |
| `calendar-date-query` | supported | Android date-layer port, 1901-2100 range, edge-case tests |
| `calendar-date-query-v1-meta` | supported | response metadata and `/api/capabilities` declaration |
| `frontend-date-layer-probe` | supported | frontend date-layer panel and tests |
| `chart-basis-preview` | restricted | M2 contract route and ADR 0009 |
| `chart-create` | supported | M3 chart engine core and tests |
| `analysis-snapshot` | supported | M4 structured analysis and forbidden-output audit |
| `case-management` | restricted | M5 local volatile case store and ADR 0013 |
| `settings` | restricted | M5 local volatile preferences |
| `share-preview` | restricted | M6 local volatile hash-only token and redacted DTO |
| `frontend-chart-workspace` | restricted | M7 workspace consuming supported/restricted APIs |
| `frontend-share-preview` | restricted | M7 redacted share preview panel |
| `luck-cycles` | planned | DG-005 remains open |
| `glossary` | planned | content/source API remains planned |
| `astronomy-engine` | target | DG-008 and M9 remain future scope |

## Boundary Freeze

- V1 official validated date range is `1901-2100`.
- Android date layer remains the accepted current baseline.
- IANA timezone history, true solar time, direct lunar input, luck cycles, durable public sharing, accounts, cloud sync, cross-device sync, glossary, and astronomy replacement are not V1 supported capabilities.
- Structured analysis remains deterministic and fixed-template only.
- Share preview is redacted and restricted; public DTOs must not expose private notes, raw titles, tags, private case ids, exact birth-time/location fields, or snapshot ids.

## Release Evidence

- M1-M7 closeouts exist and are referenced by the recursive cursor/log.
- `tools/check-project.ps1` includes Rust tests, frontend tests, governance scaffold checks, and release-candidate checks.
- Browser verification in M7 closeout covered desktop render, save/share interaction, share redaction, and mobile 390px layout.
- M8 browser probe confirmed 9 workspace panels, capability boundary visibility, no luck-cycle/durable-sharing supported claim, and no mobile horizontal overflow at 390px.

## Rollback And Downgrade

- If any supported or restricted capability loses test evidence, downgrade the capability before release.
- If any public surface leaks private data or overclaims unsupported scope, M8 must not close.
- M9 astronomy work must not rewrite V1 release status until a later ADR and validation cycle replace or supersede this release candidate.
