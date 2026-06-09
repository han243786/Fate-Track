# ADR 0018: Solar Term Month Boundary Correction

## Status

Accepted.

## Problem

`solar_term_month_index` in `calendar/ganzhi.rs` has three defects that produce incorrect month pillars:

1. **Loop short-circuit (January)**: The loop checks 立春 (Feb 4, index 0) before 小寒 (Jan 6, index 11). All January dates hit `month(1) < 2` at index 0 and return 丑月(11), making the 小寒 check unreachable. January 1–5 (before 小寒) should be 子月(10).

2. **December default (Dec 7–31)**: Dates on or after 大雪 (Dec 7, index 10) pass all loop iterations without matching, then fall through to the default return of 丑月(11). They should be 子月(10).

3. **Fixed approximate dates**: `SOLAR_TERM_MONTH_STARTS` uses hardcoded dates (e.g. 小寒 always Jan 6), but actual solar terms vary ±1–2 days per year. The YAML data (`s` field) already carries exact day-of-year offsets.

Combined impact: ~30 days per year have wrong month pillars (early January + late December), plus boundary dates where fixed approximations disagree with actual solar terms.

## Decision

Replace `solar_term_month_index` with `solar_term_month_from_terms` that:

- Uses year-specific solar term day-of-year values from the YAML data (`s` field, even indices are month-start 节).
- Determines the correct solar term month by checking `doy` against solar term boundaries in proper order.
- Correctly assigns 子月 to dates before 小寒 and after 大雪 within the same year's solar term array.

Simultaneously correct the year-stem selection for month calculation:

- Use the **solar term year's stem** (determined by whether the date is before or after 立春) rather than the **CNY-based year's stem**.
- Before 立春: use previous Gregorian year's entry stem.
- On or after 立春: use current Gregorian year's entry stem.

## Alternatives Considered

1. **Keep current algorithm; document deviation**: Rejected. ~8% of dates have wrong month pillars. This is not a documented ruleset variation; it is a logic error.

2. **Fix loop order only, keep fixed dates**: Rejected. The YAML has exact solar term dates; using approximations when exact data is available would be a regression.

3. **Port Android source's month boundary logic exactly**: Rejected. The Android source likely has the same loop ordering bug (the golden cases for December dates all show 丑月 across the board, which is uniformly incorrect per standard solar term rules).

## Consequences

- `ganzhi::month_ganzhi` signature changes: no longer receives `&YearEntry` for lunar year and previous year; instead receives `&[u16]` (solar terms) and `usize` (solar year stem index).
- `LunarTable::lookup` must determine the solar year stem using `date.year` and the 立春 boundary from the YAML data.
- Android golden test cases for dates between Dec 7–31 and Jan 1–5 will have corrected `month_gz` expectations.
- The `month_boundary_rule` metadata changes from `"android-fixed-solar-term-month-starts"` to `"solar-term-data-driven-month-starts"` with an ADR reference.
- No capability promotion; existing `chart-create` and `analysis-snapshot` remain `supported`.
- No change to year pillar, day pillar, or hour pillar logic.
- No change to data files or YAML format.

## Affected Modules

- `calendar/ganzhi.rs` — `month_ganzhi`, new `solar_term_month_from_terms`
- `calendar/lunar_data.rs` — `lookup` method, solar year stem determination
- `domain/bazi.rs` — `CHART_ENGINE_ALGO_VERSION` metadata
- `tests` — Android edge case `month_gz` expectations for Dec/Jan boundary dates

## Required Tests

- Android golden cases updated with corrected month_gz values for affected dates.
- New test cases explicitly covering month boundaries (day before/after 小寒, 立春, 大雪).
- Existing unaffected golden cases must continue to pass unchanged.
