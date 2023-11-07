use axum::{extract::State, http::StatusCode, routing::post, Form, Router};

use crate::{
    api::app::App,
    domain::{error::Result, user::NewUserRequest},
};

pub fn router() -> Router<App> {
    Router::new().route("/signup", post(signup))
}

#[tracing::instrument(skip(app))]
async fn signup(
    State(mut app): State<App>,
    Form(payload): Form<NewUserRequest>,
) -> Result<StatusCode> {
    app.signup(payload).await.map(|_| StatusCode::OK)
}
