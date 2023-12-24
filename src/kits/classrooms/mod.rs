use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

mod tests;

pub fn create_router() -> Router {
    Router::new()
        .route("/list", get(|| async { Json(json!([])) }))
        .route(
            "/create",
            post(|| async {
                Json(json!({
                        "title": "Fullstack Rust",
                }))
            }),
        )
}
