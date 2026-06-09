use crate::error::AppError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Method {
    Get,
    Options,
    Other(String),
}

impl Method {
    fn parse(raw: &str) -> Self {
        match raw {
            "GET" => Self::Get,
            "OPTIONS" => Self::Options,
            other => Self::Other(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Get => "GET",
            Self::Options => "OPTIONS",
            Self::Other(method) => method,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: Option<String>,
}

impl Request {
    pub fn parse(bytes: &[u8]) -> Result<Self, AppError> {
        let request = String::from_utf8_lossy(bytes);
        let first_line = request
            .lines()
            .next()
            .ok_or_else(|| AppError::BadRequest("missing request line".to_string()))?;

        let mut parts = first_line.split_whitespace();
        let method = parts
            .next()
            .ok_or_else(|| AppError::BadRequest("missing request method".to_string()))?;
        let target = parts
            .next()
            .ok_or_else(|| AppError::BadRequest("missing request path".to_string()))?;

        let (path, query) = split_target(target);

        Ok(Self {
            method: Method::parse(method),
            path,
            query,
        })
    }

    pub fn query_value(&self, key: &str) -> Option<String> {
        let query = self.query.as_deref()?;
        for pair in query.split('&') {
            let mut parts = pair.splitn(2, '=');
            let pair_key = parts.next()?;
            let pair_value = parts.next().unwrap_or_default();
            if pair_key == key {
                return Some(percent_decode(pair_value));
            }
        }
        None
    }
}

fn split_target(target: &str) -> (String, Option<String>) {
    let mut parts = target.splitn(2, '?');
    let path = parts.next().unwrap_or("/").to_string();
    let query = parts.next().map(ToString::to_string);
    (path, query)
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                let hex = &value[index + 1..index + 3];
                if let Ok(byte) = u8::from_str_radix(hex, 16) {
                    decoded.push(byte);
                    index += 3;
                } else {
                    decoded.push(bytes[index]);
                    index += 1;
                }
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }

    String::from_utf8_lossy(&decoded).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_method_path_and_query() {
        let request = Request::parse(b"GET /api/lunar-data/meta?x=1 HTTP/1.1\r\n\r\n").unwrap();

        assert_eq!(request.method, Method::Get);
        assert_eq!(request.path, "/api/lunar-data/meta");
        assert_eq!(request.query, Some("x=1".to_string()));
    }

    #[test]
    fn reads_query_value() {
        let request =
            Request::parse(b"GET /api/calendar/query?date=2025-01-01 HTTP/1.1\r\n\r\n").unwrap();

        assert_eq!(request.query_value("date"), Some("2025-01-01".to_string()));
    }
}
