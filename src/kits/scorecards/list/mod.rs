use self::repo::Repo;
use super::{model::Scorecard, Kit};
use crate::io::result::Result;
use axum::{routing::get, Json, Router};

mod repo;

#[async_trait::async_trait]
pub trait List {
    async fn list(&self) -> Result<Vec<Scorecard>>;
}

#[async_trait::async_trait]
impl List for Kit {
    async fn list(&self) -> Result<Vec<Scorecard>> {
        self.repo.list().await
    }
}

pub fn route(kit: Kit) -> Router {
    Router::new().route(
        &format!("/{}", module_path!().split("::").last().unwrap()),
        get({
            || async move {
                let scorecards = kit.list().await.unwrap();
                Json(scorecards)
            }
        }),
    )
}
