mod auth;
mod error_response;
mod health_check;
mod locations;
mod partners;
mod records;

use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;

use crate::{app::App, config::Config};

pub struct HttpServer {
    listener: TcpListener,
    router: Router,
}

impl HttpServer {
    #[tracing::instrument]
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        // Get application state.
        let app = App::new(config.app).await?;
        // Get http multiplexer.
        let router = router().with_state(app);
        // Bind to the address specified in the configuration.
        let addr = SocketAddr::from((config.http.host, config.http.port));
        let listener = TcpListener::bind(addr).await?;
        // Return the server which is ready to start.
        Ok(Self { listener, router })
    }

    #[tracing::instrument(skip(self))]
    pub async fn start(self) -> anyhow::Result<()> {
        // Listen for incoming connections.
        axum::serve(self.listener, self.router)
            .await
            .context("failed to start HTTP server")
    }

    /// Returns the actual address of the server.
    ///
    /// If port is binded to 0, then OS will choose random port.
    /// This method returns the actual port assigned on start.
    pub fn addr(&self) -> anyhow::Result<SocketAddr> {
        self.listener
            .local_addr()
            .context("failed to get TCP listener local address")
    }
}

fn router() -> Router<App> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/partners", partners::router())
        .nest("/locations", locations::router())
        .nest("/records", records::router())
        .nest("/health_check", health_check::router())
}
