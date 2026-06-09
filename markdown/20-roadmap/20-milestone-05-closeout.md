# M5 Closeout: Case Storage

## 1. Result

M5 is closed for the selected restricted invariant:

```text
Local volatile cases, local volatile settings, and immutable chart/analysis snapshot references.
```

This is not full persistence. It does not include accounts, database storage, cloud sync, cross-device sync, public sharing, share tokens, luck cycles, or generated analysis.

## 2. Decision Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| DG-006 | closed for M5 | ADR 0013 selects local in-process volatile storage only. |
| DG-005 | open | Luck cycles remain planned and `/api/luck/cycles` still returns unsupported. |
| DG-008 | open for M9 | No astronomy replacement. |
| DG-009 | target-proposed for M6 | Share preview remains planned; no share token or public view in M5. |

## 3. Capability Changes

| Capability | Before | After | Route |
| --- | --- | --- | --- |
| `case-management` | planned | restricted | `GET /api/cases` |
| `settings` | planned | restricted | `GET /api/settings` |
| `share-preview` | planned | planned | `GET /api/share/preview` |
| `luck-cycles` | planned | planned | `GET /api/luck/cycles` |

## 4. Implementation Evidence

| Work Package | Evidence |
| --- | --- |
| M5-WP1 storage model | `backend/src/domain/cases.rs`, `backend/src/domain/settings.rs` |
| M5-WP2 immutable snapshot refs | `ChartSnapshot`, `AnalysisSnapshotRef`, metadata preservation tests |
| M5-WP3 case actions | `backend/src/api/cases.rs`, app tests for create/list/update/archive/delete |
| M5-WP4 preferences | `backend/src/api/settings.rs`, preference validation tests |
| M5-WP5 sensitive boundary | list response omits `private_note`; deleted cases are omitted |
| M5-WP6 storage strategy | ADR 0013 and README/module tree/capability ledger updates |

## 5. Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Latest green result:

- Rust: 46 tests passed.
- Frontend: 6 tests passed.
- Governance scaffold: OK.

## 6. Regression Locks

- Case list responses must not expose `private_note`.
- Metadata updates must not rewrite chart or analysis snapshot algorithm versions.
- Deleted cases must be omitted from normal reads and lists.
- `case-management` and `settings` remain `restricted`, not `supported`, until durable storage/account/privacy semantics are separately decided.
- Share preview must remain planned until M6 closes DG-009.

## 7. Next Cursor

Proceed to M6 share privacy preflight.

Required before M6 implementation:

- Read M5 closeout and ADR 0013.
- Close or scope DG-009 for share token storage, expiration, revocation, noindex, and public-view privacy.
- Keep DG-005 open unless luck cycles are explicitly selected.
- Do not add cloud sync, account storage, or cross-device persistence through sharing work.
