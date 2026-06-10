mod analysis;
mod calendar;
mod capabilities;
mod cases;
mod chart_basis;
mod chart_detail;
mod charts;
mod derive;
mod glossary_data;
mod health;
mod luck;
mod lunar;
mod report;
mod settings;
mod share;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::http::{Method, Request, Response};

pub fn route(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    if !matches!(request.method, Method::Get) {
        return Err(AppError::MethodNotAllowed(
            request.method.as_str().to_string(),
        ));
    }

    match request.path.as_str() {
        "/api/health" => Ok(health::health()),
        "/api/capabilities" => Ok(capabilities::capabilities()),
        "/api/lunar-data/meta" => lunar::meta(config),
        "/api/calendar/query" => calendar::query(config, request),
        "/api/charts/basis/preview" => chart_basis::preview(request),
        "/api/charts" => charts::create(config, request),
        "/api/charts/detail" => chart_detail::detail(config, request),
        "/api/analysis/snapshot" => analysis::snapshot(config, request),
        "/api/luck/cycles" => luck::cycles(config, request),
        "/api/cases" => cases::cases(config, request),
        "/api/cases/export" => cases::export_case(config, request),
        "/api/share/preview" => share::preview(request),
        "/api/settings" => settings::settings(request),
        "/api/glossary" => Ok(glossary_data::glossary(
            request.query_value("term").as_deref(),
            request.query_value("category").as_deref(),
        )),
        "/api/data/derive" => derive::derive(request),
        "/api/charts/report" => report::generate(config, request),
        _ => Err(AppError::NotFound(request.path.clone())),
    }
}
