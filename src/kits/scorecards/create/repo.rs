use crate::{
    io::{db::Db, result::Result},
    kits::scorecards::model::Scorecard,
};
use sqlx::query;

#[async_trait::async_trait]
pub trait Repo {
    async fn create(&self, scorecard: Scorecard) -> Result<()>;
}

#[async_trait::async_trait]
impl Repo for Db {
    async fn create(&self, scorecard: Scorecard) -> Result<()> {
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
