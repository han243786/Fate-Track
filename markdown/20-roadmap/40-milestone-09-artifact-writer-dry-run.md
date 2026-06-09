# M9 Artifact Writer Dry-Run Evidence

## 1. Scope

This evidence belongs to LOOP-033. It adds artifact-writer dry-run planning for the M9 generated-data path.

It does not create the output directory, does not write artifact files, does not compute hashes for nonexistent files, does not update manifest hashes, and does not create accepted evidence.

## 2. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/artifact-writer-plan.json` | dry-run only | Defines output directory, planned artifact paths, hash algorithm, and dry-run forbidden actions. |
| `tools/artifact-writer-dry-run.ps1` | dry-run only | Reports planned artifact write/hash preview without writing files. |

## 3. Dry-Run Result

- mode: `artifact_writer_dry_run_only`
- planned artifact count: 4
- output directory exists: false
- writes performed: false
- hashes computed: 0
- accepted evidence: false

## 4. Validation

`tools/check-astronomy-preflight.ps1` executes the artifact writer dry-run and fails if it writes files, computes hashes, reports existing planned artifacts, claims accepted evidence, or mismatches the generator contract output count.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 5. Next Work

LOOP-034 should add comparison-runner dry-run planning. It must still avoid generated rows and accepted artifacts.
