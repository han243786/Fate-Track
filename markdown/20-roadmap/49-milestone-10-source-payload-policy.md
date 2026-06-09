# M10 Source Payload Materialization Policy Evidence

## 1. Scope

This evidence belongs to LOOP-041. It defines the source payload materialization policy for M10-WP2.

It does not create the source payload directory, does not write source payload files, does not compute source payload hashes, does not write generated astronomy artifacts, does not compute generated artifact hashes, does not change draft manifest acceptance, does not change runtime behavior, and does not promote `astronomy-engine`.

## 2. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/source-payload-materialization-policy.json` | payload policy only | Defines future per-source payload paths, formats, and materialization blockers. |
| `tools/source-payload-materialization-dry-run.ps1` | dry-run only | Verifies that the payload directory and payload files are still absent. |

## 3. Dry-Run Result

The inspection command is:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\source-payload-materialization-dry-run.ps1 -ProjectRoot .
```

Expected result:

- mode: `source_payload_materialization_policy_dry_run`
- planned payloads: 4
- payload directory exists: false
- existing payload files: 0
- source payloads materialized: 0
- payload hashes computed: 0
- generated artifacts written: 0
- generated artifact hashes computed: 0
- acceptance status changed: false
- runtime behavior changed: false

## 4. Payload Boundary

The future payloads are:

1. GB/T 33661 rule reference.
2. JPL Horizons validation sample set.
3. IAU SOFA routine version record.
4. NAIF CSPICE toolkit and kernel boundary.

Every planned payload remains `not_materialized`, with `hash_status=not_computed`, `runtime_dependency=false`, and `output_claim_allowed=false`.

## 5. Validation

`tools/check-astronomy-preflight.ps1` validates the policy, planned payload coverage, dry-run output, no payload files, no hashes, no generated artifacts, and no capability promotion.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 6. Next Work

LOOP-042 should define the per-source payload schemas or materialize the first source payload only if it can remain source evidence, not generated astronomy output.
