use anyhow::Context;
use libsql::params;

use crate::{
    domain::{
        error::Result,
        id::Id,
        records::{NewRecord, Record},
    },
    telemetry,
};

use super::Database;

impl Database {
    #[tracing::instrument(skip(self), err(Debug))]
    pub async fn get_records(&self) -> Result<Vec<Record>> {
        let records = self
            .connection
            .query(
                "
                select r.id
                     , r.date
                     , r.depth
                     , r.max_pressure
                     , r.user_id
                     , r.location_id
                  from records as r;
                ",
                params!(),
            )
            .await
            .context("failed to find records")
            .and_then(|mut rows| {
                let mut partners = Vec::new();
                while let Some(row) = rows.next().context("failed to get row")? {
                    partners.push(Record::try_from(row)?);
                }
                Ok(partners)
            })?;
        Ok(records)
    }

    #[tracing::instrument(skip(self), err(Debug))]
    pub async fn create_record(&self, record: &NewRecord) -> Result<()> {
        self.connection
            .execute(
                "
                insert into records(date, depth, max_pressure, user_id, location_id)
                values ($1, $2, $3, $4, $5);
                ",
                params!(
                    record.date.as_str(),
                    record.depth as i32,
                    record.max_pressure as i32,
                    record.user_id.0 as i32,
                    record.location_id.0 as i32,
                ),
            )
            .await
            .context("failed to insert record")
            .map_err(telemetry::error)?;
        Ok(())
    }

    #[tracing::instrument(skip(self), err(Debug))]
    pub async fn update_record(&self, record: &Record) -> Result<()> {
        self.connection
            .execute(
                "
                update records
                set date = $1
                  , depth = $2
                  , max_pressure = $3
                  , user_id = $4
                  , location_id = $5
                where id = $6;
                ",
                params!(
                    record.date.as_str(),
                    record.depth as i32,
                    record.max_pressure as i32,
                    record.user_id.0 as i32,
                    record.location_id.0 as i32,
                    record.id.0 as i32,
                ),
            )
            .await
            .context("failed to update record")
            .map_err(telemetry::error)?;
        Ok(())
    }

    #[tracing::instrument(skip(self), err(Debug))]
    pub async fn delete_record(&self, id: &Id) -> Result<()> {
        self.connection
            .execute(
                "
                delete from records
                where id = $1;
                ",
                params!(id.0 as i32),
            )
            .await
            .context("failed to delete record")
            .map_err(telemetry::error)?;
        Ok(())
    }
}
