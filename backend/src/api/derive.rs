use crate::api::cases::derive_stats;
use crate::http::{Request, Response};

const MIN_THRESHOLD: u32 = 5;

pub fn derive(request: &Request) -> Result<Response, crate::error::AppError> {
    let dt = request.query_value("type").unwrap_or_else(|| "summary".to_string());
    let stats = derive_stats();
    let total = stats.total_cases;

    let body = if dt == "summary" {
        format!(r#"{{"status":"restricted","capability":"data-derivation","type":"summary","total_cases":{},"privacy":{{"threshold":{},"note":"No individual data. Aggregates suppressed below {} cases."}}}}"#, total, MIN_THRESHOLD, MIN_THRESHOLD)
    } else if dt == "day_masters" {
        to_json("day_masters", total, &stats.day_masters)
    } else if dt == "elements" {
        to_json("elements", total, &stats.elements)
    } else if dt == "ten_gods" {
        to_json("ten_gods", total, &stats.ten_gods)
    } else if dt == "hours" {
        to_json("hours", total, &stats.hour_distribution)
    } else {
        r#"{"status":"restricted","capability":"data-derivation","type":"unknown","data":{},"note":"Valid types: summary, day_masters, elements, ten_gods, hours"}"#.to_string()
    };

    Ok(Response::json(body))
}

fn to_json(ty: &str, total: u32, map: &std::collections::BTreeMap<String, u32>) -> String {
    let entries: Vec<String> = map.iter()
        .filter(|(_, c)| **c >= MIN_THRESHOLD)
        .map(|(k, c)| format!("\"{}\":{}", k, c))
        .collect();
    format!(r#"{{"status":"restricted","capability":"data-derivation","type":"{}","total_cases":{},"threshold":{},"data":{{{}}}}}"#, ty, total, MIN_THRESHOLD, entries.join(","))
}
