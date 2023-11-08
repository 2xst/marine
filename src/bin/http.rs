use marine::{init_telemetry, Config, HttpServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_telemetry()?;
    let config = Config::init()?;
    let server = HttpServer::new(config).await?;
    server.start().await
}
