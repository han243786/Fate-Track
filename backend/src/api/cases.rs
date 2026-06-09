use crate::api::chart_basis::string_array_to_json;
use crate::api::charts::build_chart_result;
use crate::config::AppConfig;
use crate::domain::analysis::AnalysisSnapshot;
use crate::domain::cases::{
    AnalysisSnapshotRef, CaseRecord, CaseRepository, CaseStatus, CaseSummary, ChartSnapshot,
};
use crate::error::AppError;
use crate::http::{Request, Response, json};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

static CASE_STORE: OnceLock<Mutex<CaseRepository>> = OnceLock::new();

pub fn cases(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    match request.query_value("action").as_deref().unwrap_or("list") {
        "list" => list_cases(),
        "create" => create_case(config, request),
        "detail" => case_detail(request),
        "update_metadata" => update_metadata(request),
        "archive" => archive_case(request),
        "delete" => delete_case(request),
        other => Err(AppError::BadRequest(format!(
            "unsupported case action: {other}"
        ))),
    }
}

pub(crate) fn case_for_share(id: &str) -> Option<CaseRecord> {
    store().lock().unwrap().get(id).cloned()
}

fn list_cases() -> Result<Response, AppError> {
    let summaries = store().lock().unwrap().list();

    Ok(Response::json(format!(
        "{{\"status\":\"restricted\",\"capability\":\"case-management\",\"storage\":\"local-volatile-case-store-v1\",\"cases\":{}}}",
        case_summaries_to_json(&summaries)
    )))
}

fn create_case(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    let id = required_query(request, "id")?;
    let title = required_query(request, "title")?;
    if id.trim().is_empty() {
        return Err(AppError::BadRequest("case id is required".to_string()));
    }
    if title.trim().is_empty() {
        return Err(AppError::BadRequest("case title is required".to_string()));
    }

    let chart = build_chart_result(config, request)?;
    let analysis = AnalysisSnapshot::build(&chart);
    let now = now_unix();
    let record = CaseRecord {
        id: id.clone(),
        title,
        tags: parse_tags(request.query_value("tags")),
        private_note: parse_optional_note(request.query_value("note")),
        status: CaseStatus::Active,
        chart_snapshot: ChartSnapshot {
            snapshot_id: format!("{}:chart:{}", id, chart.metadata.algo_version),
            chart_algo_version: chart.metadata.algo_version.to_string(),
            ruleset_id: chart.metadata.ruleset_id.to_string(),
            day_master: chart.chart.day.stem.clone(),
        },
        analysis_snapshot: AnalysisSnapshotRef {
            snapshot_id: format!("{}:analysis:{}", id, analysis.algo_version),
            analysis_algo_version: analysis.algo_version.to_string(),
            disclaimer_id: analysis.disclaimer_id.to_string(),
        },
        created_at_unix: now,
        updated_at_unix: now,
    };

    store().lock().unwrap().upsert(record.clone());

    Ok(Response::json(case_detail_to_json(&record)))
}

fn case_detail(request: &Request) -> Result<Response, AppError> {
    let id = required_query(request, "id")?;
    let record = store()
        .lock()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("case not found: {id}")))?;

    Ok(Response::json(case_detail_to_json(&record)))
}

fn update_metadata(request: &Request) -> Result<Response, AppError> {
    let id = required_query(request, "id")?;
    let title = request.query_value("title");
    let tags = request
        .query_value("tags")
        .map(|tags| parse_tags(Some(tags)));
    let private_note = request
        .query_value("note")
        .map(|note| parse_optional_note(Some(note)));
    let updated = store()
        .lock()
        .unwrap()
        .update_metadata(&id, title, tags, private_note, now_unix())
        .ok_or_else(|| AppError::NotFound(format!("case not found: {id}")))?;

    Ok(Response::json(case_detail_to_json(&updated)))
}

fn archive_case(request: &Request) -> Result<Response, AppError> {
    let id = required_query(request, "id")?;
    let archived = store()
        .lock()
        .unwrap()
        .archive(&id, now_unix())
        .ok_or_else(|| AppError::NotFound(format!("case not found: {id}")))?;

    Ok(Response::json(case_detail_to_json(&archived)))
}

fn delete_case(request: &Request) -> Result<Response, AppError> {
    let id = required_query(request, "id")?;
    store()
        .lock()
        .unwrap()
        .delete(&id, now_unix())
        .ok_or_else(|| AppError::NotFound(format!("case not found: {id}")))?;

    Ok(Response::json(format!(
        "{{\"status\":\"restricted\",\"capability\":\"case-management\",\"deleted\":true,\"id\":{}}}",
        json::string(&id)
    )))
}

fn store() -> &'static Mutex<CaseRepository> {
    CASE_STORE.get_or_init(|| Mutex::new(CaseRepository::new()))
}

fn required_query(request: &Request, key: &str) -> Result<String, AppError> {
    request
        .query_value(key)
        .ok_or_else(|| AppError::BadRequest(format!("missing query parameter: {key}")))
}

fn parse_tags(value: Option<String>) -> Vec<String> {
    value
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn parse_optional_note(value: Option<String>) -> Option<String> {
    value.and_then(|note| {
        let trimmed = note.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn case_summaries_to_json(summaries: &[CaseSummary]) -> String {
    let items = summaries
        .iter()
        .map(case_summary_to_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}

fn case_summary_to_json(summary: &CaseSummary) -> String {
    format!(
        "{{\"id\":{},\"title\":{},\"tags\":{},\"status\":{},\"chart_snapshot_id\":{},\"analysis_snapshot_id\":{}}}",
        json::string(&summary.id),
        json::string(&summary.title),
        string_vec_to_json(&summary.tags),
        json::string(summary.status.as_str()),
        json::string(&summary.chart_snapshot_id),
        json::string(&summary.analysis_snapshot_id),
    )
}

fn case_detail_to_json(record: &CaseRecord) -> String {
    format!(
        concat!(
            "{{",
            "\"status\":\"restricted\",",
            "\"capability\":\"case-management\",",
            "\"storage\":\"local-volatile-case-store-v1\",",
            "\"id\":{},",
            "\"title\":{},",
            "\"tags\":{},",
            "\"case_status\":{},",
            "\"private_note\":{},",
            "\"chart_snapshot\":{},",
            "\"analysis_snapshot\":{},",
            "\"created_at_unix\":{},",
            "\"updated_at_unix\":{}",
            "}}"
        ),
        json::string(&record.id),
        json::string(&record.title),
        string_vec_to_json(&record.tags),
        json::string(record.status.as_str()),
        json::option_string(record.private_note.as_deref()),
        chart_snapshot_to_json(&record.chart_snapshot),
        analysis_snapshot_to_json(&record.analysis_snapshot),
        record.created_at_unix,
        record.updated_at_unix,
    )
}

fn chart_snapshot_to_json(snapshot: &ChartSnapshot) -> String {
    format!(
        "{{\"snapshot_id\":{},\"chart_algo_version\":{},\"ruleset_id\":{},\"day_master\":{}}}",
        json::string(&snapshot.snapshot_id),
        json::string(&snapshot.chart_algo_version),
        json::string(&snapshot.ruleset_id),
        json::string(&snapshot.day_master),
    )
}

fn analysis_snapshot_to_json(snapshot: &AnalysisSnapshotRef) -> String {
    format!(
        "{{\"snapshot_id\":{},\"analysis_algo_version\":{},\"disclaimer_id\":{}}}",
        json::string(&snapshot.snapshot_id),
        json::string(&snapshot.analysis_algo_version),
        json::string(&snapshot.disclaimer_id),
    )
}

fn string_vec_to_json(values: &[String]) -> String {
    let refs = values.iter().map(String::as_str).collect::<Vec<_>>();
    string_array_to_json(&refs)
}
