use std::{path::PathBuf, str::FromStr};

use anyhow::Context;
use config::Config;
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use strum::VariantNames;
use strum_macros::{Display, EnumString, EnumVariantNames};

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Clone, Copy, Debug, Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
enum Environment {
    Local,
    Deployment,
}

pub fn init<C: DeserializeOwned>() -> anyhow::Result<C> {
    CONFIG
        .get_or_try_init(build_config)?
        .clone()
        .try_deserialize::<C>()
        .context("Failed to deserialize configuration")
}

fn build_config() -> anyhow::Result<Config> {
    Config::builder()
        .add_source(config::File::from(config_path(environment()?)?))
        .add_source(config::Environment::default().separator("__"))
        .build()
        .context("Failed to read configuration")
}

fn environment() -> anyhow::Result<Environment> {
    std::env::var("ENVIRONMENT")
        .context("ENVIRONMENT must be present")
        .map(|env| Environment::from_str(env.as_str()))?
        .with_context(|| {
            format!(
                "Failed to determine application environment. \
                    Valid options are : {:?}",
                Environment::VARIANTS
            )
        })
}

fn config_path(environment: Environment) -> anyhow::Result<PathBuf> {
    std::env::current_dir()
        .context("Could not determine current working directory")
        .map(|dir| dir.join("config").join(format!("{environment}.yaml")))
        .context("Failed to get the configuration file")
}
