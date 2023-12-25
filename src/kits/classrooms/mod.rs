use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Json, Router,
};

use self::{classroom::Classroom, repo::Repo};

mod classroom;
mod repo;
mod tests;

#[derive(Clone)]
pub struct ClassroomsKit {
    repo: Arc<Mutex<Repo>>,
}

impl ClassroomsKit {
    pub fn new() -> Self {
        Self {
            repo: Arc::new(Mutex::new(Repo::new())),
        }
    }

    pub fn list(&self) -> Vec<classroom::Classroom> {
        self.repo.lock().unwrap().list()
    }

    pub fn create(&self, classroom: classroom::Classroom) {
        self.repo.lock().unwrap().create(classroom);
    }

    pub fn create_router(&self) -> Router {
        let list_self = self.clone();
        let create_self = self.clone();

        Router::new()
            .route("/list", get(move || async move { Json(list_self.list()) }))
            .route(
                "/create",
                post(|body: Json<Classroom>| async move { create_self.create(body.0) }),
            )
    }
}
