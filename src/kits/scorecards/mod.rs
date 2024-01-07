use self::{db::Db, repo::Repo, scorecard::Scorecard};
use axum::{
    routing::{get, post},
    Json, Router,
};
use std::error::Error;

mod db;
mod mock_db;
mod repo;
mod scorecard;
mod tests;

#[derive(Clone)]
pub struct ScorecardsKit {
    repo: Db,
}

impl ScorecardsKit {
    pub async fn new() -> Self {
        Self {
            repo: Db::new().await,
        }
    }

    pub async fn list(&self) -> Result<Vec<scorecard::Scorecard>, Box<dyn Error>> {
        self.repo.list().await
    }

    pub async fn create(&mut self, classroom: scorecard::Scorecard) -> Result<(), Box<dyn Error>> {
        self.repo.create(classroom).await
    }

    pub fn create_router(&self) -> Router {
        Router::new()
            .route(
                "/list",
                get({
                    let this = self.clone();
                    || async move {
                        let scorecards = this.list().await.unwrap();
                        Json(scorecards)
                    }
                }),
            )
            .route(
                "/create",
                post({
                    let mut this = self.clone();
                    |body: Json<Scorecard>| async move { this.create(body.0).await.unwrap() }
                }),
            )
    }
}
