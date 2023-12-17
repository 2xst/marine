use crate::domain::{
    error::Result,
    id::Id,
    partners::{Location, NewLocation, Partner},
};

use super::App;

impl App {
    #[tracing::instrument(skip(self))]
    pub async fn get_partner(&self, id: &Id) -> Result<Partner> {
        self.database.get_partner(id).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_partners(&self) -> Result<Vec<Partner>> {
        self.database.get_partners().await
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_location(&self, id: &Id) -> Result<Location> {
        let location = self
            .database
            .get_locations()
            .await?
            .iter()
            .find(|location| location.id == *id)
            .cloned()
            .ok_or(anyhow::anyhow!("Location not found"))?;
        Ok(location)
    }

    #[tracing::instrument(skip(self))]
    pub async fn create_location(&self, location: &NewLocation) -> Result<()> {
        self.database.create_location(location).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn update_location(&self, location: &Location) -> Result<()> {
        self.database.update_location(location).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn delete_location(&self, id: &Id) -> Result<()> {
        self.database.delete_location(id).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_locations(&self) -> Result<Vec<Location>> {
        self.database.get_locations().await
    }
}
