mod auth;
mod error_response;
mod health_check;

use std::net::{SocketAddr, TcpListener};

use anyhow::Context;
use axum::Router;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_with::serde_as;

use crate::{api::app::App, config};

use super::app::AppConfig;

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
struct HttpConfig {
    pub host: [u8; 4],
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HttpServerConfig {
    #[serde(flatten)]
    app: AppConfig,
    http: HttpConfig,
}

pub struct HttpServer {
    listener: TcpListener,
    router: Router,
}

impl HttpServer {
    #[tracing::instrument]
    pub async fn new(config: HttpServerConfig) -> anyhow::Result<Self> {
        let app = App::new(config.app).await?;
        let router = router().with_state(app);
        let addr = SocketAddr::from((config.http.host, config.http.port));
        let listener = TcpListener::bind(addr)?;
        Ok(Self { listener, router })
    }

    #[tracing::instrument(skip(self))]
    pub async fn start(self) -> anyhow::Result<()> {
        axum::Server::from_tcp(self.listener)?
            .serve(self.router.into_make_service())
            .await?;
        Ok(())
    }

    pub fn addr(&self) -> anyhow::Result<SocketAddr> {
        self.listener
            .local_addr()
            .context("Failed to get TCP listener local address")
    }
}

fn router() -> Router<App> {
    let api_router = Router::new()
        .nest("/auth", auth::router())
        .nest("/health_check", health_check::router());
    Router::new().nest("/api", api_router)
}

impl HttpServerConfig {
    pub fn init() -> anyhow::Result<Self> {
        config::init()
    }
}
