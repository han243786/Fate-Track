# M9 Generator Contract Evidence

## 1. Scope

This evidence belongs to LOOP-031. It defines the generator contract before any M9 generated astronomy rows exist.

The contract does not generate rows, does not accept artifacts, does not update runtime behavior, and does not promote `astronomy-engine`.

## 2. Contract Artifact

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/generator-contract.json` | contract only | Defines required inputs, planned outputs, canonical output encoding, `sha256` hash policy, manifest update rules, and forbidden contract-stage actions. |

## 3. Contract Guarantees

| Area | Rule |
| --- | --- |
| Inputs | Source policy, draft manifest, generation plan, and implementation plan are required. |
| Outputs | Planned outputs mirror `generation-plan.json` and remain `not_generated`. |
| Hashes | Every planned output requires `sha256`. |
| Manifest | `acceptance_status` must remain `not_accepted`; artifact hashes remain missing until real files exist. |
| Runtime | No date-layer runtime behavior may change in this stage. |

## 4. Dry-Run Integration

`tools/generate-astronomy-tables.ps1 -DryRun` now reads the generator contract and reports:

- `generator_contract_id`
- `hash_algorithm`
- planned artifact count
- existing planned artifacts
- writes performed: false
- acceptance status changed: false

## 5. Validation

`tools/check-astronomy-preflight.ps1` verifies the generator contract, the implementation-plan stage status, dry-run output, `sha256` requirement, planned output count, required inputs, and forbidden contract-stage actions.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 6. Next Work

LOOP-032 should define the M9 source-adapter contract. It must still avoid generated rows and accepted artifacts.
