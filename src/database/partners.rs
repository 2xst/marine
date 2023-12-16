use anyhow::Context;
use libsql::params;

use crate::{
    domain::{
        error::{Error, Result},
        id::Id,
        partners::{Location, Partner},
    },
    telemetry,
};

use super::Database;

impl Database {
    #[tracing::instrument(skip(self))]
    pub async fn get_partner(&self, id: &Id) -> Result<Partner> {
        self.connection
            .query(
                "
                 select p.id
                      , p.name
                      , p.password_hash
                 from partners as p
                 where name = ?
                 limit 1;
                ",
                params!(id.0 as i32),
            )
            .await
            .context("failed to select from partners")
            .map_err(telemetry::error)?
            .next()
            .context("failed to iterate over rows")
            .map_err(telemetry::error)?
            .ok_or(Error::NotFound)
            .map_err(telemetry::warn)
            .and_then(Partner::try_from)
    }

    #[tracing::instrument(skip(self), err(Debug))]
    pub async fn get_partners(&self) -> Result<Vec<Partner>> {
        let mut partners = self
            .connection
            .query(
                "
                 select p.id
                      , p.name
                      , p.password_hash
                 from partners as p;
                ",
                (),
            )
            .await
            .context("failed to select from partners")
            .and_then(|mut rows| {
                let mut partners = Vec::new();
                while let Some(row) = rows.next().context("failed to get row")? {
                    partners.push(Partner::try_from(row)?);
                }
                Ok(partners)
            })?;
        for partner in partners.iter_mut() {
            partner.locations = self.get_locations(&partner.id).await?;
        }
        Ok(partners)
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn get_locations(&self, partner_id: &Id) -> Result<Vec<Location>> {
        let locations = self
            .connection
            .query(
                "
                 select l.id
                      , l.country
                      , l.city
                      , l.address
                 from locations as l
                 where partner_id = ?;
                ",
                params!(partner_id.0 as i32),
            )
            .await
            .context("failed to select from partners")
            .and_then(|mut rows| {
                let mut locations = Vec::new();
                while let Some(row) = rows.next().context("failed to get row")? {
                    locations.push(Location::try_from(row)?);
                }
                Ok(locations)
            })?;
        Ok(locations)
    }
}
