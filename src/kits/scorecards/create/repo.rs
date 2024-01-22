use crate::{io::db::Db, kits::scorecards::model::Scorecard};
use sqlx::query;
use std::error::Error;

#[async_trait::async_trait]
pub trait Repo {
    async fn create(&self, scorecard: Scorecard) -> Result<(), Box<dyn Error>>;
}

#[async_trait::async_trait]
impl Repo for Db {
    async fn create(&self, scorecard: Scorecard) -> Result<(), Box<dyn Error>> {
        query!(
            "INSERT INTO scorecards (id, full_name) VALUES ($1, $2)",
            scorecard.id,
            scorecard.full_name
        )
        .execute(&self.pool)
        .await
        .unwrap();

        Ok(())
    }
}
