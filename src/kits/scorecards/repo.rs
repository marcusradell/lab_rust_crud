use std::error::Error;

use sqlx::query_as;

use super::{db::Db, mock_db::InMemoryDb, scorecard::Scorecard};

#[async_trait::async_trait]
pub trait Repo {
    async fn list(&self) -> Result<Vec<Scorecard>, Box<dyn Error>>;
    async fn create(&mut self, scorecard: Scorecard) -> Result<(), Box<dyn Error>>;
}

#[async_trait::async_trait]
impl Repo for InMemoryDb {
    async fn list(&self) -> Result<Vec<Scorecard>, Box<dyn Error>> {
        Ok(self.data.values().cloned().collect())
    }

    async fn create(&mut self, scorecard: Scorecard) -> Result<(), Box<dyn Error>> {
        self.data.insert(scorecard.full_name.clone(), scorecard);
        Ok(())
    }
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
        todo!()
    }
}
