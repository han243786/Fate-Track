# M9 Replay-Policy Dry-Run Evidence

## 1. Scope

This evidence belongs to LOOP-028. It adds a replay-policy dry-run scaffold that reports the controls required before Android baseline replacement can be considered.

It does not execute replay tests, does not write files, does not replace `android-date-layer-v1`, and does not create accepted evidence.

## 2. Artifact

| Artifact | Status | Purpose |
| --- | --- | --- |
| `tools/replay-policy-dry-run.ps1` | dry-run only | Reads `replay-policy-draft.md` and the not-accepted manifest, then reports required replay controls without executing replay tests. |

## 3. Dry-Run Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\replay-policy-dry-run.ps1 -ProjectRoot . -Manifest data/generated/astronomy/manifests/astronomy-engine-v0-draft.json
```

## 4. Latest Dry-Run Result

- mode: `replay_policy_dry_run_only`
- required control count: 5
- replay tests executed: 0
- writes performed: false
- accepted evidence: false
- replacement allowed: false

## 5. Validation

`tools/check-astronomy-preflight.ps1` invokes this dry-run and fails if it reports executed replay tests, writes files, claims accepted evidence, allows replacement, or mismatches the required control count.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 6. Next Work

LOOP-029 may add an M9 final pre-closeout audit. It must not generate accepted astronomy artifacts or promote `astronomy-engine`.
