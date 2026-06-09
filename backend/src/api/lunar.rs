use crate::calendar::lunar_data::{LunarDataMeta, LunarDataSource};
use crate::config::AppConfig;
use crate::error::AppError;
use crate::http::{Response, json};

pub fn meta(config: &AppConfig) -> Result<Response, AppError> {
    let source = LunarDataSource::new(config.lunar_data_path.clone());
    let meta = source.meta()?;
    Ok(Response::json(meta_to_json(&meta)))
}

fn meta_to_json(meta: &LunarDataMeta) -> String {
    format!(
        "{{\"path\":{},\"bytes\":{},\"modified_unix\":{},\"version\":{},\"epoch\":{},\"term_count\":{},\"year_count\":{},\"min_year\":{},\"max_year\":{}}}",
        json::string(&meta.path.display().to_string()),
        meta.bytes,
        meta.modified_unix,
        json::option_string(meta.version.as_deref()),
        json::option_string(meta.epoch.as_deref()),
        meta.term_count,
        meta.year_count,
        json::option_u16(meta.min_year),
        json::option_u16(meta.max_year),
    )
}
