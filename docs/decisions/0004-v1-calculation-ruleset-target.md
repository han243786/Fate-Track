# ADR 0004: V1 Calculation Ruleset Target

## Status

Accepted for M2 chart-basis contract. Full chart implementation remains not fully implemented.

## Decision

Fate-Track V1 will use one documented default ruleset before exposing multiple schools or advanced rule matrices.

Target ruleset identifier:

```text
ft-v1-default
```

Target defaults:

| Rule | Default | User-visible requirement |
| --- | --- | --- |
| Year pillar boundary | exact Lichun instant | show that year pillar follows Lichun, not lunar new year |
| Month pillar boundary | solar-term jie intervals | show that month pillar follows solar terms, not lunar month labels |
| Day boundary | local civil midnight, `00:00` | echo `day_boundary_rule` in metadata |
| Alternative day boundary | Zi-start `23:00`, future advanced option | not enabled until tests and UI copy exist |
| Hour pillar basis | civil local time | echo timezone and resolved offset |
| True solar time | optional advanced mode | show offset minutes and changed pillars |
| Unknown hour | no fabricated hour pillar | return partial chart and hour sensitivity summary |
| Lunar conversion | current Android date layer now; ephemeris-derived target later | preserve golden cases and source provenance |

## Current Implementation Boundary

The current supported date API is:

```text
GET /api/calendar/query?date=YYYY-MM-DD
```

It returns Gregorian date, lunar date, year/month/day Gan-Zhi, zodiac, and solar term using the Android date-layer port. It does not yet implement full birth-time normalization, hour pillar, true solar time, IANA timezone history, unknown-hour aggregation, chart creation, or `/api/v1/...` target routes.

## Rationale

All three research reports identify hidden convention choices as the main trust risk. A single versioned default ruleset lets the product ship deterministic results while keeping future school variants possible.

The Android date layer is valuable because it already provides concrete data and edge cases. The ephemeris-backed model is the long-term target for stronger provenance and wider validation.

## Support Gate

No chart feature may be marked `supported` until it provides:

- `ruleset_id` and `algo_version`;
- timezone or timezone fallback metadata;
- rule metadata for year, month, day, and hour boundaries;
- golden tests for current Android edge cases and any replacement research vectors;
- explicit unsupported behavior for true solar time, unknown hour, and lunar input if not implemented.

## Consequences

- The frontend must display the active rule profile wherever chart results are shown.
- Date-layer changes must retain Android three-pillar edge cases or register stronger replacement vectors.
- A future ephemeris implementation must be introduced as a versioned engine, not as a silent replacement.
