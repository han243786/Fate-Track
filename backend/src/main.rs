use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

const DEFAULT_ADDR: &str = "127.0.0.1:8787";

#[derive(Debug)]
struct LunarMeta {
    path: PathBuf,
    bytes: u64,
    modified_unix: u64,
    version: Option<String>,
    epoch: Option<String>,
    term_count: usize,
    year_count: usize,
    min_year: Option<u16>,
    max_year: Option<u16>,
}

fn main() -> std::io::Result<()> {
    let addr = env::var("FT_BACKEND_ADDR").unwrap_or_else(|_| DEFAULT_ADDR.to_string());
    let listener = TcpListener::bind(&addr)?;

    println!("minggui backend listening on http://{addr}");
    println!("lunar data path: {}", lunar_data_path().display());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(error) => eprintln!("connection error: {error}"),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0_u8; 2048];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(size) => size,
        Err(error) => {
            eprintln!("request read error: {error}");
            return;
        }
    };

    if bytes_read == 0 {
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let Some(first_line) = request.lines().next() else {
        write_response(&mut stream, 400, "text/plain; charset=utf-8", "Bad Request");
        return;
    };

    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let raw_path = parts.next().unwrap_or("/");
    let path = raw_path.split('?').next().unwrap_or("/");

    if method == "OPTIONS" {
        write_response(&mut stream, 204, "text/plain; charset=utf-8", "");
        return;
    }

    if method != "GET" {
        write_response(
            &mut stream,
            405,
            "application/json; charset=utf-8",
            r#"{"error":"method_not_allowed"}"#,
        );
        return;
    }

    match path {
        "/api/health" => write_response(
            &mut stream,
            200,
            "application/json; charset=utf-8",
            r#"{"service":"minggui-backend","status":"ok"}"#,
        ),
        "/api/lunar-data/meta" => match read_lunar_meta() {
            Ok(meta) => write_response(
                &mut stream,
                200,
                "application/json; charset=utf-8",
                &meta_to_json(&meta),
            ),
            Err(error) => write_response(
                &mut stream,
                500,
                "application/json; charset=utf-8",
                &format!(
                    r#"{{"error":"lunar_data_unavailable","message":{}}}"#,
                    json_string(&error.to_string())
                ),
            ),
        },
        _ => write_response(
            &mut stream,
            404,
            "application/json; charset=utf-8",
            r#"{"error":"not_found"}"#,
        ),
    }
}

fn write_response(stream: &mut TcpStream, status_code: u16, content_type: &str, body: &str) {
    let status_text = match status_code {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        _ => "OK",
    };

    let response = format!(
        "HTTP/1.1 {status_code} {status_text}\r\n\
         Content-Type: {content_type}\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type\r\n\
         Connection: close\r\n\r\n{}",
        body.as_bytes().len(),
        body
    );

    if let Err(error) = stream.write_all(response.as_bytes()) {
        eprintln!("response write error: {error}");
    }
}

fn lunar_data_path() -> PathBuf {
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

fn read_lunar_meta() -> std::io::Result<LunarMeta> {
    let path = lunar_data_path();
    let content = fs::read_to_string(&path)?;
    let metadata = fs::metadata(&path)?;
    let modified_unix = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs())
        .unwrap_or(0);

    let mut version = None;
    let mut epoch = None;
    let mut term_count = 0;
    let mut year_count = 0;
    let mut min_year = None;
    let mut max_year = None;
    let mut in_years = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(value) = trimmed.strip_prefix("version:") {
            version = Some(value.trim().to_string());
        } else if let Some(value) = trimmed.strip_prefix("epoch:") {
            epoch = Some(value.trim().to_string());
        } else if let Some(value) = trimmed.strip_prefix("term_names:") {
            term_count = value.matches('\'').count() / 2;
        } else if trimmed == "years:" {
            in_years = true;
        } else if in_years && line.starts_with("  \"") {
            if let Some(year) = trimmed
                .trim_matches(':')
                .trim_matches('"')
                .parse::<u16>()
                .ok()
            {
                year_count += 1;
                min_year = Some(min_year.map_or(year, |current: u16| current.min(year)));
                max_year = Some(max_year.map_or(year, |current: u16| current.max(year)));
            }
        }
    }

    Ok(LunarMeta {
        path,
        bytes: metadata.len(),
        modified_unix,
        version,
        epoch,
        term_count,
        year_count,
        min_year,
        max_year,
    })
}

fn meta_to_json(meta: &LunarMeta) -> String {
    format!(
        "{{\"path\":{},\"bytes\":{},\"modified_unix\":{},\"version\":{},\"epoch\":{},\"term_count\":{},\"year_count\":{},\"min_year\":{},\"max_year\":{}}}",
        json_string(&meta.path.display().to_string()),
        meta.bytes,
        meta.modified_unix,
        option_string_json(meta.version.as_deref()),
        option_string_json(meta.epoch.as_deref()),
        meta.term_count,
        meta.year_count,
        option_number_json(meta.min_year),
        option_number_json(meta.max_year),
    )
}

fn option_string_json(value: Option<&str>) -> String {
    value.map(json_string).unwrap_or_else(|| "null".to_string())
}

fn option_number_json(value: Option<u16>) -> String {
    value
        .map(|number| number.to_string())
        .unwrap_or_else(|| "null".to_string())
}

fn json_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('"');
    for ch in value.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            other => escaped.push(other),
        }
    }
    escaped.push('"');
    escaped
}
