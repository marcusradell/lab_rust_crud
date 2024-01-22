use super::{dto::Scorecard, Kit};
use crate::io::{result::Result, route_path::route_path};
use axum::{routing::post, Json, Router};
use repo::Repo;

mod repo;

#[async_trait::async_trait]
pub trait Create {
    async fn create(&self, scorecard: Scorecard) -> Result<()>;
}

#[async_trait::async_trait]
impl Create for Kit {
    async fn create(&self, scorecard: Scorecard) -> Result<()> {
        self.repo.create(scorecard).await
    }
}

pub fn route(kit: Kit) -> Router {
    Router::new().route(
        route_path!(),
        post(|body: Json<Scorecard>| async move { kit.create(body.0).await.unwrap() }),
    )
}
