#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Context, Result};
use include_dir::{Dir, include_dir};
use minggui_backend::{
    app::{App, parse_and_handle},
    config::AppConfig,
};
use percent_encoding::percent_decode_str;
use std::{
    fs,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::PathBuf,
    sync::Arc,
    thread,
};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[cfg(target_os = "linux")]
use tao::platform::unix::WindowExtUnix;

#[cfg(target_os = "linux")]
use wry::WebViewBuilderExtUnix;

static FRONTEND: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../frontend");
static LUNAR_DATA: &[u8] = include_bytes!("../../data/raw/lunar_data.yaml");

const INIT_SCRIPT: &str = r#"
window.__FATE_TRACK_DESKTOP__ = true;
document.documentElement.classList.add("desktop-shell");
"#;

struct LocalServer {
    _addr: SocketAddr,
    url: String,
}

struct StaticResponse {
    status: &'static str,
    content_type: &'static str,
    cache_control: &'static str,
    body: Vec<u8>,
}

fn main() -> Result<()> {
    let server = spawn_local_server()?;
    println!("命轨桌面壳已启动");
    println!("Local server: {}", server.url);
    run_webview(&server.url)
}

fn run_webview(url: &str) -> Result<()> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("命轨 - Fate Track")
        .with_inner_size(LogicalSize::new(1480.0, 920.0))
        .with_min_inner_size(LogicalSize::new(1180.0, 720.0))
        .with_resizable(true)
        .with_maximized(true)
        .build(&event_loop)
        .context("创建命轨桌面窗口失败")?;

    let builder = WebViewBuilder::new()
        .with_url(url)
        .with_initialization_script(INIT_SCRIPT)
        .with_devtools(cfg!(debug_assertions));

    #[cfg(target_os = "linux")]
    let _webview = builder
        .build_gtk(window.gtk_window())
        .context("创建 Linux WebView 失败")?;

    #[cfg(not(target_os = "linux"))]
    let _webview = builder.build(&window).context("创建 WebView 失败")?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}

fn spawn_local_server() -> Result<LocalServer> {
    let listener = TcpListener::bind(("127.0.0.1", 0)).context("绑定本地端口失败")?;
    let addr = listener.local_addr().context("读取本地端口失败")?;
    let url = format!("http://{}", addr);

    let lunar_data_path = materialize_lunar_data().context("准备农历数据文件失败")?;
    let app = Arc::new(App::new(AppConfig {
        addr: addr.to_string(),
        lunar_data_path,
    }));
    let origin = url.clone();

    thread::Builder::new()
        .name("minggui-desktop-http".to_string())
        .spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let app = Arc::clone(&app);
                        let origin = origin.clone();
                        thread::spawn(move || handle_stream(stream, &app, &origin));
                    }
                    Err(error) => eprintln!("desktop server connection error: {error}"),
                }
            }
        })
        .context("启动桌面本地服务失败")?;

    Ok(LocalServer { _addr: addr, url })
}

fn materialize_lunar_data() -> Result<PathBuf> {
    let dir = std::env::temp_dir().join("fate-track-desktop");
    fs::create_dir_all(&dir).context("创建临时数据目录失败")?;
    let path = dir.join("lunar_data.yaml");
    fs::write(&path, LUNAR_DATA).context("写入内嵌农历数据失败")?;
    Ok(path)
}

fn handle_stream(mut stream: TcpStream, app: &App, origin: &str) {
    let mut buffer = [0_u8; 64 * 1024];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(size) => size,
        Err(error) => {
            eprintln!("desktop request read error: {error}");
            return;
        }
    };
    if bytes_read == 0 {
        return;
    }

    let request_text = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request_line = request_text.lines().next().unwrap_or_default();
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let raw_path = parts.next().unwrap_or("/");

    if raw_path.starts_with("/api/") || method == "OPTIONS" {
        let response = parse_and_handle(app, &buffer[..bytes_read]);
        if let Err(error) = stream.write_all(response.to_http().as_bytes()) {
            eprintln!("desktop api response write error: {error}");
        }
        return;
    }

    if method != "GET" && method != "HEAD" {
        let response = static_error("405 Method Not Allowed", "当前操作方式不支持。");
        let _ = write_static_response(&mut stream, response, method == "HEAD");
        return;
    }

    let response = static_response(raw_path, origin);
    let head_only = method == "HEAD";
    if let Err(error) = write_static_response(&mut stream, response, head_only) {
        eprintln!("desktop static response write error: {error}");
    }
}

fn static_response(raw_path: &str, origin: &str) -> StaticResponse {
    let path_without_query = raw_path.split('?').next().unwrap_or("/");
    let decoded = percent_decode_str(path_without_query).decode_utf8_lossy();
    let mut asset_path = decoded.trim_start_matches('/').to_string();

    if asset_path.is_empty() {
        asset_path = "index.html".to_string();
    }
    if asset_path.ends_with('/') {
        asset_path.push_str("index.html");
    }
    if asset_path.contains("..") || asset_path.contains('\\') {
        return static_error("400 Bad Request", "请求路径不可用。");
    }

    if asset_path == "src/config.js" {
        return StaticResponse {
            status: "200 OK",
            content_type: "text/javascript; charset=utf-8",
            cache_control: "no-store",
            body: desktop_config_js(origin),
        };
    }

    if let Some(file) = FRONTEND.get_file(&asset_path) {
        return StaticResponse {
            status: "200 OK",
            content_type: mime_type(&asset_path),
            cache_control: "no-store",
            body: file.contents().to_vec(),
        };
    }

    static_error("404 Not Found", "未找到可查看内容。")
}

fn desktop_config_js(origin: &str) -> Vec<u8> {
    format!(
        r#"export const defaultApiBase = "{origin}";

const apiBaseStorageKey = "minggui.apiBase";

export function loadApiBase() {{
  return defaultApiBase;
}}

export function saveApiBase(apiBase) {{
  localStorage.setItem(apiBaseStorageKey, apiBase);
}}
"#
    )
    .into_bytes()
}

fn static_error(status: &'static str, message: &'static str) -> StaticResponse {
    StaticResponse {
        status,
        content_type: "text/plain; charset=utf-8",
        cache_control: "no-store",
        body: message.as_bytes().to_vec(),
    }
}

fn write_static_response(
    stream: &mut TcpStream,
    response: StaticResponse,
    head_only: bool,
) -> std::io::Result<()> {
    let headers = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: {}\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n\r\n",
        response.status,
        response.content_type,
        response.body.len(),
        response.cache_control,
    );
    stream.write_all(headers.as_bytes())?;
    if !head_only {
        stream.write_all(&response.body)?;
    }
    Ok(())
}

fn mime_type(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or_default() {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "wasm" => "application/wasm",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_resource_errors_use_product_copy() {
        let bad = static_response("/../secret", "http://127.0.0.1:1");
        assert_eq!(bad.status, "400 Bad Request");
        assert_eq!(String::from_utf8(bad.body).unwrap(), "请求路径不可用。");

        let missing = static_response("/missing-file.html", "http://127.0.0.1:1");
        assert_eq!(missing.status, "404 Not Found");
        assert_eq!(
            String::from_utf8(missing.body).unwrap(),
            "未找到可查看内容。"
        );

        let method = static_error("405 Method Not Allowed", "当前操作方式不支持。");
        assert_eq!(method.status, "405 Method Not Allowed");
        assert_eq!(
            String::from_utf8(method.body).unwrap(),
            "当前操作方式不支持。"
        );
    }
}
