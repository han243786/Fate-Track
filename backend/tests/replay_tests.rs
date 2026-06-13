// M20: Replay tests — verify old algorithm snapshots remain reproducible.
use minggui_backend::calendar::civil::CivilDate;
use minggui_backend::calendar::ganzhi;
use minggui_backend::calendar::lunar_data::{LunarDataSource, LunarTable};
use std::path::Path;

/// Known golden values from the Android edge-case manifest.
/// These must never change — they are the replay anchor.
const GOLDEN_ANCHORS: &[(&str, &str, &str, &str)] = &[
    // (iso_date, year_gz, month_gz, day_gz)
    ("1901-02-19", "辛丑", "庚寅", "戊辰"),
    ("1904-02-29", "甲辰", "丙寅", "癸巳"),
    ("1932-02-29", "壬申", "壬寅", "庚申"),
    ("1960-02-29", "庚子", "戊寅", "丁亥"),
    ("1988-02-29", "戊辰", "甲寅", "甲寅"),
    ("2000-02-29", "庚辰", "戊寅", "丁巳"),
    ("2024-02-29", "甲辰", "丙寅", "癸亥"),
    ("2025-01-01", "甲辰", "丙子", "庚午"),
    ("1903-07-09", "癸卯", "己未", "戊戌"),
    ("1950-02-17", "庚寅", "戊寅", "癸未"),
    ("2000-02-05", "庚辰", "戊寅", "癸巳"),
    ("2025-01-29", "乙巳", "丁丑", "戊戌"),
    ("2100-02-09", "庚申", "戊寅", "壬午"),
    ("1901-12-31", "辛丑", "庚子", "癸未"),
    ("2100-12-31", "庚申", "戊子", "丁未"),
];

#[test]
fn replay_android_golden_anchors() {
    let table = load_android();
    let mut passed = 0u32;
    let mut failed = Vec::new();

    for (iso, exp_year, exp_month, exp_day) in GOLDEN_ANCHORS {
        let date = CivilDate::parse_iso(iso).expect("valid iso date");
        let result = table.lookup(date).expect("date in range");
        let day_gz = ganzhi::day_ganzhi(date);

        let mut ok = true;
        if &result.gan_zhi_year != exp_year {
            ok = false;
        }
        if &result.gan_zhi_month != exp_month {
            ok = false;
        }
        if &result.gan_zhi_day != exp_day {
            ok = false;
        }
        if day_gz != *exp_day {
            ok = false;
        }

        if ok {
            passed += 1;
        } else {
            failed.push(format!(
                "{}: expected {}/{}/{}, got {}/{}/{} (astro day: {})",
                iso,
                exp_year,
                exp_month,
                exp_day,
                result.gan_zhi_year,
                result.gan_zhi_month,
                result.gan_zhi_day,
                day_gz
            ));
        }
    }

    if !failed.is_empty() {
        for f in &failed {
            eprintln!("REPLAY FAIL: {}", f);
        }
        panic!("{} replay anchors failed", failed.len());
    }
    eprintln!("Replay: {} golden anchors all verified", passed);
    assert_eq!(passed, GOLDEN_ANCHORS.len() as u32);
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
