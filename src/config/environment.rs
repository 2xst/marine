use std::str::FromStr;

use anyhow::Context;
use config::{File, FileFormat, FileSourceFile};
use strum::VariantNames;
use strum_macros::{Display, EnumString, EnumVariantNames};

#[derive(Clone, Copy, Debug, Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    Local,
    Deployment,
}

impl Environment {
    pub fn init() -> anyhow::Result<Self> {
        std::env::var("ENVIRONMENT")
            .context("ENVIRONMENT must be present")
            .map(|env| Environment::from_str(env.as_str()))?
            .with_context(|| {
                format!(
                    "failed to determine application environment. \
                    Valid options are: {:?}",
                    Environment::VARIANTS
                )
            })
    }

    pub fn config_file(&self) -> anyhow::Result<File<FileSourceFile, FileFormat>> {
        std::env::current_dir()
            .context("could not determine current working directory")
            .map(|dir| dir.join("config").join(format!("{self}.yaml")))
            .map(File::from)
            .context("failed to get the configuration file")
    }
}
