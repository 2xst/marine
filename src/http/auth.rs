use axum::{extract::State, http::StatusCode, routing::post, Form, Json, Router};

use crate::{
    app::App,
    domain::{
        error::Result,
        user::{AuthTokens, NewUserRequest},
    },
};

pub fn router() -> Router<App> {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}

#[tracing::instrument(skip(app))]
async fn signup(
    State(mut app): State<App>,
    Form(payload): Form<NewUserRequest>,
) -> Result<StatusCode> {
    app.signup(payload).await.map(|_| StatusCode::CREATED)
}

#[tracing::instrument(skip(app))]
async fn login(
    State(mut app): State<App>,
    Form(payload): Form<NewUserRequest>,
) -> Result<Json<AuthTokens>> {
    app.login(payload).await.map(Json)
}
