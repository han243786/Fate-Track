use crate::calendar::civil::CivilDate;
use crate::calendar::lunar_data::{LunarDataSource, LunarDateResult};
use crate::config::AppConfig;
use crate::error::AppError;
use crate::http::{Request, Response, json};

const DATE_LAYER_SOURCE: &str = "android-date-layer-port";
const DATE_LAYER_ALGORITHM_VERSION: &str = "android-date-layer-v1";
const DATE_LAYER_RULESET_ID: &str = "ft-date-layer-android-v1";
const DATE_LAYER_RANGE_START_YEAR: i32 = 1901;
const DATE_LAYER_RANGE_END_YEAR: i32 = 2100;
const DATE_LAYER_YEAR_RULE: &str = "android-lunar-year-base-date";
const DATE_LAYER_MONTH_RULE: &str = "android-fixed-solar-term-month-starts";
const DATE_LAYER_DAY_RULE: &str = "android-day-ganzhi-epoch-1900-01-01-index-10";
const DATE_LAYER_BOUNDARY_POLICY: &str = "date-only-gregorian-query-no-timezone";
const DATE_LAYER_CONFIDENCE: &str = "android-golden-edge-cases";
const DATE_LAYER_LIMITATION_NO_TIME: &str = "no hour pillar, timezone history, or true solar time";
const DATE_LAYER_LIMITATION_RANGE: &str =
    "1901-2100 support only unless a later astronomy engine is validated";

pub fn query(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    let date = request
        .query_value("date")
        .ok_or_else(|| AppError::BadRequest("missing query parameter: date".to_string()))
        .and_then(|value| {
            CivilDate::parse_iso(&value).ok_or_else(|| {
                AppError::BadRequest(
                    "date must use YYYY-MM-DD and be a valid Gregorian date".to_string(),
                )
            })
        })?;

    let table = LunarDataSource::new(config.lunar_data_path.clone()).load_table()?;
    let result = table.lookup(date).ok_or_else(|| {
        AppError::OutOfRange(format!(
            "calendar date out of supported range: {}-{:02}-{:02}",
            date.year, date.month, date.day
        ))
    })?;

    Ok(Response::json(date_layer_to_json(&result)))
}

fn date_layer_to_json(result: &LunarDateResult) -> String {
    format!(
        "{{\"meta\":{},\"gregorian\":{{\"year\":{},\"month\":{},\"day\":{}}},\"lunar\":{{\"year\":{},\"month\":{},\"day\":{},\"month_name\":{},\"day_name\":{},\"is_leap_month\":{}}},\"ganzhi\":{{\"year\":{},\"month\":{},\"day\":{}}},\"zodiac\":{},\"solar_term\":{}}}",
        date_layer_meta_json(),
        result.gregorian.year,
        result.gregorian.month,
        result.gregorian.day,
        result.lunar_year,
        result.lunar_month,
        result.lunar_day,
        json::string(&result.month_name()),
        json::string(&result.day_name()),
        result.is_leap_month,
        json::string(&result.gan_zhi_year),
        json::string(&result.gan_zhi_month),
        json::string(&result.gan_zhi_day),
        json::string(&result.zodiac),
        json::option_string(result.solar_term.as_deref()),
    )
}

fn date_layer_meta_json() -> String {
    format!(
        "{{\"source\":{},\"algorithm_version\":{},\"ruleset_id\":{},\"support_range\":{{\"start_year\":{},\"end_year\":{}}},\"rules\":{{\"year\":{},\"month\":{},\"day\":{}}},\"boundary_policy\":{},\"confidence\":{},\"limitations\":[{},{}]}}",
        json::string(DATE_LAYER_SOURCE),
        json::string(DATE_LAYER_ALGORITHM_VERSION),
        json::string(DATE_LAYER_RULESET_ID),
        DATE_LAYER_RANGE_START_YEAR,
        DATE_LAYER_RANGE_END_YEAR,
        json::string(DATE_LAYER_YEAR_RULE),
        json::string(DATE_LAYER_MONTH_RULE),
        json::string(DATE_LAYER_DAY_RULE),
        json::string(DATE_LAYER_BOUNDARY_POLICY),
        json::string(DATE_LAYER_CONFIDENCE),
        json::string(DATE_LAYER_LIMITATION_NO_TIME),
        json::string(DATE_LAYER_LIMITATION_RANGE),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_layer_json_includes_traceable_metadata() {
        let result = LunarDateResult {
            gregorian: CivilDate {
                year: 2025,
                month: 1,
                day: 1,
            },
            lunar_year: 2024,
            lunar_month: 12,
            lunar_day: 2,
            is_leap_month: false,
            gan_zhi_year: "year-gz".to_string(),
            gan_zhi_month: "month-gz".to_string(),
            gan_zhi_day: "day-gz".to_string(),
            zodiac: "zodiac".to_string(),
            solar_term: None,
        };

        let body = date_layer_to_json(&result);

        assert!(body.starts_with("{\"meta\":"));
        assert!(body.contains("\"source\":\"android-date-layer-port\""));
        assert!(body.contains("\"algorithm_version\":\"android-date-layer-v1\""));
        assert!(body.contains("\"ruleset_id\":\"ft-date-layer-android-v1\""));
        assert!(body.contains("\"start_year\":1901"));
        assert!(body.contains("\"end_year\":2100"));
        assert!(body.contains("\"boundary_policy\":\"date-only-gregorian-query-no-timezone\""));
        assert!(body.contains("\"gregorian\":{\"year\":2025,\"month\":1,\"day\":1}"));
        assert!(body.contains(
            "\"ganzhi\":{\"year\":\"year-gz\",\"month\":\"month-gz\",\"day\":\"day-gz\"}"
        ));
    }
}
