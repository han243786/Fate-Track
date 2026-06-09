use crate::domain::settings::UserPreference;
use crate::error::AppError;
use crate::http::{Request, Response, json};
use std::sync::{Mutex, OnceLock};

static USER_PREFS: OnceLock<Mutex<UserPreference>> = OnceLock::new();

pub fn settings(request: &Request) -> Result<Response, AppError> {
    match request.query_value("action").as_deref().unwrap_or("get") {
        "get" => get_settings(),
        "update" => update_settings(request),
        other => Err(AppError::BadRequest(format!(
            "unsupported settings action: {other}"
        ))),
    }
}

fn get_settings() -> Result<Response, AppError> {
    let preferences = prefs().lock().unwrap().clone();

    Ok(Response::json(preferences_to_json(&preferences)))
}

fn update_settings(request: &Request) -> Result<Response, AppError> {
    let mut preferences = prefs().lock().unwrap();
    preferences
        .update(
            request.query_value("default_calendar"),
            request.query_value("privacy_default"),
            request.query_value("language"),
            request.query_value("theme"),
        )
        .map_err(AppError::BadRequest)?;

    Ok(Response::json(preferences_to_json(&preferences)))
}

fn prefs() -> &'static Mutex<UserPreference> {
    USER_PREFS.get_or_init(|| Mutex::new(UserPreference::default()))
}

fn preferences_to_json(preferences: &UserPreference) -> String {
    format!(
        concat!(
            "{{",
            "\"status\":\"restricted\",",
            "\"capability\":\"settings\",",
            "\"storage\":\"local-volatile-preferences-v1\",",
            "\"default_calendar\":{},",
            "\"privacy_default\":{},",
            "\"language\":{},",
            "\"theme\":{},",
            "\"show_professional_fields\":{},",
            "\"show_nayin\":{},",
            "\"show_void_branches\":{},",
            "\"show_shensha\":{}",
            "}}"
        ),
        json::string(&preferences.default_calendar),
        json::string(&preferences.privacy_default),
        json::string(&preferences.language),
        json::string(&preferences.theme),
        preferences.show_professional_fields,
        preferences.show_nayin,
        preferences.show_void_branches,
        preferences.show_shensha,
    )
}
