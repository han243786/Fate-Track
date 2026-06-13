// M20: Generate golden row fixtures for 6 required categories.
use minggui_backend::calendar::civil::CivilDate;
use minggui_backend::calendar::lunar_data::{LunarDataSource, LunarTable};
use std::fs;
use std::path::Path;

#[test]
fn generate_golden_fixtures() {
    let table = load_android();
    let mut golden = Vec::new();

    // Category 1: 1901-2100-boundary (4 cases)
    for (label, iso) in &[
        ("boundary-start", "1901-02-19"),
        ("boundary-end", "2100-12-31"),
        ("boundary-leap-2000", "2000-02-29"),
        ("boundary-leap-2024", "2024-02-29"),
    ] {
        if let Some(d) = CivilDate::parse_iso(iso) {
            if let Some(r) = table.lookup(d) {
                golden.push(format!(
                    r#"{{"category":"1901-2100-boundary","label":"{}","date":"{}","year_gz":"{}","month_gz":"{}","day_gz":"{}","lunar":"{}{}月{}{}"}}"#,
                    label, iso, r.gan_zhi_year, r.gan_zhi_month, r.gan_zhi_day,
                    r.lunar_year, if r.is_leap_month {"闰"} else {""}, r.lunar_month, r.day_name()
                ));
            }
        }
    }

    // Category 2: 2033-anomaly (6 cases around the special leap month year)
    for iso in &[
        "2033-01-01",
        "2033-03-15",
        "2033-07-01",
        "2033-09-15",
        "2033-11-01",
        "2033-12-31",
    ] {
        if let Some(d) = CivilDate::parse_iso(iso) {
            if let Some(r) = table.lookup(d) {
                golden.push(format!(
                    r#"{{"category":"2033-anomaly","date":"{}","year_gz":"{}","month_gz":"{}","day_gz":"{}","lunar":"{}{}月"}}"#,
                    iso, r.gan_zhi_year, r.gan_zhi_month, r.gan_zhi_day,
                    r.lunar_year, r.month_name()
                ));
            }
        }
    }

    // Category 3: lichun-boundary (6 cases around Start of Spring)
    for iso in &[
        "2020-02-03",
        "2020-02-04",
        "2020-02-05",
        "2024-02-03",
        "2024-02-04",
        "2024-02-05",
    ] {
        if let Some(d) = CivilDate::parse_iso(iso) {
            if let Some(r) = table.lookup(d) {
                golden.push(format!(
                    r#"{{"category":"lichun-boundary","date":"{}","year_gz":"{}","month_gz":"{}","day_gz":"{}"}}"#,
                    iso, r.gan_zhi_year, r.gan_zhi_month, r.gan_zhi_day
                ));
            }
        }
    }

    // Category 4: qingming-boundary (4 cases)
    for iso in &["2020-04-04", "2020-04-05", "2024-04-04", "2024-04-05"] {
        if let Some(d) = CivilDate::parse_iso(iso) {
            if let Some(r) = table.lookup(d) {
                golden.push(format!(
                    r#"{{"category":"qingming-boundary","date":"{}","year_gz":"{}","month_gz":"{}","day_gz":"{}"}}"#,
                    iso, r.gan_zhi_year, r.gan_zhi_month, r.gan_zhi_day
                ));
            }
        }
    }

    // Category 5: jiazi-day-anchor (4 cases)
    for iso in &["1984-02-02", "2000-01-01", "2025-01-01", "2044-02-10"] {
        if let Some(d) = CivilDate::parse_iso(iso) {
            if let Some(r) = table.lookup(d) {
                golden.push(format!(
                    r#"{{"category":"jiazi-day-anchor","date":"{}","day_gz":"{}"}}"#,
                    iso, r.gan_zhi_day
                ));
            }
        }
    }

    // Category 6: near-midnight (6 cases at day boundaries)
    for iso in &[
        "2024-12-31",
        "2025-01-01",
        "2025-06-30",
        "2025-07-01",
        "2099-12-31",
        "2100-01-01",
    ] {
        if let Some(d) = CivilDate::parse_iso(iso) {
            if let Some(r) = table.lookup(d) {
                golden.push(format!(
                    r#"{{"category":"near-midnight-solar-lunar-event","date":"{}","year_gz":"{}","month_gz":"{}","day_gz":"{}","lunar":"{}{}月{}"}}"#,
                    iso, r.gan_zhi_year, r.gan_zhi_month, r.gan_zhi_day,
                    r.lunar_year, if r.is_leap_month {"闰"} else {""}, r.lunar_month
                ));
            }
        }
    }

    let out = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data")
        .join("generated")
        .join("astronomy")
        .join("golden-fixtures.json");
    let artifact = format!(
        r#"{{"artifact_id":"golden-fixtures-v1","kind":"golden-row-fixtures","generation_status":"computed","golden_plan_id":"astronomy-engine-v0-golden-cases-plan","categories":["1901-2100-boundary","2033-anomaly","lichun-boundary","qingming-boundary","jiazi-day-anchor","near-midnight-solar-lunar-event"],"entries":[{}],"entry_count":{},"created_at_utc":"2026-06-09T00:00:00Z"}}"#,
        golden.join(","),
        golden.len()
    );
    fs::write(&out, &artifact).unwrap();
    eprintln!(
        "M20 golden fixtures: {} entries across 6 categories",
        golden.len()
    );
    assert!(
        golden.len() >= 30,
        "expected >=30 golden entries, got {}",
        golden.len()
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
