use axum::{routing::post, Json, Router};
use serde_json::json;

mod tests;

pub fn create_router() -> Router {
    Router::new().route(
        "/classrooms/create",
        post(|| async {
            Json(json!({
                    "title": "Fullstack Rust",
            }))
        }),
    )
}
