use crate::api::chart_basis::{
    birth_profile_to_json, chart_request_from_query, metadata_to_json, string_array_to_json,
};
use crate::calendar::lunar_data::{LunarDataSource, LunarDateResult};
use crate::config::AppConfig;
use crate::domain::bazi::{ChartBasis, ChartResult, DateLayerPillars, Pillar};
use crate::error::AppError;
use crate::http::{Request, Response, json};

pub fn create(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    let result = build_chart_result(config, request)?;

    Ok(Response::json(chart_result_to_json(&result)))
}

pub(crate) fn build_chart_result(
    config: &AppConfig,
    request: &Request,
) -> Result<ChartResult, AppError> {
    let chart_request = chart_request_from_query(request)?;
    let basis = ChartBasis::build(chart_request)?;

    let table = LunarDataSource::new(config.lunar_data_path.clone()).load_table()?;
    let date = basis.request.birth_profile.date;
    let date_layer = table.lookup(date).ok_or_else(|| {
        AppError::OutOfRange(format!(
            "birth date out of supported range: {}-{:02}-{:02}",
            date.year, date.month, date.day
        ))
    })?;

    ChartResult::build(basis, date_layer_pillars(&date_layer))
}

fn date_layer_pillars(result: &LunarDateResult) -> DateLayerPillars {
    DateLayerPillars {
        year: result.gan_zhi_year.clone(),
        month: result.gan_zhi_month.clone(),
        day: result.gan_zhi_day.clone(),
    }
}

fn chart_result_to_json(result: &ChartResult) -> String {
    format!(
        concat!(
            "{{",
            "\"status\":\"supported\",",
            "\"capability\":\"chart-create\",",
            "\"birth_profile\":{},",
            "\"metadata\":{},",
            "\"validated_range\":{{\"start_year\":{},\"end_year\":{}}},",
            "\"pillars\":{},",
            "\"warnings\":{},",
            "\"ambiguity_flags\":{},",
            "\"unsupported_outputs\":{}",
            "}}"
        ),
        birth_profile_to_json(&result.basis.request.birth_profile),
        metadata_to_json(&result.metadata),
        result.basis.validated_range_start_year,
        result.basis.validated_range_end_year,
        pillars_to_json(result),
        string_array_to_json(&result.warnings),
        string_array_to_json(&result.ambiguity_flags),
        string_array_to_json(&result.unsupported_outputs),
    )
}

fn pillars_to_json(result: &ChartResult) -> String {
    format!(
        "{{\"year\":{},\"month\":{},\"day\":{},\"hour\":{},\"hour_candidates\":{}}}",
        pillar_to_json(&result.chart.year),
        pillar_to_json(&result.chart.month),
        pillar_to_json(&result.chart.day),
        result
            .chart
            .hour
            .as_ref()
            .map(pillar_to_json)
            .unwrap_or_else(|| "null".to_string()),
        pillar_array_to_json(&result.chart.hour_candidates),
    )
}

fn pillar_to_json(pillar: &Pillar) -> String {
    format!(
        "{{\"stem\":{},\"branch\":{},\"ganzhi\":{}}}",
        json::string(&pillar.stem),
        json::string(&pillar.branch),
        json::string(&pillar.ganzhi()),
    )
}

fn pillar_array_to_json(pillars: &[Pillar]) -> String {
    let items = pillars
        .iter()
        .map(pillar_to_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}
