use anyhow::Context;
use serde::{Deserialize, Serialize};

use super::id::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: Id,
    pub date: String,
    pub depth: u64,
    pub max_pressure: u64,
    pub user_id: Id,
    pub location_id: Id,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewRecord {
    pub date: String,
    pub depth: u64,
    pub max_pressure: u64,
    pub user_id: Id,
    pub location_id: Id,
}

impl TryFrom<libsql::Row> for Record {
    type Error = super::error::Error;

    fn try_from(row: libsql::Row) -> Result<Self, Self::Error> {
        let id = row
            .get::<u64>(0)
            .map(Id::new)
            .context("failed to get id from row")?;
        let date = row
            .get::<String>(1)
            .context("failed to get date from row")?;
        let depth = row.get::<u64>(2).context("failed to get depth from row")?;
        let max_pressure = row
            .get::<u64>(3)
            .context("failed to get max_pressure from row")?;
        let user_id = row
            .get::<u64>(4)
            .map(Id::new)
            .context("failed to get user_id from row")?;
        let location_id = row
            .get::<u64>(5)
            .map(Id::new)
            .context("failed to get location_id from row")?;
        Ok(Self {
            id,
            date,
            depth,
            max_pressure,
            user_id,
            location_id,
        })
    }
}
