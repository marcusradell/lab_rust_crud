use super::{model::Scorecard, Kit};
use axum::{routing::post, Json, Router};
use repo::Repo;
use std::error::Error;

mod repo;

#[async_trait::async_trait]
pub trait Create {
    async fn create(&self, scorecard: Scorecard) -> Result<(), Box<dyn Error>>;
}

#[async_trait::async_trait]
impl Create for Kit {
    async fn create(&self, scorecard: Scorecard) -> Result<(), Box<dyn Error>> {
        self.repo.create(scorecard).await
    }
}

pub fn route(kit: Kit) -> Router {
    Router::new().route(
        &format!("/{}", module_path!().split("::").last().unwrap()),
        post(|body: Json<Scorecard>| async move { kit.create(body.0).await.unwrap() }),
    )
}
