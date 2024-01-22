use self::repo::Repo;
use crate::io::db::Db;
use axum::{routing::get, Json, Router};
use std::error::Error;

mod create;
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

    pub async fn list(&self) -> Result<Vec<model::Scorecard>, Box<dyn Error>> {
        self.repo.list().await
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
            .merge(create::route(self.clone()));

        let router = Router::new().nest(
            &format!("/{}", module_path!().split("::").last().unwrap()),
            routes,
        );

        router
    }
}
