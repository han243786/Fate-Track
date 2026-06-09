# ADR 0010: HTTP Architecture Through Chart Engine

## Status

Accepted.

## Decision

Fate-Track will keep the current lightweight Rust HTTP skeleton through M3 chart-engine implementation.

This closes DG-007 for M3:

```text
No Axum or multi-crate migration before M3 chart engine.
```

## Rationale

M3 needs to stabilize chart calculation behavior, metadata, warnings, and regression vectors. Migrating the web framework before chart behavior is stable would add a second invariant to the milestone and increase rollback risk.

## Boundary

- The current HTTP skeleton may continue for M3-M8 if it remains sufficient.
- Axum or multi-crate migration may be revisited as a separate architecture ADR or during M9.
- API responses must keep the same JSON error envelope while the framework remains unchanged.

## Consequences

- M3 can focus on chart engine logic and API contracts.
- No code may claim Axum is in use until a migration ADR and implementation exist.
- `tools/check-project.ps1` remains the full gate for the current architecture.

