# M6 Closeout: Share Privacy

## 1. Result

M6 is closed for the selected restricted invariant:

```text
Local volatile share previews with hash-only tokens, expiration, revocation, noindex, non-editable public DTOs, and redaction from immutable M5 snapshots.
```

This is not durable public sharing. It does not include accounts, database storage, cloud sync, cross-device sync, public directories, comments, analytics, luck cycles, generated analysis, or permanent public links.

## 2. Decision Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| DG-009 | closed for M6 | ADR 0014 selects hash-only local volatile tokens, expiration, revocation, noindex, and redacted public DTOs. |
| DG-006 | closed for M5 | M6 does not add persistence, accounts, cloud sync, or cross-device sync. |
| DG-005 | open | Luck cycles remain planned and are not included in share DTOs. |
| DG-008 | open for M9 | No astronomy replacement. |

## 3. Capability Changes

| Capability | Before | After | Route |
| --- | --- | --- | --- |
| `share-preview` | planned | restricted | `GET /api/share/preview` |
| `case-management` | restricted | restricted | `GET /api/cases` |
| `settings` | restricted | restricted | `GET /api/settings` |
| `luck-cycles` | planned | planned | `GET /api/luck/cycles` |

## 4. Implementation Evidence

| Work Package | Evidence |
| --- | --- |
| M6-WP1 share record | `backend/src/domain/share.rs` |
| M6-WP2 token hash | `ShareRepository::create`, `hash_token`, domain tests |
| M6-WP3 public DTO | `backend/src/api/share.rs` redacted public JSON |
| M6-WP4 preview/public consistency | create response embeds redacted DTO; public read returns same DTO version |
| M6-WP5 expired/revoked response | repository tests and app revoke/public tests |
| M6-WP6 noindex/non-editable | app tests assert `noindex:true` and `editable:false` |

## 5. Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Latest green result:

- Rust: 51 tests passed.
- Frontend: 6 tests passed.
- Governance scaffold: OK.

## 6. Regression Locks

- Public share DTOs must not expose `private_note`, raw title, tags, private case id, or snapshot id.
- Raw token must be returned only in the create response; stored records keep only `token_hash`.
- Missing, expired, invalid, and revoked tokens must use the same unavailable response shape.
- Public DTOs must remain `noindex:true` and `editable:false`.
- `share-preview` remains `restricted`, not `supported`, until durable public sharing policy and storage are separately decided.

## 7. Next Cursor

Proceed to M7 frontend workspace preflight.

Required before M7 implementation:

- Read M6 closeout and ADR 0014.
- Consume only supported/restricted backend APIs.
- Do not make frontend UI claim luck cycles, durable sharing, cloud sync, account storage, true solar time, timezone history, or astronomy replacement.
- Keep share UI read-only and redacted unless a later milestone changes the capability status.
