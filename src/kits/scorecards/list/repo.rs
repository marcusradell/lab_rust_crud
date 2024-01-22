use crate::{io::db::Db, kits::scorecards::dto::Scorecard};
use sqlx::query_as;
use std::error::Error;

#[async_trait::async_trait]
pub trait Repo {
    async fn list(&self) -> Result<Vec<Scorecard>, Box<dyn Error>>;
}

#[async_trait::async_trait]
impl Repo for Db {
    async fn list(&self) -> Result<Vec<Scorecard>, Box<dyn Error>> {
        Ok(query_as!(Scorecard, "SELECT * FROM scorecards")
            .fetch_all(&self.pool)
            .await
            .unwrap())
    }
}
