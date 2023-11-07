mod users;

use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlSslMode},
    MySql, Pool,
};

use crate::ports::database::Database;

#[derive(Clone)]
pub struct MySqlDatabase {
    pool: Pool<MySql>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MySqlConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: Secret<String>,
    pub database: String,
    pub require_ssl: bool,
}

impl MySqlDatabase {
    pub async fn new(config: MySqlConfig) -> anyhow::Result<Self> {
        let options = MySqlConnectOptions::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.user)
            .password(config.password.expose_secret())
            .database(&config.database)
            .ssl_mode(if config.require_ssl {
                MySqlSslMode::Required
            } else {
                MySqlSslMode::Preferred
            });
        let pool = Pool::connect_with(options)
            .await
            .context("Failed to connect to the database")?;
        Ok(Self { pool })
    }
}

impl Database for MySqlDatabase {}

fn is_unique_violation(error: &sqlx::Error) -> bool {
    error
        .as_database_error()
        .map(sqlx::error::DatabaseError::kind)
        .is_some_and(|k| k == sqlx::error::ErrorKind::UniqueViolation)
}
