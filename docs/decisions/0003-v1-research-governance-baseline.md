# ADR 0003: V1 Research Reports as Governance Baseline

## Status

Accepted.

## Decision

The three reports under `markdown/reserch/` are accepted as the V1 planning baseline for product scope, calculation conventions, domain model, API target contract, frontend information architecture, privacy policy, and validation strategy.

The English reports have been translated into Chinese under `markdown/reserch/zh-CN/`, and the governance extraction is tracked in `markdown/reserch/00-research-intake.md`.

Research conclusions do not automatically mark a capability as supported. Runtime support is still governed by the capability table, implemented Rust code, tests, and module-tree registration.

## Source Reports

| Report ID | File | Role |
| --- | --- | --- |
| RPT-001 | `markdown/reserch/命轨 Fate-Track V1 产品需求与八字算法规格研究报告.md` | Chinese product and Bazi algorithm research |
| RPT-002 | `markdown/reserch/Fate-Track V1 Design Report.md` | Design, IA, architecture, safe interpretation |
| RPT-003 | `markdown/reserch/Fate-Track V1 Product Spec and Engineering Plan.md` | Product spec, API target, security, validation |

## Rationale

The reports converge on several high-risk themes:

- calendrical correctness must be deterministic and versioned;
- convention choices must be visible to users;
- birth data and chart outputs must be treated as sensitive;
- interpretation must remain structured and non-deterministic;
- unsupported or target capabilities must not be presented as available.

These themes fit the existing heavy-governance model and should become explicit policy, not informal notes.

## Consequences

- Future product, API, or algorithm changes must cite the relevant research report or ADR when they implement a researched target.
- Research-derived targets remain `planned` until backed by Rust/API tests and frontend behavior.
- The current Android date-layer port remains the accepted current baseline until an ephemeris-backed engine and replacement golden suite are implemented.
- English research reports are not edited in place; Chinese translations and governance extraction provide the working project surface.

