# Fate Track V1 Release Candidate

## Status

All M0-M28 closed. Release candidate supported. V1 capability matrix: 10 supported, 7 restricted, 0 target, 0 planned. Boundary locked.

当前 post-preview 产品边界已经另行冻结在 `docs/release/current-product-boundary.md`；该文件记录当前运行时的 10 supported、14 restricted、0 planned 产品口径，不改写本 V1 preview release candidate 的历史矩阵。

## Validation Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## Capability Freeze

| Capability | Status | Evidence |
| --- | --- | --- |
| `health` | supported | Rust route + test |
| `lunar-data-meta` | supported | raw data reader + parse test |
| `calendar-date-query` | supported | Android date-layer port, 1901-2100 range, edge-case tests |
| `calendar-date-query-v1-meta` | supported | response metadata and `/api/capabilities` declaration |
| `chart-basis-preview` | restricted | M2 contract route and ADR 0009 |
| `chart-create` | supported | M3 chart engine core and tests |
| `chart-detail` | supported | M12 immutable snapshot and tests |
| `analysis-snapshot` | supported | M4 structured analysis and forbidden-output audit |
| `luck-cycles` | supported | M13 luck-cycle engine, ADR 0020, DG-005 closed |
| `case-management` | restricted | M5 local volatile case store and ADR 0013 |
| `settings` | restricted | M5 local volatile preferences |
| `share-preview` | restricted | M6 local volatile hash-only token and redacted DTO |
| `glossary` | supported | M14 55 structured terminology entries |
| `case-export` | restricted | M14 JSON export with optional notes |
| `data-derivation` | restricted | M15 aggregate statistics, >=5 threshold |
| `astronomy-engine` | supported | M11 engine (ADR 0019), M23 promotion (ADR 0021), 1598 Android comparison 0 diff |
| `chart-report` | restricted | M24 colloquial CN report, 9 blocks, hard-coded templates |
| `frontend-chart-workspace` | restricted | M7 workspace, M25 GPT Pro design, 3-column layout |
| `frontend-share-preview` | restricted | M7 redacted share preview panel |

## Boundary Freeze

- V1 official validated date range is `1901-2100`.
- Android date layer remains the accepted current baseline.
- Astronomy engine is supported as an independent verified computation capability (ADR 0021). Android date layer runtime replacement requires separate ADR.
- IANA timezone history, true solar time, and direct lunar input remain unsupported.
- Durable public sharing, accounts, cloud sync, and cross-device sync remain unsupported.
- Structured analysis remains deterministic and fixed-template only.
- Share preview is redacted and restricted; public DTOs must not expose private notes, raw titles, tags, private case ids, exact birth-time/location fields, or snapshot ids.
- Chart report is restricted; all text is hard-coded, no AI/LLM, no deterministic life claims.

## Release Evidence

- M1-M28 closeouts exist and are referenced by the recursive cursor/log.
- `tools/check-project.ps1` includes Rust tests, frontend tests, governance scaffold checks, and release-candidate checks.
- Desktop shell (M28) embeds frontend and backend into a single executable via Tao + Wry.

## Rollback And Downgrade

- If any supported or restricted capability loses test evidence, downgrade the capability before release.
- If any public surface leaks private data or overclaims unsupported scope, the release must not proceed.
- Astronomy engine runtime replacement must go through a separate ADR before Android baseline can be superseded.
