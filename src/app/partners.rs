use crate::domain::{partners::Partner, error::Result, id::Id};

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
}
