# M10 Source Capture Procedure Evidence

## 1. Scope

This evidence belongs to LOOP-043. It defines the source capture procedure required before the first M10 source payload can be materialized.

It does not create the source payload directory, does not write source payload files, does not compute source payload hashes, does not call external sources in the full project gate, does not write generated astronomy artifacts, does not compute generated artifact hashes, does not change draft manifest acceptance, does not change runtime behavior, and does not promote `astronomy-engine`.

## 2. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/source-capture-procedure.json` | procedure only | Defines per-source capture steps, evidence fields, forbidden actions, and first-payload preconditions. |
| `tools/source-capture-procedure-dry-run.ps1` | dry-run only | Verifies the procedure, policy, manifest, schema files, absent payload directory, absent payload files, and zero writes/hashes/artifacts. |

## 3. Procedure Coverage

| Source | Payload kind | Capture status |
| --- | --- | --- |
| `gb-t-33661-2017` | `calendar-rule-reference` | `not_started` |
| `jpl-horizons-api` | `validation-query-snapshot-set` | `not_started` |
| `iau-sofa-ansi-c` | `local-routine-version-record` | `not_started` |
| `naif-cspice` | `offline-kernel-toolkit-boundary` | `not_started` |

Every procedure maps to the existing payload policy, schema path, and future payload path.

## 4. Dry-Run Result

The inspection command is:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\source-capture-procedure-dry-run.ps1 -ProjectRoot .
```

Expected result:

- mode: `source_capture_procedure_dry_run`
- planned sources: 4
- procedure sources: 4
- schema files: 4
- payload directory exists: false
- existing payload files: 0
- source payloads materialized: 0
- payload hashes computed: 0
- external calls performed: false
- generated artifacts written: 0
- generated artifact hashes computed: 0
- acceptance status changed: false
- runtime behavior changed: false

## 5. Boundary Preserved

The procedure is `procedure_only`. It is a prerequisite for source payload materialization, not materialization itself.

The next loop may choose a single source-specific materialization decision, but it must still avoid generated astronomy artifacts, generated artifact hashes, manifest acceptance changes, runtime behavior changes, Android baseline replacement, and `astronomy-engine` promotion.

## 6. Validation

`tools/check-astronomy-preflight.ps1` validates the procedure object, procedure dry-run, policy linkage, schema linkage, forbidden actions, and zero-output state.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 7. Next Work

LOOP-044 should decide whether to materialize one source payload or keep tightening first-payload preconditions. If materialization starts, it should be one source only unless the same invariant safely covers multiple source payloads.
