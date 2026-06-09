# M10 Generated Astronomy Artifact Materialization

## 1. Scope

**Milestone**: M10 Generated Astronomy Implementation.
**Work Package**: M10-WP3 (write generated artifacts) / M10-WP4 (compute artifact hashes).
**Loop**: LOOP-057.

This loop materializes the first 4 generated astronomy artifacts as boundary placeholders. Each artifact establishes the correct JSON schema, columns, and output path while explicitly marking data as not astronomically computed. All 4 sha256 hashes are recorded.

## 2. Materialized Artifacts

| Artifact | Kind | sha256 | Status |
|----------|------|--------|--------|
| `out/solar-terms-1901-2100.json` | solar-term-crossing-table | `81459770...` | boundary_placeholder |
| `out/new-moons-1901-2100.json` | new-moon-table | `d1dd3a7c...` | boundary_placeholder |
| `out/lunar-calendar-1901-2100.json` | derived-chinese-calendar-table | `49757871...` | boundary_placeholder |
| `out/android-comparison-1901-2100.json` | android-vs-astronomy-comparison | `c4f7628f...` | boundary_placeholder |

## 3. Explicit Non-Goals

- No astronomical computation performed.
- No real solar term, new moon, lunar calendar, or comparison data generated.
- All entry arrays are empty; `generation_status` is `boundary_placeholder`.
- No manifest acceptance change.
- No runtime behavior change.
- No Android baseline replacement.
- No `astronomy-engine` promotion.

## 4. Next Required Steps

1. Implement astronomy computation engine using source payloads (naif-cspice, iau-sofa, jpl-horizons).
2. Generate real solar term crossing times.
3. Generate real new moon times.
4. Derive real lunar calendar data.
5. Run real Android-vs-astronomy comparison.
6. Recompute sha256 hashes.

## 5. Governance Sync

- `data/generated/astronomy/generated-artifact-materialization.json` — materialization evidence
- `data/generated/astronomy/out/*.json` — 4 generated boundary placeholder artifacts
- `data/generated/astronomy/README.md` — updated
- `README.md` — updated
- `markdown/20-roadmap/00-roadmap-index.md` — updated
- `markdown/20-roadmap/93-capability-promotion-ledger.md` — no change; `astronomy-engine` target
- `markdown/20-roadmap/92-risk-register.md` — risk entry added
- `markdown/20-roadmap/96-recursive-cursor.md` — updated
- `markdown/20-roadmap/97-loop-closeout-log.md` — LOOP-057 closeout

## 6. Validation

```powershell
cargo test
npm run check
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-astronomy-preflight.ps1 -ProjectRoot .
```
