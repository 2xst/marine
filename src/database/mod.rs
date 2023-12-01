mod users;

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

pub async fn connect_to_db(
    config: DatabaseConfig,
) -> anyhow::Result<Connection> {
    use DatabaseConfig as DC;
    let conn = match config {
        DC::Replicated { .. } => {
            unimplemented!("wait until libsql is stabilized")
        }
        DC::Remote { db_url, auth_token } => {
            libsql::Database::open_remote(db_url, auth_token.expose_secret())?
                .connect()?
        }
        DC::Local { db_url } => libsql::Database::open(db_url)?.connect()?,
        DC::Memory => libsql::Database::open_in_memory()?.connect()?,
    };
    Ok(conn)
}
