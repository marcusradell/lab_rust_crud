use self::{repo::Repo, scorecard::Scorecard};
use axum::{
    routing::{get, post},
    Json, Router,
};
use std::sync::{Arc, Mutex};

mod repo;
mod scorecard;
mod tests;

#[derive(Clone)]
pub struct ScorecardsKit {
    repo: Arc<Mutex<Repo>>,
}

impl ScorecardsKit {
    pub fn new() -> Self {
        Self {
            repo: Arc::new(Mutex::new(Repo::new())),
        }
    }

    pub fn list(&self) -> Vec<scorecard::Scorecard> {
        self.repo.lock().unwrap().list()
    }

    pub fn create(&self, classroom: scorecard::Scorecard) {
        self.repo.lock().unwrap().create(classroom);
    }

    pub fn create_router(&self) -> Router {
        let list_self = self.clone();
        let create_self = self.clone();

        Router::new()
            .route("/list", get(move || async move { Json(list_self.list()) }))
            .route(
                "/create",
                post(|body: Json<Scorecard>| async move { create_self.create(body.0) }),
            )
    }
}
