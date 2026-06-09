# M10 Source Payload Schema Evidence

## 1. Scope

This evidence belongs to LOOP-042. It defines per-source payload schemas for M10-WP2.

It does not create the source payload directory, does not write source payload files, does not compute source payload hashes, does not write generated astronomy artifacts, does not compute generated artifact hashes, does not change draft manifest acceptance, does not change runtime behavior, and does not promote `astronomy-engine`.

## 2. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `source-payload-schemas/gb-t-33661-2017-rule-reference.schema.json` | schema only | Defines the future GB/T rule-reference payload shape. |
| `source-payload-schemas/jpl-horizons-validation-samples.schema.json` | schema only | Defines the future JPL Horizons validation sample payload shape. |
| `source-payload-schemas/iau-sofa-routine-version.schema.json` | schema only | Defines the future SOFA routine-version payload shape. |
| `source-payload-schemas/naif-cspice-kernel-boundary.schema.json` | schema only | Defines the future NAIF CSPICE toolkit/kernel boundary payload shape. |

## 3. Dry-Run Result

The inspection command is:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\source-payload-materialization-dry-run.ps1 -ProjectRoot .
```

Expected result:

- mode: `source_payload_materialization_policy_dry_run`
- planned payloads: 4
- schema files: 4
- payload directory exists: false
- existing payload files: 0
- source payloads materialized: 0
- payload hashes computed: 0
- generated artifacts written: 0
- generated artifact hashes computed: 0
- acceptance status changed: false
- runtime behavior changed: false

## 4. Boundary Preserved

Every schema is `schema_only`, and every planned payload remains `not_materialized` with `hash_status=not_computed`, `runtime_dependency=false`, and `output_claim_allowed=false`.

## 5. Validation

`tools/check-astronomy-preflight.ps1` validates schema existence, schema/source/kind matching, required common fields, forbidden claims, dry-run output, no payload files, no hashes, no generated artifacts, and no capability promotion.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 6. Next Work

LOOP-043 should decide whether to materialize the first source payload or add capture procedure docs. It must keep the result as source evidence only, not generated astronomy output.
