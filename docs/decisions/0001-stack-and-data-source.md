# ADR 0001: Rust Backend, JavaScript Frontend, Lunar Data Source

## Status

Accepted.

## Decision

The project uses a Rust backend and a JavaScript frontend as its primary technical stack.

The first lunar-calendar source of truth is copied into the project at `data/raw/lunar_data.yaml` from `D:\myproject\Perpetual calendar\data\yaml\lunar_data.yaml`.

## Rationale

- Rust owns calculation services, API boundaries, validation, and future persistence.
- JavaScript owns browser UI, local interaction state, and API result projection.
- The lunar data must be governed as raw source data before derived tables, caches, or database imports are introduced.

## Consequences

- Backend capability claims must be backed by Rust code and tests.
- Frontend capability claims must be backed by backend API support.
- Raw data changes require provenance, regeneration commands, and validation evidence.

