use axum::routing::get;
use tower_http::trace::TraceLayer;

mod io;
mod kits;

pub async fn lib() {
    io::logging::init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    let db = crate::io::db::Db::new().await;

    let api_router = kits::api::router(db).await;

    axum::serve(
        listener,
        api_router
            .route("/status", get(|| async {}))
            .layer(TraceLayer::new_for_http()),
    )
    .await
    .unwrap();
}
