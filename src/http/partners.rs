use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    app::App,
    domain::{error::Result, id::Id, partners::Partner},
};

pub fn router() -> Router<App> {
    Router::new()
        .route("/:id", get(get_one))
        .route("/", get(get_many))
}

#[tracing::instrument(skip(app))]
pub async fn get_one(Path(id): Path<Id>, State(app): State<App>) -> Result<Json<Partner>> {
    app.get_partner(&id).await.map(Json)
}

#[tracing::instrument(skip(app))]
pub async fn get_many(State(app): State<App>) -> Result<Json<Vec<Partner>>> {
    app.get_partners().await.map(Json)
}
