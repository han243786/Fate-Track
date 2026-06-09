# M10 First Source Payload Materialization Decision

## 1. Scope

This evidence belongs to LOOP-044. It selects the first M10 source payload candidate for a later materialization loop.

It does not create the source payload directory, does not write source payload files, does not compute source payload hashes, does not call external sources in the full project gate, does not write generated astronomy artifacts, does not compute generated artifact hashes, does not change draft manifest acceptance, does not change runtime behavior, and does not promote `astronomy-engine`.

## 2. Decision

| Field | Value |
| --- | --- |
| decision id | `m10-first-source-payload-materialization-decision-v1` |
| status | `decision_only` |
| selected source | `naif-cspice` |
| payload kind | `offline-kernel-toolkit-boundary` |
| future payload path | `data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json` |
| decision scope | `single_source_only` |

`naif-cspice` is selected first because the future payload can record offline toolkit/kernel boundary policy without online full-gate calls, runtime integration, generated astronomy output, or copied standard text.

## 3. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/source-payload-materialization-decision.json` | decision only | Selects the first source payload candidate and locks forbidden actions for this loop. |
| `tools/source-payload-materialization-decision-dry-run.ps1` | dry-run only | Verifies the decision, selected source linkage, absent payload directory, absent payload files, and zero writes/hashes/artifacts. |

## 4. Dry-Run Result

The inspection command is:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\source-payload-materialization-decision-dry-run.ps1 -ProjectRoot .
```

Expected result:

- mode: `source_payload_materialization_decision_dry_run`
- selected source: `naif-cspice`
- payload directory exists: false
- selected payload exists: false
- existing payload files: 0
- source payloads materialized: 0
- payload hashes computed: 0
- external calls performed: false
- generated artifacts written: 0
- generated artifact hashes computed: 0
- acceptance status changed: false
- runtime behavior changed: false

## 5. Boundary Preserved

This decision is not payload materialization. It only narrows the next stable slice.

LOOP-045 may update the payload directory policy and materialize exactly the selected `naif-cspice` source payload if the source evidence remains source-only and no generated astronomy artifact, manifest acceptance change, runtime behavior change, Android baseline replacement, or `astronomy-engine` promotion occurs.

## 6. Validation

`tools/check-astronomy-preflight.ps1` validates the decision object, selected-source linkage, decision dry-run, forbidden actions, and zero-output state.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```
