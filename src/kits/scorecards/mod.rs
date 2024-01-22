use crate::io::db::Db;
use axum::Router;

pub mod create;
pub mod list;
pub mod model;
mod tests;

#[derive(Clone)]
pub struct Kit {
    repo: Db,
}

impl Kit {
    pub fn new(repo: Db) -> Self {
        Self { repo }
    }

    pub fn router(&self) -> Router {
        Router::new().nest(
            &format!("/{}", module_path!().split("::").last().unwrap()),
            Router::new()
                .merge(list::route(self.clone()))
                .merge(create::route(self.clone())),
        )
    }
}
