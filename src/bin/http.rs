use hexagon::{
    api::http::{HttpServer, HttpServerConfig},
    telemetry::init_telemetry,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_telemetry()?;
    let config = HttpServerConfig::init()?;
    let server = HttpServer::new(config).await?;
    server.start().await
}
