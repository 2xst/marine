use anyhow::Context;
use libsql::params;

use crate::{
    domain::{
        error::{Error, Result},
        id::Id,
        partners::{Location, NewLocation, Partner},
    },
    telemetry,
};

use super::Database;

impl Database {
    #[tracing::instrument(skip(self))]
    pub async fn get_partner(&self, id: &Id) -> Result<Partner> {
        let mut partner = self
            .connection
            .query(
                "
                 select p.id
                      , p.name
                      , p.password_hash
                 from partners as p
                 where id = ?
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
            .and_then(Partner::try_from)?;
        partner.locations = self.get_partner_locations(&partner.id).await?;
        Ok(partner)
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
            partner.locations = self.get_partner_locations(&partner.id).await?;
        }
        Ok(partners)
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn get_partner_locations(&self, partner_id: &Id) -> Result<Vec<Location>> {
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

    #[tracing::instrument(skip(self), err)]
    pub async fn delete_location(&self, partner_id: &Id) -> Result<()> {
        self.connection
            .execute(
                "
                 delete from locations
                 where id = ?;
                ",
                params!(partner_id.0 as i32),
            )
            .await
            .context("failed to delete from locations")
            .map_err(telemetry::error)?;
        Ok(())
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn create_location(&self, location: &NewLocation) -> Result<()> {
        self.connection
            .execute(
                "
                 insert into locations (country, city, address, partner_id)
                 values (?, ?, ?, ?);
                ",
                params!(
                    location.country.as_str(),
                    location.city.as_str(),
                    location.address.as_str(),
                    location.partner_id.0 as i32
                ),
            )
            .await
            .context("failed to insert into locations")
            .map_err(telemetry::error)?;
        Ok(())
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn update_location(&self, location: &Location) -> Result<()> {
        self.connection
            .execute(
                "
                 update locations
                 set country = ?
                   , city = ?
                   , address = ?
                 where id = ?;
                ",
                params!(
                    location.country.as_str(),
                    location.city.as_str(),
                    location.address.as_str(),
                    location.id.0 as i32
                ),
            )
            .await
            .context("failed to update locations")
            .map_err(telemetry::error)?;
        Ok(())
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn get_locations(&self) -> Result<Vec<Location>> {
        let locations = self
            .connection
            .query(
                "
                 select l.id
                      , l.country
                      , l.city
                      , l.address
                 from locations as l;
                ",
                params!(),
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
