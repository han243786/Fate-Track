#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StatusCode {
    Ok,
    NoContent,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
    NotImplemented,
}

impl StatusCode {
    pub fn code(self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::NoContent => 204,
            Self::BadRequest => 400,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
        }
    }

    pub fn reason(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::NoContent => "No Content",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::MethodNotAllowed => "Method Not Allowed",
            Self::InternalServerError => "Internal Server Error",
            Self::NotImplemented => "Not Implemented",
        }
    }
}
