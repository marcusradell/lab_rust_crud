use self::{mock_db::InMemoryDb, repo::Repo, scorecard::Scorecard};
use axum::{
    routing::{get, post},
    Json, Router,
};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

mod db;
mod mock_db;
mod repo;
mod scorecard;
mod tests;

#[derive(Clone)]
pub struct ScorecardsKit {
    repo: Arc<Mutex<InMemoryDb>>,
}

impl ScorecardsKit {
    pub fn new() -> Self {
        Self {
            repo: Arc::new(Mutex::new(InMemoryDb::new())),
        }
    }

    pub async fn list(&self) -> Result<Vec<scorecard::Scorecard>, Box<dyn Error>> {
        self.repo.lock().unwrap().list().await
    }

    pub async fn create(&self, classroom: scorecard::Scorecard) -> Result<(), Box<dyn Error>> {
        self.repo.lock().unwrap().create(classroom).await
    }

    pub fn create_router(&self) -> Router {
        Router::new()
            .route(
                "/list",
                get({
                    let this = self.clone();

                    || async move { Json(this.list().await.unwrap()) }
                }),
            )
            .route(
                "/create",
                post({
                    let this = self.clone();
                    |body: Json<Scorecard>| async move { this.create(body.0).await.unwrap() }
                }),
            )
    }
}
