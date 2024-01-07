use std::error::Error;

use sqlx::{query, query_as};

use super::{db::Db, scorecard::Scorecard};

#[async_trait::async_trait]
pub trait Repo {
    async fn list(&self) -> Result<Vec<Scorecard>, Box<dyn Error>>;
    async fn create(&mut self, scorecard: Scorecard) -> Result<(), Box<dyn Error>>;
}

#[async_trait::async_trait]
impl Repo for Db {
    async fn list(&self) -> Result<Vec<Scorecard>, Box<dyn Error>> {
        Ok(query_as!(Scorecard, "SELECT * FROM scorecards")
            .fetch_all(&self.pool)
            .await
            .unwrap())
    }

    async fn create(&mut self, scorecard: Scorecard) -> Result<(), Box<dyn Error>> {
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
