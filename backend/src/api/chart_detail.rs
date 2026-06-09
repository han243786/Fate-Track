// M12: Chart detail snapshot route
use crate::api::chart_basis::{birth_profile_to_json, metadata_to_json, string_array_to_json};
use crate::api::charts::build_chart_result;
use crate::config::AppConfig;
use crate::error::AppError;
use crate::http::{Request, Response, json};

pub fn detail(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    let chart = build_chart_result(config, request)?;
    let profile_json = birth_profile_to_json(&chart.basis.request.birth_profile);
    let metadata_json = metadata_to_json(&chart.metadata);

    let body = format!(
        concat!(
            "{{",
            "\"status\":\"supported\",",
            "\"capability\":\"chart-detail\",",
            "\"snapshot_id\":\"chart:{}\",",
            "\"algo_version\":{},",
            "\"ruleset_id\":{},",
            "\"birth_profile\":{},",
            "\"metadata\":{},",
            "\"pillars\":{{\"year\":\"{}\",\"month\":\"{}\",\"day\":\"{}\",\"hour\":{}}},",
            "\"warnings\":{},",
            "\"ambiguity_flags\":{}",
            "}}"
        ),
        json::string(&chart.chart.day.ganzhi()),
        json::string(chart.metadata.algo_version),
        json::string(chart.metadata.ruleset_id),
        profile_json,
        metadata_json,
        json::string(&chart.chart.year.ganzhi()),
        json::string(&chart.chart.month.ganzhi()),
        json::string(&chart.chart.day.ganzhi()),
        chart
            .chart
            .hour
            .as_ref()
            .map(|h| json::string(&h.ganzhi()))
            .unwrap_or_else(|| "null".to_string()),
        string_array_to_json(&chart.warnings),
        string_array_to_json(&chart.ambiguity_flags),
    );

    Ok(Response::json(body))
}
