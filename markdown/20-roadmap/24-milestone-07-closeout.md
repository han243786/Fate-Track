# M7 Closeout: Frontend Workspace

## 1. Result

M7 is closed for the selected restricted frontend invariant:

```text
Frontend workspace consuming existing supported/restricted APIs for chart, analysis, local cases, redacted share preview, date-layer query, data metadata, and capability boundaries.
```

This is not a full product UI for luck cycles, durable sharing, accounts, cloud sync, glossary, true solar time, timezone history, or astronomy replacement.

## 2. Decision Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| DG-005 | open | Luck cycles remain planned and are displayed only via backend capability status. |
| DG-008 | open for M9 | No astronomy replacement or expanded date range. |
| DG-006 | closed for M5 | Case UI consumes restricted local volatile case APIs only. |
| DG-009 | closed for M6 | Share UI consumes restricted redacted share-preview API only. |

## 3. Capability Changes

| Capability | Before | After | Surface |
| --- | --- | --- | --- |
| `frontend-chart-workspace` | planned | restricted | Frontend chart/analysis/case workspace |
| `frontend-share-preview` | planned | restricted | Frontend redacted share preview panel |
| `frontend-date-layer-probe` | supported | supported | Existing date-layer panel retained |
| `glossary` | planned | planned | No frontend glossary success path |

## 4. Implementation Evidence

| Work Package | Evidence |
| --- | --- |
| M7-WP1 shell | `frontend/index.html`, `frontend/src/styles.css` |
| M7-WP2 chart input | `frontend/src/main.js`, `frontend/src/state.js`, DOM mapping |
| M7-WP3 chart workspace | `ApiClient.chartCreate`, `renderChart`, API client tests |
| M7-WP4 analysis cards | `ApiClient.analysisSnapshot`, `renderAnalysis`, API client tests |
| M7-WP5 case entry | `ApiClient.createCase`, `ApiClient.listCases`, case panel |
| M7-WP6 share preview | `ApiClient.createShare`, share panel |
| M7-WP7 calendar page | existing `calendarDate` route and date-layer panel retained |
| M7-WP9 responsive/accessibility | browser desktop/mobile checks; labeled controls and region headings |

## 5. Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Latest green result:

- Rust: 51 tests passed.
- Frontend: 8 tests passed.
- Governance scaffold: OK.

Browser verification:

- Desktop DOM rendered 9 workspace panels with chart, analysis, calendar, data, and capability content.
- Save Case and Share Preview actions completed from the frontend.
- Share preview did not expose private note or snapshot id.
- Mobile 390px viewport had no horizontal overflow (`scrollWidth` 375, viewport width 390) and retained chart content.

## 6. Regression Locks

- Frontend capability labels must reflect backend `/api/capabilities` or the capability ledger.
- Frontend must not implement chart/analysis/luck algorithms locally.
- Frontend share preview must remain redacted and restricted.
- UI must not claim luck cycles, durable public sharing, account storage, cloud sync, true solar time, timezone history, wider date range, or astronomy replacement.
- Glossary remains planned until backend evidence exists.

## 7. Next Cursor

Proceed to M8 validation release preflight.

Required before M8 implementation:

- Treat M8 as validation and release hardening, not feature expansion.
- Freeze supported/restricted/planned labels across API, UI, README, module tree, and capability ledger.
- Add or run release-level checks without weakening existing tests.
