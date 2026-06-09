# ADR 0017: M9 Generated-Data Implementation Path

## Status

Accepted for implementation planning.

## Context

LOOP-029 proved that M9 full closeout is blocked while preflight review is ready. The blocked items are generated astronomy tables, artifact hashes, a completed Android comparison report, generated golden rows, replay tests, and runtime integration.

The project still needs a path toward full M9 without promoting `astronomy-engine` from target or replacing `android-date-layer-v1`.

## Decision

Continue inside M9 with generated-data implementation planning.

The next stage must create implementation scaffolding in this order:

1. `generator-contract`: define deterministic input/output contract for generated tables.
2. `source-adapter-contract`: define how source references are converted into reproducible generation inputs.
3. `artifact-writer-dry-run`: prepare hash and output paths without writing accepted artifacts.
4. `comparison-runner-dry-run`: bind Android baseline rows to future astronomy rows without accepting differences.
5. `golden-row-materialization`: only after the generator contract is stable.
6. `replay-test-materialization`: only after generated rows and old-snapshot replay fixtures exist.

## Hard Boundaries

- No generated row may be accepted before hashes and manifest update rules exist.
- No Android baseline replacement is allowed in this stage.
- No `calendar-date-query` or `chart-create` runtime behavior changes are allowed in this stage.
- No wider date range, true solar time, or IANA timezone-history support may be claimed.
- `astronomy-engine` remains target until all full M9 acceptance blockers are cleared.

## Rejected Options

| Option | Reason |
| --- | --- |
| Close M9 as full astronomy-engine milestone | Contradicts `precloseout-audit.json`; generated artifacts and replay evidence are missing. |
| Split actual generated-data work out immediately | Premature split would leave the active M9 milestone without an implementation path. |
| Generate data before contract/hash rules | Would create artifacts that cannot be safely accepted or replayed. |
| Replace Android date layer after source planning | Violates ADR 0015 and replay policy. |

## Validation

`data/generated/astronomy/implementation-plan.json` must remain `planning_only` until a later loop implements the generator contract. The astronomy preflight checker must verify this ADR, the planning status, the selected path, and all hard boundaries.
