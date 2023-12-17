use axum::{
    extract::State,
    http::StatusCode,
    routing::{post, put},
    Form, Json, Router,
};

use crate::{
    app::App,
    domain::{
        error::Result,
        id::Id,
        user::{AuthTokens, LoginRequest, NewUserRequest},
    },
};

pub fn router() -> Router<App> {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/update", put(update_credentials))
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
    Form(payload): Form<LoginRequest>,
) -> Result<Json<AuthTokens>> {
    app.login(payload).await.map(Json)
}

#[tracing::instrument(skip(app))]
async fn update_credentials(
    user: Id,
    State(mut app): State<App>,
    Form(payload): Form<NewUserRequest>,
) -> Result<()> {
    app.update_credentials(&user, payload).await
}
