# ADR 0019: M11 Astronomy Engine Architecture

## Status

Accepted.

## Decision

M11 implements a pure-Rust astronomy computation engine using:

- **VSOP87** truncated series for solar position (Earth heliocentric longitude/latitude/radius).
- **Simplified lunar theory** (Meeus Chapter 47, ~60 periodic terms) for lunar longitude and latitude.
- **Newton-Raphson iteration** for solar term crossing and new moon conjunction time finding.
- **Morrison & Stephenson (2004)** polynomial for ΔT estimation over 1901-2100.
- **IAU SOFA** conventions for time-scale conversions (JD, TT, TAI, UTC) — algorithm ported, not C library linked.
- **GB/T 33661-2017** rule reference for leap-month placement and month numbering.

All computation is deterministic and reproducible — no network calls, no runtime external dependencies.

## Rationale

- Route A (pure Rust) preserves the project's zero-external-dependency constraint.
- VSOP87 truncated to ~300 periodic terms per coordinate yields <1 arcminute solar accuracy, sufficient for solar term timing to ±1 minute.
- Simplified lunar theory with ~60 terms yields ~10 arcminute lunar accuracy, sufficient for new moon timing to ±5 minutes.
- Accuracy is validated against JPL Horizons offline snapshots (already materialized as source boundary payload).
- Full SPICE/SOFA integration remains a future precision upgrade path.

## Module Structure

```
backend/src/astronomy/
  mod.rs       — module declarations
  time.rs      — Julian Day, ΔT, time-scale conversions
  sun.rs       — VSOP87 Earth position, solar longitude, equation of center
  terms.rs     — solar term crossing time finder
  moon.rs      — lunar longitude/latitude, new moon finder
  calendar.rs  — lunar calendar month table derivation
  compare.rs   — Android baseline comparison
```

## Precision Targets

| Computation | Target Accuracy | Method |
|-------------|----------------|--------|
| Solar longitude | < 1 arcminute | VSOP87 truncated |
| Lunar longitude | < 10 arcminutes | Meeus Ch.47 simplified |
| Solar term time | < ±2 minutes | Newton-Raphson on solar longitude |
| New moon time | < ±10 minutes | Newton-Raphson on Sun-Moon longitude difference |

## Consequences

- Engine module lives under `backend/src/astronomy/`, gated behind `#[cfg(feature = "astronomy")]` or always compiled.
- Android date layer remains the runtime baseline; engine output is generated data only.
- Generated artifact files (`out/*.json`) will be rewritten with real computed data and new sha256 hashes.
- Comparison report will classify differences between Android baseline and engine output.
- No capability promotion — `astronomy-engine` remains `target`.
