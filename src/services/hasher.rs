use anyhow::Context;
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHasher,
    PasswordVerifier, Version,
};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

use crate::domain::{
    error::{Error, Result},
    password::{Password, PasswordHash},
};

#[derive(Clone)]
pub struct Hasher {
    secret: Secret<String>,
    algorithm: Algorithm,
    version: Version,
    params: Params,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HasherConfig {
    pub secret: Secret<String>,
    pub memory_size: u32,
    pub iterations: u32,
    pub parallelism_factor: u32,
    pub output_length: Option<usize>,
}

impl Hasher {
    pub fn new(config: HasherConfig) -> anyhow::Result<Self> {
        let secret = config.secret;
        let algorithm = Algorithm::default();
        let version = Version::default();
        let params = Params::new(
            config.memory_size,
            config.iterations,
            config.parallelism_factor,
            config.output_length,
        )?;
        Ok(Self {
            secret,
            algorithm,
            version,
            params,
        })
    }

    pub fn hash_password(&self, password: Password) -> Result<PasswordHash> {
        let password = password.expose_secret().as_bytes();
        let salt = &SaltString::generate(&mut rand::thread_rng());
        self.hasher()?
            .hash_password(password, salt)
            .map(|h| h.to_string())
            .map(PasswordHash::from)
            .context("Failed to hash password")
            .map_err(Error::Unexpected)
    }

    pub fn _verify_password(
        &self,
        password: Password,
        hash: PasswordHash,
    ) -> Result<()> {
        let password = password.expose_secret().as_bytes();
        let hash = &argon2::PasswordHash::new(hash.expose_secret())
            .context("Failed to parse hash in PHC string format")?;
        self.hasher()?
            .verify_password(password, hash)
            .map_err(|_| Error::InvalidPassword)
    }
    fn hasher(&self) -> anyhow::Result<Argon2<'_>> {
        Argon2::new_with_secret(
            self.secret.expose_secret().as_bytes(),
            self.algorithm,
            self.version,
            self.params.clone(),
        )
        .map_err(anyhow::Error::from)
    }
}
