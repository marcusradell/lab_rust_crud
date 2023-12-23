use axum::{
    routing::{get, post},
    Json, Router,
};

use tower_http::trace::TraceLayer;

mod tests;

pub fn create() -> Router {
    Router::new()
        .route("/status", get(|| async {}))
        .route(
            "/json",
            post(|payload: Json<serde_json::Value>| async move {
                Json(serde_json::json!({ "data": payload.0 }))
            }),
        )
        // We can still add middleware
        .layer(TraceLayer::new_for_http())
}
