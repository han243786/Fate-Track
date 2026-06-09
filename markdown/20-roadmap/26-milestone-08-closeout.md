# M8 Closeout: Validation and Release Candidate

## 1. Result

M8 is closed for the selected release-validation invariant:

```text
V1 release candidate freezes the existing supported/restricted/planned capability boundary and makes the validation path reproducible.
```

This closeout does not add a new backend business API. `release-candidate` is a governance and delivery capability backed by release notes, full gates, closeout evidence, and boundary checks.

## 2. Decision Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| DG-005 | open | Luck cycles remain planned in release notes, README, capability ledger, and `/api/capabilities`. |
| DG-008 | open for M9 | Astronomy engine and wider date-range replacement remain target scope only. |
| DG-006 | closed for M5 | Case/settings stay restricted local volatile. |
| DG-009 | closed for M6 | Share preview stays restricted local volatile, hash-only, redacted, and noindex. |

## 3. Capability Changes

| Capability | Before | After | Surface |
| --- | --- | --- | --- |
| `release-candidate` | planned | supported | `docs/release/v1-release-candidate.md`, `tools/check-release-candidate.ps1`, full project gate |
| all backend V1 capabilities | mixed | frozen | No status expansion beyond existing supported/restricted/planned labels |
| `luck-cycles` | planned | planned | DG-005 remains open |
| `astronomy-engine` | target | target | DG-008 and M9 remain future scope |

## 4. Implementation Evidence

| Work Package | Evidence |
| --- | --- |
| M8-WP1 full gates | `tools/check-project.ps1` includes Rust tests, frontend tests, governance scaffold, and release candidate checks. |
| M8-WP2 API contract freeze | `/api/capabilities`, README, `93-capability-promotion-ledger.md`, and release document freeze statuses. |
| M8-WP3 golden sample freeze | M1-M7 closeouts remain present; V1 date range remains 1901-2100. |
| M8-WP4 frontend key path | M7 browser evidence plus `frontend/tests/workspace-markup.test.mjs`. |
| M8-WP5 mobile/accessibility | M7 mobile browser evidence; static tests protect region headings and labeled controls. |
| M8-WP6 privacy/share review | Release checker verifies backend share redaction test names and forbidden private-state markers. |
| M8-WP7 release notes | `docs/release/v1-release-candidate.md`. |
| M8-WP8 downgrade | Release document records downgrade rules when evidence is lost or overclaim appears. |

## 5. Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

Latest green result:

- Rust: 51 tests passed.
- Frontend: 10 tests passed.
- Governance scaffold: OK.
- Release candidate check: OK.

Browser evidence:

- M7 desktop browser verification rendered 9 workspace panels and completed Save Case and Share Preview actions.
- M7 share preview browser verification showed no private note or snapshot id exposure.
- M7 mobile 390px browser verification had no horizontal overflow and retained chart content.
- M8 browser probe rendered 9 workspace panels, preserved Chart Workspace, Share Preview, and Capability Boundary, found no luck-cycle or durable-sharing supported claim, and confirmed mobile 390px had no horizontal overflow (`scrollWidth` 375, viewport width 390).
- M8 adds static markup and overclaim tests for release-level freeze; no new UI surface was introduced.

## 6. Regression Locks

- `release-candidate` must remain a governance/release capability, not a backend API business surface.
- Full project gate must continue to run the release-candidate checker.
- README, module tree, engineering tree, capability ledger, and release notes must stay synchronized with capability status.
- V1 release text must not claim luck cycles, durable sharing, accounts, cloud sync, true solar time, timezone history, wider date range, glossary, or astronomy replacement.
- If any supported/restricted capability loses evidence, the release candidate must be reopened or the capability downgraded before release.

## 7. Next Cursor

Proceed to M9 astronomy-upgrade preflight.

Required before M9 implementation:

- Read DG-008, ADR 0008, M8 closeout, release candidate document, and M9 milestone file.
- Do not replace the Android date-layer baseline until an ADR, generated data manifest, golden table, and full gate prove the replacement or parallel engine.
- Keep the V1 release candidate status frozen unless M9 explicitly supersedes it through governance.
