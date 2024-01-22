use crate::io::{db::Db, routable::Routable, route_path::route_path, router::kit_router};
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

kit_router!(create, list);
