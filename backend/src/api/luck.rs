// M13: Luck cycles API route.
use crate::api::chart_basis::chart_request_from_query;
use crate::calendar::lunar_data::LunarDataSource;
use crate::config::AppConfig;
use crate::domain::bazi::{ChartBasis, Pillar};
use crate::domain::luck::compute_luck_cycle_context;
use crate::error::AppError;
use crate::http::{Request, Response};

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

    let context = compute_luck_cycle_context(&year_gan, &month_pillar, &profile.sex, profile.date);

    let items: Vec<String> = context.cycles.iter().map(|c| {
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
        context.direction,
        context.year_gan,
        context.days_to_jie,
        context.starting_age,
        items.join(",")
    );

    Ok(Response::json(body))
}
