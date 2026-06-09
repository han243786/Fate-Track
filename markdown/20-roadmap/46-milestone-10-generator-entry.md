# M10 Generator Implementation Entry Evidence

## 1. Scope

This evidence belongs to LOOP-038. It adds the first guarded M10 generator implementation entrypoint.

The entrypoint is non-dry-run in shape because it can be invoked with `-PrepareImplementation` instead of `-DryRun`. It still performs no writes, computes no hashes, accepts no artifacts, changes no runtime behavior, and does not promote `astronomy-engine`.

## 2. Artifacts

| Artifact | Status | Purpose |
| --- | --- | --- |
| `data/generated/astronomy/generator-implementation-entry.json` | guarded entrypoint only | Defines M10-WP1 guard flags, missing prerequisites, and no-write result policy. |
| `tools/generate-astronomy-tables.ps1 -PrepareImplementation` | guarded entrypoint only | Exercises the first non-dry-run implementation path while blocking materialization. |

## 3. Guarded Entrypoint Result

The M10 entry command is:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\generate-astronomy-tables.ps1 -ProjectRoot . -Manifest data/generated/astronomy/manifests/astronomy-engine-v0-draft.json -PrepareImplementation
```

Expected result:

- mode: `implementation_entry_guarded`
- dry_run: false
- source snapshot manifest exists: true after LOOP-040, but metadata only
- generation blocked: true
- writes performed: false
- hashes computed: 0
- acceptance status changed: false
- runtime behavior changed: false

## 4. Blockers Preserved

The entrypoint remains blocked until:

1. Source payloads are materialized.
2. Local generation adapter exists.
3. Planned artifacts can be generated deterministically.
4. `sha256` hashes can be computed.
5. Android-vs-astronomy comparison evidence exists.
6. Golden rows exist.
7. Replay tests exist.
8. Replacement ADR exists before runtime replacement.

## 5. Validation

`tools/check-astronomy-preflight.ps1` invokes both the M9 dry-run path and the M10 guarded entrypoint. The checker fails if the guarded entrypoint writes files, computes hashes, changes acceptance status, changes runtime behavior, or drops `astronomy-engine` from target.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```

## 6. Next Work

LOOP-039 should define the source snapshot manifest boundary for M10-WP2. It must not write generated astronomy artifacts until the source snapshot manifest and local adapter evidence are available.
