use self::{model::Scorecard, repo::Repo};
use crate::io::db::Db;
use axum::{
    routing::{get, post},
    Json, Router,
};
use std::error::Error;

mod model;
mod repo;
mod tests;

#[derive(Clone)]
pub struct Kit {
    repo: Db,
}

impl Kit {
    pub fn new(repo: Db) -> Self {
        Self { repo }
    }

    pub fn name() -> &'static str {
        module_path!().split("::").last().unwrap()
    }

    pub async fn list(&self) -> Result<Vec<model::Scorecard>, Box<dyn Error>> {
        self.repo.list().await
    }

    pub async fn create(&mut self, classroom: model::Scorecard) -> Result<(), Box<dyn Error>> {
        self.repo.create(classroom).await
    }

    pub fn create_router(&self) -> Router {
        let routes = Router::new()
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
            );

        Router::new().nest(&format!("/{}", Self::name()), routes)
    }
}
