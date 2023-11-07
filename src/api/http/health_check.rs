use axum::{http::StatusCode, routing::get, Router};

use crate::api::app::App;

pub fn router() -> Router<App> {
    Router::new().route("/", get(health_check))
}

#[tracing::instrument]
async fn health_check() -> StatusCode {
    StatusCode::OK
}
