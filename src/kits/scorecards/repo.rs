use std::error::Error;

use sqlx::query_as;

use super::model::Scorecard;
use crate::io::db::Db;

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
