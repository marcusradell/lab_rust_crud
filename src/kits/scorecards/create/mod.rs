use super::{model::Scorecard, Kit};
use crate::io::db::Db;
use axum::{routing::post, Json, Router};
use sqlx::query;
use std::error::Error;

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

#[async_trait::async_trait]
trait Repo {
    async fn create(&self, scorecard: Scorecard) -> Result<(), Box<dyn Error>>;
}

#[async_trait::async_trait]
impl Repo for Db {
    async fn create(&self, scorecard: Scorecard) -> Result<(), Box<dyn Error>> {
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

pub fn route(kit: Kit) -> Router {
    Router::new().route(
        &format!("/{}", module_path!().split("::").last().unwrap()),
        post(|body: Json<Scorecard>| async move { kit.create(body.0).await.unwrap() }),
    )
}
