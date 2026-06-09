use minggui_backend::astronomy::calendar::{LunarMonthEntry, generate_all_lunar_calendar};
use minggui_backend::astronomy::moon::new_moons_for_year;
use minggui_backend::astronomy::terms::{SolarTermEntry, generate_all_solar_terms};
use std::fs;

#[test]
fn generate_all_artifacts() {
    let out_dir = "../data/generated/astronomy/out";
    let _ = fs::create_dir_all(out_dir);

    // Solar terms
    let terms = generate_all_solar_terms();
    let tj = terms_json(&terms);
    fs::write(format!("{}/solar-terms-1901-2100.json", out_dir), &tj).unwrap();
    eprintln!("Solar terms: {} entries", terms.len());
    assert!(terms.len() >= 4790); // 200*24 - some at boundaries

    // New moons
    let mut moons = Vec::new();
    for y in 1901..=2100 {
        for (jd, greg) in new_moons_for_year(y) {
            moons.push((jd, greg));
        }
    }
    let mj = moons_json(&moons);
    fs::write(format!("{}/new-moons-1901-2100.json", out_dir), &mj).unwrap();
    eprintln!("New moons: {} entries", moons.len());
    eprintln!("New moons: {} entries (expected ~2470)", moons.len());

    // Lunar calendar
    let cal = generate_all_lunar_calendar();
    let cj = calendar_json(&cal);
    fs::write(format!("{}/lunar-calendar-1901-2100.json", out_dir), &cj).unwrap();
    eprintln!("Lunar calendar: {} months", cal.len());
    eprintln!("Lunar calendar: {} months", cal.len());

    // Comparison stub
    let cmp = comp_json(terms.len(), moons.len(), cal.len());
    fs::write(
        format!("{}/android-comparison-1901-2100.json", out_dir),
        &cmp,
    )
    .unwrap();
    eprintln!("Comparison: written");
}

fn terms_json(terms: &[SolarTermEntry]) -> String {
    let items: Vec<String> = terms
        .iter()
        .map(|t| {
            format!(
                r#"{{"year":{},"term_index":{},"term_name":"{}","gregorian_utc":"{}","jd_tt":{:.6}}}"#,
                t.year, t.term_index, t.term_name, t.gregorian_utc, t.jd_tt
            )
        })
        .collect();
    format!(
        r#"{{"artifact_id":"solar-terms-1901-2100-v1","kind":"solar-term-crossing-table","generation_status":"computed","range":{{"start_year":1901,"end_year":2100}},"entries":[{}],"entry_count":{}}}"#,
        items.join(","),
        terms.len()
    )
}

fn moons_json(moons: &[(f64, String)]) -> String {
    let items: Vec<String> = moons
        .iter()
        .map(|(jd, greg)| format!(r#"{{"jd_tt":{:.6},"gregorian_utc":"{}"}}"#, jd, greg))
        .collect();
    format!(
        r#"{{"artifact_id":"new-moons-1901-2100-v1","kind":"new-moon-table","generation_status":"computed","range":{{"start_year":1901,"end_year":2100}},"entries":[{}],"entry_count":{}}}"#,
        items.join(","),
        moons.len()
    )
}

fn calendar_json(cal: &[LunarMonthEntry]) -> String {
    let items: Vec<String> = cal
        .iter()
        .map(|m| {
            format!(
                r#"{{"year":{},"month_index":{},"month_name":"{}","is_leap_month":{},"day_count":{},"first_day_gregorian_utc":"{}","first_day_jd_tt":{:.6}}}"#,
                m.year, m.month_index, m.month_name, m.is_leap_month, m.day_count, m.first_day_gregorian_utc, m.first_day_jd_tt
            )
        })
        .collect();
    format!(
        r#"{{"artifact_id":"lunar-calendar-1901-2100-v1","kind":"derived-chinese-calendar-table","generation_status":"computed","range":{{"start_year":1901,"end_year":2100}},"entries":[{}],"entry_count":{}}}"#,
        items.join(","),
        cal.len()
    )
}

fn comp_json(terms: usize, moons: usize, cal: usize) -> String {
    format!(
        r#"{{"artifact_id":"android-comparison-1901-2100-v1","kind":"android-vs-astronomy-comparison","generation_status":"engine_computed","note":"M11 astronomy engine computed {} solar terms, {} new moons, {} lunar calendar months. Full comparison pending.","computed_counts":{{"solar_terms":{},"new_moons":{},"lunar_calendar":{}}},"entries":[],"entry_count":0}}"#,
        terms, moons, cal, terms, moons, cal
    )
}
