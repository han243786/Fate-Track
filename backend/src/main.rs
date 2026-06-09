use minggui_backend::{app::App, config::AppConfig, server::TcpServer};

fn main() -> std::io::Result<()> {
    let config = AppConfig::from_env();
    let app = App::new(config.clone());

    println!("minggui backend listening on http://{}", config.addr);
    println!("lunar data path: {}", config.lunar_data_path.display());

    TcpServer::bind(&config.addr)?.serve(app)
}
