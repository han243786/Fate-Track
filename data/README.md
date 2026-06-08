# Data Sources

## Lunar Calendar Raw Data

| Field | Value |
| --- | --- |
| Project copy | `data/raw/lunar_data.yaml` |
| Original source path | `D:\myproject\Perpetual calendar\data\yaml\lunar_data.yaml` |
| Declared range | 1901-2100 |
| Format | YAML skip table |
| Purpose | Gregorian to lunar lookup, Gan-Zhi year, zodiac, lunar month lengths, solar-term offsets |
| Lifecycle | Raw source copy; do not mutate in application code |

The raw table is the current source of truth for the first backend lunar-calendar capability. Any generated Rust, JSON, database, or cache artifact derived from this file must record its source file, generation command, and validation evidence.

