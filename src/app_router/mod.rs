use crate::kits::classrooms;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

pub fn create() -> Router {
    let api_router = Router::new().nest("/classrooms", classrooms::create_router());

    Router::new()
        .route("/status", get(|| async {}))
        .nest("/api", api_router)
        .layer(TraceLayer::new_for_http())
}
