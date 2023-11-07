use crate::{
    domain::{
        error::Result,
        password::{Password, PasswordHash},
    },
    ports::hash::Hasher,
};

mockall::mock! {
    pub Hasher {}

    impl Hasher for Hasher {
        fn hash_password(&self, password: Password) -> Result<PasswordHash>;
        fn verify_password(
            &self,
            password: Password,
            hash: PasswordHash,
        ) -> Result<()>;
    }
}
