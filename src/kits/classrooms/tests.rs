#![cfg(test)]

use axum::{
    body::Body,
    http::{self, Request},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use super::*;

#[tokio::test]
pub async fn list_classrooms() {
    let router = create_router();

    let request = Request::builder()
        .method(http::Method::GET)
        .uri("/list")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), http::StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body, json!([]));
}

#[tokio::test]
pub async fn create_classroom() {
    let router = create_router();

    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/create")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(
            serde_json::to_vec(&json!({
                "title": "Fullstack Rust",
            }))
            .unwrap(),
        ))
        .unwrap();
    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), http::StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(
        body,
        json!({
                "title": "Fullstack Rust",
        })
    );
}
