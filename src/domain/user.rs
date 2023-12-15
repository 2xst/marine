use anyhow::Context;
use secrecy::Secret;
use serde::{Deserialize, Serialize};

use super::{email::Email, id::Id, password::PasswordHash, sensitive::Sensitive};

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct NewUserRequest {
    pub email: String,
    #[cfg_attr(test, dummy(faker = "super::password::FakePassword"))]
    pub password: Secret<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct NewUser {
    pub email: Email,
    pub password_hash: PasswordHash,
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct User {
    pub id: Id,
    pub email: Email,
    pub password_hash: PasswordHash,
}

#[derive(Clone, Debug, Serialize)]
pub struct AuthTokens {
    pub access_token: String,
}

impl TryFrom<libsql::Row> for User {
    type Error = super::error::Error;

    fn try_from(row: libsql::Row) -> Result<Self, Self::Error> {
        let id = row
            .get::<u64>(0)
            .map(Id::new)
            .context("failed to get id from row")?;
        let email = row
            .get::<String>(1)
            .map(Email::new_unchecked)
            .context("failed to get email from row")?;
        let password_hash = row
            .get::<String>(2)
            .map(Sensitive::new)
            .context("failed to get password hash from row")?;
        Ok(Self {
            id,
            email,
            password_hash,
        })
    }
}
