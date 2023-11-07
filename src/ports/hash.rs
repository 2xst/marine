use crate::domain::{
    error::Result,
    password::{Password, PasswordHash},
};

pub trait Hasher {
    fn hash_password(&self, password: Password) -> Result<PasswordHash>;
    fn verify_password(
        &self,
        password: Password,
        hash: PasswordHash,
    ) -> Result<()>;
}
