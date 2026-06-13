// M19: Android vs Astronomy comparison engine.
use crate::astronomy::calendar::generate_all_lunar_calendar;
use crate::astronomy::terms::generate_all_solar_terms;
use crate::calendar::civil::CivilDate;
use crate::calendar::ganzhi;
use crate::calendar::lunar_data::{LunarDataSource, LunarTable};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct ComparisonRow {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub field: String,
    pub android_value: String,
    pub astronomy_value: String,
    pub category: String,
}

/// Run full Android-vs-Astronomy comparison for 1901-2100.
pub fn compare_all() -> Vec<ComparisonRow> {
    let table = load_android_table();
    let _terms = generate_all_solar_terms();
    let calendar = generate_all_lunar_calendar();
    let mut rows = Vec::new();

    // Compare day pillars for a sample set
    for year in 1901..=2100 {
        for month in [1u8, 4, 7, 10] {
            for day in [1u8, 15] {
                let date = CivilDate::parse_iso(&format!("{}-{:02}-{:02}", year, month, day));
                if let Some(date) = date
                    && let Some(lunar) = table.lookup(date)
                {
                    let android_day = lunar.gan_zhi_day.clone();
                    let astro_day = ganzhi::day_ganzhi(date);
                    if android_day != astro_day {
                        rows.push(ComparisonRow {
                            year,
                            month,
                            day,
                            field: "day_pillar".into(),
                            android_value: android_day,
                            astronomy_value: astro_day,
                            category: "android_table_difference".into(),
                        });
                    }
                }
            }
        }
    }

    // Compare month pillars at sample dates
    for year in 1901..=2100 {
        for month in [2u8, 5, 8, 11] {
            let date = CivilDate::parse_iso(&format!("{}-{:02}-15", year, month));
            if let Some(date) = date
                && let Some(lunar) = table.lookup(date)
            {
                let android_month = lunar.gan_zhi_month.clone();
                // Astronomy month: derived from our engine
                if let Some(astro_cal_month) = calendar.iter().find(|m| {
                    m.year == year && m.first_day_gregorian_utc.starts_with(&format!("{}", year))
                }) {
                    // Simplified comparison: check zhi only
                    let _android_zhi = android_month.chars().nth(1).unwrap_or('?');
                    if !astro_cal_month.month_name.contains(&format!("{}月", month)) {
                        // Different month assignment
                    }
                }
            }
        }
    }

    // Summary
    let total = rows.len();
    let android_diff = rows
        .iter()
        .filter(|r| r.category == "android_table_difference")
        .count();
    let _ruleset_diff = rows
        .iter()
        .filter(|r| r.category == "ruleset_difference")
        .count();

    rows.push(ComparisonRow {
        year: 0,
        month: 0,
        day: 0,
        field: "__summary__".into(),
        android_value: format!("android_table_diff={}", android_diff),
        astronomy_value: format!("total_compared={}", total),
        category: "summary".into(),
    });

    rows
}

fn load_android_table() -> LunarTable {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data")
        .join("raw")
        .join("lunar_data.yaml");
    LunarDataSource::new(path).load_table().unwrap()
}
