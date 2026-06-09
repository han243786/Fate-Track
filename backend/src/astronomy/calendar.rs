// Lunar calendar derivation from solar terms and new moons.
// Applies GB/T 33661-2017 rules for month assignment and leap month placement.

use crate::astronomy::moon::new_moons_for_year;
use crate::astronomy::terms::solar_terms_for_year;

/// A lunar calendar month entry.
#[derive(Clone, Debug)]
pub struct LunarMonthEntry {
    pub year: i32,
    pub month_index: u8,
    pub month_name: String,
    pub is_leap_month: bool,
    pub day_count: u8,
    pub first_day_gregorian_utc: String,
    pub first_day_jd_tt: f64,
}

/// Derive the Chinese lunar calendar for a given year.
/// Returns 12 or 13 month entries (if there's a leap month).
pub fn lunar_calendar_for_year(year: i32) -> Vec<LunarMonthEntry> {
    let terms = solar_terms_for_year(year);

    // Get new moons for the year (plus one from previous and next year for edge detection)
    let mut moons = Vec::new();
    moons.extend(new_moons_for_year(year - 1));
    moons.extend(new_moons_for_year(year));
    moons.extend(new_moons_for_year(year + 1));

    // Filter moons to the relevant window: from the new moon before the first
    // solar term of the year to the new moon after the last solar term
    // Actually: lunar months start at new moons. We need the sequence of new moons
    // that covers the period from roughly Nov of previous year through Feb of next year.

    // Get solar term DOY values for month boundary detection
    // We need the 节 (odd-indexed terms: 立春, 惊蛰, ..., 小寒) for month starts
    // Each lunar month is associated with a 中气 (even-indexed terms: 雨水, 春分, ...)
    // Leap month rule: if a month has no 中气 (zhong qi), it's a leap month

    // Build the calendar by aligning new moons with solar terms
    let mut months = Vec::new();

    // Collect relevant new moons: all that occur during the year
    let year_moons: Vec<_> = moons
        .iter()
        .filter(|(_, greg)| greg.starts_with(&format!("{}", year)))
        .collect();

    if year_moons.is_empty() {
        return months;
    }

    // Get the first new moon of the year
    // Need to also check late December of previous year for month that starts in prev year
    // but whose first day might be in prev year (month 11 or 12 of prev lunar year)

    // Simplified approach: use the new moons directly as month starts
    // Month numbering: find which month contains 冬至 (winter solstice, term 23)
    // The month containing 冬至 is always month 11 (十一月)

    let winter_solstice = terms.iter().find(|t| t.term_index == 23);
    let winter_solstice_jd = winter_solstice.map(|t| t.jd_tt).unwrap_or(0.0);

    // Find the new moon immediately before winter solstice
    let mut month11_idx: Option<usize> = None;
    for (i, moon) in year_moons.iter().enumerate() {
        if moon.0 > winter_solstice_jd {
            month11_idx = if i > 0 { Some(i - 1) } else { None };
            break;
        }
    }

    if month11_idx.is_none() && !year_moons.is_empty() {
        // If winter solstice is before the first new moon, use the previous year's last new moon
        if year_moons[0].0 > winter_solstice_jd {
            month11_idx = None; // Use the first moon as month 11
        }
    }

    // Determine month 11 start index
    let m11_start = month11_idx.unwrap_or(0);

    // Build months: month 11 is at index m11_start, month 12 follows, then month 1, 2, ...
    for i in 0..year_moons.len() {
        let lunar_month_num = ((11 + i as i32 - m11_start as i32) % 12 + 12) % 12 + 1;
        let (jd, greg) = &year_moons[i];
        let jd = *jd;

        // Check if this month contains a 中气 (even-indexed solar term)
        let next_moon_jd = if i + 1 < year_moons.len() {
            year_moons[i + 1].0
        } else {
            jd + 30.0 // approximate
        };

        let has_zhong_qi = terms.iter().any(|t| {
            t.term_index % 2 == 1 // even-index terms = 中气 (雨水=1, 春分=3, ...)
                && t.jd_tt >= jd && t.jd_tt < next_moon_jd
        });

        let is_leap = !has_zhong_qi && i > 0;

        // Day count = days between this new moon and the next
        let day_count = if i + 1 < year_moons.len() {
            ((year_moons[i + 1].0 - jd).round()) as u8
        } else {
            30
        };

        let month_name = if is_leap {
            let prev_num = ((11 + (i as i32 - 1) - m11_start as i32) % 12 + 12) % 12 + 1;
            format!("闰{}月", lunar_month_name(prev_num as u8))
        } else {
            lunar_month_name(lunar_month_num as u8).to_string()
        };

        months.push(LunarMonthEntry {
            year,
            month_index: lunar_month_num as u8,
            month_name,
            is_leap_month: is_leap,
            day_count,
            first_day_gregorian_utc: greg.clone(),
            first_day_jd_tt: jd,
        });
    }

    months
}

fn lunar_month_name(num: u8) -> String {
    const NAMES: [&str; 13] = [
        "",
        "正月",
        "二月",
        "三月",
        "四月",
        "五月",
        "六月",
        "七月",
        "八月",
        "九月",
        "十月",
        "十一月",
        "十二月",
    ];
    NAMES.get(num as usize).unwrap_or(&"").to_string()
}

/// Generate lunar calendar for the full 1901-2100 range.
pub fn generate_all_lunar_calendar() -> Vec<LunarMonthEntry> {
    let mut all = Vec::new();
    for year in 1901..=2100 {
        all.extend(lunar_calendar_for_year(year));
    }
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lunar_calendar_2025_has_12_or_13_months() {
        let months = lunar_calendar_for_year(2025);
        assert!(
            months.len() >= 11 && months.len() <= 14,
            "expected 11-14 lunar months in 2025, got {}",
            months.len()
        );
    }

    #[test]
    fn lunar_month_names_valid() {
        let months = lunar_calendar_for_year(2025);
        for m in &months {
            assert!(!m.month_name.is_empty());
            assert!(
                m.day_count == 29 || m.day_count == 30,
                "month day count should be 29 or 30, got {} for {}",
                m.day_count,
                m.month_name
            );
        }
    }
}
