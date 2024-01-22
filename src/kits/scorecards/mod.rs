use crate::io::{db::Db, routable::Routable};
use axum::Router;

pub mod create;
pub mod dto;
pub mod list;
mod tests;

#[derive(Clone)]
pub struct Kit {
    repo: Db,
}

impl Kit {
    pub fn new(repo: Db) -> Self {
        Self { repo }
    }
}

impl Routable for Kit {
    fn router(&self) -> Router {
        Router::new().nest(
            &format!("/{}", module_path!().split("::").last().unwrap()),
            Router::new()
                .merge(list::route(self.clone()))
                .merge(create::route(self.clone())),
        )
    }
}
