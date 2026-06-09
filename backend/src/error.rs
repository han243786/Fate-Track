use crate::http::StatusCode;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppError {
    BadRequest(String),
    Io { context: String, message: String },
    MethodNotAllowed(String),
    NotFound(String),
    OutOfRange(String),
    Unsupported { capability: String, route: String },
}

impl AppError {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BadRequest,
            Self::Io { .. } => StatusCode::InternalServerError,
            Self::MethodNotAllowed(_) => StatusCode::MethodNotAllowed,
            Self::NotFound(_) => StatusCode::NotFound,
            Self::OutOfRange(_) => StatusCode::NotFound,
            Self::Unsupported { .. } => StatusCode::NotImplemented,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "bad_request",
            Self::Io { .. } => "io_error",
            Self::MethodNotAllowed(_) => "method_not_allowed",
            Self::NotFound(_) => "not_found",
            Self::OutOfRange(_) => "out_of_range",
            Self::Unsupported { .. } => "unsupported_capability",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::BadRequest(message) => message.clone(),
            Self::Io { context, message } => format!("{context}: {message}"),
            Self::MethodNotAllowed(method) => format!("method is not allowed: {method}"),
            Self::NotFound(path) => format!("route not found: {path}"),
            Self::OutOfRange(message) => message.clone(),
            Self::Unsupported { capability, route } => {
                format!("capability is planned but not implemented: {capability} at {route}")
            }
        }
    }
}
