# M9 Replay-Test Materialization Readiness Evidence

## 1. Scope

This evidence belongs to LOOP-036. It adds readiness planning for future replay-test materialization.

It does not execute replay tests, does not recompute old snapshots with the astronomy engine, does not change default runtime behavior, and does not create accepted evidence.

## 2. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/replay-test-readiness-plan.json` | readiness only | Defines prerequisites and readiness controls before replay tests may be materialized. |
| `tools/replay-policy-dry-run.ps1` | dry-run only | Reports replay policy controls and readiness controls with zero replay tests executed. |

## 3. Dry-Run Result

- mode: `replay_policy_dry_run_only`
- replay test readiness plan: `m9-replay-test-materialization-readiness-v1`
- Android algorithm version: `android-date-layer-v1`
- Android ruleset id: `ft-date-layer-android-v1`
- readiness controls: 5
- replay tests executed: 0
- writes performed: false
- accepted evidence: false
- replacement allowed: false

## 4. Validation

`tools/check-astronomy-preflight.ps1` verifies readiness-only status, Android baseline bindings, replay prerequisites, forbidden readiness-stage actions, and zero-test dry-run output.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 5. Next Work

LOOP-037 should decide whether M9 can close as a preflight milestone or must continue into real generated-data implementation. It must not promote `astronomy-engine` without generated artifacts, hashes, comparison report, golden rows, and replay tests.
