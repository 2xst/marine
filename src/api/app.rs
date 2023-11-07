use serde::Deserialize;

use crate::adapters::{
    argon2::{Argon2Config, Argon2Hasher},
    mysql::{MySqlConfig, MySqlDatabase},
};

pub type App = crate::app::App<MySqlDatabase, Argon2Hasher>;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub mysql: MySqlConfig,
    pub argon2: Argon2Config,
}

impl App {
    #[tracing::instrument]
    pub async fn new(config: AppConfig) -> anyhow::Result<Self> {
        let database = MySqlDatabase::new(config.mysql).await?;
        let hasher = Argon2Hasher::new(config.argon2)?;
        Ok(Self::with(database, hasher))
    }
}
