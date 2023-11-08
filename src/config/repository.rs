use anyhow::Context;
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;

use super::environment::Environment;

static CONFIG: OnceCell<config::Config> = OnceCell::new();

pub fn read<C: DeserializeOwned>() -> anyhow::Result<C> {
    CONFIG
        .get_or_try_init(build_config)?
        .clone()
        .try_deserialize::<C>()
        .context("Failed to deserialize configuration")
}

fn build_config() -> anyhow::Result<config::Config> {
    config::Config::builder()
        .add_source(Environment::init()?.config_file()?)
        .add_source(config::Environment::default().separator("__"))
        .build()
        .context("Failed to read configuration")
}
