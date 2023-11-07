use anyhow::Context;
use async_trait::async_trait;

use crate::{
    domain::{
        error::{Error, Result},
        user::NewUser,
    },
    ports::database::UserDatabase,
    telemetry,
};

use super::{is_unique_violation, MySqlDatabase};

#[async_trait]
impl UserDatabase for MySqlDatabase {
    #[tracing::instrument(skip(self))]
    async fn create_user(&mut self, user: &NewUser) -> Result<()> {
        match sqlx::query!(
            "
            insert into users (
              email,
              password_hash,
              verification_token,
              verified,
              refresh_token
            )
            values (?, ?, ?, ?, ?);
            ",
            user.email,
            user.password_hash,
            user.verification_token,
            user.verified,
            user.refresh_token
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) if is_unique_violation(&e) => {
                Err(Error::EmailTaken).map_err(telemetry::warn)
            }
            Err(e) => Err(e)
                .context("Failed to insert user")
                .map_err(Error::Unexpected)
                .map_err(telemetry::error),
        }
    }
}

#[cfg(test)]
#[cfg(not(feature = "skip-io-tests"))]
mod tests {
    use fake::{Fake, Faker};
    use sqlx::{MySql, Pool};

    use crate::telemetry::init_test_telemetry;

    use super::{Error, MySqlDatabase, NewUser, UserDatabase};

    #[sqlx::test]
    async fn reject_duplicate_email(pool: Pool<MySql>) {
        init_test_telemetry();
        let mut db = MySqlDatabase { pool };
        let user = Faker.fake();
        let res = db.create_user(&user).await;
        assert!(res.is_ok());
        let user = NewUser {
            email: user.email,
            ..Faker.fake()
        };
        let res = db.create_user(&user).await;
        assert!(res.is_err_and(|e| matches!(e, Error::EmailTaken)));
    }
}
