use crate::http::{Response, json};

pub fn health() -> Response {
    Response::json(format!(
        "{{\"service\":\"minggui-backend\",\"status\":\"ok\",\"version\":{}}}",
        json::string(env!("CARGO_PKG_VERSION"))
    ))
}
