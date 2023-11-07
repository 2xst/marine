mod users;

use serde::Deserialize;

use crate::services::{
    database::{Database, DatabaseConfig},
    hasher::{Hasher, HasherConfig},
};

#[derive(Clone)]
pub struct App {
    database: Database,
    hasher: Hasher,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub hasher: HasherConfig,
}

impl App {
    #[tracing::instrument]
    pub async fn new(config: AppConfig) -> anyhow::Result<Self> {
        let database = Database::new(config.database)?;
        let hasher = Hasher::new(config.hasher)?;
        Ok(Self { database, hasher })
    }
}
