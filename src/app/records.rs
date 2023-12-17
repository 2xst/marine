use crate::domain::{
    error::Result,
    id::Id,
    records::{NewRecord, Record},
};

use super::App;

impl App {
    #[tracing::instrument(skip(self))]
    pub async fn get_records(&self, user_id: &Id) -> Result<Vec<Record>> {
        let records = self
            .database
            .get_records()
            .await?
            .iter()
            .filter(|record| record.user_id == *user_id)
            .cloned()
            .collect::<Vec<_>>();
        Ok(records)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_record(&self, record_id: &Id) -> Result<Record> {
        let records = self
            .database
            .get_records()
            .await?
            .iter()
            .find(|record| record.id == *record_id)
            .cloned()
            .ok_or(anyhow::anyhow!("Records not found"))?;
        Ok(records)
    }

    #[tracing::instrument(skip(self))]
    pub async fn create_record(&self, record: &NewRecord) -> Result<()> {
        self.database.create_record(record).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn update_record(&self, record: &Record) -> Result<()> {
        self.database.update_record(record).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn delete_record(&self, id: &Id) -> Result<()> {
        self.database.delete_record(id).await
    }
}
