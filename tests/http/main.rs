#![cfg(not(feature = "skip-io-tests"))]

use std::net::SocketAddr;

use hexagon::{
    api::http::{HttpServer, HttpServerConfig},
    telemetry::init_test_telemetry,
};
use reqwest::{Client, Method, RequestBuilder};

mod health_check;

struct TestServer {
    address: SocketAddr,
    http_client: Client,
}

impl TestServer {
    async fn start() -> anyhow::Result<Self> {
        init_test_telemetry();
        std::env::set_var("HTTP__PORT", "0");
        let config = HttpServerConfig::init()?;
        let server = HttpServer::new(config).await?;
        let address = server.addr()?;
        let http_client = Client::new();
        tokio::spawn(server.start());
        Ok(Self {
            address,
            http_client,
        })
    }

    fn call(&self, method: Method, endpoint: &str) -> RequestBuilder {
        let url = self.endpoint_url(endpoint);
        self.http_client.request(method, url)
    }

    fn endpoint_url(&self, endpoint: &str) -> String {
        format!("http://{}{}", self.address, endpoint)
    }
}
