mod users;

use serde::Deserialize;

#[derive(Clone)]
pub struct Database;

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig;

impl Database {
    pub fn new(_config: DatabaseConfig) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}
