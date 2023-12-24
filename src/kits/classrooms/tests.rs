#![cfg(test)]

use super::*;
use axum::{
    body::Body,
    http::{self, Request},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::{Service, ServiceExt};

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
    let mut router = create_router();

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

    let router = ServiceExt::<Request<Body>>::ready(&mut router)
        .await
        .unwrap();

    let response = router.call(request).await.unwrap();

    assert_eq!(response.status(), http::StatusCode::OK);

    let request = Request::builder()
        .method(http::Method::GET)
        .uri("/list")
        .body(Body::empty())
        .unwrap();

    let response = router.call(request).await.unwrap();

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body, json!([]));
}
