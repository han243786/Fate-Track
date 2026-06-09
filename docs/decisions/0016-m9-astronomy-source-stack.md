# ADR 0016: M9 Astronomy Source Stack

## Status

Accepted for source/tooling planning.

## Decision

M9 will use a staged source stack:

| Layer | Selected Source | Role | Runtime Status |
| --- | --- | --- | --- |
| Calendar rule reference | GB/T 33661-2017 | Modern Chinese calendar rule reference for lunar compilation, UTC+8 baseline, and calendar publication governance | reference only |
| Online validation source | NASA/JPL Horizons API | Programmatic ephemeris query source for source availability probes and future validation samples | external validation source |
| Standards routines | IAU SOFA ANSI C | Target standards library for time-scale and fundamental astronomy routines | target dependency only |
| Offline reproducibility candidate | NAIF CSPICE / SPICE kernels | Future offline kernel workflow for reproducible generated tables | target dependency only |

The first generated-table path must start with a source policy and a reproducible manifest. It must not claim `astronomy-engine` support until actual generated artifacts, hashes, comparison reports, and golden tests exist.

## Official Source Evidence

| Source | Evidence |
| --- | --- |
| GB/T 33661-2017 | National standard page marks the Chinese calendar calculation standard as current and reviewed on 2023-12-28. |
| NASA/JPL Horizons API | Official API documentation exposes `https://ssd.jpl.nasa.gov/api/horizons.api` and version 1.3 dated 2025 June. |
| IAU SOFA | SOFA provides authoritative algorithms and an ANSI C software collection; latest release shown as 2023-10-11. |
| NAIF SPICE Toolkit | Official toolkit page lists CSPICE/other toolkit packages and current Toolkit N0067. |

## Required Source Policy Fields

- `source_policy_id`
- `calendar_standard`
- `online_validation_source`
- `standards_routine_source`
- `offline_reproducibility_source`
- `first_generated_range`
- `required_golden_categories`
- `forbidden_until_generated`

## Rejected Options

| Option | Reason |
| --- | --- |
| Use Horizons API as production runtime dependency | Network availability and external API drift would make chart results non-reproducible. |
| Use only SOFA without ephemeris validation | SOFA provides standards routines but does not by itself prove Chinese-calendar event tables. |
| Use SPICE immediately inside Rust runtime | Native toolkit integration is higher risk than source-policy and generated-table planning. |
| Use research citations as generated evidence | Citations guide source selection but do not provide hashes, generated rows, or diff reports. |

## Rollback Rule

If any selected source becomes unavailable or fails validation, keep Android date-layer output accepted-current and keep `astronomy-engine` as target while a replacement source ADR is written.
