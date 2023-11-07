use secrecy::Secret;
use serde::Deserialize;

use super::{email::Email, id::Id, password::PasswordHash, token::Token};

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
    pub verification_token: Token,
    #[cfg_attr(test, dummy(faker = "false"))]
    pub verified: bool,
    pub refresh_token: Token,
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct User {
    pub id: Id,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub verification_token: Token,
    #[cfg_attr(test, dummy(faker = "false"))]
    pub verified: bool,
    pub refresh_token: Token,
}
