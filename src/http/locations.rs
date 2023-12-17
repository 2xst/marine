use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::{
    app::App,
    domain::{
        error::Result,
        id::Id,
        partners::{Location, NewLocation, NewLocationRequest},
    },
};

pub fn router() -> Router<App> {
    Router::new()
        .route("/:id", get(get_one))
        .route("/:id", delete(delete_location))
        .route("/", post(create_location))
        .route("/", put(update_location))
        .route("/", get(get_many))
}

#[tracing::instrument(skip(app))]
pub async fn get_one(Path(id): Path<Id>, State(app): State<App>) -> Result<Json<Location>> {
    app.get_location(&id).await.map(Json)
}

#[tracing::instrument(skip(app))]
pub async fn create_location(
    user: Id,
    State(app): State<App>,
    Json(location): Json<NewLocationRequest>,
) -> Result<StatusCode> {
    let location = NewLocation {
        partner_id: user,
        country: location.country,
        city: location.city,
        address: location.address,
    };
    app.create_location(&location)
        .await
        .map(|_| StatusCode::CREATED)
}

#[tracing::instrument(skip(app))]
pub async fn update_location(
    user: Id,
    State(app): State<App>,
    Json(location): Json<Location>,
) -> Result<()> {
    app.update_location(&location).await
}

#[tracing::instrument(skip(app))]
pub async fn delete_location(user: Id, Path(id): Path<Id>, State(app): State<App>) -> Result<()> {
    app.delete_location(&id).await
}

#[tracing::instrument(skip(app))]
pub async fn get_many(State(app): State<App>) -> Result<Json<Vec<Location>>> {
    app.get_locations().await.map(Json)
}
