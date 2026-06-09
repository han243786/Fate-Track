# M6 Preflight: Share Privacy

## 1. Cursor

| Field | Value |
| --- | --- |
| `loop_id` | `LOOP-016` |
| `mode` | `milestone_loop` |
| `milestone` | M6 |
| `invariant` | Create restricted local volatile share previews from immutable M5 snapshots without exposing private case state. |

## 2. Decision Gate Audit

| Gate | Status | M6 Handling |
| --- | --- | --- |
| DG-009 | closed for M6 | ADR 0014 selects local volatile hash-only tokens, expiration, revocation, noindex, and redacted public DTOs. |
| DG-006 | closed for M5 | M6 preserves local volatile boundaries and does not add persistence, accounts, cloud sync, or cross-device sync. |
| DG-005 | open | Luck cycles remain planned; no luck data is added to share DTOs. |
| DG-008 | open for M9 | No astronomy replacement. |

## 3. Selected Slice

Use one L-sized milestone loop:

| Work Package | Included? | Notes |
| --- | --- | --- |
| M6-WP1 share preset/expiry/revoke model | yes | Implemented as local volatile `ShareRecord`. |
| M6-WP2 token generation and hash storage | yes | Raw token returned only on create; repository stores `token_hash`. |
| M6-WP3 public share DTO | yes | Redacted DTO omits private note, title, tags, case id, and snapshot id. |
| M6-WP4 preview/public consistency | yes | Create response embeds the same redacted DTO shape as public read. |
| M6-WP5 expired/revoked response | yes | Missing, invalid, expired, and revoked tokens use the same unavailable response. |
| M6-WP6 noindex/non-editable policy | yes | Public DTO returns `noindex:true` and `editable:false`. |

## 4. Explicit Non-Goals

- No database persistence.
- No account or ownership model.
- No cloud sync or cross-device sync.
- No public directory, comments, analytics, or community surface.
- No luck cycles.
- No generated analysis.

## 5. Validation

Required:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```
