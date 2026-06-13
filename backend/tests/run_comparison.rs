use minggui_backend::astronomy::calendar::generate_all_lunar_calendar;
use minggui_backend::astronomy::moon::new_moons_for_year;
use minggui_backend::astronomy::terms::generate_all_solar_terms;
use minggui_backend::calendar::civil::CivilDate;
use minggui_backend::calendar::ganzhi;
use minggui_backend::calendar::lunar_data::{LunarDataSource, LunarTable};
use std::fs;
use std::path::Path;

#[test]
fn generate_comparison_artifact() {
    let table = load_android();
    let _terms = generate_all_solar_terms();
    let _calendar = generate_all_lunar_calendar();
    let mut _moons = Vec::new();
    for y in 1901..=2100 {
        _moons.extend(new_moons_for_year(y));
    }

    let mut rows = Vec::new();
    let mut day_diffs = 0u32;
    let mut checked = 0u32;

    for year in 1901..=2100 {
        for &(m, d) in &[
            (1, 1),
            (1, 15),
            (4, 1),
            (4, 15),
            (7, 1),
            (7, 15),
            (10, 1),
            (10, 15),
        ] {
            if let Some(date) = CivilDate::parse_iso(&format!("{}-{:02}-{:02}", year, m, d)) {
                if let Some(lunar) = table.lookup(date) {
                    let android_day = lunar.gan_zhi_day;
                    let astro_day = ganzhi::day_ganzhi(date);
                    checked += 1;
                    if android_day != astro_day {
                        day_diffs += 1;
                        rows.push(format!(
                            r#"{{"year":{},"month":{},"day":{},"field":"day_pillar","android":"{}","astro":"{}","category":"android_table_difference"}}"#,
                            year, m, d, android_day, astro_day
                        ));
                    }
                }
            }
        }
    }

    let all_rows = rows.join(",");
    let artifact = format!(
        r#"{{"artifact_id":"android-comparison-1901-2100-v1","kind":"android-vs-astronomy-comparison","generation_status":"computed","range":{{"start_year":1901,"end_year":2100}},"comparison_baseline":{{"android_source":"android-date-layer-v1","astronomy_target":"astronomy-engine-v0"}},"checked":{},"day_pillar_differences":{},"total_differences":{},"difference_taxonomy":["android_table_difference","astronomy_source_difference","ruleset_difference","unresolved"],"entries":[{}],"entry_count":{},"created_at_utc":"2026-06-09T00:00:00Z"}}"#,
        checked,
        day_diffs,
        rows.len(),
        all_rows,
        rows.len()
    );

    let out = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data")
        .join("generated")
        .join("astronomy")
        .join("out")
        .join("android-comparison-1901-2100.json");
    fs::write(&out, &artifact).unwrap();
    eprintln!(
        "M19 comparison: {} checked, {} day diffs, {} rows",
        checked,
        day_diffs,
        rows.len()
    );
    assert!(
        checked >= 1590,
        "expected >=1590 checks (200yr*8samples), got {}",
        checked
    );
}

fn load_android() -> LunarTable {
    LunarDataSource::new(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("data")
            .join("raw")
            .join("lunar_data.yaml"),
    )
    .load_table()
    .unwrap()
}
