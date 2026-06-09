# M5 Preflight: Case Storage

## 1. Cursor

| Field | Value |
| --- | --- |
| `loop_id` | `LOOP-015` |
| `mode` | `milestone_loop` |
| `milestone` | M5 |
| `invariant` | Store local volatile cases, preferences, and immutable chart/analysis snapshots without cloud sync or sharing. |

## 2. Decision Gate Audit

| Gate | Status | M5 Handling |
| --- | --- | --- |
| DG-006 | closed for M5 | ADR 0013 restricts storage to local in-process volatile state. |
| DG-005 | open | Luck cycles remain planned; no luck storage in M5. |
| DG-008 | open for M9 | No astronomy replacement. |
| DG-009 | target-proposed for M6 | Sharing remains planned; no share tokens in M5. |

## 3. Selected Slice

Use one L-sized milestone loop:

| Work Package | Included? | Notes |
| --- | --- | --- |
| M5-WP1 storage model | yes | CaseRecord, immutable chart snapshot, immutable analysis snapshot, user preferences. |
| M5-WP2 snapshot versioning | yes | Store chart and analysis algo versions. |
| M5-WP3 case create/read/update/archive/delete | yes | Implement local volatile API operations. |
| M5-WP4 preferences read/update | yes | Local preference surface only. |
| M5-WP5 log/sensitive boundary | yes | API should not echo private note in list responses. |
| M5-WP6 storage strategy docs | yes | ADR 0013 documents volatile local strategy. |

## 4. Explicit Non-Goals

- No database persistence.
- No account.
- No cloud sync or cross-device sync.
- No public sharing or share tokens.
- No luck cycles.
- No generated analysis.

## 5. Validation

Required:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

M5 implementation may start after this preflight because DG-006 is closed for M5 and the selected invariant is explicit.

