// M13: Luck cycles API route.
use crate::api::chart_basis::{chart_request_from_query, string_array_to_json};
use crate::astronomy::terms::solar_terms_for_year;
use crate::calendar::lunar_data::LunarDataSource;
use crate::config::AppConfig;
use crate::domain::bazi::{ChartBasis, Pillar, Sex, TimePrecision};
use crate::domain::luck::compute_luck_cycles;
use crate::error::AppError;
use crate::http::{Request, Response, json};

pub fn cycles(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    let chart_request = chart_request_from_query(request)?;
    let basis = ChartBasis::build(chart_request)?;
    let profile = &basis.request.birth_profile;

    let table = LunarDataSource::new(config.lunar_data_path.clone()).load_table()?;
    let date_layer = table
        .lookup(profile.date)
        .ok_or_else(|| AppError::OutOfRange("birth date out of supported range".to_string()))?;

    // Determine year stem from year pillar
    let year_gan = date_layer
        .gan_zhi_year
        .chars()
        .next()
        .unwrap_or('甲')
        .to_string();

    // Parse month pillar for first cycle base
    let month_pillar = Pillar::from_ganzhi(&date_layer.gan_zhi_month)?;

    // Compute days to next/previous 节
    let terms = solar_terms_for_year(profile.date.year);
    let doy = profile.date.day_of_year() as u16;

    let days_to_jie = if is_forward(&year_gan, &profile.sex) {
        // Forward: find next 节 after birth date
        let jie_indices = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
        let mut next_jie_doy = 366u16;
        for &ji in &jie_indices {
            if let Some(term) = terms.get(ji) {
                let td = (term.jd_tt
                    - crate::astronomy::time::gregorian_to_jd(profile.date.year, 1, 1, 0.0).whole())
                    as u16
                    + 1;
                if td > doy && td < next_jie_doy {
                    next_jie_doy = td;
                }
            }
        }
        if next_jie_doy == 366 {
            // Wrap to next year's first 节 (小寒)
            let next_year_terms = solar_terms_for_year(profile.date.year + 1);
            if let Some(term) = next_year_terms.first() {
                next_jie_doy = (term.jd_tt
                    - crate::astronomy::time::gregorian_to_jd(profile.date.year + 1, 1, 1, 0.0)
                        .whole()) as u16
                    + 365;
            }
        }
        next_jie_doy.saturating_sub(doy)
    } else {
        // Reverse: find previous 节 before birth date
        let jie_indices = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
        let mut prev_jie_doy = 0u16;
        for &ji in &jie_indices {
            if let Some(term) = terms.get(ji) {
                let td = (term.jd_tt
                    - crate::astronomy::time::gregorian_to_jd(profile.date.year, 1, 1, 0.0).whole())
                    as u16
                    + 1;
                if td <= doy && td > prev_jie_doy {
                    prev_jie_doy = td;
                }
            }
        }
        if prev_jie_doy == 0 {
            // Use previous year's last 节 (大雪)
            let prev_year_terms = solar_terms_for_year(profile.date.year - 1);
            if let Some(term) = prev_year_terms.get(11) {
                let td = (term.jd_tt
                    - crate::astronomy::time::gregorian_to_jd(profile.date.year - 1, 1, 1, 0.0)
                        .whole()) as u16
                    + 1;
                prev_jie_doy = td;
            }
            doy + 365u16 - prev_jie_doy
        } else {
            doy - prev_jie_doy
        }
    };

    let cycles = compute_luck_cycles(&year_gan, &month_pillar, &profile.sex, days_to_jie);

    let items: Vec<String> = cycles.iter().map(|c| {
        format!(
            r#"{{"label":"{}","start_age":{},"end_age":{},"stem":"{}","branch":"{}","ganzhi":"{}"}}"#,
            c.label, c.start_age, c.end_age, c.pillar.stem, c.pillar.branch, c.pillar.ganzhi()
        )
    }).collect();

    let body = format!(
        concat!(
            r#"{{"status":"supported","capability":"luck-cycles","#,
            r#""algo_version":"luck-engine-v1-adr-0020","#,
            r#""ruleset_id":"ft-v1-default","#,
            r#""direction":"{}","#,
            r#""year_gan":"{}","#,
            r#""days_to_jie":{},"#,
            r#""starting_age":{},"#,
            r#""cycles":[{}],"#,
            r#""liu_nian":{{"status":"planned","note":"流年分析尚未实现，仅大运排盘可用"}},"#,
            r#""liu_yue":{{"status":"planned","note":"流月分析尚未实现"}}"#,
            r#"}}"#
        ),
        if is_forward(&year_gan, &profile.sex) {
            "forward"
        } else {
            "reverse"
        },
        year_gan,
        days_to_jie,
        if cycles.is_empty() {
            0
        } else {
            cycles[0].start_age
        },
        items.join(",")
    );

    Ok(Response::json(body))
}

fn is_forward(year_gan: &str, sex: &Sex) -> bool {
    let yang = ["甲", "丙", "戊", "庚", "壬"].contains(&year_gan);
    let male = matches!(sex, Sex::Male);
    (yang && male) || (!yang && !male)
}
