use crate::api::chart_basis::{birth_profile_to_json, metadata_to_json, string_array_to_json};
use crate::api::charts::build_chart_result;
use crate::config::AppConfig;
use crate::domain::bazi::ChartDetail;
use crate::error::AppError;
use crate::http::{Request, Response, json};

pub fn detail(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    let chart = build_chart_result(config, request)?;
    let detail = ChartDetail::from_result(&chart);
    Ok(Response::json(chart_detail_to_json(&detail)))
}

fn chart_detail_to_json(d: &ChartDetail) -> String {
    format!(
        concat!(
            r#"{{"status":"supported","capability":"chart-detail","#,
            r#""snapshot_id":{},"algo_version":{},"ruleset_id":{},"#,
            r#""birth_profile":{},"metadata":{},"#,
            r#""pillars":{{"year":"{}","month":"{}","day":"{}","hour":{}}},"#,
            r#""warnings":{},"ambiguity_flags":{},"#,
            r#""created_at_unix":{}"#,
            r#"}}"#
        ),
        json::string(&d.snapshot_id),
        json::string(d.algo_version),
        json::string(d.ruleset_id),
        birth_profile_to_json(&d.birth_profile),
        metadata_to_json(&d.metadata),
        json::string(&d.pillars.year.ganzhi()),
        json::string(&d.pillars.month.ganzhi()),
        json::string(&d.pillars.day.ganzhi()),
        d.pillars.hour.as_ref()
            .map(|h| json::string(&h.ganzhi()))
            .unwrap_or_else(|| "null".to_string()),
        string_array_to_json(&d.warnings),
        string_array_to_json(&d.ambiguity_flags),
        d.created_at_unix,
    )
}
