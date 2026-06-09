use crate::calendar::civil::CivilDate;
use crate::domain::bazi::{
    BirthProfile, BirthTime, CalendarKind, ChartBasis, ChartRequest, PrivacyLevel, Sex,
    TimePrecision,
};
use crate::error::AppError;
use crate::http::{Request, Response, json};

pub fn preview(request: &Request) -> Result<Response, AppError> {
    let chart_request = chart_request_from_query(request)?;
    let basis = ChartBasis::build(chart_request)?;

    Ok(Response::json(chart_basis_to_json(&basis)))
}

pub(crate) fn chart_request_from_query(request: &Request) -> Result<ChartRequest, AppError> {
    let date = required_query(request, "date")?;
    let date = CivilDate::parse_iso(&date)
        .ok_or_else(|| AppError::BadRequest("date must use YYYY-MM-DD".to_string()))?;

    let time_precision = TimePrecision::parse(request.query_value("time_precision").as_deref())?;
    let time = match request.query_value("time") {
        Some(value) if !value.is_empty() => Some(BirthTime::parse(&value).ok_or_else(|| {
            AppError::BadRequest("time must use HH:MM in 24-hour format".to_string())
        })?),
        _ => None,
    };

    let timezone = required_query(request, "timezone")?;
    let use_true_solar_time = parse_bool_query(request, "true_solar_time", false)?;
    let is_leap_month = parse_bool_query(request, "is_leap_month", false)?;

    Ok(ChartRequest {
        birth_profile: BirthProfile {
            display_name: request
                .query_value("display_name")
                .filter(|value| !value.is_empty()),
            sex: Sex::parse(request.query_value("sex").as_deref())?,
            privacy_level: PrivacyLevel::parse(request.query_value("privacy").as_deref())?,
            calendar_kind: CalendarKind::parse(request.query_value("calendar").as_deref())?,
            date,
            time,
            time_precision,
            timezone,
            is_leap_month,
            use_true_solar_time,
        },
    })
}

fn required_query(request: &Request, key: &str) -> Result<String, AppError> {
    request
        .query_value(key)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| AppError::BadRequest(format!("missing query parameter: {key}")))
}

fn parse_bool_query(request: &Request, key: &str, default: bool) -> Result<bool, AppError> {
    match request.query_value(key).as_deref() {
        None | Some("") => Ok(default),
        Some("true") => Ok(true),
        Some("false") => Ok(false),
        Some(other) => Err(AppError::BadRequest(format!(
            "{key} must be true or false, got {other}"
        ))),
    }
}

fn chart_basis_to_json(basis: &ChartBasis) -> String {
    let profile = &basis.request.birth_profile;
    format!(
        concat!(
            "{{",
            "\"status\":\"restricted\",",
            "\"capability\":\"chart-basis-preview\",",
            "\"birth_profile\":{},",
            "\"calculation_metadata\":{},",
            "\"validated_range\":{{\"start_year\":{},\"end_year\":{}}},",
            "\"supported_outputs\":{},",
            "\"unsupported_outputs\":{}",
            "}}"
        ),
        birth_profile_to_json(profile),
        metadata_to_json(&basis.metadata),
        basis.validated_range_start_year,
        basis.validated_range_end_year,
        string_array_to_json(&basis.supported_outputs),
        string_array_to_json(&basis.unsupported_outputs),
    )
}

pub(crate) fn birth_profile_to_json(profile: &BirthProfile) -> String {
    let time = profile
        .time
        .map(|time| format!("{:02}:{:02}", time.hour, time.minute));
    format!(
        concat!(
            "{{",
            "\"display_name\":{},",
            "\"sex\":{},",
            "\"privacy_level\":{},",
            "\"calendar_kind\":{},",
            "\"date\":\"{:04}-{:02}-{:02}\",",
            "\"time\":{},",
            "\"time_precision\":{},",
            "\"timezone\":{},",
            "\"is_leap_month\":{},",
            "\"use_true_solar_time\":{}",
            "}}"
        ),
        json::option_string(profile.display_name.as_deref()),
        json::string(profile.sex.as_str()),
        json::string(profile.privacy_level.as_str()),
        json::string(profile.calendar_kind.as_str()),
        profile.date.year,
        profile.date.month,
        profile.date.day,
        json::option_string(time.as_deref()),
        json::string(profile.time_precision.as_str()),
        json::string(&profile.timezone),
        profile.is_leap_month,
        profile.use_true_solar_time,
    )
}

pub(crate) fn metadata_to_json(metadata: &crate::domain::bazi::CalculationMetadata) -> String {
    format!(
        concat!(
            "{{",
            "\"ruleset_id\":{},",
            "\"algo_version\":{},",
            "\"date_layer_ruleset_id\":{},",
            "\"year_boundary_rule\":{},",
            "\"month_boundary_rule\":{},",
            "\"day_boundary_rule\":{},",
            "\"hour_policy\":{},",
            "\"timezone_policy\":{},",
            "\"true_solar_time_policy\":{},",
            "\"lunar_input_policy\":{}",
            "}}"
        ),
        json::string(metadata.ruleset_id),
        json::string(metadata.algo_version),
        json::string(metadata.date_layer_ruleset_id),
        json::string(metadata.year_boundary_rule),
        json::string(metadata.month_boundary_rule),
        json::string(metadata.day_boundary_rule),
        json::string(metadata.hour_policy),
        json::string(metadata.timezone_policy),
        json::string(metadata.true_solar_time_policy),
        json::string(metadata.lunar_input_policy),
    )
}

pub(crate) fn string_array_to_json(values: &[&str]) -> String {
    let items = values
        .iter()
        .map(|value| json::string(value))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}
