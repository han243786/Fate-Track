# ADR 0002: Android Date Layer as Calendar Reference

## Status

Accepted.

## Decision

The first Fate-Track date layer is ported from the Android perpetual-calendar project at `D:\myproject\Perpetual calendar`.

Reference files:

| Source | Role |
| --- | --- |
| `app/src/main/java/com/perpetualcalendar/app/lunar/LunarCalendar.java` | Gregorian date to lunar date lookup, solar term lookup, year/month/day Gan-Zhi composition |
| `app/src/main/java/com/perpetualcalendar/app/lunar/GanZhi.java` | Day Gan-Zhi epoch, month Gan-Zhi rule, fixed solar-term month starts |
| `app/src/main/java/com/perpetualcalendar/app/lunar/HuangLi.java` | Almanac side calculations; not yet ported |
| `data/build_skip_table.py` | Skip-table generation model |
| `data/edge_case_test.txt` | Golden edge cases for leap days, leap months, CNY boundaries, cross-year continuity, and solar-term dates |

## Current Port

Implemented in:

- `backend/src/calendar/civil.rs`
- `backend/src/calendar/ganzhi.rs`
- `backend/src/calendar/lunar_data.rs`
- `backend/src/api/calendar.rs`

The supported API is:

```text
GET /api/calendar/query?date=YYYY-MM-DD
```

## Guardrail

The date layer must preserve year, month, and day Gan-Zhi consistency before any Bazi chart feature is marked supported. If future research changes the algorithm, it must provide replacement golden cases for the Android edge-case set.

