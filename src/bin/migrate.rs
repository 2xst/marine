use std::str::FromStr;

use anyhow::{anyhow, Context};
use chrono::Utc;
use libsql::params;
use strum::VariantNames;
use strum_macros::{Display, EnumString, EnumVariantNames};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let command = args
        .next()
        .ok_or_else(|| anyhow!("missing command"))
        .and_then(Command::new)?;
    match command {
        Command::New => {
            let migration_name = args
                .next()
                .ok_or_else(|| anyhow!("missing migration name"))?;
            create_migration_files(migration_name)
        }
        Command::Run => {
            let migration_path = args
                .next()
                .ok_or_else(|| anyhow!("missing migration path"))?;
            run_migration(migration_path).await
        }
    }
}

#[derive(Clone, Copy, Debug, Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
enum Command {
    New,
    Run,
}

impl Command {
    fn new(command: String) -> anyhow::Result<Self> {
        Command::from_str(&command).with_context(|| {
            format!(
                "Invalid command, valid options are: {:?}",
                Command::VARIANTS
            )
        })
    }
}

fn create_migration_files(name: String) -> anyhow::Result<()> {
    let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let files = ["up", "down"].map(|filetype| format!("migrations/{now}_{name}.{filetype}.sql"));
    for file in files {
        println!("Creating file: {}", file);
        std::fs::write(file, "")?;
    }
    Ok(())
}

async fn run_migration(path: String) -> anyhow::Result<()> {
    println!("Running migration: {}", path);
    let script = std::fs::read_to_string(path)?;
    let config = marine::Config::init()?.app.database;
    let connection = marine::connect_to_db(config).await?;
    for command in script.trim().trim_end_matches(';').split(';') {
        connection.execute(command, params!()).await?;
    }
    Ok(())
}
