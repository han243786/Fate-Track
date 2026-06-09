# M10 Generated Astronomy Artifact Materialization Preflight

## 1. Scope

**Milestone**: M10 Generated Astronomy Implementation.
**Work Package**: M10-WP3 (write generated artifacts) / M10-WP4 (compute artifact hashes).
**Loop**: LOOP-055.

This preflight defines the boundary for the first generated astronomy artifact materialization. All four source-boundary payloads (`naif-cspice`, `iau-sofa-ansi-c`, `jpl-horizons-api`, `gb-t-33661-2017`) are now materialized and referenced in the source snapshot manifest. The next loop (LOOP-057) will write exactly the four planned generated artifacts and compute their sha256 hashes.

## 2. Preflight Boundary

### Allowed in this loop

- Define the next generated-artifact write boundary and planned artifact path set.
- Record source-payload prerequisites and materialized status.
- Add generated-artifact-materialization-preflight dry-run checker.
- Update the astronomy preflight checker, README, module tree, engineering tree, capability ledger, risk register, cursor, and closeout log.

### Forbidden in this loop

- Create the output directory `data/generated/astronomy/out/`.
- Write any generated astronomy artifact file.
- Compute any generated artifact sha256 hash.
- Mark the draft manifest `artifact_hashes.status` as present.
- Mark the draft manifest as accepted.
- Change calendar-date-query or chart-create runtime behavior.
- Replace the Android date layer as the runtime baseline.
- Promote `astronomy-engine` from target to restricted or supported.

## 3. Source Payload Prerequisites

| Source | Payload | sha256 | Status |
|--------|---------|--------|--------|
| `naif-cspice` | `naif-cspice-kernel-boundary.json` | `4c946457...` | materialized |
| `iau-sofa-ansi-c` | `iau-sofa-routine-version.json` | `436e197e...` | materialized |
| `jpl-horizons-api` | `jpl-horizons-validation-samples.json` | `acddbee9...` | materialized |
| `gb-t-33661-2017` | `gb-t-33661-2017-rule-reference.json` | `7145ecb9...` | materialized |

## 4. Planned Generated Artifacts

| Artifact | Kind | Status |
|----------|------|--------|
| `out/solar-terms-1901-2100.json` | solar-term-crossing-table | not_generated |
| `out/new-moons-1901-2100.json` | new-moon-table | not_generated |
| `out/lunar-calendar-1901-2100.json` | derived-chinese-calendar-table | not_generated |
| `out/android-comparison-1901-2100.json` | android-vs-astronomy-comparison | not_generated |

## 5. Next Loop Scope (LOOP-057)

After this preflight closes, LOOP-057 may:

- Create the `data/generated/astronomy/out/` directory.
- Write exactly the 4 planned generated artifacts as canonical UTF-8 JSON.
- Compute and record `sha256` hashes for each generated artifact.
- Record artifact hashes in the source payload materialization policy.

LOOP-057 must not:

- Mark the draft manifest as accepted.
- Change calendar-date-query or chart-create runtime behavior.
- Replace the Android date layer.
- Promote `astronomy-engine`.

## 6. Governance Sync

- `data/generated/astronomy/generated-artifact-materialization-preflight.json` — preflight evidence
- `tools/generated-artifact-materialization-preflight-dry-run.ps1` — dry-run checker
- `tools/check-astronomy-preflight.ps1` — integrated into astronomy preflight gate
- `data/generated/astronomy/README.md` — updated
- `markdown/00-matrix-governance/module-tree.md` — updated
- `markdown/10-overview/overview-full-feature-tree.md` — updated
- `markdown/20-roadmap/93-capability-promotion-ledger.md` — no change; `astronomy-engine` target
- `markdown/20-roadmap/92-risk-register.md` — risk entry added
- `README.md` — updated

## 7. Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools/generated-artifact-materialization-preflight-dry-run.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-astronomy-preflight.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```
