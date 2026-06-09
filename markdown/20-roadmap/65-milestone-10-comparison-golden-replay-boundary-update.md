# M10 Comparison, Golden, Replay Boundary Update

## 1. Scope

**Milestone**: M10 Generated Astronomy Implementation.
**Work Package**: M10-WP5/M10-WP6/M10-WP7.
**Loop**: LOOP-058.

This loop updates the comparison runner plan, golden row readiness plan, and replay test readiness plan to acknowledge that generated artifacts exist as boundary placeholders. No real comparison, golden rows, or replay tests are executed.

## 2. Updated Plans

| Plan | Change | Status |
|------|--------|--------|
| `comparison-runner-plan.json` | Added `generated_artifact_status: boundary_placeholder` and materialization reference | dry_run_only |
| `golden-row-readiness-plan.json` | Added `generated_artifact_status: boundary_placeholder` and materialization reference | readiness_only |
| `replay-test-readiness-plan.json` | Added `generated_artifact_status: boundary_placeholder` and materialization reference | readiness_only |

## 3. Explicit Non-Goals

- No comparison rows generated.
- No golden rows materialized.
- No replay tests executed.
- No manifest acceptance change.
- No runtime behavior change.
- No Android baseline replacement.
- No `astronomy-engine` promotion.

## 4. Blockers Remaining

All three downstream work packages (M10-WP5 manifest, M10-WP6 comparison, M10-WP7 golden/replay) require real astronomically computed data. The astronomy computation engine must be implemented before:

- Comparison can produce meaningful difference rows.
- Golden rows can be materialized and verified.
- Replay tests can execute against real astronomy output.

## 5. Governance Sync

- `data/generated/astronomy/comparison-runner-plan.json` — updated
- `data/generated/astronomy/golden-row-readiness-plan.json` — updated
- `data/generated/astronomy/replay-test-readiness-plan.json` — updated
- `markdown/20-roadmap/65-milestone-10-comparison-golden-replay-boundary-update.md` — milestone evidence
- `markdown/20-roadmap/96-recursive-cursor.md` — updated
- `markdown/20-roadmap/97-loop-closeout-log.md` — LOOP-058 closeout
