mod environment;
mod repository;

use secrecy::Secret;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_with::serde_as;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub app: AppConfig,
    pub http: HttpConfig,
}

impl Config {
    pub fn init() -> anyhow::Result<Self> {
        repository::read()
    }
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct HttpConfig {
    pub host: [u8; 4],
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub hasher: HasherConfig,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum DatabaseConfig {
    Replicated {
        db_path: String,
        sync_url: String,
        auth_token: Secret<String>,
    },
    Remote {
        db_url: String,
        auth_token: Secret<String>,
    },
    Local {
        db_url: String,
    },
    Memory,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HasherConfig {
    pub secret: Secret<String>,
    pub memory_size: u32,
    pub iterations: u32,
    pub parallelism_factor: u32,
    pub output_length: Option<usize>,
}
