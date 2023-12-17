use anyhow::Context;
use libsql::params;
use secrecy::ExposeSecret;

use crate::{
    domain::{
        error::{Error, Result},
        id::Id,
        user::{NewUser, User},
    },
    telemetry,
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

    #[tracing::instrument(skip(self))]
    pub async fn update_user(&mut self, id: &Id, user: &NewUser) -> Result<()> {
        match self
            .connection
            .execute(
                "
                update users
                   set email = ?
                     , password_hash = ?
                 where id = ?;
                ",
                params!(
                    user.email.as_ref(),
                    user.password_hash.expose_secret().as_str(),
                    id.0 as i32
                ),
            )
            .await
        {
            // Unique constraint violation
            Err(libsql::Error::SqliteFailure(2067, _)) => Err(Error::EmailTaken),
            result => {
                result.context("failed to update user")?;
                Ok(())
            }
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn find_user(&self, ident: &str) -> Result<User> {
        self.connection
            .query(
                "
                select u.id
                     , u.email
                     , u.password_hash
                  from users as u
                 where u.email = ?
                 union 
                select p.id
                     , p.name
                     , p.password_hash
                  from partners as p
                 where p.name = ?
                 limit 1;
                ",
                params!(ident, ident),
            )
            .await
            .context("failed to select from users")
            .map_err(telemetry::error)?
            .next()
            .context("failed to iterate over rows")
            .map_err(telemetry::error)?
            .ok_or(Error::NotFound)
            .map_err(telemetry::warn)
            .and_then(User::try_from)
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

    #[tokio::test]
    async fn find_not_found() {
        init_telemetry().unwrap();
        let db = Database::test().await;
        let res = db.find_user(fake::faker::lorem::en::Word().fake()).await;
        assert!(res.is_err_and(|e| matches!(e, Error::NotFound)));
    }
}
