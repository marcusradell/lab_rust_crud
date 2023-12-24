mod app_router;
mod kits;
mod logging;

pub async fn lib() {
    println!("lib");
    logging::init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let router = app_router::create();

    axum::serve(listener, router).await.unwrap();
}
