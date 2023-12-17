mod partners;
mod records;
mod users;

use libsql::Connection;
use secrecy::ExposeSecret;

use crate::config::DatabaseConfig;

#[derive(Clone)]
pub struct Database {
    connection: Connection,
}

impl Database {
    pub async fn new(config: DatabaseConfig) -> anyhow::Result<Self> {
        let connection = connect_to_db(config).await?;
        Ok(Self { connection })
    }

    #[cfg(test)]
    pub async fn test() -> Self {
        use libsql::params;

        let connection = connect_to_db(DatabaseConfig::Memory).await.unwrap();
        let migrations = std::fs::read_dir("./migrations")
            .unwrap()
            .map(Result::unwrap)
            .filter(|path| path.file_name().to_str().unwrap().ends_with(".up.sql"))
            .map(|migration| std::fs::read_to_string(migration.path()).unwrap());
        for sql in migrations {
            connection.execute(&sql, params!()).await.unwrap();
        }
        Self { connection }
    }
}

pub async fn connect_to_db(config: DatabaseConfig) -> anyhow::Result<Connection> {
    use DatabaseConfig as DC;
    let conn = match config {
        DC::Replicated { .. } => {
            unimplemented!("wait until libsql is stabilized")
        }
        DC::Remote { db_url, auth_token } => {
            libsql::Database::open_remote(db_url, auth_token.expose_secret())?.connect()?
        }
        DC::Local { db_path } => libsql::Database::open(db_path)?.connect()?,
        DC::Memory => libsql::Database::open_in_memory()?.connect()?,
    };
    Ok(conn)
}
