use crate::kits::classrooms::ClassroomsKit;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

pub fn create() -> Router {
    let api_router = Router::new().nest("/classrooms", ClassroomsKit::new().create_router());

    Router::new()
        .route("/status", get(|| async {}))
        .nest("/api", api_router)
        .layer(TraceLayer::new_for_http())
}
