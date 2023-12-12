use anyhow::Context;
use libsql::params;
use secrecy::ExposeSecret;

use crate::domain::{
    error::{Error, Result},
    user::NewUser,
};

use super::Database;

impl Database {
    #[tracing::instrument(skip(self))]
    pub async fn insert_user(&mut self, user: &NewUser) -> Result<()> {
        match self
            .connection
            .execute(
                "
                insert into users(email, password_hash)
                values ($1, $2);
                ",
                params!(
                    user.email.as_ref(),
                    user.password_hash.expose_secret().as_str()
                ),
            )
            .await
        {
            // Unique constraint violation
            Err(libsql::Error::SqliteFailure(2067, _)) => Err(Error::EmailTaken),
            result => {
                result.context("failed to insert user")?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    use crate::{domain::error::Error, telemetry::init_telemetry};

    use super::{Database, NewUser};

    #[tokio::test]
    async fn insert_successful() {
        init_telemetry().unwrap();
        let mut db = Database::test().await;
        let user = Faker.fake();
        let res = db.insert_user(&user).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn insert_reject_duplicate_email() {
        init_telemetry().unwrap();
        let mut db = Database::test().await;
        let user = Faker.fake();
        db.insert_user(&user).await.ok();
        let user = NewUser {
            email: user.email,
            ..Faker.fake()
        };
        let res = db.insert_user(&user).await;
        assert!(res.is_err_and(|e| matches!(e, Error::EmailTaken)));
    }
}
