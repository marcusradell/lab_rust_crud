use crate::io::{routable::Routable, route_path::route_path};
use axum::Router;

pub async fn router(kits: Vec<impl Routable>) -> Router {
    let mut merged_router = Router::new();

    for kit in kits.iter() {
        merged_router = merged_router.merge(kit.router());
    }

    Router::new().nest(route_path!(), merged_router)
}
