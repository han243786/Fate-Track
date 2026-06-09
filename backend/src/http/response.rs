use crate::error::AppError;
use crate::http::StatusCode;
use crate::http::json;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    pub status: StatusCode,
    pub content_type: &'static str,
    pub body: String,
}

impl Response {
    pub fn json(body: String) -> Self {
        Self {
            status: StatusCode::Ok,
            content_type: "application/json; charset=utf-8",
            body,
        }
    }

    pub fn no_content() -> Self {
        Self {
            status: StatusCode::NoContent,
            content_type: "text/plain; charset=utf-8",
            body: String::new(),
        }
    }

    pub fn json_error(error: AppError) -> Self {
        let status = error.status();
        let body = format!(
            "{{\"error\":{},\"message\":{}}}",
            json::string(error.code()),
            json::string(&error.message())
        );

        Self {
            status,
            content_type: "application/json; charset=utf-8",
            body,
        }
    }

    pub fn to_http(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: {}\r\n\
             Content-Length: {}\r\n\
             Access-Control-Allow-Origin: *\r\n\
             Access-Control-Allow-Methods: GET, OPTIONS\r\n\
             Access-Control-Allow-Headers: Content-Type\r\n\
             Connection: close\r\n\r\n{}",
            self.status.code(),
            self.status.reason(),
            self.content_type,
            self.body.as_bytes().len(),
            self.body
        )
    }
}
