use crate::api::chart_basis::string_array_to_json;
use crate::api::charts::build_chart_result;
use crate::config::AppConfig;
use crate::domain::analysis::{AnalysisCard, AnalysisSnapshot, WeightedMetric};
use crate::error::AppError;
use crate::http::{Request, Response, json};

pub fn snapshot(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    let chart = build_chart_result(config, request)?;
    let snapshot = AnalysisSnapshot::build(&chart);

    Ok(Response::json(snapshot_to_json(&snapshot)))
}

fn snapshot_to_json(snapshot: &AnalysisSnapshot) -> String {
    format!(
        concat!(
            "{{",
            "\"status\":\"supported\",",
            "\"capability\":\"analysis-snapshot\",",
            "\"algo_version\":{},",
            "\"disclaimer_id\":{},",
            "\"day_master\":{},",
            "\"metrics\":{{",
            "\"elements\":{},",
            "\"ten_gods\":{},",
            "\"hidden_stems\":{}",
            "}},",
            "\"relation_flags\":{},",
            "\"sensitivity_flags\":{},",
            "\"cards\":{},",
            "\"forbidden_output_audit\":{{\"status\":{},\"checked_patterns\":{}}}",
            "}}"
        ),
        json::string(snapshot.algo_version),
        json::string(snapshot.disclaimer_id),
        json::string(&snapshot.day_master),
        metrics_to_json(&snapshot.element_metrics),
        metrics_to_json(&snapshot.ten_god_metrics),
        metrics_to_json(&snapshot.hidden_stem_metrics),
        string_array_to_json(&snapshot.relation_flags),
        string_array_to_json(&snapshot.sensitivity_flags),
        cards_to_json(&snapshot.cards),
        json::string(snapshot.forbidden_output_audit.status),
        snapshot.forbidden_output_audit.checked_patterns,
    )
}

fn metrics_to_json(metrics: &[WeightedMetric]) -> String {
    let items = metrics
        .iter()
        .map(|metric| {
            format!(
                "{{\"id\":{},\"weight_x2\":{}}}",
                json::string(metric.id),
                metric.weight_x2
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}

fn cards_to_json(cards: &[AnalysisCard]) -> String {
    let items = cards
        .iter()
        .map(|card| {
            format!(
                "{{\"id\":{},\"title\":{},\"severity\":{},\"body\":{}}}",
                json::string(card.id),
                json::string(card.title),
                json::string(card.severity),
                json::string(card.body),
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}
