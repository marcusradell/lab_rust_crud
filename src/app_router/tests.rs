#![cfg(test)]
use super::*;

use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt; // for `collect`
use serde_json::{json, Value};
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

#[tokio::test]
async fn json() {
    let router = create();

    let response = router
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/json")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: Value =
        serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap();
    assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
}
