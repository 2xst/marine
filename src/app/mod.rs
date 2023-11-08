mod users;

use crate::{config::AppConfig, database::Database, hasher::Hasher};

#[derive(Clone)]
pub struct App {
    database: Database,
    hasher: Hasher,
}

impl App {
    #[tracing::instrument]
    pub async fn new(config: AppConfig) -> anyhow::Result<Self> {
        let database = Database::new(config.database).await?;
        let hasher = Hasher::new(config.hasher)?;
        Ok(Self { database, hasher })
    }
}
