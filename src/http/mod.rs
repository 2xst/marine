mod auth;
mod error_response;
mod health_check;

use std::net::{SocketAddr, TcpListener};

use anyhow::Context;
use axum::Router;

use crate::{app::App, config::Config};

pub struct HttpServer {
    listener: TcpListener,
    router: Router,
}

impl HttpServer {
    #[tracing::instrument]
    pub async fn new(config: Config) -> anyhow::Result<Self> {
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
            .context("failed to get TCP listener local address")
    }
}

fn router() -> Router<App> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/health_check", health_check::router())
}
