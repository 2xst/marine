use anyhow::Context;
use serde::Serialize;

use super::{id::Id, password::PasswordHash, sensitive::Sensitive};

#[derive(Clone, Debug, Serialize)]
pub struct Partner {
    pub id: Id,
    pub name: String,
    #[serde(skip_serializing)]
    pub password_hash: PasswordHash,
    pub locations: Vec<Location>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Location {
    pub id: Id,
    pub country: String,
    pub city: String,
    pub address: String,
}

impl TryFrom<libsql::Row> for Partner {
    type Error = super::error::Error;

    fn try_from(row: libsql::Row) -> Result<Self, Self::Error> {
        let id = row
            .get::<u64>(0)
            .map(Id::new)
            .context("failed to get id from row")?;
        let name = row
            .get::<String>(1)
            .context("failed to get name from row")?;
        let password_hash = row
            .get::<String>(2)
            .map(Sensitive::new)
            .context("failed to get password hash from row")?;
        Ok(Self {
            id,
            name,
            password_hash,
            locations: vec![],
        })
    }
}
impl TryFrom<libsql::Row> for Location {
    type Error = super::error::Error;

    fn try_from(row: libsql::Row) -> Result<Self, Self::Error> {
        let id = row
            .get::<u64>(0)
            .map(Id::new)
            .context("failed to get id from row")?;
        let country = row
            .get::<String>(1)
            .context("failed to get country from row")?;
        let city = row
            .get::<String>(2)
            .context("failed to get city from row")?;
        let address = row
            .get::<String>(3)
            .context("failed to get address from row")?;
        Ok(Self {
            id,
            country,
            city,
            address,
        })
    }
}
