mod users;

use anyhow::anyhow;
use libsql::Connection;
use secrecy::ExposeSecret;

use crate::config::DatabaseConfig;

#[derive(Clone)]
pub struct Database {
    _connection: Connection,
}

impl Database {
    pub async fn new(config: DatabaseConfig) -> anyhow::Result<Self> {
        let connection = connect_to_db(config).await?;
        Ok(Self {
            _connection: connection,
        })
    }
}

// TODO: embedded replica
pub async fn connect_to_db(
    config: DatabaseConfig,
) -> anyhow::Result<Connection> {
    let DatabaseConfig { db_url, auth_token } = config;
    let db = match db_url.as_str() {
        ":memory:" => libsql::Database::open_in_memory()?,
        url if url.starts_with("libsql://") => {
            let auth_token = auth_token
                .ok_or_else(|| anyhow!("Missing database auth token"))?;
            libsql::Database::open_remote(db_url, auth_token.expose_secret())?
        }
        _ => libsql::Database::open(db_url)?,
    };
    db.connect().map_err(anyhow::Error::from)
}
