use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::{
    app::App,
    domain::{
        error::Result,
        id::Id,
        records::{NewRecord, Record},
    },
};

pub fn router() -> Router<App> {
    Router::new()
        .route("/:id", get(get_one))
        .route("/:id", delete(delete_record))
        .route("/", post(create_record))
        .route("/", put(update_record))
        .route("/", get(get_many))
        .route("/stress_koefficient", get(stress_koefficient))
}

#[tracing::instrument(skip(app))]
pub async fn get_one(Path(id): Path<Id>, State(app): State<App>) -> Result<Json<Record>> {
    app.get_record(&id).await.map(Json)
}

#[tracing::instrument(skip(app))]
pub async fn create_record(
    user: Id,
    State(app): State<App>,
    Json(record): Json<NewRecord>,
) -> Result<StatusCode> {
    app.create_record(&record)
        .await
        .map(|_| StatusCode::CREATED)
}

#[tracing::instrument(skip(app))]
pub async fn update_record(
    user: Id,
    State(app): State<App>,
    Json(record): Json<Record>,
) -> Result<()> {
    app.update_record(&record).await
}

#[tracing::instrument(skip(app))]
pub async fn delete_record(user: Id, Path(id): Path<Id>, State(app): State<App>) -> Result<()> {
    app.delete_record(&id).await
}

#[derive(Debug, serde::Deserialize)]
pub struct Quer {
    user: Id,
}

#[tracing::instrument(skip(app))]
pub async fn get_many(Query(q): Query<Quer>, State(app): State<App>) -> Result<Json<Vec<Record>>> {
    app.get_records(&q.user).await.map(Json)
}

#[derive(Debug, serde::Serialize)]
pub struct Stuff {
    record: Record,
    stress_koefficient: f64,
}

#[tracing::instrument(skip(app))]
pub async fn stress_koefficient(
    Query(q): Query<Quer>,
    State(app): State<App>,
) -> Result<Json<Stuff>> {
    app.get_records(&q.user)
        .await
        .map(|v| v.into_iter().max_by_key(|x| x.max_pressure).unwrap())
        .map(|r| Stuff {
            record: r.clone(),
            stress_koefficient: (r.max_pressure as f64 / r.depth as f64 / 8492f64 * 100f64) as u64
                as f64
                / 100f64,
        })
        .map(Json)
}
