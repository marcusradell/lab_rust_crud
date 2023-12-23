#![cfg(test)]
use super::*;

use axum::{
    body::Body,
    extract::connect_info::MockConnectInfo,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt; // for `collect`
use serde_json::{json, Value};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

#[tokio::test]
async fn hello_world() {
    let router = create();

    let response = router
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    assert_eq!(&body[..], b"Hello, World!");
}

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

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
}

#[tokio::test]
async fn not_found() {
    let app = create();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/does-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}

// You can also spawn a server and talk to it like any other HTTP server:
#[tokio::test]
async fn the_real_deal() {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, create()).await.unwrap();
    });

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build_http();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{addr}"))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, World!");
}

// You can use `ready()` and `call()` to avoid using `clone()`
// in multiple request
#[tokio::test]
async fn multiple_request() {
    let mut app = create().into_service();

    let request = Request::builder().uri("/").body(Body::empty()).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let request = Request::builder().uri("/").body(Body::empty()).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

// Here we're calling `/requires-connect-into` which requires `ConnectInfo`
//
// That is normally set with `Router::into_make_service_with_connect_info` but we can't easily
// use that during tests. The solution is instead to set the `MockConnectInfo` layer during
// tests.
#[tokio::test]
async fn with_into_make_service_with_connect_info() {
    let mut app = create()
        .layer(MockConnectInfo(SocketAddr::from(([0, 0, 0, 0], 3000))))
        .into_service();

    let request = Request::builder()
        .uri("/requires-connect-into")
        .body(Body::empty())
        .unwrap();
    let response = app.ready().await.unwrap().call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
