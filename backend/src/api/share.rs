use crate::api::cases::case_for_share;
use crate::domain::share::{
    PUBLIC_SHARE_DTO_VERSION, RedactedShareSnapshot, SHARE_STORE_VERSION, ShareCreateResult,
    ShareRepository, UNAVAILABLE_MESSAGE,
};
use crate::error::AppError;
use crate::http::{Request, Response, json};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

static SHARE_STORE: OnceLock<Mutex<ShareRepository>> = OnceLock::new();

pub fn preview(request: &Request) -> Result<Response, AppError> {
    match request.query_value("action").as_deref().unwrap_or("public") {
        "create" => create_share(request),
        "public" => public_share(request),
        "revoke" => revoke_share(request),
        other => Err(AppError::BadRequest(format!(
            "unsupported share action: {other}"
        ))),
    }
}

fn create_share(request: &Request) -> Result<Response, AppError> {
    let case_id = required_query(request, "case_id")?;
    let ttl_seconds = request
        .query_value("ttl_seconds")
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(86_400)
        .clamp(60, 604_800);
    let case = case_for_share(&case_id).ok_or_else(unavailable_error)?;
    let created = store()
        .lock()
        .unwrap()
        .create(&case, ttl_seconds, now_unix());

    Ok(Response::json(create_result_to_json(&created)))
}

fn public_share(request: &Request) -> Result<Response, AppError> {
    let token = required_query(request, "token")?;
    let snapshot = store()
        .lock()
        .unwrap()
        .resolve_public(&token, now_unix())
        .ok_or_else(unavailable_error)?;

    Ok(Response::json(public_snapshot_to_json(&snapshot)))
}

fn revoke_share(request: &Request) -> Result<Response, AppError> {
    let token = required_query(request, "token")?;
    let revoked = store().lock().unwrap().revoke(&token, now_unix());
    if !revoked {
        return Err(unavailable_error());
    }

    Ok(Response::json(
        "{\"status\":\"restricted\",\"capability\":\"share-preview\",\"revoked\":true}".to_string(),
    ))
}

fn store() -> &'static Mutex<ShareRepository> {
    SHARE_STORE.get_or_init(|| Mutex::new(ShareRepository::new()))
}

fn required_query(request: &Request, key: &str) -> Result<String, AppError> {
    request
        .query_value(key)
        .ok_or_else(|| AppError::BadRequest(format!("missing query parameter: {key}")))
}

fn unavailable_error() -> AppError {
    AppError::NotFound(UNAVAILABLE_MESSAGE.to_string())
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn create_result_to_json(created: &ShareCreateResult) -> String {
    format!(
        concat!(
            "{{",
            "\"status\":\"restricted\",",
            "\"capability\":\"share-preview\",",
            "\"storage\":{},",
            "\"token\":{},",
            "\"token_storage\":\"hash-only\",",
            "\"expires_at_unix\":{},",
            "\"public_dto\":{}",
            "}}"
        ),
        json::string(SHARE_STORE_VERSION),
        json::string(&created.token),
        created.record.expires_at_unix,
        public_snapshot_to_json(&created.record.snapshot),
    )
}

fn public_snapshot_to_json(snapshot: &RedactedShareSnapshot) -> String {
    format!(
        concat!(
            "{{",
            "\"status\":\"restricted\",",
            "\"capability\":\"share-preview\",",
            "\"dto_version\":{},",
            "\"label\":{},",
            "\"noindex\":{},",
            "\"editable\":{},",
            "\"chart_snapshot\":{},",
            "\"analysis_snapshot\":{}",
            "}}"
        ),
        json::string(PUBLIC_SHARE_DTO_VERSION),
        json::string(&snapshot.label),
        snapshot.noindex,
        snapshot.editable,
        chart_snapshot_to_json(&snapshot.chart_snapshot),
        analysis_snapshot_to_json(&snapshot.analysis_snapshot),
    )
}

fn chart_snapshot_to_json(snapshot: &crate::domain::cases::ChartSnapshot) -> String {
    format!(
        "{{\"chart_algo_version\":{},\"ruleset_id\":{},\"day_master\":{}}}",
        json::string(&snapshot.chart_algo_version),
        json::string(&snapshot.ruleset_id),
        json::string(&snapshot.day_master),
    )
}

fn analysis_snapshot_to_json(snapshot: &crate::domain::cases::AnalysisSnapshotRef) -> String {
    format!(
        "{{\"analysis_algo_version\":{},\"disclaimer_id\":{}}}",
        json::string(&snapshot.analysis_algo_version),
        json::string(&snapshot.disclaimer_id),
    )
}
