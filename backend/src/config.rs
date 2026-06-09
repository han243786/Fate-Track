use std::env;
use std::path::{Path, PathBuf};

pub const DEFAULT_ADDR: &str = "127.0.0.1:8787";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppConfig {
    pub addr: String,
    pub lunar_data_path: PathBuf,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            addr: env::var("FT_BACKEND_ADDR").unwrap_or_else(|_| DEFAULT_ADDR.to_string()),
            lunar_data_path: lunar_data_path_from_env(),
        }
    }
}

fn lunar_data_path_from_env() -> PathBuf {
    if let Ok(path) = env::var("FT_LUNAR_DATA_PATH") {
        return PathBuf::from(path);
    }

    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("data")
        .join("raw")
        .join("lunar_data.yaml")
}
