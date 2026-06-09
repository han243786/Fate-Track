# M9 Generated-Data Implementation Plan Evidence

## 1. Scope

This evidence belongs to LOOP-030. It records the decision to continue M9 into generated-data implementation planning without generating rows, accepting artifacts, or promoting `astronomy-engine`.

## 2. Decision

ADR 0017 chooses `continue_m9_generated_data_planning`.

This means M9 does not close as a full astronomy-engine milestone yet, but it also does not split away from generated-data work before a concrete generator contract is defined.

## 3. Machine Plan

`data/generated/astronomy/implementation-plan.json` records:

- status: `planning_only`
- decision: `continue_m9_generated_data_planning`
- capability status: `target`
- next loop: `LOOP-031`
- next work package: `M9 generator contract planning`

## 4. Planned Stage Order

| Stage | Status | Purpose |
| --- | --- | --- |
| `generator-contract` | next | Define deterministic inputs, outputs, manifest updates, and hash requirements before any generated row exists. |
| `source-adapter-contract` | planned | Define how selected sources map to reproducible generation inputs. |
| `artifact-writer-dry-run` | planned | Prepare output paths and hash calculation without accepting artifacts. |
| `comparison-runner-dry-run` | planned | Bind Android baseline rows to future astronomy rows without accepting differences. |
| `golden-row-materialization` | blocked | Materialize golden rows only after the generator contract is stable. |
| `replay-test-materialization` | blocked | Prove old Android snapshots remain reproducible only after generated rows exist. |

## 5. Forbidden In This Loop

- No generated artifact acceptance.
- No Android baseline replacement.
- No `calendar-date-query` runtime change.
- No `chart-create` runtime change.
- No `astronomy-engine` supported claim.
- No wider date range, true solar time, or timezone-history support claim.

## 6. Validation

`tools/check-astronomy-preflight.ps1` verifies ADR 0017, `implementation-plan.json`, stage coverage, planning-only status, target capability status, and forbidden runtime changes.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 7. Next Work

LOOP-031 should implement the M9 generator contract planning surface. It must still avoid generated rows and accepted artifacts until manifest update and hash rules are defined.
