# M8 Preflight: Validation and Release Candidate

## 1. Cursor

| Field | Value |
| --- | --- |
| `loop_id` | `LOOP-018` |
| `mode` | `milestone_loop` |
| `milestone` | M8 |
| `invariant` | Freeze the V1 supported/restricted/planned capability boundary and make release validation reproducible. |

## 2. Decision Gate Audit

| Gate | Status | M8 Handling |
| --- | --- | --- |
| DG-005 | open | Luck cycles remain planned and must not appear as V1 supported. |
| DG-008 | open for M9 | Astronomy engine, true solar time, timezone history, and range expansion remain future scope. |
| DG-006 | closed for M5 | Case/settings remain restricted local volatile surfaces. |
| DG-009 | closed for M6 | Share preview remains restricted local volatile redacted sharing only. |

## 3. Selected Slice

Use one M8 release-validation loop:

| Work Package | Included? | Notes |
| --- | --- | --- |
| M8-WP1 full gates | yes | Run `tools/check-project.ps1`, which includes Rust, frontend, governance scaffold, and release-candidate checks. |
| M8-WP2 API contract freeze | yes | Freeze supported/restricted/planned statuses through `/api/capabilities`, README, and the capability ledger. |
| M8-WP3 golden sample freeze | yes | Preserve Android date-layer 1901-2100 boundary and M1-M7 closeout evidence. |
| M8-WP4 frontend key path E2E | yes | Reuse M7 browser evidence and run release-level markup/overclaim tests. |
| M8-WP5 mobile/accessibility | yes | Keep M7 mobile/browser evidence and add static labeled-control tests. |
| M8-WP6 privacy/log/share review | yes | Release checker verifies backend share privacy tests and share boundary text. |
| M8-WP7 README/release notes/closeout | yes | Add `docs/release/v1-release-candidate.md` and M8 closeout. |
| M8-WP8 rollback/downgrade | yes | Release document records downgrade rules for lost evidence or overclaim. |

## 4. Explicit Non-Goals

- No new business capability.
- No durable public sharing.
- No account, database, cloud sync, or cross-device persistence.
- No luck cycles, glossary success path, true solar time, timezone history, or astronomy replacement.
- No weakening of existing tests or closeout evidence.

## 5. Validation

Required:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Also required:

- Release candidate document exists and records V1 boundaries.
- Release checker is included in the full gate.
- Capability ledger promotes only `release-candidate` as a governance/release surface.
- Recursive cursor advances to M9 only after the full gate is green.
